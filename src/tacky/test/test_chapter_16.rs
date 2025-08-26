use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_char_constants_char_constant_operations() {
    let src = r#"
        double d = '\\';
        int main(void) {
            if (d != 92.0) {
                return 1;
            }
            unsigned long array['\n'] = {1, 2, 'a', '\b', 3, 4, 5, '!', '%', '~'};
            if (array[2] != 97) {
                return 2;
            }
            if (array[3] != 8) {
                return 3;
            }
            if (array[7] != 33) {
                return 4;
            }
            if (array[8] != 37) {
                return 5;
            }
            if (array[9] != 126) {
                return 6;
            }
            unsigned long (*array_ptr)[10] = &array;
            if (array_ptr[0][9] != '~') {
                return 7;
            }
            int i = array['\a'];
            if (i != 33) {
                return 8;
            }
            double d = 10 % '\a' + 4.0 * '_' - ~'@';
            if (d != 448.0) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = d != 92D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = sign_extend 1
            array.0[0] = tmp.1
            tmp.2 = sign_extend 2
            array.0[8] = tmp.2
            tmp.3 = sign_extend 97
            array.0[16] = tmp.3
            tmp.4 = sign_extend 8
            array.0[24] = tmp.4
            tmp.5 = sign_extend 3
            array.0[32] = tmp.5
            tmp.6 = sign_extend 4
            array.0[40] = tmp.6
            tmp.7 = sign_extend 5
            array.0[48] = tmp.7
            tmp.8 = sign_extend 33
            array.0[56] = tmp.8
            tmp.9 = sign_extend 37
            array.0[64] = tmp.9
            tmp.10 = sign_extend 126
            array.0[72] = tmp.10
            tmp.11 = &array.0
            tmp.12 = sign_extend 2
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=8)
            tmp.14 = *tmp.13
            tmp.16 = sign_extend 97
            tmp.15 = tmp.14 != tmp.16
            if !tmp.15 jump end_if_2
            return 2
        
          end_if_2:
            tmp.17 = &array.0
            tmp.18 = sign_extend 3
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=8)
            tmp.20 = *tmp.19
            tmp.22 = sign_extend 8
            tmp.21 = tmp.20 != tmp.22
            if !tmp.21 jump end_if_4
            return 3
        
          end_if_4:
            tmp.23 = &array.0
            tmp.24 = sign_extend 7
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=8)
            tmp.26 = *tmp.25
            tmp.28 = sign_extend 33
            tmp.27 = tmp.26 != tmp.28
            if !tmp.27 jump end_if_6
            return 4
        
          end_if_6:
            tmp.29 = &array.0
            tmp.30 = sign_extend 8
            tmp.31 = add_ptr(tmp.29, index=tmp.30, scale=8)
            tmp.32 = *tmp.31
            tmp.34 = sign_extend 37
            tmp.33 = tmp.32 != tmp.34
            if !tmp.33 jump end_if_8
            return 5
        
          end_if_8:
            tmp.35 = &array.0
            tmp.36 = sign_extend 9
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=8)
            tmp.38 = *tmp.37
            tmp.40 = sign_extend 126
            tmp.39 = tmp.38 != tmp.40
            if !tmp.39 jump end_if_10
            return 6
        
          end_if_10:
            tmp.41 = &array.0
            array_ptr.1 = tmp.41
            tmp.42 = sign_extend 0
            tmp.43 = add_ptr(array_ptr.1, index=tmp.42, scale=80)
            tmp.44 = sign_extend 9
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=8)
            tmp.46 = *tmp.45
            tmp.48 = sign_extend 126
            tmp.47 = tmp.46 != tmp.48
            if !tmp.47 jump end_if_12
            return 7
        
          end_if_12:
            tmp.49 = &array.0
            tmp.50 = sign_extend 7
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=8)
            tmp.52 = *tmp.51
            tmp.53 = truncate tmp.52
            i.2 = tmp.53
            tmp.54 = i.2 != 33
            if !tmp.54 jump end_if_14
            return 8
        
          end_if_14:
            tmp.55 = 10 % 7
            tmp.56 = int_to_double tmp.55
            tmp.59 = int_to_double 95
            tmp.58 = 4D * tmp.59
            tmp.57 = tmp.56 + tmp.58
            tmp.61 = ~ 64
            tmp.62 = int_to_double tmp.61
            tmp.60 = tmp.57 - tmp.62
            d.3 = tmp.60
            tmp.63 = d.3 != 448D
            if !tmp.63 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
        static global d: Double = 92D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_char_constants_control_characters() {
    let src = r#"
        int main(void)
        {
            int tab = '	';
            int vertical_tab = '';
            int form_feed = '';
            if (tab != '\t') {
                return 1;
            }
            if (vertical_tab != '\v') {
                return 2;
            }
            if (form_feed != '\f') {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tab.0 = 9
            vertical_tab.1 = 11
            form_feed.2 = 12
            tmp.0 = tab.0 != 9
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = vertical_tab.1 != 11
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = form_feed.2 != 12
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_char_constants_escape_sequences() {
    let src = r#"
        
        int main(void) {
            if ('\?' != 63) {
                return 1;
            }
            if ('\"' != 34) {
                return 2;
            }
            if ('\'' != 39) {
                return 3;
            }
            if ('\\' != 92) {
                return 4;
            }
            if ('\a' != 7) {
                return 5;
            }
            if ('\b' != 8) {
                return 6;
            }
            if ('\f' != 12) {
                return 7;
            }
            if ('\n' != 10) {
                return 8;
            }
            if ('\r' != 13) {
                return 9;
            }
            if ('\t' != 9) {
                return 10;
            }
            if ('\v' != 11) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 63 != 63
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = 34 != 34
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = 39 != 39
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = 92 != 92
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = 7 != 7
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.5 = 8 != 8
            if !tmp.5 jump end_if_10
            return 6
        
          end_if_10:
            tmp.6 = 12 != 12
            if !tmp.6 jump end_if_12
            return 7
        
          end_if_12:
            tmp.7 = 10 != 10
            if !tmp.7 jump end_if_14
            return 8
        
          end_if_14:
            tmp.8 = 13 != 13
            if !tmp.8 jump end_if_16
            return 9
        
          end_if_16:
            tmp.9 = 9 != 9
            if !tmp.9 jump end_if_18
            return 10
        
          end_if_18:
            tmp.10 = 11 != 11
            if !tmp.10 jump end_if_20
            return 11
        
          end_if_20:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_char_constants_return_char_constant() {
    let src = r#"
        
        int main(void) {
            return 'c';
        }
    "#;
    let expected = r#"
        global function main() { 
            return 99
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_access_through_char_pointer() {
    let src = r#"
        int main(void) {
            int x = 100;
            char *byte_ptr = (char *) &x;
            if (byte_ptr[0] != 100) {
                return 1;
            }
            if (byte_ptr[1] || byte_ptr[2] || byte_ptr[3]) {
                return 2;
            }
            double d = -0.0;
            byte_ptr = (char *) &d;
            if (byte_ptr[7] != -128) {
                return 3;
            }
            for (int i = 0; i < 7; i = i + 1) {
                if (byte_ptr[i]) {
                    return 4;
                }
            }
            unsigned int array[3][2][1] = {
                {{-1}, {-1}},
                {{-1}, {-1}},
                {{4294901760u}}
            };
            byte_ptr = (char *) array;
            byte_ptr = byte_ptr + 16;
            if (byte_ptr[0] || byte_ptr[1]) {
                return 5;
            }
            if (byte_ptr[2] != -1) {
                return 6;
            }
            if (byte_ptr[3] != -1) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 100
            tmp.0 = &x.0
            tmp.1 = tmp.0
            byte_ptr.1 = tmp.1
            tmp.2 = sign_extend 0
            tmp.3 = add_ptr(byte_ptr.1, index=tmp.2, scale=1)
            tmp.4 = *tmp.3
            tmp.5 = sign_extend tmp.4
            tmp.6 = tmp.5 != 100
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = sign_extend 1
            tmp.8 = add_ptr(byte_ptr.1, index=tmp.7, scale=1)
            tmp.9 = *tmp.8
            if tmp.9 jump or_true_2
            tmp.12 = sign_extend 2
            tmp.13 = add_ptr(byte_ptr.1, index=tmp.12, scale=1)
            tmp.14 = *tmp.13
            if tmp.14 jump or_true_2
            tmp.11 = 0
            jump or_end_3
        
          or_true_2:
            tmp.11 = 1
        
          or_end_3:
            if tmp.11 jump or_true_4
            tmp.17 = sign_extend 3
            tmp.18 = add_ptr(byte_ptr.1, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            if tmp.19 jump or_true_4
            tmp.16 = 0
            jump or_end_5
        
          or_true_4:
            tmp.16 = 1
        
          or_end_5:
            if !tmp.16 jump end_if_6
            return 2
        
          end_if_6:
            tmp.20 = - 0D
            d.2 = tmp.20
            tmp.21 = &d.2
            tmp.22 = tmp.21
            byte_ptr.1 = tmp.22
            tmp.23 = sign_extend 7
            tmp.24 = add_ptr(byte_ptr.1, index=tmp.23, scale=1)
            tmp.25 = *tmp.24
            tmp.26 = sign_extend tmp.25
            tmp.28 = - 128
            tmp.27 = tmp.26 != tmp.28
            if !tmp.27 jump end_if_8
            return 3
        
          end_if_8:
            i.3 = 0
        
          start_loop_0:
            tmp.29 = i.3 < 7
            if !tmp.29 jump break_loop_0
            tmp.30 = sign_extend i.3
            tmp.31 = add_ptr(byte_ptr.1, index=tmp.30, scale=1)
            tmp.32 = *tmp.31
            if !tmp.32 jump end_if_10
            return 4
        
          end_if_10:
        
          continue_loop_0:
            tmp.33 = i.3 + 1
            i.3 = tmp.33
            jump start_loop_0
        
          break_loop_0:
            tmp.34 = - 1
            tmp.35 = tmp.34
            array.4[0] = tmp.35
            tmp.36 = - 1
            tmp.37 = tmp.36
            array.4[4] = tmp.37
            tmp.38 = - 1
            tmp.39 = tmp.38
            array.4[8] = tmp.39
            tmp.40 = - 1
            tmp.41 = tmp.40
            array.4[12] = tmp.41
            array.4[16] = 4294901760U
            array.4[20] = 0U
            tmp.42 = &array.4
            tmp.43 = tmp.42
            byte_ptr.1 = tmp.43
            tmp.45 = sign_extend 16
            tmp.44 = add_ptr(byte_ptr.1, index=tmp.45, scale=1)
            byte_ptr.1 = tmp.44
            tmp.46 = sign_extend 0
            tmp.47 = add_ptr(byte_ptr.1, index=tmp.46, scale=1)
            tmp.48 = *tmp.47
            if tmp.48 jump or_true_12
            tmp.51 = sign_extend 1
            tmp.52 = add_ptr(byte_ptr.1, index=tmp.51, scale=1)
            tmp.53 = *tmp.52
            if tmp.53 jump or_true_12
            tmp.50 = 0
            jump or_end_13
        
          or_true_12:
            tmp.50 = 1
        
          or_end_13:
            if !tmp.50 jump end_if_14
            return 5
        
          end_if_14:
            tmp.54 = sign_extend 2
            tmp.55 = add_ptr(byte_ptr.1, index=tmp.54, scale=1)
            tmp.56 = *tmp.55
            tmp.57 = sign_extend tmp.56
            tmp.59 = - 1
            tmp.58 = tmp.57 != tmp.59
            if !tmp.58 jump end_if_16
            return 6
        
          end_if_16:
            tmp.60 = sign_extend 3
            tmp.61 = add_ptr(byte_ptr.1, index=tmp.60, scale=1)
            tmp.62 = *tmp.61
            tmp.63 = sign_extend tmp.62
            tmp.65 = - 1
            tmp.64 = tmp.63 != tmp.65
            if !tmp.64 jump end_if_18
            return 7
        
          end_if_18:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_chained_casts() {
    let src = r#"
        unsigned int ui = 4294967200u;
        int main(void) {
            ui = (unsigned int)(unsigned char)ui;
            if (ui != 160) {
                return 1;
            }
            int i = (int)(signed char)ui;
            if (i != -96) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate ui
            tmp.1 = zero_extend tmp.0
            ui = tmp.1
            tmp.3 = 160
            tmp.2 = ui != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = truncate ui
            tmp.5 = sign_extend tmp.4
            i.0 = tmp.5
            tmp.7 = - 96
            tmp.6 = i.0 != tmp.7
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
fn test_valid_chars_char_arguments() {
    let src = r#"
        
        int check_args(char a, signed char b, char c, unsigned char d, char e, char f, signed char g, char h) {
            char expected_a = 5;
            signed char expected_b = -12;
            char expected_c = 117;
            unsigned char expected_d = 254;
            char expected_e = 1;
            char expected_f = -20;
            signed char expected_g = 60;
            char expected_h = 100;
            if (expected_a != a) {
             return 1;
            }
            if (expected_b != b) {
             return 2;
            }
            if (expected_c != c) {
             return 3;
            }
            if (expected_d != d) {
             return 4;
            }
            if (expected_e != e) {
             return 5;
            }
            if (expected_f != f) {
             return 6;
            }
            if (expected_g != g) {
             return 7;
            }
            if (expected_h != h) {
             return 8;
            }
            return 0;
        }
        int main(void) {
            char a = 5;
            signed char b = -12;
            char c = 117;
            unsigned char d = 254;
            char e = 1;
            char f = -20;
            signed char g = 60;
            char h = 100;
            return check_args(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = r#"
        global function check_args(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7) { 
            tmp.0 = truncate 5
            expected_a.8 = tmp.0
            tmp.1 = - 12
            tmp.2 = truncate tmp.1
            expected_b.9 = tmp.2
            tmp.3 = truncate 117
            expected_c.10 = tmp.3
            tmp.4 = truncate 254
            expected_d.11 = tmp.4
            tmp.5 = truncate 1
            expected_e.12 = tmp.5
            tmp.6 = - 20
            tmp.7 = truncate tmp.6
            expected_f.13 = tmp.7
            tmp.8 = truncate 60
            expected_g.14 = tmp.8
            tmp.9 = truncate 100
            expected_h.15 = tmp.9
            tmp.10 = sign_extend expected_a.8
            tmp.12 = sign_extend a.0
            tmp.11 = tmp.10 != tmp.12
            if !tmp.11 jump end_if_0
            return 1
        
          end_if_0:
            tmp.13 = sign_extend expected_b.9
            tmp.15 = sign_extend b.1
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_2
            return 2
        
          end_if_2:
            tmp.16 = sign_extend expected_c.10
            tmp.18 = sign_extend c.2
            tmp.17 = tmp.16 != tmp.18
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            tmp.19 = zero_extend expected_d.11
            tmp.21 = zero_extend d.3
            tmp.20 = tmp.19 != tmp.21
            if !tmp.20 jump end_if_6
            return 4
        
          end_if_6:
            tmp.22 = sign_extend expected_e.12
            tmp.24 = sign_extend e.4
            tmp.23 = tmp.22 != tmp.24
            if !tmp.23 jump end_if_8
            return 5
        
          end_if_8:
            tmp.25 = sign_extend expected_f.13
            tmp.27 = sign_extend f.5
            tmp.26 = tmp.25 != tmp.27
            if !tmp.26 jump end_if_10
            return 6
        
          end_if_10:
            tmp.28 = sign_extend expected_g.14
            tmp.30 = sign_extend g.6
            tmp.29 = tmp.28 != tmp.30
            if !tmp.29 jump end_if_12
            return 7
        
          end_if_12:
            tmp.31 = sign_extend expected_h.15
            tmp.33 = sign_extend h.7
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        global function main() { 
            tmp.34 = truncate 5
            a.16 = tmp.34
            tmp.35 = - 12
            tmp.36 = truncate tmp.35
            b.17 = tmp.36
            tmp.37 = truncate 117
            c.18 = tmp.37
            tmp.38 = truncate 254
            d.19 = tmp.38
            tmp.39 = truncate 1
            e.20 = tmp.39
            tmp.40 = - 20
            tmp.41 = truncate tmp.40
            f.21 = tmp.41
            tmp.42 = truncate 60
            g.22 = tmp.42
            tmp.43 = truncate 100
            h.23 = tmp.43
            tmp.44 = check_args(a.16, b.17, c.18, d.19, e.20, f.21, g.22, h.23)
            return tmp.44
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_char_expressions() {
    let src = r#"
        int add_chars(char c1, char c2) {
            return c1 + c2;
        }
        int divide_chars(unsigned char c1, unsigned char c2) {
            return c1 / c2;
        }
        int le(char c1, char c2) {
            return c1 <= c2;
        }
        int subscript_char(int *ptr, char idx){
            return ptr[idx];
        }
        int *sub_char_from_pointer(int *ptr, signed char idx) {
            return ptr - idx;
        }
        int and_char(signed char c1, int i) {
            return c1 && i;
        }
        int or_char(signed char c1, unsigned char c2) {
            return c1 || c2;
        }
        int test_for_loop_char(void) {
            int counter = 0;
            for (signed char s = 127; s > 0; s = s - 1) {
                counter = counter + 1;
            }
            return (counter == 127);
        }
        int main(void) {
            char c1 = 8;
            char c2 = 4;
            if (add_chars(c1, c2) != 12) {
                return 1;
            }
            unsigned char uc1 = 250;
            unsigned char uc2 = 25;
            if (divide_chars(uc1, uc2) != 10) {
                return 2;
            }
            if (le(c1, c2)) {
                return 3;
            }
            if (!le(c2, c2)) {
                return 4;
            }
            int arr[4] = {11, 12, 13, 14};
            char idx = 2;
            if (subscript_char(arr, idx) != 13) {
                return 5;
            }
            signed char offset = 1;
            if (sub_char_from_pointer(arr + 1, offset) != arr) {
                return 6;
            }
            signed char zero = 0;
            if (zero) {
                return 7;
            }
            if (and_char(zero, 12)) {
                return 8;
            }
            uc2 = 0;
            if (or_char(zero, uc2)) {
                return 9;
            }
            if (!test_for_loop_char()) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function add_chars(c1.0, c2.1) { 
            tmp.0 = sign_extend c1.0
            tmp.2 = sign_extend c2.1
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
        global function divide_chars(c1.2, c2.3) { 
            tmp.3 = zero_extend c1.2
            tmp.5 = zero_extend c2.3
            tmp.4 = tmp.3 / tmp.5
            return tmp.4
            return 0
        }
        global function le(c1.4, c2.5) { 
            tmp.6 = sign_extend c1.4
            tmp.8 = sign_extend c2.5
            tmp.7 = tmp.6 <= tmp.8
            return tmp.7
            return 0
        }
        global function subscript_char(ptr.6, idx.7) { 
            tmp.9 = sign_extend idx.7
            tmp.10 = add_ptr(ptr.6, index=tmp.9, scale=4)
            tmp.11 = *tmp.10
            return tmp.11
            return 0
        }
        global function sub_char_from_pointer(ptr.8, idx.9) { 
            tmp.13 = sign_extend idx.9
            tmp.14 = - tmp.13
            tmp.12 = add_ptr(ptr.8, index=tmp.14, scale=4)
            return tmp.12
            return 0
        }
        global function and_char(c1.10, i.11) { 
            if !c1.10 jump and_false_0
            if !i.11 jump and_false_0
            tmp.16 = 1
            jump and_end_1
        
          and_false_0:
            tmp.16 = 0
        
          and_end_1:
            return tmp.16
            return 0
        }
        global function or_char(c1.12, c2.13) { 
            if c1.12 jump or_true_2
            if c2.13 jump or_true_2
            tmp.18 = 0
            jump or_end_3
        
          or_true_2:
            tmp.18 = 1
        
          or_end_3:
            return tmp.18
            return 0
        }
        global function test_for_loop_char() { 
            counter.14 = 0
            tmp.19 = truncate 127
            s.15 = tmp.19
        
          start_loop_0:
            tmp.20 = sign_extend s.15
            tmp.21 = tmp.20 > 0
            if !tmp.21 jump break_loop_0
            tmp.22 = counter.14 + 1
            counter.14 = tmp.22
        
          continue_loop_0:
            tmp.23 = sign_extend s.15
            tmp.24 = tmp.23 - 1
            tmp.25 = truncate tmp.24
            s.15 = tmp.25
            jump start_loop_0
        
          break_loop_0:
            tmp.26 = counter.14 == 127
            return tmp.26
            return 0
        }
        global function main() { 
            tmp.27 = truncate 8
            c1.16 = tmp.27
            tmp.28 = truncate 4
            c2.17 = tmp.28
            tmp.29 = add_chars(c1.16, c2.17)
            tmp.30 = tmp.29 != 12
            if !tmp.30 jump end_if_4
            return 1
        
          end_if_4:
            tmp.31 = truncate 250
            uc1.18 = tmp.31
            tmp.32 = truncate 25
            uc2.19 = tmp.32
            tmp.33 = divide_chars(uc1.18, uc2.19)
            tmp.34 = tmp.33 != 10
            if !tmp.34 jump end_if_6
            return 2
        
          end_if_6:
            tmp.35 = le(c1.16, c2.17)
            if !tmp.35 jump end_if_8
            return 3
        
          end_if_8:
            tmp.36 = le(c2.17, c2.17)
            tmp.37 = ! tmp.36
            if !tmp.37 jump end_if_10
            return 4
        
          end_if_10:
            arr.20[0] = 11
            arr.20[4] = 12
            arr.20[8] = 13
            arr.20[12] = 14
            tmp.38 = truncate 2
            idx.21 = tmp.38
            tmp.39 = &arr.20
            tmp.40 = subscript_char(tmp.39, idx.21)
            tmp.41 = tmp.40 != 13
            if !tmp.41 jump end_if_12
            return 5
        
          end_if_12:
            tmp.42 = truncate 1
            offset.22 = tmp.42
            tmp.43 = &arr.20
            tmp.45 = sign_extend 1
            tmp.44 = add_ptr(tmp.43, index=tmp.45, scale=4)
            tmp.46 = sub_char_from_pointer(tmp.44, offset.22)
            tmp.48 = &arr.20
            tmp.47 = tmp.46 != tmp.48
            if !tmp.47 jump end_if_14
            return 6
        
          end_if_14:
            tmp.49 = truncate 0
            zero.23 = tmp.49
            if !zero.23 jump end_if_16
            return 7
        
          end_if_16:
            tmp.50 = and_char(zero.23, 12)
            if !tmp.50 jump end_if_18
            return 8
        
          end_if_18:
            tmp.51 = truncate 0
            uc2.19 = tmp.51
            tmp.52 = or_char(zero.23, uc2.19)
            if !tmp.52 jump end_if_20
            return 9
        
          end_if_20:
            tmp.53 = test_for_loop_char()
            tmp.54 = ! tmp.53
            if !tmp.54 jump end_if_22
            return 10
        
          end_if_22:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_common_type() {
    let src = r#"
        long ternary(int flag, char c) {
            return flag ? c : 1u;
        }
        int char_lt_int(char c, int i) {
            return c < i;
        }
        int uchar_gt_long(unsigned char uc, long l) {
            return uc > l;
        }
        int char_lt_uchar(char c, unsigned char u) {
            return c < u;
        }
        int signed_char_le_char(signed char s, char c) {
            return s <= c;
        }
        char ten = 10;
        int multiply(void) {
            char i = 10.75 * ten;
            return i == 107;
        }
        int main(void) {
            if (ternary(1, -10) != 4294967286l) {
                return 1;
            }
            if (!char_lt_int((char)1, 256)) {
                return 2;
            }
            if (!uchar_gt_long((unsigned char)100, -2)) {
                return 3;
            }
            char c = -1;
            unsigned char u = 2;
            if (!char_lt_uchar(c, u)) {
                return 4;
            }
            signed char s = -1;
            if (!signed_char_le_char(s, c)) {
                return 5;
            }
            if (!multiply()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function ternary(flag.0, c.1) { 
            if !flag.0 jump else_1
            tmp.1 = sign_extend c.1
            tmp.0 = tmp.1
            jump end_if_0
        
          else_1:
            tmp.0 = 1U
        
          end_if_0:
            tmp.2 = zero_extend tmp.0
            return tmp.2
            return 0
        }
        global function char_lt_int(c.2, i.3) { 
            tmp.3 = sign_extend c.2
            tmp.4 = tmp.3 < i.3
            return tmp.4
            return 0
        }
        global function uchar_gt_long(uc.4, l.5) { 
            tmp.5 = zero_extend uc.4
            tmp.6 = tmp.5 > l.5
            return tmp.6
            return 0
        }
        global function char_lt_uchar(c.6, u.7) { 
            tmp.7 = sign_extend c.6
            tmp.9 = zero_extend u.7
            tmp.8 = tmp.7 < tmp.9
            return tmp.8
            return 0
        }
        global function signed_char_le_char(s.8, c.9) { 
            tmp.10 = sign_extend s.8
            tmp.12 = sign_extend c.9
            tmp.11 = tmp.10 <= tmp.12
            return tmp.11
            return 0
        }
        global function multiply() { 
            tmp.14 = int_to_double ten
            tmp.13 = 10.75D * tmp.14
            tmp.15 = double_to_int tmp.13
            i.10 = tmp.15
            tmp.16 = sign_extend i.10
            tmp.17 = tmp.16 == 107
            return tmp.17
            return 0
        }
        global function main() { 
            tmp.18 = - 10
            tmp.19 = truncate tmp.18
            tmp.20 = ternary(1, tmp.19)
            tmp.21 = tmp.20 != 4294967286L
            if !tmp.21 jump end_if_2
            return 1
        
          end_if_2:
            tmp.22 = truncate 1
            tmp.23 = char_lt_int(tmp.22, 256)
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_4
            return 2
        
          end_if_4:
            tmp.25 = truncate 100
            tmp.26 = - 2
            tmp.27 = sign_extend tmp.26
            tmp.28 = uchar_gt_long(tmp.25, tmp.27)
            tmp.29 = ! tmp.28
            if !tmp.29 jump end_if_6
            return 3
        
          end_if_6:
            tmp.30 = - 1
            tmp.31 = truncate tmp.30
            c.11 = tmp.31
            tmp.32 = truncate 2
            u.12 = tmp.32
            tmp.33 = char_lt_uchar(c.11, u.12)
            tmp.34 = ! tmp.33
            if !tmp.34 jump end_if_8
            return 4
        
          end_if_8:
            tmp.35 = - 1
            tmp.36 = truncate tmp.35
            s.13 = tmp.36
            tmp.37 = signed_char_le_char(s.13, c.11)
            tmp.38 = ! tmp.37
            if !tmp.38 jump end_if_10
            return 5
        
          end_if_10:
            tmp.39 = multiply()
            tmp.40 = ! tmp.39
            if !tmp.40 jump end_if_12
            return 6
        
          end_if_12:
            return 0
            return 0
        }
        static global ten: Char = '\n'
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_convert_by_assignment() {
    let src = r#"
        int check_int(int converted, int expected) {
            return (converted == expected);
        }
        int check_uint(unsigned int converted, unsigned int expected) {
            return (converted == expected);
        }
        int check_long(long converted, long expected) {
            return (converted == expected);
        }
        int check_ulong(unsigned long converted, unsigned long expected) {
            return (converted == expected);
        }
        int check_double(double converted, double expected) {
            return (converted == expected);
        }
        int check_char(char converted, char expected) {
            return (converted == expected);
        }
        int check_uchar(unsigned char converted, unsigned char expected) {
            return (converted == expected);
        }
        int check_char_on_stack(signed char expected, int dummy1, int dummy2,
                                int dummy3, int dummy4, int dummy5, int dummy6,
                                signed char converted) {
            return converted == expected;
        }
        int return_extended_uchar(unsigned char c) {
            return c;
        }
        unsigned long return_extended_schar(signed char sc) {
            return sc;
        }
        unsigned char return_truncated_long(long l) {
            return l;
        }
        int main(void) {
            signed char sc = -10;
            if (!check_long(sc, -10l)) {
                return 1;
            }
            if (!check_uint(sc, 4294967286u)) {
                return 2;
            }
            if (!check_double(sc, -10.0)) {
                return 3;
            }
            unsigned char uc = 246;
            if (!check_uchar(sc, uc)) {
                return 4;
            }
            char c = -10;
            if (!check_char(-10, c)) {
                return 5;
            }
            if (!check_char(4294967286u, c)) {
                return 6;
            }
            if (!check_char(-10.0, c)) {
                return 7;
            }
            if (!check_char_on_stack(c, 0, 0, 0, 0, 0, 0, -10.0)) {
                return 8;
            }
            if (!check_int(uc, 246)) {
                return 9;
            }
            if (!check_ulong(uc, 246ul)) {
                return 10;
            }
            char expected_char = -10;
            if (!check_char(uc, expected_char)) {
                return 11;
            }
            if (!check_uchar(18446744073709551606ul, uc)) {
                return 12;
            }
            if (return_extended_uchar(uc) != 246) {
                return 13;
            }
            if (return_extended_schar(sc) != 18446744073709551606ul) {
                return 14;
            }
            if (return_truncated_long(5369233654l) != uc) {
                return 15;
            }
            char array[3] = {0, 0, 0};
            array[1] = 128;
            if (array[0] || array[2] || array[1] != -128) {
                return 16;
            }
            array[1] = 9224497936761618562ul;
            if (array[0] || array[2] || array[1] != -126) {
                return 17;
            }
            array[1] = -2.6;
            if (array[0] || array[2] || array[1] != -2) {
                return 18;
            }
            unsigned char uchar_array[3] = {0, 0, 0};
            uchar_array[1] = 17592186044416l;
            if (uchar_array[0] || uchar_array[2] || uchar_array[1] != 0) {
                return 19;
            }
            uchar_array[1] = 2147483898u;
            if (uchar_array[0] || uchar_array[2] || uchar_array[1] != 250) {
                return 20;
            }
            unsigned int ui = 4294967295U;
            static unsigned char
                uc_static;
            ui = uc_static;
            if (ui) {
                return 21;
            }
            signed long l = -1;
            static signed s_static =
                0;
            l = s_static;
            if (l) {
                return 22;
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
        global function check_uint(converted.2, expected.3) { 
            tmp.1 = converted.2 == expected.3
            return tmp.1
            return 0
        }
        global function check_long(converted.4, expected.5) { 
            tmp.2 = converted.4 == expected.5
            return tmp.2
            return 0
        }
        global function check_ulong(converted.6, expected.7) { 
            tmp.3 = converted.6 == expected.7
            return tmp.3
            return 0
        }
        global function check_double(converted.8, expected.9) { 
            tmp.4 = converted.8 == expected.9
            return tmp.4
            return 0
        }
        global function check_char(converted.10, expected.11) { 
            tmp.5 = sign_extend converted.10
            tmp.7 = sign_extend expected.11
            tmp.6 = tmp.5 == tmp.7
            return tmp.6
            return 0
        }
        global function check_uchar(converted.12, expected.13) { 
            tmp.8 = zero_extend converted.12
            tmp.10 = zero_extend expected.13
            tmp.9 = tmp.8 == tmp.10
            return tmp.9
            return 0
        }
        global function check_char_on_stack(expected.14, dummy1.15, dummy2.16, dummy3.17, dummy4.18, dummy5.19, dummy6.20, converted.21) { 
            tmp.11 = sign_extend converted.21
            tmp.13 = sign_extend expected.14
            tmp.12 = tmp.11 == tmp.13
            return tmp.12
            return 0
        }
        global function return_extended_uchar(c.22) { 
            tmp.14 = zero_extend c.22
            return tmp.14
            return 0
        }
        global function return_extended_schar(sc.23) { 
            tmp.15 = sign_extend sc.23
            return tmp.15
            return 0
        }
        global function return_truncated_long(l.24) { 
            tmp.16 = truncate l.24
            return tmp.16
            return 0
        }
        global function main() { 
            tmp.17 = - 10
            tmp.18 = truncate tmp.17
            sc.25 = tmp.18
            tmp.19 = sign_extend sc.25
            tmp.20 = - 10L
            tmp.21 = check_long(tmp.19, tmp.20)
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_0
            return 1
        
          end_if_0:
            tmp.23 = sign_extend sc.25
            tmp.24 = check_uint(tmp.23, 4294967286U)
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_2
            return 2
        
          end_if_2:
            tmp.26 = int_to_double sc.25
            tmp.27 = - 10D
            tmp.28 = check_double(tmp.26, tmp.27)
            tmp.29 = ! tmp.28
            if !tmp.29 jump end_if_4
            return 3
        
          end_if_4:
            tmp.30 = truncate 246
            uc.26 = tmp.30
            tmp.31 = sc.25
            tmp.32 = check_uchar(tmp.31, uc.26)
            tmp.33 = ! tmp.32
            if !tmp.33 jump end_if_6
            return 4
        
          end_if_6:
            tmp.34 = - 10
            tmp.35 = truncate tmp.34
            c.27 = tmp.35
            tmp.36 = - 10
            tmp.37 = truncate tmp.36
            tmp.38 = check_char(tmp.37, c.27)
            tmp.39 = ! tmp.38
            if !tmp.39 jump end_if_8
            return 5
        
          end_if_8:
            tmp.40 = truncate 4294967286U
            tmp.41 = check_char(tmp.40, c.27)
            tmp.42 = ! tmp.41
            if !tmp.42 jump end_if_10
            return 6
        
          end_if_10:
            tmp.43 = - 10D
            tmp.44 = double_to_int tmp.43
            tmp.45 = check_char(tmp.44, c.27)
            tmp.46 = ! tmp.45
            if !tmp.46 jump end_if_12
            return 7
        
          end_if_12:
            tmp.47 = c.27
            tmp.48 = - 10D
            tmp.49 = double_to_int tmp.48
            tmp.50 = check_char_on_stack(tmp.47, 0, 0, 0, 0, 0, 0, tmp.49)
            tmp.51 = ! tmp.50
            if !tmp.51 jump end_if_14
            return 8
        
          end_if_14:
            tmp.52 = zero_extend uc.26
            tmp.53 = check_int(tmp.52, 246)
            tmp.54 = ! tmp.53
            if !tmp.54 jump end_if_16
            return 9
        
          end_if_16:
            tmp.55 = zero_extend uc.26
            tmp.56 = check_ulong(tmp.55, 246UL)
            tmp.57 = ! tmp.56
            if !tmp.57 jump end_if_18
            return 10
        
          end_if_18:
            tmp.58 = - 10
            tmp.59 = truncate tmp.58
            expected_char.28 = tmp.59
            tmp.60 = uc.26
            tmp.61 = check_char(tmp.60, expected_char.28)
            tmp.62 = ! tmp.61
            if !tmp.62 jump end_if_20
            return 11
        
          end_if_20:
            tmp.63 = truncate 18446744073709551606UL
            tmp.64 = check_uchar(tmp.63, uc.26)
            tmp.65 = ! tmp.64
            if !tmp.65 jump end_if_22
            return 12
        
          end_if_22:
            tmp.66 = return_extended_uchar(uc.26)
            tmp.67 = tmp.66 != 246
            if !tmp.67 jump end_if_24
            return 13
        
          end_if_24:
            tmp.68 = return_extended_schar(sc.25)
            tmp.69 = tmp.68 != 18446744073709551606UL
            if !tmp.69 jump end_if_26
            return 14
        
          end_if_26:
            tmp.70 = return_truncated_long(5369233654L)
            tmp.71 = zero_extend tmp.70
            tmp.73 = zero_extend uc.26
            tmp.72 = tmp.71 != tmp.73
            if !tmp.72 jump end_if_28
            return 15
        
          end_if_28:
            tmp.74 = truncate 0
            array.29[0] = tmp.74
            tmp.75 = truncate 0
            array.29[1] = tmp.75
            tmp.76 = truncate 0
            array.29[2] = tmp.76
            tmp.77 = &array.29
            tmp.78 = sign_extend 1
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=1)
            tmp.80 = truncate 128
            *tmp.79 = tmp.80
            tmp.81 = &array.29
            tmp.82 = sign_extend 0
            tmp.83 = add_ptr(tmp.81, index=tmp.82, scale=1)
            tmp.84 = *tmp.83
            if tmp.84 jump or_true_30
            tmp.87 = &array.29
            tmp.88 = sign_extend 2
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=1)
            tmp.90 = *tmp.89
            if tmp.90 jump or_true_30
            tmp.86 = 0
            jump or_end_31
        
          or_true_30:
            tmp.86 = 1
        
          or_end_31:
            if tmp.86 jump or_true_32
            tmp.93 = &array.29
            tmp.94 = sign_extend 1
            tmp.95 = add_ptr(tmp.93, index=tmp.94, scale=1)
            tmp.96 = *tmp.95
            tmp.97 = sign_extend tmp.96
            tmp.99 = - 128
            tmp.98 = tmp.97 != tmp.99
            if tmp.98 jump or_true_32
            tmp.92 = 0
            jump or_end_33
        
          or_true_32:
            tmp.92 = 1
        
          or_end_33:
            if !tmp.92 jump end_if_34
            return 16
        
          end_if_34:
            tmp.100 = &array.29
            tmp.101 = sign_extend 1
            tmp.102 = add_ptr(tmp.100, index=tmp.101, scale=1)
            tmp.103 = truncate 9224497936761618562UL
            *tmp.102 = tmp.103
            tmp.104 = &array.29
            tmp.105 = sign_extend 0
            tmp.106 = add_ptr(tmp.104, index=tmp.105, scale=1)
            tmp.107 = *tmp.106
            if tmp.107 jump or_true_36
            tmp.110 = &array.29
            tmp.111 = sign_extend 2
            tmp.112 = add_ptr(tmp.110, index=tmp.111, scale=1)
            tmp.113 = *tmp.112
            if tmp.113 jump or_true_36
            tmp.109 = 0
            jump or_end_37
        
          or_true_36:
            tmp.109 = 1
        
          or_end_37:
            if tmp.109 jump or_true_38
            tmp.116 = &array.29
            tmp.117 = sign_extend 1
            tmp.118 = add_ptr(tmp.116, index=tmp.117, scale=1)
            tmp.119 = *tmp.118
            tmp.120 = sign_extend tmp.119
            tmp.122 = - 126
            tmp.121 = tmp.120 != tmp.122
            if tmp.121 jump or_true_38
            tmp.115 = 0
            jump or_end_39
        
          or_true_38:
            tmp.115 = 1
        
          or_end_39:
            if !tmp.115 jump end_if_40
            return 17
        
          end_if_40:
            tmp.123 = &array.29
            tmp.124 = sign_extend 1
            tmp.125 = add_ptr(tmp.123, index=tmp.124, scale=1)
            tmp.126 = - 2.6D
            tmp.127 = double_to_int tmp.126
            *tmp.125 = tmp.127
            tmp.128 = &array.29
            tmp.129 = sign_extend 0
            tmp.130 = add_ptr(tmp.128, index=tmp.129, scale=1)
            tmp.131 = *tmp.130
            if tmp.131 jump or_true_42
            tmp.134 = &array.29
            tmp.135 = sign_extend 2
            tmp.136 = add_ptr(tmp.134, index=tmp.135, scale=1)
            tmp.137 = *tmp.136
            if tmp.137 jump or_true_42
            tmp.133 = 0
            jump or_end_43
        
          or_true_42:
            tmp.133 = 1
        
          or_end_43:
            if tmp.133 jump or_true_44
            tmp.140 = &array.29
            tmp.141 = sign_extend 1
            tmp.142 = add_ptr(tmp.140, index=tmp.141, scale=1)
            tmp.143 = *tmp.142
            tmp.144 = sign_extend tmp.143
            tmp.146 = - 2
            tmp.145 = tmp.144 != tmp.146
            if tmp.145 jump or_true_44
            tmp.139 = 0
            jump or_end_45
        
          or_true_44:
            tmp.139 = 1
        
          or_end_45:
            if !tmp.139 jump end_if_46
            return 18
        
          end_if_46:
            tmp.147 = truncate 0
            uchar_array.30[0] = tmp.147
            tmp.148 = truncate 0
            uchar_array.30[1] = tmp.148
            tmp.149 = truncate 0
            uchar_array.30[2] = tmp.149
            tmp.150 = &uchar_array.30
            tmp.151 = sign_extend 1
            tmp.152 = add_ptr(tmp.150, index=tmp.151, scale=1)
            tmp.153 = truncate 17592186044416L
            *tmp.152 = tmp.153
            tmp.154 = &uchar_array.30
            tmp.155 = sign_extend 0
            tmp.156 = add_ptr(tmp.154, index=tmp.155, scale=1)
            tmp.157 = *tmp.156
            if tmp.157 jump or_true_48
            tmp.160 = &uchar_array.30
            tmp.161 = sign_extend 2
            tmp.162 = add_ptr(tmp.160, index=tmp.161, scale=1)
            tmp.163 = *tmp.162
            if tmp.163 jump or_true_48
            tmp.159 = 0
            jump or_end_49
        
          or_true_48:
            tmp.159 = 1
        
          or_end_49:
            if tmp.159 jump or_true_50
            tmp.166 = &uchar_array.30
            tmp.167 = sign_extend 1
            tmp.168 = add_ptr(tmp.166, index=tmp.167, scale=1)
            tmp.169 = *tmp.168
            tmp.170 = zero_extend tmp.169
            tmp.171 = tmp.170 != 0
            if tmp.171 jump or_true_50
            tmp.165 = 0
            jump or_end_51
        
          or_true_50:
            tmp.165 = 1
        
          or_end_51:
            if !tmp.165 jump end_if_52
            return 19
        
          end_if_52:
            tmp.172 = &uchar_array.30
            tmp.173 = sign_extend 1
            tmp.174 = add_ptr(tmp.172, index=tmp.173, scale=1)
            tmp.175 = truncate 2147483898U
            *tmp.174 = tmp.175
            tmp.176 = &uchar_array.30
            tmp.177 = sign_extend 0
            tmp.178 = add_ptr(tmp.176, index=tmp.177, scale=1)
            tmp.179 = *tmp.178
            if tmp.179 jump or_true_54
            tmp.182 = &uchar_array.30
            tmp.183 = sign_extend 2
            tmp.184 = add_ptr(tmp.182, index=tmp.183, scale=1)
            tmp.185 = *tmp.184
            if tmp.185 jump or_true_54
            tmp.181 = 0
            jump or_end_55
        
          or_true_54:
            tmp.181 = 1
        
          or_end_55:
            if tmp.181 jump or_true_56
            tmp.188 = &uchar_array.30
            tmp.189 = sign_extend 1
            tmp.190 = add_ptr(tmp.188, index=tmp.189, scale=1)
            tmp.191 = *tmp.190
            tmp.192 = zero_extend tmp.191
            tmp.193 = tmp.192 != 250
            if tmp.193 jump or_true_56
            tmp.187 = 0
            jump or_end_57
        
          or_true_56:
            tmp.187 = 1
        
          or_end_57:
            if !tmp.187 jump end_if_58
            return 20
        
          end_if_58:
            ui.31 = 4294967295U
            tmp.194 = zero_extend uc_static.32
            ui.31 = tmp.194
            if !ui.31 jump end_if_60
            return 21
        
          end_if_60:
            tmp.195 = - 1
            tmp.196 = sign_extend tmp.195
            l.33 = tmp.196
            tmp.197 = sign_extend s_static.34
            l.33 = tmp.197
            if !l.33 jump end_if_62
            return 22
        
          end_if_62:
            return 0
            return 0
        }
        static s_static.34: Int = 0
        static uc_static.32: Unsigned Char = zero[1]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_explicit_casts() {
    let src = r#"
        unsigned char char_to_uchar(char c) {
            return (unsigned char)c;
        }
        signed char char_to_schar(char c) {
            return (signed char)c;
        }
        char uchar_to_char(unsigned char u) {
            return (char)u;
        }
        char schar_to_char(signed char u) {
            return (char)u;
        }
        signed char uchar_to_schar(unsigned char u) {
            return (signed char)u;
        }
        unsigned char schar_to_uchar(signed char u) {
            return (unsigned char)u;
        }
        int char_to_int(char c) {
            return (int)c;
        }
        unsigned int char_to_uint(char c) {
            return (unsigned int)c;
        }
        long char_to_long(char c) {
            return (long)c;
        }
        unsigned long char_to_ulong(char c) {
            return (unsigned long)c;
        }
        double char_to_double(char c) {
            return (double)c;
        }
        int schar_to_int(signed char s) {
            return (int)s;
        }
        unsigned int schar_to_uint(signed char s) {
            return (unsigned int)s;
        }
        long schar_to_long(signed char s) {
            return (long)s;
        }
        unsigned long schar_to_ulong(signed char s) {
            return (unsigned long)s;
        }
        double schar_to_double(signed char s) {
            return (double)s;
        }
        int uchar_to_int(unsigned char u) {
            return (int)u;
        }
        unsigned int uchar_to_uint(unsigned char u) {
            return (unsigned int)u;
        }
        long uchar_to_long(unsigned char u) {
            return (long)u;
        }
        unsigned long uchar_to_ulong(unsigned char u) {
            return (unsigned long)u;
        }
        double uchar_to_double(unsigned char u) {
            return (double)u;
        }
        char int_to_char(int i) {
            return (char)i;
        }
        char uint_to_char(unsigned int u) {
            return (char)u;
        }
        char double_to_char(double d) {
            return (char)d;
        }
        signed char long_to_schar(long l) {
            return (signed char)l;
        }
        signed char ulong_to_schar(unsigned long l) {
            return (signed char)l;
        }
        unsigned char int_to_uchar(int i) {
            return (unsigned char)i;
        }
        unsigned char uint_to_uchar(unsigned int ui) {
            return (unsigned char)ui;
        }
        unsigned char long_to_uchar(long l) {
            return (unsigned char)l;
        }
        unsigned char ulong_to_uchar(unsigned long ul) {
            return (unsigned char)ul;
        }
        unsigned char double_to_uchar(double d) {
            return (unsigned char)d;
        }
        int main(void) {
            char c = 127;
            if (char_to_uchar(c) != 127) {
                return 1;
            }
            if (char_to_int(c) != 127) {
                return 2;
            }
            if (char_to_ulong(c) != 127) {
                return 3;
            }
            signed char sc = -10;
            if (schar_to_uchar(sc) != 246) {
                return 4;
            }
            if (schar_to_long(sc) != -10) {
                return 5;
            }
            if (schar_to_uint(sc) != 4294967286u) {
                return 6;
            }
            if (schar_to_double(sc) != -10.0) {
                return 7;
            }
            unsigned char uc = 250;
            if (uchar_to_int(uc) != 250) {
                return 8;
            }
            if (uchar_to_long(uc) != 250) {
                return 9;
            }
            if (uchar_to_uint(uc) != 250) {
                return 10;
            }
            if (uchar_to_ulong(uc) != 250) {
                return 11;
            }
            if (uchar_to_double(uc) != 250.0) {
                return 12;
            }
            if (uchar_to_schar(uc) != -6) {
                return 13;
            }
            if (uchar_to_char(uc) != -6) {
                return 14;
            }
            c = (char)-128;
            if (int_to_char(128) != c) {
                return 15;
            }
            c = (char)-6;
            if (uint_to_char(2147483898u) != c) {
                return 16;
            }
            c = (char)-2;
            if (double_to_char(-2.6) != c) {
                return 17;
            }
            if (long_to_schar(17592186044416l)) {
                return 18;
            }
            sc = (signed char)-126;
            if (ulong_to_schar(9224497936761618562ul) != sc) {
                return 19;
            }
            uc = (unsigned char)200;
            if (int_to_uchar(-1234488) != uc) {
                return 20;
            }
            if (uint_to_uchar(4293732808) != uc) {
                return 21;
            }
            if (long_to_uchar(-36283884951096l) != uc) {
                return 22;
            }
            if (ulong_to_uchar(9224497936761618632ul) != uc) {
                return 23;
            }
            if (double_to_uchar(200.99) != uc) {
                return 24;
            }
            static long *null_ptr;
            char zero = (char)null_ptr;
            if (zero) {
                return 25;
            }
            c = 32;
            int *i = (int *)c;
            if ((char)i != c) {
                return 26;
            }
            if ((char)300 != (char)44) {
                return 27;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function char_to_uchar(c.0) { 
            tmp.0 = c.0
            return tmp.0
            return 0
        }
        global function char_to_schar(c.1) { 
            tmp.1 = c.1
            return tmp.1
            return 0
        }
        global function uchar_to_char(u.2) { 
            tmp.2 = u.2
            return tmp.2
            return 0
        }
        global function schar_to_char(u.3) { 
            tmp.3 = u.3
            return tmp.3
            return 0
        }
        global function uchar_to_schar(u.4) { 
            tmp.4 = u.4
            return tmp.4
            return 0
        }
        global function schar_to_uchar(u.5) { 
            tmp.5 = u.5
            return tmp.5
            return 0
        }
        global function char_to_int(c.6) { 
            tmp.6 = sign_extend c.6
            return tmp.6
            return 0
        }
        global function char_to_uint(c.7) { 
            tmp.7 = sign_extend c.7
            return tmp.7
            return 0
        }
        global function char_to_long(c.8) { 
            tmp.8 = sign_extend c.8
            return tmp.8
            return 0
        }
        global function char_to_ulong(c.9) { 
            tmp.9 = sign_extend c.9
            return tmp.9
            return 0
        }
        global function char_to_double(c.10) { 
            tmp.10 = int_to_double c.10
            return tmp.10
            return 0
        }
        global function schar_to_int(s.11) { 
            tmp.11 = sign_extend s.11
            return tmp.11
            return 0
        }
        global function schar_to_uint(s.12) { 
            tmp.12 = sign_extend s.12
            return tmp.12
            return 0
        }
        global function schar_to_long(s.13) { 
            tmp.13 = sign_extend s.13
            return tmp.13
            return 0
        }
        global function schar_to_ulong(s.14) { 
            tmp.14 = sign_extend s.14
            return tmp.14
            return 0
        }
        global function schar_to_double(s.15) { 
            tmp.15 = int_to_double s.15
            return tmp.15
            return 0
        }
        global function uchar_to_int(u.16) { 
            tmp.16 = zero_extend u.16
            return tmp.16
            return 0
        }
        global function uchar_to_uint(u.17) { 
            tmp.17 = zero_extend u.17
            return tmp.17
            return 0
        }
        global function uchar_to_long(u.18) { 
            tmp.18 = zero_extend u.18
            return tmp.18
            return 0
        }
        global function uchar_to_ulong(u.19) { 
            tmp.19 = zero_extend u.19
            return tmp.19
            return 0
        }
        global function uchar_to_double(u.20) { 
            tmp.20 = uint_to_double u.20
            return tmp.20
            return 0
        }
        global function int_to_char(i.21) { 
            tmp.21 = truncate i.21
            return tmp.21
            return 0
        }
        global function uint_to_char(u.22) { 
            tmp.22 = truncate u.22
            return tmp.22
            return 0
        }
        global function double_to_char(d.23) { 
            tmp.23 = double_to_int d.23
            return tmp.23
            return 0
        }
        global function long_to_schar(l.24) { 
            tmp.24 = truncate l.24
            return tmp.24
            return 0
        }
        global function ulong_to_schar(l.25) { 
            tmp.25 = truncate l.25
            return tmp.25
            return 0
        }
        global function int_to_uchar(i.26) { 
            tmp.26 = truncate i.26
            return tmp.26
            return 0
        }
        global function uint_to_uchar(ui.27) { 
            tmp.27 = truncate ui.27
            return tmp.27
            return 0
        }
        global function long_to_uchar(l.28) { 
            tmp.28 = truncate l.28
            return tmp.28
            return 0
        }
        global function ulong_to_uchar(ul.29) { 
            tmp.29 = truncate ul.29
            return tmp.29
            return 0
        }
        global function double_to_uchar(d.30) { 
            tmp.30 = double_to_uint d.30
            return tmp.30
            return 0
        }
        global function main() { 
            tmp.31 = truncate 127
            c.31 = tmp.31
            tmp.32 = char_to_uchar(c.31)
            tmp.33 = zero_extend tmp.32
            tmp.34 = tmp.33 != 127
            if !tmp.34 jump end_if_0
            return 1
        
          end_if_0:
            tmp.35 = char_to_int(c.31)
            tmp.36 = tmp.35 != 127
            if !tmp.36 jump end_if_2
            return 2
        
          end_if_2:
            tmp.37 = char_to_ulong(c.31)
            tmp.39 = sign_extend 127
            tmp.38 = tmp.37 != tmp.39
            if !tmp.38 jump end_if_4
            return 3
        
          end_if_4:
            tmp.40 = - 10
            tmp.41 = truncate tmp.40
            sc.32 = tmp.41
            tmp.42 = schar_to_uchar(sc.32)
            tmp.43 = zero_extend tmp.42
            tmp.44 = tmp.43 != 246
            if !tmp.44 jump end_if_6
            return 4
        
          end_if_6:
            tmp.45 = schar_to_long(sc.32)
            tmp.47 = - 10
            tmp.48 = sign_extend tmp.47
            tmp.46 = tmp.45 != tmp.48
            if !tmp.46 jump end_if_8
            return 5
        
          end_if_8:
            tmp.49 = schar_to_uint(sc.32)
            tmp.50 = tmp.49 != 4294967286U
            if !tmp.50 jump end_if_10
            return 6
        
          end_if_10:
            tmp.51 = schar_to_double(sc.32)
            tmp.53 = - 10D
            tmp.52 = tmp.51 != tmp.53
            if !tmp.52 jump end_if_12
            return 7
        
          end_if_12:
            tmp.54 = truncate 250
            uc.33 = tmp.54
            tmp.55 = uchar_to_int(uc.33)
            tmp.56 = tmp.55 != 250
            if !tmp.56 jump end_if_14
            return 8
        
          end_if_14:
            tmp.57 = uchar_to_long(uc.33)
            tmp.59 = sign_extend 250
            tmp.58 = tmp.57 != tmp.59
            if !tmp.58 jump end_if_16
            return 9
        
          end_if_16:
            tmp.60 = uchar_to_uint(uc.33)
            tmp.62 = 250
            tmp.61 = tmp.60 != tmp.62
            if !tmp.61 jump end_if_18
            return 10
        
          end_if_18:
            tmp.63 = uchar_to_ulong(uc.33)
            tmp.65 = sign_extend 250
            tmp.64 = tmp.63 != tmp.65
            if !tmp.64 jump end_if_20
            return 11
        
          end_if_20:
            tmp.66 = uchar_to_double(uc.33)
            tmp.67 = tmp.66 != 250D
            if !tmp.67 jump end_if_22
            return 12
        
          end_if_22:
            tmp.68 = uchar_to_schar(uc.33)
            tmp.69 = sign_extend tmp.68
            tmp.71 = - 6
            tmp.70 = tmp.69 != tmp.71
            if !tmp.70 jump end_if_24
            return 13
        
          end_if_24:
            tmp.72 = uchar_to_char(uc.33)
            tmp.73 = sign_extend tmp.72
            tmp.75 = - 6
            tmp.74 = tmp.73 != tmp.75
            if !tmp.74 jump end_if_26
            return 14
        
          end_if_26:
            tmp.76 = - 128
            tmp.77 = truncate tmp.76
            c.31 = tmp.77
            tmp.78 = int_to_char(128)
            tmp.79 = sign_extend tmp.78
            tmp.81 = sign_extend c.31
            tmp.80 = tmp.79 != tmp.81
            if !tmp.80 jump end_if_28
            return 15
        
          end_if_28:
            tmp.82 = - 6
            tmp.83 = truncate tmp.82
            c.31 = tmp.83
            tmp.84 = uint_to_char(2147483898U)
            tmp.85 = sign_extend tmp.84
            tmp.87 = sign_extend c.31
            tmp.86 = tmp.85 != tmp.87
            if !tmp.86 jump end_if_30
            return 16
        
          end_if_30:
            tmp.88 = - 2
            tmp.89 = truncate tmp.88
            c.31 = tmp.89
            tmp.90 = - 2.6D
            tmp.91 = double_to_char(tmp.90)
            tmp.92 = sign_extend tmp.91
            tmp.94 = sign_extend c.31
            tmp.93 = tmp.92 != tmp.94
            if !tmp.93 jump end_if_32
            return 17
        
          end_if_32:
            tmp.95 = long_to_schar(17592186044416L)
            if !tmp.95 jump end_if_34
            return 18
        
          end_if_34:
            tmp.96 = - 126
            tmp.97 = truncate tmp.96
            sc.32 = tmp.97
            tmp.98 = ulong_to_schar(9224497936761618562UL)
            tmp.99 = sign_extend tmp.98
            tmp.101 = sign_extend sc.32
            tmp.100 = tmp.99 != tmp.101
            if !tmp.100 jump end_if_36
            return 19
        
          end_if_36:
            tmp.102 = truncate 200
            uc.33 = tmp.102
            tmp.103 = - 1234488
            tmp.104 = int_to_uchar(tmp.103)
            tmp.105 = zero_extend tmp.104
            tmp.107 = zero_extend uc.33
            tmp.106 = tmp.105 != tmp.107
            if !tmp.106 jump end_if_38
            return 20
        
          end_if_38:
            tmp.108 = truncate 4293732808L
            tmp.109 = uint_to_uchar(tmp.108)
            tmp.110 = zero_extend tmp.109
            tmp.112 = zero_extend uc.33
            tmp.111 = tmp.110 != tmp.112
            if !tmp.111 jump end_if_40
            return 21
        
          end_if_40:
            tmp.113 = - 36283884951096L
            tmp.114 = long_to_uchar(tmp.113)
            tmp.115 = zero_extend tmp.114
            tmp.117 = zero_extend uc.33
            tmp.116 = tmp.115 != tmp.117
            if !tmp.116 jump end_if_42
            return 22
        
          end_if_42:
            tmp.118 = ulong_to_uchar(9224497936761618632UL)
            tmp.119 = zero_extend tmp.118
            tmp.121 = zero_extend uc.33
            tmp.120 = tmp.119 != tmp.121
            if !tmp.120 jump end_if_44
            return 23
        
          end_if_44:
            tmp.122 = double_to_uchar(200.99D)
            tmp.123 = zero_extend tmp.122
            tmp.125 = zero_extend uc.33
            tmp.124 = tmp.123 != tmp.125
            if !tmp.124 jump end_if_46
            return 24
        
          end_if_46:
            tmp.126 = truncate null_ptr.34
            zero.35 = tmp.126
            if !zero.35 jump end_if_48
            return 25
        
          end_if_48:
            tmp.127 = truncate 32
            c.31 = tmp.127
            tmp.128 = sign_extend c.31
            i.36 = tmp.128
            tmp.129 = truncate i.36
            tmp.130 = sign_extend tmp.129
            tmp.132 = sign_extend c.31
            tmp.131 = tmp.130 != tmp.132
            if !tmp.131 jump end_if_50
            return 26
        
          end_if_50:
            tmp.133 = truncate 300
            tmp.134 = sign_extend tmp.133
            tmp.136 = truncate 44
            tmp.137 = sign_extend tmp.136
            tmp.135 = tmp.134 != tmp.137
            if !tmp.135 jump end_if_52
            return 27
        
          end_if_52:
            return 0
            return 0
        }
        static null_ptr.34: Pointer(Long) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_integer_promotion() {
    let src = r#"
        int add_chars(char c1, char c2, char c3) {
            return c1 + c2 + c3;
        }
        int negate(unsigned char uc) {
            return -uc;
        }
        int complement(unsigned char uc) {
            return ~uc;
        }
        int add_then_div(signed char a, signed char b, signed char c) {
            return (a + b) / c;
        }
        int mixed_multiply(signed char s, unsigned char u) {
            return s * u;
        }
        signed char decrement(signed char s) {
            s = s - 1;
            return s;
        }
        int main(void) {
            char a = 100;
            char b = 109;
            if (add_chars(a, a, b) != 309) {
                return 1;
            }
            unsigned char one = 1;
            if (negate(one) != -1) {
                return 2;
            }
            if (complement(one) != -2) {
                return 3;
            }
            signed char w = 127;
            signed char x = 3;
            signed char y = 2;
            if (add_then_div(w, x, y) != 65)
                return 4;
            signed char sc = -3;
            unsigned char uc = 250;
            if (mixed_multiply(sc, uc) != -750)
                return 5;
            sc = -128;
            if (sc != -128) {
                return 6;
            }
            if (decrement(sc) != 127) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function add_chars(c1.0, c2.1, c3.2) { 
            tmp.0 = sign_extend c1.0
            tmp.2 = sign_extend c2.1
            tmp.1 = tmp.0 + tmp.2
            tmp.4 = sign_extend c3.2
            tmp.3 = tmp.1 + tmp.4
            return tmp.3
            return 0
        }
        global function negate(uc.3) { 
            tmp.5 = zero_extend uc.3
            tmp.6 = - tmp.5
            return tmp.6
            return 0
        }
        global function complement(uc.4) { 
            tmp.7 = zero_extend uc.4
            tmp.8 = ~ tmp.7
            return tmp.8
            return 0
        }
        global function add_then_div(a.5, b.6, c.7) { 
            tmp.9 = sign_extend a.5
            tmp.11 = sign_extend b.6
            tmp.10 = tmp.9 + tmp.11
            tmp.13 = sign_extend c.7
            tmp.12 = tmp.10 / tmp.13
            return tmp.12
            return 0
        }
        global function mixed_multiply(s.8, u.9) { 
            tmp.14 = sign_extend s.8
            tmp.16 = zero_extend u.9
            tmp.15 = tmp.14 * tmp.16
            return tmp.15
            return 0
        }
        global function decrement(s.10) { 
            tmp.17 = sign_extend s.10
            tmp.18 = tmp.17 - 1
            tmp.19 = truncate tmp.18
            s.10 = tmp.19
            return s.10
            return 0
        }
        global function main() { 
            tmp.20 = truncate 100
            a.11 = tmp.20
            tmp.21 = truncate 109
            b.12 = tmp.21
            tmp.22 = add_chars(a.11, a.11, b.12)
            tmp.23 = tmp.22 != 309
            if !tmp.23 jump end_if_0
            return 1
        
          end_if_0:
            tmp.24 = truncate 1
            one.13 = tmp.24
            tmp.25 = negate(one.13)
            tmp.27 = - 1
            tmp.26 = tmp.25 != tmp.27
            if !tmp.26 jump end_if_2
            return 2
        
          end_if_2:
            tmp.28 = complement(one.13)
            tmp.30 = - 2
            tmp.29 = tmp.28 != tmp.30
            if !tmp.29 jump end_if_4
            return 3
        
          end_if_4:
            tmp.31 = truncate 127
            w.14 = tmp.31
            tmp.32 = truncate 3
            x.15 = tmp.32
            tmp.33 = truncate 2
            y.16 = tmp.33
            tmp.34 = add_then_div(w.14, x.15, y.16)
            tmp.35 = tmp.34 != 65
            if !tmp.35 jump end_if_6
            return 4
        
          end_if_6:
            tmp.36 = - 3
            tmp.37 = truncate tmp.36
            sc.17 = tmp.37
            tmp.38 = truncate 250
            uc.18 = tmp.38
            tmp.39 = mixed_multiply(sc.17, uc.18)
            tmp.41 = - 750
            tmp.40 = tmp.39 != tmp.41
            if !tmp.40 jump end_if_8
            return 5
        
          end_if_8:
            tmp.42 = - 128
            tmp.43 = truncate tmp.42
            sc.17 = tmp.43
            tmp.44 = sign_extend sc.17
            tmp.46 = - 128
            tmp.45 = tmp.44 != tmp.46
            if !tmp.45 jump end_if_10
            return 6
        
          end_if_10:
            tmp.47 = decrement(sc.17)
            tmp.48 = sign_extend tmp.47
            tmp.49 = tmp.48 != 127
            if !tmp.49 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_partial_initialization() {
    let src = r#"
        char static1[4] = {1, 2};
        signed char static2[4] = {3, 4};
        unsigned char static3[3] = {5};
        int main(void)
        {
            if (static1[0] != 1 || static1[1] != 2 || static1[2] || static1[3])
                return 1;
            if (static2[0] != 3 || static2[1] != 4 || static2[2] || static2[3])
                return 2;
            if (static3[0] != 5 || static3[1] || static3[2])
                return 3;
            char auto1[5] = {-4, 66, 4.0};
            signed char auto2[3] = {static1[2], -static1[0]};
            unsigned char auto3[2] = {'a'};
            if (auto1[0] != -4 || auto1[1] != 66 || auto1[2] != 4 || auto1[3] || auto1[4])
                return 4;
            if (auto2[0] || auto2[1] != -1 || auto2[2])
                return 5;
            if (auto3[0] != 'a' || auto3[1])
                return 6;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &static1
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 != 1
            if tmp.5 jump or_true_0
            tmp.8 = &static1
            tmp.9 = sign_extend 1
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=1)
            tmp.11 = *tmp.10
            tmp.12 = sign_extend tmp.11
            tmp.13 = tmp.12 != 2
            if tmp.13 jump or_true_0
            tmp.7 = 0
            jump or_end_1
        
          or_true_0:
            tmp.7 = 1
        
          or_end_1:
            if tmp.7 jump or_true_2
            tmp.16 = &static1
            tmp.17 = sign_extend 2
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            if tmp.19 jump or_true_2
            tmp.15 = 0
            jump or_end_3
        
          or_true_2:
            tmp.15 = 1
        
          or_end_3:
            if tmp.15 jump or_true_4
            tmp.22 = &static1
            tmp.23 = sign_extend 3
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=1)
            tmp.25 = *tmp.24
            if tmp.25 jump or_true_4
            tmp.21 = 0
            jump or_end_5
        
          or_true_4:
            tmp.21 = 1
        
          or_end_5:
            if !tmp.21 jump end_if_6
            return 1
        
          end_if_6:
            tmp.26 = &static2
            tmp.27 = sign_extend 0
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=1)
            tmp.29 = *tmp.28
            tmp.30 = sign_extend tmp.29
            tmp.31 = tmp.30 != 3
            if tmp.31 jump or_true_8
            tmp.34 = &static2
            tmp.35 = sign_extend 1
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=1)
            tmp.37 = *tmp.36
            tmp.38 = sign_extend tmp.37
            tmp.39 = tmp.38 != 4
            if tmp.39 jump or_true_8
            tmp.33 = 0
            jump or_end_9
        
          or_true_8:
            tmp.33 = 1
        
          or_end_9:
            if tmp.33 jump or_true_10
            tmp.42 = &static2
            tmp.43 = sign_extend 2
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=1)
            tmp.45 = *tmp.44
            if tmp.45 jump or_true_10
            tmp.41 = 0
            jump or_end_11
        
          or_true_10:
            tmp.41 = 1
        
          or_end_11:
            if tmp.41 jump or_true_12
            tmp.48 = &static2
            tmp.49 = sign_extend 3
            tmp.50 = add_ptr(tmp.48, index=tmp.49, scale=1)
            tmp.51 = *tmp.50
            if tmp.51 jump or_true_12
            tmp.47 = 0
            jump or_end_13
        
          or_true_12:
            tmp.47 = 1
        
          or_end_13:
            if !tmp.47 jump end_if_14
            return 2
        
          end_if_14:
            tmp.52 = &static3
            tmp.53 = sign_extend 0
            tmp.54 = add_ptr(tmp.52, index=tmp.53, scale=1)
            tmp.55 = *tmp.54
            tmp.56 = zero_extend tmp.55
            tmp.57 = tmp.56 != 5
            if tmp.57 jump or_true_16
            tmp.60 = &static3
            tmp.61 = sign_extend 1
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=1)
            tmp.63 = *tmp.62
            if tmp.63 jump or_true_16
            tmp.59 = 0
            jump or_end_17
        
          or_true_16:
            tmp.59 = 1
        
          or_end_17:
            if tmp.59 jump or_true_18
            tmp.66 = &static3
            tmp.67 = sign_extend 2
            tmp.68 = add_ptr(tmp.66, index=tmp.67, scale=1)
            tmp.69 = *tmp.68
            if tmp.69 jump or_true_18
            tmp.65 = 0
            jump or_end_19
        
          or_true_18:
            tmp.65 = 1
        
          or_end_19:
            if !tmp.65 jump end_if_20
            return 3
        
          end_if_20:
            tmp.70 = - 4
            tmp.71 = truncate tmp.70
            auto1.0[0] = tmp.71
            tmp.72 = truncate 66
            auto1.0[1] = tmp.72
            tmp.73 = double_to_int 4D
            auto1.0[2] = tmp.73
            auto1.0[3] = '\0'
            auto1.0[4] = '\0'
            tmp.74 = &static1
            tmp.75 = sign_extend 2
            tmp.76 = add_ptr(tmp.74, index=tmp.75, scale=1)
            tmp.77 = *tmp.76
            tmp.78 = tmp.77
            auto2.1[0] = tmp.78
            tmp.79 = &static1
            tmp.80 = sign_extend 0
            tmp.81 = add_ptr(tmp.79, index=tmp.80, scale=1)
            tmp.82 = *tmp.81
            tmp.83 = sign_extend tmp.82
            tmp.84 = - tmp.83
            tmp.85 = truncate tmp.84
            auto2.1[1] = tmp.85
            auto2.1[2] = '\0'
            tmp.86 = truncate 97
            auto3.2[0] = tmp.86
            auto3.2[1] = 0UC
            tmp.87 = &auto1.0
            tmp.88 = sign_extend 0
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=1)
            tmp.90 = *tmp.89
            tmp.91 = sign_extend tmp.90
            tmp.93 = - 4
            tmp.92 = tmp.91 != tmp.93
            if tmp.92 jump or_true_22
            tmp.96 = &auto1.0
            tmp.97 = sign_extend 1
            tmp.98 = add_ptr(tmp.96, index=tmp.97, scale=1)
            tmp.99 = *tmp.98
            tmp.100 = sign_extend tmp.99
            tmp.101 = tmp.100 != 66
            if tmp.101 jump or_true_22
            tmp.95 = 0
            jump or_end_23
        
          or_true_22:
            tmp.95 = 1
        
          or_end_23:
            if tmp.95 jump or_true_24
            tmp.104 = &auto1.0
            tmp.105 = sign_extend 2
            tmp.106 = add_ptr(tmp.104, index=tmp.105, scale=1)
            tmp.107 = *tmp.106
            tmp.108 = sign_extend tmp.107
            tmp.109 = tmp.108 != 4
            if tmp.109 jump or_true_24
            tmp.103 = 0
            jump or_end_25
        
          or_true_24:
            tmp.103 = 1
        
          or_end_25:
            if tmp.103 jump or_true_26
            tmp.112 = &auto1.0
            tmp.113 = sign_extend 3
            tmp.114 = add_ptr(tmp.112, index=tmp.113, scale=1)
            tmp.115 = *tmp.114
            if tmp.115 jump or_true_26
            tmp.111 = 0
            jump or_end_27
        
          or_true_26:
            tmp.111 = 1
        
          or_end_27:
            if tmp.111 jump or_true_28
            tmp.118 = &auto1.0
            tmp.119 = sign_extend 4
            tmp.120 = add_ptr(tmp.118, index=tmp.119, scale=1)
            tmp.121 = *tmp.120
            if tmp.121 jump or_true_28
            tmp.117 = 0
            jump or_end_29
        
          or_true_28:
            tmp.117 = 1
        
          or_end_29:
            if !tmp.117 jump end_if_30
            return 4
        
          end_if_30:
            tmp.122 = &auto2.1
            tmp.123 = sign_extend 0
            tmp.124 = add_ptr(tmp.122, index=tmp.123, scale=1)
            tmp.125 = *tmp.124
            if tmp.125 jump or_true_32
            tmp.128 = &auto2.1
            tmp.129 = sign_extend 1
            tmp.130 = add_ptr(tmp.128, index=tmp.129, scale=1)
            tmp.131 = *tmp.130
            tmp.132 = sign_extend tmp.131
            tmp.134 = - 1
            tmp.133 = tmp.132 != tmp.134
            if tmp.133 jump or_true_32
            tmp.127 = 0
            jump or_end_33
        
          or_true_32:
            tmp.127 = 1
        
          or_end_33:
            if tmp.127 jump or_true_34
            tmp.137 = &auto2.1
            tmp.138 = sign_extend 2
            tmp.139 = add_ptr(tmp.137, index=tmp.138, scale=1)
            tmp.140 = *tmp.139
            if tmp.140 jump or_true_34
            tmp.136 = 0
            jump or_end_35
        
          or_true_34:
            tmp.136 = 1
        
          or_end_35:
            if !tmp.136 jump end_if_36
            return 5
        
          end_if_36:
            tmp.141 = &auto3.2
            tmp.142 = sign_extend 0
            tmp.143 = add_ptr(tmp.141, index=tmp.142, scale=1)
            tmp.144 = *tmp.143
            tmp.145 = zero_extend tmp.144
            tmp.146 = tmp.145 != 97
            if tmp.146 jump or_true_38
            tmp.149 = &auto3.2
            tmp.150 = sign_extend 1
            tmp.151 = add_ptr(tmp.149, index=tmp.150, scale=1)
            tmp.152 = *tmp.151
            if tmp.152 jump or_true_38
            tmp.148 = 0
            jump or_end_39
        
          or_true_38:
            tmp.148 = 1
        
          or_end_39:
            if !tmp.148 jump end_if_40
            return 6
        
          end_if_40:
            return 0
            return 0
        }
        static global static1: Array(4,Char) = [ '\u{1}', '\u{2}', zero[2]]
        static global static2: Array(4,Signed Char) = [ '\u{3}', '\u{4}', zero[2]]
        static global static3: Array(3,Unsigned Char) = [ 5UC, zero[2]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_push_arg_on_page_boundary() {
    let src = r#"
        extern char zed;
        int foo(int a, int b, int c, int d, int e, int f, char g) {
            return g + 1;
        }
        int main(void) {
            return foo(0, 0, 0, 0, 0, 0, zed);
        }
    "#;
    let expected = r#"
        global function foo(a.0, b.1, c.2, d.3, e.4, f.5, g.6) { 
            tmp.0 = sign_extend g.6
            tmp.1 = tmp.0 + 1
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = foo(0, 0, 0, 0, 0, 0, zed)
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_return_char() {
    let src = r#"
        char return_char(void) {
            return 5369233654l;
        }
        signed char return_schar(void) {
            return 5369233654l;
        }
        unsigned char return_uchar(void) {
            return 5369233654l;
        }
        int main(void) {
            char char_array[3] = {121, -122, -3};
            char retval_c = return_char();
            char char_array2[3] = {-5, 88, -100};
            signed char retval_sc = return_schar();
            char char_array3[3] = {10, 11, 12};
            unsigned char retval_uc = return_uchar();
            char char_array4[2] = {-5, -6};
            if (char_array[0] != 121 || char_array[1] != -122 || char_array[2] != -3) {
                return 1;
            }
            if (retval_c != -10) {
                return 2;
            }
            if (char_array2[0] != -5 || char_array2[1] != 88 ||
                char_array2[2] != -100) {
                return 3;
            }
            if (retval_sc != -10) {
                return 4;
            }
            if (char_array3[0] != 10 || char_array3[1] != 11 || char_array3[2] != 12) {
                return 5;
            }
            if (retval_uc != 246) {
                return 6;
            }
            if (char_array4[0] != -5 || char_array4[1] != -6) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_char() { 
            tmp.0 = truncate 5369233654L
            return tmp.0
            return 0
        }
        global function return_schar() { 
            tmp.1 = truncate 5369233654L
            return tmp.1
            return 0
        }
        global function return_uchar() { 
            tmp.2 = truncate 5369233654L
            return tmp.2
            return 0
        }
        global function main() { 
            tmp.3 = truncate 121
            char_array.0[0] = tmp.3
            tmp.4 = - 122
            tmp.5 = truncate tmp.4
            char_array.0[1] = tmp.5
            tmp.6 = - 3
            tmp.7 = truncate tmp.6
            char_array.0[2] = tmp.7
            tmp.8 = return_char()
            retval_c.1 = tmp.8
            tmp.9 = - 5
            tmp.10 = truncate tmp.9
            char_array2.2[0] = tmp.10
            tmp.11 = truncate 88
            char_array2.2[1] = tmp.11
            tmp.12 = - 100
            tmp.13 = truncate tmp.12
            char_array2.2[2] = tmp.13
            tmp.14 = return_schar()
            retval_sc.3 = tmp.14
            tmp.15 = truncate 10
            char_array3.4[0] = tmp.15
            tmp.16 = truncate 11
            char_array3.4[1] = tmp.16
            tmp.17 = truncate 12
            char_array3.4[2] = tmp.17
            tmp.18 = return_uchar()
            retval_uc.5 = tmp.18
            tmp.19 = - 5
            tmp.20 = truncate tmp.19
            char_array4.6[0] = tmp.20
            tmp.21 = - 6
            tmp.22 = truncate tmp.21
            char_array4.6[1] = tmp.22
            tmp.23 = &char_array.0
            tmp.24 = sign_extend 0
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = sign_extend tmp.26
            tmp.28 = tmp.27 != 121
            if tmp.28 jump or_true_0
            tmp.31 = &char_array.0
            tmp.32 = sign_extend 1
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=1)
            tmp.34 = *tmp.33
            tmp.35 = sign_extend tmp.34
            tmp.37 = - 122
            tmp.36 = tmp.35 != tmp.37
            if tmp.36 jump or_true_0
            tmp.30 = 0
            jump or_end_1
        
          or_true_0:
            tmp.30 = 1
        
          or_end_1:
            if tmp.30 jump or_true_2
            tmp.40 = &char_array.0
            tmp.41 = sign_extend 2
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=1)
            tmp.43 = *tmp.42
            tmp.44 = sign_extend tmp.43
            tmp.46 = - 3
            tmp.45 = tmp.44 != tmp.46
            if tmp.45 jump or_true_2
            tmp.39 = 0
            jump or_end_3
        
          or_true_2:
            tmp.39 = 1
        
          or_end_3:
            if !tmp.39 jump end_if_4
            return 1
        
          end_if_4:
            tmp.47 = sign_extend retval_c.1
            tmp.49 = - 10
            tmp.48 = tmp.47 != tmp.49
            if !tmp.48 jump end_if_6
            return 2
        
          end_if_6:
            tmp.50 = &char_array2.2
            tmp.51 = sign_extend 0
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=1)
            tmp.53 = *tmp.52
            tmp.54 = sign_extend tmp.53
            tmp.56 = - 5
            tmp.55 = tmp.54 != tmp.56
            if tmp.55 jump or_true_8
            tmp.59 = &char_array2.2
            tmp.60 = sign_extend 1
            tmp.61 = add_ptr(tmp.59, index=tmp.60, scale=1)
            tmp.62 = *tmp.61
            tmp.63 = sign_extend tmp.62
            tmp.64 = tmp.63 != 88
            if tmp.64 jump or_true_8
            tmp.58 = 0
            jump or_end_9
        
          or_true_8:
            tmp.58 = 1
        
          or_end_9:
            if tmp.58 jump or_true_10
            tmp.67 = &char_array2.2
            tmp.68 = sign_extend 2
            tmp.69 = add_ptr(tmp.67, index=tmp.68, scale=1)
            tmp.70 = *tmp.69
            tmp.71 = sign_extend tmp.70
            tmp.73 = - 100
            tmp.72 = tmp.71 != tmp.73
            if tmp.72 jump or_true_10
            tmp.66 = 0
            jump or_end_11
        
          or_true_10:
            tmp.66 = 1
        
          or_end_11:
            if !tmp.66 jump end_if_12
            return 3
        
          end_if_12:
            tmp.74 = sign_extend retval_sc.3
            tmp.76 = - 10
            tmp.75 = tmp.74 != tmp.76
            if !tmp.75 jump end_if_14
            return 4
        
          end_if_14:
            tmp.77 = &char_array3.4
            tmp.78 = sign_extend 0
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=1)
            tmp.80 = *tmp.79
            tmp.81 = sign_extend tmp.80
            tmp.82 = tmp.81 != 10
            if tmp.82 jump or_true_16
            tmp.85 = &char_array3.4
            tmp.86 = sign_extend 1
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=1)
            tmp.88 = *tmp.87
            tmp.89 = sign_extend tmp.88
            tmp.90 = tmp.89 != 11
            if tmp.90 jump or_true_16
            tmp.84 = 0
            jump or_end_17
        
          or_true_16:
            tmp.84 = 1
        
          or_end_17:
            if tmp.84 jump or_true_18
            tmp.93 = &char_array3.4
            tmp.94 = sign_extend 2
            tmp.95 = add_ptr(tmp.93, index=tmp.94, scale=1)
            tmp.96 = *tmp.95
            tmp.97 = sign_extend tmp.96
            tmp.98 = tmp.97 != 12
            if tmp.98 jump or_true_18
            tmp.92 = 0
            jump or_end_19
        
          or_true_18:
            tmp.92 = 1
        
          or_end_19:
            if !tmp.92 jump end_if_20
            return 5
        
          end_if_20:
            tmp.99 = zero_extend retval_uc.5
            tmp.100 = tmp.99 != 246
            if !tmp.100 jump end_if_22
            return 6
        
          end_if_22:
            tmp.101 = &char_array4.6
            tmp.102 = sign_extend 0
            tmp.103 = add_ptr(tmp.101, index=tmp.102, scale=1)
            tmp.104 = *tmp.103
            tmp.105 = sign_extend tmp.104
            tmp.107 = - 5
            tmp.106 = tmp.105 != tmp.107
            if tmp.106 jump or_true_24
            tmp.110 = &char_array4.6
            tmp.111 = sign_extend 1
            tmp.112 = add_ptr(tmp.110, index=tmp.111, scale=1)
            tmp.113 = *tmp.112
            tmp.114 = sign_extend tmp.113
            tmp.116 = - 6
            tmp.115 = tmp.114 != tmp.116
            if tmp.115 jump or_true_24
            tmp.109 = 0
            jump or_end_25
        
          or_true_24:
            tmp.109 = 1
        
          or_end_25:
            if !tmp.109 jump end_if_26
            return 7
        
          end_if_26:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_rewrite_movz_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        unsigned char glob = 5;
        int main(void) {
            int should_spill = (int)glob;
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
            int thirteen = 8 + glob;
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
            if (should_spill != 5) {
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
            tmp.1 = zero_extend glob
            tmp.2 = tmp.1 - 4
            one.14 = tmp.2
            tmp.3 = one.14 + one.14
            two.15 = tmp.3
            tmp.4 = 2 + one.14
            three.16 = tmp.4
            tmp.5 = two.15 * two.15
            four.17 = tmp.5
            tmp.6 = 6 - one.14
            five.18 = tmp.6
            tmp.7 = two.15 * three.16
            six.19 = tmp.7
            tmp.8 = one.14 + 6
            seven.20 = tmp.8
            tmp.9 = two.15 * 4
            eight.21 = tmp.9
            tmp.10 = three.16 * three.16
            nine.22 = tmp.10
            tmp.11 = four.17 + six.19
            ten.23 = tmp.11
            tmp.12 = 16 - five.18
            eleven.24 = tmp.12
            tmp.13 = six.19 + six.19
            twelve.25 = tmp.13
            tmp.14 = check_12_ints(one.14, two.15, three.16, four.17, five.18, six.19, seven.20, eight.21, nine.22, ten.23, eleven.24, twelve.25, 1)
            tmp.16 = zero_extend glob
            tmp.15 = 8 + tmp.16
            thirteen.26 = tmp.15
            tmp.17 = thirteen.26 + 1
            fourteen.27 = tmp.17
            tmp.18 = 28 - thirteen.26
            fifteen.28 = tmp.18
            tmp.19 = fourteen.27 + 2
            sixteen.29 = tmp.19
            tmp.20 = 4 + thirteen.26
            seventeen.30 = tmp.20
            tmp.21 = 32 - fourteen.27
            eighteen.31 = tmp.21
            tmp.22 = 35 - sixteen.29
            nineteen.32 = tmp.22
            tmp.23 = fifteen.28 + 5
            twenty.33 = tmp.23
            tmp.24 = thirteen.26 * 2
            tmp.25 = tmp.24 - 5
            twenty_one.34 = tmp.25
            tmp.26 = fifteen.28 + 7
            twenty_two.35 = tmp.26
            tmp.27 = 6 + seventeen.30
            twenty_three.36 = tmp.27
            tmp.28 = thirteen.26 + 11
            twenty_four.37 = tmp.28
            tmp.29 = check_12_ints(thirteen.26, fourteen.27, fifteen.28, sixteen.29, seventeen.30, eighteen.31, nineteen.32, twenty.33, twenty_one.34, twenty_two.35, twenty_three.36, twenty_four.37, 13)
            tmp.30 = should_spill.13 != 5
            if !tmp.30 jump end_if_0
            tmp.31 = - 1
            return tmp.31
        
          end_if_0:
            return 0
            return 0
        }
        global function check_12_ints(a.38, b.39, c.40, d.41, e.42, f.43, g.44, h.45, i.46, j.47, k.48, l.49, start.50) { 
            expected.51 = 0
            tmp.32 = start.50 + 0
            expected.51 = tmp.32
            tmp.33 = a.38 != expected.51
            if !tmp.33 jump end_if_2
            return expected.51
        
          end_if_2:
            tmp.34 = start.50 + 1
            expected.51 = tmp.34
            tmp.35 = b.39 != expected.51
            if !tmp.35 jump end_if_4
            return expected.51
        
          end_if_4:
            tmp.36 = start.50 + 2
            expected.51 = tmp.36
            tmp.37 = c.40 != expected.51
            if !tmp.37 jump end_if_6
            return expected.51
        
          end_if_6:
            tmp.38 = start.50 + 3
            expected.51 = tmp.38
            tmp.39 = d.41 != expected.51
            if !tmp.39 jump end_if_8
            return expected.51
        
          end_if_8:
            tmp.40 = start.50 + 4
            expected.51 = tmp.40
            tmp.41 = e.42 != expected.51
            if !tmp.41 jump end_if_10
            return expected.51
        
          end_if_10:
            tmp.42 = start.50 + 5
            expected.51 = tmp.42
            tmp.43 = f.43 != expected.51
            if !tmp.43 jump end_if_12
            return expected.51
        
          end_if_12:
            tmp.44 = start.50 + 6
            expected.51 = tmp.44
            tmp.45 = g.44 != expected.51
            if !tmp.45 jump end_if_14
            return expected.51
        
          end_if_14:
            tmp.46 = start.50 + 7
            expected.51 = tmp.46
            tmp.47 = h.45 != expected.51
            if !tmp.47 jump end_if_16
            return expected.51
        
          end_if_16:
            tmp.48 = start.50 + 8
            expected.51 = tmp.48
            tmp.49 = i.46 != expected.51
            if !tmp.49 jump end_if_18
            return expected.51
        
          end_if_18:
            tmp.50 = start.50 + 9
            expected.51 = tmp.50
            tmp.51 = j.47 != expected.51
            if !tmp.51 jump end_if_20
            return expected.51
        
          end_if_20:
            tmp.52 = start.50 + 10
            expected.51 = tmp.52
            tmp.53 = k.48 != expected.51
            if !tmp.53 jump end_if_22
            return expected.51
        
          end_if_22:
            tmp.54 = start.50 + 11
            expected.51 = tmp.54
            tmp.55 = l.49 != expected.51
            if !tmp.55 jump end_if_24
            return expected.51
        
          end_if_24:
            return 0
            return 0
        }
        static global glob: Unsigned Char = 5UC
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_static_initializers() {
    let src = r#"
        char from_long = 17592186044416l;
        char from_double = 15.6;
        char from_uint = 2147483777u;
        char from_ulong = 9223372037928517642ul;
        signed char schar_from_long = 17592186044419l;
        signed char schar_from_uint = 2147483898u;
        signed char schar_from_ulong = 9223372037928517642ul;
        signed char schar_from_double = 1e-10;
        unsigned char uchar_from_int = 13526;
        unsigned char uchar_from_uint = 2147483898u;
        unsigned char uchar_from_long = 1101659111674l;
        unsigned char uchar_from_ulong = 9223372037928517642ul;
        unsigned char uchar_from_double = 77.7;
        int main(void) {
            if (from_long != 0) {
                return 1;
            }
            if (from_double != 15) {
                return 2;
            }
            if (from_uint != -127) {
                return 3;
            }
            if (from_ulong != 10) {
                return 4;
            }
            if (schar_from_uint != -6) {
                return 5;
            }
            if (schar_from_ulong != 10) {
                return 6;
            }
            if (schar_from_double != 0) {
                return 7;
            }
            if (uchar_from_int != 214) {
                return 8;
            }
            if (uchar_from_uint != 250) {
                return 9;
            }
            if (uchar_from_ulong != 10) {
                return 10;
            }
            if (uchar_from_double != 77) {
                return 11;
            }
            if (schar_from_long != 3) {
                return 12;
            }
            if (uchar_from_long != 250) {
                return 13;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend from_long
            tmp.1 = tmp.0 != 0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = sign_extend from_double
            tmp.3 = tmp.2 != 15
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = sign_extend from_uint
            tmp.6 = - 127
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend from_ulong
            tmp.8 = tmp.7 != 10
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend schar_from_uint
            tmp.11 = - 6
            tmp.10 = tmp.9 != tmp.11
            if !tmp.10 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = sign_extend schar_from_ulong
            tmp.13 = tmp.12 != 10
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            tmp.14 = sign_extend schar_from_double
            tmp.15 = tmp.14 != 0
            if !tmp.15 jump end_if_12
            return 7
        
          end_if_12:
            tmp.16 = zero_extend uchar_from_int
            tmp.17 = tmp.16 != 214
            if !tmp.17 jump end_if_14
            return 8
        
          end_if_14:
            tmp.18 = zero_extend uchar_from_uint
            tmp.19 = tmp.18 != 250
            if !tmp.19 jump end_if_16
            return 9
        
          end_if_16:
            tmp.20 = zero_extend uchar_from_ulong
            tmp.21 = tmp.20 != 10
            if !tmp.21 jump end_if_18
            return 10
        
          end_if_18:
            tmp.22 = zero_extend uchar_from_double
            tmp.23 = tmp.22 != 77
            if !tmp.23 jump end_if_20
            return 11
        
          end_if_20:
            tmp.24 = sign_extend schar_from_long
            tmp.25 = tmp.24 != 3
            if !tmp.25 jump end_if_22
            return 12
        
          end_if_22:
            tmp.26 = zero_extend uchar_from_long
            tmp.27 = tmp.26 != 250
            if !tmp.27 jump end_if_24
            return 13
        
          end_if_24:
            return 0
            return 0
        }
        static global from_double: Char = '\u{f}'
        static global from_long: Char = '\0'
        static global from_uint: Char = '\u{81}'
        static global from_ulong: Char = '\n'
        static global schar_from_double: Signed Char = '\0'
        static global schar_from_long: Signed Char = '\u{3}'
        static global schar_from_uint: Signed Char = 'ú'
        static global schar_from_ulong: Signed Char = '\n'
        static global uchar_from_double: Unsigned Char = 77UC
        static global uchar_from_int: Unsigned Char = 214UC
        static global uchar_from_long: Unsigned Char = 250UC
        static global uchar_from_uint: Unsigned Char = 250UC
        static global uchar_from_ulong: Unsigned Char = 10UC
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_chars_type_specifiers() {
    let src = r#"
        char signed static a = 10;
        unsigned static char b = 20;
        char c = 30;
        int main(void)
        {
            extern signed char a;
            char unsigned extern b;
            extern char c;
            if (a != 10) {
                return 1;
            }
            if (b != 20) {
                return 2;
            }
            if (c != 30) {
                return 3;
            }
            int loop_counter = 0;
            for (unsigned char d = 0; d < 100; d = d + 1) {
                loop_counter = loop_counter + 1;
            }
            if (loop_counter != 100) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend a
            tmp.1 = tmp.0 != 10
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = zero_extend b
            tmp.3 = tmp.2 != 20
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = sign_extend c
            tmp.5 = tmp.4 != 30
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            loop_counter.3 = 0
            tmp.6 = truncate 0
            d.4 = tmp.6
        
          start_loop_0:
            tmp.7 = zero_extend d.4
            tmp.8 = tmp.7 < 100
            if !tmp.8 jump break_loop_0
            tmp.9 = loop_counter.3 + 1
            loop_counter.3 = tmp.9
        
          continue_loop_0:
            tmp.10 = zero_extend d.4
            tmp.11 = tmp.10 + 1
            tmp.12 = truncate tmp.11
            d.4 = tmp.12
            jump start_loop_0
        
          break_loop_0:
            tmp.13 = loop_counter.3 != 100
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static a: Signed Char = '\n'
        static b: Unsigned Char = 20UC
        static global c: Char = '\u{1e}'
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitshift_chars() {
    let src = r#"
        int main(void) {
            unsigned char uc = 255;
            if ((uc >> 3) != 31) {
                return 2;
            }
            signed char sc = -127;
            char c = 5;
            if ((sc >> c) != -4) {
                return 3;
            }
            if (((-(c << 3ul)) >> 3) != -5) {
                return 4;
            }
            if ((-(uc << 5u) >> 5u) != -255l) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 255
            uc.0 = tmp.0
            tmp.1 = zero_extend uc.0
            tmp.2 = tmp.1 >> 3
            tmp.3 = tmp.2 != 31
            if !tmp.3 jump end_if_0
            return 2
        
          end_if_0:
            tmp.4 = - 127
            tmp.5 = truncate tmp.4
            sc.1 = tmp.5
            tmp.6 = truncate 5
            c.2 = tmp.6
            tmp.7 = sign_extend sc.1
            tmp.8 = tmp.7 >> c.2
            tmp.10 = - 4
            tmp.9 = tmp.8 != tmp.10
            if !tmp.9 jump end_if_2
            return 3
        
          end_if_2:
            tmp.11 = sign_extend c.2
            tmp.12 = tmp.11 << 3UL
            tmp.13 = - tmp.12
            tmp.14 = tmp.13 >> 3
            tmp.16 = - 5
            tmp.15 = tmp.14 != tmp.16
            if !tmp.15 jump end_if_4
            return 4
        
          end_if_4:
            tmp.17 = zero_extend uc.0
            tmp.18 = tmp.17 << 5U
            tmp.19 = - tmp.18
            tmp.20 = tmp.19 >> 5U
            tmp.21 = sign_extend tmp.20
            tmp.23 = - 255L
            tmp.22 = tmp.21 != tmp.23
            if !tmp.22 jump end_if_6
            return 5
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_ops_character_constants() {
    let src = r#"
        int main(void) {
            int x = 10;
            if ((x ^ 'A') != 75) {
                return 1;
            }
            static char c = 132;
            if (('!' | c) != -91) {
                return 2;
            }
            static unsigned long ul = 9259400834947493926ul;
            if ((ul & '~') != 38) {
                return 3;
            }
            if ((ul << ' ') != 4611738958194278400ul) {
                return 4;
            }
            if (('{' >> 3) != 15) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = x.0 ^ 65
            tmp.1 = tmp.0 != 75
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend c.1
            tmp.2 = 33 | tmp.3
            tmp.5 = - 91
            tmp.4 = tmp.2 != tmp.5
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = sign_extend 126
            tmp.6 = ul.2 & tmp.7
            tmp.9 = sign_extend 38
            tmp.8 = tmp.6 != tmp.9
            if !tmp.8 jump end_if_4
            return 3
        
          end_if_4:
            tmp.10 = ul.2 << 32
            tmp.11 = tmp.10 != 4611738958194278400UL
            if !tmp.11 jump end_if_6
            return 4
        
          end_if_6:
            tmp.12 = 123 >> 3
            tmp.13 = tmp.12 != 15
            if !tmp.13 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static c.1: Char = '\u{84}'
        static ul.2: Unsigned Long = 9259400834947493926UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_ops_chars() {
    let src = r#"
        int main(void) {
            unsigned char uc = 135;
            char c = -116;
            if ((uc & c) != 132) {
                return 1;
            }
            if ((uc | c) != -113) {
                return 2;
            }
            if (((c ^ 1001u) | 360l) != 4294966637) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 135
            uc.0 = tmp.0
            tmp.1 = - 116
            tmp.2 = truncate tmp.1
            c.1 = tmp.2
            tmp.3 = zero_extend uc.0
            tmp.5 = sign_extend c.1
            tmp.4 = tmp.3 & tmp.5
            tmp.6 = tmp.4 != 132
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = zero_extend uc.0
            tmp.9 = sign_extend c.1
            tmp.8 = tmp.7 | tmp.9
            tmp.11 = - 113
            tmp.10 = tmp.8 != tmp.11
            if !tmp.10 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = sign_extend c.1
            tmp.13 = tmp.12 ^ 1001U
            tmp.14 = zero_extend tmp.13
            tmp.15 = tmp.14 | 360L
            tmp.16 = tmp.15 != 4294966637L
            if !tmp.16 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_char_consts_as_cases() {
    let src = r#"
        
        int main(void) {
            static int i = 65;
            switch (i) {
                case 100l:
                    return 1;
                case 'A':
                    return 0;
                case 'B':
                    return 2;
                case 2000u:
                    return 3;
                default:
                    return -1;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 100 == i.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 65 == i.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = 66 == i.0
            if tmp.2 jump switch_0_case__3
            tmp.3 = 2000 == i.0
            if tmp.3 jump switch_0_case__4
            jump switch_0_default_5
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 0
        
          switch_0_case__3:
            return 2
        
          switch_0_case__4:
            return 3
        
          switch_0_default_5:
            tmp.4 = - 1
            return tmp.4
        
          break_switch_0:
            return 0
        }
        static i.0: Int = 65
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_chars() {
    let src = r#"
        int main(void) {
            static char c = 100;
            char c2 = 100;
            c += c2;
            if (c != -56) {
                return 1;
            }
            static unsigned char uc = 200;
            c2 = -100;
            uc /= c2;
            if (uc != 254) {
                return 2;
            }
            uc -= 250.0;
            if (uc != 4) {
                 return 3;
            }
            static signed char sc = 70;
            sc = -sc;
            sc *= c;
            if (sc != 80) {
                return 4;
            }
            if ((sc %= c) != 24) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 100
            c2.1 = tmp.0
            tmp.1 = sign_extend c.0
            tmp.3 = sign_extend c2.1
            tmp.2 = tmp.1 + tmp.3
            tmp.4 = truncate tmp.2
            c.0 = tmp.4
            tmp.5 = truncate tmp.4
            tmp.6 = sign_extend c.0
            tmp.8 = - 56
            tmp.7 = tmp.6 != tmp.8
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = - 100
            tmp.10 = truncate tmp.9
            c2.1 = tmp.10
            tmp.11 = zero_extend uc.2
            tmp.13 = sign_extend c2.1
            tmp.12 = tmp.11 / tmp.13
            tmp.14 = truncate tmp.12
            uc.2 = tmp.14
            tmp.15 = truncate tmp.14
            tmp.16 = zero_extend uc.2
            tmp.17 = tmp.16 != 254
            if !tmp.17 jump end_if_2
            return 2
        
          end_if_2:
            tmp.18 = uint_to_double uc.2
            tmp.19 = tmp.18 - 250D
            tmp.20 = double_to_uint tmp.19
            uc.2 = tmp.20
            tmp.21 = double_to_uint tmp.20
            tmp.22 = zero_extend uc.2
            tmp.23 = tmp.22 != 4
            if !tmp.23 jump end_if_4
            return 3
        
          end_if_4:
            tmp.24 = sign_extend sc.3
            tmp.25 = - tmp.24
            tmp.26 = truncate tmp.25
            sc.3 = tmp.26
            tmp.27 = sign_extend sc.3
            tmp.29 = sign_extend c.0
            tmp.28 = tmp.27 * tmp.29
            tmp.30 = truncate tmp.28
            sc.3 = tmp.30
            tmp.31 = truncate tmp.30
            tmp.32 = sign_extend sc.3
            tmp.33 = tmp.32 != 80
            if !tmp.33 jump end_if_6
            return 4
        
          end_if_6:
            tmp.34 = sign_extend sc.3
            tmp.36 = sign_extend c.0
            tmp.35 = tmp.34 % tmp.36
            tmp.37 = truncate tmp.35
            sc.3 = tmp.37
            tmp.38 = truncate tmp.37
            tmp.39 = tmp.38 != 24
            if !tmp.39 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static c.0: Char = 'd'
        static sc.3: Signed Char = 'F'
        static uc.2: Unsigned Char = 200UC
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_bitwise_ops_chars() {
    let src = r#"
        int main(void) {
            signed char arr[5] = {-128, -120, -2, 1, 120};
            unsigned char u_arr[4] = {0, 170, 250, 255};
            arr[0] ^= 12345;
            arr[1] |= u_arr[3];
            arr[2] &= u_arr[1] - (unsigned char) 185;
            arr[3] <<= 7u;
            static long x = 32;
            arr[4] >>= 31;
            u_arr[3] <<= 12;
            u_arr[2] >>= (x - 1);
            u_arr[1] |= -399;
            x = -4296140120l;
            u_arr[0] ^= x;
            if (arr[0] != -71) {
                return 1;
            }
            if (arr[1] != -1) {
                return 2;
            }
            if (arr[2] != -16) {
                return 3;
            }
            if (arr[3] != -128) {
                return 4;
            }
            if (arr[4] != 0) {
                return 5;
            }
            if (u_arr[0] != 168) {
                return 6;
            }
            if (u_arr[1] != 251) {
                return 7;
            }
            if (u_arr[2] != 0) {
                return 8;
            }
            if (u_arr[3] != 0) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 128
            tmp.1 = truncate tmp.0
            arr.0[0] = tmp.1
            tmp.2 = - 120
            tmp.3 = truncate tmp.2
            arr.0[1] = tmp.3
            tmp.4 = - 2
            tmp.5 = truncate tmp.4
            arr.0[2] = tmp.5
            tmp.6 = truncate 1
            arr.0[3] = tmp.6
            tmp.7 = truncate 120
            arr.0[4] = tmp.7
            tmp.8 = truncate 0
            u_arr.1[0] = tmp.8
            tmp.9 = truncate 170
            u_arr.1[1] = tmp.9
            tmp.10 = truncate 250
            u_arr.1[2] = tmp.10
            tmp.11 = truncate 255
            u_arr.1[3] = tmp.11
            tmp.12 = &arr.0
            tmp.13 = sign_extend 0
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = sign_extend tmp.15
            tmp.17 = tmp.16 ^ 12345
            tmp.18 = truncate tmp.17
            *tmp.14 = tmp.18
            tmp.19 = truncate tmp.18
            tmp.20 = &arr.0
            tmp.21 = sign_extend 1
            tmp.22 = add_ptr(tmp.20, index=tmp.21, scale=1)
            tmp.23 = *tmp.22
            tmp.24 = sign_extend tmp.23
            tmp.26 = &u_arr.1
            tmp.27 = sign_extend 3
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=1)
            tmp.29 = *tmp.28
            tmp.30 = zero_extend tmp.29
            tmp.25 = tmp.24 | tmp.30
            tmp.31 = truncate tmp.25
            *tmp.22 = tmp.31
            tmp.32 = truncate tmp.31
            tmp.33 = &arr.0
            tmp.34 = sign_extend 2
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = sign_extend tmp.36
            tmp.39 = &u_arr.1
            tmp.40 = sign_extend 1
            tmp.41 = add_ptr(tmp.39, index=tmp.40, scale=1)
            tmp.42 = *tmp.41
            tmp.43 = zero_extend tmp.42
            tmp.45 = truncate 185
            tmp.46 = zero_extend tmp.45
            tmp.44 = tmp.43 - tmp.46
            tmp.38 = tmp.37 & tmp.44
            tmp.47 = truncate tmp.38
            *tmp.35 = tmp.47
            tmp.48 = truncate tmp.47
            tmp.49 = &arr.0
            tmp.50 = sign_extend 3
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=1)
            tmp.52 = *tmp.51
            tmp.54 = truncate 7U
            tmp.53 = tmp.52 << tmp.54
            *tmp.51 = tmp.53
            tmp.55 = &arr.0
            tmp.56 = sign_extend 4
            tmp.57 = add_ptr(tmp.55, index=tmp.56, scale=1)
            tmp.58 = *tmp.57
            tmp.60 = truncate 31
            tmp.59 = tmp.58 >> tmp.60
            *tmp.57 = tmp.59
            tmp.61 = &u_arr.1
            tmp.62 = sign_extend 3
            tmp.63 = add_ptr(tmp.61, index=tmp.62, scale=1)
            tmp.64 = *tmp.63
            tmp.66 = truncate 12
            tmp.65 = tmp.64 << tmp.66
            *tmp.63 = tmp.65
            tmp.67 = &u_arr.1
            tmp.68 = sign_extend 2
            tmp.69 = add_ptr(tmp.67, index=tmp.68, scale=1)
            tmp.70 = *tmp.69
            tmp.73 = sign_extend 1
            tmp.72 = x.2 - tmp.73
            tmp.74 = truncate tmp.72
            tmp.71 = tmp.70 >> tmp.74
            *tmp.69 = tmp.71
            tmp.75 = &u_arr.1
            tmp.76 = sign_extend 1
            tmp.77 = add_ptr(tmp.75, index=tmp.76, scale=1)
            tmp.78 = *tmp.77
            tmp.79 = zero_extend tmp.78
            tmp.81 = - 399
            tmp.80 = tmp.79 | tmp.81
            tmp.82 = truncate tmp.80
            *tmp.77 = tmp.82
            tmp.83 = truncate tmp.82
            tmp.84 = - 4296140120L
            x.2 = tmp.84
            tmp.85 = &u_arr.1
            tmp.86 = sign_extend 0
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=1)
            tmp.88 = *tmp.87
            tmp.89 = zero_extend tmp.88
            tmp.90 = tmp.89 ^ x.2
            tmp.91 = truncate tmp.90
            *tmp.87 = tmp.91
            tmp.92 = truncate tmp.91
            tmp.93 = &arr.0
            tmp.94 = sign_extend 0
            tmp.95 = add_ptr(tmp.93, index=tmp.94, scale=1)
            tmp.96 = *tmp.95
            tmp.97 = sign_extend tmp.96
            tmp.99 = - 71
            tmp.98 = tmp.97 != tmp.99
            if !tmp.98 jump end_if_0
            return 1
        
          end_if_0:
            tmp.100 = &arr.0
            tmp.101 = sign_extend 1
            tmp.102 = add_ptr(tmp.100, index=tmp.101, scale=1)
            tmp.103 = *tmp.102
            tmp.104 = sign_extend tmp.103
            tmp.106 = - 1
            tmp.105 = tmp.104 != tmp.106
            if !tmp.105 jump end_if_2
            return 2
        
          end_if_2:
            tmp.107 = &arr.0
            tmp.108 = sign_extend 2
            tmp.109 = add_ptr(tmp.107, index=tmp.108, scale=1)
            tmp.110 = *tmp.109
            tmp.111 = sign_extend tmp.110
            tmp.113 = - 16
            tmp.112 = tmp.111 != tmp.113
            if !tmp.112 jump end_if_4
            return 3
        
          end_if_4:
            tmp.114 = &arr.0
            tmp.115 = sign_extend 3
            tmp.116 = add_ptr(tmp.114, index=tmp.115, scale=1)
            tmp.117 = *tmp.116
            tmp.118 = sign_extend tmp.117
            tmp.120 = - 128
            tmp.119 = tmp.118 != tmp.120
            if !tmp.119 jump end_if_6
            return 4
        
          end_if_6:
            tmp.121 = &arr.0
            tmp.122 = sign_extend 4
            tmp.123 = add_ptr(tmp.121, index=tmp.122, scale=1)
            tmp.124 = *tmp.123
            tmp.125 = sign_extend tmp.124
            tmp.126 = tmp.125 != 0
            if !tmp.126 jump end_if_8
            return 5
        
          end_if_8:
            tmp.127 = &u_arr.1
            tmp.128 = sign_extend 0
            tmp.129 = add_ptr(tmp.127, index=tmp.128, scale=1)
            tmp.130 = *tmp.129
            tmp.131 = zero_extend tmp.130
            tmp.132 = tmp.131 != 168
            if !tmp.132 jump end_if_10
            return 6
        
          end_if_10:
            tmp.133 = &u_arr.1
            tmp.134 = sign_extend 1
            tmp.135 = add_ptr(tmp.133, index=tmp.134, scale=1)
            tmp.136 = *tmp.135
            tmp.137 = zero_extend tmp.136
            tmp.138 = tmp.137 != 251
            if !tmp.138 jump end_if_12
            return 7
        
          end_if_12:
            tmp.139 = &u_arr.1
            tmp.140 = sign_extend 2
            tmp.141 = add_ptr(tmp.139, index=tmp.140, scale=1)
            tmp.142 = *tmp.141
            tmp.143 = zero_extend tmp.142
            tmp.144 = tmp.143 != 0
            if !tmp.144 jump end_if_14
            return 8
        
          end_if_14:
            tmp.145 = &u_arr.1
            tmp.146 = sign_extend 3
            tmp.147 = add_ptr(tmp.145, index=tmp.146, scale=1)
            tmp.148 = *tmp.147
            tmp.149 = zero_extend tmp.148
            tmp.150 = tmp.149 != 0
            if !tmp.150 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
        static x.2: Long = 32L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_incr_decr_chars() {
    let src = r#"
        
        int main(void) {
            static char chars[5] = {123, 124, 125, 126, 127};
            if (chars[0]++ != 123) {
                return 1;
            }
            if (chars[1]-- != 124) {
                return 2;
            }
            if (++chars[2] != 126) {
                return 3;
            }
            if (--chars[3] != 125) {
                return 4;
            }
            if (++chars[4] != -128) {
                return 5;
            }
            if (chars[0] != 124) {
                return 6;
            }
            if (chars[1] != 123) {
                return 7;
            }
            if (chars[2] != 126) {
                return 8;
            }
            if (chars[3] != 125) {
                return 9;
            }
            if (chars[4] != -128) {
                return 10;
            }
            signed char c = -128;
            c--;
            if (c != 127) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &chars.0
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = tmp.3
            tmp.5 = inc tmp.3
            *tmp.2 = tmp.5
            tmp.6 = sign_extend tmp.4
            tmp.7 = tmp.6 != 123
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = &chars.0
            tmp.9 = sign_extend 1
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=1)
            tmp.11 = *tmp.10
            tmp.12 = tmp.11
            tmp.13 = dec tmp.11
            *tmp.10 = tmp.13
            tmp.14 = sign_extend tmp.12
            tmp.15 = tmp.14 != 124
            if !tmp.15 jump end_if_2
            return 2
        
          end_if_2:
            tmp.16 = &chars.0
            tmp.17 = sign_extend 2
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            tmp.20 = inc tmp.19
            *tmp.18 = tmp.20
            tmp.21 = sign_extend tmp.20
            tmp.22 = tmp.21 != 126
            if !tmp.22 jump end_if_4
            return 3
        
          end_if_4:
            tmp.23 = &chars.0
            tmp.24 = sign_extend 3
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = dec tmp.26
            *tmp.25 = tmp.27
            tmp.28 = sign_extend tmp.27
            tmp.29 = tmp.28 != 125
            if !tmp.29 jump end_if_6
            return 4
        
          end_if_6:
            tmp.30 = &chars.0
            tmp.31 = sign_extend 4
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=1)
            tmp.33 = *tmp.32
            tmp.34 = inc tmp.33
            *tmp.32 = tmp.34
            tmp.35 = sign_extend tmp.34
            tmp.37 = - 128
            tmp.36 = tmp.35 != tmp.37
            if !tmp.36 jump end_if_8
            return 5
        
          end_if_8:
            tmp.38 = &chars.0
            tmp.39 = sign_extend 0
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=1)
            tmp.41 = *tmp.40
            tmp.42 = sign_extend tmp.41
            tmp.43 = tmp.42 != 124
            if !tmp.43 jump end_if_10
            return 6
        
          end_if_10:
            tmp.44 = &chars.0
            tmp.45 = sign_extend 1
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=1)
            tmp.47 = *tmp.46
            tmp.48 = sign_extend tmp.47
            tmp.49 = tmp.48 != 123
            if !tmp.49 jump end_if_12
            return 7
        
          end_if_12:
            tmp.50 = &chars.0
            tmp.51 = sign_extend 2
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=1)
            tmp.53 = *tmp.52
            tmp.54 = sign_extend tmp.53
            tmp.55 = tmp.54 != 126
            if !tmp.55 jump end_if_14
            return 8
        
          end_if_14:
            tmp.56 = &chars.0
            tmp.57 = sign_extend 3
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=1)
            tmp.59 = *tmp.58
            tmp.60 = sign_extend tmp.59
            tmp.61 = tmp.60 != 125
            if !tmp.61 jump end_if_16
            return 9
        
          end_if_16:
            tmp.62 = &chars.0
            tmp.63 = sign_extend 4
            tmp.64 = add_ptr(tmp.62, index=tmp.63, scale=1)
            tmp.65 = *tmp.64
            tmp.66 = sign_extend tmp.65
            tmp.68 = - 128
            tmp.67 = tmp.66 != tmp.68
            if !tmp.67 jump end_if_18
            return 10
        
          end_if_18:
            tmp.69 = - 128
            tmp.70 = truncate tmp.69
            c.1 = tmp.70
            tmp.71 = c.1
            tmp.72 = dec c.1
            c.1 = tmp.72
            tmp.73 = sign_extend c.1
            tmp.74 = tmp.73 != 127
            if !tmp.74 jump end_if_20
            return 11
        
          end_if_20:
            return 0
            return 0
        }
        static chars.0: Array(5,Char) = [ '{', '|', '}', '~', '\u{7f}']
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_incr_decr_unsigned_chars() {
    let src = r#"
        
        int main(void) {
            unsigned char chars[5] = {0, 2, 4, 253, 255};
            if (chars[0]--) {
                return 1;
            }
            if (chars[1]++ != 2) {
                return 2;
            }
            if (--chars[3] != 252) {
                return 3;
            }
            if (++chars[4] != 0) {
                return 4;
            }
            if (chars[0] != 255) {
                return 5;
            }
            if (chars[1] != 3) {
                return 6;
            }
            if (chars[2] != 4) {
                return 7;
            }
            if (chars[3] != 252) {
                return 8;
            }
            if (chars[4]) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 0
            chars.0[0] = tmp.0
            tmp.1 = truncate 2
            chars.0[1] = tmp.1
            tmp.2 = truncate 4
            chars.0[2] = tmp.2
            tmp.3 = truncate 253
            chars.0[3] = tmp.3
            tmp.4 = truncate 255
            chars.0[4] = tmp.4
            tmp.5 = &chars.0
            tmp.6 = sign_extend 0
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=1)
            tmp.8 = *tmp.7
            tmp.9 = tmp.8
            tmp.10 = dec tmp.8
            *tmp.7 = tmp.10
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = &chars.0
            tmp.12 = sign_extend 1
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=1)
            tmp.14 = *tmp.13
            tmp.15 = tmp.14
            tmp.16 = inc tmp.14
            *tmp.13 = tmp.16
            tmp.17 = zero_extend tmp.15
            tmp.18 = tmp.17 != 2
            if !tmp.18 jump end_if_2
            return 2
        
          end_if_2:
            tmp.19 = &chars.0
            tmp.20 = sign_extend 3
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=1)
            tmp.22 = *tmp.21
            tmp.23 = dec tmp.22
            *tmp.21 = tmp.23
            tmp.24 = zero_extend tmp.23
            tmp.25 = tmp.24 != 252
            if !tmp.25 jump end_if_4
            return 3
        
          end_if_4:
            tmp.26 = &chars.0
            tmp.27 = sign_extend 4
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=1)
            tmp.29 = *tmp.28
            tmp.30 = inc tmp.29
            *tmp.28 = tmp.30
            tmp.31 = zero_extend tmp.30
            tmp.32 = tmp.31 != 0
            if !tmp.32 jump end_if_6
            return 4
        
          end_if_6:
            tmp.33 = &chars.0
            tmp.34 = sign_extend 0
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = zero_extend tmp.36
            tmp.38 = tmp.37 != 255
            if !tmp.38 jump end_if_8
            return 5
        
          end_if_8:
            tmp.39 = &chars.0
            tmp.40 = sign_extend 1
            tmp.41 = add_ptr(tmp.39, index=tmp.40, scale=1)
            tmp.42 = *tmp.41
            tmp.43 = zero_extend tmp.42
            tmp.44 = tmp.43 != 3
            if !tmp.44 jump end_if_10
            return 6
        
          end_if_10:
            tmp.45 = &chars.0
            tmp.46 = sign_extend 2
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=1)
            tmp.48 = *tmp.47
            tmp.49 = zero_extend tmp.48
            tmp.50 = tmp.49 != 4
            if !tmp.50 jump end_if_12
            return 7
        
          end_if_12:
            tmp.51 = &chars.0
            tmp.52 = sign_extend 3
            tmp.53 = add_ptr(tmp.51, index=tmp.52, scale=1)
            tmp.54 = *tmp.53
            tmp.55 = zero_extend tmp.54
            tmp.56 = tmp.55 != 252
            if !tmp.56 jump end_if_14
            return 8
        
          end_if_14:
            tmp.57 = &chars.0
            tmp.58 = sign_extend 4
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=1)
            tmp.60 = *tmp.59
            if !tmp.60 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_promote_switch_cond() {
    let src = r#"
        int main(void) {
            char c = 100;
            switch (c) {
                case 0:
                    return 1;
                case 100:
                    return 0;
                case 356:
                    return 2;
                default:
                    return 3;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 100
            c.0 = tmp.0
            tmp.1 = sign_extend c.0
            tmp.2 = 0 == tmp.1
            if tmp.2 jump switch_0_case__1
            tmp.3 = 100 == tmp.1
            if tmp.3 jump switch_0_case__2
            tmp.4 = 356 == tmp.1
            if tmp.4 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 0
        
          switch_0_case__3:
            return 2
        
          switch_0_default_4:
            return 3
        
          break_switch_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_promote_switch_cond_2() {
    let src = r#"
        int main(void) {
            char c = -56;
            switch (c) {
                case 33554632:
                    return 1;
                default:
                    return 0;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 56
            tmp.1 = truncate tmp.0
            c.0 = tmp.1
            tmp.2 = sign_extend c.0
            tmp.3 = 33554632 == tmp.2
            if tmp.3 jump switch_0_case__1
            jump switch_0_default_2
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_default_2:
            return 0
        
          break_switch_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_on_char_const() {
    let src = r#"
        
        int main(void) {
            switch ('x') {
                case 1:
                    return 1;
                case 2:
                    return 2;
                case 120:
                    return 0;
                default:
                    return -1;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 == 120
            if tmp.0 jump switch_0_case__1
            tmp.1 = 2 == 120
            if tmp.1 jump switch_0_case__2
            tmp.2 = 120 == 120
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 2
        
          switch_0_case__3:
            return 0
        
          switch_0_default_4:
            tmp.3 = - 1
            return tmp.3
        
          break_switch_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_char_arguments() {
    let src = r#"
        int check_args(char a, signed char b, char c, unsigned char d, char e, char f, signed char g, char h) {
            char expected_a = 5;
            signed char expected_b = -12;
            char expected_c = 117;
            unsigned char expected_d = 254;
            char expected_e = 1;
            char expected_f = -20;
            signed char expected_g = 60;
            char expected_h = 100;
            if (expected_a != a) {
             return 1;
            }
            if (expected_b != b) {
             return 2;
            }
            if (expected_c != c) {
             return 3;
            }
            if (expected_d != d) {
             return 4;
            }
            if (expected_e != e) {
             return 5;
            }
            if (expected_f != f) {
             return 6;
            }
            if (expected_g != g) {
             return 7;
            }
            if (expected_h != h) {
             return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function check_args(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7) { 
            tmp.0 = truncate 5
            expected_a.8 = tmp.0
            tmp.1 = - 12
            tmp.2 = truncate tmp.1
            expected_b.9 = tmp.2
            tmp.3 = truncate 117
            expected_c.10 = tmp.3
            tmp.4 = truncate 254
            expected_d.11 = tmp.4
            tmp.5 = truncate 1
            expected_e.12 = tmp.5
            tmp.6 = - 20
            tmp.7 = truncate tmp.6
            expected_f.13 = tmp.7
            tmp.8 = truncate 60
            expected_g.14 = tmp.8
            tmp.9 = truncate 100
            expected_h.15 = tmp.9
            tmp.10 = sign_extend expected_a.8
            tmp.12 = sign_extend a.0
            tmp.11 = tmp.10 != tmp.12
            if !tmp.11 jump end_if_0
            return 1
        
          end_if_0:
            tmp.13 = sign_extend expected_b.9
            tmp.15 = sign_extend b.1
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_2
            return 2
        
          end_if_2:
            tmp.16 = sign_extend expected_c.10
            tmp.18 = sign_extend c.2
            tmp.17 = tmp.16 != tmp.18
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            tmp.19 = zero_extend expected_d.11
            tmp.21 = zero_extend d.3
            tmp.20 = tmp.19 != tmp.21
            if !tmp.20 jump end_if_6
            return 4
        
          end_if_6:
            tmp.22 = sign_extend expected_e.12
            tmp.24 = sign_extend e.4
            tmp.23 = tmp.22 != tmp.24
            if !tmp.23 jump end_if_8
            return 5
        
          end_if_8:
            tmp.25 = sign_extend expected_f.13
            tmp.27 = sign_extend f.5
            tmp.26 = tmp.25 != tmp.27
            if !tmp.26 jump end_if_10
            return 6
        
          end_if_10:
            tmp.28 = sign_extend expected_g.14
            tmp.30 = sign_extend g.6
            tmp.29 = tmp.28 != tmp.30
            if !tmp.29 jump end_if_12
            return 7
        
          end_if_12:
            tmp.31 = sign_extend expected_h.15
            tmp.33 = sign_extend h.7
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_char_arguments_client() {
    let src = r#"
        
        int check_args(char a, signed char b, char c, unsigned char d, char e, char f, signed char g, char h);
        int main(void) {
            char a = 5;
            signed char b = -12;
            char c = 117;
            unsigned char d = 254;
            char e = 1;
            char f = -20;
            signed char g = 60;
            char h = 100;
            return check_args(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 5
            a.8 = tmp.0
            tmp.1 = - 12
            tmp.2 = truncate tmp.1
            b.9 = tmp.2
            tmp.3 = truncate 117
            c.10 = tmp.3
            tmp.4 = truncate 254
            d.11 = tmp.4
            tmp.5 = truncate 1
            e.12 = tmp.5
            tmp.6 = - 20
            tmp.7 = truncate tmp.6
            f.13 = tmp.7
            tmp.8 = truncate 60
            g.14 = tmp.8
            tmp.9 = truncate 100
            h.15 = tmp.9
            tmp.10 = check_args(a.8, b.9, c.10, d.11, e.12, f.13, g.14, h.15)
            return tmp.10
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_global_char() {
    let src = r#"
        char c = 100;
        unsigned char uc = 250;
        signed char sc = 0;
        int update_global_chars(void) {
            c = c + 10;
            uc = uc + 10;
            sc = sc - 10;
            return 0;
        }
    "#;
    let expected = r#"
        global function update_global_chars() { 
            tmp.0 = sign_extend c
            tmp.1 = tmp.0 + 10
            tmp.2 = truncate tmp.1
            c = tmp.2
            tmp.3 = zero_extend uc
            tmp.4 = tmp.3 + 10
            tmp.5 = truncate tmp.4
            uc = tmp.5
            tmp.6 = sign_extend sc
            tmp.7 = tmp.6 - 10
            tmp.8 = truncate tmp.7
            sc = tmp.8
            return 0
            return 0
        }
        static global c: Char = 'd'
        static global sc: Signed Char = '\0'
        static global uc: Unsigned Char = 250UC
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_global_char_client() {
    let src = r#"
        extern char c;
        extern unsigned char uc;
        extern signed char sc;
        int update_global_chars(void);
        int main(void) {
            if (c != 100) {
                return 1;
            }
            if (uc != 250) {
                return 2;
            }
            if (sc != 0) {
                return 3;
            }
            update_global_chars();
            if (c != 110) {
                return 4;
            }
            if (uc != 4) {
                return 5;
            }
            if (sc != -10) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend c
            tmp.1 = tmp.0 != 100
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = zero_extend uc
            tmp.3 = tmp.2 != 250
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = sign_extend sc
            tmp.5 = tmp.4 != 0
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.6 = update_global_chars()
            tmp.7 = sign_extend c
            tmp.8 = tmp.7 != 110
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = zero_extend uc
            tmp.10 = tmp.9 != 4
            if !tmp.10 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = sign_extend sc
            tmp.13 = - 10
            tmp.12 = tmp.11 != tmp.13
            if !tmp.12 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_return_char() {
    let src = r#"
        char return_char(void) {
            return 5369233654l;
        }
        signed char return_schar(void) {
            return 5369233654l;
        }
        unsigned char return_uchar(void) {
            return 5369233654l;
        }
    "#;
    let expected = r#"
        global function return_char() { 
            tmp.0 = truncate 5369233654L
            return tmp.0
            return 0
        }
        global function return_schar() { 
            tmp.1 = truncate 5369233654L
            return tmp.1
            return 0
        }
        global function return_uchar() { 
            tmp.2 = truncate 5369233654L
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_return_char_client() {
    let src = r#"
        char return_char(void);
        signed char return_schar(void);
        unsigned char return_uchar(void);
        int main(void) {
            char char_array[3] = {121, -122, -3};
            char retval_c = return_char();
            char char_array2[3] = {-5, 88, -100};
            signed char retval_sc = return_schar();
            char char_array3[3] = {10, 11, 12};
            unsigned char retval_uc = return_uchar();
            char char_array4[2] = {-5, -6};
            if (char_array[0] != 121 || char_array[1] != -122 || char_array[2] != -3) {
                return 1;
            }
            if (retval_c != -10) {
                return 2;
            }
            if (char_array2[0] != -5 || char_array2[1] != 88 ||
                char_array2[2] != -100) {
                return 3;
            }
            if (retval_sc != -10) {
                return 4;
            }
            if (char_array3[0] != 10 || char_array3[1] != 11 || char_array3[2] != 12) {
                return 5;
            }
            if (retval_uc != 246) {
                return 6;
            }
            if (char_array4[0] != -5 || char_array4[1] != -6) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 121
            char_array.0[0] = tmp.0
            tmp.1 = - 122
            tmp.2 = truncate tmp.1
            char_array.0[1] = tmp.2
            tmp.3 = - 3
            tmp.4 = truncate tmp.3
            char_array.0[2] = tmp.4
            tmp.5 = return_char()
            retval_c.1 = tmp.5
            tmp.6 = - 5
            tmp.7 = truncate tmp.6
            char_array2.2[0] = tmp.7
            tmp.8 = truncate 88
            char_array2.2[1] = tmp.8
            tmp.9 = - 100
            tmp.10 = truncate tmp.9
            char_array2.2[2] = tmp.10
            tmp.11 = return_schar()
            retval_sc.3 = tmp.11
            tmp.12 = truncate 10
            char_array3.4[0] = tmp.12
            tmp.13 = truncate 11
            char_array3.4[1] = tmp.13
            tmp.14 = truncate 12
            char_array3.4[2] = tmp.14
            tmp.15 = return_uchar()
            retval_uc.5 = tmp.15
            tmp.16 = - 5
            tmp.17 = truncate tmp.16
            char_array4.6[0] = tmp.17
            tmp.18 = - 6
            tmp.19 = truncate tmp.18
            char_array4.6[1] = tmp.19
            tmp.20 = &char_array.0
            tmp.21 = sign_extend 0
            tmp.22 = add_ptr(tmp.20, index=tmp.21, scale=1)
            tmp.23 = *tmp.22
            tmp.24 = sign_extend tmp.23
            tmp.25 = tmp.24 != 121
            if tmp.25 jump or_true_0
            tmp.28 = &char_array.0
            tmp.29 = sign_extend 1
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=1)
            tmp.31 = *tmp.30
            tmp.32 = sign_extend tmp.31
            tmp.34 = - 122
            tmp.33 = tmp.32 != tmp.34
            if tmp.33 jump or_true_0
            tmp.27 = 0
            jump or_end_1
        
          or_true_0:
            tmp.27 = 1
        
          or_end_1:
            if tmp.27 jump or_true_2
            tmp.37 = &char_array.0
            tmp.38 = sign_extend 2
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = sign_extend tmp.40
            tmp.43 = - 3
            tmp.42 = tmp.41 != tmp.43
            if tmp.42 jump or_true_2
            tmp.36 = 0
            jump or_end_3
        
          or_true_2:
            tmp.36 = 1
        
          or_end_3:
            if !tmp.36 jump end_if_4
            return 1
        
          end_if_4:
            tmp.44 = sign_extend retval_c.1
            tmp.46 = - 10
            tmp.45 = tmp.44 != tmp.46
            if !tmp.45 jump end_if_6
            return 2
        
          end_if_6:
            tmp.47 = &char_array2.2
            tmp.48 = sign_extend 0
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=1)
            tmp.50 = *tmp.49
            tmp.51 = sign_extend tmp.50
            tmp.53 = - 5
            tmp.52 = tmp.51 != tmp.53
            if tmp.52 jump or_true_8
            tmp.56 = &char_array2.2
            tmp.57 = sign_extend 1
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=1)
            tmp.59 = *tmp.58
            tmp.60 = sign_extend tmp.59
            tmp.61 = tmp.60 != 88
            if tmp.61 jump or_true_8
            tmp.55 = 0
            jump or_end_9
        
          or_true_8:
            tmp.55 = 1
        
          or_end_9:
            if tmp.55 jump or_true_10
            tmp.64 = &char_array2.2
            tmp.65 = sign_extend 2
            tmp.66 = add_ptr(tmp.64, index=tmp.65, scale=1)
            tmp.67 = *tmp.66
            tmp.68 = sign_extend tmp.67
            tmp.70 = - 100
            tmp.69 = tmp.68 != tmp.70
            if tmp.69 jump or_true_10
            tmp.63 = 0
            jump or_end_11
        
          or_true_10:
            tmp.63 = 1
        
          or_end_11:
            if !tmp.63 jump end_if_12
            return 3
        
          end_if_12:
            tmp.71 = sign_extend retval_sc.3
            tmp.73 = - 10
            tmp.72 = tmp.71 != tmp.73
            if !tmp.72 jump end_if_14
            return 4
        
          end_if_14:
            tmp.74 = &char_array3.4
            tmp.75 = sign_extend 0
            tmp.76 = add_ptr(tmp.74, index=tmp.75, scale=1)
            tmp.77 = *tmp.76
            tmp.78 = sign_extend tmp.77
            tmp.79 = tmp.78 != 10
            if tmp.79 jump or_true_16
            tmp.82 = &char_array3.4
            tmp.83 = sign_extend 1
            tmp.84 = add_ptr(tmp.82, index=tmp.83, scale=1)
            tmp.85 = *tmp.84
            tmp.86 = sign_extend tmp.85
            tmp.87 = tmp.86 != 11
            if tmp.87 jump or_true_16
            tmp.81 = 0
            jump or_end_17
        
          or_true_16:
            tmp.81 = 1
        
          or_end_17:
            if tmp.81 jump or_true_18
            tmp.90 = &char_array3.4
            tmp.91 = sign_extend 2
            tmp.92 = add_ptr(tmp.90, index=tmp.91, scale=1)
            tmp.93 = *tmp.92
            tmp.94 = sign_extend tmp.93
            tmp.95 = tmp.94 != 12
            if tmp.95 jump or_true_18
            tmp.89 = 0
            jump or_end_19
        
          or_true_18:
            tmp.89 = 1
        
          or_end_19:
            if !tmp.89 jump end_if_20
            return 5
        
          end_if_20:
            tmp.96 = zero_extend retval_uc.5
            tmp.97 = tmp.96 != 246
            if !tmp.97 jump end_if_22
            return 6
        
          end_if_22:
            tmp.98 = &char_array4.6
            tmp.99 = sign_extend 0
            tmp.100 = add_ptr(tmp.98, index=tmp.99, scale=1)
            tmp.101 = *tmp.100
            tmp.102 = sign_extend tmp.101
            tmp.104 = - 5
            tmp.103 = tmp.102 != tmp.104
            if tmp.103 jump or_true_24
            tmp.107 = &char_array4.6
            tmp.108 = sign_extend 1
            tmp.109 = add_ptr(tmp.107, index=tmp.108, scale=1)
            tmp.110 = *tmp.109
            tmp.111 = sign_extend tmp.110
            tmp.113 = - 6
            tmp.112 = tmp.111 != tmp.113
            if tmp.112 jump or_true_24
            tmp.106 = 0
            jump or_end_25
        
          or_true_24:
            tmp.106 = 1
        
          or_end_25:
            if !tmp.106 jump end_if_26
            return 7
        
          end_if_26:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_adjacent_strings_in_initializer() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int main(void) {
            char multi_string[6] =
                "yes"
                "no";
            char nested_multi_string[2][3] = {
                "a"
                "b",
                "c"
                "d"};
            if (strcmp(multi_string, "yesno"))
                return 1;
            if (strcmp(nested_multi_string[0], "ab"))
                return 2;
            if (strcmp(nested_multi_string[1], "cd"))
                return 3;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            multi_string.2[0] = 'y'
            multi_string.2[1] = 'e'
            multi_string.2[2] = 's'
            multi_string.2[3] = 'n'
            multi_string.2[4] = 'o'
            multi_string.2[5] = '\0'
            nested_multi_string.3[0] = 'a'
            nested_multi_string.3[1] = 'b'
            nested_multi_string.3[2] = '\0'
            nested_multi_string.3[3] = 'c'
            nested_multi_string.3[4] = 'd'
            nested_multi_string.3[5] = '\0'
            tmp.0 = &multi_string.2
            tmp.1 = &string.0
            tmp.2 = strcmp(tmp.0, tmp.1)
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = &nested_multi_string.3
            tmp.4 = sign_extend 0
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=3)
            tmp.6 = &string.1
            tmp.7 = strcmp(tmp.5, tmp.6)
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = &nested_multi_string.3
            tmp.9 = sign_extend 1
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=3)
            tmp.11 = &string.2
            tmp.12 = strcmp(tmp.10, tmp.11)
            if !tmp.12 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        constant string.0: Array(6,Char) = "yesno\\0"
        constant string.1: Array(3,Char) = "ab\\0"
        constant string.2: Array(3,Char) = "cd\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_array_init_special_chars() {
    let src = r#"
        
        int main(void) {
            char special[6] = "\a\b\n	";
            if (special[0] != '\a') {
                return 1;
            }
            if (special[1] != '\b') {
                return 2;
            }
            if (special[2] != '\n') {
                return 3;
            }
            if (special[3] != '\v') {
                return 4;
            }
            if (special[4] != '\f') {
                return 5;
            }
            if (special[5] != '\t') {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            special.0[0] = '\u{7}'
            special.0[1] = '\u{8}'
            special.0[2] = '\n'
            special.0[3] = '\u{b}'
            special.0[4] = '\u{c}'
            special.0[5] = '\t'
            tmp.0 = &special.0
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 != 7
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = &special.0
            tmp.7 = sign_extend 1
            tmp.8 = add_ptr(tmp.6, index=tmp.7, scale=1)
            tmp.9 = *tmp.8
            tmp.10 = sign_extend tmp.9
            tmp.11 = tmp.10 != 8
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = &special.0
            tmp.13 = sign_extend 2
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = sign_extend tmp.15
            tmp.17 = tmp.16 != 10
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            tmp.18 = &special.0
            tmp.19 = sign_extend 3
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=1)
            tmp.21 = *tmp.20
            tmp.22 = sign_extend tmp.21
            tmp.23 = tmp.22 != 11
            if !tmp.23 jump end_if_6
            return 4
        
          end_if_6:
            tmp.24 = &special.0
            tmp.25 = sign_extend 4
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=1)
            tmp.27 = *tmp.26
            tmp.28 = sign_extend tmp.27
            tmp.29 = tmp.28 != 12
            if !tmp.29 jump end_if_8
            return 5
        
          end_if_8:
            tmp.30 = &special.0
            tmp.31 = sign_extend 5
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=1)
            tmp.33 = *tmp.32
            tmp.34 = sign_extend tmp.33
            tmp.35 = tmp.34 != 9
            if !tmp.35 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_literals_and_compound_initializers() {
    let src = r#"
        signed char static_array[3][4] = {{'a', 'b', 'c', 'd'}, "efgh", "ijk"};
        int main(void) {
            unsigned char auto_array[2][3] = {"lmn", {'o', 'p'}};
            for (int i = 0; i < 3; i = i + 1)
                for (int j = 0; j < 4; j = j + 1)
                    if (static_array[i][j] != "abcdefghijk"[i * 4 + j])
                        return 1;
            for (int i = 0; i < 2; i = i + 1)
                for (int j = 0; j < 3; j = j + 1)
                    if (auto_array[i][j] != "lmnop"[i * 3 + j])
                        return 2;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            auto_array.0[0] = 108UC
            auto_array.0[1] = 109UC
            auto_array.0[2] = 110UC
            tmp.0 = truncate 111
            auto_array.0[3] = tmp.0
            tmp.1 = truncate 112
            auto_array.0[4] = tmp.1
            auto_array.0[5] = 0UC
            i.1 = 0
        
          start_loop_0:
            tmp.2 = i.1 < 3
            if !tmp.2 jump break_loop_0
            j.2 = 0
        
          start_loop_1:
            tmp.3 = j.2 < 4
            if !tmp.3 jump break_loop_1
            tmp.4 = &static_array
            tmp.5 = sign_extend i.1
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=4)
            tmp.7 = sign_extend j.2
            tmp.8 = add_ptr(tmp.6, index=tmp.7, scale=1)
            tmp.9 = *tmp.8
            tmp.10 = sign_extend tmp.9
            tmp.12 = &string.0
            tmp.13 = i.1 * 4
            tmp.14 = tmp.13 + j.2
            tmp.15 = sign_extend tmp.14
            tmp.16 = add_ptr(tmp.12, index=tmp.15, scale=1)
            tmp.17 = *tmp.16
            tmp.18 = sign_extend tmp.17
            tmp.11 = tmp.10 != tmp.18
            if !tmp.11 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_1:
            tmp.19 = j.2 + 1
            j.2 = tmp.19
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.20 = i.1 + 1
            i.1 = tmp.20
            jump start_loop_0
        
          break_loop_0:
            i.3 = 0
        
          start_loop_2:
            tmp.21 = i.3 < 2
            if !tmp.21 jump break_loop_2
            j.4 = 0
        
          start_loop_3:
            tmp.22 = j.4 < 3
            if !tmp.22 jump break_loop_3
            tmp.23 = &auto_array.0
            tmp.24 = sign_extend i.3
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=3)
            tmp.26 = sign_extend j.4
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=1)
            tmp.28 = *tmp.27
            tmp.29 = zero_extend tmp.28
            tmp.31 = &string.1
            tmp.32 = i.3 * 3
            tmp.33 = tmp.32 + j.4
            tmp.34 = sign_extend tmp.33
            tmp.35 = add_ptr(tmp.31, index=tmp.34, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = sign_extend tmp.36
            tmp.30 = tmp.29 != tmp.37
            if !tmp.30 jump end_if_2
            return 2
        
          end_if_2:
        
          continue_loop_3:
            tmp.38 = j.4 + 1
            j.4 = tmp.38
            jump start_loop_3
        
          break_loop_3:
        
          continue_loop_2:
            tmp.39 = i.3 + 1
            i.3 = tmp.39
            jump start_loop_2
        
          break_loop_2:
            return 0
            return 0
        }
        static global static_array: Array(3,Array(4,Signed Char)) = [ 'a', 'b', 'c', 'd', "efgh", "ijk\\0"]
        constant string.0: Array(12,Char) = "abcdefghijk\\0"
        constant string.1: Array(6,Char) = "lmnop\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_partial_initialize_via_string() {
    let src = r#"
        static char static_arr[5] = "hi";
        int test_static(void) {
            return (static_arr[0] == 'h' && static_arr[1] == 'i' &&
                    !(static_arr[2] || static_arr[3] || static_arr[4]));
        }
        static signed char nested_static_arr[3][4] = {
            "", "bc"};
        int test_static_nested(void) {
            for (int i = 0; i < 3; i = i + 1)
                for (int j = 0; j < 4; j = j + 1) {
                    signed char c = nested_static_arr[i][j];
                    signed char expected = 0;
                    if (i == 1 && j == 0) {
                        expected = 'b';
                    } else if (i == 1 && j == 1) {
                        expected = 'c';
                    }
                    if (c != expected) {
                        return 0;
                    }
                }
            return 1;
        }
        int test_automatic(void) {
            unsigned char aut[4] = "ab";
            return (aut[0] == 'a' && aut[1] == 'b' && !(aut[2] || aut[3]));
        }
        int test_automatic_nested(void) {
            signed char nested_auto[2][2][4] = {{"foo"}, {"x", "yz"}};
            for (int i = 0; i < 2; i = i + 1) {
                for (int j = 0; j < 2; j = j + 1) {
                    for (int k = 0; k < 4; k = k + 1) {
                        signed char c = nested_auto[i][j][k];
                        signed char expected = 0;
                        if (i == 0 && j == 0) {
                            if (k == 0) {
                                expected = 'f';
                            } else if (k == 1 || k == 2) {
                                expected = 'o';
                            }
                        } else if (i == 1 && j == 0 && k == 0) {
                            expected = 'x';
                        } else if (i == 1 && j == 1 && k == 0) {
                            expected = 'y';
                        } else if (i == 1 && j == 1 && k == 1) {
                            expected = 'z';
                        }
                        if (c != expected) {
                            return 0;
                        }
                    }
                }
            }
            return 1;
        }
        int main(void) {
            if (!test_static()) {
                return 1;
            }
            if (!test_static_nested()) {
                return 2;
            }
            if (!test_automatic()) {
                return 3;
            }
            if (!test_automatic_nested()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_static() { 
            tmp.0 = &static_arr
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 == 104
            if !tmp.5 jump and_false_0
            tmp.8 = &static_arr
            tmp.9 = sign_extend 1
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=1)
            tmp.11 = *tmp.10
            tmp.12 = sign_extend tmp.11
            tmp.13 = tmp.12 == 105
            if !tmp.13 jump and_false_0
            tmp.7 = 1
            jump and_end_1
        
          and_false_0:
            tmp.7 = 0
        
          and_end_1:
            if !tmp.7 jump and_false_2
            tmp.16 = &static_arr
            tmp.17 = sign_extend 2
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            if tmp.19 jump or_true_4
            tmp.22 = &static_arr
            tmp.23 = sign_extend 3
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=1)
            tmp.25 = *tmp.24
            if tmp.25 jump or_true_4
            tmp.21 = 0
            jump or_end_5
        
          or_true_4:
            tmp.21 = 1
        
          or_end_5:
            if tmp.21 jump or_true_6
            tmp.28 = &static_arr
            tmp.29 = sign_extend 4
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=1)
            tmp.31 = *tmp.30
            if tmp.31 jump or_true_6
            tmp.27 = 0
            jump or_end_7
        
          or_true_6:
            tmp.27 = 1
        
          or_end_7:
            tmp.32 = ! tmp.27
            if !tmp.32 jump and_false_2
            tmp.15 = 1
            jump and_end_3
        
          and_false_2:
            tmp.15 = 0
        
          and_end_3:
            return tmp.15
            return 0
        }
        global function test_static_nested() { 
            i.0 = 0
        
          start_loop_0:
            tmp.33 = i.0 < 3
            if !tmp.33 jump break_loop_0
            j.1 = 0
        
          start_loop_1:
            tmp.34 = j.1 < 4
            if !tmp.34 jump break_loop_1
            tmp.35 = &nested_static_arr
            tmp.36 = sign_extend i.0
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=4)
            tmp.38 = sign_extend j.1
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=1)
            tmp.40 = *tmp.39
            c.2 = tmp.40
            tmp.41 = truncate 0
            expected.3 = tmp.41
            tmp.42 = i.0 == 1
            if !tmp.42 jump and_false_8
            tmp.45 = j.1 == 0
            if !tmp.45 jump and_false_8
            tmp.44 = 1
            jump and_end_9
        
          and_false_8:
            tmp.44 = 0
        
          and_end_9:
            if !tmp.44 jump else_11
            tmp.46 = truncate 98
            expected.3 = tmp.46
            jump end_if_10
        
          else_11:
            tmp.47 = i.0 == 1
            if !tmp.47 jump and_false_12
            tmp.50 = j.1 == 1
            if !tmp.50 jump and_false_12
            tmp.49 = 1
            jump and_end_13
        
          and_false_12:
            tmp.49 = 0
        
          and_end_13:
            if !tmp.49 jump end_if_14
            tmp.51 = truncate 99
            expected.3 = tmp.51
        
          end_if_14:
        
          end_if_10:
            tmp.52 = sign_extend c.2
            tmp.54 = sign_extend expected.3
            tmp.53 = tmp.52 != tmp.54
            if !tmp.53 jump end_if_16
            return 0
        
          end_if_16:
        
          continue_loop_1:
            tmp.55 = j.1 + 1
            j.1 = tmp.55
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.56 = i.0 + 1
            i.0 = tmp.56
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
        global function test_automatic() { 
            aut.4[0] = 97UC
            aut.4[1] = 98UC
            aut.4[2] = '\0'
            aut.4[3] = '\0'
            tmp.57 = &aut.4
            tmp.58 = sign_extend 0
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=1)
            tmp.60 = *tmp.59
            tmp.61 = zero_extend tmp.60
            tmp.62 = tmp.61 == 97
            if !tmp.62 jump and_false_18
            tmp.65 = &aut.4
            tmp.66 = sign_extend 1
            tmp.67 = add_ptr(tmp.65, index=tmp.66, scale=1)
            tmp.68 = *tmp.67
            tmp.69 = zero_extend tmp.68
            tmp.70 = tmp.69 == 98
            if !tmp.70 jump and_false_18
            tmp.64 = 1
            jump and_end_19
        
          and_false_18:
            tmp.64 = 0
        
          and_end_19:
            if !tmp.64 jump and_false_20
            tmp.73 = &aut.4
            tmp.74 = sign_extend 2
            tmp.75 = add_ptr(tmp.73, index=tmp.74, scale=1)
            tmp.76 = *tmp.75
            if tmp.76 jump or_true_22
            tmp.79 = &aut.4
            tmp.80 = sign_extend 3
            tmp.81 = add_ptr(tmp.79, index=tmp.80, scale=1)
            tmp.82 = *tmp.81
            if tmp.82 jump or_true_22
            tmp.78 = 0
            jump or_end_23
        
          or_true_22:
            tmp.78 = 1
        
          or_end_23:
            tmp.83 = ! tmp.78
            if !tmp.83 jump and_false_20
            tmp.72 = 1
            jump and_end_21
        
          and_false_20:
            tmp.72 = 0
        
          and_end_21:
            return tmp.72
            return 0
        }
        global function test_automatic_nested() { 
            nested_auto.5[0] = 'f'
            nested_auto.5[1] = 'o'
            nested_auto.5[2] = 'o'
            nested_auto.5[3] = '\0'
            nested_auto.5[4] = '\0'
            nested_auto.5[5] = '\0'
            nested_auto.5[6] = '\0'
            nested_auto.5[7] = '\0'
            nested_auto.5[8] = 'x'
            nested_auto.5[9] = '\0'
            nested_auto.5[10] = '\0'
            nested_auto.5[11] = '\0'
            nested_auto.5[12] = 'y'
            nested_auto.5[13] = 'z'
            nested_auto.5[14] = '\0'
            nested_auto.5[15] = '\0'
            i.6 = 0
        
          start_loop_2:
            tmp.84 = i.6 < 2
            if !tmp.84 jump break_loop_2
            j.7 = 0
        
          start_loop_3:
            tmp.85 = j.7 < 2
            if !tmp.85 jump break_loop_3
            k.8 = 0
        
          start_loop_4:
            tmp.86 = k.8 < 4
            if !tmp.86 jump break_loop_4
            tmp.87 = &nested_auto.5
            tmp.88 = sign_extend i.6
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=8)
            tmp.90 = sign_extend j.7
            tmp.91 = add_ptr(tmp.89, index=tmp.90, scale=4)
            tmp.92 = sign_extend k.8
            tmp.93 = add_ptr(tmp.91, index=tmp.92, scale=1)
            tmp.94 = *tmp.93
            c.9 = tmp.94
            tmp.95 = truncate 0
            expected.10 = tmp.95
            tmp.96 = i.6 == 0
            if !tmp.96 jump and_false_24
            tmp.99 = j.7 == 0
            if !tmp.99 jump and_false_24
            tmp.98 = 1
            jump and_end_25
        
          and_false_24:
            tmp.98 = 0
        
          and_end_25:
            if !tmp.98 jump else_27
            tmp.100 = k.8 == 0
            if !tmp.100 jump else_29
            tmp.101 = truncate 102
            expected.10 = tmp.101
            jump end_if_28
        
          else_29:
            tmp.102 = k.8 == 1
            if tmp.102 jump or_true_30
            tmp.105 = k.8 == 2
            if tmp.105 jump or_true_30
            tmp.104 = 0
            jump or_end_31
        
          or_true_30:
            tmp.104 = 1
        
          or_end_31:
            if !tmp.104 jump end_if_32
            tmp.106 = truncate 111
            expected.10 = tmp.106
        
          end_if_32:
        
          end_if_28:
            jump end_if_26
        
          else_27:
            tmp.107 = i.6 == 1
            if !tmp.107 jump and_false_34
            tmp.110 = j.7 == 0
            if !tmp.110 jump and_false_34
            tmp.109 = 1
            jump and_end_35
        
          and_false_34:
            tmp.109 = 0
        
          and_end_35:
            if !tmp.109 jump and_false_36
            tmp.113 = k.8 == 0
            if !tmp.113 jump and_false_36
            tmp.112 = 1
            jump and_end_37
        
          and_false_36:
            tmp.112 = 0
        
          and_end_37:
            if !tmp.112 jump else_39
            tmp.114 = truncate 120
            expected.10 = tmp.114
            jump end_if_38
        
          else_39:
            tmp.115 = i.6 == 1
            if !tmp.115 jump and_false_40
            tmp.118 = j.7 == 1
            if !tmp.118 jump and_false_40
            tmp.117 = 1
            jump and_end_41
        
          and_false_40:
            tmp.117 = 0
        
          and_end_41:
            if !tmp.117 jump and_false_42
            tmp.121 = k.8 == 0
            if !tmp.121 jump and_false_42
            tmp.120 = 1
            jump and_end_43
        
          and_false_42:
            tmp.120 = 0
        
          and_end_43:
            if !tmp.120 jump else_45
            tmp.122 = truncate 121
            expected.10 = tmp.122
            jump end_if_44
        
          else_45:
            tmp.123 = i.6 == 1
            if !tmp.123 jump and_false_46
            tmp.126 = j.7 == 1
            if !tmp.126 jump and_false_46
            tmp.125 = 1
            jump and_end_47
        
          and_false_46:
            tmp.125 = 0
        
          and_end_47:
            if !tmp.125 jump and_false_48
            tmp.129 = k.8 == 1
            if !tmp.129 jump and_false_48
            tmp.128 = 1
            jump and_end_49
        
          and_false_48:
            tmp.128 = 0
        
          and_end_49:
            if !tmp.128 jump end_if_50
            tmp.130 = truncate 122
            expected.10 = tmp.130
        
          end_if_50:
        
          end_if_44:
        
          end_if_38:
        
          end_if_26:
            tmp.131 = sign_extend c.9
            tmp.133 = sign_extend expected.10
            tmp.132 = tmp.131 != tmp.133
            if !tmp.132 jump end_if_52
            return 0
        
          end_if_52:
        
          continue_loop_4:
            tmp.134 = k.8 + 1
            k.8 = tmp.134
            jump start_loop_4
        
          break_loop_4:
        
          continue_loop_3:
            tmp.135 = j.7 + 1
            j.7 = tmp.135
            jump start_loop_3
        
          break_loop_3:
        
          continue_loop_2:
            tmp.136 = i.6 + 1
            i.6 = tmp.136
            jump start_loop_2
        
          break_loop_2:
            return 1
            return 0
        }
        global function main() { 
            tmp.137 = test_static()
            tmp.138 = ! tmp.137
            if !tmp.138 jump end_if_54
            return 1
        
          end_if_54:
            tmp.139 = test_static_nested()
            tmp.140 = ! tmp.139
            if !tmp.140 jump end_if_56
            return 2
        
          end_if_56:
            tmp.141 = test_automatic()
            tmp.142 = ! tmp.141
            if !tmp.142 jump end_if_58
            return 3
        
          end_if_58:
            tmp.143 = test_automatic_nested()
            tmp.144 = ! tmp.143
            if !tmp.144 jump end_if_60
            return 4
        
          end_if_60:
            return 0
            return 0
        }
        static nested_static_arr: Array(3,Array(4,Signed Char)) = [ "\\0", zero[3], "bc\\0", zero[1], zero[4]]
        static static_arr: Array(5,Char) = [ "hi\\0", zero[2]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_simple() {
    let src = r#"
        int main(void) {
            unsigned char chars[4] = "abc";
            return chars[2];
        }
    "#;
    let expected = r#"
        global function main() { 
            chars.0[0] = 97UC
            chars.0[1] = 98UC
            chars.0[2] = 99UC
            chars.0[3] = '\0'
            tmp.0 = &chars.0
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = zero_extend tmp.3
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_terminating_null_bytes() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int test_flat_static_with_null_byte(void) {
            static unsigned char flat[4] = "dog";
            return (flat[0] == 'd' && flat[1] == 'o' && flat[2] == 'g' && flat[3] == 0);
        }
        int test_nested_static_with_null_byte(void) {
            static char nested[2][4] = {"yes", "yup"};
            return (nested[0][0] == 'y' && nested[0][1] == 'e' && nested[0][2] == 's' &&
                    nested[0][3] == 0 && nested[1][0] == 'y' && nested[1][1] == 'u' &&
                    nested[1][2] == 'p' && nested[1][3] == 0);
        }
        int test_flat_auto_with_null_byte(void) {
            char flat_auto[2] = "x";
            return (flat_auto[0] == 'x' && flat_auto[1] == 0);
        }
        int test_nested_auto_with_null_byte(void) {
            char nested_auto[2][2][2] = {{"a", "b"}, {"c", "d"}};
            return (nested_auto[0][0][0] == 'a' && nested_auto[0][0][1] == 0 &&
                    nested_auto[0][1][0] == 'b' && nested_auto[0][1][1] == 0 &&
                    nested_auto[1][0][0] == 'c' && nested_auto[1][0][1] == 0 &&
                    nested_auto[1][1][0] == 'd' && nested_auto[1][1][1] == 0);
        }
        int test_flat_static_without_null_byte(void) {
            static char letters[4] = "abcd";
            return letters[0] == 'a' && letters[1] == 'b' && letters[2] == 'c' &&
                   letters[3] == 'd';
        }
        char nested[3][3] = {"yes", "no", "ok"};
        int test_nested_static_without_null_byte(void) {
            char *whole_array = (char *)nested;
            char *word1 = (char *)nested[0];
            char *word2 = (char *)nested[1];
            char *word3 = (char *)nested[2];
            return !(strcmp(whole_array, "yesno") || strcmp(word1, "yesno") ||
                     strcmp(word2, "no") || strcmp(word3, "ok"));
        }
        int test_flat_auto_without_null_byte(void) {
            int x = -1;
            char letters[4] = "abcd";
            int y = -1;
            return (x == -1 && y == -1 && letters[0] == 'a' && letters[1] == 'b' &&
                    letters[2] == 'c' && letters[3] == 'd');
        }
        int test_nested_auto_without_null_byte(void) {
            char nested[3][3] = {"yes", "no", "ok"};
            char *whole_array = (char *)nested;
            char *word1 = (char *)nested[0];
            char *word2 = (char *)nested[1];
            char *word3 = (char *)nested[2];
            return !(strcmp(whole_array, "yesno") || strcmp(word1, "yesno") ||
                     strcmp(word2, "no") || strcmp(word3, "ok"));
        }
        int main(void) {
            if (!test_flat_static_with_null_byte()) {
                return 1;
            }
            if (!test_nested_static_with_null_byte()) {
                return 2;
            }
            if (!test_flat_auto_with_null_byte()) {
                return 3;
            }
            if (!test_nested_auto_with_null_byte()) {
                return 4;
            }
            if (!test_flat_static_without_null_byte()) {
                return 5;
            }
            if (!test_nested_static_without_null_byte()) {
                return 6;
            }
            if (!test_flat_auto_without_null_byte()) {
                return 7;
            }
            if (!test_nested_auto_without_null_byte()) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_flat_static_with_null_byte() { 
            tmp.0 = &flat.2
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = zero_extend tmp.3
            tmp.5 = tmp.4 == 100
            if !tmp.5 jump and_false_0
            tmp.8 = &flat.2
            tmp.9 = sign_extend 1
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=1)
            tmp.11 = *tmp.10
            tmp.12 = zero_extend tmp.11
            tmp.13 = tmp.12 == 111
            if !tmp.13 jump and_false_0
            tmp.7 = 1
            jump and_end_1
        
          and_false_0:
            tmp.7 = 0
        
          and_end_1:
            if !tmp.7 jump and_false_2
            tmp.16 = &flat.2
            tmp.17 = sign_extend 2
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            tmp.20 = zero_extend tmp.19
            tmp.21 = tmp.20 == 103
            if !tmp.21 jump and_false_2
            tmp.15 = 1
            jump and_end_3
        
          and_false_2:
            tmp.15 = 0
        
          and_end_3:
            if !tmp.15 jump and_false_4
            tmp.24 = &flat.2
            tmp.25 = sign_extend 3
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=1)
            tmp.27 = *tmp.26
            tmp.28 = zero_extend tmp.27
            tmp.29 = tmp.28 == 0
            if !tmp.29 jump and_false_4
            tmp.23 = 1
            jump and_end_5
        
          and_false_4:
            tmp.23 = 0
        
          and_end_5:
            return tmp.23
            return 0
        }
        global function test_nested_static_with_null_byte() { 
            tmp.30 = &nested.3
            tmp.31 = sign_extend 0
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=4)
            tmp.33 = sign_extend 0
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=1)
            tmp.35 = *tmp.34
            tmp.36 = sign_extend tmp.35
            tmp.37 = tmp.36 == 121
            if !tmp.37 jump and_false_6
            tmp.40 = &nested.3
            tmp.41 = sign_extend 0
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=4)
            tmp.43 = sign_extend 1
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=1)
            tmp.45 = *tmp.44
            tmp.46 = sign_extend tmp.45
            tmp.47 = tmp.46 == 101
            if !tmp.47 jump and_false_6
            tmp.39 = 1
            jump and_end_7
        
          and_false_6:
            tmp.39 = 0
        
          and_end_7:
            if !tmp.39 jump and_false_8
            tmp.50 = &nested.3
            tmp.51 = sign_extend 0
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=4)
            tmp.53 = sign_extend 2
            tmp.54 = add_ptr(tmp.52, index=tmp.53, scale=1)
            tmp.55 = *tmp.54
            tmp.56 = sign_extend tmp.55
            tmp.57 = tmp.56 == 115
            if !tmp.57 jump and_false_8
            tmp.49 = 1
            jump and_end_9
        
          and_false_8:
            tmp.49 = 0
        
          and_end_9:
            if !tmp.49 jump and_false_10
            tmp.60 = &nested.3
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=4)
            tmp.63 = sign_extend 3
            tmp.64 = add_ptr(tmp.62, index=tmp.63, scale=1)
            tmp.65 = *tmp.64
            tmp.66 = sign_extend tmp.65
            tmp.67 = tmp.66 == 0
            if !tmp.67 jump and_false_10
            tmp.59 = 1
            jump and_end_11
        
          and_false_10:
            tmp.59 = 0
        
          and_end_11:
            if !tmp.59 jump and_false_12
            tmp.70 = &nested.3
            tmp.71 = sign_extend 1
            tmp.72 = add_ptr(tmp.70, index=tmp.71, scale=4)
            tmp.73 = sign_extend 0
            tmp.74 = add_ptr(tmp.72, index=tmp.73, scale=1)
            tmp.75 = *tmp.74
            tmp.76 = sign_extend tmp.75
            tmp.77 = tmp.76 == 121
            if !tmp.77 jump and_false_12
            tmp.69 = 1
            jump and_end_13
        
          and_false_12:
            tmp.69 = 0
        
          and_end_13:
            if !tmp.69 jump and_false_14
            tmp.80 = &nested.3
            tmp.81 = sign_extend 1
            tmp.82 = add_ptr(tmp.80, index=tmp.81, scale=4)
            tmp.83 = sign_extend 1
            tmp.84 = add_ptr(tmp.82, index=tmp.83, scale=1)
            tmp.85 = *tmp.84
            tmp.86 = sign_extend tmp.85
            tmp.87 = tmp.86 == 117
            if !tmp.87 jump and_false_14
            tmp.79 = 1
            jump and_end_15
        
          and_false_14:
            tmp.79 = 0
        
          and_end_15:
            if !tmp.79 jump and_false_16
            tmp.90 = &nested.3
            tmp.91 = sign_extend 1
            tmp.92 = add_ptr(tmp.90, index=tmp.91, scale=4)
            tmp.93 = sign_extend 2
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=1)
            tmp.95 = *tmp.94
            tmp.96 = sign_extend tmp.95
            tmp.97 = tmp.96 == 112
            if !tmp.97 jump and_false_16
            tmp.89 = 1
            jump and_end_17
        
          and_false_16:
            tmp.89 = 0
        
          and_end_17:
            if !tmp.89 jump and_false_18
            tmp.100 = &nested.3
            tmp.101 = sign_extend 1
            tmp.102 = add_ptr(tmp.100, index=tmp.101, scale=4)
            tmp.103 = sign_extend 3
            tmp.104 = add_ptr(tmp.102, index=tmp.103, scale=1)
            tmp.105 = *tmp.104
            tmp.106 = sign_extend tmp.105
            tmp.107 = tmp.106 == 0
            if !tmp.107 jump and_false_18
            tmp.99 = 1
            jump and_end_19
        
          and_false_18:
            tmp.99 = 0
        
          and_end_19:
            return tmp.99
            return 0
        }
        global function test_flat_auto_with_null_byte() { 
            flat_auto.4[0] = 'x'
            flat_auto.4[1] = '\0'
            tmp.108 = &flat_auto.4
            tmp.109 = sign_extend 0
            tmp.110 = add_ptr(tmp.108, index=tmp.109, scale=1)
            tmp.111 = *tmp.110
            tmp.112 = sign_extend tmp.111
            tmp.113 = tmp.112 == 120
            if !tmp.113 jump and_false_20
            tmp.116 = &flat_auto.4
            tmp.117 = sign_extend 1
            tmp.118 = add_ptr(tmp.116, index=tmp.117, scale=1)
            tmp.119 = *tmp.118
            tmp.120 = sign_extend tmp.119
            tmp.121 = tmp.120 == 0
            if !tmp.121 jump and_false_20
            tmp.115 = 1
            jump and_end_21
        
          and_false_20:
            tmp.115 = 0
        
          and_end_21:
            return tmp.115
            return 0
        }
        global function test_nested_auto_with_null_byte() { 
            nested_auto.5[0] = 'a'
            nested_auto.5[1] = '\0'
            nested_auto.5[2] = 'b'
            nested_auto.5[3] = '\0'
            nested_auto.5[4] = 'c'
            nested_auto.5[5] = '\0'
            nested_auto.5[6] = 'd'
            nested_auto.5[7] = '\0'
            tmp.122 = &nested_auto.5
            tmp.123 = sign_extend 0
            tmp.124 = add_ptr(tmp.122, index=tmp.123, scale=4)
            tmp.125 = sign_extend 0
            tmp.126 = add_ptr(tmp.124, index=tmp.125, scale=2)
            tmp.127 = sign_extend 0
            tmp.128 = add_ptr(tmp.126, index=tmp.127, scale=1)
            tmp.129 = *tmp.128
            tmp.130 = sign_extend tmp.129
            tmp.131 = tmp.130 == 97
            if !tmp.131 jump and_false_22
            tmp.134 = &nested_auto.5
            tmp.135 = sign_extend 0
            tmp.136 = add_ptr(tmp.134, index=tmp.135, scale=4)
            tmp.137 = sign_extend 0
            tmp.138 = add_ptr(tmp.136, index=tmp.137, scale=2)
            tmp.139 = sign_extend 1
            tmp.140 = add_ptr(tmp.138, index=tmp.139, scale=1)
            tmp.141 = *tmp.140
            tmp.142 = sign_extend tmp.141
            tmp.143 = tmp.142 == 0
            if !tmp.143 jump and_false_22
            tmp.133 = 1
            jump and_end_23
        
          and_false_22:
            tmp.133 = 0
        
          and_end_23:
            if !tmp.133 jump and_false_24
            tmp.146 = &nested_auto.5
            tmp.147 = sign_extend 0
            tmp.148 = add_ptr(tmp.146, index=tmp.147, scale=4)
            tmp.149 = sign_extend 1
            tmp.150 = add_ptr(tmp.148, index=tmp.149, scale=2)
            tmp.151 = sign_extend 0
            tmp.152 = add_ptr(tmp.150, index=tmp.151, scale=1)
            tmp.153 = *tmp.152
            tmp.154 = sign_extend tmp.153
            tmp.155 = tmp.154 == 98
            if !tmp.155 jump and_false_24
            tmp.145 = 1
            jump and_end_25
        
          and_false_24:
            tmp.145 = 0
        
          and_end_25:
            if !tmp.145 jump and_false_26
            tmp.158 = &nested_auto.5
            tmp.159 = sign_extend 0
            tmp.160 = add_ptr(tmp.158, index=tmp.159, scale=4)
            tmp.161 = sign_extend 1
            tmp.162 = add_ptr(tmp.160, index=tmp.161, scale=2)
            tmp.163 = sign_extend 1
            tmp.164 = add_ptr(tmp.162, index=tmp.163, scale=1)
            tmp.165 = *tmp.164
            tmp.166 = sign_extend tmp.165
            tmp.167 = tmp.166 == 0
            if !tmp.167 jump and_false_26
            tmp.157 = 1
            jump and_end_27
        
          and_false_26:
            tmp.157 = 0
        
          and_end_27:
            if !tmp.157 jump and_false_28
            tmp.170 = &nested_auto.5
            tmp.171 = sign_extend 1
            tmp.172 = add_ptr(tmp.170, index=tmp.171, scale=4)
            tmp.173 = sign_extend 0
            tmp.174 = add_ptr(tmp.172, index=tmp.173, scale=2)
            tmp.175 = sign_extend 0
            tmp.176 = add_ptr(tmp.174, index=tmp.175, scale=1)
            tmp.177 = *tmp.176
            tmp.178 = sign_extend tmp.177
            tmp.179 = tmp.178 == 99
            if !tmp.179 jump and_false_28
            tmp.169 = 1
            jump and_end_29
        
          and_false_28:
            tmp.169 = 0
        
          and_end_29:
            if !tmp.169 jump and_false_30
            tmp.182 = &nested_auto.5
            tmp.183 = sign_extend 1
            tmp.184 = add_ptr(tmp.182, index=tmp.183, scale=4)
            tmp.185 = sign_extend 0
            tmp.186 = add_ptr(tmp.184, index=tmp.185, scale=2)
            tmp.187 = sign_extend 1
            tmp.188 = add_ptr(tmp.186, index=tmp.187, scale=1)
            tmp.189 = *tmp.188
            tmp.190 = sign_extend tmp.189
            tmp.191 = tmp.190 == 0
            if !tmp.191 jump and_false_30
            tmp.181 = 1
            jump and_end_31
        
          and_false_30:
            tmp.181 = 0
        
          and_end_31:
            if !tmp.181 jump and_false_32
            tmp.194 = &nested_auto.5
            tmp.195 = sign_extend 1
            tmp.196 = add_ptr(tmp.194, index=tmp.195, scale=4)
            tmp.197 = sign_extend 1
            tmp.198 = add_ptr(tmp.196, index=tmp.197, scale=2)
            tmp.199 = sign_extend 0
            tmp.200 = add_ptr(tmp.198, index=tmp.199, scale=1)
            tmp.201 = *tmp.200
            tmp.202 = sign_extend tmp.201
            tmp.203 = tmp.202 == 100
            if !tmp.203 jump and_false_32
            tmp.193 = 1
            jump and_end_33
        
          and_false_32:
            tmp.193 = 0
        
          and_end_33:
            if !tmp.193 jump and_false_34
            tmp.206 = &nested_auto.5
            tmp.207 = sign_extend 1
            tmp.208 = add_ptr(tmp.206, index=tmp.207, scale=4)
            tmp.209 = sign_extend 1
            tmp.210 = add_ptr(tmp.208, index=tmp.209, scale=2)
            tmp.211 = sign_extend 1
            tmp.212 = add_ptr(tmp.210, index=tmp.211, scale=1)
            tmp.213 = *tmp.212
            tmp.214 = sign_extend tmp.213
            tmp.215 = tmp.214 == 0
            if !tmp.215 jump and_false_34
            tmp.205 = 1
            jump and_end_35
        
          and_false_34:
            tmp.205 = 0
        
          and_end_35:
            return tmp.205
            return 0
        }
        global function test_flat_static_without_null_byte() { 
            tmp.216 = &letters.6
            tmp.217 = sign_extend 0
            tmp.218 = add_ptr(tmp.216, index=tmp.217, scale=1)
            tmp.219 = *tmp.218
            tmp.220 = sign_extend tmp.219
            tmp.221 = tmp.220 == 97
            if !tmp.221 jump and_false_36
            tmp.224 = &letters.6
            tmp.225 = sign_extend 1
            tmp.226 = add_ptr(tmp.224, index=tmp.225, scale=1)
            tmp.227 = *tmp.226
            tmp.228 = sign_extend tmp.227
            tmp.229 = tmp.228 == 98
            if !tmp.229 jump and_false_36
            tmp.223 = 1
            jump and_end_37
        
          and_false_36:
            tmp.223 = 0
        
          and_end_37:
            if !tmp.223 jump and_false_38
            tmp.232 = &letters.6
            tmp.233 = sign_extend 2
            tmp.234 = add_ptr(tmp.232, index=tmp.233, scale=1)
            tmp.235 = *tmp.234
            tmp.236 = sign_extend tmp.235
            tmp.237 = tmp.236 == 99
            if !tmp.237 jump and_false_38
            tmp.231 = 1
            jump and_end_39
        
          and_false_38:
            tmp.231 = 0
        
          and_end_39:
            if !tmp.231 jump and_false_40
            tmp.240 = &letters.6
            tmp.241 = sign_extend 3
            tmp.242 = add_ptr(tmp.240, index=tmp.241, scale=1)
            tmp.243 = *tmp.242
            tmp.244 = sign_extend tmp.243
            tmp.245 = tmp.244 == 100
            if !tmp.245 jump and_false_40
            tmp.239 = 1
            jump and_end_41
        
          and_false_40:
            tmp.239 = 0
        
          and_end_41:
            return tmp.239
            return 0
        }
        global function test_nested_static_without_null_byte() { 
            tmp.246 = &nested
            tmp.247 = tmp.246
            whole_array.7 = tmp.247
            tmp.248 = &nested
            tmp.249 = sign_extend 0
            tmp.250 = add_ptr(tmp.248, index=tmp.249, scale=3)
            word1.8 = tmp.250
            tmp.251 = &nested
            tmp.252 = sign_extend 1
            tmp.253 = add_ptr(tmp.251, index=tmp.252, scale=3)
            word2.9 = tmp.253
            tmp.254 = &nested
            tmp.255 = sign_extend 2
            tmp.256 = add_ptr(tmp.254, index=tmp.255, scale=3)
            word3.10 = tmp.256
            tmp.257 = &string.0
            tmp.258 = strcmp(whole_array.7, tmp.257)
            if tmp.258 jump or_true_42
            tmp.261 = &string.0
            tmp.262 = strcmp(word1.8, tmp.261)
            if tmp.262 jump or_true_42
            tmp.260 = 0
            jump or_end_43
        
          or_true_42:
            tmp.260 = 1
        
          or_end_43:
            if tmp.260 jump or_true_44
            tmp.265 = &string.1
            tmp.266 = strcmp(word2.9, tmp.265)
            if tmp.266 jump or_true_44
            tmp.264 = 0
            jump or_end_45
        
          or_true_44:
            tmp.264 = 1
        
          or_end_45:
            if tmp.264 jump or_true_46
            tmp.269 = &string.2
            tmp.270 = strcmp(word3.10, tmp.269)
            if tmp.270 jump or_true_46
            tmp.268 = 0
            jump or_end_47
        
          or_true_46:
            tmp.268 = 1
        
          or_end_47:
            tmp.271 = ! tmp.268
            return tmp.271
            return 0
        }
        global function test_flat_auto_without_null_byte() { 
            tmp.272 = - 1
            x.11 = tmp.272
            letters.12[0] = 'a'
            letters.12[1] = 'b'
            letters.12[2] = 'c'
            letters.12[3] = 'd'
            tmp.273 = - 1
            y.13 = tmp.273
            tmp.275 = - 1
            tmp.274 = x.11 == tmp.275
            if !tmp.274 jump and_false_48
            tmp.279 = - 1
            tmp.278 = y.13 == tmp.279
            if !tmp.278 jump and_false_48
            tmp.277 = 1
            jump and_end_49
        
          and_false_48:
            tmp.277 = 0
        
          and_end_49:
            if !tmp.277 jump and_false_50
            tmp.282 = &letters.12
            tmp.283 = sign_extend 0
            tmp.284 = add_ptr(tmp.282, index=tmp.283, scale=1)
            tmp.285 = *tmp.284
            tmp.286 = sign_extend tmp.285
            tmp.287 = tmp.286 == 97
            if !tmp.287 jump and_false_50
            tmp.281 = 1
            jump and_end_51
        
          and_false_50:
            tmp.281 = 0
        
          and_end_51:
            if !tmp.281 jump and_false_52
            tmp.290 = &letters.12
            tmp.291 = sign_extend 1
            tmp.292 = add_ptr(tmp.290, index=tmp.291, scale=1)
            tmp.293 = *tmp.292
            tmp.294 = sign_extend tmp.293
            tmp.295 = tmp.294 == 98
            if !tmp.295 jump and_false_52
            tmp.289 = 1
            jump and_end_53
        
          and_false_52:
            tmp.289 = 0
        
          and_end_53:
            if !tmp.289 jump and_false_54
            tmp.298 = &letters.12
            tmp.299 = sign_extend 2
            tmp.300 = add_ptr(tmp.298, index=tmp.299, scale=1)
            tmp.301 = *tmp.300
            tmp.302 = sign_extend tmp.301
            tmp.303 = tmp.302 == 99
            if !tmp.303 jump and_false_54
            tmp.297 = 1
            jump and_end_55
        
          and_false_54:
            tmp.297 = 0
        
          and_end_55:
            if !tmp.297 jump and_false_56
            tmp.306 = &letters.12
            tmp.307 = sign_extend 3
            tmp.308 = add_ptr(tmp.306, index=tmp.307, scale=1)
            tmp.309 = *tmp.308
            tmp.310 = sign_extend tmp.309
            tmp.311 = tmp.310 == 100
            if !tmp.311 jump and_false_56
            tmp.305 = 1
            jump and_end_57
        
          and_false_56:
            tmp.305 = 0
        
          and_end_57:
            return tmp.305
            return 0
        }
        global function test_nested_auto_without_null_byte() { 
            nested.14[0] = 'y'
            nested.14[1] = 'e'
            nested.14[2] = 's'
            nested.14[3] = 'n'
            nested.14[4] = 'o'
            nested.14[5] = '\0'
            nested.14[6] = 'o'
            nested.14[7] = 'k'
            nested.14[8] = '\0'
            tmp.312 = &nested.14
            tmp.313 = tmp.312
            whole_array.15 = tmp.313
            tmp.314 = &nested.14
            tmp.315 = sign_extend 0
            tmp.316 = add_ptr(tmp.314, index=tmp.315, scale=3)
            word1.16 = tmp.316
            tmp.317 = &nested.14
            tmp.318 = sign_extend 1
            tmp.319 = add_ptr(tmp.317, index=tmp.318, scale=3)
            word2.17 = tmp.319
            tmp.320 = &nested.14
            tmp.321 = sign_extend 2
            tmp.322 = add_ptr(tmp.320, index=tmp.321, scale=3)
            word3.18 = tmp.322
            tmp.323 = &string.0
            tmp.324 = strcmp(whole_array.15, tmp.323)
            if tmp.324 jump or_true_58
            tmp.327 = &string.0
            tmp.328 = strcmp(word1.16, tmp.327)
            if tmp.328 jump or_true_58
            tmp.326 = 0
            jump or_end_59
        
          or_true_58:
            tmp.326 = 1
        
          or_end_59:
            if tmp.326 jump or_true_60
            tmp.331 = &string.1
            tmp.332 = strcmp(word2.17, tmp.331)
            if tmp.332 jump or_true_60
            tmp.330 = 0
            jump or_end_61
        
          or_true_60:
            tmp.330 = 1
        
          or_end_61:
            if tmp.330 jump or_true_62
            tmp.335 = &string.2
            tmp.336 = strcmp(word3.18, tmp.335)
            if tmp.336 jump or_true_62
            tmp.334 = 0
            jump or_end_63
        
          or_true_62:
            tmp.334 = 1
        
          or_end_63:
            tmp.337 = ! tmp.334
            return tmp.337
            return 0
        }
        global function main() { 
            tmp.338 = test_flat_static_with_null_byte()
            tmp.339 = ! tmp.338
            if !tmp.339 jump end_if_64
            return 1
        
          end_if_64:
            tmp.340 = test_nested_static_with_null_byte()
            tmp.341 = ! tmp.340
            if !tmp.341 jump end_if_66
            return 2
        
          end_if_66:
            tmp.342 = test_flat_auto_with_null_byte()
            tmp.343 = ! tmp.342
            if !tmp.343 jump end_if_68
            return 3
        
          end_if_68:
            tmp.344 = test_nested_auto_with_null_byte()
            tmp.345 = ! tmp.344
            if !tmp.345 jump end_if_70
            return 4
        
          end_if_70:
            tmp.346 = test_flat_static_without_null_byte()
            tmp.347 = ! tmp.346
            if !tmp.347 jump end_if_72
            return 5
        
          end_if_72:
            tmp.348 = test_nested_static_without_null_byte()
            tmp.349 = ! tmp.348
            if !tmp.349 jump end_if_74
            return 6
        
          end_if_74:
            tmp.350 = test_flat_auto_without_null_byte()
            tmp.351 = ! tmp.350
            if !tmp.351 jump end_if_76
            return 7
        
          end_if_76:
            tmp.352 = test_nested_auto_without_null_byte()
            tmp.353 = ! tmp.352
            if !tmp.353 jump end_if_78
            return 8
        
          end_if_78:
            return 0
            return 0
        }
        static flat.2: Array(4,Unsigned Char) = "dog\\0"
        static letters.6: Array(4,Char) = "abcd"
        static global nested: Array(3,Array(3,Char)) = [ "yes", "no\\0", "ok\\0"]
        static nested.3: Array(2,Array(4,Char)) = [ "yes\\0", "yup\\0"]
        constant string.0: Array(6,Char) = "yesno\\0"
        constant string.1: Array(3,Char) = "no\\0"
        constant string.2: Array(3,Char) = "ok\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_test_alignment() {
    let src = r#"
        int check_aligment(char *c) {
            unsigned long l = (unsigned long)c;
            return (l % 16 == 0);
        }
        static signed char flat_static[16] = "x";
        static unsigned char nested_static[3][4][2] = {{"a"}, {"b"}};
        int main(void) {
            char flat_auto[22];
            char nested_auto[10][3];
            if (!check_aligment((char *)flat_static)) {
                return 1;
            }
            if (!check_aligment((char *)nested_static)) {
                return 2;
            }
            if (!check_aligment((char *)flat_auto)) {
                return 3;
            }
            if (!check_aligment((char *)nested_auto)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function check_aligment(c.0) { 
            tmp.0 = c.0
            l.1 = tmp.0
            tmp.2 = sign_extend 16
            tmp.1 = l.1 % tmp.2
            tmp.4 = sign_extend 0
            tmp.3 = tmp.1 == tmp.4
            return tmp.3
            return 0
        }
        global function main() { 
            tmp.5 = &flat_static
            tmp.6 = tmp.5
            tmp.7 = check_aligment(tmp.6)
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = &nested_static
            tmp.10 = tmp.9
            tmp.11 = check_aligment(tmp.10)
            tmp.12 = ! tmp.11
            if !tmp.12 jump end_if_2
            return 2
        
          end_if_2:
            tmp.13 = &flat_auto.2
            tmp.14 = check_aligment(tmp.13)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_4
            return 3
        
          end_if_4:
            tmp.16 = &nested_auto.3
            tmp.17 = tmp.16
            tmp.18 = check_aligment(tmp.17)
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static flat_static: Array(16,Signed Char) = [ "x\\0", zero[14]]
        static nested_static: Array(3,Array(4,Array(2,Unsigned Char))) = [ "a\\0", zero[6], "b\\0", zero[6], zero[8]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_transfer_by_eightbyte() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int main(void) {
            char strings[2][13] = {"abcdefghijkl", "z"};
            if (strcmp(strings[0], "abcdefghijkl"))
                return 1;
            if (strings[1][0] != 'z')
                return 2;
            for (int i = 1; i < 13; i = i + 1) {
                if (strings[1][i])
                    return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            strings.2[0] = 'a'
            strings.2[1] = 'b'
            strings.2[2] = 'c'
            strings.2[3] = 'd'
            strings.2[4] = 'e'
            strings.2[5] = 'f'
            strings.2[6] = 'g'
            strings.2[7] = 'h'
            strings.2[8] = 'i'
            strings.2[9] = 'j'
            strings.2[10] = 'k'
            strings.2[11] = 'l'
            strings.2[12] = '\0'
            strings.2[13] = 'z'
            strings.2[14] = '\0'
            strings.2[15] = '\0'
            strings.2[16] = '\0'
            strings.2[17] = '\0'
            strings.2[18] = '\0'
            strings.2[19] = '\0'
            strings.2[20] = '\0'
            strings.2[21] = '\0'
            strings.2[22] = '\0'
            strings.2[23] = '\0'
            strings.2[24] = '\0'
            strings.2[25] = '\0'
            tmp.0 = &strings.2
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=13)
            tmp.3 = &string.0
            tmp.4 = strcmp(tmp.2, tmp.3)
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = &strings.2
            tmp.6 = sign_extend 1
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=13)
            tmp.8 = sign_extend 0
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=1)
            tmp.10 = *tmp.9
            tmp.11 = sign_extend tmp.10
            tmp.12 = tmp.11 != 122
            if !tmp.12 jump end_if_2
            return 2
        
          end_if_2:
            i.3 = 1
        
          start_loop_0:
            tmp.13 = i.3 < 13
            if !tmp.13 jump break_loop_0
            tmp.14 = &strings.2
            tmp.15 = sign_extend 1
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=13)
            tmp.17 = sign_extend i.3
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            if !tmp.19 jump end_if_4
            return 3
        
          end_if_4:
        
          continue_loop_0:
            tmp.20 = i.3 + 1
            i.3 = tmp.20
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
        constant string.0: Array(13,Char) = "abcdefghijkl\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_initializers_write_to_array() {
    let src = r#"
        int puts(char *s);
        int main(void) {
            char flat_arr[4] = "abc";
            puts(flat_arr);
            flat_arr[2] = 'x';
            puts(flat_arr);
            char nested_array[2][6] = {"Hello", "World"};
            puts(nested_array[0]);
            puts(nested_array[1]);
            nested_array[0][0] = 'J';
            puts(nested_array[0]);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            flat_arr.1[0] = 'a'
            flat_arr.1[1] = 'b'
            flat_arr.1[2] = 'c'
            flat_arr.1[3] = '\0'
            tmp.0 = &flat_arr.1
            tmp.1 = puts(tmp.0)
            tmp.2 = &flat_arr.1
            tmp.3 = sign_extend 2
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=1)
            tmp.5 = truncate 120
            *tmp.4 = tmp.5
            tmp.6 = &flat_arr.1
            tmp.7 = puts(tmp.6)
            nested_array.2[0] = 'H'
            nested_array.2[1] = 'e'
            nested_array.2[2] = 'l'
            nested_array.2[3] = 'l'
            nested_array.2[4] = 'o'
            nested_array.2[5] = '\0'
            nested_array.2[6] = 'W'
            nested_array.2[7] = 'o'
            nested_array.2[8] = 'r'
            nested_array.2[9] = 'l'
            nested_array.2[10] = 'd'
            nested_array.2[11] = '\0'
            tmp.8 = &nested_array.2
            tmp.9 = sign_extend 0
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=6)
            tmp.11 = puts(tmp.10)
            tmp.12 = &nested_array.2
            tmp.13 = sign_extend 1
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=6)
            tmp.15 = puts(tmp.14)
            tmp.16 = &nested_array.2
            tmp.17 = sign_extend 0
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=6)
            tmp.19 = sign_extend 0
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=1)
            tmp.21 = truncate 74
            *tmp.20 = tmp.21
            tmp.22 = &nested_array.2
            tmp.23 = sign_extend 0
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=6)
            tmp.25 = puts(tmp.24)
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_addr_of_string() {
    let src = r#"
        int puts(char *s);
        int main(void) {
            char(*str)[16] = &"Sample\tstring!\n";
            puts(*str);
            char (*one_past_the_end)[16] = str + 1;
            char *last_byte_pointer = (char *)one_past_the_end - 1;
            if (*last_byte_pointer != 0) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            str.1 = tmp.0
            tmp.1 = puts(str.1)
            tmp.3 = sign_extend 1
            tmp.2 = add_ptr(str.1, index=tmp.3, scale=16)
            one_past_the_end.2 = tmp.2
            tmp.4 = one_past_the_end.2
            tmp.6 = sign_extend 1
            tmp.7 = - tmp.6
            tmp.5 = add_ptr(tmp.4, index=tmp.7, scale=1)
            last_byte_pointer.3 = tmp.5
            tmp.8 = *last_byte_pointer.3
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 0
            if !tmp.10 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
        constant string.0: Array(16,Char) = "Sample\tstring!\n\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_adjacent_strings() {
    let src = r#"
        int puts(char *s);
        int main(void) {
            char *strings = "Hello," " World";
            puts(strings);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            strings.1 = tmp.0
            tmp.1 = puts(strings.1)
            return 0
            return 0
        }
        constant string.0: Array(13,Char) = "Hello, World\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_array_of_strings() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int main(void) {
            char *strings[4] = {"yes", "no", "maybe"};
            if (strcmp(strings[0], "yes")) {
                return 1;
            }
            if (strcmp(strings[1], "no")) {
                return 2;
            }
            if (strcmp(strings[2], "maybe")) {
                return 3;
            }
            if (strings[3]) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            strings.2[0] = tmp.0
            tmp.1 = &string.1
            strings.2[8] = tmp.1
            tmp.2 = &string.2
            strings.2[16] = tmp.2
            strings.2[24] = 0UL
            tmp.3 = &strings.2
            tmp.4 = sign_extend 0
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=8)
            tmp.6 = *tmp.5
            tmp.7 = &string.0
            tmp.8 = strcmp(tmp.6, tmp.7)
            if !tmp.8 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = &strings.2
            tmp.10 = sign_extend 1
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=8)
            tmp.12 = *tmp.11
            tmp.13 = &string.1
            tmp.14 = strcmp(tmp.12, tmp.13)
            if !tmp.14 jump end_if_2
            return 2
        
          end_if_2:
            tmp.15 = &strings.2
            tmp.16 = sign_extend 2
            tmp.17 = add_ptr(tmp.15, index=tmp.16, scale=8)
            tmp.18 = *tmp.17
            tmp.19 = &string.2
            tmp.20 = strcmp(tmp.18, tmp.19)
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
            tmp.21 = &strings.2
            tmp.22 = sign_extend 3
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=8)
            tmp.24 = *tmp.23
            if !tmp.24 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        constant string.0: Array(4,Char) = "yes\\0"
        constant string.1: Array(3,Char) = "no\\0"
        constant string.2: Array(6,Char) = "maybe\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_cast_string_pointer() {
    let src = r#"
        int main(void) {
            char *c = "This is a string!";
            unsigned char *uc = (unsigned char *)c;
            if (uc[3] != 's') {
                return 1;
            }
            signed char *sc = (signed char *)c;
            if (sc[3] != 's'){
                    return 2;
                }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            c.0 = tmp.0
            tmp.1 = c.0
            uc.1 = tmp.1
            tmp.2 = sign_extend 3
            tmp.3 = add_ptr(uc.1, index=tmp.2, scale=1)
            tmp.4 = *tmp.3
            tmp.5 = zero_extend tmp.4
            tmp.6 = tmp.5 != 115
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = c.0
            sc.2 = tmp.7
            tmp.8 = sign_extend 3
            tmp.9 = add_ptr(sc.2, index=tmp.8, scale=1)
            tmp.10 = *tmp.9
            tmp.11 = sign_extend tmp.10
            tmp.12 = tmp.11 != 115
            if !tmp.12 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        constant string.0: Array(18,Char) = "This is a string!\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_empty_string() {
    let src = r#"
        
        int main(void) {
            char *empty = "";
            return empty[0];
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            empty.0 = tmp.0
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(empty.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            return tmp.4
            return 0
        }
        constant string.0: Array(1,Char) = "\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_pointer_operations() {
    let src = r#"
        int main(void) {
            if ("abcdefg"[2] != 'c') {
                return 1;
            }
            char *ptr = "This is a string!" + 10;
            if (*ptr != 's') {
                return 2;
            }
            if (ptr[6] != '!') {
                return 3;
            }
            if (ptr[7]) {
                return 4;
            }
            if (!"Not a null pointer!") {
                return 5;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 != 99
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = &string.1
            tmp.8 = sign_extend 10
            tmp.7 = add_ptr(tmp.6, index=tmp.8, scale=1)
            ptr.0 = tmp.7
            tmp.9 = *ptr.0
            tmp.10 = sign_extend tmp.9
            tmp.11 = tmp.10 != 115
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = sign_extend 6
            tmp.13 = add_ptr(ptr.0, index=tmp.12, scale=1)
            tmp.14 = *tmp.13
            tmp.15 = sign_extend tmp.14
            tmp.16 = tmp.15 != 33
            if !tmp.16 jump end_if_4
            return 3
        
          end_if_4:
            tmp.17 = sign_extend 7
            tmp.18 = add_ptr(ptr.0, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            if !tmp.19 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = &string.2
            tmp.21 = ! tmp.20
            if !tmp.21 jump end_if_8
            return 5
        
          end_if_8:
            return 0
        }
        constant string.0: Array(8,Char) = "abcdefg\\0"
        constant string.1: Array(18,Char) = "This is a string!\\0"
        constant string.2: Array(20,Char) = "Not a null pointer!\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_simple() {
    let src = r#"
        int main(void) {
            char *x = "Hello, World!";
            return x[2];
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            x.0 = tmp.0
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(x.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            return tmp.4
            return 0
        }
        constant string.0: Array(14,Char) = "Hello, World!\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_standard_library_calls() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int puts(char *s);
        unsigned long strlen(char *s);
        int atoi(char *s);
        int main(void) {
            if (strcmp("abc", "abc")) {
                return 1;
            }
            if (strcmp("ab", "xy") >= 0) {
                return 2;
            }
            puts("Hello, World!");
            if (strlen("")) {
                return 3;
            }
            int i = atoi("10");
            if (i != 10) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            tmp.1 = &string.0
            tmp.2 = strcmp(tmp.0, tmp.1)
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = &string.1
            tmp.4 = &string.2
            tmp.5 = strcmp(tmp.3, tmp.4)
            tmp.6 = tmp.5 >= 0
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = &string.3
            tmp.8 = puts(tmp.7)
            tmp.9 = &string.4
            tmp.10 = strlen(tmp.9)
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = &string.5
            tmp.12 = atoi(tmp.11)
            i.5 = tmp.12
            tmp.13 = i.5 != 10
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        constant string.0: Array(4,Char) = "abc\\0"
        constant string.1: Array(3,Char) = "ab\\0"
        constant string.2: Array(3,Char) = "xy\\0"
        constant string.3: Array(14,Char) = "Hello, World!\\0"
        constant string.4: Array(1,Char) = "\\0"
        constant string.5: Array(3,Char) = "10\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_string_special_characters() {
    let src = r#"
        int puts(char *s);
        int strcmp(char *s1, char *s2);
        int main(void) {
            char *escape_sequence = "\a\b";
            if (escape_sequence[0] != 7) {
                return 1;
            }
            if (escape_sequence[1] != 8) {
                return 2;
            }
            if (escape_sequence[2]) {
                return 3;
            }
            char *with_double_quote = "Hello\"world";
            if (with_double_quote[5] != '"') {
                return 4;
            }
            puts(with_double_quote);
            char *with_backslash = "Hello\\World";
            if (with_backslash[5] != '\\') {
                return 5;
            }
            puts(with_backslash);
            char *with_newline = "Line\nbreak!";
            if (with_newline[4] != 10) {
                return 6;
            }
            puts(with_newline);
            char *tab = "	";
            if (strcmp(tab, "\t")) {
                return 7;
            }
           puts("Testing, 123.");
            puts("^@1 _\\]");
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            escape_sequence.3 = tmp.0
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(escape_sequence.3, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 != 7
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = sign_extend 1
            tmp.7 = add_ptr(escape_sequence.3, index=tmp.6, scale=1)
            tmp.8 = *tmp.7
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 8
            if !tmp.10 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = sign_extend 2
            tmp.12 = add_ptr(escape_sequence.3, index=tmp.11, scale=1)
            tmp.13 = *tmp.12
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            tmp.14 = &string.1
            with_double_quote.4 = tmp.14
            tmp.15 = sign_extend 5
            tmp.16 = add_ptr(with_double_quote.4, index=tmp.15, scale=1)
            tmp.17 = *tmp.16
            tmp.18 = sign_extend tmp.17
            tmp.19 = tmp.18 != 34
            if !tmp.19 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = puts(with_double_quote.4)
            tmp.21 = &string.2
            with_backslash.5 = tmp.21
            tmp.22 = sign_extend 5
            tmp.23 = add_ptr(with_backslash.5, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            tmp.25 = sign_extend tmp.24
            tmp.26 = tmp.25 != 92
            if !tmp.26 jump end_if_8
            return 5
        
          end_if_8:
            tmp.27 = puts(with_backslash.5)
            tmp.28 = &string.3
            with_newline.6 = tmp.28
            tmp.29 = sign_extend 4
            tmp.30 = add_ptr(with_newline.6, index=tmp.29, scale=1)
            tmp.31 = *tmp.30
            tmp.32 = sign_extend tmp.31
            tmp.33 = tmp.32 != 10
            if !tmp.33 jump end_if_10
            return 6
        
          end_if_10:
            tmp.34 = puts(with_newline.6)
            tmp.35 = &string.4
            tab.7 = tmp.35
            tmp.36 = &string.4
            tmp.37 = strcmp(tab.7, tmp.36)
            if !tmp.37 jump end_if_12
            return 7
        
          end_if_12:
            tmp.38 = &string.5
            tmp.39 = puts(tmp.38)
            tmp.40 = &string.6
            tmp.41 = puts(tmp.40)
            return 0
            return 0
        }
        constant string.0: Array(3,Char) = "\u{7}\u{8}\\0"
        constant string.1: Array(12,Char) = "Hello\"world\\0"
        constant string.2: Array(12,Char) = "Hello\\World\\0"
        constant string.3: Array(12,Char) = "Line\nbreak!\\0"
        constant string.4: Array(2,Char) = "\t\\0"
        constant string.5: Array(14,Char) = "Testing, 123.\\0"
        constant string.6: Array(8,Char) = "^@1 _\\]\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_strings_as_lvalues_strings_in_function_calls() {
    let src = r#"
        unsigned long strlen(char *s);
        char *return_string(void) {
            return "I'm a string!";
        }
        int pass_string_args(char *s1, char *s2) {
            if (s1 == 0 || s2 == 0) {
                return 0;
            }
            if (strlen(s1) != 45) {
                return 0;
            }
            if (s1[41] != 'd' || s1[42] != 'o' || s1[43] != 'g') {
                return 0;
            }
            if (s2[0]) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            char *ptr = 0;
            ptr = return_string();
            if (!ptr)
                return 1;
            if (ptr[0] != 'I' || ptr[1] != '\'' || ptr[13]) {
                return 2;
            }
            if (!pass_string_args("The quick brown fox jumped over the lazy dog.",
                                  "")) {
                return 3;
            }
            return 0;
            char *ptr2;
            ptr2 = 1 ? ptr + 2 : ptr + 4;
            return *ptr2 == 'm';
        }
    "#;
    let expected = r#"
        global function return_string() { 
            tmp.0 = &string.0
            return tmp.0
            return 0
        }
        global function pass_string_args(s1.1, s2.2) { 
            tmp.2 = sign_extend 0
            tmp.1 = s1.1 == tmp.2
            if tmp.1 jump or_true_0
            tmp.6 = sign_extend 0
            tmp.5 = s2.2 == tmp.6
            if tmp.5 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if !tmp.4 jump end_if_2
            return 0
        
          end_if_2:
            tmp.7 = strlen(s1.1)
            tmp.9 = sign_extend 45
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_4
            return 0
        
          end_if_4:
            tmp.10 = sign_extend 41
            tmp.11 = add_ptr(s1.1, index=tmp.10, scale=1)
            tmp.12 = *tmp.11
            tmp.13 = sign_extend tmp.12
            tmp.14 = tmp.13 != 100
            if tmp.14 jump or_true_6
            tmp.17 = sign_extend 42
            tmp.18 = add_ptr(s1.1, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            tmp.20 = sign_extend tmp.19
            tmp.21 = tmp.20 != 111
            if tmp.21 jump or_true_6
            tmp.16 = 0
            jump or_end_7
        
          or_true_6:
            tmp.16 = 1
        
          or_end_7:
            if tmp.16 jump or_true_8
            tmp.24 = sign_extend 43
            tmp.25 = add_ptr(s1.1, index=tmp.24, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = sign_extend tmp.26
            tmp.28 = tmp.27 != 103
            if tmp.28 jump or_true_8
            tmp.23 = 0
            jump or_end_9
        
          or_true_8:
            tmp.23 = 1
        
          or_end_9:
            if !tmp.23 jump end_if_10
            return 0
        
          end_if_10:
            tmp.29 = sign_extend 0
            tmp.30 = add_ptr(s2.2, index=tmp.29, scale=1)
            tmp.31 = *tmp.30
            if !tmp.31 jump end_if_12
            return 0
        
          end_if_12:
            return 1
            return 0
        }
        global function main() { 
            tmp.32 = sign_extend 0
            ptr.3 = tmp.32
            tmp.33 = return_string()
            ptr.3 = tmp.33
            tmp.34 = ! ptr.3
            if !tmp.34 jump end_if_14
            return 1
        
          end_if_14:
            tmp.35 = sign_extend 0
            tmp.36 = add_ptr(ptr.3, index=tmp.35, scale=1)
            tmp.37 = *tmp.36
            tmp.38 = sign_extend tmp.37
            tmp.39 = tmp.38 != 73
            if tmp.39 jump or_true_16
            tmp.42 = sign_extend 1
            tmp.43 = add_ptr(ptr.3, index=tmp.42, scale=1)
            tmp.44 = *tmp.43
            tmp.45 = sign_extend tmp.44
            tmp.46 = tmp.45 != 39
            if tmp.46 jump or_true_16
            tmp.41 = 0
            jump or_end_17
        
          or_true_16:
            tmp.41 = 1
        
          or_end_17:
            if tmp.41 jump or_true_18
            tmp.49 = sign_extend 13
            tmp.50 = add_ptr(ptr.3, index=tmp.49, scale=1)
            tmp.51 = *tmp.50
            if tmp.51 jump or_true_18
            tmp.48 = 0
            jump or_end_19
        
          or_true_18:
            tmp.48 = 1
        
          or_end_19:
            if !tmp.48 jump end_if_20
            return 2
        
          end_if_20:
            tmp.52 = &string.1
            tmp.53 = &string.2
            tmp.54 = pass_string_args(tmp.52, tmp.53)
            tmp.55 = ! tmp.54
            if !tmp.55 jump end_if_22
            return 3
        
          end_if_22:
            return 0
            if !1 jump else_25
            tmp.58 = sign_extend 2
            tmp.57 = add_ptr(ptr.3, index=tmp.58, scale=1)
            tmp.56 = tmp.57
            jump end_if_24
        
          else_25:
            tmp.60 = sign_extend 4
            tmp.59 = add_ptr(ptr.3, index=tmp.60, scale=1)
            tmp.56 = tmp.59
        
          end_if_24:
            ptr2.4 = tmp.56
            tmp.61 = *ptr2.4
            tmp.62 = sign_extend tmp.61
            tmp.63 = tmp.62 == 109
            return tmp.63
            return 0
        }
        constant string.0: Array(14,Char) = "I'm a string!\\0"
        constant string.1: Array(46,Char) = "The quick brown fox jumped over the lazy dog.\\0"
        constant string.2: Array(1,Char) = "\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
