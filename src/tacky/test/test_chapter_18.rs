use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_extra_credit_other_features_bitwise_ops_struct_members() {
    let src = r#"
        struct inner {
            char b;
            unsigned int u;
        };
        struct outer {
            unsigned long l;
            struct inner *in_ptr;
            int bar;
            struct inner in;
        };
        int main(void) {
            struct inner i = {'a', 100000u};
            struct outer o = {9223372036854775810ul, &i, 100, {-80, 4294967295U}};
            if ((i.b | o.l) != 9223372036854775907ul) {
                return 1;
            }
            if ((o.bar ^ i.u) != 100036u) {
                return 2;
            }
            if ((o.in_ptr->b & o.in.b) != 32) {
                return 3;
            }
            if ((o.l >> 26) != 137438953472ul) {
                return 4;
            }
            o.bar = 12;
            if ((i.b << o.bar) != 397312) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 97
            inner.0[0] = tmp.0
            inner.0[4] = 100000U
            outer.1[0] = 9223372036854775810UL
            tmp.1 = &i.2
            outer.1[8] = tmp.1
            outer.1[16] = 100
            tmp.2 = - 80
            tmp.3 = truncate tmp.2
            inner.0[20] = tmp.3
            inner.0[24] = 4294967295U
            tmp.4 = i.2[0]
            tmp.5 = sign_extend tmp.4
            tmp.7 = o.3[0]
            tmp.6 = tmp.5 | tmp.7
            tmp.8 = tmp.6 != 9223372036854775907UL
            if !tmp.8 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = o.3[2]
            tmp.10 = tmp.9
            tmp.12 = i.2[1]
            tmp.11 = tmp.10 ^ tmp.12
            tmp.13 = tmp.11 != 100036U
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            tmp.14 = o.3[1]
            tmp.15 = sign_extend tmp.14
            tmp.17 = o.3[3]
            tmp.18 = sign_extend tmp.17
            tmp.16 = tmp.15 & tmp.18
            tmp.19 = tmp.16 != 32
            if !tmp.19 jump end_if_4
            return 3
        
          end_if_4:
            tmp.20 = o.3[0]
            tmp.21 = tmp.20 >> 26
            tmp.22 = tmp.21 != 137438953472UL
            if !tmp.22 jump end_if_6
            return 4
        
          end_if_6:
            o.3[2] = 12
            tmp.23 = i.2[0]
            tmp.24 = sign_extend tmp.23
            tmp.26 = o.3[2]
            tmp.25 = tmp.24 << tmp.26
            tmp.27 = tmp.25 != 397312
            if !tmp.27 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_other_features_compound_assign_struct_members() {
    let src = r#"
        
        struct inner {
            double a;
            char b;
            int *ptr;
        };
        struct outer {
            unsigned long l;
            struct inner *in_ptr;
            struct inner in_array[4];
            int bar;
        };
        int main(void) {
            int i = -1;
            int i2 = -2;
            struct inner si = {150., -12, &i};
            struct outer o = {
                              18446744073709551615UL,
                              &si,
                              {si, {-20e20, 120, 0}, {0, 0, 0}, {1, 1, &i2}},
                              2000};
            si.a += 10;
            if (si.a != 160) {
                return 1;
            }
            o.in_array[0].b -= 460;
            if (o.in_array[0].b != 40) {
                return 2;
            }
            o.in_array[1].a *= -4;
            if (o.in_array[1].a != 80e20) {
                return 4;
            }
            o.in_ptr->a /= 5;
            if (si.a != 32) {
                return 5;
            }
            (&o)->l %= o.bar;
            if (o.l != 1615) {
                return 6;
            }
            o.in_ptr = o.in_array;
            if ((o.in_ptr += 3)->a != 1) {
                return 7;
            }
            if (*o.in_ptr->ptr != -2) {
                return 8;
            }
            o.in_ptr -= 1u;
            if (o.in_ptr->a || o.in_ptr->b || o.in_ptr->ptr) {
                return 9;
            }
            if (si.a != 32 || si.b != -12 || si.ptr != &i) {
                return 10;
            }
            if (o.l != 1615) {
                return 11;
            }
            if (o.in_ptr != &o.in_array[2]) {
                return 12;
            }
            if (o.in_array[0].a != 150. || o.in_array[0].b != 40 ||
                o.in_array[0].ptr != &i) {
                return 13;
            }
            if (o.in_array[1].a != 80e20 || o.in_array[1].b != 120 ||
                o.in_array[1].ptr) {
                return 14;
            }
            if (o.in_array[2].a || o.in_array[2].b || o.in_array[2].ptr) {
                return 15;
            }
            if (o.in_array[3].a != 1 || o.in_array[3].b != 1 ||
                o.in_array[3].ptr != &i2) {
                return 16;
            }
            if (o.bar != 2000) {
                return 17;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            i.2 = tmp.0
            tmp.1 = - 2
            i2.3 = tmp.1
            inner.0[0] = 150D
            tmp.2 = - 12
            tmp.3 = truncate tmp.2
            inner.0[8] = tmp.3
            tmp.4 = &i.2
            inner.0[16] = tmp.4
            outer.1[0] = 18446744073709551615UL
            tmp.5 = &si.4
            outer.1[8] = tmp.5
            outer.1[16] = si.4
            tmp.6 = - 2000000000000000000000D
            inner.0[40] = tmp.6
            tmp.7 = truncate 120
            inner.0[48] = tmp.7
            tmp.8 = sign_extend 0
            inner.0[56] = tmp.8
            tmp.9 = int_to_double 0
            inner.0[64] = tmp.9
            tmp.10 = truncate 0
            inner.0[72] = tmp.10
            tmp.11 = sign_extend 0
            inner.0[80] = tmp.11
            tmp.12 = int_to_double 1
            inner.0[88] = tmp.12
            tmp.13 = truncate 1
            inner.0[96] = tmp.13
            tmp.14 = &i2.3
            inner.0[104] = tmp.14
            outer.1[112] = 2000
            tmp.15 = si.4[0]
            tmp.17 = int_to_double 10
            tmp.16 = tmp.15 + tmp.17
            si.4[0] = tmp.16
            tmp.18 = si.4[0]
            tmp.20 = int_to_double 160
            tmp.19 = tmp.18 != tmp.20
            if !tmp.19 jump end_if_0
            return 1
        
          end_if_0:
            tmp.21 = o.5[2]
            tmp.22 = &tmp.21
            tmp.23 = sign_extend 0
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=24)
            tmp.25 = add_ptr(tmp.24, index=1L, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = sign_extend tmp.26
            tmp.28 = tmp.27 - 460
            tmp.29 = truncate tmp.28
            *tmp.25 = tmp.29
            tmp.30 = truncate tmp.29
            tmp.31 = o.5[2]
            tmp.32 = &tmp.31
            tmp.33 = sign_extend 0
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=24)
            tmp.35 = add_ptr(tmp.34, index=1L, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = sign_extend tmp.36
            tmp.38 = tmp.37 != 40
            if !tmp.38 jump end_if_2
            return 2
        
          end_if_2:
            tmp.39 = o.5[2]
            tmp.40 = &tmp.39
            tmp.41 = sign_extend 1
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=24)
            tmp.43 = *tmp.42
            tmp.45 = - 4
            tmp.46 = int_to_double tmp.45
            tmp.44 = tmp.43 * tmp.46
            *tmp.42 = tmp.44
            tmp.47 = o.5[2]
            tmp.48 = &tmp.47
            tmp.49 = sign_extend 1
            tmp.50 = add_ptr(tmp.48, index=tmp.49, scale=24)
            tmp.51 = *tmp.50
            tmp.52 = tmp.51 != 8000000000000000000000D
            if !tmp.52 jump end_if_4
            return 4
        
          end_if_4:
            tmp.53 = o.5[1]
            tmp.55 = int_to_double 5
            tmp.54 = tmp.53 / tmp.55
            tmp.53 = tmp.54
            tmp.56 = si.4[0]
            tmp.58 = int_to_double 32
            tmp.57 = tmp.56 != tmp.58
            if !tmp.57 jump end_if_6
            return 5
        
          end_if_6:
            tmp.59 = &o.5
            tmp.61 = o.5[3]
            tmp.62 = sign_extend tmp.61
            tmp.60 = tmp.59 % tmp.62
            tmp.59 = tmp.60
            tmp.63 = o.5[0]
            tmp.65 = sign_extend 1615
            tmp.64 = tmp.63 != tmp.65
            if !tmp.64 jump end_if_8
            return 6
        
          end_if_8:
            tmp.66 = o.5[2]
            tmp.67 = &tmp.66
            o.5[1] = tmp.67
            tmp.68 = o.5[1]
            tmp.70 = sign_extend 3
            tmp.69 = add_ptr(tmp.68, index=tmp.70, scale=24)
            o.5[1] = tmp.69
            tmp.72 = int_to_double 1
            tmp.71 = tmp.69 != tmp.72
            if !tmp.71 jump end_if_10
            return 7
        
          end_if_10:
            tmp.73 = o.5[1]
            tmp.74 = add_ptr(tmp.73, index=2L, scale=1)
            tmp.75 = *tmp.74
            tmp.77 = - 2
            tmp.76 = tmp.75 != tmp.77
            if !tmp.76 jump end_if_12
            return 8
        
          end_if_12:
            tmp.78 = o.5[1]
            tmp.80 = zero_extend 1U
            tmp.81 = - tmp.80
            tmp.79 = add_ptr(tmp.78, index=tmp.81, scale=24)
            o.5[1] = tmp.79
            tmp.82 = o.5[1]
            if tmp.82 jump or_true_14
            tmp.85 = o.5[1]
            tmp.86 = add_ptr(tmp.85, index=1L, scale=1)
            if tmp.86 jump or_true_14
            tmp.84 = 0
            jump or_end_15
        
          or_true_14:
            tmp.84 = 1
        
          or_end_15:
            if tmp.84 jump or_true_16
            tmp.89 = o.5[1]
            tmp.90 = add_ptr(tmp.89, index=2L, scale=1)
            if tmp.90 jump or_true_16
            tmp.88 = 0
            jump or_end_17
        
          or_true_16:
            tmp.88 = 1
        
          or_end_17:
            if !tmp.88 jump end_if_18
            return 9
        
          end_if_18:
            tmp.91 = si.4[0]
            tmp.93 = int_to_double 32
            tmp.92 = tmp.91 != tmp.93
            if tmp.92 jump or_true_20
            tmp.96 = si.4[1]
            tmp.97 = sign_extend tmp.96
            tmp.99 = - 12
            tmp.98 = tmp.97 != tmp.99
            if tmp.98 jump or_true_20
            tmp.95 = 0
            jump or_end_21
        
          or_true_20:
            tmp.95 = 1
        
          or_end_21:
            if tmp.95 jump or_true_22
            tmp.102 = si.4[2]
            tmp.104 = &i.2
            tmp.103 = tmp.102 != tmp.104
            if tmp.103 jump or_true_22
            tmp.101 = 0
            jump or_end_23
        
          or_true_22:
            tmp.101 = 1
        
          or_end_23:
            if !tmp.101 jump end_if_24
            return 10
        
          end_if_24:
            tmp.105 = o.5[0]
            tmp.107 = sign_extend 1615
            tmp.106 = tmp.105 != tmp.107
            if !tmp.106 jump end_if_26
            return 11
        
          end_if_26:
            tmp.108 = o.5[1]
            tmp.110 = o.5[2]
            tmp.111 = &tmp.110
            tmp.112 = sign_extend 2
            tmp.113 = add_ptr(tmp.111, index=tmp.112, scale=24)
            tmp.109 = tmp.108 != tmp.113
            if !tmp.109 jump end_if_28
            return 12
        
          end_if_28:
            tmp.114 = o.5[2]
            tmp.115 = &tmp.114
            tmp.116 = sign_extend 0
            tmp.117 = add_ptr(tmp.115, index=tmp.116, scale=24)
            tmp.118 = *tmp.117
            tmp.119 = tmp.118 != 150D
            if tmp.119 jump or_true_30
            tmp.122 = o.5[2]
            tmp.123 = &tmp.122
            tmp.124 = sign_extend 0
            tmp.125 = add_ptr(tmp.123, index=tmp.124, scale=24)
            tmp.126 = add_ptr(tmp.125, index=1L, scale=1)
            tmp.127 = *tmp.126
            tmp.128 = sign_extend tmp.127
            tmp.129 = tmp.128 != 40
            if tmp.129 jump or_true_30
            tmp.121 = 0
            jump or_end_31
        
          or_true_30:
            tmp.121 = 1
        
          or_end_31:
            if tmp.121 jump or_true_32
            tmp.132 = o.5[2]
            tmp.133 = &tmp.132
            tmp.134 = sign_extend 0
            tmp.135 = add_ptr(tmp.133, index=tmp.134, scale=24)
            tmp.136 = add_ptr(tmp.135, index=2L, scale=1)
            tmp.137 = *tmp.136
            tmp.139 = &i.2
            tmp.138 = tmp.137 != tmp.139
            if tmp.138 jump or_true_32
            tmp.131 = 0
            jump or_end_33
        
          or_true_32:
            tmp.131 = 1
        
          or_end_33:
            if !tmp.131 jump end_if_34
            return 13
        
          end_if_34:
            tmp.140 = o.5[2]
            tmp.141 = &tmp.140
            tmp.142 = sign_extend 1
            tmp.143 = add_ptr(tmp.141, index=tmp.142, scale=24)
            tmp.144 = *tmp.143
            tmp.145 = tmp.144 != 8000000000000000000000D
            if tmp.145 jump or_true_36
            tmp.148 = o.5[2]
            tmp.149 = &tmp.148
            tmp.150 = sign_extend 1
            tmp.151 = add_ptr(tmp.149, index=tmp.150, scale=24)
            tmp.152 = add_ptr(tmp.151, index=1L, scale=1)
            tmp.153 = *tmp.152
            tmp.154 = sign_extend tmp.153
            tmp.155 = tmp.154 != 120
            if tmp.155 jump or_true_36
            tmp.147 = 0
            jump or_end_37
        
          or_true_36:
            tmp.147 = 1
        
          or_end_37:
            if tmp.147 jump or_true_38
            tmp.158 = o.5[2]
            tmp.159 = &tmp.158
            tmp.160 = sign_extend 1
            tmp.161 = add_ptr(tmp.159, index=tmp.160, scale=24)
            tmp.162 = add_ptr(tmp.161, index=2L, scale=1)
            tmp.163 = *tmp.162
            if tmp.163 jump or_true_38
            tmp.157 = 0
            jump or_end_39
        
          or_true_38:
            tmp.157 = 1
        
          or_end_39:
            if !tmp.157 jump end_if_40
            return 14
        
          end_if_40:
            tmp.164 = o.5[2]
            tmp.165 = &tmp.164
            tmp.166 = sign_extend 2
            tmp.167 = add_ptr(tmp.165, index=tmp.166, scale=24)
            tmp.168 = *tmp.167
            if tmp.168 jump or_true_42
            tmp.171 = o.5[2]
            tmp.172 = &tmp.171
            tmp.173 = sign_extend 2
            tmp.174 = add_ptr(tmp.172, index=tmp.173, scale=24)
            tmp.175 = add_ptr(tmp.174, index=1L, scale=1)
            tmp.176 = *tmp.175
            if tmp.176 jump or_true_42
            tmp.170 = 0
            jump or_end_43
        
          or_true_42:
            tmp.170 = 1
        
          or_end_43:
            if tmp.170 jump or_true_44
            tmp.179 = o.5[2]
            tmp.180 = &tmp.179
            tmp.181 = sign_extend 2
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=24)
            tmp.183 = add_ptr(tmp.182, index=2L, scale=1)
            tmp.184 = *tmp.183
            if tmp.184 jump or_true_44
            tmp.178 = 0
            jump or_end_45
        
          or_true_44:
            tmp.178 = 1
        
          or_end_45:
            if !tmp.178 jump end_if_46
            return 15
        
          end_if_46:
            tmp.185 = o.5[2]
            tmp.186 = &tmp.185
            tmp.187 = sign_extend 3
            tmp.188 = add_ptr(tmp.186, index=tmp.187, scale=24)
            tmp.189 = *tmp.188
            tmp.191 = int_to_double 1
            tmp.190 = tmp.189 != tmp.191
            if tmp.190 jump or_true_48
            tmp.194 = o.5[2]
            tmp.195 = &tmp.194
            tmp.196 = sign_extend 3
            tmp.197 = add_ptr(tmp.195, index=tmp.196, scale=24)
            tmp.198 = add_ptr(tmp.197, index=1L, scale=1)
            tmp.199 = *tmp.198
            tmp.200 = sign_extend tmp.199
            tmp.201 = tmp.200 != 1
            if tmp.201 jump or_true_48
            tmp.193 = 0
            jump or_end_49
        
          or_true_48:
            tmp.193 = 1
        
          or_end_49:
            if tmp.193 jump or_true_50
            tmp.204 = o.5[2]
            tmp.205 = &tmp.204
            tmp.206 = sign_extend 3
            tmp.207 = add_ptr(tmp.205, index=tmp.206, scale=24)
            tmp.208 = add_ptr(tmp.207, index=2L, scale=1)
            tmp.209 = *tmp.208
            tmp.211 = &i2.3
            tmp.210 = tmp.209 != tmp.211
            if tmp.210 jump or_true_50
            tmp.203 = 0
            jump or_end_51
        
          or_true_50:
            tmp.203 = 1
        
          or_end_51:
            if !tmp.203 jump end_if_52
            return 16
        
          end_if_52:
            tmp.212 = o.5[3]
            tmp.213 = tmp.212 != 2000
            if !tmp.213 jump end_if_54
            return 17
        
          end_if_54:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_other_features_decr_arrow_lexing() {
    let src = r#"
        
        int main(void) {
            int arr[3] = {0, 1, 2};
            int *ptr = arr + 2;
            if(ptr-->arr) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.0[0] = 0
            arr.0[4] = 1
            arr.0[8] = 2
            tmp.0 = &arr.0
            tmp.2 = sign_extend 2
            tmp.1 = add_ptr(tmp.0, index=tmp.2, scale=4)
            ptr.1 = tmp.1
            tmp.3 = ptr.1
            tmp.4 = add_ptr(ptr.1, index=-1L, scale=4)
            ptr.1 = tmp.4
            tmp.6 = &arr.0
            tmp.5 = tmp.3 > tmp.6
            if !tmp.5 jump end_if_0
            return 0
        
          end_if_0:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_other_features_incr_struct_members() {
    let src = r#"
        struct inner {
            char c;
            unsigned int u;
        };
        struct outer {
            unsigned long l;
            struct inner *in_ptr;
            int array[3];
        };
        void *calloc(unsigned long nmemb, unsigned long size);
        int main(void) {
            struct outer my_struct = {
                9223372036854775900ul,
                calloc(3, sizeof (struct inner)),
                {-1000, -2000, -3000},
            };
            struct outer *my_struct_ptr = &my_struct;
            if (++my_struct.l != 9223372036854775901ul) {
                return 1;
            }
            if (--my_struct.in_ptr[0].u != 4294967295U) {
                return 2;
            }
            if (my_struct.in_ptr->c++) {
                return 3;
            }
            if (my_struct_ptr->array[1]-- != -2000) {
                return 4;
            }
            if (my_struct_ptr->l != 9223372036854775901ul) {
                return 5;
            }
            if (my_struct.in_ptr->c != 1) {
                return 6;
            }
            if (my_struct_ptr->in_ptr->u != 4294967295U) {
                return 7;
            }
            if (my_struct_ptr->array[1] != -2001) {
                return 8;
            }
            if (my_struct_ptr->array[0] != -1000 || my_struct_ptr->array[2] != -3000) {
                return 9;
            }
            my_struct_ptr->in_ptr[1].c = -1;
            my_struct_ptr->in_ptr[1].u = 1u;
            my_struct_ptr->in_ptr[2].c = 'X';
            my_struct_ptr->in_ptr[2].u = 100000u;
            (++my_struct_ptr->in_ptr)->c--;
            my_struct_ptr->in_ptr++->u++;
            if (my_struct_ptr->in_ptr[-2].c != 1 || my_struct_ptr->in_ptr[-2].u != 4294967295U) {
                return 10;
            }
            if (my_struct_ptr->in_ptr[-1].c != -2) {
                return 11;
            }
            if (my_struct_ptr->in_ptr[-1].u != 2) {
                return 12;
            }
            if (my_struct_ptr->in_ptr[0].c != 'X' || my_struct_ptr->in_ptr[0].u != 100000u) {
                return 13;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            outer.1[0] = 9223372036854775900UL
            tmp.0 = sign_extend 3
            tmp.1 = calloc(tmp.0, 8UL)
            tmp.2 = tmp.1
            outer.1[8] = tmp.2
            tmp.3 = - 1000
            outer.1[16] = tmp.3
            tmp.4 = - 2000
            outer.1[20] = tmp.4
            tmp.5 = - 3000
            outer.1[24] = tmp.5
            tmp.6 = &my_struct.4
            my_struct_ptr.5 = tmp.6
            tmp.7 = my_struct.4[0]
            tmp.8 = inc tmp.7
            my_struct.4[0] = tmp.8
            tmp.9 = tmp.8 != 9223372036854775901UL
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.10 = my_struct.4[1]
            tmp.11 = sign_extend 0
            tmp.12 = add_ptr(tmp.10, index=tmp.11, scale=8)
            tmp.13 = add_ptr(tmp.12, index=1L, scale=1)
            tmp.14 = *tmp.13
            tmp.15 = dec tmp.14
            *tmp.13 = tmp.15
            tmp.16 = tmp.15 != 4294967295U
            if !tmp.16 jump end_if_2
            return 2
        
          end_if_2:
            tmp.17 = my_struct.4[1]
            tmp.18 = tmp.17
            tmp.19 = inc tmp.17
            tmp.17 = tmp.19
            if !tmp.18 jump end_if_4
            return 3
        
          end_if_4:
            tmp.20 = add_ptr(my_struct_ptr.5, index=2L, scale=1)
            tmp.21 = &tmp.20
            tmp.22 = sign_extend 1
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=4)
            tmp.24 = *tmp.23
            tmp.25 = tmp.24
            tmp.26 = dec tmp.24
            *tmp.23 = tmp.26
            tmp.28 = - 2000
            tmp.27 = tmp.25 != tmp.28
            if !tmp.27 jump end_if_6
            return 4
        
          end_if_6:
            tmp.29 = my_struct_ptr.5 != 9223372036854775901UL
            if !tmp.29 jump end_if_8
            return 5
        
          end_if_8:
            tmp.30 = my_struct.4[1]
            tmp.31 = sign_extend tmp.30
            tmp.32 = tmp.31 != 1
            if !tmp.32 jump end_if_10
            return 6
        
          end_if_10:
            tmp.33 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.34 = add_ptr(tmp.33, index=1L, scale=1)
            tmp.35 = tmp.34 != 4294967295U
            if !tmp.35 jump end_if_12
            return 7
        
          end_if_12:
            tmp.36 = add_ptr(my_struct_ptr.5, index=2L, scale=1)
            tmp.37 = &tmp.36
            tmp.38 = sign_extend 1
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=4)
            tmp.40 = *tmp.39
            tmp.42 = - 2001
            tmp.41 = tmp.40 != tmp.42
            if !tmp.41 jump end_if_14
            return 8
        
          end_if_14:
            tmp.43 = add_ptr(my_struct_ptr.5, index=2L, scale=1)
            tmp.44 = &tmp.43
            tmp.45 = sign_extend 0
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=4)
            tmp.47 = *tmp.46
            tmp.49 = - 1000
            tmp.48 = tmp.47 != tmp.49
            if tmp.48 jump or_true_16
            tmp.52 = add_ptr(my_struct_ptr.5, index=2L, scale=1)
            tmp.53 = &tmp.52
            tmp.54 = sign_extend 2
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=4)
            tmp.56 = *tmp.55
            tmp.58 = - 3000
            tmp.57 = tmp.56 != tmp.58
            if tmp.57 jump or_true_16
            tmp.51 = 0
            jump or_end_17
        
          or_true_16:
            tmp.51 = 1
        
          or_end_17:
            if !tmp.51 jump end_if_18
            return 9
        
          end_if_18:
            tmp.59 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.60 = sign_extend 1
            tmp.61 = add_ptr(tmp.59, index=tmp.60, scale=8)
            tmp.62 = - 1
            tmp.63 = truncate tmp.62
            *tmp.61 = tmp.63
            tmp.64 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.65 = sign_extend 1
            tmp.66 = add_ptr(tmp.64, index=tmp.65, scale=8)
            tmp.67 = add_ptr(tmp.66, index=1L, scale=1)
            *tmp.67 = 1U
            tmp.68 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.69 = sign_extend 2
            tmp.70 = add_ptr(tmp.68, index=tmp.69, scale=8)
            tmp.71 = truncate 88
            *tmp.70 = tmp.71
            tmp.72 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.73 = sign_extend 2
            tmp.74 = add_ptr(tmp.72, index=tmp.73, scale=8)
            tmp.75 = add_ptr(tmp.74, index=1L, scale=1)
            *tmp.75 = 100000U
            tmp.76 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.77 = add_ptr(tmp.76, index=1L, scale=8)
            tmp.76 = tmp.77
            tmp.78 = tmp.77
            tmp.79 = dec tmp.77
            tmp.77 = tmp.79
            tmp.80 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.81 = tmp.80
            tmp.82 = add_ptr(tmp.80, index=1L, scale=8)
            tmp.80 = tmp.82
            tmp.83 = add_ptr(tmp.81, index=1L, scale=1)
            tmp.84 = tmp.83
            tmp.85 = inc tmp.83
            tmp.83 = tmp.85
            tmp.86 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.87 = - 2
            tmp.88 = sign_extend tmp.87
            tmp.89 = add_ptr(tmp.86, index=tmp.88, scale=8)
            tmp.90 = *tmp.89
            tmp.91 = sign_extend tmp.90
            tmp.92 = tmp.91 != 1
            if tmp.92 jump or_true_20
            tmp.95 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.96 = - 2
            tmp.97 = sign_extend tmp.96
            tmp.98 = add_ptr(tmp.95, index=tmp.97, scale=8)
            tmp.99 = add_ptr(tmp.98, index=1L, scale=1)
            tmp.100 = *tmp.99
            tmp.101 = tmp.100 != 4294967295U
            if tmp.101 jump or_true_20
            tmp.94 = 0
            jump or_end_21
        
          or_true_20:
            tmp.94 = 1
        
          or_end_21:
            if !tmp.94 jump end_if_22
            return 10
        
          end_if_22:
            tmp.102 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.103 = - 1
            tmp.104 = sign_extend tmp.103
            tmp.105 = add_ptr(tmp.102, index=tmp.104, scale=8)
            tmp.106 = *tmp.105
            tmp.107 = sign_extend tmp.106
            tmp.109 = - 2
            tmp.108 = tmp.107 != tmp.109
            if !tmp.108 jump end_if_24
            return 11
        
          end_if_24:
            tmp.110 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.111 = - 1
            tmp.112 = sign_extend tmp.111
            tmp.113 = add_ptr(tmp.110, index=tmp.112, scale=8)
            tmp.114 = add_ptr(tmp.113, index=1L, scale=1)
            tmp.115 = *tmp.114
            tmp.117 = 2
            tmp.116 = tmp.115 != tmp.117
            if !tmp.116 jump end_if_26
            return 12
        
          end_if_26:
            tmp.118 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.119 = sign_extend 0
            tmp.120 = add_ptr(tmp.118, index=tmp.119, scale=8)
            tmp.121 = *tmp.120
            tmp.122 = sign_extend tmp.121
            tmp.123 = tmp.122 != 88
            if tmp.123 jump or_true_28
            tmp.126 = add_ptr(my_struct_ptr.5, index=1L, scale=1)
            tmp.127 = sign_extend 0
            tmp.128 = add_ptr(tmp.126, index=tmp.127, scale=8)
            tmp.129 = add_ptr(tmp.128, index=1L, scale=1)
            tmp.130 = *tmp.129
            tmp.131 = tmp.130 != 100000U
            if tmp.131 jump or_true_28
            tmp.125 = 0
            jump or_end_29
        
          or_true_28:
            tmp.125 = 1
        
          or_end_29:
            if !tmp.125 jump end_if_30
            return 13
        
          end_if_30:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_other_features_label_tag_member_namespace() {
    let src = r#"
        int main(void) {
            struct x {
                int x;
            };
            struct x x = {10};
            goto x;
            return 0;
        x:
            return x.x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0[0] = 10
            jump x_0
            return 0
        
          x_0:
            tmp.0 = x.1[0]
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_other_features_struct_decl_in_switch_statement() {
    let src = r#"
        struct s {
            int a;
            int b;
        };
        int main(void) {
            struct s my_struct = {1, 2};
            int result = 0;
            switch (my_struct.a) {
                struct s {
                    double x;
                    double y;
                    double z;
                };
                struct s my_struct;
                case 1:
                    my_struct.x = 20.0;
                    my_struct.y = 30.0;
                    result = my_struct.x + my_struct.y;
                    break;
                case 2:
                    my_struct.x = 11.;
                    my_struct.y = 12.;
                    result = my_struct.x + my_struct.y;
                    break;
                default:
                    my_struct.x = 0.;
                    my_struct.y = 0.;
                    result = my_struct.x + my_struct.y;
            }
            return result;
        }
    "#;
    let expected = r#"
        global function main() { 
            s.0[0] = 1
            s.0[4] = 2
            result.2 = 0
            tmp.0 = my_struct.1[0]
            tmp.1 = 1 == tmp.0
            if tmp.1 jump switch_0_case__1
            tmp.2 = 2 == tmp.0
            if tmp.2 jump switch_0_case__2
            jump switch_0_default_3
            jump break_switch_0
        
          switch_0_case__1:
            my_struct.4[0] = 20D
            my_struct.4[1] = 30D
            tmp.3 = my_struct.4[0]
            tmp.5 = my_struct.4[1]
            tmp.4 = tmp.3 + tmp.5
            tmp.6 = double_to_int tmp.4
            result.2 = tmp.6
            jump break_switch_0
        
          switch_0_case__2:
            my_struct.4[0] = 11D
            my_struct.4[1] = 12D
            tmp.7 = my_struct.4[0]
            tmp.9 = my_struct.4[1]
            tmp.8 = tmp.7 + tmp.9
            tmp.10 = double_to_int tmp.8
            result.2 = tmp.10
            jump break_switch_0
        
          switch_0_default_3:
            my_struct.4[0] = 0D
            my_struct.4[1] = 0D
            tmp.11 = my_struct.4[0]
            tmp.13 = my_struct.4[1]
            tmp.12 = tmp.11 + tmp.13
            tmp.14 = double_to_int tmp.12
            result.2 = tmp.14
        
          break_switch_0:
            return result.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_array_of_structs() {
    let src = r#"
        struct inner {
            long l;
            char arr[2];
        };
        struct outer {
            char a;
            struct inner b;
        };
        int validate_struct_array(struct outer *struct_array);
        
        int validate_struct_array(struct outer *struct_array) {
            for (int i = 0; i < 3; i = i + 1) {
                if (struct_array[i].a != i * 2)
                    return 0;
                if (struct_array[i].b.l != i * 3)
                    return 0;
                if (struct_array[i].b.arr[0] != i * 4)
                    return 0;
                if (struct_array[i].b.arr[1] != i * 5)
                    return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function validate_struct_array(struct_array.3) { 
            i.4 = 0
        
          start_loop_0:
            tmp.0 = i.4 < 3
            if !tmp.0 jump break_loop_0
            tmp.1 = sign_extend i.4
            tmp.2 = add_ptr(struct_array.3, index=tmp.1, scale=24)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.6 = i.4 * 2
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_0
            return 0
        
          end_if_0:
            tmp.7 = sign_extend i.4
            tmp.8 = add_ptr(struct_array.3, index=tmp.7, scale=24)
            tmp.9 = add_ptr(tmp.8, index=1L, scale=1)
            tmp.10 = *tmp.9
            tmp.12 = i.4 * 3
            tmp.13 = sign_extend tmp.12
            tmp.11 = tmp.10 != tmp.13
            if !tmp.11 jump end_if_2
            return 0
        
          end_if_2:
            tmp.14 = sign_extend i.4
            tmp.15 = add_ptr(struct_array.3, index=tmp.14, scale=24)
            tmp.16 = add_ptr(tmp.15, index=1L, scale=1)
            tmp.17 = add_ptr(tmp.16, index=1L, scale=1)
            tmp.18 = sign_extend 0
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=1)
            tmp.20 = *tmp.19
            tmp.21 = sign_extend tmp.20
            tmp.23 = i.4 * 4
            tmp.22 = tmp.21 != tmp.23
            if !tmp.22 jump end_if_4
            return 0
        
          end_if_4:
            tmp.24 = sign_extend i.4
            tmp.25 = add_ptr(struct_array.3, index=tmp.24, scale=24)
            tmp.26 = add_ptr(tmp.25, index=1L, scale=1)
            tmp.27 = add_ptr(tmp.26, index=1L, scale=1)
            tmp.28 = sign_extend 1
            tmp.29 = add_ptr(tmp.27, index=tmp.28, scale=1)
            tmp.30 = *tmp.29
            tmp.31 = sign_extend tmp.30
            tmp.33 = i.4 * 5
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_6
            return 0
        
          end_if_6:
        
          continue_loop_0:
            tmp.34 = i.4 + 1
            i.4 = tmp.34
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_array_of_structs_client() {
    let src = r#"
        struct inner {
            long l;
            char arr[2];
        };
        struct outer {
            char a;
            struct inner b;
        };
        int validate_struct_array(struct outer *struct_array);
        
        static struct outer static_array[3] = {
            {0, {0, {0, 0}}}, {2, {3, {4, 5}}}, {4, {6, {8, 10}}}};
        int main(void) {
            struct outer auto_array[3] = {
                {0, {0, {0, 0}}}, {2, {3, {4, 5}}}, {4, {6, {8, 10}}}};
            if (!validate_struct_array(static_array)) {
                return 1;
            }
            if (!validate_struct_array(auto_array)) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 0
            outer.1[0] = tmp.0
            tmp.1 = sign_extend 0
            inner.0[8] = tmp.1
            tmp.2 = truncate 0
            inner.0[16] = tmp.2
            tmp.3 = truncate 0
            inner.0[17] = tmp.3
            tmp.4 = truncate 2
            outer.1[24] = tmp.4
            tmp.5 = sign_extend 3
            inner.0[32] = tmp.5
            tmp.6 = truncate 4
            inner.0[40] = tmp.6
            tmp.7 = truncate 5
            inner.0[41] = tmp.7
            tmp.8 = truncate 4
            outer.1[48] = tmp.8
            tmp.9 = sign_extend 6
            inner.0[56] = tmp.9
            tmp.10 = truncate 8
            inner.0[64] = tmp.10
            tmp.11 = truncate 10
            inner.0[65] = tmp.11
            tmp.12 = &static_array
            tmp.13 = validate_struct_array(tmp.12)
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_0
            return 1
        
          end_if_0:
            tmp.15 = &auto_array.3
            tmp.16 = validate_struct_array(tmp.15)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static static_array: Array(3,Struct(outer.1)) = [ '\0', zero[7], 0L, '\0', '\0', zero[6], '\u{2}', zero[7], 3L, '\u{4}', '\u{5}', zero[6], '\u{4}', zero[7], 6L, '\u{8}', '\n', zero[6]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_global_struct() {
    let src = r#"
        struct s {
            int i;
            char arr[2];
            double d;
        };
        struct outer {
            char c;
            struct s inner;
        };
        extern struct s global;
        extern struct outer global_outer;
        void update_struct(void);
        void update_outer_struct(void);
        
        void update_struct(void) {
            global.arr[1] = global.arr[0] * 2;
            global.d = 5.0;
        }
        void update_outer_struct(void) {
            struct s inner = {0, {-1, -1}, 0};
            global_outer.inner = inner;
        }
    "#;
    let expected = r#"
        global function update_struct() { 
            tmp.0 = global[1]
            tmp.1 = &tmp.0
            tmp.2 = sign_extend 1
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=1)
            tmp.4 = global[1]
            tmp.5 = &tmp.4
            tmp.6 = sign_extend 0
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=1)
            tmp.8 = *tmp.7
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 * 2
            tmp.11 = truncate tmp.10
            *tmp.3 = tmp.11
            global[2] = 5D
            return 0
        }
        global function update_outer_struct() { 
            s.0[0] = 0
            tmp.12 = - 1
            tmp.13 = truncate tmp.12
            s.0[4] = tmp.13
            tmp.14 = - 1
            tmp.15 = truncate tmp.14
            s.0[5] = tmp.15
            tmp.16 = int_to_double 0
            s.0[8] = tmp.16
            global_outer[1] = inner.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_global_struct_client() {
    let src = r#"
        struct s {
            int i;
            char arr[2];
            double d;
        };
        struct outer {
            char c;
            struct s inner;
        };
        extern struct s global;
        extern struct outer global_outer;
        void update_struct(void);
        void update_outer_struct(void);
        
        struct s global = {1, {2, 3}, 4.0};
        struct outer global_outer = {5, {6, {7, 8}, 9.0}};
        int main(void) {
            update_struct();
            if (global.arr[1] != 4) {
                return 1;
            }
            if (global.d != 5.0) {
                return 2;
            }
            update_outer_struct();
            if (global_outer.c != 5) {
                return 3;
            }
            if (global_outer.inner.i || global_outer.inner.d) {
                return 4;
            }
            if (global_outer.inner.arr[0] != -1 || global_outer.inner.arr[1] != -1) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            update_struct()
            tmp.0 = global[1]
            tmp.1 = &tmp.0
            tmp.2 = sign_extend 1
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=1)
            tmp.4 = *tmp.3
            tmp.5 = sign_extend tmp.4
            tmp.6 = tmp.5 != 4
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = global[2]
            tmp.8 = tmp.7 != 5D
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            update_outer_struct()
            tmp.9 = global_outer[0]
            tmp.10 = sign_extend tmp.9
            tmp.11 = tmp.10 != 5
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            tmp.12 = global_outer[1]
            if tmp.12 jump or_true_6
            tmp.15 = global_outer[3]
            if tmp.15 jump or_true_6
            tmp.14 = 0
            jump or_end_7
        
          or_true_6:
            tmp.14 = 1
        
          or_end_7:
            if !tmp.14 jump end_if_8
            return 4
        
          end_if_8:
            tmp.16 = global_outer[2]
            tmp.17 = &tmp.16
            tmp.18 = sign_extend 0
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=1)
            tmp.20 = *tmp.19
            tmp.21 = sign_extend tmp.20
            tmp.23 = - 1
            tmp.22 = tmp.21 != tmp.23
            if tmp.22 jump or_true_10
            tmp.26 = global_outer[2]
            tmp.27 = &tmp.26
            tmp.28 = sign_extend 1
            tmp.29 = add_ptr(tmp.27, index=tmp.28, scale=1)
            tmp.30 = *tmp.29
            tmp.31 = sign_extend tmp.30
            tmp.33 = - 1
            tmp.32 = tmp.31 != tmp.33
            if tmp.32 jump or_true_10
            tmp.25 = 0
            jump or_end_11
        
          or_true_10:
            tmp.25 = 1
        
          or_end_11:
            if !tmp.25 jump end_if_12
            return 5
        
          end_if_12:
            return 0
            return 0
        }
        static global global: Struct(s.0) = [ 1, '\u{2}', '\u{3}', zero[2], 4D]
        static global global_outer: Struct(outer.1) = [ '\u{5}', zero[7], 6, '\u{7}', '\u{8}', zero[2], 9D]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_auto_struct_initializers() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        void *malloc(unsigned long size);
        void *calloc(unsigned long nmemb, unsigned long size);
        struct s {
            char *one_msg;
            unsigned char two_arr[3];
            struct s *three_self_ptr;
            double four_d;
            double *five_d_ptr;
        };
        int validate_full_initialization(struct s *ptr);
        int validate_partial_initialization(struct s *ptr, char *expected_msg);
        int validate_converted(struct s *ptr);
        int validate_two_structs(struct s *ptr1, struct s *ptr2);
        
        int validate_full_initialization(struct s *ptr) {
            if (strcmp(ptr->one_msg, "I'm a struct!") || ptr->two_arr[0] != 's' ||
                ptr->two_arr[1] != 'u' || ptr->two_arr[2] != 'p' ||
                ptr->three_self_ptr != ptr || ptr->four_d != 2e12 ||
                *ptr->five_d_ptr != 2e12) {
                return 0;
            }
            return 1;
        }
        int validate_partial_initialization(struct s *ptr, char *expected_msg) {
            if (ptr->one_msg != expected_msg || ptr->two_arr[0] != 'a' ||
                ptr->two_arr[1] != 'b') {
                return 0;
            }
            if (ptr->three_self_ptr->one_msg) {
                return 0;
            }
            if (ptr->two_arr[2] || ptr->four_d || ptr->five_d_ptr) {
                return 0;
            }
            return 1;
        }
        int validate_converted(struct s *ptr) {
            if (!ptr->one_msg ||
                ptr->two_arr[0] != 220 || ptr->two_arr[1] != 232 ||
                ptr->two_arr[2] != 224 || ptr->three_self_ptr ||
                ptr->four_d != 2999.0 || *ptr->five_d_ptr != 0.0) {
                return 0;
            }
            return 1;
        }
        int validate_two_structs(struct s *ptr1, struct s *ptr2) {
            if (strcmp(ptr2->one_msg, "Yet another string") ||
                ptr2->one_msg != ptr1->one_msg ||
                ptr2->two_arr[0] != 'x' || ptr2->two_arr[1] != 'y' ||
                ptr2->three_self_ptr !=
                    ptr1 ||
                ptr2->four_d != 150.0 ||
                *ptr1->five_d_ptr != 123.4) {
                return 0;
            }
            if (ptr1->two_arr == ptr2->two_arr) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function validate_full_initialization(ptr.12) { 
            tmp.0 = &string.0
            tmp.1 = strcmp(ptr.12, tmp.0)
            if tmp.1 jump or_true_0
            tmp.4 = add_ptr(ptr.12, index=1L, scale=1)
            tmp.5 = &tmp.4
            tmp.6 = sign_extend 0
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=1)
            tmp.8 = *tmp.7
            tmp.9 = zero_extend tmp.8
            tmp.10 = tmp.9 != 115
            if tmp.10 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if tmp.3 jump or_true_2
            tmp.13 = add_ptr(ptr.12, index=1L, scale=1)
            tmp.14 = &tmp.13
            tmp.15 = sign_extend 1
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=1)
            tmp.17 = *tmp.16
            tmp.18 = zero_extend tmp.17
            tmp.19 = tmp.18 != 117
            if tmp.19 jump or_true_2
            tmp.12 = 0
            jump or_end_3
        
          or_true_2:
            tmp.12 = 1
        
          or_end_3:
            if tmp.12 jump or_true_4
            tmp.22 = add_ptr(ptr.12, index=1L, scale=1)
            tmp.23 = &tmp.22
            tmp.24 = sign_extend 2
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = zero_extend tmp.26
            tmp.28 = tmp.27 != 112
            if tmp.28 jump or_true_4
            tmp.21 = 0
            jump or_end_5
        
          or_true_4:
            tmp.21 = 1
        
          or_end_5:
            if tmp.21 jump or_true_6
            tmp.31 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.32 = tmp.31 != ptr.12
            if tmp.32 jump or_true_6
            tmp.30 = 0
            jump or_end_7
        
          or_true_6:
            tmp.30 = 1
        
          or_end_7:
            if tmp.30 jump or_true_8
            tmp.35 = add_ptr(ptr.12, index=3L, scale=1)
            tmp.36 = tmp.35 != 2000000000000D
            if tmp.36 jump or_true_8
            tmp.34 = 0
            jump or_end_9
        
          or_true_8:
            tmp.34 = 1
        
          or_end_9:
            if tmp.34 jump or_true_10
            tmp.39 = add_ptr(ptr.12, index=4L, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = tmp.40 != 2000000000000D
            if tmp.41 jump or_true_10
            tmp.38 = 0
            jump or_end_11
        
          or_true_10:
            tmp.38 = 1
        
          or_end_11:
            if !tmp.38 jump end_if_12
            return 0
        
          end_if_12:
            return 1
            return 0
        }
        global function validate_partial_initialization(ptr.13, expected_msg.14) { 
            tmp.42 = ptr.13 != expected_msg.14
            if tmp.42 jump or_true_14
            tmp.45 = add_ptr(ptr.13, index=1L, scale=1)
            tmp.46 = &tmp.45
            tmp.47 = sign_extend 0
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=1)
            tmp.49 = *tmp.48
            tmp.50 = zero_extend tmp.49
            tmp.51 = tmp.50 != 97
            if tmp.51 jump or_true_14
            tmp.44 = 0
            jump or_end_15
        
          or_true_14:
            tmp.44 = 1
        
          or_end_15:
            if tmp.44 jump or_true_16
            tmp.54 = add_ptr(ptr.13, index=1L, scale=1)
            tmp.55 = &tmp.54
            tmp.56 = sign_extend 1
            tmp.57 = add_ptr(tmp.55, index=tmp.56, scale=1)
            tmp.58 = *tmp.57
            tmp.59 = zero_extend tmp.58
            tmp.60 = tmp.59 != 98
            if tmp.60 jump or_true_16
            tmp.53 = 0
            jump or_end_17
        
          or_true_16:
            tmp.53 = 1
        
          or_end_17:
            if !tmp.53 jump end_if_18
            return 0
        
          end_if_18:
            tmp.61 = add_ptr(ptr.13, index=2L, scale=1)
            if !tmp.61 jump end_if_20
            return 0
        
          end_if_20:
            tmp.62 = add_ptr(ptr.13, index=1L, scale=1)
            tmp.63 = &tmp.62
            tmp.64 = sign_extend 2
            tmp.65 = add_ptr(tmp.63, index=tmp.64, scale=1)
            tmp.66 = *tmp.65
            if tmp.66 jump or_true_22
            tmp.69 = add_ptr(ptr.13, index=3L, scale=1)
            if tmp.69 jump or_true_22
            tmp.68 = 0
            jump or_end_23
        
          or_true_22:
            tmp.68 = 1
        
          or_end_23:
            if tmp.68 jump or_true_24
            tmp.72 = add_ptr(ptr.13, index=4L, scale=1)
            if tmp.72 jump or_true_24
            tmp.71 = 0
            jump or_end_25
        
          or_true_24:
            tmp.71 = 1
        
          or_end_25:
            if !tmp.71 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function validate_converted(ptr.15) { 
            tmp.73 = ! ptr.15
            if tmp.73 jump or_true_28
            tmp.76 = add_ptr(ptr.15, index=1L, scale=1)
            tmp.77 = &tmp.76
            tmp.78 = sign_extend 0
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=1)
            tmp.80 = *tmp.79
            tmp.81 = zero_extend tmp.80
            tmp.82 = tmp.81 != 220
            if tmp.82 jump or_true_28
            tmp.75 = 0
            jump or_end_29
        
          or_true_28:
            tmp.75 = 1
        
          or_end_29:
            if tmp.75 jump or_true_30
            tmp.85 = add_ptr(ptr.15, index=1L, scale=1)
            tmp.86 = &tmp.85
            tmp.87 = sign_extend 1
            tmp.88 = add_ptr(tmp.86, index=tmp.87, scale=1)
            tmp.89 = *tmp.88
            tmp.90 = zero_extend tmp.89
            tmp.91 = tmp.90 != 232
            if tmp.91 jump or_true_30
            tmp.84 = 0
            jump or_end_31
        
          or_true_30:
            tmp.84 = 1
        
          or_end_31:
            if tmp.84 jump or_true_32
            tmp.94 = add_ptr(ptr.15, index=1L, scale=1)
            tmp.95 = &tmp.94
            tmp.96 = sign_extend 2
            tmp.97 = add_ptr(tmp.95, index=tmp.96, scale=1)
            tmp.98 = *tmp.97
            tmp.99 = zero_extend tmp.98
            tmp.100 = tmp.99 != 224
            if tmp.100 jump or_true_32
            tmp.93 = 0
            jump or_end_33
        
          or_true_32:
            tmp.93 = 1
        
          or_end_33:
            if tmp.93 jump or_true_34
            tmp.103 = add_ptr(ptr.15, index=2L, scale=1)
            if tmp.103 jump or_true_34
            tmp.102 = 0
            jump or_end_35
        
          or_true_34:
            tmp.102 = 1
        
          or_end_35:
            if tmp.102 jump or_true_36
            tmp.106 = add_ptr(ptr.15, index=3L, scale=1)
            tmp.107 = tmp.106 != 2999D
            if tmp.107 jump or_true_36
            tmp.105 = 0
            jump or_end_37
        
          or_true_36:
            tmp.105 = 1
        
          or_end_37:
            if tmp.105 jump or_true_38
            tmp.110 = add_ptr(ptr.15, index=4L, scale=1)
            tmp.111 = *tmp.110
            tmp.112 = tmp.111 != 0D
            if tmp.112 jump or_true_38
            tmp.109 = 0
            jump or_end_39
        
          or_true_38:
            tmp.109 = 1
        
          or_end_39:
            if !tmp.109 jump end_if_40
            return 0
        
          end_if_40:
            return 1
            return 0
        }
        global function validate_two_structs(ptr1.16, ptr2.17) { 
            tmp.113 = &string.1
            tmp.114 = strcmp(ptr2.17, tmp.113)
            if tmp.114 jump or_true_42
            tmp.117 = ptr2.17 != ptr1.16
            if tmp.117 jump or_true_42
            tmp.116 = 0
            jump or_end_43
        
          or_true_42:
            tmp.116 = 1
        
          or_end_43:
            if tmp.116 jump or_true_44
            tmp.120 = add_ptr(ptr2.17, index=1L, scale=1)
            tmp.121 = &tmp.120
            tmp.122 = sign_extend 0
            tmp.123 = add_ptr(tmp.121, index=tmp.122, scale=1)
            tmp.124 = *tmp.123
            tmp.125 = zero_extend tmp.124
            tmp.126 = tmp.125 != 120
            if tmp.126 jump or_true_44
            tmp.119 = 0
            jump or_end_45
        
          or_true_44:
            tmp.119 = 1
        
          or_end_45:
            if tmp.119 jump or_true_46
            tmp.129 = add_ptr(ptr2.17, index=1L, scale=1)
            tmp.130 = &tmp.129
            tmp.131 = sign_extend 1
            tmp.132 = add_ptr(tmp.130, index=tmp.131, scale=1)
            tmp.133 = *tmp.132
            tmp.134 = zero_extend tmp.133
            tmp.135 = tmp.134 != 121
            if tmp.135 jump or_true_46
            tmp.128 = 0
            jump or_end_47
        
          or_true_46:
            tmp.128 = 1
        
          or_end_47:
            if tmp.128 jump or_true_48
            tmp.138 = add_ptr(ptr2.17, index=2L, scale=1)
            tmp.139 = tmp.138 != ptr1.16
            if tmp.139 jump or_true_48
            tmp.137 = 0
            jump or_end_49
        
          or_true_48:
            tmp.137 = 1
        
          or_end_49:
            if tmp.137 jump or_true_50
            tmp.142 = add_ptr(ptr2.17, index=3L, scale=1)
            tmp.143 = tmp.142 != 150D
            if tmp.143 jump or_true_50
            tmp.141 = 0
            jump or_end_51
        
          or_true_50:
            tmp.141 = 1
        
          or_end_51:
            if tmp.141 jump or_true_52
            tmp.146 = add_ptr(ptr1.16, index=4L, scale=1)
            tmp.147 = *tmp.146
            tmp.148 = tmp.147 != 123.4D
            if tmp.148 jump or_true_52
            tmp.145 = 0
            jump or_end_53
        
          or_true_52:
            tmp.145 = 1
        
          or_end_53:
            if !tmp.145 jump end_if_54
            return 0
        
          end_if_54:
            tmp.149 = add_ptr(ptr1.16, index=1L, scale=1)
            tmp.150 = &tmp.149
            tmp.152 = add_ptr(ptr2.17, index=1L, scale=1)
            tmp.153 = &tmp.152
            tmp.151 = tmp.150 == tmp.153
            if !tmp.151 jump end_if_56
            return 0
        
          end_if_56:
            return 1
            return 0
        }
        constant string.0: Array(14,Char) = "I'm a struct!\\0"
        constant string.1: Array(19,Char) = "Yet another string\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_auto_struct_initializers_client() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        void *malloc(unsigned long size);
        void *calloc(unsigned long nmemb, unsigned long size);
        struct s {
            char *one_msg;
            unsigned char two_arr[3];
            struct s *three_self_ptr;
            double four_d;
            double *five_d_ptr;
        };
        int validate_full_initialization(struct s *ptr);
        int validate_partial_initialization(struct s *ptr, char *expected_msg);
        int validate_converted(struct s *ptr);
        int validate_two_structs(struct s *ptr1, struct s *ptr2);
        double get_double(void) {
            return 2e12;
        }
        int test_full_initialization(void) {
            struct s full = {
                "I'm a struct!", "sup",
                &full,
                get_double(),
                &(full.four_d)
            };
            return validate_full_initialization(&full);
        }
        int test_partial_initialization(void) {
            static char *msg = "Another string literal";
            struct s partial = {
                msg,
                {'a', 'b'},
                (struct s *)calloc(
                    1,
                    sizeof(struct s))
            };
            return validate_partial_initialization(&partial, msg);
        }
        int test_implicit_type_conversions(void) {
            static int i = 3000;
            struct s converted = {
                malloc(5),
                {i / 2, i / 3, i * 4},
                0l,
                i - 1,
                calloc(1, sizeof(double))
            };
            return validate_converted(&converted);
        }
        int test_single_exp_initializer(void) {
            double d = 123.4;
            struct s s1 = {"Yet another string", "xy", &s1, 150.0, &d};
            struct s s2 = s1;
            return validate_two_structs(&s1, &s2);
        }
        int main(void) {
            if (!test_full_initialization()) {
                return 1;
            }
            if (!test_partial_initialization()) {
                return 2;
            }
            if (!test_implicit_type_conversions()) {
                return 3;
            }
            if (!test_single_exp_initializer()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_double() { 
            return 2000000000000D
            return 0
        }
        global function test_full_initialization() { 
            tmp.0 = &string.0
            s.5[0] = tmp.0
            s.5[8] = 115UC
            s.5[9] = 117UC
            s.5[10] = 112UC
            tmp.1 = &full.12
            s.5[16] = tmp.1
            tmp.2 = get_double()
            s.5[24] = tmp.2
            tmp.3 = &full.12
            tmp.3 = add_ptr(tmp.3, index=3L, scale=1)
            s.5[32] = tmp.3
            tmp.4 = &full.12
            tmp.5 = validate_full_initialization(tmp.4)
            return tmp.5
            return 0
        }
        global function test_partial_initialization() { 
            s.5[0] = msg.13
            tmp.6 = truncate 97
            s.5[8] = tmp.6
            tmp.7 = truncate 98
            s.5[9] = tmp.7
            s.5[10] = 0UC
            tmp.8 = sign_extend 1
            tmp.9 = calloc(tmp.8, 40UL)
            tmp.10 = tmp.9
            s.5[16] = tmp.10
            s.5[24] = 0D
            s.5[32] = 0UL
            tmp.11 = &partial.14
            tmp.12 = validate_partial_initialization(tmp.11, msg.13)
            return tmp.12
            return 0
        }
        global function test_implicit_type_conversions() { 
            tmp.13 = sign_extend 5
            tmp.14 = malloc(tmp.13)
            tmp.15 = tmp.14
            s.5[0] = tmp.15
            tmp.16 = i.15 / 2
            tmp.17 = truncate tmp.16
            s.5[8] = tmp.17
            tmp.18 = i.15 / 3
            tmp.19 = truncate tmp.18
            s.5[9] = tmp.19
            tmp.20 = i.15 * 4
            tmp.21 = truncate tmp.20
            s.5[10] = tmp.21
            tmp.22 = 0L
            s.5[16] = tmp.22
            tmp.23 = i.15 - 1
            tmp.24 = int_to_double tmp.23
            s.5[24] = tmp.24
            tmp.25 = sign_extend 1
            tmp.26 = calloc(tmp.25, 8UL)
            tmp.27 = tmp.26
            s.5[32] = tmp.27
            tmp.28 = &converted.16
            tmp.29 = validate_converted(tmp.28)
            return tmp.29
            return 0
        }
        global function test_single_exp_initializer() { 
            d.17 = 123.4D
            tmp.30 = &string.1
            s.5[0] = tmp.30
            s.5[8] = 120UC
            s.5[9] = 121UC
            s.5[10] = '\0'
            tmp.31 = &s1.18
            s.5[16] = tmp.31
            s.5[24] = 150D
            tmp.32 = &d.17
            s.5[32] = tmp.32
            s2.19 = s1.18
            tmp.33 = &s1.18
            tmp.34 = &s2.19
            tmp.35 = validate_two_structs(tmp.33, tmp.34)
            return tmp.35
            return 0
        }
        global function main() { 
            tmp.36 = test_full_initialization()
            tmp.37 = ! tmp.36
            if !tmp.37 jump end_if_0
            return 1
        
          end_if_0:
            tmp.38 = test_partial_initialization()
            tmp.39 = ! tmp.38
            if !tmp.39 jump end_if_2
            return 2
        
          end_if_2:
            tmp.40 = test_implicit_type_conversions()
            tmp.41 = ! tmp.40
            if !tmp.41 jump end_if_4
            return 3
        
          end_if_4:
            tmp.42 = test_single_exp_initializer()
            tmp.43 = ! tmp.42
            if !tmp.43 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static i.15: Int = 3000
        static msg.13: Pointer(Char) = &Another string literal
        constant string.0: Array(14,Char) = "I'm a struct!\\0"
        constant string.1: Array(19,Char) = "Yet another string\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_nested_auto_struct_initializers() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct pair {
            int a;
            int b;
        };
        struct inner {
            int one_i;
            unsigned char two_arr[3];
            unsigned three_u;
        };
        struct outer {
            long one_l;
            struct inner two_struct;
            char *three_msg;
            double four_d;
            struct pair five_pair;
        };
        int validate_full_initialization(struct outer *ptr);
        int validate_partial_initialization(struct outer *ptr);
        int validate_mixed_initialization(struct outer *ptr);
        int validate_array_of_structs(struct outer *struct_array);
        
        int validate_full_initialization(struct outer *ptr) {
            if (ptr->one_l != -200l || ptr->two_struct.one_i != -171 ||
                ptr->two_struct.two_arr[0] != 200 ||
                ptr->two_struct.two_arr[1] != 202 ||
                ptr->two_struct.two_arr[2] != 203 || ptr->two_struct.three_u != 40u ||
                strcmp(ptr->three_msg, "Important message!") || ptr->four_d != -22. ||
                ptr->five_pair.a != 1 || ptr->five_pair.b != 2) {
                return 0;
            }
            return 1;
        }
        int validate_partial_initialization(struct outer *ptr) {
            if (ptr->one_l != 1000 || ptr->two_struct.one_i != 1 ||
                strcmp(ptr->three_msg, "Partial")) {
                return 0;
            }
            if (ptr->two_struct.two_arr[0] || ptr->two_struct.two_arr[1] ||
                ptr->two_struct.two_arr[2] || ptr->two_struct.three_u || ptr->four_d ||
                ptr->five_pair.a || ptr->five_pair.b) {
                return 0;
            }
            return 1;
        }
        int validate_mixed_initialization(struct outer *ptr) {
            if (ptr->one_l != 200 || ptr->two_struct.one_i != 20 ||
                ptr->two_struct.two_arr[0] != 21 || ptr->two_struct.three_u != 22u ||
                strcmp(ptr->three_msg, "mixed") || ptr->four_d != 10.0 ||
                ptr->five_pair.a != 99 || ptr->five_pair.b != 100) {
                return 0;
            }
            if (ptr->two_struct.two_arr[1] || ptr->two_struct.two_arr[2]) {
                return 0;
            }
            return 1;
        }
        int validate_array_of_structs(struct outer *struct_array) {
            if (struct_array[0].one_l != 1 || struct_array[0].two_struct.one_i != 2 ||
                struct_array[0].two_struct.two_arr[0] != 3 ||
                struct_array[0].two_struct.two_arr[1] != 4 ||
                struct_array[0].two_struct.two_arr[2] != 5 ||
                struct_array[0].two_struct.three_u != 6 ||
                strcmp(struct_array[0].three_msg, "7") ||
                struct_array[0].four_d != 8.0 || struct_array[0].five_pair.a != 9 ||
                struct_array[0].five_pair.b != 10) {
                return 0;
            }
            if (struct_array[1].one_l != 101 ||
                struct_array[1].two_struct.one_i != 102 ||
                struct_array[1].two_struct.two_arr[0] != 103 ||
                struct_array[1].two_struct.two_arr[1] != 104 ||
                struct_array[1].two_struct.two_arr[2] != 105 ||
                struct_array[1].two_struct.three_u != 106 ||
                strcmp(struct_array[1].three_msg, "107") ||
                struct_array[1].four_d != 108.0 || struct_array[1].five_pair.a != 109 ||
                struct_array[1].five_pair.b != 110) {
                return 0;
            }
            if (struct_array[2].one_l != 201 ||
                struct_array[2].two_struct.one_i != 202 ||
                struct_array[2].two_struct.two_arr[0] != 203 ||
                struct_array[2].two_struct.two_arr[1] ||
                struct_array[2].two_struct.two_arr[2] ||
                struct_array[2].two_struct.three_u ||
                strcmp(struct_array[2].three_msg, "207") ||
                struct_array[2].four_d != 208.0 || struct_array[2].five_pair.a != 209 ||
                struct_array[2].five_pair.b) {
                return 0;
            }
            if (struct_array[3].one_l != 301 || struct_array[3].two_struct.one_i ||
                struct_array[3].two_struct.two_arr[0] ||
                struct_array[3].two_struct.two_arr[1] ||
                struct_array[3].two_struct.two_arr[2] ||
                struct_array[3].two_struct.three_u || struct_array[3].three_msg ||
                struct_array[3].four_d || struct_array[3].five_pair.a ||
                struct_array[3].five_pair.b) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function validate_full_initialization(ptr.9) { 
            tmp.1 = - 200L
            tmp.0 = ptr.9 != tmp.1
            if tmp.0 jump or_true_0
            tmp.4 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.5 = tmp.4[0]
            tmp.7 = - 171
            tmp.6 = tmp.5 != tmp.7
            if tmp.6 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if tmp.3 jump or_true_2
            tmp.10 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.11 = tmp.10[1]
            tmp.12 = &tmp.11
            tmp.13 = sign_extend 0
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = zero_extend tmp.15
            tmp.17 = tmp.16 != 200
            if tmp.17 jump or_true_2
            tmp.9 = 0
            jump or_end_3
        
          or_true_2:
            tmp.9 = 1
        
          or_end_3:
            if tmp.9 jump or_true_4
            tmp.20 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.21 = tmp.20[1]
            tmp.22 = &tmp.21
            tmp.23 = sign_extend 1
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=1)
            tmp.25 = *tmp.24
            tmp.26 = zero_extend tmp.25
            tmp.27 = tmp.26 != 202
            if tmp.27 jump or_true_4
            tmp.19 = 0
            jump or_end_5
        
          or_true_4:
            tmp.19 = 1
        
          or_end_5:
            if tmp.19 jump or_true_6
            tmp.30 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.31 = tmp.30[1]
            tmp.32 = &tmp.31
            tmp.33 = sign_extend 2
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=1)
            tmp.35 = *tmp.34
            tmp.36 = zero_extend tmp.35
            tmp.37 = tmp.36 != 203
            if tmp.37 jump or_true_6
            tmp.29 = 0
            jump or_end_7
        
          or_true_6:
            tmp.29 = 1
        
          or_end_7:
            if tmp.29 jump or_true_8
            tmp.40 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.41 = tmp.40[2]
            tmp.42 = tmp.41 != 40U
            if tmp.42 jump or_true_8
            tmp.39 = 0
            jump or_end_9
        
          or_true_8:
            tmp.39 = 1
        
          or_end_9:
            if tmp.39 jump or_true_10
            tmp.45 = add_ptr(ptr.9, index=2L, scale=1)
            tmp.46 = &string.0
            tmp.47 = strcmp(tmp.45, tmp.46)
            if tmp.47 jump or_true_10
            tmp.44 = 0
            jump or_end_11
        
          or_true_10:
            tmp.44 = 1
        
          or_end_11:
            if tmp.44 jump or_true_12
            tmp.50 = add_ptr(ptr.9, index=3L, scale=1)
            tmp.52 = - 22D
            tmp.51 = tmp.50 != tmp.52
            if tmp.51 jump or_true_12
            tmp.49 = 0
            jump or_end_13
        
          or_true_12:
            tmp.49 = 1
        
          or_end_13:
            if tmp.49 jump or_true_14
            tmp.55 = add_ptr(ptr.9, index=4L, scale=1)
            tmp.56 = tmp.55[0]
            tmp.57 = tmp.56 != 1
            if tmp.57 jump or_true_14
            tmp.54 = 0
            jump or_end_15
        
          or_true_14:
            tmp.54 = 1
        
          or_end_15:
            if tmp.54 jump or_true_16
            tmp.60 = add_ptr(ptr.9, index=4L, scale=1)
            tmp.61 = tmp.60[1]
            tmp.62 = tmp.61 != 2
            if tmp.62 jump or_true_16
            tmp.59 = 0
            jump or_end_17
        
          or_true_16:
            tmp.59 = 1
        
          or_end_17:
            if !tmp.59 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function validate_partial_initialization(ptr.10) { 
            tmp.64 = sign_extend 1000
            tmp.63 = ptr.10 != tmp.64
            if tmp.63 jump or_true_20
            tmp.67 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.68 = tmp.67[0]
            tmp.69 = tmp.68 != 1
            if tmp.69 jump or_true_20
            tmp.66 = 0
            jump or_end_21
        
          or_true_20:
            tmp.66 = 1
        
          or_end_21:
            if tmp.66 jump or_true_22
            tmp.72 = add_ptr(ptr.10, index=2L, scale=1)
            tmp.73 = &string.1
            tmp.74 = strcmp(tmp.72, tmp.73)
            if tmp.74 jump or_true_22
            tmp.71 = 0
            jump or_end_23
        
          or_true_22:
            tmp.71 = 1
        
          or_end_23:
            if !tmp.71 jump end_if_24
            return 0
        
          end_if_24:
            tmp.75 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.76 = tmp.75[1]
            tmp.77 = &tmp.76
            tmp.78 = sign_extend 0
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=1)
            tmp.80 = *tmp.79
            if tmp.80 jump or_true_26
            tmp.83 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.84 = tmp.83[1]
            tmp.85 = &tmp.84
            tmp.86 = sign_extend 1
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=1)
            tmp.88 = *tmp.87
            if tmp.88 jump or_true_26
            tmp.82 = 0
            jump or_end_27
        
          or_true_26:
            tmp.82 = 1
        
          or_end_27:
            if tmp.82 jump or_true_28
            tmp.91 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.92 = tmp.91[1]
            tmp.93 = &tmp.92
            tmp.94 = sign_extend 2
            tmp.95 = add_ptr(tmp.93, index=tmp.94, scale=1)
            tmp.96 = *tmp.95
            if tmp.96 jump or_true_28
            tmp.90 = 0
            jump or_end_29
        
          or_true_28:
            tmp.90 = 1
        
          or_end_29:
            if tmp.90 jump or_true_30
            tmp.99 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.100 = tmp.99[2]
            if tmp.100 jump or_true_30
            tmp.98 = 0
            jump or_end_31
        
          or_true_30:
            tmp.98 = 1
        
          or_end_31:
            if tmp.98 jump or_true_32
            tmp.103 = add_ptr(ptr.10, index=3L, scale=1)
            if tmp.103 jump or_true_32
            tmp.102 = 0
            jump or_end_33
        
          or_true_32:
            tmp.102 = 1
        
          or_end_33:
            if tmp.102 jump or_true_34
            tmp.106 = add_ptr(ptr.10, index=4L, scale=1)
            tmp.107 = tmp.106[0]
            if tmp.107 jump or_true_34
            tmp.105 = 0
            jump or_end_35
        
          or_true_34:
            tmp.105 = 1
        
          or_end_35:
            if tmp.105 jump or_true_36
            tmp.110 = add_ptr(ptr.10, index=4L, scale=1)
            tmp.111 = tmp.110[1]
            if tmp.111 jump or_true_36
            tmp.109 = 0
            jump or_end_37
        
          or_true_36:
            tmp.109 = 1
        
          or_end_37:
            if !tmp.109 jump end_if_38
            return 0
        
          end_if_38:
            return 1
            return 0
        }
        global function validate_mixed_initialization(ptr.11) { 
            tmp.113 = sign_extend 200
            tmp.112 = ptr.11 != tmp.113
            if tmp.112 jump or_true_40
            tmp.116 = add_ptr(ptr.11, index=1L, scale=1)
            tmp.117 = tmp.116[0]
            tmp.118 = tmp.117 != 20
            if tmp.118 jump or_true_40
            tmp.115 = 0
            jump or_end_41
        
          or_true_40:
            tmp.115 = 1
        
          or_end_41:
            if tmp.115 jump or_true_42
            tmp.121 = add_ptr(ptr.11, index=1L, scale=1)
            tmp.122 = tmp.121[1]
            tmp.123 = &tmp.122
            tmp.124 = sign_extend 0
            tmp.125 = add_ptr(tmp.123, index=tmp.124, scale=1)
            tmp.126 = *tmp.125
            tmp.127 = zero_extend tmp.126
            tmp.128 = tmp.127 != 21
            if tmp.128 jump or_true_42
            tmp.120 = 0
            jump or_end_43
        
          or_true_42:
            tmp.120 = 1
        
          or_end_43:
            if tmp.120 jump or_true_44
            tmp.131 = add_ptr(ptr.11, index=1L, scale=1)
            tmp.132 = tmp.131[2]
            tmp.133 = tmp.132 != 22U
            if tmp.133 jump or_true_44
            tmp.130 = 0
            jump or_end_45
        
          or_true_44:
            tmp.130 = 1
        
          or_end_45:
            if tmp.130 jump or_true_46
            tmp.136 = add_ptr(ptr.11, index=2L, scale=1)
            tmp.137 = &string.2
            tmp.138 = strcmp(tmp.136, tmp.137)
            if tmp.138 jump or_true_46
            tmp.135 = 0
            jump or_end_47
        
          or_true_46:
            tmp.135 = 1
        
          or_end_47:
            if tmp.135 jump or_true_48
            tmp.141 = add_ptr(ptr.11, index=3L, scale=1)
            tmp.142 = tmp.141 != 10D
            if tmp.142 jump or_true_48
            tmp.140 = 0
            jump or_end_49
        
          or_true_48:
            tmp.140 = 1
        
          or_end_49:
            if tmp.140 jump or_true_50
            tmp.145 = add_ptr(ptr.11, index=4L, scale=1)
            tmp.146 = tmp.145[0]
            tmp.147 = tmp.146 != 99
            if tmp.147 jump or_true_50
            tmp.144 = 0
            jump or_end_51
        
          or_true_50:
            tmp.144 = 1
        
          or_end_51:
            if tmp.144 jump or_true_52
            tmp.150 = add_ptr(ptr.11, index=4L, scale=1)
            tmp.151 = tmp.150[1]
            tmp.152 = tmp.151 != 100
            if tmp.152 jump or_true_52
            tmp.149 = 0
            jump or_end_53
        
          or_true_52:
            tmp.149 = 1
        
          or_end_53:
            if !tmp.149 jump end_if_54
            return 0
        
          end_if_54:
            tmp.153 = add_ptr(ptr.11, index=1L, scale=1)
            tmp.154 = tmp.153[1]
            tmp.155 = &tmp.154
            tmp.156 = sign_extend 1
            tmp.157 = add_ptr(tmp.155, index=tmp.156, scale=1)
            tmp.158 = *tmp.157
            if tmp.158 jump or_true_56
            tmp.161 = add_ptr(ptr.11, index=1L, scale=1)
            tmp.162 = tmp.161[1]
            tmp.163 = &tmp.162
            tmp.164 = sign_extend 2
            tmp.165 = add_ptr(tmp.163, index=tmp.164, scale=1)
            tmp.166 = *tmp.165
            if tmp.166 jump or_true_56
            tmp.160 = 0
            jump or_end_57
        
          or_true_56:
            tmp.160 = 1
        
          or_end_57:
            if !tmp.160 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function validate_array_of_structs(struct_array.12) { 
            tmp.167 = sign_extend 0
            tmp.168 = add_ptr(struct_array.12, index=tmp.167, scale=48)
            tmp.169 = *tmp.168
            tmp.171 = sign_extend 1
            tmp.170 = tmp.169 != tmp.171
            if tmp.170 jump or_true_60
            tmp.174 = sign_extend 0
            tmp.175 = add_ptr(struct_array.12, index=tmp.174, scale=48)
            tmp.176 = add_ptr(tmp.175, index=1L, scale=1)
            tmp.177 = *tmp.176
            tmp.178 = tmp.177 != 2
            if tmp.178 jump or_true_60
            tmp.173 = 0
            jump or_end_61
        
          or_true_60:
            tmp.173 = 1
        
          or_end_61:
            if tmp.173 jump or_true_62
            tmp.181 = sign_extend 0
            tmp.182 = add_ptr(struct_array.12, index=tmp.181, scale=48)
            tmp.183 = add_ptr(tmp.182, index=1L, scale=1)
            tmp.184 = add_ptr(tmp.183, index=1L, scale=1)
            tmp.185 = sign_extend 0
            tmp.186 = add_ptr(tmp.184, index=tmp.185, scale=1)
            tmp.187 = *tmp.186
            tmp.188 = zero_extend tmp.187
            tmp.189 = tmp.188 != 3
            if tmp.189 jump or_true_62
            tmp.180 = 0
            jump or_end_63
        
          or_true_62:
            tmp.180 = 1
        
          or_end_63:
            if tmp.180 jump or_true_64
            tmp.192 = sign_extend 0
            tmp.193 = add_ptr(struct_array.12, index=tmp.192, scale=48)
            tmp.194 = add_ptr(tmp.193, index=1L, scale=1)
            tmp.195 = add_ptr(tmp.194, index=1L, scale=1)
            tmp.196 = sign_extend 1
            tmp.197 = add_ptr(tmp.195, index=tmp.196, scale=1)
            tmp.198 = *tmp.197
            tmp.199 = zero_extend tmp.198
            tmp.200 = tmp.199 != 4
            if tmp.200 jump or_true_64
            tmp.191 = 0
            jump or_end_65
        
          or_true_64:
            tmp.191 = 1
        
          or_end_65:
            if tmp.191 jump or_true_66
            tmp.203 = sign_extend 0
            tmp.204 = add_ptr(struct_array.12, index=tmp.203, scale=48)
            tmp.205 = add_ptr(tmp.204, index=1L, scale=1)
            tmp.206 = add_ptr(tmp.205, index=1L, scale=1)
            tmp.207 = sign_extend 2
            tmp.208 = add_ptr(tmp.206, index=tmp.207, scale=1)
            tmp.209 = *tmp.208
            tmp.210 = zero_extend tmp.209
            tmp.211 = tmp.210 != 5
            if tmp.211 jump or_true_66
            tmp.202 = 0
            jump or_end_67
        
          or_true_66:
            tmp.202 = 1
        
          or_end_67:
            if tmp.202 jump or_true_68
            tmp.214 = sign_extend 0
            tmp.215 = add_ptr(struct_array.12, index=tmp.214, scale=48)
            tmp.216 = add_ptr(tmp.215, index=1L, scale=1)
            tmp.217 = add_ptr(tmp.216, index=2L, scale=1)
            tmp.218 = *tmp.217
            tmp.220 = 6
            tmp.219 = tmp.218 != tmp.220
            if tmp.219 jump or_true_68
            tmp.213 = 0
            jump or_end_69
        
          or_true_68:
            tmp.213 = 1
        
          or_end_69:
            if tmp.213 jump or_true_70
            tmp.223 = sign_extend 0
            tmp.224 = add_ptr(struct_array.12, index=tmp.223, scale=48)
            tmp.225 = add_ptr(tmp.224, index=2L, scale=1)
            tmp.226 = *tmp.225
            tmp.227 = &string.3
            tmp.228 = strcmp(tmp.226, tmp.227)
            if tmp.228 jump or_true_70
            tmp.222 = 0
            jump or_end_71
        
          or_true_70:
            tmp.222 = 1
        
          or_end_71:
            if tmp.222 jump or_true_72
            tmp.231 = sign_extend 0
            tmp.232 = add_ptr(struct_array.12, index=tmp.231, scale=48)
            tmp.233 = add_ptr(tmp.232, index=3L, scale=1)
            tmp.234 = *tmp.233
            tmp.235 = tmp.234 != 8D
            if tmp.235 jump or_true_72
            tmp.230 = 0
            jump or_end_73
        
          or_true_72:
            tmp.230 = 1
        
          or_end_73:
            if tmp.230 jump or_true_74
            tmp.238 = sign_extend 0
            tmp.239 = add_ptr(struct_array.12, index=tmp.238, scale=48)
            tmp.240 = add_ptr(tmp.239, index=4L, scale=1)
            tmp.241 = *tmp.240
            tmp.242 = tmp.241 != 9
            if tmp.242 jump or_true_74
            tmp.237 = 0
            jump or_end_75
        
          or_true_74:
            tmp.237 = 1
        
          or_end_75:
            if tmp.237 jump or_true_76
            tmp.245 = sign_extend 0
            tmp.246 = add_ptr(struct_array.12, index=tmp.245, scale=48)
            tmp.247 = add_ptr(tmp.246, index=4L, scale=1)
            tmp.248 = add_ptr(tmp.247, index=1L, scale=1)
            tmp.249 = *tmp.248
            tmp.250 = tmp.249 != 10
            if tmp.250 jump or_true_76
            tmp.244 = 0
            jump or_end_77
        
          or_true_76:
            tmp.244 = 1
        
          or_end_77:
            if !tmp.244 jump end_if_78
            return 0
        
          end_if_78:
            tmp.251 = sign_extend 1
            tmp.252 = add_ptr(struct_array.12, index=tmp.251, scale=48)
            tmp.253 = *tmp.252
            tmp.255 = sign_extend 101
            tmp.254 = tmp.253 != tmp.255
            if tmp.254 jump or_true_80
            tmp.258 = sign_extend 1
            tmp.259 = add_ptr(struct_array.12, index=tmp.258, scale=48)
            tmp.260 = add_ptr(tmp.259, index=1L, scale=1)
            tmp.261 = *tmp.260
            tmp.262 = tmp.261 != 102
            if tmp.262 jump or_true_80
            tmp.257 = 0
            jump or_end_81
        
          or_true_80:
            tmp.257 = 1
        
          or_end_81:
            if tmp.257 jump or_true_82
            tmp.265 = sign_extend 1
            tmp.266 = add_ptr(struct_array.12, index=tmp.265, scale=48)
            tmp.267 = add_ptr(tmp.266, index=1L, scale=1)
            tmp.268 = add_ptr(tmp.267, index=1L, scale=1)
            tmp.269 = sign_extend 0
            tmp.270 = add_ptr(tmp.268, index=tmp.269, scale=1)
            tmp.271 = *tmp.270
            tmp.272 = zero_extend tmp.271
            tmp.273 = tmp.272 != 103
            if tmp.273 jump or_true_82
            tmp.264 = 0
            jump or_end_83
        
          or_true_82:
            tmp.264 = 1
        
          or_end_83:
            if tmp.264 jump or_true_84
            tmp.276 = sign_extend 1
            tmp.277 = add_ptr(struct_array.12, index=tmp.276, scale=48)
            tmp.278 = add_ptr(tmp.277, index=1L, scale=1)
            tmp.279 = add_ptr(tmp.278, index=1L, scale=1)
            tmp.280 = sign_extend 1
            tmp.281 = add_ptr(tmp.279, index=tmp.280, scale=1)
            tmp.282 = *tmp.281
            tmp.283 = zero_extend tmp.282
            tmp.284 = tmp.283 != 104
            if tmp.284 jump or_true_84
            tmp.275 = 0
            jump or_end_85
        
          or_true_84:
            tmp.275 = 1
        
          or_end_85:
            if tmp.275 jump or_true_86
            tmp.287 = sign_extend 1
            tmp.288 = add_ptr(struct_array.12, index=tmp.287, scale=48)
            tmp.289 = add_ptr(tmp.288, index=1L, scale=1)
            tmp.290 = add_ptr(tmp.289, index=1L, scale=1)
            tmp.291 = sign_extend 2
            tmp.292 = add_ptr(tmp.290, index=tmp.291, scale=1)
            tmp.293 = *tmp.292
            tmp.294 = zero_extend tmp.293
            tmp.295 = tmp.294 != 105
            if tmp.295 jump or_true_86
            tmp.286 = 0
            jump or_end_87
        
          or_true_86:
            tmp.286 = 1
        
          or_end_87:
            if tmp.286 jump or_true_88
            tmp.298 = sign_extend 1
            tmp.299 = add_ptr(struct_array.12, index=tmp.298, scale=48)
            tmp.300 = add_ptr(tmp.299, index=1L, scale=1)
            tmp.301 = add_ptr(tmp.300, index=2L, scale=1)
            tmp.302 = *tmp.301
            tmp.304 = 106
            tmp.303 = tmp.302 != tmp.304
            if tmp.303 jump or_true_88
            tmp.297 = 0
            jump or_end_89
        
          or_true_88:
            tmp.297 = 1
        
          or_end_89:
            if tmp.297 jump or_true_90
            tmp.307 = sign_extend 1
            tmp.308 = add_ptr(struct_array.12, index=tmp.307, scale=48)
            tmp.309 = add_ptr(tmp.308, index=2L, scale=1)
            tmp.310 = *tmp.309
            tmp.311 = &string.4
            tmp.312 = strcmp(tmp.310, tmp.311)
            if tmp.312 jump or_true_90
            tmp.306 = 0
            jump or_end_91
        
          or_true_90:
            tmp.306 = 1
        
          or_end_91:
            if tmp.306 jump or_true_92
            tmp.315 = sign_extend 1
            tmp.316 = add_ptr(struct_array.12, index=tmp.315, scale=48)
            tmp.317 = add_ptr(tmp.316, index=3L, scale=1)
            tmp.318 = *tmp.317
            tmp.319 = tmp.318 != 108D
            if tmp.319 jump or_true_92
            tmp.314 = 0
            jump or_end_93
        
          or_true_92:
            tmp.314 = 1
        
          or_end_93:
            if tmp.314 jump or_true_94
            tmp.322 = sign_extend 1
            tmp.323 = add_ptr(struct_array.12, index=tmp.322, scale=48)
            tmp.324 = add_ptr(tmp.323, index=4L, scale=1)
            tmp.325 = *tmp.324
            tmp.326 = tmp.325 != 109
            if tmp.326 jump or_true_94
            tmp.321 = 0
            jump or_end_95
        
          or_true_94:
            tmp.321 = 1
        
          or_end_95:
            if tmp.321 jump or_true_96
            tmp.329 = sign_extend 1
            tmp.330 = add_ptr(struct_array.12, index=tmp.329, scale=48)
            tmp.331 = add_ptr(tmp.330, index=4L, scale=1)
            tmp.332 = add_ptr(tmp.331, index=1L, scale=1)
            tmp.333 = *tmp.332
            tmp.334 = tmp.333 != 110
            if tmp.334 jump or_true_96
            tmp.328 = 0
            jump or_end_97
        
          or_true_96:
            tmp.328 = 1
        
          or_end_97:
            if !tmp.328 jump end_if_98
            return 0
        
          end_if_98:
            tmp.335 = sign_extend 2
            tmp.336 = add_ptr(struct_array.12, index=tmp.335, scale=48)
            tmp.337 = *tmp.336
            tmp.339 = sign_extend 201
            tmp.338 = tmp.337 != tmp.339
            if tmp.338 jump or_true_100
            tmp.342 = sign_extend 2
            tmp.343 = add_ptr(struct_array.12, index=tmp.342, scale=48)
            tmp.344 = add_ptr(tmp.343, index=1L, scale=1)
            tmp.345 = *tmp.344
            tmp.346 = tmp.345 != 202
            if tmp.346 jump or_true_100
            tmp.341 = 0
            jump or_end_101
        
          or_true_100:
            tmp.341 = 1
        
          or_end_101:
            if tmp.341 jump or_true_102
            tmp.349 = sign_extend 2
            tmp.350 = add_ptr(struct_array.12, index=tmp.349, scale=48)
            tmp.351 = add_ptr(tmp.350, index=1L, scale=1)
            tmp.352 = add_ptr(tmp.351, index=1L, scale=1)
            tmp.353 = sign_extend 0
            tmp.354 = add_ptr(tmp.352, index=tmp.353, scale=1)
            tmp.355 = *tmp.354
            tmp.356 = zero_extend tmp.355
            tmp.357 = tmp.356 != 203
            if tmp.357 jump or_true_102
            tmp.348 = 0
            jump or_end_103
        
          or_true_102:
            tmp.348 = 1
        
          or_end_103:
            if tmp.348 jump or_true_104
            tmp.360 = sign_extend 2
            tmp.361 = add_ptr(struct_array.12, index=tmp.360, scale=48)
            tmp.362 = add_ptr(tmp.361, index=1L, scale=1)
            tmp.363 = add_ptr(tmp.362, index=1L, scale=1)
            tmp.364 = sign_extend 1
            tmp.365 = add_ptr(tmp.363, index=tmp.364, scale=1)
            tmp.366 = *tmp.365
            if tmp.366 jump or_true_104
            tmp.359 = 0
            jump or_end_105
        
          or_true_104:
            tmp.359 = 1
        
          or_end_105:
            if tmp.359 jump or_true_106
            tmp.369 = sign_extend 2
            tmp.370 = add_ptr(struct_array.12, index=tmp.369, scale=48)
            tmp.371 = add_ptr(tmp.370, index=1L, scale=1)
            tmp.372 = add_ptr(tmp.371, index=1L, scale=1)
            tmp.373 = sign_extend 2
            tmp.374 = add_ptr(tmp.372, index=tmp.373, scale=1)
            tmp.375 = *tmp.374
            if tmp.375 jump or_true_106
            tmp.368 = 0
            jump or_end_107
        
          or_true_106:
            tmp.368 = 1
        
          or_end_107:
            if tmp.368 jump or_true_108
            tmp.378 = sign_extend 2
            tmp.379 = add_ptr(struct_array.12, index=tmp.378, scale=48)
            tmp.380 = add_ptr(tmp.379, index=1L, scale=1)
            tmp.381 = add_ptr(tmp.380, index=2L, scale=1)
            tmp.382 = *tmp.381
            if tmp.382 jump or_true_108
            tmp.377 = 0
            jump or_end_109
        
          or_true_108:
            tmp.377 = 1
        
          or_end_109:
            if tmp.377 jump or_true_110
            tmp.385 = sign_extend 2
            tmp.386 = add_ptr(struct_array.12, index=tmp.385, scale=48)
            tmp.387 = add_ptr(tmp.386, index=2L, scale=1)
            tmp.388 = *tmp.387
            tmp.389 = &string.5
            tmp.390 = strcmp(tmp.388, tmp.389)
            if tmp.390 jump or_true_110
            tmp.384 = 0
            jump or_end_111
        
          or_true_110:
            tmp.384 = 1
        
          or_end_111:
            if tmp.384 jump or_true_112
            tmp.393 = sign_extend 2
            tmp.394 = add_ptr(struct_array.12, index=tmp.393, scale=48)
            tmp.395 = add_ptr(tmp.394, index=3L, scale=1)
            tmp.396 = *tmp.395
            tmp.397 = tmp.396 != 208D
            if tmp.397 jump or_true_112
            tmp.392 = 0
            jump or_end_113
        
          or_true_112:
            tmp.392 = 1
        
          or_end_113:
            if tmp.392 jump or_true_114
            tmp.400 = sign_extend 2
            tmp.401 = add_ptr(struct_array.12, index=tmp.400, scale=48)
            tmp.402 = add_ptr(tmp.401, index=4L, scale=1)
            tmp.403 = *tmp.402
            tmp.404 = tmp.403 != 209
            if tmp.404 jump or_true_114
            tmp.399 = 0
            jump or_end_115
        
          or_true_114:
            tmp.399 = 1
        
          or_end_115:
            if tmp.399 jump or_true_116
            tmp.407 = sign_extend 2
            tmp.408 = add_ptr(struct_array.12, index=tmp.407, scale=48)
            tmp.409 = add_ptr(tmp.408, index=4L, scale=1)
            tmp.410 = add_ptr(tmp.409, index=1L, scale=1)
            tmp.411 = *tmp.410
            if tmp.411 jump or_true_116
            tmp.406 = 0
            jump or_end_117
        
          or_true_116:
            tmp.406 = 1
        
          or_end_117:
            if !tmp.406 jump end_if_118
            return 0
        
          end_if_118:
            tmp.412 = sign_extend 3
            tmp.413 = add_ptr(struct_array.12, index=tmp.412, scale=48)
            tmp.414 = *tmp.413
            tmp.416 = sign_extend 301
            tmp.415 = tmp.414 != tmp.416
            if tmp.415 jump or_true_120
            tmp.419 = sign_extend 3
            tmp.420 = add_ptr(struct_array.12, index=tmp.419, scale=48)
            tmp.421 = add_ptr(tmp.420, index=1L, scale=1)
            tmp.422 = *tmp.421
            if tmp.422 jump or_true_120
            tmp.418 = 0
            jump or_end_121
        
          or_true_120:
            tmp.418 = 1
        
          or_end_121:
            if tmp.418 jump or_true_122
            tmp.425 = sign_extend 3
            tmp.426 = add_ptr(struct_array.12, index=tmp.425, scale=48)
            tmp.427 = add_ptr(tmp.426, index=1L, scale=1)
            tmp.428 = add_ptr(tmp.427, index=1L, scale=1)
            tmp.429 = sign_extend 0
            tmp.430 = add_ptr(tmp.428, index=tmp.429, scale=1)
            tmp.431 = *tmp.430
            if tmp.431 jump or_true_122
            tmp.424 = 0
            jump or_end_123
        
          or_true_122:
            tmp.424 = 1
        
          or_end_123:
            if tmp.424 jump or_true_124
            tmp.434 = sign_extend 3
            tmp.435 = add_ptr(struct_array.12, index=tmp.434, scale=48)
            tmp.436 = add_ptr(tmp.435, index=1L, scale=1)
            tmp.437 = add_ptr(tmp.436, index=1L, scale=1)
            tmp.438 = sign_extend 1
            tmp.439 = add_ptr(tmp.437, index=tmp.438, scale=1)
            tmp.440 = *tmp.439
            if tmp.440 jump or_true_124
            tmp.433 = 0
            jump or_end_125
        
          or_true_124:
            tmp.433 = 1
        
          or_end_125:
            if tmp.433 jump or_true_126
            tmp.443 = sign_extend 3
            tmp.444 = add_ptr(struct_array.12, index=tmp.443, scale=48)
            tmp.445 = add_ptr(tmp.444, index=1L, scale=1)
            tmp.446 = add_ptr(tmp.445, index=1L, scale=1)
            tmp.447 = sign_extend 2
            tmp.448 = add_ptr(tmp.446, index=tmp.447, scale=1)
            tmp.449 = *tmp.448
            if tmp.449 jump or_true_126
            tmp.442 = 0
            jump or_end_127
        
          or_true_126:
            tmp.442 = 1
        
          or_end_127:
            if tmp.442 jump or_true_128
            tmp.452 = sign_extend 3
            tmp.453 = add_ptr(struct_array.12, index=tmp.452, scale=48)
            tmp.454 = add_ptr(tmp.453, index=1L, scale=1)
            tmp.455 = add_ptr(tmp.454, index=2L, scale=1)
            tmp.456 = *tmp.455
            if tmp.456 jump or_true_128
            tmp.451 = 0
            jump or_end_129
        
          or_true_128:
            tmp.451 = 1
        
          or_end_129:
            if tmp.451 jump or_true_130
            tmp.459 = sign_extend 3
            tmp.460 = add_ptr(struct_array.12, index=tmp.459, scale=48)
            tmp.461 = add_ptr(tmp.460, index=2L, scale=1)
            tmp.462 = *tmp.461
            if tmp.462 jump or_true_130
            tmp.458 = 0
            jump or_end_131
        
          or_true_130:
            tmp.458 = 1
        
          or_end_131:
            if tmp.458 jump or_true_132
            tmp.465 = sign_extend 3
            tmp.466 = add_ptr(struct_array.12, index=tmp.465, scale=48)
            tmp.467 = add_ptr(tmp.466, index=3L, scale=1)
            tmp.468 = *tmp.467
            if tmp.468 jump or_true_132
            tmp.464 = 0
            jump or_end_133
        
          or_true_132:
            tmp.464 = 1
        
          or_end_133:
            if tmp.464 jump or_true_134
            tmp.471 = sign_extend 3
            tmp.472 = add_ptr(struct_array.12, index=tmp.471, scale=48)
            tmp.473 = add_ptr(tmp.472, index=4L, scale=1)
            tmp.474 = *tmp.473
            if tmp.474 jump or_true_134
            tmp.470 = 0
            jump or_end_135
        
          or_true_134:
            tmp.470 = 1
        
          or_end_135:
            if tmp.470 jump or_true_136
            tmp.477 = sign_extend 3
            tmp.478 = add_ptr(struct_array.12, index=tmp.477, scale=48)
            tmp.479 = add_ptr(tmp.478, index=4L, scale=1)
            tmp.480 = add_ptr(tmp.479, index=1L, scale=1)
            tmp.481 = *tmp.480
            if tmp.481 jump or_true_136
            tmp.476 = 0
            jump or_end_137
        
          or_true_136:
            tmp.476 = 1
        
          or_end_137:
            if !tmp.476 jump end_if_138
            return 0
        
          end_if_138:
            return 1
            return 0
        }
        constant string.0: Array(19,Char) = "Important message!\\0"
        constant string.1: Array(8,Char) = "Partial\\0"
        constant string.2: Array(6,Char) = "mixed\\0"
        constant string.3: Array(2,Char) = "7\\0"
        constant string.4: Array(4,Char) = "107\\0"
        constant string.5: Array(4,Char) = "207\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_nested_auto_struct_initializers_client(
) {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct pair {
            int a;
            int b;
        };
        struct inner {
            int one_i;
            unsigned char two_arr[3];
            unsigned three_u;
        };
        struct outer {
            long one_l;
            struct inner two_struct;
            char *three_msg;
            double four_d;
            struct pair five_pair;
        };
        int validate_full_initialization(struct outer *ptr);
        int validate_partial_initialization(struct outer *ptr);
        int validate_mixed_initialization(struct outer *ptr);
        int validate_array_of_structs(struct outer *struct_array);
        int test_full_initialization(void) {
            struct outer full = {-200,
                                 {-171l, {-56, -54, -53}, 40.5},
                                 "Important message!",
                                 -22,
                                 {1, 2}};
            return validate_full_initialization(&full);
        }
        int test_partial_initialization(void) {
            struct outer partial = {1000,
                                    {
                                        1,
                                    },
                                    "Partial"};
            return validate_partial_initialization(&partial);
        }
        int test_mixed_initialization(void) {
            struct inner inner1 = {10};
            struct inner inner2 = {20, {21}, 22u};
            static int flag = 0;
            struct outer mixed = {
                200,
                flag ? inner1 : inner2,
                "mixed",
                10.0,
                {99,
                 100}
            };
            return validate_mixed_initialization(&mixed);
        }
        int test_array_of_structs(void) {
            struct outer s0 = {1, {2, {3, 4, 5}, 6}, "7", 8.0, {9, 10}};
            struct inner in1 = {102, {103, 104, 105}, 106};
            struct pair pair1 = {109, 110};
            struct pair pair2 = {209};
            struct outer s3 = {301};
            struct outer struct_array[4] = {
                s0,
                {101, in1, "107", 108.0, pair1},
                {201,
                 {202, {203}},
                 "207",
                 208.0,
                 pair2},
                s3};
            return validate_array_of_structs(struct_array);
        }
        int main(void) {
            if (!test_full_initialization()) {
                return 1;
            }
            if (!test_partial_initialization()) {
                return 2;
            }
            if (!test_mixed_initialization()) {
                return 3;
            }
            if (!test_array_of_structs()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_full_initialization() { 
            tmp.0 = - 200
            tmp.1 = sign_extend tmp.0
            outer.4[0] = tmp.1
            tmp.2 = - 171L
            tmp.3 = truncate tmp.2
            inner.3[8] = tmp.3
            tmp.4 = - 56
            tmp.5 = truncate tmp.4
            inner.3[12] = tmp.5
            tmp.6 = - 54
            tmp.7 = truncate tmp.6
            inner.3[13] = tmp.7
            tmp.8 = - 53
            tmp.9 = truncate tmp.8
            inner.3[14] = tmp.9
            tmp.10 = double_to_uint 40.5D
            inner.3[16] = tmp.10
            tmp.11 = &string.0
            outer.4[24] = tmp.11
            tmp.12 = - 22
            tmp.13 = int_to_double tmp.12
            outer.4[32] = tmp.13
            pair.2[40] = 1
            pair.2[44] = 2
            tmp.14 = &full.9
            tmp.15 = validate_full_initialization(tmp.14)
            return tmp.15
            return 0
        }
        global function test_partial_initialization() { 
            tmp.16 = sign_extend 1000
            outer.4[0] = tmp.16
            inner.3[8] = 1
            inner.3[12] = 0UC
            inner.3[13] = 0UC
            inner.3[14] = 0UC
            inner.3[16] = 0U
            tmp.17 = &string.1
            outer.4[24] = tmp.17
            outer.4[32] = 0D
            pair.2[40] = 0
            pair.2[44] = 0
            tmp.18 = &partial.10
            tmp.19 = validate_partial_initialization(tmp.18)
            return tmp.19
            return 0
        }
        global function test_mixed_initialization() { 
            inner.3[0] = 10
            inner.3[4] = 0UC
            inner.3[5] = 0UC
            inner.3[6] = 0UC
            inner.3[8] = 0U
            inner.3[0] = 20
            tmp.20 = truncate 21
            inner.3[4] = tmp.20
            inner.3[5] = 0UC
            inner.3[6] = 0UC
            inner.3[8] = 22U
            tmp.21 = sign_extend 200
            outer.4[0] = tmp.21
            if !flag.13 jump else_1
            tmp.22 = inner1.11
            jump end_if_0
        
          else_1:
            tmp.22 = inner2.12
        
          end_if_0:
            outer.4[8] = tmp.22
            tmp.23 = &string.2
            outer.4[24] = tmp.23
            outer.4[32] = 10D
            pair.2[40] = 99
            pair.2[44] = 100
            tmp.24 = &mixed.14
            tmp.25 = validate_mixed_initialization(tmp.24)
            return tmp.25
            return 0
        }
        global function test_array_of_structs() { 
            tmp.26 = sign_extend 1
            outer.4[0] = tmp.26
            inner.3[8] = 2
            tmp.27 = truncate 3
            inner.3[12] = tmp.27
            tmp.28 = truncate 4
            inner.3[13] = tmp.28
            tmp.29 = truncate 5
            inner.3[14] = tmp.29
            tmp.30 = 6
            inner.3[16] = tmp.30
            tmp.31 = &string.3
            outer.4[24] = tmp.31
            outer.4[32] = 8D
            pair.2[40] = 9
            pair.2[44] = 10
            inner.3[0] = 102
            tmp.32 = truncate 103
            inner.3[4] = tmp.32
            tmp.33 = truncate 104
            inner.3[5] = tmp.33
            tmp.34 = truncate 105
            inner.3[6] = tmp.34
            tmp.35 = 106
            inner.3[8] = tmp.35
            pair.2[0] = 109
            pair.2[4] = 110
            pair.2[0] = 209
            pair.2[4] = 0
            tmp.36 = sign_extend 301
            outer.4[0] = tmp.36
            inner.3[8] = 0
            inner.3[12] = 0UC
            inner.3[13] = 0UC
            inner.3[14] = 0UC
            inner.3[16] = 0U
            outer.4[24] = 0UL
            outer.4[32] = 0D
            pair.2[40] = 0
            pair.2[44] = 0
            struct_array.20[0] = s0.15
            tmp.37 = sign_extend 101
            outer.4[48] = tmp.37
            outer.4[56] = in1.16
            tmp.38 = &string.4
            outer.4[72] = tmp.38
            outer.4[80] = 108D
            outer.4[88] = pair1.17
            tmp.39 = sign_extend 201
            outer.4[96] = tmp.39
            inner.3[104] = 202
            tmp.40 = truncate 203
            inner.3[108] = tmp.40
            inner.3[109] = 0UC
            inner.3[110] = 0UC
            inner.3[112] = 0U
            tmp.41 = &string.5
            outer.4[120] = tmp.41
            outer.4[128] = 208D
            outer.4[136] = pair2.18
            struct_array.20[144] = s3.19
            tmp.42 = &struct_array.20
            tmp.43 = validate_array_of_structs(tmp.42)
            return tmp.43
            return 0
        }
        global function main() { 
            tmp.44 = test_full_initialization()
            tmp.45 = ! tmp.44
            if !tmp.45 jump end_if_2
            return 1
        
          end_if_2:
            tmp.46 = test_partial_initialization()
            tmp.47 = ! tmp.46
            if !tmp.47 jump end_if_4
            return 2
        
          end_if_4:
            tmp.48 = test_mixed_initialization()
            tmp.49 = ! tmp.48
            if !tmp.49 jump end_if_6
            return 3
        
          end_if_6:
            tmp.50 = test_array_of_structs()
            tmp.51 = ! tmp.50
            if !tmp.51 jump end_if_8
            return 4
        
          end_if_8:
            return 0
            return 0
        }
        static flag.13: Int = 0
        constant string.0: Array(19,Char) = "Important message!\\0"
        constant string.1: Array(8,Char) = "Partial\\0"
        constant string.2: Array(6,Char) = "mixed\\0"
        constant string.3: Array(2,Char) = "7\\0"
        constant string.4: Array(4,Char) = "107\\0"
        constant string.5: Array(4,Char) = "207\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_nested_static_struct_initializers() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct inner {
            int one_i;
            signed char two_arr[3];
            unsigned three_u;
        };
        struct outer {
            long one_l;
            struct inner two_struct;
            char *three_msg;
            double four_d;
        };
        extern struct outer all_zeros;
        extern struct outer partial;
        extern struct outer full;
        extern struct outer converted;
        extern struct outer struct_array[3];
        int test_uninitialized(void);
        int test_partially_initialized(void);
        int test_fully_intialized(void);
        int test_implicit_conversions(void);
        int test_array_of_structs(void);
        int test_uninitialized(void) {
            if (all_zeros.one_l || all_zeros.three_msg || all_zeros.four_d) {
                return 0;
            }
            if (all_zeros.two_struct.one_i || all_zeros.two_struct.two_arr[0] ||
                all_zeros.two_struct.two_arr[1] || all_zeros.two_struct.two_arr[2] ||
                all_zeros.two_struct.three_u) {
                return 0;
            }
            return 1;
        }
        int test_partially_initialized(void) {
            if (partial.one_l != 100l || strcmp(partial.three_msg, "Hello!")) {
                return 0;
            }
            if (partial.four_d) {
                return 0;
            }
            if (partial.two_struct.one_i != 10 || partial.two_struct.two_arr[0] != 10) {
                return 0;
            }
            if (partial.two_struct.two_arr[1] || partial.two_struct.two_arr[2] ||
                partial.two_struct
                    .three_u) {
                return 0;
            }
            return 1;
        }
        int test_fully_intialized(void) {
            if (full.one_l != 18014398509481979l ||
                strcmp(full.three_msg, "Another message") || full.four_d != 2e12) {
                return 0;
            }
            if (full.two_struct.one_i != 1000 || full.two_struct.two_arr[0] != 'o' ||
                full.two_struct.two_arr[1] != 'k' || full.two_struct.two_arr[2] != 0 ||
                full.two_struct.three_u != 4292870144u) {
                return 0;
            }
            return 1;
        }
        int test_implicit_conversions(void) {
            if (converted.one_l != 10l || converted.three_msg != 0 ||
                converted.four_d != 9223372036854777856.0) {
                return 0;
            }
            if (converted.two_struct.one_i != -2147483646 ||
                converted.two_struct.two_arr[0] != 15 ||
                converted.two_struct.two_arr[1] != 3 ||
                converted.two_struct.two_arr[2] != -127 ||
                converted.two_struct.three_u != 2147483648u) {
                return 0;
            }
            return 1;
        }
        int test_array_of_structs(void) {
            if (struct_array[0].one_l != 1 || struct_array[0].three_msg != 0 ||
                struct_array[0].four_d != 5) {
                return 0;
            }
            if (struct_array[0].two_struct.one_i != 2 ||
                strcmp((char *)struct_array[0].two_struct.two_arr, "ab") ||
                struct_array[0].two_struct.three_u != 3) {
                return 0;
            }
            if (struct_array[1].one_l != 6 ||
                strcmp((char *)struct_array[1].three_msg, "Message") ||
                struct_array[1].four_d != 9) {
                return 0;
            }
            if (struct_array[1].two_struct.one_i != 7 ||
                strcmp((char *)struct_array[1].two_struct.two_arr, "cd") ||
                struct_array[1].two_struct.three_u != 8) {
                return 0;
            }
            if (struct_array[2].one_l || struct_array[2].three_msg ||
                struct_array[2].four_d) {
                return 0;
            }
            if (struct_array[2].two_struct.one_i ||
                struct_array[2].two_struct.two_arr[0] ||
                struct_array[2].two_struct.two_arr[1] ||
                struct_array[2].two_struct.two_arr[2] ||
                struct_array[2].two_struct.three_u) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function test_uninitialized() { 
            tmp.0 = all_zeros[0]
            if tmp.0 jump or_true_0
            tmp.3 = all_zeros[2]
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if tmp.2 jump or_true_2
            tmp.6 = all_zeros[3]
            if tmp.6 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            if !tmp.5 jump end_if_4
            return 0
        
          end_if_4:
            tmp.7 = all_zeros[1]
            if tmp.7 jump or_true_6
            tmp.10 = all_zeros[2]
            tmp.11 = &tmp.10
            tmp.12 = sign_extend 0
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=1)
            tmp.14 = *tmp.13
            if tmp.14 jump or_true_6
            tmp.9 = 0
            jump or_end_7
        
          or_true_6:
            tmp.9 = 1
        
          or_end_7:
            if tmp.9 jump or_true_8
            tmp.17 = all_zeros[2]
            tmp.18 = &tmp.17
            tmp.19 = sign_extend 1
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=1)
            tmp.21 = *tmp.20
            if tmp.21 jump or_true_8
            tmp.16 = 0
            jump or_end_9
        
          or_true_8:
            tmp.16 = 1
        
          or_end_9:
            if tmp.16 jump or_true_10
            tmp.24 = all_zeros[2]
            tmp.25 = &tmp.24
            tmp.26 = sign_extend 2
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=1)
            tmp.28 = *tmp.27
            if tmp.28 jump or_true_10
            tmp.23 = 0
            jump or_end_11
        
          or_true_10:
            tmp.23 = 1
        
          or_end_11:
            if tmp.23 jump or_true_12
            tmp.31 = all_zeros[3]
            if tmp.31 jump or_true_12
            tmp.30 = 0
            jump or_end_13
        
          or_true_12:
            tmp.30 = 1
        
          or_end_13:
            if !tmp.30 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_partially_initialized() { 
            tmp.32 = partial[0]
            tmp.33 = tmp.32 != 100L
            if tmp.33 jump or_true_16
            tmp.36 = partial[2]
            tmp.37 = &string.0
            tmp.38 = strcmp(tmp.36, tmp.37)
            if tmp.38 jump or_true_16
            tmp.35 = 0
            jump or_end_17
        
          or_true_16:
            tmp.35 = 1
        
          or_end_17:
            if !tmp.35 jump end_if_18
            return 0
        
          end_if_18:
            tmp.39 = partial[3]
            tmp.40 = tmp.39 != 0D
            if !tmp.40 jump end_if_20
            return 0
        
          end_if_20:
            tmp.41 = partial[1]
            tmp.42 = tmp.41 != 10
            if tmp.42 jump or_true_22
            tmp.45 = partial[2]
            tmp.46 = &tmp.45
            tmp.47 = sign_extend 0
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=1)
            tmp.49 = *tmp.48
            tmp.50 = sign_extend tmp.49
            tmp.51 = tmp.50 != 10
            if tmp.51 jump or_true_22
            tmp.44 = 0
            jump or_end_23
        
          or_true_22:
            tmp.44 = 1
        
          or_end_23:
            if !tmp.44 jump end_if_24
            return 0
        
          end_if_24:
            tmp.52 = partial[2]
            tmp.53 = &tmp.52
            tmp.54 = sign_extend 1
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=1)
            tmp.56 = *tmp.55
            if tmp.56 jump or_true_26
            tmp.59 = partial[2]
            tmp.60 = &tmp.59
            tmp.61 = sign_extend 2
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=1)
            tmp.63 = *tmp.62
            if tmp.63 jump or_true_26
            tmp.58 = 0
            jump or_end_27
        
          or_true_26:
            tmp.58 = 1
        
          or_end_27:
            if tmp.58 jump or_true_28
            tmp.66 = partial[3]
            if tmp.66 jump or_true_28
            tmp.65 = 0
            jump or_end_29
        
          or_true_28:
            tmp.65 = 1
        
          or_end_29:
            if !tmp.65 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_fully_intialized() { 
            tmp.67 = full[0]
            tmp.68 = tmp.67 != 18014398509481979L
            if tmp.68 jump or_true_32
            tmp.71 = full[2]
            tmp.72 = &string.1
            tmp.73 = strcmp(tmp.71, tmp.72)
            if tmp.73 jump or_true_32
            tmp.70 = 0
            jump or_end_33
        
          or_true_32:
            tmp.70 = 1
        
          or_end_33:
            if tmp.70 jump or_true_34
            tmp.76 = full[3]
            tmp.77 = tmp.76 != 2000000000000D
            if tmp.77 jump or_true_34
            tmp.75 = 0
            jump or_end_35
        
          or_true_34:
            tmp.75 = 1
        
          or_end_35:
            if !tmp.75 jump end_if_36
            return 0
        
          end_if_36:
            tmp.78 = full[1]
            tmp.79 = tmp.78 != 1000
            if tmp.79 jump or_true_38
            tmp.82 = full[2]
            tmp.83 = &tmp.82
            tmp.84 = sign_extend 0
            tmp.85 = add_ptr(tmp.83, index=tmp.84, scale=1)
            tmp.86 = *tmp.85
            tmp.87 = sign_extend tmp.86
            tmp.88 = tmp.87 != 111
            if tmp.88 jump or_true_38
            tmp.81 = 0
            jump or_end_39
        
          or_true_38:
            tmp.81 = 1
        
          or_end_39:
            if tmp.81 jump or_true_40
            tmp.91 = full[2]
            tmp.92 = &tmp.91
            tmp.93 = sign_extend 1
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=1)
            tmp.95 = *tmp.94
            tmp.96 = sign_extend tmp.95
            tmp.97 = tmp.96 != 107
            if tmp.97 jump or_true_40
            tmp.90 = 0
            jump or_end_41
        
          or_true_40:
            tmp.90 = 1
        
          or_end_41:
            if tmp.90 jump or_true_42
            tmp.100 = full[2]
            tmp.101 = &tmp.100
            tmp.102 = sign_extend 2
            tmp.103 = add_ptr(tmp.101, index=tmp.102, scale=1)
            tmp.104 = *tmp.103
            tmp.105 = sign_extend tmp.104
            tmp.106 = tmp.105 != 0
            if tmp.106 jump or_true_42
            tmp.99 = 0
            jump or_end_43
        
          or_true_42:
            tmp.99 = 1
        
          or_end_43:
            if tmp.99 jump or_true_44
            tmp.109 = full[3]
            tmp.110 = tmp.109 != 4292870144U
            if tmp.110 jump or_true_44
            tmp.108 = 0
            jump or_end_45
        
          or_true_44:
            tmp.108 = 1
        
          or_end_45:
            if !tmp.108 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function test_implicit_conversions() { 
            tmp.111 = converted[0]
            tmp.112 = tmp.111 != 10L
            if tmp.112 jump or_true_48
            tmp.115 = converted[2]
            tmp.117 = sign_extend 0
            tmp.116 = tmp.115 != tmp.117
            if tmp.116 jump or_true_48
            tmp.114 = 0
            jump or_end_49
        
          or_true_48:
            tmp.114 = 1
        
          or_end_49:
            if tmp.114 jump or_true_50
            tmp.120 = converted[3]
            tmp.121 = tmp.120 != 9223372036854778000D
            if tmp.121 jump or_true_50
            tmp.119 = 0
            jump or_end_51
        
          or_true_50:
            tmp.119 = 1
        
          or_end_51:
            if !tmp.119 jump end_if_52
            return 0
        
          end_if_52:
            tmp.122 = converted[1]
            tmp.124 = - 2147483646
            tmp.123 = tmp.122 != tmp.124
            if tmp.123 jump or_true_54
            tmp.127 = converted[2]
            tmp.128 = &tmp.127
            tmp.129 = sign_extend 0
            tmp.130 = add_ptr(tmp.128, index=tmp.129, scale=1)
            tmp.131 = *tmp.130
            tmp.132 = sign_extend tmp.131
            tmp.133 = tmp.132 != 15
            if tmp.133 jump or_true_54
            tmp.126 = 0
            jump or_end_55
        
          or_true_54:
            tmp.126 = 1
        
          or_end_55:
            if tmp.126 jump or_true_56
            tmp.136 = converted[2]
            tmp.137 = &tmp.136
            tmp.138 = sign_extend 1
            tmp.139 = add_ptr(tmp.137, index=tmp.138, scale=1)
            tmp.140 = *tmp.139
            tmp.141 = sign_extend tmp.140
            tmp.142 = tmp.141 != 3
            if tmp.142 jump or_true_56
            tmp.135 = 0
            jump or_end_57
        
          or_true_56:
            tmp.135 = 1
        
          or_end_57:
            if tmp.135 jump or_true_58
            tmp.145 = converted[2]
            tmp.146 = &tmp.145
            tmp.147 = sign_extend 2
            tmp.148 = add_ptr(tmp.146, index=tmp.147, scale=1)
            tmp.149 = *tmp.148
            tmp.150 = sign_extend tmp.149
            tmp.152 = - 127
            tmp.151 = tmp.150 != tmp.152
            if tmp.151 jump or_true_58
            tmp.144 = 0
            jump or_end_59
        
          or_true_58:
            tmp.144 = 1
        
          or_end_59:
            if tmp.144 jump or_true_60
            tmp.155 = converted[3]
            tmp.156 = tmp.155 != 2147483648U
            if tmp.156 jump or_true_60
            tmp.154 = 0
            jump or_end_61
        
          or_true_60:
            tmp.154 = 1
        
          or_end_61:
            if !tmp.154 jump end_if_62
            return 0
        
          end_if_62:
            return 1
            return 0
        }
        global function test_array_of_structs() { 
            tmp.157 = &struct_array
            tmp.158 = sign_extend 0
            tmp.159 = add_ptr(tmp.157, index=tmp.158, scale=40)
            tmp.160 = *tmp.159
            tmp.162 = sign_extend 1
            tmp.161 = tmp.160 != tmp.162
            if tmp.161 jump or_true_64
            tmp.165 = &struct_array
            tmp.166 = sign_extend 0
            tmp.167 = add_ptr(tmp.165, index=tmp.166, scale=40)
            tmp.168 = add_ptr(tmp.167, index=2L, scale=1)
            tmp.169 = *tmp.168
            tmp.171 = sign_extend 0
            tmp.170 = tmp.169 != tmp.171
            if tmp.170 jump or_true_64
            tmp.164 = 0
            jump or_end_65
        
          or_true_64:
            tmp.164 = 1
        
          or_end_65:
            if tmp.164 jump or_true_66
            tmp.174 = &struct_array
            tmp.175 = sign_extend 0
            tmp.176 = add_ptr(tmp.174, index=tmp.175, scale=40)
            tmp.177 = add_ptr(tmp.176, index=3L, scale=1)
            tmp.178 = *tmp.177
            tmp.180 = int_to_double 5
            tmp.179 = tmp.178 != tmp.180
            if tmp.179 jump or_true_66
            tmp.173 = 0
            jump or_end_67
        
          or_true_66:
            tmp.173 = 1
        
          or_end_67:
            if !tmp.173 jump end_if_68
            return 0
        
          end_if_68:
            tmp.181 = &struct_array
            tmp.182 = sign_extend 0
            tmp.183 = add_ptr(tmp.181, index=tmp.182, scale=40)
            tmp.184 = add_ptr(tmp.183, index=1L, scale=1)
            tmp.185 = *tmp.184
            tmp.186 = tmp.185 != 2
            if tmp.186 jump or_true_70
            tmp.189 = &struct_array
            tmp.190 = sign_extend 0
            tmp.191 = add_ptr(tmp.189, index=tmp.190, scale=40)
            tmp.192 = add_ptr(tmp.191, index=1L, scale=1)
            tmp.193 = add_ptr(tmp.192, index=1L, scale=1)
            tmp.194 = tmp.193
            tmp.195 = &string.2
            tmp.196 = strcmp(tmp.194, tmp.195)
            if tmp.196 jump or_true_70
            tmp.188 = 0
            jump or_end_71
        
          or_true_70:
            tmp.188 = 1
        
          or_end_71:
            if tmp.188 jump or_true_72
            tmp.199 = &struct_array
            tmp.200 = sign_extend 0
            tmp.201 = add_ptr(tmp.199, index=tmp.200, scale=40)
            tmp.202 = add_ptr(tmp.201, index=1L, scale=1)
            tmp.203 = add_ptr(tmp.202, index=2L, scale=1)
            tmp.204 = *tmp.203
            tmp.206 = 3
            tmp.205 = tmp.204 != tmp.206
            if tmp.205 jump or_true_72
            tmp.198 = 0
            jump or_end_73
        
          or_true_72:
            tmp.198 = 1
        
          or_end_73:
            if !tmp.198 jump end_if_74
            return 0
        
          end_if_74:
            tmp.207 = &struct_array
            tmp.208 = sign_extend 1
            tmp.209 = add_ptr(tmp.207, index=tmp.208, scale=40)
            tmp.210 = *tmp.209
            tmp.212 = sign_extend 6
            tmp.211 = tmp.210 != tmp.212
            if tmp.211 jump or_true_76
            tmp.215 = &struct_array
            tmp.216 = sign_extend 1
            tmp.217 = add_ptr(tmp.215, index=tmp.216, scale=40)
            tmp.218 = add_ptr(tmp.217, index=2L, scale=1)
            tmp.219 = *tmp.218
            tmp.220 = &string.3
            tmp.221 = strcmp(tmp.219, tmp.220)
            if tmp.221 jump or_true_76
            tmp.214 = 0
            jump or_end_77
        
          or_true_76:
            tmp.214 = 1
        
          or_end_77:
            if tmp.214 jump or_true_78
            tmp.224 = &struct_array
            tmp.225 = sign_extend 1
            tmp.226 = add_ptr(tmp.224, index=tmp.225, scale=40)
            tmp.227 = add_ptr(tmp.226, index=3L, scale=1)
            tmp.228 = *tmp.227
            tmp.230 = int_to_double 9
            tmp.229 = tmp.228 != tmp.230
            if tmp.229 jump or_true_78
            tmp.223 = 0
            jump or_end_79
        
          or_true_78:
            tmp.223 = 1
        
          or_end_79:
            if !tmp.223 jump end_if_80
            return 0
        
          end_if_80:
            tmp.231 = &struct_array
            tmp.232 = sign_extend 1
            tmp.233 = add_ptr(tmp.231, index=tmp.232, scale=40)
            tmp.234 = add_ptr(tmp.233, index=1L, scale=1)
            tmp.235 = *tmp.234
            tmp.236 = tmp.235 != 7
            if tmp.236 jump or_true_82
            tmp.239 = &struct_array
            tmp.240 = sign_extend 1
            tmp.241 = add_ptr(tmp.239, index=tmp.240, scale=40)
            tmp.242 = add_ptr(tmp.241, index=1L, scale=1)
            tmp.243 = add_ptr(tmp.242, index=1L, scale=1)
            tmp.244 = tmp.243
            tmp.245 = &string.4
            tmp.246 = strcmp(tmp.244, tmp.245)
            if tmp.246 jump or_true_82
            tmp.238 = 0
            jump or_end_83
        
          or_true_82:
            tmp.238 = 1
        
          or_end_83:
            if tmp.238 jump or_true_84
            tmp.249 = &struct_array
            tmp.250 = sign_extend 1
            tmp.251 = add_ptr(tmp.249, index=tmp.250, scale=40)
            tmp.252 = add_ptr(tmp.251, index=1L, scale=1)
            tmp.253 = add_ptr(tmp.252, index=2L, scale=1)
            tmp.254 = *tmp.253
            tmp.256 = 8
            tmp.255 = tmp.254 != tmp.256
            if tmp.255 jump or_true_84
            tmp.248 = 0
            jump or_end_85
        
          or_true_84:
            tmp.248 = 1
        
          or_end_85:
            if !tmp.248 jump end_if_86
            return 0
        
          end_if_86:
            tmp.257 = &struct_array
            tmp.258 = sign_extend 2
            tmp.259 = add_ptr(tmp.257, index=tmp.258, scale=40)
            tmp.260 = *tmp.259
            if tmp.260 jump or_true_88
            tmp.263 = &struct_array
            tmp.264 = sign_extend 2
            tmp.265 = add_ptr(tmp.263, index=tmp.264, scale=40)
            tmp.266 = add_ptr(tmp.265, index=2L, scale=1)
            tmp.267 = *tmp.266
            if tmp.267 jump or_true_88
            tmp.262 = 0
            jump or_end_89
        
          or_true_88:
            tmp.262 = 1
        
          or_end_89:
            if tmp.262 jump or_true_90
            tmp.270 = &struct_array
            tmp.271 = sign_extend 2
            tmp.272 = add_ptr(tmp.270, index=tmp.271, scale=40)
            tmp.273 = add_ptr(tmp.272, index=3L, scale=1)
            tmp.274 = *tmp.273
            if tmp.274 jump or_true_90
            tmp.269 = 0
            jump or_end_91
        
          or_true_90:
            tmp.269 = 1
        
          or_end_91:
            if !tmp.269 jump end_if_92
            return 0
        
          end_if_92:
            tmp.275 = &struct_array
            tmp.276 = sign_extend 2
            tmp.277 = add_ptr(tmp.275, index=tmp.276, scale=40)
            tmp.278 = add_ptr(tmp.277, index=1L, scale=1)
            tmp.279 = *tmp.278
            if tmp.279 jump or_true_94
            tmp.282 = &struct_array
            tmp.283 = sign_extend 2
            tmp.284 = add_ptr(tmp.282, index=tmp.283, scale=40)
            tmp.285 = add_ptr(tmp.284, index=1L, scale=1)
            tmp.286 = add_ptr(tmp.285, index=1L, scale=1)
            tmp.287 = sign_extend 0
            tmp.288 = add_ptr(tmp.286, index=tmp.287, scale=1)
            tmp.289 = *tmp.288
            if tmp.289 jump or_true_94
            tmp.281 = 0
            jump or_end_95
        
          or_true_94:
            tmp.281 = 1
        
          or_end_95:
            if tmp.281 jump or_true_96
            tmp.292 = &struct_array
            tmp.293 = sign_extend 2
            tmp.294 = add_ptr(tmp.292, index=tmp.293, scale=40)
            tmp.295 = add_ptr(tmp.294, index=1L, scale=1)
            tmp.296 = add_ptr(tmp.295, index=1L, scale=1)
            tmp.297 = sign_extend 1
            tmp.298 = add_ptr(tmp.296, index=tmp.297, scale=1)
            tmp.299 = *tmp.298
            if tmp.299 jump or_true_96
            tmp.291 = 0
            jump or_end_97
        
          or_true_96:
            tmp.291 = 1
        
          or_end_97:
            if tmp.291 jump or_true_98
            tmp.302 = &struct_array
            tmp.303 = sign_extend 2
            tmp.304 = add_ptr(tmp.302, index=tmp.303, scale=40)
            tmp.305 = add_ptr(tmp.304, index=1L, scale=1)
            tmp.306 = add_ptr(tmp.305, index=1L, scale=1)
            tmp.307 = sign_extend 2
            tmp.308 = add_ptr(tmp.306, index=tmp.307, scale=1)
            tmp.309 = *tmp.308
            if tmp.309 jump or_true_98
            tmp.301 = 0
            jump or_end_99
        
          or_true_98:
            tmp.301 = 1
        
          or_end_99:
            if tmp.301 jump or_true_100
            tmp.312 = &struct_array
            tmp.313 = sign_extend 2
            tmp.314 = add_ptr(tmp.312, index=tmp.313, scale=40)
            tmp.315 = add_ptr(tmp.314, index=1L, scale=1)
            tmp.316 = add_ptr(tmp.315, index=2L, scale=1)
            tmp.317 = *tmp.316
            if tmp.317 jump or_true_100
            tmp.311 = 0
            jump or_end_101
        
          or_true_100:
            tmp.311 = 1
        
          or_end_101:
            if !tmp.311 jump end_if_102
            return 0
        
          end_if_102:
            return 1
            return 0
        }
        constant string.0: Array(7,Char) = "Hello!\\0"
        constant string.1: Array(16,Char) = "Another message\\0"
        constant string.2: Array(3,Char) = "ab\\0"
        constant string.3: Array(8,Char) = "Message\\0"
        constant string.4: Array(3,Char) = "cd\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_nested_static_struct_initializers_client(
) {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct inner {
            int one_i;
            signed char two_arr[3];
            unsigned three_u;
        };
        struct outer {
            long one_l;
            struct inner two_struct;
            char *three_msg;
            double four_d;
        };
        extern struct outer all_zeros;
        extern struct outer partial;
        extern struct outer full;
        extern struct outer converted;
        extern struct outer struct_array[3];
        int test_uninitialized(void);
        int test_partially_initialized(void);
        int test_fully_intialized(void);
        int test_implicit_conversions(void);
        int test_array_of_structs(void);
        struct outer all_zeros;
        struct outer partial = {
            100l,
            {10, {10}},
            "Hello!"};
        struct outer full = {
            18014398509481979l,
            {1000, "ok",
             4292870144u},
            "Another message",
            2e12};
        struct outer converted = {
            10.5,
            {
                2147483650u,
                {
                    15.6,
                    17592186044419l,
                    2147483777u
                },
                1152921506754330624ul
            },
            0ul,
            9223372036854776833ul
        };
        struct outer struct_array[3] = {{1, {2, "ab", 3}, 0, 5},
                                        {6, {7, "cd", 8}, "Message", 9}};
        int main(void) {
            if (!test_uninitialized()) {
                return 1;
            }
            if (!test_partially_initialized()) {
                return 2;
            }
            if (!test_fully_intialized()) {
                return 3;
            }
            if (!test_implicit_conversions()) {
                return 4;
            }
            if (!test_array_of_structs()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = test_uninitialized()
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = test_partially_initialized()
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = test_fully_intialized()
            tmp.5 = ! tmp.4
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.6 = test_implicit_conversions()
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            tmp.8 = test_array_of_structs()
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static global all_zeros: Struct(outer.3) = zero[40]
        static global converted: Struct(outer.3) = [ 10L, -2147483646, 15, '\u{3}', '\u{81}', zero[1], 2147483648U, zero[4], 0UL, 9223372036854778000D]
        static global full: Struct(outer.3) = [ 18014398509481979L, 1000, "ok\\0", zero[1], 4292870144U, zero[4], &Another message, 2000000000000D]
        static global partial: Struct(outer.3) = [ 100L, 10, '\n', zero[2], zero[5], zero[4], &Hello!, zero[8]]
        static global struct_array: Array(3,Struct(outer.3)) = [ 1L, 2, "ab\\0", zero[1], 3U, zero[4], 0UL, 5D, 6L, 7, "cd\\0", zero[1], 8U, zero[4], &Message, 9D, zero[40]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_static_struct_initializers() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct s {
            double one_d;
            char *two_msg;
            unsigned char three_arr[3];
            int four_i;
        };
        extern struct s uninitialized;
        extern struct s partial;
        extern struct s partial_with_array;
        extern struct s converted;
        int test_uninitialized(void);
        int test_partially_initialized(void);
        int test_partial_inner_init(void);
        int test_implicit_conversion(void);
        int test_uninitialized(void) {
            if (uninitialized.one_d || uninitialized.two_msg ||
                uninitialized.three_arr[0] || uninitialized.three_arr[1] ||
                uninitialized.three_arr[2] || uninitialized.four_i) {
                return 0;
            }
            return 1;
        }
        int test_partially_initialized(void) {
            if (partial.one_d != 1.0 || strcmp(partial.two_msg, "Hello")) {
                return 0;
            }
            if (partial.three_arr[0] || partial.three_arr[1] || partial.three_arr[2] ||
                partial.four_i) {
                return 0;
            }
            return 1;
        }
        int test_partial_inner_init(void) {
            if (partial_with_array.one_d != 3.0 ||
                strcmp(partial_with_array.two_msg, "!") ||
                partial_with_array.three_arr[0] != 1 ||
                partial_with_array.four_i != 2) {
                return 0;
            }
            if (partial_with_array.three_arr[1] || partial_with_array.three_arr[2]) {
                return 0;
            }
            return 1;
        }
        int test_implicit_conversion(void) {
            if (converted.one_d != 1152921504606846976.0 || converted.two_msg ||
                converted.three_arr[0] != 'a' || converted.three_arr[1] != 'b' ||
                converted.three_arr[2] != 'c' || converted.four_i != 5) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function test_uninitialized() { 
            tmp.0 = uninitialized[0]
            if tmp.0 jump or_true_0
            tmp.3 = uninitialized[1]
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if tmp.2 jump or_true_2
            tmp.6 = uninitialized[2]
            tmp.7 = &tmp.6
            tmp.8 = sign_extend 0
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=1)
            tmp.10 = *tmp.9
            if tmp.10 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            if tmp.5 jump or_true_4
            tmp.13 = uninitialized[2]
            tmp.14 = &tmp.13
            tmp.15 = sign_extend 1
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=1)
            tmp.17 = *tmp.16
            if tmp.17 jump or_true_4
            tmp.12 = 0
            jump or_end_5
        
          or_true_4:
            tmp.12 = 1
        
          or_end_5:
            if tmp.12 jump or_true_6
            tmp.20 = uninitialized[2]
            tmp.21 = &tmp.20
            tmp.22 = sign_extend 2
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            if tmp.24 jump or_true_6
            tmp.19 = 0
            jump or_end_7
        
          or_true_6:
            tmp.19 = 1
        
          or_end_7:
            if tmp.19 jump or_true_8
            tmp.27 = uninitialized[3]
            if tmp.27 jump or_true_8
            tmp.26 = 0
            jump or_end_9
        
          or_true_8:
            tmp.26 = 1
        
          or_end_9:
            if !tmp.26 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_partially_initialized() { 
            tmp.28 = partial[0]
            tmp.29 = tmp.28 != 1D
            if tmp.29 jump or_true_12
            tmp.32 = partial[1]
            tmp.33 = &string.0
            tmp.34 = strcmp(tmp.32, tmp.33)
            if tmp.34 jump or_true_12
            tmp.31 = 0
            jump or_end_13
        
          or_true_12:
            tmp.31 = 1
        
          or_end_13:
            if !tmp.31 jump end_if_14
            return 0
        
          end_if_14:
            tmp.35 = partial[2]
            tmp.36 = &tmp.35
            tmp.37 = sign_extend 0
            tmp.38 = add_ptr(tmp.36, index=tmp.37, scale=1)
            tmp.39 = *tmp.38
            if tmp.39 jump or_true_16
            tmp.42 = partial[2]
            tmp.43 = &tmp.42
            tmp.44 = sign_extend 1
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=1)
            tmp.46 = *tmp.45
            if tmp.46 jump or_true_16
            tmp.41 = 0
            jump or_end_17
        
          or_true_16:
            tmp.41 = 1
        
          or_end_17:
            if tmp.41 jump or_true_18
            tmp.49 = partial[2]
            tmp.50 = &tmp.49
            tmp.51 = sign_extend 2
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=1)
            tmp.53 = *tmp.52
            if tmp.53 jump or_true_18
            tmp.48 = 0
            jump or_end_19
        
          or_true_18:
            tmp.48 = 1
        
          or_end_19:
            if tmp.48 jump or_true_20
            tmp.56 = partial[3]
            if tmp.56 jump or_true_20
            tmp.55 = 0
            jump or_end_21
        
          or_true_20:
            tmp.55 = 1
        
          or_end_21:
            if !tmp.55 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function test_partial_inner_init() { 
            tmp.57 = partial_with_array[0]
            tmp.58 = tmp.57 != 3D
            if tmp.58 jump or_true_24
            tmp.61 = partial_with_array[1]
            tmp.62 = &string.1
            tmp.63 = strcmp(tmp.61, tmp.62)
            if tmp.63 jump or_true_24
            tmp.60 = 0
            jump or_end_25
        
          or_true_24:
            tmp.60 = 1
        
          or_end_25:
            if tmp.60 jump or_true_26
            tmp.66 = partial_with_array[2]
            tmp.67 = &tmp.66
            tmp.68 = sign_extend 0
            tmp.69 = add_ptr(tmp.67, index=tmp.68, scale=1)
            tmp.70 = *tmp.69
            tmp.71 = zero_extend tmp.70
            tmp.72 = tmp.71 != 1
            if tmp.72 jump or_true_26
            tmp.65 = 0
            jump or_end_27
        
          or_true_26:
            tmp.65 = 1
        
          or_end_27:
            if tmp.65 jump or_true_28
            tmp.75 = partial_with_array[3]
            tmp.76 = tmp.75 != 2
            if tmp.76 jump or_true_28
            tmp.74 = 0
            jump or_end_29
        
          or_true_28:
            tmp.74 = 1
        
          or_end_29:
            if !tmp.74 jump end_if_30
            return 0
        
          end_if_30:
            tmp.77 = partial_with_array[2]
            tmp.78 = &tmp.77
            tmp.79 = sign_extend 1
            tmp.80 = add_ptr(tmp.78, index=tmp.79, scale=1)
            tmp.81 = *tmp.80
            if tmp.81 jump or_true_32
            tmp.84 = partial_with_array[2]
            tmp.85 = &tmp.84
            tmp.86 = sign_extend 2
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=1)
            tmp.88 = *tmp.87
            if tmp.88 jump or_true_32
            tmp.83 = 0
            jump or_end_33
        
          or_true_32:
            tmp.83 = 1
        
          or_end_33:
            if !tmp.83 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function test_implicit_conversion() { 
            tmp.89 = converted[0]
            tmp.90 = tmp.89 != 1152921504606847000D
            if tmp.90 jump or_true_36
            tmp.93 = converted[1]
            if tmp.93 jump or_true_36
            tmp.92 = 0
            jump or_end_37
        
          or_true_36:
            tmp.92 = 1
        
          or_end_37:
            if tmp.92 jump or_true_38
            tmp.96 = converted[2]
            tmp.97 = &tmp.96
            tmp.98 = sign_extend 0
            tmp.99 = add_ptr(tmp.97, index=tmp.98, scale=1)
            tmp.100 = *tmp.99
            tmp.101 = zero_extend tmp.100
            tmp.102 = tmp.101 != 97
            if tmp.102 jump or_true_38
            tmp.95 = 0
            jump or_end_39
        
          or_true_38:
            tmp.95 = 1
        
          or_end_39:
            if tmp.95 jump or_true_40
            tmp.105 = converted[2]
            tmp.106 = &tmp.105
            tmp.107 = sign_extend 1
            tmp.108 = add_ptr(tmp.106, index=tmp.107, scale=1)
            tmp.109 = *tmp.108
            tmp.110 = zero_extend tmp.109
            tmp.111 = tmp.110 != 98
            if tmp.111 jump or_true_40
            tmp.104 = 0
            jump or_end_41
        
          or_true_40:
            tmp.104 = 1
        
          or_end_41:
            if tmp.104 jump or_true_42
            tmp.114 = converted[2]
            tmp.115 = &tmp.114
            tmp.116 = sign_extend 2
            tmp.117 = add_ptr(tmp.115, index=tmp.116, scale=1)
            tmp.118 = *tmp.117
            tmp.119 = zero_extend tmp.118
            tmp.120 = tmp.119 != 99
            if tmp.120 jump or_true_42
            tmp.113 = 0
            jump or_end_43
        
          or_true_42:
            tmp.113 = 1
        
          or_end_43:
            if tmp.113 jump or_true_44
            tmp.123 = converted[3]
            tmp.124 = tmp.123 != 5
            if tmp.124 jump or_true_44
            tmp.122 = 0
            jump or_end_45
        
          or_true_44:
            tmp.122 = 1
        
          or_end_45:
            if !tmp.122 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        constant string.0: Array(6,Char) = "Hello\\0"
        constant string.1: Array(2,Char) = "!\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_initializers_static_struct_initializers_client() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct s {
            double one_d;
            char *two_msg;
            unsigned char three_arr[3];
            int four_i;
        };
        extern struct s uninitialized;
        extern struct s partial;
        extern struct s partial_with_array;
        extern struct s converted;
        int test_uninitialized(void);
        int test_partially_initialized(void);
        int test_partial_inner_init(void);
        int test_implicit_conversion(void);
        struct s uninitialized;
        struct s partial = {1.0, "Hello"};
        struct s partial_with_array = {3.0, "!", {1}, 2};
        struct s converted = {
            1152921504606846977l,
            0l,
            "abc",
            17179869189l
        };
        int main(void) {
            if (!test_uninitialized()) {
                return 1;
            }
            if (!test_partially_initialized()) {
                return 2;
            }
            if (!test_partial_inner_init()) {
                return 3;
            }
            if (!test_implicit_conversion()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = test_uninitialized()
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = test_partially_initialized()
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = test_partial_inner_init()
            tmp.5 = ! tmp.4
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.6 = test_implicit_conversion()
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static global converted: Struct(s.2) = [ 1152921504606847000D, 0UL, "abc", zero[1], 5]
        static global partial: Struct(s.2) = [ 1D, &Hello, zero[8]]
        static global partial_with_array: Struct(s.2) = [ 3D, &!, 1UC, zero[2], zero[1], 2]
        static global uninitialized: Struct(s.2) = zero[24]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_opaque_struct() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int puts(char *s);
        void *malloc(unsigned long size);
        struct s {
            int member1;
            double member2;
            char *member3;
        };
        struct s *create_struct(int i, double d, char *s) {
            struct s *ptr = malloc(sizeof(struct s));
            ptr->member1 = i;
            ptr->member2 = d;
            ptr->member3 = s;
            return ptr;
        }
        void increment_struct(struct s *ptr) {
            ptr->member1 = ptr->member1 + 1;
            ptr->member2 = ptr->member2 + 1;
            ptr->member3 = ptr->member3;
        }
        int check_struct(struct s *ptr, int expected_i, double expected_d,
                         char *expected_s) {
            if (ptr->member1 != expected_i) {
                return 0;
            }
            if (ptr->member2 != expected_d) {
                return 0;
            }
            if (strcmp(ptr->member3, expected_s)) {
                return 0;
            }
            return 1;
        }
        void print_struct_msg(struct s *ptr) {
            puts(ptr->member3);
        }
        static struct s internal = {1, 2.0, "static struct"};
        struct s *get_internal_struct(void) {
            return &internal;
        }
        struct s incomplete_var = {3, 4.0, "global struct"};
    "#;
    let expected = r#"
        global function create_struct(i.5, d.6, s.7) { 
            tmp.0 = malloc(24UL)
            tmp.1 = tmp.0
            ptr.8 = tmp.1
            ptr.8 = i.5
            tmp.2 = add_ptr(ptr.8, index=1L, scale=1)
            tmp.2 = d.6
            tmp.3 = add_ptr(ptr.8, index=2L, scale=1)
            tmp.3 = s.7
            return ptr.8
            return 0
        }
        global function increment_struct(ptr.9) { 
            tmp.4 = ptr.9 + 1
            ptr.9 = tmp.4
            tmp.5 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.6 = add_ptr(ptr.9, index=1L, scale=1)
            tmp.8 = int_to_double 1
            tmp.7 = tmp.6 + tmp.8
            tmp.5 = tmp.7
            tmp.9 = add_ptr(ptr.9, index=2L, scale=1)
            tmp.10 = add_ptr(ptr.9, index=2L, scale=1)
            tmp.9 = tmp.10
            return 0
        }
        global function check_struct(ptr.10, expected_i.11, expected_d.12, expected_s.13) { 
            tmp.11 = ptr.10 != expected_i.11
            if !tmp.11 jump end_if_0
            return 0
        
          end_if_0:
            tmp.12 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.13 = tmp.12 != expected_d.12
            if !tmp.13 jump end_if_2
            return 0
        
          end_if_2:
            tmp.14 = add_ptr(ptr.10, index=2L, scale=1)
            tmp.15 = strcmp(tmp.14, expected_s.13)
            if !tmp.15 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function print_struct_msg(ptr.14) { 
            tmp.16 = add_ptr(ptr.14, index=2L, scale=1)
            tmp.17 = puts(tmp.16)
            return 0
        }
        global function get_internal_struct() { 
            tmp.18 = &internal
            return tmp.18
            return 0
        }
        static global incomplete_var: Struct(s.4) = [ 3, zero[4], 4D, &global struct]
        static internal: Struct(s.4) = [ 1, zero[4], 2D, &static struct]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_opaque_struct_client() {
    let src = r#"
        struct s;
        struct s *create_struct(int i, double d, char *s);
        void increment_struct(struct s *src_ptr);
        int check_struct(struct s *ptr, int expected_i, double expected_d,
                         char *expected_s);
        void print_struct_msg(struct s *ptr);
        struct s *get_internal_struct(void);
        extern struct s incomplete_var;
        int main(void) {
            struct s *new_struct = create_struct(101, 102.0, "new struct");
            struct s *internal_struct = get_internal_struct();
            print_struct_msg(new_struct);
            print_struct_msg(internal_struct);
            print_struct_msg(&incomplete_var);
            increment_struct(new_struct);
            increment_struct(&incomplete_var);
            if (!check_struct(new_struct, 102, 103.0, "new struct")) {
                return 1;
            }
            if (!check_struct(&incomplete_var, 4, 5.0, "global struct")) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &string.0
            tmp.1 = create_struct(101, 102D, tmp.0)
            new_struct.10 = tmp.1
            tmp.2 = get_internal_struct()
            internal_struct.11 = tmp.2
            print_struct_msg(new_struct.10)
            print_struct_msg(internal_struct.11)
            tmp.3 = &incomplete_var
            print_struct_msg(tmp.3)
            increment_struct(new_struct.10)
            tmp.4 = &incomplete_var
            increment_struct(tmp.4)
            tmp.5 = &string.0
            tmp.6 = check_struct(new_struct.10, 102, 103D, tmp.5)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = &incomplete_var
            tmp.9 = &string.1
            tmp.10 = check_struct(tmp.8, 4, 5D, tmp.9)
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        constant string.0: Array(11,Char) = "new struct\\0"
        constant string.1: Array(14,Char) = "global struct\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_param_struct_pointer() {
    let src = r#"
        void *malloc(unsigned long size);
        struct inner {
            double d;
            int i;
        };
        struct outer {
            char a;
            char b;
            struct inner substruct;
        };
        int access_members_through_pointer(struct outer *ptr, int expected_a,
                                           int expected_b, double expected_d,
                                           int expected_i);
        void update_members_through_pointer(struct outer *ptr, int a, int b,
                                            struct inner *inner_ptr);
        
        int access_members_through_pointer(struct outer *ptr, int expected_a,
                                           int expected_b, double expected_d,
                                           int expected_i) {
            if (ptr->a != expected_a) {
                return 0;
            }
            if (ptr->b != expected_b) {
                return 0;
            }
            if (ptr->substruct.d != expected_d) {
                return 0;
            }
            if (ptr->substruct.i != expected_i) {
                return 0;
            }
            return 1;
        }
        void update_members_through_pointer(struct outer *ptr, int a, int b,
                                            struct inner *inner_ptr) {
            ptr->a = a;
            ptr->b = b;
            ptr->substruct = *inner_ptr;
            return;
        }
    "#;
    let expected = r#"
        global function access_members_through_pointer(ptr.12, expected_a.13, expected_b.14, expected_d.15, expected_i.16) { 
            tmp.0 = sign_extend ptr.12
            tmp.1 = tmp.0 != expected_a.13
            if !tmp.1 jump end_if_0
            return 0
        
          end_if_0:
            tmp.2 = add_ptr(ptr.12, index=1L, scale=1)
            tmp.3 = sign_extend tmp.2
            tmp.4 = tmp.3 != expected_b.14
            if !tmp.4 jump end_if_2
            return 0
        
          end_if_2:
            tmp.5 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.6 = tmp.5[0]
            tmp.7 = tmp.6 != expected_d.15
            if !tmp.7 jump end_if_4
            return 0
        
          end_if_4:
            tmp.8 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.9 = tmp.8[1]
            tmp.10 = tmp.9 != expected_i.16
            if !tmp.10 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function update_members_through_pointer(ptr.17, a.18, b.19, inner_ptr.20) { 
            tmp.11 = truncate a.18
            ptr.17 = tmp.11
            tmp.12 = add_ptr(ptr.17, index=1L, scale=1)
            tmp.13 = truncate b.19
            tmp.12 = tmp.13
            tmp.14 = add_ptr(ptr.17, index=2L, scale=1)
            tmp.15 = *inner_ptr.20
            tmp.14 = tmp.15
            return 
        
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_param_struct_pointer_client() {
    let src = r#"
        void *malloc(unsigned long size);
        struct inner {
            double d;
            int i;
        };
        struct outer {
            char a;
            char b;
            struct inner substruct;
        };
        int access_members_through_pointer(struct outer *ptr, int expected_a,
                                           int expected_b, double expected_d,
                                           int expected_i);
        void update_members_through_pointer(struct outer *ptr, int a, int b,
                                            struct inner *inner_ptr);
        
        int main(void) {
            struct outer s = {1, 2, {3.0, 4}};
            if (!access_members_through_pointer(&s, 1, 2, 3.0, 4)) {
                return 1;
            }
            struct inner inner_struct = {7, 8};
            update_members_through_pointer(&s, 5, 6, &inner_struct);
            if (s.a != 5 || s.b != 6 || s.substruct.d != 7 || s.substruct.i != 8) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate 1
            outer.2[0] = tmp.0
            tmp.1 = truncate 2
            outer.2[1] = tmp.1
            inner.1[8] = 3D
            inner.1[16] = 4
            tmp.2 = &s.12
            tmp.3 = access_members_through_pointer(tmp.2, 1, 2, 3D, 4)
            tmp.4 = ! tmp.3
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = int_to_double 7
            inner.1[0] = tmp.5
            inner.1[8] = 8
            tmp.6 = &s.12
            tmp.7 = &inner_struct.13
            update_members_through_pointer(tmp.6, 5, 6, tmp.7)
            tmp.8 = s.12[0]
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 5
            if tmp.10 jump or_true_2
            tmp.13 = s.12[1]
            tmp.14 = sign_extend tmp.13
            tmp.15 = tmp.14 != 6
            if tmp.15 jump or_true_2
            tmp.12 = 0
            jump or_end_3
        
          or_true_2:
            tmp.12 = 1
        
          or_end_3:
            if tmp.12 jump or_true_4
            tmp.18 = s.12[2]
            tmp.20 = int_to_double 7
            tmp.19 = tmp.18 != tmp.20
            if tmp.19 jump or_true_4
            tmp.17 = 0
            jump or_end_5
        
          or_true_4:
            tmp.17 = 1
        
          or_end_5:
            if tmp.17 jump or_true_6
            tmp.23 = s.12[3]
            tmp.24 = tmp.23 != 8
            if tmp.24 jump or_true_6
            tmp.22 = 0
            jump or_end_7
        
          or_true_6:
            tmp.22 = 1
        
          or_end_7:
            if !tmp.22 jump end_if_8
            return 2
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_return_struct_pointer() {
    let src = r#"
        void *malloc(unsigned long size);
        struct inner {
            double d;
            int i;
        };
        struct outer {
            char a;
            char b;
            struct inner substruct;
        };
        struct outermost {
            int i;
            struct outer *nested_ptr;
            struct outer nested_struct;
        };
        struct inner *make_struct_inner(int seed);
        struct outer *make_struct_outer(int seed);
        struct outermost *make_struct_outermost(int seed);
        struct inner *make_struct_inner(int seed) {
            struct inner *ptr = malloc(sizeof(struct inner));
            ptr->d = seed;
            ptr->i = seed;
            return ptr;
        }
        struct outer *make_struct_outer(int seed) {
            struct outer *ptr = malloc(sizeof(struct outer));
            ptr->a = seed;
            ptr->b = seed + 1;
            ptr->substruct.d = seed + 2;
            ptr->substruct.i = seed + 3;
            return ptr;
        }
        struct outermost *make_struct_outermost(int seed) {
            struct outermost *ptr = malloc(sizeof(struct outermost));
            ptr->i = seed;
            ptr->nested_ptr = make_struct_outer(seed + 1);
            ptr->nested_struct.a = seed + 5;
            ptr->nested_struct.b = seed + 6;
            ptr->nested_struct.substruct.d = seed + 7;
            ptr->nested_struct.substruct.i = seed + 8;
            return ptr;
        }
    "#;
    let expected = r#"
        global function make_struct_inner(seed.7) { 
            tmp.0 = malloc(16UL)
            tmp.1 = tmp.0
            ptr.8 = tmp.1
            tmp.2 = int_to_double seed.7
            ptr.8 = tmp.2
            tmp.3 = add_ptr(ptr.8, index=1L, scale=1)
            tmp.3 = seed.7
            return ptr.8
            return 0
        }
        global function make_struct_outer(seed.9) { 
            tmp.4 = malloc(24UL)
            tmp.5 = tmp.4
            ptr.10 = tmp.5
            tmp.6 = truncate seed.9
            ptr.10 = tmp.6
            tmp.7 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.8 = seed.9 + 1
            tmp.9 = truncate tmp.8
            tmp.7 = tmp.9
            tmp.10 = add_ptr(ptr.10, index=2L, scale=1)
            tmp.11 = seed.9 + 2
            tmp.12 = int_to_double tmp.11
            tmp.10[0] = tmp.12
            tmp.13 = add_ptr(ptr.10, index=2L, scale=1)
            tmp.14 = seed.9 + 3
            tmp.13[1] = tmp.14
            return ptr.10
            return 0
        }
        global function make_struct_outermost(seed.11) { 
            tmp.15 = malloc(40UL)
            tmp.16 = tmp.15
            ptr.12 = tmp.16
            ptr.12 = seed.11
            tmp.17 = add_ptr(ptr.12, index=1L, scale=1)
            tmp.18 = seed.11 + 1
            tmp.19 = make_struct_outer(tmp.18)
            tmp.17 = tmp.19
            tmp.20 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.21 = seed.11 + 5
            tmp.22 = truncate tmp.21
            tmp.20[0] = tmp.22
            tmp.23 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.24 = seed.11 + 6
            tmp.25 = truncate tmp.24
            tmp.23[1] = tmp.25
            tmp.26 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.27 = seed.11 + 7
            tmp.28 = int_to_double tmp.27
            tmp.26[2] = tmp.28
            tmp.29 = add_ptr(ptr.12, index=2L, scale=1)
            tmp.30 = seed.11 + 8
            tmp.29[3] = tmp.30
            return ptr.12
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_libraries_return_struct_pointer_client() {
    let src = r#"
        void *malloc(unsigned long size);
        struct inner {
            double d;
            int i;
        };
        struct outer {
            char a;
            char b;
            struct inner substruct;
        };
        struct outermost {
            int i;
            struct outer *nested_ptr;
            struct outer nested_struct;
        };
        struct inner *make_struct_inner(int seed);
        struct outer *make_struct_outer(int seed);
        struct outermost *make_struct_outermost(int seed);
        int test_get_struct_ptr(void) {
            struct inner *inner_ptr = make_struct_inner(11);
            if (inner_ptr->d != 11 || inner_ptr->i != 11) {
                return 0;
            }
            struct outermost o = {0, 0, {0, 0, {0, 0}}};
            o.nested_ptr = make_struct_outer(20);
            if (o.nested_ptr->a != 20 || o.nested_ptr->b != 21 ||
                o.nested_ptr->substruct.d != 22 || o.nested_ptr->substruct.i != 23) {
                return 0;
            }
            return 1;
        }
        int test_get_struct_pointer_member(void) {
            if (make_struct_inner(2)->d != 2) {
                return 0;
            }
            if (make_struct_outer(2)->substruct.d != 4) {
                return 0;
            }
            if (make_struct_outermost(0)->nested_ptr->a != 1) {
                return 0;
            }
            return 1;
        }
        struct outer *get_static_struct_ptr(void) {
            static struct outer s;
            return &s;
        }
        int test_update_member_thru_retval(void) {
            get_static_struct_ptr()->a = 10;
            get_static_struct_ptr()->substruct.d = 20.0;
            struct outer *ptr = get_static_struct_ptr();
            if (ptr->a != 10 || ptr->substruct.d != 20.0) {
                return 0;
            }
            return 1;
        }
        int test_update_nested_struct_thru_retval(void) {
            struct inner small = {12.0, 13};
            get_static_struct_ptr()->substruct = small;
            if (get_static_struct_ptr()->substruct.d != 12.0) {
                return 0;
            }
            if (get_static_struct_ptr()->substruct.i != 13) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_get_struct_ptr()) {
                return 1;
            }
            if (!test_get_struct_pointer_member()) {
                return 2;
            }
            if (!test_update_member_thru_retval()) {
                return 3;
            }
            if (!test_update_nested_struct_thru_retval()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_get_struct_ptr() { 
            tmp.0 = make_struct_inner(11)
            inner_ptr.7 = tmp.0
            tmp.2 = int_to_double 11
            tmp.1 = inner_ptr.7 != tmp.2
            if tmp.1 jump or_true_0
            tmp.5 = add_ptr(inner_ptr.7, index=1L, scale=1)
            tmp.6 = tmp.5 != 11
            if tmp.6 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if !tmp.4 jump end_if_2
            return 0
        
          end_if_2:
            outermost.3[0] = 0
            tmp.7 = sign_extend 0
            outermost.3[8] = tmp.7
            tmp.8 = truncate 0
            outer.2[16] = tmp.8
            tmp.9 = truncate 0
            outer.2[17] = tmp.9
            tmp.10 = int_to_double 0
            inner.1[24] = tmp.10
            inner.1[32] = 0
            tmp.11 = make_struct_outer(20)
            o.8[1] = tmp.11
            tmp.12 = o.8[1]
            tmp.13 = sign_extend tmp.12
            tmp.14 = tmp.13 != 20
            if tmp.14 jump or_true_4
            tmp.17 = o.8[1]
            tmp.18 = add_ptr(tmp.17, index=1L, scale=1)
            tmp.19 = sign_extend tmp.18
            tmp.20 = tmp.19 != 21
            if tmp.20 jump or_true_4
            tmp.16 = 0
            jump or_end_5
        
          or_true_4:
            tmp.16 = 1
        
          or_end_5:
            if tmp.16 jump or_true_6
            tmp.23 = o.8[1]
            tmp.24 = add_ptr(tmp.23, index=2L, scale=1)
            tmp.25 = tmp.24[0]
            tmp.27 = int_to_double 22
            tmp.26 = tmp.25 != tmp.27
            if tmp.26 jump or_true_6
            tmp.22 = 0
            jump or_end_7
        
          or_true_6:
            tmp.22 = 1
        
          or_end_7:
            if tmp.22 jump or_true_8
            tmp.30 = o.8[1]
            tmp.31 = add_ptr(tmp.30, index=2L, scale=1)
            tmp.32 = tmp.31[1]
            tmp.33 = tmp.32 != 23
            if tmp.33 jump or_true_8
            tmp.29 = 0
            jump or_end_9
        
          or_true_8:
            tmp.29 = 1
        
          or_end_9:
            if !tmp.29 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_get_struct_pointer_member() { 
            tmp.34 = make_struct_inner(2)
            tmp.36 = int_to_double 2
            tmp.35 = tmp.34 != tmp.36
            if !tmp.35 jump end_if_12
            return 0
        
          end_if_12:
            tmp.37 = make_struct_outer(2)
            tmp.38 = add_ptr(tmp.37, index=2L, scale=1)
            tmp.39 = tmp.38[0]
            tmp.41 = int_to_double 4
            tmp.40 = tmp.39 != tmp.41
            if !tmp.40 jump end_if_14
            return 0
        
          end_if_14:
            tmp.42 = make_struct_outermost(0)
            tmp.43 = add_ptr(tmp.42, index=1L, scale=1)
            tmp.44 = sign_extend tmp.43
            tmp.45 = tmp.44 != 1
            if !tmp.45 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function get_static_struct_ptr() { 
            tmp.46 = &s.9
            return tmp.46
            return 0
        }
        global function test_update_member_thru_retval() { 
            tmp.47 = get_static_struct_ptr()
            tmp.48 = truncate 10
            tmp.47 = tmp.48
            tmp.49 = get_static_struct_ptr()
            tmp.50 = add_ptr(tmp.49, index=2L, scale=1)
            tmp.50[0] = 20D
            tmp.51 = get_static_struct_ptr()
            ptr.10 = tmp.51
            tmp.52 = sign_extend ptr.10
            tmp.53 = tmp.52 != 10
            if tmp.53 jump or_true_18
            tmp.56 = add_ptr(ptr.10, index=2L, scale=1)
            tmp.57 = tmp.56[0]
            tmp.58 = tmp.57 != 20D
            if tmp.58 jump or_true_18
            tmp.55 = 0
            jump or_end_19
        
          or_true_18:
            tmp.55 = 1
        
          or_end_19:
            if !tmp.55 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function test_update_nested_struct_thru_retval() { 
            inner.1[0] = 12D
            inner.1[8] = 13
            tmp.59 = get_static_struct_ptr()
            tmp.60 = add_ptr(tmp.59, index=2L, scale=1)
            tmp.60 = small.11
            tmp.61 = get_static_struct_ptr()
            tmp.62 = add_ptr(tmp.61, index=2L, scale=1)
            tmp.63 = tmp.62[0]
            tmp.64 = tmp.63 != 12D
            if !tmp.64 jump end_if_22
            return 0
        
          end_if_22:
            tmp.65 = get_static_struct_ptr()
            tmp.66 = add_ptr(tmp.65, index=2L, scale=1)
            tmp.67 = tmp.66[1]
            tmp.68 = tmp.67 != 13
            if !tmp.68 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function main() { 
            tmp.69 = test_get_struct_ptr()
            tmp.70 = ! tmp.69
            if !tmp.70 jump end_if_26
            return 1
        
          end_if_26:
            tmp.71 = test_get_struct_pointer_member()
            tmp.72 = ! tmp.71
            if !tmp.72 jump end_if_28
            return 2
        
          end_if_28:
            tmp.73 = test_update_member_thru_retval()
            tmp.74 = ! tmp.73
            if !tmp.74 jump end_if_30
            return 3
        
          end_if_30:
            tmp.75 = test_update_nested_struct_thru_retval()
            tmp.76 = ! tmp.75
            if !tmp.76 jump end_if_32
            return 4
        
          end_if_32:
            return 0
            return 0
        }
        static s.9: Struct(outer.2) = zero[24]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_parse_and_lex_postfix_precedence() {
    let src = r#"
        
        struct inner {
            int inner_arr[3];
        };
        struct outer {
            int a;
            struct inner b;
        };
        int main(void) {
            struct outer array[4] = {{1, {{2, 3, 4}}},
                                     {5, {{6, 7, 8}}},
                                     {9, {{10, 11, 12}}},
                                     {13, {{14, 15, 16}}}};
            int i = -array[2].b.inner_arr[1];
            return i == -11;
        }
    "#;
    let expected = r#"
        global function main() { 
            outer.1[0] = 1
            inner.0[4] = 2
            inner.0[8] = 3
            inner.0[12] = 4
            outer.1[16] = 5
            inner.0[20] = 6
            inner.0[24] = 7
            inner.0[28] = 8
            outer.1[32] = 9
            inner.0[36] = 10
            inner.0[40] = 11
            inner.0[44] = 12
            outer.1[48] = 13
            inner.0[52] = 14
            inner.0[56] = 15
            inner.0[60] = 16
            tmp.0 = &array.2
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=16)
            tmp.3 = add_ptr(tmp.2, index=1L, scale=1)
            tmp.4 = sign_extend 1
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=4)
            tmp.6 = *tmp.5
            tmp.7 = - tmp.6
            i.3 = tmp.7
            tmp.9 = - 11
            tmp.8 = i.3 == tmp.9
            return tmp.8
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_parse_and_lex_space_around_struct_member() {
    let src = r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s foo;
            foo .a = 10;
            int b = foo .a;
            return foo . a == b;
        }
    "#;
    let expected = r#"
        global function main() { 
            foo.1[0] = 10
            tmp.0 = foo.1[0]
            b.2 = tmp.0
            tmp.1 = foo.1[0]
            tmp.2 = tmp.1 == b.2
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_parse_and_lex_struct_member_looks_like_const() {
    let src = r#"
        struct s {
            int E10;
        };
        int main(void) {
            struct s x1 = {3};
            return x1.E10;
        }
    "#;
    let expected = r#"
        global function main() { 
            s.0[0] = 3
            tmp.0 = x1.1[0]
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_parse_and_lex_trailing_comma() {
    let src = r#"
        
        struct s {
            int a;
            int b;
        };
        int main(void) {
            struct s x = {
                1,
                2,
            };
            if (x.a != 1 || x.b != 2) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            s.0[0] = 1
            s.0[4] = 2
            tmp.0 = x.1[0]
            tmp.1 = tmp.0 != 1
            if tmp.1 jump or_true_0
            tmp.4 = x.1[1]
            tmp.5 = tmp.4 != 2
            if tmp.5 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if !tmp.3 jump end_if_2
            return 1
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_scalar_member_access_arrow() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        struct four_members {
            double d;
            char c;
            long l;
            char *ptr;
        };
        double get_double(void) {
            return 2e12;
        }
        static long l = 34359738378l;
        int accept_params(int d_divided, int c_doubled, double l_cast,
                          int dereferenced_ptr, double d, int c, long l, char *ptr) {
            if (d != -1845381177299.0 || c != 127 || l != 58 || *ptr != 100 ||
                d_divided != -922690588 || c_doubled != 254 || l_cast != 58.0 ||
                dereferenced_ptr != 100) {
                return 0;
            }
            return 1;
        }
        int test_auto(void) {
            struct four_members autom;
            struct four_members *autom_ptr = &autom;
            autom_ptr->d = (l - get_double()) + (l * 3.5);
            autom_ptr->c = 127;
            autom_ptr->l = get_double() / l;
            char chr = 100;
            autom_ptr->ptr = &chr;
            if (autom_ptr->d != -1845381177299.0 || autom_ptr->c != 127 ||
                autom_ptr->l != 58 || autom_ptr->ptr != &chr) {
                return 0;
            }
            double *d_ptr = &autom_ptr->d;
            char *c_ptr = &autom_ptr->c;
            if (*d_ptr != -1845381177299.0 || *c_ptr != 127) {
                return 0;
            }
            if (*autom_ptr->ptr != 100) {
                return 0;
            }
            if (!accept_params(autom.d / 2000, autom.c * 2, (double)autom.l, *autom.ptr,
                               autom.d, autom.c, autom.l, autom.ptr)) {
                return 0;
            }
            return 1;
        }
        int test_static(void) {
            static struct four_members stat;
            static struct four_members *stat_ptr;
            stat_ptr = &stat;
            static char chr = 100;
            stat_ptr->d = (l - get_double()) + (l * 3.5);
            stat_ptr->c = 127;
            stat_ptr->l = get_double() / l;
            stat_ptr->ptr = &chr;
            if (stat_ptr->d != -1845381177299.0 || stat_ptr->c != 127 ||
                stat_ptr->l != 58 || stat_ptr->ptr != &chr) {
                return 0;
            }
            double *d_ptr = &stat_ptr->d;
            char *c_ptr = &stat_ptr->c;
            if (*d_ptr != -1845381177299.0 || *c_ptr != 127) {
                return 0;
            }
            if (*stat_ptr->ptr != 100) {
                return 0;
            }
            if (!accept_params(stat.d / 2000, stat.c * 2, (double)stat.l, *stat.ptr,
                               stat.d, stat.c, stat.l, stat.ptr)) {
                return 0;
            }
            return 1;
        }
        int test_exp_result_member(void) {
            static int flag = 1;
            struct four_members s1;
            s1.d = 10.0;
            s1.c = 99;
            s1.l = 9223372036854775807l;
            s1.ptr = 0;
            struct four_members s2;
            s2.d = 12.0;
            s2.c = 98;
            s2.l = -9223372036854775807l;
            s2.ptr = 0;
            struct four_members *s1_ptr = &s1;
            struct four_members *s2_ptr = &s2;
            (flag ? s1_ptr : s2_ptr)->c = 127;
            if (s1.c != 127) {
                return 0;
            }
            if (s2.c != 98) {
                return 0;
            }
            struct four_members *result_ptr = 0;
            if ((result_ptr = s2_ptr)->d != 12.0 ||
                result_ptr->l != -9223372036854775807l) {
                return 0;
            }
            void *void_ptr = calloc(1, sizeof(struct four_members));
            ((struct four_members *)void_ptr)->c = 80;
            result_ptr = void_ptr;
            if (result_ptr->c != 80) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_auto()) {
                return 1;
            }
            if (!test_static()) {
                return 2;
            }
            if (!test_exp_result_member()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_double() { 
            return 2000000000000D
            return 0
        }
        global function accept_params(d_divided.3, c_doubled.4, l_cast.5, dereferenced_ptr.6, d.7, c.8, l.9, ptr.10) { 
            tmp.1 = - 1845381177299D
            tmp.0 = d.7 != tmp.1
            if tmp.0 jump or_true_0
            tmp.4 = c.8 != 127
            if tmp.4 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if tmp.3 jump or_true_2
            tmp.8 = sign_extend 58
            tmp.7 = l.9 != tmp.8
            if tmp.7 jump or_true_2
            tmp.6 = 0
            jump or_end_3
        
          or_true_2:
            tmp.6 = 1
        
          or_end_3:
            if tmp.6 jump or_true_4
            tmp.11 = *ptr.10
            tmp.12 = sign_extend tmp.11
            tmp.13 = tmp.12 != 100
            if tmp.13 jump or_true_4
            tmp.10 = 0
            jump or_end_5
        
          or_true_4:
            tmp.10 = 1
        
          or_end_5:
            if tmp.10 jump or_true_6
            tmp.17 = - 922690588
            tmp.16 = d_divided.3 != tmp.17
            if tmp.16 jump or_true_6
            tmp.15 = 0
            jump or_end_7
        
          or_true_6:
            tmp.15 = 1
        
          or_end_7:
            if tmp.15 jump or_true_8
            tmp.20 = c_doubled.4 != 254
            if tmp.20 jump or_true_8
            tmp.19 = 0
            jump or_end_9
        
          or_true_8:
            tmp.19 = 1
        
          or_end_9:
            if tmp.19 jump or_true_10
            tmp.23 = l_cast.5 != 58D
            if tmp.23 jump or_true_10
            tmp.22 = 0
            jump or_end_11
        
          or_true_10:
            tmp.22 = 1
        
          or_end_11:
            if tmp.22 jump or_true_12
            tmp.26 = dereferenced_ptr.6 != 100
            if tmp.26 jump or_true_12
            tmp.25 = 0
            jump or_end_13
        
          or_true_12:
            tmp.25 = 1
        
          or_end_13:
            if !tmp.25 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_auto() { 
            tmp.27 = &autom.11
            autom_ptr.12 = tmp.27
            tmp.28 = int_to_double l
            tmp.30 = get_double()
            tmp.29 = tmp.28 - tmp.30
            tmp.32 = int_to_double l
            tmp.33 = tmp.32 * 3.5D
            tmp.31 = tmp.29 + tmp.33
            autom_ptr.12 = tmp.31
            tmp.34 = add_ptr(autom_ptr.12, index=1L, scale=1)
            tmp.35 = truncate 127
            tmp.34 = tmp.35
            tmp.36 = add_ptr(autom_ptr.12, index=2L, scale=1)
            tmp.37 = get_double()
            tmp.39 = int_to_double l
            tmp.38 = tmp.37 / tmp.39
            tmp.40 = double_to_int tmp.38
            tmp.36 = tmp.40
            tmp.41 = truncate 100
            chr.13 = tmp.41
            tmp.42 = add_ptr(autom_ptr.12, index=3L, scale=1)
            tmp.43 = &chr.13
            tmp.42 = tmp.43
            tmp.45 = - 1845381177299D
            tmp.44 = autom_ptr.12 != tmp.45
            if tmp.44 jump or_true_16
            tmp.48 = add_ptr(autom_ptr.12, index=1L, scale=1)
            tmp.49 = sign_extend tmp.48
            tmp.50 = tmp.49 != 127
            if tmp.50 jump or_true_16
            tmp.47 = 0
            jump or_end_17
        
          or_true_16:
            tmp.47 = 1
        
          or_end_17:
            if tmp.47 jump or_true_18
            tmp.53 = add_ptr(autom_ptr.12, index=2L, scale=1)
            tmp.55 = sign_extend 58
            tmp.54 = tmp.53 != tmp.55
            if tmp.54 jump or_true_18
            tmp.52 = 0
            jump or_end_19
        
          or_true_18:
            tmp.52 = 1
        
          or_end_19:
            if tmp.52 jump or_true_20
            tmp.58 = add_ptr(autom_ptr.12, index=3L, scale=1)
            tmp.60 = &chr.13
            tmp.59 = tmp.58 != tmp.60
            if tmp.59 jump or_true_20
            tmp.57 = 0
            jump or_end_21
        
          or_true_20:
            tmp.57 = 1
        
          or_end_21:
            if !tmp.57 jump end_if_22
            return 0
        
          end_if_22:
            tmp.61 = &autom_ptr.12
            d_ptr.14 = tmp.61
            tmp.62 = add_ptr(autom_ptr.12, index=1L, scale=1)
            tmp.63 = &tmp.62
            c_ptr.15 = tmp.63
            tmp.64 = *d_ptr.14
            tmp.66 = - 1845381177299D
            tmp.65 = tmp.64 != tmp.66
            if tmp.65 jump or_true_24
            tmp.69 = *c_ptr.15
            tmp.70 = sign_extend tmp.69
            tmp.71 = tmp.70 != 127
            if tmp.71 jump or_true_24
            tmp.68 = 0
            jump or_end_25
        
          or_true_24:
            tmp.68 = 1
        
          or_end_25:
            if !tmp.68 jump end_if_26
            return 0
        
          end_if_26:
            tmp.72 = add_ptr(autom_ptr.12, index=3L, scale=1)
            tmp.73 = *tmp.72
            tmp.74 = sign_extend tmp.73
            tmp.75 = tmp.74 != 100
            if !tmp.75 jump end_if_28
            return 0
        
          end_if_28:
            tmp.76 = autom.11[0]
            tmp.78 = int_to_double 2000
            tmp.77 = tmp.76 / tmp.78
            tmp.79 = double_to_int tmp.77
            tmp.80 = autom.11[1]
            tmp.81 = sign_extend tmp.80
            tmp.82 = tmp.81 * 2
            tmp.83 = autom.11[2]
            tmp.84 = int_to_double tmp.83
            tmp.85 = autom.11[3]
            tmp.86 = *tmp.85
            tmp.87 = sign_extend tmp.86
            tmp.88 = autom.11[0]
            tmp.89 = autom.11[1]
            tmp.90 = sign_extend tmp.89
            tmp.91 = autom.11[2]
            tmp.92 = autom.11[3]
            tmp.93 = accept_params(tmp.79, tmp.82, tmp.84, tmp.87, tmp.88, tmp.90, tmp.91, tmp.92)
            tmp.94 = ! tmp.93
            if !tmp.94 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_static() { 
            tmp.95 = &stat.16
            stat_ptr.17 = tmp.95
            tmp.96 = int_to_double l
            tmp.98 = get_double()
            tmp.97 = tmp.96 - tmp.98
            tmp.100 = int_to_double l
            tmp.101 = tmp.100 * 3.5D
            tmp.99 = tmp.97 + tmp.101
            stat_ptr.17 = tmp.99
            tmp.102 = add_ptr(stat_ptr.17, index=1L, scale=1)
            tmp.103 = truncate 127
            tmp.102 = tmp.103
            tmp.104 = add_ptr(stat_ptr.17, index=2L, scale=1)
            tmp.105 = get_double()
            tmp.107 = int_to_double l
            tmp.106 = tmp.105 / tmp.107
            tmp.108 = double_to_int tmp.106
            tmp.104 = tmp.108
            tmp.109 = add_ptr(stat_ptr.17, index=3L, scale=1)
            tmp.110 = &chr.18
            tmp.109 = tmp.110
            tmp.112 = - 1845381177299D
            tmp.111 = stat_ptr.17 != tmp.112
            if tmp.111 jump or_true_32
            tmp.115 = add_ptr(stat_ptr.17, index=1L, scale=1)
            tmp.116 = sign_extend tmp.115
            tmp.117 = tmp.116 != 127
            if tmp.117 jump or_true_32
            tmp.114 = 0
            jump or_end_33
        
          or_true_32:
            tmp.114 = 1
        
          or_end_33:
            if tmp.114 jump or_true_34
            tmp.120 = add_ptr(stat_ptr.17, index=2L, scale=1)
            tmp.122 = sign_extend 58
            tmp.121 = tmp.120 != tmp.122
            if tmp.121 jump or_true_34
            tmp.119 = 0
            jump or_end_35
        
          or_true_34:
            tmp.119 = 1
        
          or_end_35:
            if tmp.119 jump or_true_36
            tmp.125 = add_ptr(stat_ptr.17, index=3L, scale=1)
            tmp.127 = &chr.18
            tmp.126 = tmp.125 != tmp.127
            if tmp.126 jump or_true_36
            tmp.124 = 0
            jump or_end_37
        
          or_true_36:
            tmp.124 = 1
        
          or_end_37:
            if !tmp.124 jump end_if_38
            return 0
        
          end_if_38:
            tmp.128 = &stat_ptr.17
            d_ptr.19 = tmp.128
            tmp.129 = add_ptr(stat_ptr.17, index=1L, scale=1)
            tmp.130 = &tmp.129
            c_ptr.20 = tmp.130
            tmp.131 = *d_ptr.19
            tmp.133 = - 1845381177299D
            tmp.132 = tmp.131 != tmp.133
            if tmp.132 jump or_true_40
            tmp.136 = *c_ptr.20
            tmp.137 = sign_extend tmp.136
            tmp.138 = tmp.137 != 127
            if tmp.138 jump or_true_40
            tmp.135 = 0
            jump or_end_41
        
          or_true_40:
            tmp.135 = 1
        
          or_end_41:
            if !tmp.135 jump end_if_42
            return 0
        
          end_if_42:
            tmp.139 = add_ptr(stat_ptr.17, index=3L, scale=1)
            tmp.140 = *tmp.139
            tmp.141 = sign_extend tmp.140
            tmp.142 = tmp.141 != 100
            if !tmp.142 jump end_if_44
            return 0
        
          end_if_44:
            tmp.143 = stat.16[0]
            tmp.145 = int_to_double 2000
            tmp.144 = tmp.143 / tmp.145
            tmp.146 = double_to_int tmp.144
            tmp.147 = stat.16[1]
            tmp.148 = sign_extend tmp.147
            tmp.149 = tmp.148 * 2
            tmp.150 = stat.16[2]
            tmp.151 = int_to_double tmp.150
            tmp.152 = stat.16[3]
            tmp.153 = *tmp.152
            tmp.154 = sign_extend tmp.153
            tmp.155 = stat.16[0]
            tmp.156 = stat.16[1]
            tmp.157 = sign_extend tmp.156
            tmp.158 = stat.16[2]
            tmp.159 = stat.16[3]
            tmp.160 = accept_params(tmp.146, tmp.149, tmp.151, tmp.154, tmp.155, tmp.157, tmp.158, tmp.159)
            tmp.161 = ! tmp.160
            if !tmp.161 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function test_exp_result_member() { 
            s1.22[0] = 10D
            tmp.162 = truncate 99
            s1.22[1] = tmp.162
            s1.22[2] = 9223372036854775807L
            tmp.163 = sign_extend 0
            s1.22[3] = tmp.163
            s2.23[0] = 12D
            tmp.164 = truncate 98
            s2.23[1] = tmp.164
            tmp.165 = - 9223372036854775807L
            s2.23[2] = tmp.165
            tmp.166 = sign_extend 0
            s2.23[3] = tmp.166
            tmp.167 = &s1.22
            s1_ptr.24 = tmp.167
            tmp.168 = &s2.23
            s2_ptr.25 = tmp.168
            if !flag.21 jump else_49
            tmp.169 = s1_ptr.24
            jump end_if_48
        
          else_49:
            tmp.169 = s2_ptr.25
        
          end_if_48:
            tmp.170 = add_ptr(tmp.169, index=1L, scale=1)
            tmp.171 = truncate 127
            tmp.170 = tmp.171
            tmp.172 = s1.22[1]
            tmp.173 = sign_extend tmp.172
            tmp.174 = tmp.173 != 127
            if !tmp.174 jump end_if_50
            return 0
        
          end_if_50:
            tmp.175 = s2.23[1]
            tmp.176 = sign_extend tmp.175
            tmp.177 = tmp.176 != 98
            if !tmp.177 jump end_if_52
            return 0
        
          end_if_52:
            tmp.178 = sign_extend 0
            result_ptr.26 = tmp.178
            result_ptr.26 = s2_ptr.25
            tmp.179 = s2_ptr.25 != 12D
            if tmp.179 jump or_true_54
            tmp.182 = add_ptr(result_ptr.26, index=2L, scale=1)
            tmp.184 = - 9223372036854775807L
            tmp.183 = tmp.182 != tmp.184
            if tmp.183 jump or_true_54
            tmp.181 = 0
            jump or_end_55
        
          or_true_54:
            tmp.181 = 1
        
          or_end_55:
            if !tmp.181 jump end_if_56
            return 0
        
          end_if_56:
            tmp.185 = sign_extend 1
            tmp.186 = calloc(tmp.185, 32UL)
            void_ptr.27 = tmp.186
            tmp.187 = void_ptr.27
            tmp.188 = add_ptr(tmp.187, index=1L, scale=1)
            tmp.189 = truncate 80
            tmp.188 = tmp.189
            tmp.190 = void_ptr.27
            result_ptr.26 = tmp.190
            tmp.191 = add_ptr(result_ptr.26, index=1L, scale=1)
            tmp.192 = sign_extend tmp.191
            tmp.193 = tmp.192 != 80
            if !tmp.193 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function main() { 
            tmp.194 = test_auto()
            tmp.195 = ! tmp.194
            if !tmp.195 jump end_if_60
            return 1
        
          end_if_60:
            tmp.196 = test_static()
            tmp.197 = ! tmp.196
            if !tmp.197 jump end_if_62
            return 2
        
          end_if_62:
            tmp.198 = test_exp_result_member()
            tmp.199 = ! tmp.198
            if !tmp.199 jump end_if_64
            return 3
        
          end_if_64:
            return 0
            return 0
        }
        static chr.18: Char = 'd'
        static flag.21: Int = 1
        static l: Long = 34359738378L
        static stat.16: Struct(four_members.2) = zero[32]
        static stat_ptr.17: Pointer(Struct(four_members.2)) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_scalar_member_access_dot() {
    let src = r#"
        struct four_members {
            double d;
            char c;
            long l;
            char *ptr;
        };
        double get_double(void) {
            return 2e12;
        }
        static long l = 34359738378l;
        int accept_params(int d_divided, int c_doubled, double l_cast,
                          int dereferenced_ptr, double d, int c, long l, char *ptr) {
            if (d != 4e12 || c != 127 || l != 8589934594l || *ptr != 100 ||
                d_divided != 100.0 || c_doubled != 254 || l_cast != 8589934594.0 ||
                dereferenced_ptr != 100) {
                return 0;
            }
            return 1;
        }
        int test_auto(void) {
            struct four_members autom;
            autom.d = get_double() * 2.0;
            autom.c = 127;
            autom.l = l / 4;
            char chr = 100;
            autom.ptr = &chr;
            if (autom.d != 4e12 || autom.c != 127 || autom.l != 8589934594l ||
                autom.ptr != &chr) {
                return 0;
            }
            double *d_ptr = &autom.d;
            char *c_ptr = &autom.c;
            if (*d_ptr != 4e12 || *c_ptr != 127) {
                return 0;
            }
            if (*autom.ptr != 100) {
                return 0;
            }
            if (!accept_params(autom.d / 4e10, autom.c * 2, (double)autom.l, *autom.ptr,
                               autom.d, autom.c, autom.l, autom.ptr)) {
                return 0;
            }
            return 1;
        }
        int test_static(void) {
            static struct four_members stat;
            static char chr = 100;
            stat.d = get_double() * 2.0;
            stat.c = 127;
            stat.l = l / 4;
            stat.ptr = &chr;
            if (stat.d != 4e12 || stat.c != 127 || stat.l != 8589934594l ||
                stat.ptr != &chr) {
                return 0;
            }
            double *d_ptr = &stat.d;
            char *c_ptr = &stat.c;
            if (*d_ptr != 4e12 || *c_ptr != 127) {
                return 0;
            }
            if (*stat.ptr != 100) {
                return 0;
            }
            if (!accept_params(stat.d / 4e10, stat.c * 2, (double)stat.l, *stat.ptr,
                               stat.d, stat.c, stat.l, stat.ptr)) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_auto()) {
                return 1;
            }
            if (!test_static()) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_double() { 
            return 2000000000000D
            return 0
        }
        global function accept_params(d_divided.1, c_doubled.2, l_cast.3, dereferenced_ptr.4, d.5, c.6, l.7, ptr.8) { 
            tmp.0 = d.5 != 4000000000000D
            if tmp.0 jump or_true_0
            tmp.3 = c.6 != 127
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if tmp.2 jump or_true_2
            tmp.6 = l.7 != 8589934594L
            if tmp.6 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            if tmp.5 jump or_true_4
            tmp.9 = *ptr.8
            tmp.10 = sign_extend tmp.9
            tmp.11 = tmp.10 != 100
            if tmp.11 jump or_true_4
            tmp.8 = 0
            jump or_end_5
        
          or_true_4:
            tmp.8 = 1
        
          or_end_5:
            if tmp.8 jump or_true_6
            tmp.14 = int_to_double d_divided.1
            tmp.15 = tmp.14 != 100D
            if tmp.15 jump or_true_6
            tmp.13 = 0
            jump or_end_7
        
          or_true_6:
            tmp.13 = 1
        
          or_end_7:
            if tmp.13 jump or_true_8
            tmp.18 = c_doubled.2 != 254
            if tmp.18 jump or_true_8
            tmp.17 = 0
            jump or_end_9
        
          or_true_8:
            tmp.17 = 1
        
          or_end_9:
            if tmp.17 jump or_true_10
            tmp.21 = l_cast.3 != 8589934594D
            if tmp.21 jump or_true_10
            tmp.20 = 0
            jump or_end_11
        
          or_true_10:
            tmp.20 = 1
        
          or_end_11:
            if tmp.20 jump or_true_12
            tmp.24 = dereferenced_ptr.4 != 100
            if tmp.24 jump or_true_12
            tmp.23 = 0
            jump or_end_13
        
          or_true_12:
            tmp.23 = 1
        
          or_end_13:
            if !tmp.23 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_auto() { 
            tmp.25 = get_double()
            tmp.26 = tmp.25 * 2D
            autom.9[0] = tmp.26
            tmp.27 = truncate 127
            autom.9[1] = tmp.27
            tmp.29 = sign_extend 4
            tmp.28 = l / tmp.29
            autom.9[2] = tmp.28
            tmp.30 = truncate 100
            chr.10 = tmp.30
            tmp.31 = &chr.10
            autom.9[3] = tmp.31
            tmp.32 = autom.9[0]
            tmp.33 = tmp.32 != 4000000000000D
            if tmp.33 jump or_true_16
            tmp.36 = autom.9[1]
            tmp.37 = sign_extend tmp.36
            tmp.38 = tmp.37 != 127
            if tmp.38 jump or_true_16
            tmp.35 = 0
            jump or_end_17
        
          or_true_16:
            tmp.35 = 1
        
          or_end_17:
            if tmp.35 jump or_true_18
            tmp.41 = autom.9[2]
            tmp.42 = tmp.41 != 8589934594L
            if tmp.42 jump or_true_18
            tmp.40 = 0
            jump or_end_19
        
          or_true_18:
            tmp.40 = 1
        
          or_end_19:
            if tmp.40 jump or_true_20
            tmp.45 = autom.9[3]
            tmp.47 = &chr.10
            tmp.46 = tmp.45 != tmp.47
            if tmp.46 jump or_true_20
            tmp.44 = 0
            jump or_end_21
        
          or_true_20:
            tmp.44 = 1
        
          or_end_21:
            if !tmp.44 jump end_if_22
            return 0
        
          end_if_22:
            tmp.48 = &autom.9
            d_ptr.11 = tmp.48
            tmp.49 = &autom.9
            tmp.49 = add_ptr(tmp.49, index=1L, scale=1)
            c_ptr.12 = tmp.49
            tmp.50 = *d_ptr.11
            tmp.51 = tmp.50 != 4000000000000D
            if tmp.51 jump or_true_24
            tmp.54 = *c_ptr.12
            tmp.55 = sign_extend tmp.54
            tmp.56 = tmp.55 != 127
            if tmp.56 jump or_true_24
            tmp.53 = 0
            jump or_end_25
        
          or_true_24:
            tmp.53 = 1
        
          or_end_25:
            if !tmp.53 jump end_if_26
            return 0
        
          end_if_26:
            tmp.57 = autom.9[3]
            tmp.58 = *tmp.57
            tmp.59 = sign_extend tmp.58
            tmp.60 = tmp.59 != 100
            if !tmp.60 jump end_if_28
            return 0
        
          end_if_28:
            tmp.61 = autom.9[0]
            tmp.62 = tmp.61 / 40000000000D
            tmp.63 = double_to_int tmp.62
            tmp.64 = autom.9[1]
            tmp.65 = sign_extend tmp.64
            tmp.66 = tmp.65 * 2
            tmp.67 = autom.9[2]
            tmp.68 = int_to_double tmp.67
            tmp.69 = autom.9[3]
            tmp.70 = *tmp.69
            tmp.71 = sign_extend tmp.70
            tmp.72 = autom.9[0]
            tmp.73 = autom.9[1]
            tmp.74 = sign_extend tmp.73
            tmp.75 = autom.9[2]
            tmp.76 = autom.9[3]
            tmp.77 = accept_params(tmp.63, tmp.66, tmp.68, tmp.71, tmp.72, tmp.74, tmp.75, tmp.76)
            tmp.78 = ! tmp.77
            if !tmp.78 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_static() { 
            tmp.79 = get_double()
            tmp.80 = tmp.79 * 2D
            stat.13[0] = tmp.80
            tmp.81 = truncate 127
            stat.13[1] = tmp.81
            tmp.83 = sign_extend 4
            tmp.82 = l / tmp.83
            stat.13[2] = tmp.82
            tmp.84 = &chr.14
            stat.13[3] = tmp.84
            tmp.85 = stat.13[0]
            tmp.86 = tmp.85 != 4000000000000D
            if tmp.86 jump or_true_32
            tmp.89 = stat.13[1]
            tmp.90 = sign_extend tmp.89
            tmp.91 = tmp.90 != 127
            if tmp.91 jump or_true_32
            tmp.88 = 0
            jump or_end_33
        
          or_true_32:
            tmp.88 = 1
        
          or_end_33:
            if tmp.88 jump or_true_34
            tmp.94 = stat.13[2]
            tmp.95 = tmp.94 != 8589934594L
            if tmp.95 jump or_true_34
            tmp.93 = 0
            jump or_end_35
        
          or_true_34:
            tmp.93 = 1
        
          or_end_35:
            if tmp.93 jump or_true_36
            tmp.98 = stat.13[3]
            tmp.100 = &chr.14
            tmp.99 = tmp.98 != tmp.100
            if tmp.99 jump or_true_36
            tmp.97 = 0
            jump or_end_37
        
          or_true_36:
            tmp.97 = 1
        
          or_end_37:
            if !tmp.97 jump end_if_38
            return 0
        
          end_if_38:
            tmp.101 = &stat.13
            d_ptr.15 = tmp.101
            tmp.102 = &stat.13
            tmp.102 = add_ptr(tmp.102, index=1L, scale=1)
            c_ptr.16 = tmp.102
            tmp.103 = *d_ptr.15
            tmp.104 = tmp.103 != 4000000000000D
            if tmp.104 jump or_true_40
            tmp.107 = *c_ptr.16
            tmp.108 = sign_extend tmp.107
            tmp.109 = tmp.108 != 127
            if tmp.109 jump or_true_40
            tmp.106 = 0
            jump or_end_41
        
          or_true_40:
            tmp.106 = 1
        
          or_end_41:
            if !tmp.106 jump end_if_42
            return 0
        
          end_if_42:
            tmp.110 = stat.13[3]
            tmp.111 = *tmp.110
            tmp.112 = sign_extend tmp.111
            tmp.113 = tmp.112 != 100
            if !tmp.113 jump end_if_44
            return 0
        
          end_if_44:
            tmp.114 = stat.13[0]
            tmp.115 = tmp.114 / 40000000000D
            tmp.116 = double_to_int tmp.115
            tmp.117 = stat.13[1]
            tmp.118 = sign_extend tmp.117
            tmp.119 = tmp.118 * 2
            tmp.120 = stat.13[2]
            tmp.121 = int_to_double tmp.120
            tmp.122 = stat.13[3]
            tmp.123 = *tmp.122
            tmp.124 = sign_extend tmp.123
            tmp.125 = stat.13[0]
            tmp.126 = stat.13[1]
            tmp.127 = sign_extend tmp.126
            tmp.128 = stat.13[2]
            tmp.129 = stat.13[3]
            tmp.130 = accept_params(tmp.116, tmp.119, tmp.121, tmp.124, tmp.125, tmp.127, tmp.128, tmp.129)
            tmp.131 = ! tmp.130
            if !tmp.131 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function main() { 
            tmp.132 = test_auto()
            tmp.133 = ! tmp.132
            if !tmp.133 jump end_if_48
            return 1
        
          end_if_48:
            tmp.134 = test_static()
            tmp.135 = ! tmp.134
            if !tmp.135 jump end_if_50
            return 2
        
          end_if_50:
            return 0
            return 0
        }
        static chr.14: Char = 'd'
        static l: Long = 34359738378L
        static stat.13: Struct(four_members.0) = zero[32]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_scalar_member_access_linked_list() {
    let src = r#"
        void *malloc(unsigned long size);
        struct linked_list_node {
            int val;
            struct linked_list_node *next;
        };
        struct linked_list_node *array_to_list(int *array, int count) {
            struct linked_list_node *head =
                (struct linked_list_node *)malloc(sizeof(struct linked_list_node));
            head->val = array[0];
            head->next = 0;
            struct linked_list_node *current = head;
            for (int i = 1; i < count; i = i + 1) {
                current->next =
                    (struct linked_list_node *)malloc(sizeof(struct linked_list_node));
                current->next->next = 0;
                current->next->val = array[i];
                current = current->next;
            }
            return head;
        }
        int main(void) {
            int arr[4] = {9, 8, 7, 6};
            struct linked_list_node *elem = array_to_list(arr, 4);
            for (int i = 0; i < 4; i = i + 1) {
                int expected = arr[i];
                if (elem->val != expected) {
                    return i + 1;
                }
                elem = elem->next;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function array_to_list(array.2, count.3) { 
            tmp.0 = malloc(16UL)
            tmp.1 = tmp.0
            head.4 = tmp.1
            tmp.2 = sign_extend 0
            tmp.3 = add_ptr(array.2, index=tmp.2, scale=4)
            tmp.4 = *tmp.3
            head.4 = tmp.4
            tmp.5 = add_ptr(head.4, index=1L, scale=1)
            tmp.6 = sign_extend 0
            tmp.5 = tmp.6
            current.5 = head.4
            i.6 = 1
        
          start_loop_0:
            tmp.7 = i.6 < count.3
            if !tmp.7 jump break_loop_0
            tmp.8 = add_ptr(current.5, index=1L, scale=1)
            tmp.9 = malloc(16UL)
            tmp.10 = tmp.9
            tmp.8 = tmp.10
            tmp.11 = add_ptr(current.5, index=1L, scale=1)
            tmp.12 = add_ptr(tmp.11, index=1L, scale=1)
            tmp.13 = sign_extend 0
            tmp.12 = tmp.13
            tmp.14 = add_ptr(current.5, index=1L, scale=1)
            tmp.15 = sign_extend i.6
            tmp.16 = add_ptr(array.2, index=tmp.15, scale=4)
            tmp.17 = *tmp.16
            tmp.14 = tmp.17
            tmp.18 = add_ptr(current.5, index=1L, scale=1)
            current.5 = tmp.18
        
          continue_loop_0:
            tmp.19 = i.6 + 1
            i.6 = tmp.19
            jump start_loop_0
        
          break_loop_0:
            return head.4
            return 0
        }
        global function main() { 
            arr.7[0] = 9
            arr.7[4] = 8
            arr.7[8] = 7
            arr.7[12] = 6
            tmp.20 = &arr.7
            tmp.21 = array_to_list(tmp.20, 4)
            elem.8 = tmp.21
            i.9 = 0
        
          start_loop_1:
            tmp.22 = i.9 < 4
            if !tmp.22 jump break_loop_1
            tmp.23 = &arr.7
            tmp.24 = sign_extend i.9
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=4)
            tmp.26 = *tmp.25
            expected.10 = tmp.26
            tmp.27 = elem.8 != expected.10
            if !tmp.27 jump end_if_0
            tmp.28 = i.9 + 1
            return tmp.28
        
          end_if_0:
            tmp.29 = add_ptr(elem.8, index=1L, scale=1)
            elem.8 = tmp.29
        
          continue_loop_1:
            tmp.30 = i.9 + 1
            i.9 = tmp.30
            jump start_loop_1
        
          break_loop_1:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_scalar_member_access_nested_struct() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *malloc(unsigned long size);
        struct inner {
            double a;
            char b;
            int *ptr;
        };
        struct outer {
            unsigned long l;
            struct inner *in_ptr;
            struct inner in_array[4];
            int bar;
            struct inner in;
        };
        int ptr_target;
        int test_auto_dot(void) {
            struct outer s;
            s.in.a = 1.0;
            s.in.b = 2;
            s.in.ptr = &ptr_target;
            if (s.in.a != 1.0 || s.in.b != 2 || s.in.ptr != &ptr_target) {
                return 0;
            }
            char *char_ptr = &s.in.b;
            if (*char_ptr != 2) {
                return 0;
            }
            *s.in.ptr = 5;
            if (ptr_target != 5) {
                return 0;
            }
            s.in.a = s.in.b;
            if (s.in.a != 2.0) {
                return 0;
            }
            return 1;
        }
        int test_static_dot(void) {
            static struct outer s;
            s.in.a = 1.0;
            s.in.b = 2;
            s.in.ptr = &ptr_target;
            if (s.in.a != 1.0 || s.in.b != 2 || s.in.ptr != &ptr_target) {
                return 0;
            }
            char *char_ptr = &s.in.b;
            if (*char_ptr != 2) {
                return 0;
            }
            *s.in.ptr = 6;
            if (ptr_target != 6) {
                return 0;
            }
            s.in.a = s.in.b;
            if (s.in.a != 2.0) {
                return 0;
            }
            return 1;
        }
        int test_auto_arrow(void) {
            struct inner in;
            struct outer s;
            struct outer *s_ptr = &s;
            s_ptr->in_ptr = &in;
            s_ptr->l = 4294967295ul;
            s_ptr->bar = -5;
            s_ptr->in_ptr->a = 10.0;
            s_ptr->in_ptr->b = 'x';
            s_ptr->in_array->a = 11.0;
            (s_ptr->in_array + 3)->a = 12.0;
            s_ptr->in_array->ptr = &s_ptr->bar;
            if (s_ptr->l != 4294967295ul || s_ptr->bar != -5) {
                return 0;
            }
            if (s_ptr->in_ptr->a != 10.0 || s_ptr->in_ptr->b != 'x' ||
                s_ptr->in_array->a != 11.0 || (s_ptr->in_array + 3)->a != 12.0) {
                return 0;
            }
            char *char_ptr = &s_ptr->in_ptr->b;
            if (*char_ptr != 'x') {
                return 0;
            }
            *s_ptr->in_array->ptr = 123;
            if (s_ptr->bar != 123) {
                return 0;
            }
            s_ptr->in_array->b = s_ptr->in_ptr->b;
            if (s_ptr->in_array[0].b != 'x') {
                return 0;
            }
            return 1;
        }
        int test_static_arrow(void) {
            static struct inner in;
            static struct outer s;
            static struct outer *s_ptr;
            s_ptr = &s;
            s_ptr->in_ptr = &in;
            s_ptr->l = 4294967295ul;
            s_ptr->bar = -5;
            s_ptr->in_ptr->a = 10.0;
            s_ptr->in_ptr->b = 'x';
            s_ptr->in_array->a = 11.0;
            (s_ptr->in_array + 3)->a = 12.0;
            s_ptr->in_array->ptr = &s_ptr->bar;
            if (s_ptr->l != 4294967295ul || s_ptr->bar != -5) {
                return 0;
            }
            if (s_ptr->in_ptr->a != 10.0 || s_ptr->in_ptr->b != 'x' ||
                s_ptr->in_array->a != 11.0 || (s_ptr->in_array + 3)->a != 12.0) {
                return 0;
            }
            char *char_ptr = &s_ptr->in_ptr->b;
            if (*char_ptr != 'x') {
                return 0;
            }
            *s_ptr->in_array->ptr = 123;
            if (s_ptr->bar != 123) {
                return 0;
            }
            s_ptr->in_ptr->b = s_ptr->in_ptr->a;
            if (s_ptr->in_ptr->b != 10) {
                return 0;
            }
            return 1;
        }
        int test_mixed(void) {
            struct inner *in_ptr = malloc(sizeof(struct inner));
            struct outer out;
            out.in_ptr = in_ptr;
            struct outer *out_ptr = &out;
            out.l = 10;
            out.bar = 20;
            out.in_ptr->a = -1.0;
            out.in_ptr->b = '!';
            out.in_ptr->ptr = 0;
            out_ptr->in_array[0].a = -2.0;
            out_ptr->in_array[0].b = '?';
            out_ptr->in_array[0].ptr = 0;
            out_ptr->in_array[3].a = -3.0;
            out_ptr->in_array[3].b = '*';
            out_ptr->in_array[3].ptr = malloc(sizeof(int));
            out_ptr->in.a = -3.0;
            out_ptr->in.b = '&';
            int i = 9;
            out_ptr->in.ptr = &i;
            if (out.l != 10 || out.bar != 20) {
                return 0;
            }
            if (out.in_ptr->a != -1.0 || out.in_ptr->b != '!' || out.in_ptr->ptr) {
                return 0;
            }
            if (out_ptr->in_array[0].a != -2.0 || out_ptr->in_array[0].b != '?' ||
                out_ptr->in_array[0].ptr || out_ptr->in_array[3].a != -3.0 ||
                out_ptr->in_array[3].b != '*' || out_ptr->in.a != -3.0 ||
                out_ptr->in.b != '&' || out_ptr->in.ptr != &i) {
                return 0;
            }
            *out_ptr->in_array[3].ptr = 5;
            if (*out_ptr->in_array[3].ptr != 5) {
                return 0;
            }
            out_ptr->in.b = out.in_ptr->b;
            if (out_ptr->in.b != out.in_ptr->b) {
                return 0;
            }
            return 1;
        }
        int test_array_of_structs(void) {
            static struct outer struct_array[3];
            struct inner *in_ptr = malloc(sizeof(struct inner));
            struct_array[0].in_ptr = in_ptr;
            struct_array[1].in_ptr = in_ptr;
            struct_array[0].in_ptr->a = 20.0;
            struct_array[1].in_ptr->b = '@';
            struct_array[0].in_ptr->ptr = 0;
            struct_array[1].in_array[1].a = 30.0;
            struct_array[1].in_array[0].b = '#';
            struct_array[2].in.a = 40.0;
            struct_array[2].in.b = '$';
            if (struct_array[1].in_ptr->a != 20.0 || struct_array[0].in_ptr->b != '@' ||
                struct_array[1].in_ptr->ptr) {
                return 0;
            }
            if (struct_array[1].in_array[1].a != 30.0 ||
                struct_array[1].in_array[0].b != '#' || struct_array[2].in.a != 40.0 ||
                struct_array[2].in.b != '$') {
                return 0;
            }
            return 1;
        }
        int test_array_of_struct_pointers(void) {
            struct outer *ptr_array[2];
            ptr_array[0] = calloc(1, sizeof(struct outer));
            ptr_array[1] = calloc(1, sizeof(struct outer));
            ptr_array[1]->in_ptr = calloc(1, sizeof(struct inner));
            ptr_array[1]->in_ptr->ptr = 0;
            ptr_array[1]->in_ptr->b = '%';
            ptr_array[1]->in_ptr->a = 876.5;
            ptr_array[1]->in_array[2].a = 1000.5;
            ptr_array[1]->in.a = 7e6;
            ptr_array[0]->in_ptr = calloc(1, sizeof(struct inner));
            ptr_array[0]->in_ptr->ptr = 0;
            ptr_array[0]->in_ptr->b = '^';
            ptr_array[0]->in_ptr->a = 123.4;
            ptr_array[0]->in_array[1].b = '&';
            ptr_array[0]->in.ptr = &ptr_array[0]->bar;
            ptr_array[0]->bar = 900;
            if (ptr_array[0]->in_array[1].b != '&') {
                return 0;
            }
            if (ptr_array[0]->in_ptr->a != 123.4 || ptr_array[0]->in_ptr->b != '^' ||
                ptr_array[0]->in_ptr->ptr) {
                return 0;
            }
            if (ptr_array[1]->in.a != 7e6) {
                return 0;
            }
            if (ptr_array[1]->in_array[2].a != 1000.5) {
                return 0;
            }
            if (ptr_array[1]->in_ptr->a != 876.5 || ptr_array[1]->in_ptr->b != '%' ||
                ptr_array[1]->in_ptr->ptr) {
                return 0;
            }
            if (*ptr_array[0]->in.ptr != 900) {
                return 0;
            }
            if (ptr_array[0]->l) {
                return 0;
            }
            for (int i = 0; i < 4; i = i + 1) {
                struct inner *elem_ptr = &ptr_array[0]->in_array[i];
                if (elem_ptr->a || elem_ptr->ptr) {
                    return 0;
                }
                if (elem_ptr->b && i != 1) {
                    return 0;
                }
            }
            if (ptr_array[0]->in.a || ptr_array[0]->in.b) {
                return 0;
            }
            if (ptr_array[1]->l || ptr_array[1]->bar) {
                return 0;
            }
            for (int i = 0; i < 4; i = i + 1) {
                struct inner *elem_ptr = &ptr_array[1]->in_array[i];
                if (elem_ptr->b || elem_ptr->ptr) {
                    return 0;
                }
                if (elem_ptr->a && i != 2) {
                    return 0;
                }
            }
            if (ptr_array[1]->in.b || ptr_array[1]->in.ptr) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_auto_dot()) {
                return 1;
            }
            if (!test_static_dot()) {
                return 2;
            }
            if (!test_auto_arrow()) {
                return 3;
            }
            if (!test_static_arrow()) {
                return 4;
            }
            if (!test_mixed()) {
                return 5;
            }
            if (!test_array_of_structs()) {
                return 6;
            }
            if (!test_array_of_struct_pointers()) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_auto_dot() { 
            s.5[4] = 1D
            tmp.0 = truncate 2
            s.5[5] = tmp.0
            tmp.1 = &ptr_target
            s.5[6] = tmp.1
            tmp.2 = s.5[4]
            tmp.3 = tmp.2 != 1D
            if tmp.3 jump or_true_0
            tmp.6 = s.5[5]
            tmp.7 = sign_extend tmp.6
            tmp.8 = tmp.7 != 2
            if tmp.8 jump or_true_0
            tmp.5 = 0
            jump or_end_1
        
          or_true_0:
            tmp.5 = 1
        
          or_end_1:
            if tmp.5 jump or_true_2
            tmp.11 = s.5[6]
            tmp.13 = &ptr_target
            tmp.12 = tmp.11 != tmp.13
            if tmp.12 jump or_true_2
            tmp.10 = 0
            jump or_end_3
        
          or_true_2:
            tmp.10 = 1
        
          or_end_3:
            if !tmp.10 jump end_if_4
            return 0
        
          end_if_4:
            tmp.14 = &s.5
            tmp.14 = add_ptr(tmp.14, index=5L, scale=1)
            char_ptr.6 = tmp.14
            tmp.15 = *char_ptr.6
            tmp.16 = sign_extend tmp.15
            tmp.17 = tmp.16 != 2
            if !tmp.17 jump end_if_6
            return 0
        
          end_if_6:
            tmp.18 = s.5[6]
            *tmp.18 = 5
            tmp.19 = ptr_target != 5
            if !tmp.19 jump end_if_8
            return 0
        
          end_if_8:
            tmp.20 = s.5[5]
            tmp.21 = int_to_double tmp.20
            s.5[4] = tmp.21
            tmp.22 = s.5[4]
            tmp.23 = tmp.22 != 2D
            if !tmp.23 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_static_dot() { 
            s.7[4] = 1D
            tmp.24 = truncate 2
            s.7[5] = tmp.24
            tmp.25 = &ptr_target
            s.7[6] = tmp.25
            tmp.26 = s.7[4]
            tmp.27 = tmp.26 != 1D
            if tmp.27 jump or_true_12
            tmp.30 = s.7[5]
            tmp.31 = sign_extend tmp.30
            tmp.32 = tmp.31 != 2
            if tmp.32 jump or_true_12
            tmp.29 = 0
            jump or_end_13
        
          or_true_12:
            tmp.29 = 1
        
          or_end_13:
            if tmp.29 jump or_true_14
            tmp.35 = s.7[6]
            tmp.37 = &ptr_target
            tmp.36 = tmp.35 != tmp.37
            if tmp.36 jump or_true_14
            tmp.34 = 0
            jump or_end_15
        
          or_true_14:
            tmp.34 = 1
        
          or_end_15:
            if !tmp.34 jump end_if_16
            return 0
        
          end_if_16:
            tmp.38 = &s.7
            tmp.38 = add_ptr(tmp.38, index=5L, scale=1)
            char_ptr.8 = tmp.38
            tmp.39 = *char_ptr.8
            tmp.40 = sign_extend tmp.39
            tmp.41 = tmp.40 != 2
            if !tmp.41 jump end_if_18
            return 0
        
          end_if_18:
            tmp.42 = s.7[6]
            *tmp.42 = 6
            tmp.43 = ptr_target != 6
            if !tmp.43 jump end_if_20
            return 0
        
          end_if_20:
            tmp.44 = s.7[5]
            tmp.45 = int_to_double tmp.44
            s.7[4] = tmp.45
            tmp.46 = s.7[4]
            tmp.47 = tmp.46 != 2D
            if !tmp.47 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function test_auto_arrow() { 
            tmp.48 = &s.10
            s_ptr.11 = tmp.48
            tmp.49 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.50 = &in.9
            tmp.49 = tmp.50
            s_ptr.11 = 4294967295UL
            tmp.51 = add_ptr(s_ptr.11, index=3L, scale=1)
            tmp.52 = - 5
            tmp.51 = tmp.52
            tmp.53 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.53 = 10D
            tmp.54 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.55 = add_ptr(tmp.54, index=1L, scale=1)
            tmp.56 = truncate 120
            tmp.55 = tmp.56
            tmp.57 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.58 = &tmp.57
            tmp.58 = 11D
            tmp.59 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.60 = &tmp.59
            tmp.62 = sign_extend 3
            tmp.61 = add_ptr(tmp.60, index=tmp.62, scale=24)
            tmp.61 = 12D
            tmp.63 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.64 = &tmp.63
            tmp.65 = add_ptr(tmp.64, index=2L, scale=1)
            tmp.66 = add_ptr(s_ptr.11, index=3L, scale=1)
            tmp.67 = &tmp.66
            tmp.65 = tmp.67
            tmp.68 = s_ptr.11 != 4294967295UL
            if tmp.68 jump or_true_24
            tmp.71 = add_ptr(s_ptr.11, index=3L, scale=1)
            tmp.73 = - 5
            tmp.72 = tmp.71 != tmp.73
            if tmp.72 jump or_true_24
            tmp.70 = 0
            jump or_end_25
        
          or_true_24:
            tmp.70 = 1
        
          or_end_25:
            if !tmp.70 jump end_if_26
            return 0
        
          end_if_26:
            tmp.74 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.75 = tmp.74 != 10D
            if tmp.75 jump or_true_28
            tmp.78 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.79 = add_ptr(tmp.78, index=1L, scale=1)
            tmp.80 = sign_extend tmp.79
            tmp.81 = tmp.80 != 120
            if tmp.81 jump or_true_28
            tmp.77 = 0
            jump or_end_29
        
          or_true_28:
            tmp.77 = 1
        
          or_end_29:
            if tmp.77 jump or_true_30
            tmp.84 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.85 = &tmp.84
            tmp.86 = tmp.85 != 11D
            if tmp.86 jump or_true_30
            tmp.83 = 0
            jump or_end_31
        
          or_true_30:
            tmp.83 = 1
        
          or_end_31:
            if tmp.83 jump or_true_32
            tmp.89 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.90 = &tmp.89
            tmp.92 = sign_extend 3
            tmp.91 = add_ptr(tmp.90, index=tmp.92, scale=24)
            tmp.93 = tmp.91 != 12D
            if tmp.93 jump or_true_32
            tmp.88 = 0
            jump or_end_33
        
          or_true_32:
            tmp.88 = 1
        
          or_end_33:
            if !tmp.88 jump end_if_34
            return 0
        
          end_if_34:
            tmp.94 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.95 = add_ptr(tmp.94, index=1L, scale=1)
            tmp.96 = &tmp.95
            char_ptr.12 = tmp.96
            tmp.97 = *char_ptr.12
            tmp.98 = sign_extend tmp.97
            tmp.99 = tmp.98 != 120
            if !tmp.99 jump end_if_36
            return 0
        
          end_if_36:
            tmp.100 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.101 = &tmp.100
            tmp.102 = add_ptr(tmp.101, index=2L, scale=1)
            *tmp.102 = 123
            tmp.103 = add_ptr(s_ptr.11, index=3L, scale=1)
            tmp.104 = tmp.103 != 123
            if !tmp.104 jump end_if_38
            return 0
        
          end_if_38:
            tmp.105 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.106 = &tmp.105
            tmp.107 = add_ptr(tmp.106, index=1L, scale=1)
            tmp.108 = add_ptr(s_ptr.11, index=1L, scale=1)
            tmp.109 = add_ptr(tmp.108, index=1L, scale=1)
            tmp.107 = tmp.109
            tmp.110 = add_ptr(s_ptr.11, index=2L, scale=1)
            tmp.111 = &tmp.110
            tmp.112 = sign_extend 0
            tmp.113 = add_ptr(tmp.111, index=tmp.112, scale=24)
            tmp.114 = add_ptr(tmp.113, index=1L, scale=1)
            tmp.115 = *tmp.114
            tmp.116 = sign_extend tmp.115
            tmp.117 = tmp.116 != 120
            if !tmp.117 jump end_if_40
            return 0
        
          end_if_40:
            return 1
            return 0
        }
        global function test_static_arrow() { 
            tmp.118 = &s.14
            s_ptr.15 = tmp.118
            tmp.119 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.120 = &in.13
            tmp.119 = tmp.120
            s_ptr.15 = 4294967295UL
            tmp.121 = add_ptr(s_ptr.15, index=3L, scale=1)
            tmp.122 = - 5
            tmp.121 = tmp.122
            tmp.123 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.123 = 10D
            tmp.124 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.125 = add_ptr(tmp.124, index=1L, scale=1)
            tmp.126 = truncate 120
            tmp.125 = tmp.126
            tmp.127 = add_ptr(s_ptr.15, index=2L, scale=1)
            tmp.128 = &tmp.127
            tmp.128 = 11D
            tmp.129 = add_ptr(s_ptr.15, index=2L, scale=1)
            tmp.130 = &tmp.129
            tmp.132 = sign_extend 3
            tmp.131 = add_ptr(tmp.130, index=tmp.132, scale=24)
            tmp.131 = 12D
            tmp.133 = add_ptr(s_ptr.15, index=2L, scale=1)
            tmp.134 = &tmp.133
            tmp.135 = add_ptr(tmp.134, index=2L, scale=1)
            tmp.136 = add_ptr(s_ptr.15, index=3L, scale=1)
            tmp.137 = &tmp.136
            tmp.135 = tmp.137
            tmp.138 = s_ptr.15 != 4294967295UL
            if tmp.138 jump or_true_42
            tmp.141 = add_ptr(s_ptr.15, index=3L, scale=1)
            tmp.143 = - 5
            tmp.142 = tmp.141 != tmp.143
            if tmp.142 jump or_true_42
            tmp.140 = 0
            jump or_end_43
        
          or_true_42:
            tmp.140 = 1
        
          or_end_43:
            if !tmp.140 jump end_if_44
            return 0
        
          end_if_44:
            tmp.144 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.145 = tmp.144 != 10D
            if tmp.145 jump or_true_46
            tmp.148 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.149 = add_ptr(tmp.148, index=1L, scale=1)
            tmp.150 = sign_extend tmp.149
            tmp.151 = tmp.150 != 120
            if tmp.151 jump or_true_46
            tmp.147 = 0
            jump or_end_47
        
          or_true_46:
            tmp.147 = 1
        
          or_end_47:
            if tmp.147 jump or_true_48
            tmp.154 = add_ptr(s_ptr.15, index=2L, scale=1)
            tmp.155 = &tmp.154
            tmp.156 = tmp.155 != 11D
            if tmp.156 jump or_true_48
            tmp.153 = 0
            jump or_end_49
        
          or_true_48:
            tmp.153 = 1
        
          or_end_49:
            if tmp.153 jump or_true_50
            tmp.159 = add_ptr(s_ptr.15, index=2L, scale=1)
            tmp.160 = &tmp.159
            tmp.162 = sign_extend 3
            tmp.161 = add_ptr(tmp.160, index=tmp.162, scale=24)
            tmp.163 = tmp.161 != 12D
            if tmp.163 jump or_true_50
            tmp.158 = 0
            jump or_end_51
        
          or_true_50:
            tmp.158 = 1
        
          or_end_51:
            if !tmp.158 jump end_if_52
            return 0
        
          end_if_52:
            tmp.164 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.165 = add_ptr(tmp.164, index=1L, scale=1)
            tmp.166 = &tmp.165
            char_ptr.16 = tmp.166
            tmp.167 = *char_ptr.16
            tmp.168 = sign_extend tmp.167
            tmp.169 = tmp.168 != 120
            if !tmp.169 jump end_if_54
            return 0
        
          end_if_54:
            tmp.170 = add_ptr(s_ptr.15, index=2L, scale=1)
            tmp.171 = &tmp.170
            tmp.172 = add_ptr(tmp.171, index=2L, scale=1)
            *tmp.172 = 123
            tmp.173 = add_ptr(s_ptr.15, index=3L, scale=1)
            tmp.174 = tmp.173 != 123
            if !tmp.174 jump end_if_56
            return 0
        
          end_if_56:
            tmp.175 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.176 = add_ptr(tmp.175, index=1L, scale=1)
            tmp.177 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.178 = double_to_int tmp.177
            tmp.176 = tmp.178
            tmp.179 = add_ptr(s_ptr.15, index=1L, scale=1)
            tmp.180 = add_ptr(tmp.179, index=1L, scale=1)
            tmp.181 = sign_extend tmp.180
            tmp.182 = tmp.181 != 10
            if !tmp.182 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function test_mixed() { 
            tmp.183 = malloc(24UL)
            tmp.184 = tmp.183
            in_ptr.17 = tmp.184
            out.18[1] = in_ptr.17
            tmp.185 = &out.18
            out_ptr.19 = tmp.185
            tmp.186 = sign_extend 10
            out.18[0] = tmp.186
            out.18[3] = 20
            tmp.187 = out.18[1]
            tmp.188 = - 1D
            tmp.187 = tmp.188
            tmp.189 = out.18[1]
            tmp.190 = add_ptr(tmp.189, index=1L, scale=1)
            tmp.191 = truncate 33
            tmp.190 = tmp.191
            tmp.192 = out.18[1]
            tmp.193 = add_ptr(tmp.192, index=2L, scale=1)
            tmp.194 = sign_extend 0
            tmp.193 = tmp.194
            tmp.195 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.196 = &tmp.195
            tmp.197 = sign_extend 0
            tmp.198 = add_ptr(tmp.196, index=tmp.197, scale=24)
            tmp.199 = - 2D
            *tmp.198 = tmp.199
            tmp.200 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.201 = &tmp.200
            tmp.202 = sign_extend 0
            tmp.203 = add_ptr(tmp.201, index=tmp.202, scale=24)
            tmp.204 = add_ptr(tmp.203, index=1L, scale=1)
            tmp.205 = truncate 63
            *tmp.204 = tmp.205
            tmp.206 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.207 = &tmp.206
            tmp.208 = sign_extend 0
            tmp.209 = add_ptr(tmp.207, index=tmp.208, scale=24)
            tmp.210 = add_ptr(tmp.209, index=2L, scale=1)
            tmp.211 = sign_extend 0
            *tmp.210 = tmp.211
            tmp.212 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.213 = &tmp.212
            tmp.214 = sign_extend 3
            tmp.215 = add_ptr(tmp.213, index=tmp.214, scale=24)
            tmp.216 = - 3D
            *tmp.215 = tmp.216
            tmp.217 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.218 = &tmp.217
            tmp.219 = sign_extend 3
            tmp.220 = add_ptr(tmp.218, index=tmp.219, scale=24)
            tmp.221 = add_ptr(tmp.220, index=1L, scale=1)
            tmp.222 = truncate 42
            *tmp.221 = tmp.222
            tmp.223 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.224 = &tmp.223
            tmp.225 = sign_extend 3
            tmp.226 = add_ptr(tmp.224, index=tmp.225, scale=24)
            tmp.227 = add_ptr(tmp.226, index=2L, scale=1)
            tmp.228 = malloc(4UL)
            tmp.229 = tmp.228
            *tmp.227 = tmp.229
            tmp.230 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.231 = - 3D
            tmp.230[0] = tmp.231
            tmp.232 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.233 = truncate 38
            tmp.232[1] = tmp.233
            i.20 = 9
            tmp.234 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.235 = &i.20
            tmp.234[2] = tmp.235
            tmp.236 = out.18[0]
            tmp.238 = sign_extend 10
            tmp.237 = tmp.236 != tmp.238
            if tmp.237 jump or_true_60
            tmp.241 = out.18[3]
            tmp.242 = tmp.241 != 20
            if tmp.242 jump or_true_60
            tmp.240 = 0
            jump or_end_61
        
          or_true_60:
            tmp.240 = 1
        
          or_end_61:
            if !tmp.240 jump end_if_62
            return 0
        
          end_if_62:
            tmp.243 = out.18[1]
            tmp.245 = - 1D
            tmp.244 = tmp.243 != tmp.245
            if tmp.244 jump or_true_64
            tmp.248 = out.18[1]
            tmp.249 = add_ptr(tmp.248, index=1L, scale=1)
            tmp.250 = sign_extend tmp.249
            tmp.251 = tmp.250 != 33
            if tmp.251 jump or_true_64
            tmp.247 = 0
            jump or_end_65
        
          or_true_64:
            tmp.247 = 1
        
          or_end_65:
            if tmp.247 jump or_true_66
            tmp.254 = out.18[1]
            tmp.255 = add_ptr(tmp.254, index=2L, scale=1)
            if tmp.255 jump or_true_66
            tmp.253 = 0
            jump or_end_67
        
          or_true_66:
            tmp.253 = 1
        
          or_end_67:
            if !tmp.253 jump end_if_68
            return 0
        
          end_if_68:
            tmp.256 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.257 = &tmp.256
            tmp.258 = sign_extend 0
            tmp.259 = add_ptr(tmp.257, index=tmp.258, scale=24)
            tmp.260 = *tmp.259
            tmp.262 = - 2D
            tmp.261 = tmp.260 != tmp.262
            if tmp.261 jump or_true_70
            tmp.265 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.266 = &tmp.265
            tmp.267 = sign_extend 0
            tmp.268 = add_ptr(tmp.266, index=tmp.267, scale=24)
            tmp.269 = add_ptr(tmp.268, index=1L, scale=1)
            tmp.270 = *tmp.269
            tmp.271 = sign_extend tmp.270
            tmp.272 = tmp.271 != 63
            if tmp.272 jump or_true_70
            tmp.264 = 0
            jump or_end_71
        
          or_true_70:
            tmp.264 = 1
        
          or_end_71:
            if tmp.264 jump or_true_72
            tmp.275 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.276 = &tmp.275
            tmp.277 = sign_extend 0
            tmp.278 = add_ptr(tmp.276, index=tmp.277, scale=24)
            tmp.279 = add_ptr(tmp.278, index=2L, scale=1)
            tmp.280 = *tmp.279
            if tmp.280 jump or_true_72
            tmp.274 = 0
            jump or_end_73
        
          or_true_72:
            tmp.274 = 1
        
          or_end_73:
            if tmp.274 jump or_true_74
            tmp.283 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.284 = &tmp.283
            tmp.285 = sign_extend 3
            tmp.286 = add_ptr(tmp.284, index=tmp.285, scale=24)
            tmp.287 = *tmp.286
            tmp.289 = - 3D
            tmp.288 = tmp.287 != tmp.289
            if tmp.288 jump or_true_74
            tmp.282 = 0
            jump or_end_75
        
          or_true_74:
            tmp.282 = 1
        
          or_end_75:
            if tmp.282 jump or_true_76
            tmp.292 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.293 = &tmp.292
            tmp.294 = sign_extend 3
            tmp.295 = add_ptr(tmp.293, index=tmp.294, scale=24)
            tmp.296 = add_ptr(tmp.295, index=1L, scale=1)
            tmp.297 = *tmp.296
            tmp.298 = sign_extend tmp.297
            tmp.299 = tmp.298 != 42
            if tmp.299 jump or_true_76
            tmp.291 = 0
            jump or_end_77
        
          or_true_76:
            tmp.291 = 1
        
          or_end_77:
            if tmp.291 jump or_true_78
            tmp.302 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.303 = tmp.302[0]
            tmp.305 = - 3D
            tmp.304 = tmp.303 != tmp.305
            if tmp.304 jump or_true_78
            tmp.301 = 0
            jump or_end_79
        
          or_true_78:
            tmp.301 = 1
        
          or_end_79:
            if tmp.301 jump or_true_80
            tmp.308 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.309 = tmp.308[1]
            tmp.310 = sign_extend tmp.309
            tmp.311 = tmp.310 != 38
            if tmp.311 jump or_true_80
            tmp.307 = 0
            jump or_end_81
        
          or_true_80:
            tmp.307 = 1
        
          or_end_81:
            if tmp.307 jump or_true_82
            tmp.314 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.315 = tmp.314[2]
            tmp.317 = &i.20
            tmp.316 = tmp.315 != tmp.317
            if tmp.316 jump or_true_82
            tmp.313 = 0
            jump or_end_83
        
          or_true_82:
            tmp.313 = 1
        
          or_end_83:
            if !tmp.313 jump end_if_84
            return 0
        
          end_if_84:
            tmp.318 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.319 = &tmp.318
            tmp.320 = sign_extend 3
            tmp.321 = add_ptr(tmp.319, index=tmp.320, scale=24)
            tmp.322 = add_ptr(tmp.321, index=2L, scale=1)
            tmp.323 = *tmp.322
            *tmp.323 = 5
            tmp.324 = add_ptr(out_ptr.19, index=2L, scale=1)
            tmp.325 = &tmp.324
            tmp.326 = sign_extend 3
            tmp.327 = add_ptr(tmp.325, index=tmp.326, scale=24)
            tmp.328 = add_ptr(tmp.327, index=2L, scale=1)
            tmp.329 = *tmp.328
            tmp.330 = *tmp.329
            tmp.331 = tmp.330 != 5
            if !tmp.331 jump end_if_86
            return 0
        
          end_if_86:
            tmp.332 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.333 = out.18[1]
            tmp.334 = add_ptr(tmp.333, index=1L, scale=1)
            tmp.332[1] = tmp.334
            tmp.335 = add_ptr(out_ptr.19, index=4L, scale=1)
            tmp.336 = tmp.335[1]
            tmp.337 = sign_extend tmp.336
            tmp.339 = out.18[1]
            tmp.340 = add_ptr(tmp.339, index=1L, scale=1)
            tmp.341 = sign_extend tmp.340
            tmp.338 = tmp.337 != tmp.341
            if !tmp.338 jump end_if_88
            return 0
        
          end_if_88:
            return 1
            return 0
        }
        global function test_array_of_structs() { 
            tmp.342 = malloc(24UL)
            tmp.343 = tmp.342
            in_ptr.22 = tmp.343
            tmp.344 = &struct_array.21
            tmp.345 = sign_extend 0
            tmp.346 = add_ptr(tmp.344, index=tmp.345, scale=144)
            tmp.347 = add_ptr(tmp.346, index=1L, scale=1)
            *tmp.347 = in_ptr.22
            tmp.348 = &struct_array.21
            tmp.349 = sign_extend 1
            tmp.350 = add_ptr(tmp.348, index=tmp.349, scale=144)
            tmp.351 = add_ptr(tmp.350, index=1L, scale=1)
            *tmp.351 = in_ptr.22
            tmp.352 = &struct_array.21
            tmp.353 = sign_extend 0
            tmp.354 = add_ptr(tmp.352, index=tmp.353, scale=144)
            tmp.355 = add_ptr(tmp.354, index=1L, scale=1)
            tmp.356 = *tmp.355
            tmp.356 = 20D
            tmp.357 = &struct_array.21
            tmp.358 = sign_extend 1
            tmp.359 = add_ptr(tmp.357, index=tmp.358, scale=144)
            tmp.360 = add_ptr(tmp.359, index=1L, scale=1)
            tmp.361 = *tmp.360
            tmp.362 = add_ptr(tmp.361, index=1L, scale=1)
            tmp.363 = truncate 64
            tmp.362 = tmp.363
            tmp.364 = &struct_array.21
            tmp.365 = sign_extend 0
            tmp.366 = add_ptr(tmp.364, index=tmp.365, scale=144)
            tmp.367 = add_ptr(tmp.366, index=1L, scale=1)
            tmp.368 = *tmp.367
            tmp.369 = add_ptr(tmp.368, index=2L, scale=1)
            tmp.370 = sign_extend 0
            tmp.369 = tmp.370
            tmp.371 = &struct_array.21
            tmp.372 = sign_extend 1
            tmp.373 = add_ptr(tmp.371, index=tmp.372, scale=144)
            tmp.374 = add_ptr(tmp.373, index=2L, scale=1)
            tmp.375 = sign_extend 1
            tmp.376 = add_ptr(tmp.374, index=tmp.375, scale=24)
            *tmp.376 = 30D
            tmp.377 = &struct_array.21
            tmp.378 = sign_extend 1
            tmp.379 = add_ptr(tmp.377, index=tmp.378, scale=144)
            tmp.380 = add_ptr(tmp.379, index=2L, scale=1)
            tmp.381 = sign_extend 0
            tmp.382 = add_ptr(tmp.380, index=tmp.381, scale=24)
            tmp.383 = add_ptr(tmp.382, index=1L, scale=1)
            tmp.384 = truncate 35
            *tmp.383 = tmp.384
            tmp.385 = &struct_array.21
            tmp.386 = sign_extend 2
            tmp.387 = add_ptr(tmp.385, index=tmp.386, scale=144)
            tmp.388 = add_ptr(tmp.387, index=4L, scale=1)
            *tmp.388 = 40D
            tmp.389 = &struct_array.21
            tmp.390 = sign_extend 2
            tmp.391 = add_ptr(tmp.389, index=tmp.390, scale=144)
            tmp.392 = add_ptr(tmp.391, index=4L, scale=1)
            tmp.393 = add_ptr(tmp.392, index=1L, scale=1)
            tmp.394 = truncate 36
            *tmp.393 = tmp.394
            tmp.395 = &struct_array.21
            tmp.396 = sign_extend 1
            tmp.397 = add_ptr(tmp.395, index=tmp.396, scale=144)
            tmp.398 = add_ptr(tmp.397, index=1L, scale=1)
            tmp.399 = *tmp.398
            tmp.400 = tmp.399 != 20D
            if tmp.400 jump or_true_90
            tmp.403 = &struct_array.21
            tmp.404 = sign_extend 0
            tmp.405 = add_ptr(tmp.403, index=tmp.404, scale=144)
            tmp.406 = add_ptr(tmp.405, index=1L, scale=1)
            tmp.407 = *tmp.406
            tmp.408 = add_ptr(tmp.407, index=1L, scale=1)
            tmp.409 = sign_extend tmp.408
            tmp.410 = tmp.409 != 64
            if tmp.410 jump or_true_90
            tmp.402 = 0
            jump or_end_91
        
          or_true_90:
            tmp.402 = 1
        
          or_end_91:
            if tmp.402 jump or_true_92
            tmp.413 = &struct_array.21
            tmp.414 = sign_extend 1
            tmp.415 = add_ptr(tmp.413, index=tmp.414, scale=144)
            tmp.416 = add_ptr(tmp.415, index=1L, scale=1)
            tmp.417 = *tmp.416
            tmp.418 = add_ptr(tmp.417, index=2L, scale=1)
            if tmp.418 jump or_true_92
            tmp.412 = 0
            jump or_end_93
        
          or_true_92:
            tmp.412 = 1
        
          or_end_93:
            if !tmp.412 jump end_if_94
            return 0
        
          end_if_94:
            tmp.419 = &struct_array.21
            tmp.420 = sign_extend 1
            tmp.421 = add_ptr(tmp.419, index=tmp.420, scale=144)
            tmp.422 = add_ptr(tmp.421, index=2L, scale=1)
            tmp.423 = sign_extend 1
            tmp.424 = add_ptr(tmp.422, index=tmp.423, scale=24)
            tmp.425 = *tmp.424
            tmp.426 = tmp.425 != 30D
            if tmp.426 jump or_true_96
            tmp.429 = &struct_array.21
            tmp.430 = sign_extend 1
            tmp.431 = add_ptr(tmp.429, index=tmp.430, scale=144)
            tmp.432 = add_ptr(tmp.431, index=2L, scale=1)
            tmp.433 = sign_extend 0
            tmp.434 = add_ptr(tmp.432, index=tmp.433, scale=24)
            tmp.435 = add_ptr(tmp.434, index=1L, scale=1)
            tmp.436 = *tmp.435
            tmp.437 = sign_extend tmp.436
            tmp.438 = tmp.437 != 35
            if tmp.438 jump or_true_96
            tmp.428 = 0
            jump or_end_97
        
          or_true_96:
            tmp.428 = 1
        
          or_end_97:
            if tmp.428 jump or_true_98
            tmp.441 = &struct_array.21
            tmp.442 = sign_extend 2
            tmp.443 = add_ptr(tmp.441, index=tmp.442, scale=144)
            tmp.444 = add_ptr(tmp.443, index=4L, scale=1)
            tmp.445 = *tmp.444
            tmp.446 = tmp.445 != 40D
            if tmp.446 jump or_true_98
            tmp.440 = 0
            jump or_end_99
        
          or_true_98:
            tmp.440 = 1
        
          or_end_99:
            if tmp.440 jump or_true_100
            tmp.449 = &struct_array.21
            tmp.450 = sign_extend 2
            tmp.451 = add_ptr(tmp.449, index=tmp.450, scale=144)
            tmp.452 = add_ptr(tmp.451, index=4L, scale=1)
            tmp.453 = add_ptr(tmp.452, index=1L, scale=1)
            tmp.454 = *tmp.453
            tmp.455 = sign_extend tmp.454
            tmp.456 = tmp.455 != 36
            if tmp.456 jump or_true_100
            tmp.448 = 0
            jump or_end_101
        
          or_true_100:
            tmp.448 = 1
        
          or_end_101:
            if !tmp.448 jump end_if_102
            return 0
        
          end_if_102:
            return 1
            return 0
        }
        global function test_array_of_struct_pointers() { 
            tmp.457 = &ptr_array.23
            tmp.458 = sign_extend 0
            tmp.459 = add_ptr(tmp.457, index=tmp.458, scale=8)
            tmp.460 = sign_extend 1
            tmp.461 = calloc(tmp.460, 144UL)
            tmp.462 = tmp.461
            *tmp.459 = tmp.462
            tmp.463 = &ptr_array.23
            tmp.464 = sign_extend 1
            tmp.465 = add_ptr(tmp.463, index=tmp.464, scale=8)
            tmp.466 = sign_extend 1
            tmp.467 = calloc(tmp.466, 144UL)
            tmp.468 = tmp.467
            *tmp.465 = tmp.468
            tmp.469 = &ptr_array.23
            tmp.470 = sign_extend 1
            tmp.471 = add_ptr(tmp.469, index=tmp.470, scale=8)
            tmp.472 = *tmp.471
            tmp.473 = add_ptr(tmp.472, index=1L, scale=1)
            tmp.474 = sign_extend 1
            tmp.475 = calloc(tmp.474, 24UL)
            tmp.476 = tmp.475
            tmp.473 = tmp.476
            tmp.477 = &ptr_array.23
            tmp.478 = sign_extend 1
            tmp.479 = add_ptr(tmp.477, index=tmp.478, scale=8)
            tmp.480 = *tmp.479
            tmp.481 = add_ptr(tmp.480, index=1L, scale=1)
            tmp.482 = add_ptr(tmp.481, index=2L, scale=1)
            tmp.483 = sign_extend 0
            tmp.482 = tmp.483
            tmp.484 = &ptr_array.23
            tmp.485 = sign_extend 1
            tmp.486 = add_ptr(tmp.484, index=tmp.485, scale=8)
            tmp.487 = *tmp.486
            tmp.488 = add_ptr(tmp.487, index=1L, scale=1)
            tmp.489 = add_ptr(tmp.488, index=1L, scale=1)
            tmp.490 = truncate 37
            tmp.489 = tmp.490
            tmp.491 = &ptr_array.23
            tmp.492 = sign_extend 1
            tmp.493 = add_ptr(tmp.491, index=tmp.492, scale=8)
            tmp.494 = *tmp.493
            tmp.495 = add_ptr(tmp.494, index=1L, scale=1)
            tmp.495 = 876.5D
            tmp.496 = &ptr_array.23
            tmp.497 = sign_extend 1
            tmp.498 = add_ptr(tmp.496, index=tmp.497, scale=8)
            tmp.499 = *tmp.498
            tmp.500 = add_ptr(tmp.499, index=2L, scale=1)
            tmp.501 = &tmp.500
            tmp.502 = sign_extend 2
            tmp.503 = add_ptr(tmp.501, index=tmp.502, scale=24)
            *tmp.503 = 1000.5D
            tmp.504 = &ptr_array.23
            tmp.505 = sign_extend 1
            tmp.506 = add_ptr(tmp.504, index=tmp.505, scale=8)
            tmp.507 = *tmp.506
            tmp.508 = add_ptr(tmp.507, index=4L, scale=1)
            tmp.508[0] = 7000000D
            tmp.509 = &ptr_array.23
            tmp.510 = sign_extend 0
            tmp.511 = add_ptr(tmp.509, index=tmp.510, scale=8)
            tmp.512 = *tmp.511
            tmp.513 = add_ptr(tmp.512, index=1L, scale=1)
            tmp.514 = sign_extend 1
            tmp.515 = calloc(tmp.514, 24UL)
            tmp.516 = tmp.515
            tmp.513 = tmp.516
            tmp.517 = &ptr_array.23
            tmp.518 = sign_extend 0
            tmp.519 = add_ptr(tmp.517, index=tmp.518, scale=8)
            tmp.520 = *tmp.519
            tmp.521 = add_ptr(tmp.520, index=1L, scale=1)
            tmp.522 = add_ptr(tmp.521, index=2L, scale=1)
            tmp.523 = sign_extend 0
            tmp.522 = tmp.523
            tmp.524 = &ptr_array.23
            tmp.525 = sign_extend 0
            tmp.526 = add_ptr(tmp.524, index=tmp.525, scale=8)
            tmp.527 = *tmp.526
            tmp.528 = add_ptr(tmp.527, index=1L, scale=1)
            tmp.529 = add_ptr(tmp.528, index=1L, scale=1)
            tmp.530 = truncate 94
            tmp.529 = tmp.530
            tmp.531 = &ptr_array.23
            tmp.532 = sign_extend 0
            tmp.533 = add_ptr(tmp.531, index=tmp.532, scale=8)
            tmp.534 = *tmp.533
            tmp.535 = add_ptr(tmp.534, index=1L, scale=1)
            tmp.535 = 123.4D
            tmp.536 = &ptr_array.23
            tmp.537 = sign_extend 0
            tmp.538 = add_ptr(tmp.536, index=tmp.537, scale=8)
            tmp.539 = *tmp.538
            tmp.540 = add_ptr(tmp.539, index=2L, scale=1)
            tmp.541 = &tmp.540
            tmp.542 = sign_extend 1
            tmp.543 = add_ptr(tmp.541, index=tmp.542, scale=24)
            tmp.544 = add_ptr(tmp.543, index=1L, scale=1)
            tmp.545 = truncate 38
            *tmp.544 = tmp.545
            tmp.546 = &ptr_array.23
            tmp.547 = sign_extend 0
            tmp.548 = add_ptr(tmp.546, index=tmp.547, scale=8)
            tmp.549 = *tmp.548
            tmp.550 = add_ptr(tmp.549, index=4L, scale=1)
            tmp.551 = &ptr_array.23
            tmp.552 = sign_extend 0
            tmp.553 = add_ptr(tmp.551, index=tmp.552, scale=8)
            tmp.554 = *tmp.553
            tmp.555 = add_ptr(tmp.554, index=3L, scale=1)
            tmp.556 = &tmp.555
            tmp.550[2] = tmp.556
            tmp.557 = &ptr_array.23
            tmp.558 = sign_extend 0
            tmp.559 = add_ptr(tmp.557, index=tmp.558, scale=8)
            tmp.560 = *tmp.559
            tmp.561 = add_ptr(tmp.560, index=3L, scale=1)
            tmp.561 = 900
            tmp.562 = &ptr_array.23
            tmp.563 = sign_extend 0
            tmp.564 = add_ptr(tmp.562, index=tmp.563, scale=8)
            tmp.565 = *tmp.564
            tmp.566 = add_ptr(tmp.565, index=2L, scale=1)
            tmp.567 = &tmp.566
            tmp.568 = sign_extend 1
            tmp.569 = add_ptr(tmp.567, index=tmp.568, scale=24)
            tmp.570 = add_ptr(tmp.569, index=1L, scale=1)
            tmp.571 = *tmp.570
            tmp.572 = sign_extend tmp.571
            tmp.573 = tmp.572 != 38
            if !tmp.573 jump end_if_104
            return 0
        
          end_if_104:
            tmp.574 = &ptr_array.23
            tmp.575 = sign_extend 0
            tmp.576 = add_ptr(tmp.574, index=tmp.575, scale=8)
            tmp.577 = *tmp.576
            tmp.578 = add_ptr(tmp.577, index=1L, scale=1)
            tmp.579 = tmp.578 != 123.4D
            if tmp.579 jump or_true_106
            tmp.582 = &ptr_array.23
            tmp.583 = sign_extend 0
            tmp.584 = add_ptr(tmp.582, index=tmp.583, scale=8)
            tmp.585 = *tmp.584
            tmp.586 = add_ptr(tmp.585, index=1L, scale=1)
            tmp.587 = add_ptr(tmp.586, index=1L, scale=1)
            tmp.588 = sign_extend tmp.587
            tmp.589 = tmp.588 != 94
            if tmp.589 jump or_true_106
            tmp.581 = 0
            jump or_end_107
        
          or_true_106:
            tmp.581 = 1
        
          or_end_107:
            if tmp.581 jump or_true_108
            tmp.592 = &ptr_array.23
            tmp.593 = sign_extend 0
            tmp.594 = add_ptr(tmp.592, index=tmp.593, scale=8)
            tmp.595 = *tmp.594
            tmp.596 = add_ptr(tmp.595, index=1L, scale=1)
            tmp.597 = add_ptr(tmp.596, index=2L, scale=1)
            if tmp.597 jump or_true_108
            tmp.591 = 0
            jump or_end_109
        
          or_true_108:
            tmp.591 = 1
        
          or_end_109:
            if !tmp.591 jump end_if_110
            return 0
        
          end_if_110:
            tmp.598 = &ptr_array.23
            tmp.599 = sign_extend 1
            tmp.600 = add_ptr(tmp.598, index=tmp.599, scale=8)
            tmp.601 = *tmp.600
            tmp.602 = add_ptr(tmp.601, index=4L, scale=1)
            tmp.603 = tmp.602[0]
            tmp.604 = tmp.603 != 7000000D
            if !tmp.604 jump end_if_112
            return 0
        
          end_if_112:
            tmp.605 = &ptr_array.23
            tmp.606 = sign_extend 1
            tmp.607 = add_ptr(tmp.605, index=tmp.606, scale=8)
            tmp.608 = *tmp.607
            tmp.609 = add_ptr(tmp.608, index=2L, scale=1)
            tmp.610 = &tmp.609
            tmp.611 = sign_extend 2
            tmp.612 = add_ptr(tmp.610, index=tmp.611, scale=24)
            tmp.613 = *tmp.612
            tmp.614 = tmp.613 != 1000.5D
            if !tmp.614 jump end_if_114
            return 0
        
          end_if_114:
            tmp.615 = &ptr_array.23
            tmp.616 = sign_extend 1
            tmp.617 = add_ptr(tmp.615, index=tmp.616, scale=8)
            tmp.618 = *tmp.617
            tmp.619 = add_ptr(tmp.618, index=1L, scale=1)
            tmp.620 = tmp.619 != 876.5D
            if tmp.620 jump or_true_116
            tmp.623 = &ptr_array.23
            tmp.624 = sign_extend 1
            tmp.625 = add_ptr(tmp.623, index=tmp.624, scale=8)
            tmp.626 = *tmp.625
            tmp.627 = add_ptr(tmp.626, index=1L, scale=1)
            tmp.628 = add_ptr(tmp.627, index=1L, scale=1)
            tmp.629 = sign_extend tmp.628
            tmp.630 = tmp.629 != 37
            if tmp.630 jump or_true_116
            tmp.622 = 0
            jump or_end_117
        
          or_true_116:
            tmp.622 = 1
        
          or_end_117:
            if tmp.622 jump or_true_118
            tmp.633 = &ptr_array.23
            tmp.634 = sign_extend 1
            tmp.635 = add_ptr(tmp.633, index=tmp.634, scale=8)
            tmp.636 = *tmp.635
            tmp.637 = add_ptr(tmp.636, index=1L, scale=1)
            tmp.638 = add_ptr(tmp.637, index=2L, scale=1)
            if tmp.638 jump or_true_118
            tmp.632 = 0
            jump or_end_119
        
          or_true_118:
            tmp.632 = 1
        
          or_end_119:
            if !tmp.632 jump end_if_120
            return 0
        
          end_if_120:
            tmp.639 = &ptr_array.23
            tmp.640 = sign_extend 0
            tmp.641 = add_ptr(tmp.639, index=tmp.640, scale=8)
            tmp.642 = *tmp.641
            tmp.643 = add_ptr(tmp.642, index=4L, scale=1)
            tmp.644 = tmp.643[2]
            tmp.645 = *tmp.644
            tmp.646 = tmp.645 != 900
            if !tmp.646 jump end_if_122
            return 0
        
          end_if_122:
            tmp.647 = &ptr_array.23
            tmp.648 = sign_extend 0
            tmp.649 = add_ptr(tmp.647, index=tmp.648, scale=8)
            tmp.650 = *tmp.649
            if !tmp.650 jump end_if_124
            return 0
        
          end_if_124:
            i.24 = 0
        
          start_loop_0:
            tmp.651 = i.24 < 4
            if !tmp.651 jump break_loop_0
            tmp.652 = &ptr_array.23
            tmp.653 = sign_extend 0
            tmp.654 = add_ptr(tmp.652, index=tmp.653, scale=8)
            tmp.655 = *tmp.654
            tmp.656 = add_ptr(tmp.655, index=2L, scale=1)
            tmp.657 = &tmp.656
            tmp.658 = sign_extend i.24
            tmp.659 = add_ptr(tmp.657, index=tmp.658, scale=24)
            elem_ptr.25 = tmp.659
            if elem_ptr.25 jump or_true_126
            tmp.662 = add_ptr(elem_ptr.25, index=2L, scale=1)
            if tmp.662 jump or_true_126
            tmp.661 = 0
            jump or_end_127
        
          or_true_126:
            tmp.661 = 1
        
          or_end_127:
            if !tmp.661 jump end_if_128
            return 0
        
          end_if_128:
            tmp.663 = add_ptr(elem_ptr.25, index=1L, scale=1)
            if !tmp.663 jump and_false_130
            tmp.666 = i.24 != 1
            if !tmp.666 jump and_false_130
            tmp.665 = 1
            jump and_end_131
        
          and_false_130:
            tmp.665 = 0
        
          and_end_131:
            if !tmp.665 jump end_if_132
            return 0
        
          end_if_132:
        
          continue_loop_0:
            tmp.667 = i.24 + 1
            i.24 = tmp.667
            jump start_loop_0
        
          break_loop_0:
            tmp.668 = &ptr_array.23
            tmp.669 = sign_extend 0
            tmp.670 = add_ptr(tmp.668, index=tmp.669, scale=8)
            tmp.671 = *tmp.670
            tmp.672 = add_ptr(tmp.671, index=4L, scale=1)
            tmp.673 = tmp.672[0]
            if tmp.673 jump or_true_134
            tmp.676 = &ptr_array.23
            tmp.677 = sign_extend 0
            tmp.678 = add_ptr(tmp.676, index=tmp.677, scale=8)
            tmp.679 = *tmp.678
            tmp.680 = add_ptr(tmp.679, index=4L, scale=1)
            tmp.681 = tmp.680[1]
            if tmp.681 jump or_true_134
            tmp.675 = 0
            jump or_end_135
        
          or_true_134:
            tmp.675 = 1
        
          or_end_135:
            if !tmp.675 jump end_if_136
            return 0
        
          end_if_136:
            tmp.682 = &ptr_array.23
            tmp.683 = sign_extend 1
            tmp.684 = add_ptr(tmp.682, index=tmp.683, scale=8)
            tmp.685 = *tmp.684
            if tmp.685 jump or_true_138
            tmp.688 = &ptr_array.23
            tmp.689 = sign_extend 1
            tmp.690 = add_ptr(tmp.688, index=tmp.689, scale=8)
            tmp.691 = *tmp.690
            tmp.692 = add_ptr(tmp.691, index=3L, scale=1)
            if tmp.692 jump or_true_138
            tmp.687 = 0
            jump or_end_139
        
          or_true_138:
            tmp.687 = 1
        
          or_end_139:
            if !tmp.687 jump end_if_140
            return 0
        
          end_if_140:
            i.26 = 0
        
          start_loop_1:
            tmp.693 = i.26 < 4
            if !tmp.693 jump break_loop_1
            tmp.694 = &ptr_array.23
            tmp.695 = sign_extend 1
            tmp.696 = add_ptr(tmp.694, index=tmp.695, scale=8)
            tmp.697 = *tmp.696
            tmp.698 = add_ptr(tmp.697, index=2L, scale=1)
            tmp.699 = &tmp.698
            tmp.700 = sign_extend i.26
            tmp.701 = add_ptr(tmp.699, index=tmp.700, scale=24)
            elem_ptr.27 = tmp.701
            tmp.702 = add_ptr(elem_ptr.27, index=1L, scale=1)
            if tmp.702 jump or_true_142
            tmp.705 = add_ptr(elem_ptr.27, index=2L, scale=1)
            if tmp.705 jump or_true_142
            tmp.704 = 0
            jump or_end_143
        
          or_true_142:
            tmp.704 = 1
        
          or_end_143:
            if !tmp.704 jump end_if_144
            return 0
        
          end_if_144:
            if !elem_ptr.27 jump and_false_146
            tmp.708 = i.26 != 2
            if !tmp.708 jump and_false_146
            tmp.707 = 1
            jump and_end_147
        
          and_false_146:
            tmp.707 = 0
        
          and_end_147:
            if !tmp.707 jump end_if_148
            return 0
        
          end_if_148:
        
          continue_loop_1:
            tmp.709 = i.26 + 1
            i.26 = tmp.709
            jump start_loop_1
        
          break_loop_1:
            tmp.710 = &ptr_array.23
            tmp.711 = sign_extend 1
            tmp.712 = add_ptr(tmp.710, index=tmp.711, scale=8)
            tmp.713 = *tmp.712
            tmp.714 = add_ptr(tmp.713, index=4L, scale=1)
            tmp.715 = tmp.714[1]
            if tmp.715 jump or_true_150
            tmp.718 = &ptr_array.23
            tmp.719 = sign_extend 1
            tmp.720 = add_ptr(tmp.718, index=tmp.719, scale=8)
            tmp.721 = *tmp.720
            tmp.722 = add_ptr(tmp.721, index=4L, scale=1)
            tmp.723 = tmp.722[2]
            if tmp.723 jump or_true_150
            tmp.717 = 0
            jump or_end_151
        
          or_true_150:
            tmp.717 = 1
        
          or_end_151:
            if !tmp.717 jump end_if_152
            return 0
        
          end_if_152:
            return 1
            return 0
        }
        global function main() { 
            tmp.724 = test_auto_dot()
            tmp.725 = ! tmp.724
            if !tmp.725 jump end_if_154
            return 1
        
          end_if_154:
            tmp.726 = test_static_dot()
            tmp.727 = ! tmp.726
            if !tmp.727 jump end_if_156
            return 2
        
          end_if_156:
            tmp.728 = test_auto_arrow()
            tmp.729 = ! tmp.728
            if !tmp.729 jump end_if_158
            return 3
        
          end_if_158:
            tmp.730 = test_static_arrow()
            tmp.731 = ! tmp.730
            if !tmp.731 jump end_if_160
            return 4
        
          end_if_160:
            tmp.732 = test_mixed()
            tmp.733 = ! tmp.732
            if !tmp.733 jump end_if_162
            return 5
        
          end_if_162:
            tmp.734 = test_array_of_structs()
            tmp.735 = ! tmp.734
            if !tmp.735 jump end_if_164
            return 6
        
          end_if_164:
            tmp.736 = test_array_of_struct_pointers()
            tmp.737 = ! tmp.736
            if !tmp.737 jump end_if_166
            return 7
        
          end_if_166:
            return 0
            return 0
        }
        static in.13: Struct(inner.3) = zero[24]
        static global ptr_target: Int = zero[4]
        static s.14: Struct(outer.4) = zero[144]
        static s.7: Struct(outer.4) = zero[144]
        static s_ptr.15: Pointer(Struct(outer.4)) = zero[8]
        static struct_array.21: Array(3,Struct(outer.4)) = zero[432]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_scalar_member_access_static_structs() {
    let src = r#"
        void *malloc(unsigned long size);
        int putchar(int ch);
        int puts(char *s);
        void test_static_local(int a, int b) {
            struct s {
                int a;
                int b;
            };
            static struct s static_struct;
            if (!(static_struct.a || static_struct.b)) {
                puts("zero");
            } else {
                putchar(static_struct.a);
                putchar(static_struct.b);
                putchar('\n');
            }
            static_struct.a = a;
            static_struct.b = b;
        }
        void test_static_local_pointer(int a, int b) {
            struct s {
                int a;
                int b;
            };
            static struct s *struct_ptr;
            if (!struct_ptr) {
                struct_ptr = malloc(sizeof(struct s));
            } else {
                putchar(struct_ptr->a);
                putchar(struct_ptr->b);
                putchar('\n');
            }
            struct_ptr->a = a;
            struct_ptr->b = b;
        }
        struct global {
            char x;
            char y;
            char z;
        };
        struct global g;
        void f1(void) {
            g.x = g.x + 1;
            g.y = g.y + 1;
            g.z = g.z + 1;
        }
        void f2(void) {
            putchar(g.x);
            putchar(g.y);
            putchar(g.z);
            putchar('\n');
        }
        void test_global_struct(void) {
            g.x = 'A';
            g.y = 'B';
            g.z = 'C';
            f1();
            f2();
            f1();
            f2();
        }
        struct global *g_ptr;
        void f3(void) {
            g_ptr->x = g_ptr->x + 1;
            g_ptr->y = g_ptr->y + 1;
            g_ptr->z = g_ptr->z + 1;
        }
        void f4(void) {
            putchar(g_ptr->x);
            putchar(g_ptr->y);
            putchar(g_ptr->z);
            putchar('\n');
        }
        void test_global_struct_pointer(void) {
            g_ptr = &g;
            f3();
            f4();
            f3();
            f4();
            g_ptr = malloc(sizeof(struct global));
            g_ptr->x = 'a';
            g_ptr->y = 'b';
            g_ptr->z = 'c';
            f3();
            f4();
            f3();
            f4();
        }
        int main(void) {
            test_static_local('m', 'n');
            test_static_local('o', 'p');
            test_static_local('!', '!');
            ;
            test_static_local_pointer('w', 'x');
            test_static_local_pointer('y', 'z');
            test_static_local_pointer('!', '!');
            ;
            test_global_struct();
            test_global_struct_pointer();
            return 0;
        }
    "#;
    let expected = r#"
        global function test_static_local(a.3, b.4) { 
            tmp.0 = static_struct.6[0]
            if tmp.0 jump or_true_0
            tmp.3 = static_struct.6[1]
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            tmp.4 = ! tmp.2
            if !tmp.4 jump else_3
            tmp.5 = &string.0
            tmp.6 = puts(tmp.5)
            jump end_if_2
        
          else_3:
            tmp.7 = static_struct.6[0]
            tmp.8 = putchar(tmp.7)
            tmp.9 = static_struct.6[1]
            tmp.10 = putchar(tmp.9)
            tmp.11 = putchar(10)
        
          end_if_2:
            static_struct.6[0] = a.3
            static_struct.6[1] = b.4
            return 0
        }
        global function test_static_local_pointer(a.7, b.8) { 
            tmp.12 = ! struct_ptr.10
            if !tmp.12 jump else_5
            tmp.13 = malloc(8UL)
            tmp.14 = tmp.13
            struct_ptr.10 = tmp.14
            jump end_if_4
        
          else_5:
            tmp.15 = putchar(struct_ptr.10)
            tmp.16 = add_ptr(struct_ptr.10, index=1L, scale=1)
            tmp.17 = putchar(tmp.16)
            tmp.18 = putchar(10)
        
          end_if_4:
            struct_ptr.10 = a.7
            tmp.19 = add_ptr(struct_ptr.10, index=1L, scale=1)
            tmp.19 = b.8
            return 0
        }
        global function f1() { 
            tmp.20 = g[0]
            tmp.21 = sign_extend tmp.20
            tmp.22 = tmp.21 + 1
            tmp.23 = truncate tmp.22
            g[0] = tmp.23
            tmp.24 = g[1]
            tmp.25 = sign_extend tmp.24
            tmp.26 = tmp.25 + 1
            tmp.27 = truncate tmp.26
            g[1] = tmp.27
            tmp.28 = g[2]
            tmp.29 = sign_extend tmp.28
            tmp.30 = tmp.29 + 1
            tmp.31 = truncate tmp.30
            g[2] = tmp.31
            return 0
        }
        global function f2() { 
            tmp.32 = g[0]
            tmp.33 = sign_extend tmp.32
            tmp.34 = putchar(tmp.33)
            tmp.35 = g[1]
            tmp.36 = sign_extend tmp.35
            tmp.37 = putchar(tmp.36)
            tmp.38 = g[2]
            tmp.39 = sign_extend tmp.38
            tmp.40 = putchar(tmp.39)
            tmp.41 = putchar(10)
            return 0
        }
        global function test_global_struct() { 
            tmp.42 = truncate 65
            g[0] = tmp.42
            tmp.43 = truncate 66
            g[1] = tmp.43
            tmp.44 = truncate 67
            g[2] = tmp.44
            f1()
            f2()
            f1()
            f2()
            return 0
        }
        global function f3() { 
            tmp.45 = sign_extend g_ptr
            tmp.46 = tmp.45 + 1
            tmp.47 = truncate tmp.46
            g_ptr = tmp.47
            tmp.48 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.49 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.50 = sign_extend tmp.49
            tmp.51 = tmp.50 + 1
            tmp.52 = truncate tmp.51
            tmp.48 = tmp.52
            tmp.53 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.54 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.55 = sign_extend tmp.54
            tmp.56 = tmp.55 + 1
            tmp.57 = truncate tmp.56
            tmp.53 = tmp.57
            return 0
        }
        global function f4() { 
            tmp.58 = sign_extend g_ptr
            tmp.59 = putchar(tmp.58)
            tmp.60 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.61 = sign_extend tmp.60
            tmp.62 = putchar(tmp.61)
            tmp.63 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.64 = sign_extend tmp.63
            tmp.65 = putchar(tmp.64)
            tmp.66 = putchar(10)
            return 0
        }
        global function test_global_struct_pointer() { 
            tmp.67 = &g
            g_ptr = tmp.67
            f3()
            f4()
            f3()
            f4()
            tmp.68 = malloc(3UL)
            tmp.69 = tmp.68
            g_ptr = tmp.69
            tmp.70 = truncate 97
            g_ptr = tmp.70
            tmp.71 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.72 = truncate 98
            tmp.71 = tmp.72
            tmp.73 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.74 = truncate 99
            tmp.73 = tmp.74
            f3()
            f4()
            f3()
            f4()
            return 0
        }
        global function main() { 
            test_static_local(109, 110)
            test_static_local(111, 112)
            test_static_local(33, 33)
            test_static_local_pointer(119, 120)
            test_static_local_pointer(121, 122)
            test_static_local_pointer(33, 33)
            test_global_struct()
            test_global_struct_pointer()
            return 0
            return 0
        }
        static global g: Struct(global.11) = zero[3]
        static global g_ptr: Pointer(Struct(global.11)) = zero[8]
        static static_struct.6: Struct(s.5) = zero[8]
        constant string.0: Array(5,Char) = "zero\\0"
        static struct_ptr.10: Pointer(Struct(s.9)) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_semantic_analysis_cast_struct_to_void() {
    let src = r#"
        struct s {
            int a;
            int b;
        };
        int main(void) {
            struct s x = {1, 2};
            (void)x;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            s.0[0] = 1
            s.0[4] = 2
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_semantic_analysis_incomplete_structs() {
    let src = r#"
        void *malloc(unsigned long size);
        void *calloc(unsigned long nmemb, unsigned long size);
        int puts(char *s);
        int strcmp(char *s1, char *s2);
        struct never_used;
        struct never_used incomplete_fun(struct never_used x);
        int test_block_scope_forward_decl(void) {
            struct s;
            struct s *s_ptr = 0;
            struct s {
                int x;
                int y;
            };
            struct s val = {1, 2};
            s_ptr = &val;
            if (s_ptr->x != 1 || s_ptr->y != 2) {
                return 0;
            }
            return 1;
        }
        struct pair;
        struct pair *make_struct(void);
        int validate_struct(struct pair *ptr);
        int test_file_scope_forward_decl(void) {
            struct pair *my_struct = make_struct();
            return validate_struct(my_struct);
        }
        struct pair {
            long l;
            long m;
        };
        struct pair *make_struct(void) {
            struct pair *retval = malloc(sizeof(struct pair));
            retval->l = 100;
            retval->m = 200;
            return retval;
        }
        int validate_struct(struct pair *ptr) {
            return (ptr->l == 100 && ptr->m == 200);
        }
        struct msg_holder;
        void print_msg(struct msg_holder *param);
        int validate_incomplete_var(void);
        extern struct msg_holder incomplete_var;
        int test_incomplete_var(void) {
            print_msg(&incomplete_var);
            return validate_incomplete_var();
        }
        struct msg_holder {
            char *msg;
        };
        int validate_incomplete_var(void) {
            if (strcmp(incomplete_var.msg, "I'm a struct!")) {
                return 0;
            }
            return 1;
        }
        struct msg_holder incomplete_var = {"I'm a struct!"};
        void print_msg(struct msg_holder *param) {
            puts(param->msg);
        }
        int test_deref_incomplete_var(void) {
            struct undefined_struct;
            struct undefined_struct *ptr = malloc(4);
            return &*ptr == ptr;
        }
        struct opaque_struct;
        struct opaque_struct *use_struct_pointers(struct opaque_struct *param) {
            if (param == 0) {
                puts("empty pointer!");
            }
            return 0;
        }
        int test_use_incomplete_struct_pointers(void) {
            struct opaque_struct *ptr1 = calloc(1, 4);
            struct opaque_struct *ptr2 = calloc(1, 4);
            char *ptr1_bytes = (char *)ptr1;
            if (ptr1_bytes[0] || ptr1_bytes[1]) {
                return 0;
            }
            if (ptr1 == 0 || ptr2 == 0 || ptr1 == ptr2) {
                return 0;
            }
            static int flse = 0;
            struct opaque_struct *ptr3 = flse ? ptr1 : ptr2;
            if (ptr3 != ptr2) {
                return 0;
            }
            if (use_struct_pointers(ptr3)) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_block_scope_forward_decl()) {
                return 2;
            }
            if (!test_file_scope_forward_decl()) {
                return 3;
            }
            if (!test_incomplete_var()) {
                return 4;
            }
            if (!test_deref_incomplete_var()) {
                return 5;
            }
            if (!test_use_incomplete_struct_pointers()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_block_scope_forward_decl() { 
            tmp.0 = sign_extend 0
            s_ptr.9 = tmp.0
            s.8[0] = 1
            s.8[4] = 2
            tmp.1 = &val.11
            s_ptr.9 = tmp.1
            tmp.2 = s_ptr.9 != 1
            if tmp.2 jump or_true_0
            tmp.5 = add_ptr(s_ptr.9, index=1L, scale=1)
            tmp.6 = tmp.5 != 2
            if tmp.6 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if !tmp.4 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_file_scope_forward_decl() { 
            tmp.7 = make_struct()
            my_struct.14 = tmp.7
            tmp.8 = validate_struct(my_struct.14)
            return tmp.8
            return 0
        }
        global function make_struct() { 
            tmp.9 = malloc(16UL)
            tmp.10 = tmp.9
            retval.16 = tmp.10
            tmp.11 = sign_extend 100
            retval.16 = tmp.11
            tmp.12 = add_ptr(retval.16, index=1L, scale=1)
            tmp.13 = sign_extend 200
            tmp.12 = tmp.13
            return retval.16
            return 0
        }
        global function validate_struct(ptr.17) { 
            tmp.15 = sign_extend 100
            tmp.14 = ptr.17 == tmp.15
            if !tmp.14 jump and_false_4
            tmp.18 = add_ptr(ptr.17, index=1L, scale=1)
            tmp.20 = sign_extend 200
            tmp.19 = tmp.18 == tmp.20
            if !tmp.19 jump and_false_4
            tmp.17 = 1
            jump and_end_5
        
          and_false_4:
            tmp.17 = 0
        
          and_end_5:
            return tmp.17
            return 0
        }
        global function test_incomplete_var() { 
            tmp.21 = &incomplete_var
            print_msg(tmp.21)
            tmp.22 = validate_incomplete_var()
            return tmp.22
            return 0
        }
        global function validate_incomplete_var() { 
            tmp.23 = incomplete_var[0]
            tmp.24 = &string.0
            tmp.25 = strcmp(tmp.23, tmp.24)
            if !tmp.25 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function print_msg(param.21) { 
            tmp.26 = puts(param.21)
            return 0
        }
        global function test_deref_incomplete_var() { 
            tmp.27 = sign_extend 4
            tmp.28 = malloc(tmp.27)
            tmp.29 = tmp.28
            ptr.23 = tmp.29
            tmp.30 = ptr.23 == ptr.23
            return tmp.30
            return 0
        }
        global function use_struct_pointers(param.25) { 
            tmp.32 = sign_extend 0
            tmp.31 = param.25 == tmp.32
            if !tmp.31 jump end_if_8
            tmp.33 = &string.1
            tmp.34 = puts(tmp.33)
        
          end_if_8:
            tmp.35 = sign_extend 0
            return tmp.35
            return 0
        }
        global function test_use_incomplete_struct_pointers() { 
            tmp.36 = sign_extend 1
            tmp.37 = sign_extend 4
            tmp.38 = calloc(tmp.36, tmp.37)
            tmp.39 = tmp.38
            ptr1.26 = tmp.39
            tmp.40 = sign_extend 1
            tmp.41 = sign_extend 4
            tmp.42 = calloc(tmp.40, tmp.41)
            tmp.43 = tmp.42
            ptr2.27 = tmp.43
            tmp.44 = ptr1.26
            ptr1_bytes.28 = tmp.44
            tmp.45 = sign_extend 0
            tmp.46 = add_ptr(ptr1_bytes.28, index=tmp.45, scale=1)
            tmp.47 = *tmp.46
            if tmp.47 jump or_true_10
            tmp.50 = sign_extend 1
            tmp.51 = add_ptr(ptr1_bytes.28, index=tmp.50, scale=1)
            tmp.52 = *tmp.51
            if tmp.52 jump or_true_10
            tmp.49 = 0
            jump or_end_11
        
          or_true_10:
            tmp.49 = 1
        
          or_end_11:
            if !tmp.49 jump end_if_12
            return 0
        
          end_if_12:
            tmp.54 = sign_extend 0
            tmp.53 = ptr1.26 == tmp.54
            if tmp.53 jump or_true_14
            tmp.58 = sign_extend 0
            tmp.57 = ptr2.27 == tmp.58
            if tmp.57 jump or_true_14
            tmp.56 = 0
            jump or_end_15
        
          or_true_14:
            tmp.56 = 1
        
          or_end_15:
            if tmp.56 jump or_true_16
            tmp.61 = ptr1.26 == ptr2.27
            if tmp.61 jump or_true_16
            tmp.60 = 0
            jump or_end_17
        
          or_true_16:
            tmp.60 = 1
        
          or_end_17:
            if !tmp.60 jump end_if_18
            return 0
        
          end_if_18:
            if !flse.29 jump else_21
            tmp.62 = ptr1.26
            jump end_if_20
        
          else_21:
            tmp.62 = ptr2.27
        
          end_if_20:
            ptr3.30 = tmp.62
            tmp.63 = ptr3.30 != ptr2.27
            if !tmp.63 jump end_if_22
            return 0
        
          end_if_22:
            tmp.64 = use_struct_pointers(ptr3.30)
            if !tmp.64 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function main() { 
            tmp.65 = test_block_scope_forward_decl()
            tmp.66 = ! tmp.65
            if !tmp.66 jump end_if_26
            return 2
        
          end_if_26:
            tmp.67 = test_file_scope_forward_decl()
            tmp.68 = ! tmp.67
            if !tmp.68 jump end_if_28
            return 3
        
          end_if_28:
            tmp.69 = test_incomplete_var()
            tmp.70 = ! tmp.69
            if !tmp.70 jump end_if_30
            return 4
        
          end_if_30:
            tmp.71 = test_deref_incomplete_var()
            tmp.72 = ! tmp.71
            if !tmp.72 jump end_if_32
            return 5
        
          end_if_32:
            tmp.73 = test_use_incomplete_struct_pointers()
            tmp.74 = ! tmp.73
            if !tmp.74 jump end_if_34
            return 6
        
          end_if_34:
            return 0
            return 0
        }
        static flse.29: Int = 0
        static global incomplete_var: Struct(msg_holder.18) = &I'm a struct!
        constant string.0: Array(14,Char) = "I'm a struct!\\0"
        constant string.1: Array(15,Char) = "empty pointer!\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_semantic_analysis_namespaces() {
    let src = r#"
        int test_shared_member_names(void) {
            struct pair1 {
                int x;
                int y;
            };
            struct pair2 {
                double x;
                char y;
            };
            struct pair1 p1 = {1, 2};
            struct pair2 p2 = {3.0, 4};
            if (p1.x != 1 || p2.x != 3.0) {
                return 0;
            }
            return 1;
        }
        int test_shared_nested_member_names(void) {
            struct pair1 {
                int x;
                int *y;
            };
            struct pair2 {
                void *x;
                double y[4];
            };
            struct pair1 p1 = {3, &(p1.x)};
            struct pair2 p2 = {&p1, {1.0, 2.0, 3.0, 4.0}};
            if (((struct pair1 *)p2.x)->x != 3) {
                return 0;
            }
            return 1;
        }
        int test_same_name_var_member_and_tag(void) {
            struct x {
                int x;
            };
            struct x x = {10};
            if (x.x != 10) {
                return 0;
            }
            return 1;
        }
        int test_same_name_fun_member_and_tag(void) {
            struct f {
                int f;
            };
            int f(void);
            struct f my_struct;
            my_struct.f = f();
            if (my_struct.f != 10) {
                return 0;
            }
            return 1;
        }
        int f(void) {
            return 10;
        }
        int main(void) {
            if (!test_shared_member_names()) {
                return 1;
            }
            if (!test_shared_nested_member_names()) {
                return 2;
            }
            if (!test_same_name_var_member_and_tag()) {
                return 3;
            }
            if (!test_same_name_fun_member_and_tag()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_shared_member_names() { 
            pair1.0[0] = 1
            pair1.0[4] = 2
            pair2.1[0] = 3D
            tmp.0 = truncate 4
            pair2.1[8] = tmp.0
            tmp.1 = p1.2[0]
            tmp.2 = tmp.1 != 1
            if tmp.2 jump or_true_0
            tmp.5 = p2.3[0]
            tmp.6 = tmp.5 != 3D
            if tmp.6 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if !tmp.4 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_shared_nested_member_names() { 
            pair1.4[0] = 3
            tmp.7 = &p1.6
            pair1.4[8] = tmp.7
            tmp.8 = &p1.6
            tmp.9 = tmp.8
            pair2.5[0] = tmp.9
            pair2.5[8] = 1D
            pair2.5[16] = 2D
            pair2.5[24] = 3D
            pair2.5[32] = 4D
            tmp.10 = p2.7[0]
            tmp.11 = tmp.10
            tmp.12 = tmp.11 != 3
            if !tmp.12 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function test_same_name_var_member_and_tag() { 
            x.8[0] = 10
            tmp.13 = x.9[0]
            tmp.14 = tmp.13 != 10
            if !tmp.14 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function test_same_name_fun_member_and_tag() { 
            tmp.15 = f()
            my_struct.11[0] = tmp.15
            tmp.16 = my_struct.11[0]
            tmp.17 = tmp.16 != 10
            if !tmp.17 jump end_if_8
            return 0
        
          end_if_8:
            return 1
            return 0
        }
        global function f() { 
            return 10
            return 0
        }
        global function main() { 
            tmp.18 = test_shared_member_names()
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_10
            return 1
        
          end_if_10:
            tmp.20 = test_shared_nested_member_names()
            tmp.21 = ! tmp.20
            if !tmp.21 jump end_if_12
            return 2
        
          end_if_12:
            tmp.22 = test_same_name_var_member_and_tag()
            tmp.23 = ! tmp.22
            if !tmp.23 jump end_if_14
            return 3
        
          end_if_14:
            tmp.24 = test_same_name_fun_member_and_tag()
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_16
            return 4
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_semantic_analysis_resolve_tags() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *malloc(unsigned long size);
        struct s {
            int a;
        };
        int test_var_declaration(void) {
            struct shadow {
                int x;
            };
            struct shadow outer;
            outer.x = 2;
            {
                struct shadow {
                    int y;
                };
                struct shadow inner;
                inner.y = 3;
                if (outer.x != 2) {
                    return 0;
                }
                if (inner.y != 3) {
                    return 0;
                }
            }
            return 1;
        }
        int test_member_declaration(void) {
            struct s {
                int b;
                struct s *self_ptr;
            };
            struct s my_struct = {123, 0};
            my_struct.self_ptr = &my_struct;
            if (my_struct.self_ptr->b != 123) {
                return 0;
            }
            return 1;
        }
        int test_function_declaration(void) {
            struct s outer_struct = {1};
            {
                struct s {
                    int arr[40];
                };
            }
            struct s *copy_struct(struct s * arg);
            struct s *copy = copy_struct(&outer_struct);
            if (copy->a != outer_struct.a) {
                return 0;
            }
            return 1;
        }
        struct s *copy_struct(struct s *arg) {
            struct s *ptr = malloc(4);
            ptr->a = arg->a;
            return ptr;
        }
        int test_for_loop(void) {
            for (struct s loop_struct = {10}; loop_struct.a > 0;
                 loop_struct.a = loop_struct.a - 1) {
                struct s {
                    double d;
                };
                static struct s loop_body_struct = {0};
                loop_body_struct.d = loop_body_struct.d + 1;
                if (loop_struct.a == 1) {
                    if (loop_body_struct.d != 10.0) {
                        return 0;
                    }
                }
            }
            return 1;
        }
        int test_cast(void) {
            void *ptr = malloc(10);
            if (ptr) {
                struct s {
                    char arr[10];
                };
                ((struct s *)ptr)->arr[2] = 10;
                char byte = ((char *)ptr)[2];
                if (byte != 10) {
                    return 0;
                }
            }
            void *second_ptr = malloc(4);
            ((struct s *)second_ptr)->a = 10;
            char lowest_byte = ((char *)second_ptr)[0];
            if (lowest_byte != 10) {
                return 0;
            }
            return 1;
        }
        int test_sizeof(void) {
            struct s {
                int a;
                int b;
            };
            struct s x;
            {
                struct s {
                    char arr[15];
                };
                if (sizeof x != 8) {
                    return 0;
                };
                if (sizeof(struct s) != 15) {
                    return 0;
                }
            }
            if (sizeof(struct s) != 8) {
                return 0;
            }
            return 1;
        }
        int test_derived_types(void) {
            struct s outer_struct = {1};
            struct s *(*outer_arr)[3] = calloc(3, sizeof(void *));
            struct s {
                int x;
            };
            struct s inner_struct = {2};
            struct s *(*inner_arr)[3] = calloc(3, sizeof(void *));
            outer_arr[0][0] = &outer_struct;
            outer_arr[0][1] = &outer_struct;
            inner_arr[0][0] = &inner_struct;
            inner_arr[0][2] = &inner_struct;
            if (outer_arr[0][0]->a != 1) {
                return 0;
            }
            if (inner_arr[0][0]->x != 2) {
                return 0;
            }
            return 1;
        }
        int test_contentless_tag_noop(void) {
            struct s {
                int x;
                int y;
            };
            struct s;
            struct s var;
            var.x = 10;
            var.y = 11;
            if (var.x != 10 || var.y != 11) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_var_declaration()) {
                return 1;
            }
            if (!test_member_declaration()) {
                return 2;
            }
            if (!test_function_declaration()) {
                return 3;
            }
            if (!test_for_loop()) {
                return 4;
            }
            if (!test_cast()) {
                return 5;
            }
            if (!test_sizeof()) {
                return 6;
            }
            if (!test_derived_types()) {
                return 7;
            }
            if (!test_contentless_tag_noop()) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_var_declaration() { 
            outer.5[0] = 2
            inner.7[0] = 3
            tmp.0 = outer.5[0]
            tmp.1 = tmp.0 != 2
            if !tmp.1 jump end_if_0
            return 0
        
          end_if_0:
            tmp.2 = inner.7[0]
            tmp.3 = tmp.2 != 3
            if !tmp.3 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_member_declaration() { 
            s.8[0] = 123
            tmp.4 = sign_extend 0
            s.8[8] = tmp.4
            tmp.5 = &my_struct.9
            my_struct.9[1] = tmp.5
            tmp.6 = my_struct.9[1]
            tmp.7 = tmp.6 != 123
            if !tmp.7 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function test_function_declaration() { 
            s.3[0] = 1
            tmp.8 = &outer_struct.10
            tmp.9 = copy_struct(tmp.8)
            copy.13 = tmp.9
            tmp.11 = outer_struct.10[0]
            tmp.10 = copy.13 != tmp.11
            if !tmp.10 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function copy_struct(arg.14) { 
            tmp.12 = sign_extend 4
            tmp.13 = malloc(tmp.12)
            tmp.14 = tmp.13
            ptr.15 = tmp.14
            ptr.15 = arg.14
            return ptr.15
            return 0
        }
        global function test_for_loop() { 
            s.3[0] = 10
        
          start_loop_0:
            tmp.15 = loop_struct.16[0]
            tmp.16 = tmp.15 > 0
            if !tmp.16 jump break_loop_0
            tmp.17 = loop_body_struct.18[0]
            tmp.19 = int_to_double 1
            tmp.18 = tmp.17 + tmp.19
            loop_body_struct.18[0] = tmp.18
            tmp.20 = loop_struct.16[0]
            tmp.21 = tmp.20 == 1
            if !tmp.21 jump end_if_8
            tmp.22 = loop_body_struct.18[0]
            tmp.23 = tmp.22 != 10D
            if !tmp.23 jump end_if_10
            return 0
        
          end_if_10:
        
          end_if_8:
        
          continue_loop_0:
            tmp.24 = loop_struct.16[0]
            tmp.25 = tmp.24 - 1
            loop_struct.16[0] = tmp.25
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
        global function test_cast() { 
            tmp.26 = sign_extend 10
            tmp.27 = malloc(tmp.26)
            ptr.19 = tmp.27
            if !ptr.19 jump end_if_12
            tmp.28 = ptr.19
            tmp.29 = &tmp.28
            tmp.30 = sign_extend 2
            tmp.31 = add_ptr(tmp.29, index=tmp.30, scale=1)
            tmp.32 = truncate 10
            *tmp.31 = tmp.32
            tmp.33 = ptr.19
            tmp.34 = sign_extend 2
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=1)
            tmp.36 = *tmp.35
            byte.21 = tmp.36
            tmp.37 = sign_extend byte.21
            tmp.38 = tmp.37 != 10
            if !tmp.38 jump end_if_14
            return 0
        
          end_if_14:
        
          end_if_12:
            tmp.39 = sign_extend 4
            tmp.40 = malloc(tmp.39)
            second_ptr.22 = tmp.40
            tmp.41 = second_ptr.22
            tmp.41 = 10
            tmp.42 = second_ptr.22
            tmp.43 = sign_extend 0
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=1)
            tmp.45 = *tmp.44
            lowest_byte.23 = tmp.45
            tmp.46 = sign_extend lowest_byte.23
            tmp.47 = tmp.46 != 10
            if !tmp.47 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function test_sizeof() { 
            tmp.49 = sign_extend 8
            tmp.48 = 8UL != tmp.49
            if !tmp.48 jump end_if_18
            return 0
        
          end_if_18:
            tmp.51 = sign_extend 15
            tmp.50 = 15UL != tmp.51
            if !tmp.50 jump end_if_20
            return 0
        
          end_if_20:
            tmp.53 = sign_extend 8
            tmp.52 = 8UL != tmp.53
            if !tmp.52 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function test_derived_types() { 
            s.3[0] = 1
            tmp.54 = sign_extend 3
            tmp.55 = calloc(tmp.54, 8UL)
            tmp.56 = tmp.55
            outer_arr.28 = tmp.56
            s.29[0] = 2
            tmp.57 = sign_extend 3
            tmp.58 = calloc(tmp.57, 8UL)
            tmp.59 = tmp.58
            inner_arr.31 = tmp.59
            tmp.60 = sign_extend 0
            tmp.61 = add_ptr(outer_arr.28, index=tmp.60, scale=24)
            tmp.62 = sign_extend 0
            tmp.63 = add_ptr(tmp.61, index=tmp.62, scale=8)
            tmp.64 = &outer_struct.27
            *tmp.63 = tmp.64
            tmp.65 = sign_extend 0
            tmp.66 = add_ptr(outer_arr.28, index=tmp.65, scale=24)
            tmp.67 = sign_extend 1
            tmp.68 = add_ptr(tmp.66, index=tmp.67, scale=8)
            tmp.69 = &outer_struct.27
            *tmp.68 = tmp.69
            tmp.70 = sign_extend 0
            tmp.71 = add_ptr(inner_arr.31, index=tmp.70, scale=24)
            tmp.72 = sign_extend 0
            tmp.73 = add_ptr(tmp.71, index=tmp.72, scale=8)
            tmp.74 = &inner_struct.30
            *tmp.73 = tmp.74
            tmp.75 = sign_extend 0
            tmp.76 = add_ptr(inner_arr.31, index=tmp.75, scale=24)
            tmp.77 = sign_extend 2
            tmp.78 = add_ptr(tmp.76, index=tmp.77, scale=8)
            tmp.79 = &inner_struct.30
            *tmp.78 = tmp.79
            tmp.80 = sign_extend 0
            tmp.81 = add_ptr(outer_arr.28, index=tmp.80, scale=24)
            tmp.82 = sign_extend 0
            tmp.83 = add_ptr(tmp.81, index=tmp.82, scale=8)
            tmp.84 = *tmp.83
            tmp.85 = tmp.84 != 1
            if !tmp.85 jump end_if_24
            return 0
        
          end_if_24:
            tmp.86 = sign_extend 0
            tmp.87 = add_ptr(inner_arr.31, index=tmp.86, scale=24)
            tmp.88 = sign_extend 0
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=8)
            tmp.90 = *tmp.89
            tmp.91 = tmp.90 != 2
            if !tmp.91 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function test_contentless_tag_noop() { 
            var.34[0] = 10
            var.34[1] = 11
            tmp.92 = var.34[0]
            tmp.93 = tmp.92 != 10
            if tmp.93 jump or_true_28
            tmp.96 = var.34[1]
            tmp.97 = tmp.96 != 11
            if tmp.97 jump or_true_28
            tmp.95 = 0
            jump or_end_29
        
          or_true_28:
            tmp.95 = 1
        
          or_end_29:
            if !tmp.95 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function main() { 
            tmp.98 = test_var_declaration()
            tmp.99 = ! tmp.98
            if !tmp.99 jump end_if_32
            return 1
        
          end_if_32:
            tmp.100 = test_member_declaration()
            tmp.101 = ! tmp.100
            if !tmp.101 jump end_if_34
            return 2
        
          end_if_34:
            tmp.102 = test_function_declaration()
            tmp.103 = ! tmp.102
            if !tmp.103 jump end_if_36
            return 3
        
          end_if_36:
            tmp.104 = test_for_loop()
            tmp.105 = ! tmp.104
            if !tmp.105 jump end_if_38
            return 4
        
          end_if_38:
            tmp.106 = test_cast()
            tmp.107 = ! tmp.106
            if !tmp.107 jump end_if_40
            return 5
        
          end_if_40:
            tmp.108 = test_sizeof()
            tmp.109 = ! tmp.108
            if !tmp.109 jump end_if_42
            return 6
        
          end_if_42:
            tmp.110 = test_derived_types()
            tmp.111 = ! tmp.110
            if !tmp.111 jump end_if_44
            return 7
        
          end_if_44:
            tmp.112 = test_contentless_tag_noop()
            tmp.113 = ! tmp.112
            if !tmp.113 jump end_if_46
            return 8
        
          end_if_46:
            return 0
            return 0
        }
        static loop_body_struct.18: Struct(s.17) = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_size_and_offset_calculations_member_comparisons() {
    let src = r#"
        struct three_ints {
            int a;
            int b;
            int c;
        };
        void* calloc(unsigned long nmem, unsigned long size);
        int main(void) {
            struct three_ints* my_struct = calloc(1, sizeof(struct three_ints));
            if ((void*)my_struct != &my_struct->a) {
                return 1;
            }
            if (!((int *)my_struct <= &my_struct->a)) {
                return 2;
            }
            if (&my_struct->c <= &my_struct->a) {
                return 3;
            }
            if (&my_struct->b > &my_struct->c) {
                return 4;
            }
            if (!(&my_struct->b > &my_struct->a)) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 1
            tmp.1 = calloc(tmp.0, 12UL)
            tmp.2 = tmp.1
            my_struct.3 = tmp.2
            tmp.3 = my_struct.3
            tmp.5 = &my_struct.3
            tmp.6 = tmp.5
            tmp.4 = tmp.3 != tmp.6
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = my_struct.3
            tmp.9 = &my_struct.3
            tmp.8 = tmp.7 <= tmp.9
            tmp.10 = ! tmp.8
            if !tmp.10 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = add_ptr(my_struct.3, index=2L, scale=1)
            tmp.12 = &tmp.11
            tmp.14 = &my_struct.3
            tmp.13 = tmp.12 <= tmp.14
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            tmp.15 = add_ptr(my_struct.3, index=1L, scale=1)
            tmp.16 = &tmp.15
            tmp.18 = add_ptr(my_struct.3, index=2L, scale=1)
            tmp.19 = &tmp.18
            tmp.17 = tmp.16 > tmp.19
            if !tmp.17 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = add_ptr(my_struct.3, index=1L, scale=1)
            tmp.21 = &tmp.20
            tmp.23 = &my_struct.3
            tmp.22 = tmp.21 > tmp.23
            tmp.24 = ! tmp.22
            if !tmp.24 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_size_and_offset_calculations_member_offsets() {
    let src = r#"
        struct eight_bytes {
            int i;
            char c;
        };
        struct two_bytes {
            char arr[2];
        };
        struct three_bytes {
            char arr[3];
        };
        struct sixteen_bytes {
            struct eight_bytes eight;
            struct two_bytes two;
            struct three_bytes three;
        };
        struct seven_bytes {
            struct two_bytes two;
            struct three_bytes three;
            struct two_bytes two2;
        };
        struct twentyfour_bytes {
            struct seven_bytes seven;
            struct sixteen_bytes sixteen;
        };
        struct twenty_bytes {
            struct sixteen_bytes sixteen;
            struct two_bytes two;
        };
        struct wonky {
            char arr[19];
        };
        struct internal_padding {
            char c;
            double d;
        };
        struct contains_struct_array {
            char c;
            struct eight_bytes struct_array[3];
        };
        
        void *malloc(unsigned long size);
        int test_eightbytes(void) {
            struct eight_bytes s;
            unsigned long start_addr = (unsigned long)&s;
            unsigned long i_addr = (unsigned long)&s.i;
            unsigned long c_addr = (unsigned long)&s.c;
            unsigned long end_addr = (unsigned long)(&s + 1);
            if (start_addr % 4 != 0) {
                return 0;
            }
            if (start_addr != i_addr) {
                return 0;
            }
            if (c_addr - start_addr != 4) {
                return 0;
            }
            if (end_addr - start_addr != 8) {
                return 0;
            }
            return 1;
        }
        int test_internal_padding(void) {
            struct internal_padding *s_ptr = malloc(sizeof(struct internal_padding));
            unsigned long start_addr = (unsigned long)s_ptr;
            unsigned long c_addr = (unsigned long)&s_ptr->c;
            unsigned long d_addr = (unsigned long)&s_ptr->d;
            unsigned long end_ptr = (unsigned long)(s_ptr + 1);
            if (start_addr % 8 != 0) {
                return 0;
            }
            if (start_addr != c_addr) {
                return 0;
            }
            if (d_addr - c_addr != 8) {
                return 0;
            }
            if (end_ptr - start_addr != 16) {
                return 0;
            }
            return 1;
        }
        int test_three_bytes(void) {
            static struct three_bytes s;
            unsigned long start_addr = (unsigned long)&s;
            unsigned long arr_addr = (unsigned long)&s.arr;
            unsigned long arr0_addr = (unsigned long)&s.arr[0];
            unsigned long arr1_addr = (unsigned long)&s.arr[1];
            unsigned long arr1_addr_alt = (unsigned long)(s.arr + 1);
            unsigned long arr2_addr = (unsigned long)&s.arr[2];
            unsigned long arr_end = (unsigned long)(&s.arr + 1);
            unsigned long struct_end = (unsigned long)(&s + 1);
            if (start_addr != arr_addr) {
                return 0;
            }
            if (start_addr != arr0_addr) {
                return 0;
            }
            if (arr1_addr - start_addr != 1) {
                return 0;
            }
            if (arr1_addr != arr1_addr_alt) {
                return 0;
            }
            if (arr2_addr - start_addr != 2) {
                return 0;
            }
            if (arr_end - start_addr != 3) {
                return 0;
            }
            if (struct_end - start_addr != 3) {
                return 0;
            }
            return 1;
        }
        int test_sixteen_bytes(void) {
            static struct sixteen_bytes s;
            struct sixteen_bytes *s_ptr = &s;
            unsigned long start_addr = (unsigned long)s_ptr;
            unsigned long eight_addr = (unsigned long)&s_ptr->eight;
            unsigned long eight_i_addr = (unsigned long)&s_ptr->eight.i;
            unsigned long eight_c_addr = (unsigned long)&s_ptr->eight.c;
            unsigned long two = (unsigned long)&s_ptr->two;
            unsigned long two_arr = (unsigned long)s_ptr->two.arr;
            unsigned long two_arr0 = (unsigned long)&s_ptr->two.arr[0];
            unsigned long two_arr1 = (unsigned long)&s_ptr->two.arr[1];
            unsigned long two_arr_end = (unsigned long)(&s_ptr->two.arr + 1);
            unsigned long two_end = (unsigned long)(&s_ptr->two + 1);
            unsigned long three = (unsigned long)&s_ptr->three;
            unsigned long three_end = (unsigned long)(&s_ptr->three + 1);
            unsigned long struct_end = (unsigned long)(s_ptr + 1);
            if (start_addr % 4 != 0) {
                return 0;
            }
            if (start_addr != eight_addr) {
                return 0;
            }
            if (start_addr != eight_i_addr) {
                return 0;
            }
            if (eight_c_addr - start_addr != 4) {
                return 0;
            }
            if (two - start_addr != 8) {
                return 0;
            }
            if (two_arr - start_addr != 8) {
                return 0;
            }
            if (two_arr0 - start_addr != 8) {
                return 0;
            }
            if (two_arr1 - start_addr != 9) {
                return 0;
            }
            if (two_arr_end - start_addr != 10) {
                return 0;
            }
            if (two_arr_end != two_end) {
                return 0;
            }
            if (three - start_addr != 10) {
                return 0;
            }
            if (three_end - start_addr != 13) {
                return 0;
            }
            if (struct_end - start_addr != 16) {
                return 0;
            }
            unsigned long eight_i_addr_alt = (unsigned long)&s.eight.i;
            unsigned long eight_c_addr_alt = (unsigned long)&s.eight.c;
            unsigned long two_arr_alt = (unsigned long)s.two.arr;
            unsigned long two_arr1_alt = (unsigned long)&s.two.arr[1];
            unsigned long three_alt = (unsigned long)&s.three;
            if (eight_i_addr_alt != eight_i_addr) {
                return 0;
            }
            if (eight_c_addr_alt != eight_c_addr) {
                return 0;
            }
            if (two_arr_alt != two_arr) {
                return 0;
            }
            if (two_arr1_alt != two_arr1) {
                return 0;
            }
            if (three_alt != three) {
                return 0;
            }
            return 1;
        }
        int test_wonky_array(void) {
            struct wonky wonky_array[5];
            unsigned long array_start = (unsigned long)wonky_array;
            unsigned long elem3 = (unsigned long)(wonky_array + 3);
            unsigned long elem3_arr = (unsigned long)wonky_array[3].arr;
            unsigned long elem2_arr2 = (unsigned long)&wonky_array[2].arr[2];
            unsigned long elem2_arr_end = (unsigned long)(wonky_array[2].arr + 19);
            unsigned long elem4_arr_end = (unsigned long)(wonky_array[4].arr + 19);
            unsigned long array_end = (unsigned long)(wonky_array + 5);
            if (elem3 - array_start != 19 * 3) {
                return 0;
            }
            if (elem3_arr != elem3) {
                return 0;
            }
            if (elem2_arr2 - array_start != 19 * 2 + 2) {
                return 0;
            }
            if (elem2_arr_end != elem3) {
                return 0;
            }
            if (elem4_arr_end != array_end) {
                return 0;
            }
            return 1;
        }
        int test_contains_struct_array_array(void) {
            struct contains_struct_array arr[3];
            unsigned long array_start = (unsigned long)arr;
            unsigned long first_scalar_elem = (unsigned long)(&arr[0].c);
            unsigned long outer0_inner0_i = (unsigned long)(&arr[0].struct_array->i);
            unsigned long outer0_inner0_c = (unsigned long)(&arr->struct_array->c);
            unsigned long outer0_end = (unsigned long)(arr->struct_array + 3);
            unsigned long outer1 = (unsigned long)(&arr[1]);
            unsigned long outer1_arr = (unsigned long)(arr[1].struct_array);
            unsigned long outer1_inner1_i =
                (unsigned long)&(((arr + 1)->struct_array + 1)->i);
            unsigned long outer2_inner0_c =
                (unsigned long)&((arr + 2)->struct_array->c);
            if (array_start % 4 != 0) {
                return 0;
            }
            if (first_scalar_elem != array_start) {
                return 0;
            }
            if (outer0_inner0_i - array_start != 4) {
                return 0;
            }
            if (outer0_inner0_c - array_start != 8) {
                return 0;
            }
            if (outer0_end != outer1) {
                return 0;
            }
            if (outer1_arr - array_start != 32) {
                return 0;
            }
            if (outer1_arr - outer1 != 4) {
                return 0;
            }
            if (outer1_inner1_i - array_start != 40) {
                return 0;
            }
            if (outer2_inner0_c - array_start != 64) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_eightbytes()) {
                return 1;
            }
            if (!test_internal_padding()) {
                return 2;
            }
            if (!test_three_bytes()) {
                return 3;
            }
            if (!test_sixteen_bytes()) {
                return 4;
            }
            if (!test_wonky_array()) {
                return 5;
            }
            if (!test_contains_struct_array_array()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_eightbytes() { 
            tmp.0 = &s.11
            tmp.1 = tmp.0
            start_addr.12 = tmp.1
            tmp.2 = &s.11
            tmp.3 = tmp.2
            i_addr.13 = tmp.3
            tmp.4 = &s.11
            tmp.4 = add_ptr(tmp.4, index=1L, scale=1)
            tmp.5 = tmp.4
            c_addr.14 = tmp.5
            tmp.6 = &s.11
            tmp.8 = sign_extend 1
            tmp.7 = add_ptr(tmp.6, index=tmp.8, scale=8)
            tmp.9 = tmp.7
            end_addr.15 = tmp.9
            tmp.11 = sign_extend 4
            tmp.10 = start_addr.12 % tmp.11
            tmp.13 = sign_extend 0
            tmp.12 = tmp.10 != tmp.13
            if !tmp.12 jump end_if_0
            return 0
        
          end_if_0:
            tmp.14 = start_addr.12 != i_addr.13
            if !tmp.14 jump end_if_2
            return 0
        
          end_if_2:
            tmp.15 = c_addr.14 - start_addr.12
            tmp.17 = sign_extend 4
            tmp.16 = tmp.15 != tmp.17
            if !tmp.16 jump end_if_4
            return 0
        
          end_if_4:
            tmp.18 = end_addr.15 - start_addr.12
            tmp.20 = sign_extend 8
            tmp.19 = tmp.18 != tmp.20
            if !tmp.19 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function test_internal_padding() { 
            tmp.21 = malloc(16UL)
            tmp.22 = tmp.21
            s_ptr.16 = tmp.22
            tmp.23 = s_ptr.16
            start_addr.17 = tmp.23
            tmp.24 = &s_ptr.16
            tmp.25 = tmp.24
            c_addr.18 = tmp.25
            tmp.26 = add_ptr(s_ptr.16, index=1L, scale=1)
            tmp.27 = &tmp.26
            tmp.28 = tmp.27
            d_addr.19 = tmp.28
            tmp.30 = sign_extend 1
            tmp.29 = add_ptr(s_ptr.16, index=tmp.30, scale=16)
            tmp.31 = tmp.29
            end_ptr.20 = tmp.31
            tmp.33 = sign_extend 8
            tmp.32 = start_addr.17 % tmp.33
            tmp.35 = sign_extend 0
            tmp.34 = tmp.32 != tmp.35
            if !tmp.34 jump end_if_8
            return 0
        
          end_if_8:
            tmp.36 = start_addr.17 != c_addr.18
            if !tmp.36 jump end_if_10
            return 0
        
          end_if_10:
            tmp.37 = d_addr.19 - c_addr.18
            tmp.39 = sign_extend 8
            tmp.38 = tmp.37 != tmp.39
            if !tmp.38 jump end_if_12
            return 0
        
          end_if_12:
            tmp.40 = end_ptr.20 - start_addr.17
            tmp.42 = sign_extend 16
            tmp.41 = tmp.40 != tmp.42
            if !tmp.41 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_three_bytes() { 
            tmp.43 = &s.21
            tmp.44 = tmp.43
            start_addr.22 = tmp.44
            tmp.45 = &s.21
            tmp.46 = tmp.45
            arr_addr.23 = tmp.46
            tmp.47 = s.21[0]
            tmp.48 = &tmp.47
            tmp.49 = sign_extend 0
            tmp.50 = add_ptr(tmp.48, index=tmp.49, scale=1)
            tmp.51 = tmp.50
            arr0_addr.24 = tmp.51
            tmp.52 = s.21[0]
            tmp.53 = &tmp.52
            tmp.54 = sign_extend 1
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=1)
            tmp.56 = tmp.55
            arr1_addr.25 = tmp.56
            tmp.57 = s.21[0]
            tmp.58 = &tmp.57
            tmp.60 = sign_extend 1
            tmp.59 = add_ptr(tmp.58, index=tmp.60, scale=1)
            tmp.61 = tmp.59
            arr1_addr_alt.26 = tmp.61
            tmp.62 = s.21[0]
            tmp.63 = &tmp.62
            tmp.64 = sign_extend 2
            tmp.65 = add_ptr(tmp.63, index=tmp.64, scale=1)
            tmp.66 = tmp.65
            arr2_addr.27 = tmp.66
            tmp.67 = &s.21
            tmp.69 = sign_extend 1
            tmp.68 = add_ptr(tmp.67, index=tmp.69, scale=3)
            tmp.70 = tmp.68
            arr_end.28 = tmp.70
            tmp.71 = &s.21
            tmp.73 = sign_extend 1
            tmp.72 = add_ptr(tmp.71, index=tmp.73, scale=3)
            tmp.74 = tmp.72
            struct_end.29 = tmp.74
            tmp.75 = start_addr.22 != arr_addr.23
            if !tmp.75 jump end_if_16
            return 0
        
          end_if_16:
            tmp.76 = start_addr.22 != arr0_addr.24
            if !tmp.76 jump end_if_18
            return 0
        
          end_if_18:
            tmp.77 = arr1_addr.25 - start_addr.22
            tmp.79 = sign_extend 1
            tmp.78 = tmp.77 != tmp.79
            if !tmp.78 jump end_if_20
            return 0
        
          end_if_20:
            tmp.80 = arr1_addr.25 != arr1_addr_alt.26
            if !tmp.80 jump end_if_22
            return 0
        
          end_if_22:
            tmp.81 = arr2_addr.27 - start_addr.22
            tmp.83 = sign_extend 2
            tmp.82 = tmp.81 != tmp.83
            if !tmp.82 jump end_if_24
            return 0
        
          end_if_24:
            tmp.84 = arr_end.28 - start_addr.22
            tmp.86 = sign_extend 3
            tmp.85 = tmp.84 != tmp.86
            if !tmp.85 jump end_if_26
            return 0
        
          end_if_26:
            tmp.87 = struct_end.29 - start_addr.22
            tmp.89 = sign_extend 3
            tmp.88 = tmp.87 != tmp.89
            if !tmp.88 jump end_if_28
            return 0
        
          end_if_28:
            return 1
            return 0
        }
        global function test_sixteen_bytes() { 
            tmp.90 = &s.30
            s_ptr.31 = tmp.90
            tmp.91 = s_ptr.31
            start_addr.32 = tmp.91
            tmp.92 = &s_ptr.31
            tmp.93 = tmp.92
            eight_addr.33 = tmp.93
            tmp.94 = &s_ptr.31
            tmp.95 = tmp.94
            eight_i_addr.34 = tmp.95
            tmp.96 = &s_ptr.31
            tmp.96 = add_ptr(tmp.96, index=1L, scale=1)
            tmp.97 = tmp.96
            eight_c_addr.35 = tmp.97
            tmp.98 = add_ptr(s_ptr.31, index=1L, scale=1)
            tmp.99 = &tmp.98
            tmp.100 = tmp.99
            two.36 = tmp.100
            tmp.101 = add_ptr(s_ptr.31, index=1L, scale=1)
            tmp.102 = tmp.101[0]
            tmp.103 = &tmp.102
            tmp.104 = tmp.103
            two_arr.37 = tmp.104
            tmp.105 = add_ptr(s_ptr.31, index=1L, scale=1)
            tmp.106 = tmp.105[0]
            tmp.107 = &tmp.106
            tmp.108 = sign_extend 0
            tmp.109 = add_ptr(tmp.107, index=tmp.108, scale=1)
            tmp.110 = tmp.109
            two_arr0.38 = tmp.110
            tmp.111 = add_ptr(s_ptr.31, index=1L, scale=1)
            tmp.112 = tmp.111[0]
            tmp.113 = &tmp.112
            tmp.114 = sign_extend 1
            tmp.115 = add_ptr(tmp.113, index=tmp.114, scale=1)
            tmp.116 = tmp.115
            two_arr1.39 = tmp.116
            tmp.117 = add_ptr(s_ptr.31, index=1L, scale=1)
            tmp.118 = &tmp.117
            tmp.120 = sign_extend 1
            tmp.119 = add_ptr(tmp.118, index=tmp.120, scale=2)
            tmp.121 = tmp.119
            two_arr_end.40 = tmp.121
            tmp.122 = add_ptr(s_ptr.31, index=1L, scale=1)
            tmp.123 = &tmp.122
            tmp.125 = sign_extend 1
            tmp.124 = add_ptr(tmp.123, index=tmp.125, scale=2)
            tmp.126 = tmp.124
            two_end.41 = tmp.126
            tmp.127 = add_ptr(s_ptr.31, index=2L, scale=1)
            tmp.128 = &tmp.127
            tmp.129 = tmp.128
            three.42 = tmp.129
            tmp.130 = add_ptr(s_ptr.31, index=2L, scale=1)
            tmp.131 = &tmp.130
            tmp.133 = sign_extend 1
            tmp.132 = add_ptr(tmp.131, index=tmp.133, scale=3)
            tmp.134 = tmp.132
            three_end.43 = tmp.134
            tmp.136 = sign_extend 1
            tmp.135 = add_ptr(s_ptr.31, index=tmp.136, scale=16)
            tmp.137 = tmp.135
            struct_end.44 = tmp.137
            tmp.139 = sign_extend 4
            tmp.138 = start_addr.32 % tmp.139
            tmp.141 = sign_extend 0
            tmp.140 = tmp.138 != tmp.141
            if !tmp.140 jump end_if_30
            return 0
        
          end_if_30:
            tmp.142 = start_addr.32 != eight_addr.33
            if !tmp.142 jump end_if_32
            return 0
        
          end_if_32:
            tmp.143 = start_addr.32 != eight_i_addr.34
            if !tmp.143 jump end_if_34
            return 0
        
          end_if_34:
            tmp.144 = eight_c_addr.35 - start_addr.32
            tmp.146 = sign_extend 4
            tmp.145 = tmp.144 != tmp.146
            if !tmp.145 jump end_if_36
            return 0
        
          end_if_36:
            tmp.147 = two.36 - start_addr.32
            tmp.149 = sign_extend 8
            tmp.148 = tmp.147 != tmp.149
            if !tmp.148 jump end_if_38
            return 0
        
          end_if_38:
            tmp.150 = two_arr.37 - start_addr.32
            tmp.152 = sign_extend 8
            tmp.151 = tmp.150 != tmp.152
            if !tmp.151 jump end_if_40
            return 0
        
          end_if_40:
            tmp.153 = two_arr0.38 - start_addr.32
            tmp.155 = sign_extend 8
            tmp.154 = tmp.153 != tmp.155
            if !tmp.154 jump end_if_42
            return 0
        
          end_if_42:
            tmp.156 = two_arr1.39 - start_addr.32
            tmp.158 = sign_extend 9
            tmp.157 = tmp.156 != tmp.158
            if !tmp.157 jump end_if_44
            return 0
        
          end_if_44:
            tmp.159 = two_arr_end.40 - start_addr.32
            tmp.161 = sign_extend 10
            tmp.160 = tmp.159 != tmp.161
            if !tmp.160 jump end_if_46
            return 0
        
          end_if_46:
            tmp.162 = two_arr_end.40 != two_end.41
            if !tmp.162 jump end_if_48
            return 0
        
          end_if_48:
            tmp.163 = three.42 - start_addr.32
            tmp.165 = sign_extend 10
            tmp.164 = tmp.163 != tmp.165
            if !tmp.164 jump end_if_50
            return 0
        
          end_if_50:
            tmp.166 = three_end.43 - start_addr.32
            tmp.168 = sign_extend 13
            tmp.167 = tmp.166 != tmp.168
            if !tmp.167 jump end_if_52
            return 0
        
          end_if_52:
            tmp.169 = struct_end.44 - start_addr.32
            tmp.171 = sign_extend 16
            tmp.170 = tmp.169 != tmp.171
            if !tmp.170 jump end_if_54
            return 0
        
          end_if_54:
            tmp.172 = &s.30
            tmp.173 = tmp.172
            eight_i_addr_alt.45 = tmp.173
            tmp.174 = &s.30
            tmp.174 = add_ptr(tmp.174, index=1L, scale=1)
            tmp.175 = tmp.174
            eight_c_addr_alt.46 = tmp.175
            tmp.176 = s.30[1]
            tmp.177 = &tmp.176
            tmp.178 = tmp.177
            two_arr_alt.47 = tmp.178
            tmp.179 = s.30[1]
            tmp.180 = &tmp.179
            tmp.181 = sign_extend 1
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=1)
            tmp.183 = tmp.182
            two_arr1_alt.48 = tmp.183
            tmp.184 = &s.30
            tmp.184 = add_ptr(tmp.184, index=2L, scale=1)
            tmp.185 = tmp.184
            three_alt.49 = tmp.185
            tmp.186 = eight_i_addr_alt.45 != eight_i_addr.34
            if !tmp.186 jump end_if_56
            return 0
        
          end_if_56:
            tmp.187 = eight_c_addr_alt.46 != eight_c_addr.35
            if !tmp.187 jump end_if_58
            return 0
        
          end_if_58:
            tmp.188 = two_arr_alt.47 != two_arr.37
            if !tmp.188 jump end_if_60
            return 0
        
          end_if_60:
            tmp.189 = two_arr1_alt.48 != two_arr1.39
            if !tmp.189 jump end_if_62
            return 0
        
          end_if_62:
            tmp.190 = three_alt.49 != three.42
            if !tmp.190 jump end_if_64
            return 0
        
          end_if_64:
            return 1
            return 0
        }
        global function test_wonky_array() { 
            tmp.191 = &wonky_array.50
            tmp.192 = tmp.191
            array_start.51 = tmp.192
            tmp.193 = &wonky_array.50
            tmp.195 = sign_extend 3
            tmp.194 = add_ptr(tmp.193, index=tmp.195, scale=19)
            tmp.196 = tmp.194
            elem3.52 = tmp.196
            tmp.197 = &wonky_array.50
            tmp.198 = sign_extend 3
            tmp.199 = add_ptr(tmp.197, index=tmp.198, scale=19)
            tmp.200 = tmp.199
            elem3_arr.53 = tmp.200
            tmp.201 = &wonky_array.50
            tmp.202 = sign_extend 2
            tmp.203 = add_ptr(tmp.201, index=tmp.202, scale=19)
            tmp.204 = sign_extend 2
            tmp.205 = add_ptr(tmp.203, index=tmp.204, scale=1)
            tmp.206 = tmp.205
            elem2_arr2.54 = tmp.206
            tmp.207 = &wonky_array.50
            tmp.208 = sign_extend 2
            tmp.209 = add_ptr(tmp.207, index=tmp.208, scale=19)
            tmp.211 = sign_extend 19
            tmp.210 = add_ptr(tmp.209, index=tmp.211, scale=1)
            tmp.212 = tmp.210
            elem2_arr_end.55 = tmp.212
            tmp.213 = &wonky_array.50
            tmp.214 = sign_extend 4
            tmp.215 = add_ptr(tmp.213, index=tmp.214, scale=19)
            tmp.217 = sign_extend 19
            tmp.216 = add_ptr(tmp.215, index=tmp.217, scale=1)
            tmp.218 = tmp.216
            elem4_arr_end.56 = tmp.218
            tmp.219 = &wonky_array.50
            tmp.221 = sign_extend 5
            tmp.220 = add_ptr(tmp.219, index=tmp.221, scale=19)
            tmp.222 = tmp.220
            array_end.57 = tmp.222
            tmp.223 = elem3.52 - array_start.51
            tmp.225 = 19 * 3
            tmp.226 = sign_extend tmp.225
            tmp.224 = tmp.223 != tmp.226
            if !tmp.224 jump end_if_66
            return 0
        
          end_if_66:
            tmp.227 = elem3_arr.53 != elem3.52
            if !tmp.227 jump end_if_68
            return 0
        
          end_if_68:
            tmp.228 = elem2_arr2.54 - array_start.51
            tmp.230 = 19 * 2
            tmp.231 = tmp.230 + 2
            tmp.232 = sign_extend tmp.231
            tmp.229 = tmp.228 != tmp.232
            if !tmp.229 jump end_if_70
            return 0
        
          end_if_70:
            tmp.233 = elem2_arr_end.55 != elem3.52
            if !tmp.233 jump end_if_72
            return 0
        
          end_if_72:
            tmp.234 = elem4_arr_end.56 != array_end.57
            if !tmp.234 jump end_if_74
            return 0
        
          end_if_74:
            return 1
            return 0
        }
        global function test_contains_struct_array_array() { 
            tmp.235 = &arr.58
            tmp.236 = tmp.235
            array_start.59 = tmp.236
            tmp.237 = &arr.58
            tmp.238 = sign_extend 0
            tmp.239 = add_ptr(tmp.237, index=tmp.238, scale=28)
            tmp.240 = tmp.239
            first_scalar_elem.60 = tmp.240
            tmp.241 = &arr.58
            tmp.242 = sign_extend 0
            tmp.243 = add_ptr(tmp.241, index=tmp.242, scale=28)
            tmp.244 = add_ptr(tmp.243, index=1L, scale=1)
            tmp.245 = &tmp.244
            tmp.246 = tmp.245
            outer0_inner0_i.61 = tmp.246
            tmp.247 = &arr.58
            tmp.248 = add_ptr(tmp.247, index=1L, scale=1)
            tmp.249 = &tmp.248
            tmp.250 = add_ptr(tmp.249, index=1L, scale=1)
            tmp.251 = &tmp.250
            tmp.252 = tmp.251
            outer0_inner0_c.62 = tmp.252
            tmp.253 = &arr.58
            tmp.254 = add_ptr(tmp.253, index=1L, scale=1)
            tmp.255 = &tmp.254
            tmp.257 = sign_extend 3
            tmp.256 = add_ptr(tmp.255, index=tmp.257, scale=8)
            tmp.258 = tmp.256
            outer0_end.63 = tmp.258
            tmp.259 = &arr.58
            tmp.260 = sign_extend 1
            tmp.261 = add_ptr(tmp.259, index=tmp.260, scale=28)
            tmp.262 = tmp.261
            outer1.64 = tmp.262
            tmp.263 = &arr.58
            tmp.264 = sign_extend 1
            tmp.265 = add_ptr(tmp.263, index=tmp.264, scale=28)
            tmp.266 = add_ptr(tmp.265, index=1L, scale=1)
            tmp.267 = tmp.266
            outer1_arr.65 = tmp.267
            tmp.268 = &arr.58
            tmp.270 = sign_extend 1
            tmp.269 = add_ptr(tmp.268, index=tmp.270, scale=28)
            tmp.271 = add_ptr(tmp.269, index=1L, scale=1)
            tmp.272 = &tmp.271
            tmp.274 = sign_extend 1
            tmp.273 = add_ptr(tmp.272, index=tmp.274, scale=8)
            tmp.275 = &tmp.273
            tmp.276 = tmp.275
            outer1_inner1_i.66 = tmp.276
            tmp.277 = &arr.58
            tmp.279 = sign_extend 2
            tmp.278 = add_ptr(tmp.277, index=tmp.279, scale=28)
            tmp.280 = add_ptr(tmp.278, index=1L, scale=1)
            tmp.281 = &tmp.280
            tmp.282 = add_ptr(tmp.281, index=1L, scale=1)
            tmp.283 = &tmp.282
            tmp.284 = tmp.283
            outer2_inner0_c.67 = tmp.284
            tmp.286 = sign_extend 4
            tmp.285 = array_start.59 % tmp.286
            tmp.288 = sign_extend 0
            tmp.287 = tmp.285 != tmp.288
            if !tmp.287 jump end_if_76
            return 0
        
          end_if_76:
            tmp.289 = first_scalar_elem.60 != array_start.59
            if !tmp.289 jump end_if_78
            return 0
        
          end_if_78:
            tmp.290 = outer0_inner0_i.61 - array_start.59
            tmp.292 = sign_extend 4
            tmp.291 = tmp.290 != tmp.292
            if !tmp.291 jump end_if_80
            return 0
        
          end_if_80:
            tmp.293 = outer0_inner0_c.62 - array_start.59
            tmp.295 = sign_extend 8
            tmp.294 = tmp.293 != tmp.295
            if !tmp.294 jump end_if_82
            return 0
        
          end_if_82:
            tmp.296 = outer0_end.63 != outer1.64
            if !tmp.296 jump end_if_84
            return 0
        
          end_if_84:
            tmp.297 = outer1_arr.65 - array_start.59
            tmp.299 = sign_extend 32
            tmp.298 = tmp.297 != tmp.299
            if !tmp.298 jump end_if_86
            return 0
        
          end_if_86:
            tmp.300 = outer1_arr.65 - outer1.64
            tmp.302 = sign_extend 4
            tmp.301 = tmp.300 != tmp.302
            if !tmp.301 jump end_if_88
            return 0
        
          end_if_88:
            tmp.303 = outer1_inner1_i.66 - array_start.59
            tmp.305 = sign_extend 40
            tmp.304 = tmp.303 != tmp.305
            if !tmp.304 jump end_if_90
            return 0
        
          end_if_90:
            tmp.306 = outer2_inner0_c.67 - array_start.59
            tmp.308 = sign_extend 64
            tmp.307 = tmp.306 != tmp.308
            if !tmp.307 jump end_if_92
            return 0
        
          end_if_92:
            return 1
            return 0
        }
        global function main() { 
            tmp.309 = test_eightbytes()
            tmp.310 = ! tmp.309
            if !tmp.310 jump end_if_94
            return 1
        
          end_if_94:
            tmp.311 = test_internal_padding()
            tmp.312 = ! tmp.311
            if !tmp.312 jump end_if_96
            return 2
        
          end_if_96:
            tmp.313 = test_three_bytes()
            tmp.314 = ! tmp.313
            if !tmp.314 jump end_if_98
            return 3
        
          end_if_98:
            tmp.315 = test_sixteen_bytes()
            tmp.316 = ! tmp.315
            if !tmp.316 jump end_if_100
            return 4
        
          end_if_100:
            tmp.317 = test_wonky_array()
            tmp.318 = ! tmp.317
            if !tmp.318 jump end_if_102
            return 5
        
          end_if_102:
            tmp.319 = test_contains_struct_array_array()
            tmp.320 = ! tmp.319
            if !tmp.320 jump end_if_104
            return 6
        
          end_if_104:
            return 0
            return 0
        }
        static s.21: Struct(three_bytes.2) = zero[3]
        static s.30: Struct(sixteen_bytes.3) = zero[16]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_size_and_offset_calculations_sizeof_exps() {
    let src = r#"
        struct eight_bytes {
            int i;
            char c;
        };
        struct two_bytes {
            char arr[2];
        };
        struct three_bytes {
            char arr[3];
        };
        struct sixteen_bytes {
            struct eight_bytes eight;
            struct two_bytes two;
            struct three_bytes three;
        };
        struct seven_bytes {
            struct two_bytes two;
            struct three_bytes three;
            struct two_bytes two2;
        };
        struct twentyfour_bytes {
            struct seven_bytes seven;
            struct sixteen_bytes sixteen;
        };
        struct twenty_bytes {
            struct sixteen_bytes sixteen;
            struct two_bytes two;
        };
        struct wonky {
            char arr[19];
        };
        struct internal_padding {
            char c;
            double d;
        };
        struct contains_struct_array {
            char c;
            struct eight_bytes struct_array[3];
        };
        
        struct twenty_bytes *get_twentybyte_ptr(void) {
            return 0;
        }
        int main(void) {
            struct contains_struct_array arr_struct;
            if (sizeof arr_struct.struct_array[2] !=
                8) {
                return 1;
            }
            static struct twentyfour_bytes twentyfour;
            if (sizeof twentyfour.seven.two2 != 2) {
                return 2;
            }
            if (sizeof get_twentybyte_ptr()->sixteen.three != 3) {
                return 3;
            }
            if (sizeof get_twentybyte_ptr()->sixteen != 16) {
                return 4;
            }
            if (sizeof twentyfour.seven != 7) {
                return 5;
            }
            if (sizeof twentyfour != 24) {
                return 6;
            }
            if (sizeof *get_twentybyte_ptr() != 20) {
                return 7;
            }
            if (sizeof *((struct wonky *)0) != 19) {
                return 8;
            }
            extern struct internal_padding struct_array[4];
            if (sizeof struct_array[0] != 16) {
                return 9;
            }
            if (sizeof arr_struct != 28) {
                return 10;
            }
            if (sizeof struct_array != 64) {
                return 11;
            }
            if (sizeof arr_struct.struct_array != 24) {
                return 12;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_twentybyte_ptr() { 
            tmp.0 = sign_extend 0
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.2 = sign_extend 8
            tmp.1 = 8UL != tmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = sign_extend 2
            tmp.3 = 2UL != tmp.4
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = sign_extend 3
            tmp.5 = 3UL != tmp.6
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = sign_extend 16
            tmp.7 = 16UL != tmp.8
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            tmp.10 = sign_extend 7
            tmp.9 = 7UL != tmp.10
            if !tmp.9 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = sign_extend 24
            tmp.11 = 24UL != tmp.12
            if !tmp.11 jump end_if_10
            return 6
        
          end_if_10:
            tmp.14 = sign_extend 20
            tmp.13 = 20UL != tmp.14
            if !tmp.13 jump end_if_12
            return 7
        
          end_if_12:
            tmp.16 = sign_extend 19
            tmp.15 = 19UL != tmp.16
            if !tmp.15 jump end_if_14
            return 8
        
          end_if_14:
            tmp.18 = sign_extend 16
            tmp.17 = 16UL != tmp.18
            if !tmp.17 jump end_if_16
            return 9
        
          end_if_16:
            tmp.20 = sign_extend 28
            tmp.19 = 28UL != tmp.20
            if !tmp.19 jump end_if_18
            return 10
        
          end_if_18:
            tmp.22 = sign_extend 64
            tmp.21 = 64UL != tmp.22
            if !tmp.21 jump end_if_20
            return 11
        
          end_if_20:
            tmp.24 = sign_extend 24
            tmp.23 = 24UL != tmp.24
            if !tmp.23 jump end_if_22
            return 12
        
          end_if_22:
            return 0
            return 0
        }
        static twentyfour.11: Struct(twentyfour_bytes.5) = zero[24]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_size_and_offset_calculations_sizeof_type() {
    let src = r#"
        struct eight_bytes {
            int i;
            char c;
        };
        struct two_bytes {
            char arr[2];
        };
        struct three_bytes {
            char arr[3];
        };
        struct sixteen_bytes {
            struct eight_bytes eight;
            struct two_bytes two;
            struct three_bytes three;
        };
        struct seven_bytes {
            struct two_bytes two;
            struct three_bytes three;
            struct two_bytes two2;
        };
        struct twentyfour_bytes {
            struct seven_bytes seven;
            struct sixteen_bytes sixteen;
        };
        struct twenty_bytes {
            struct sixteen_bytes sixteen;
            struct two_bytes two;
        };
        struct wonky {
            char arr[19];
        };
        struct internal_padding {
            char c;
            double d;
        };
        struct contains_struct_array {
            char c;
            struct eight_bytes struct_array[3];
        };
        
        int main(void) {
            if (sizeof(struct eight_bytes) != 8) {
                return 1;
            }
            if (sizeof(struct two_bytes) != 2) {
                return 2;
            }
            if (sizeof(struct three_bytes) != 3) {
                return 3;
            }
            if (sizeof(struct sixteen_bytes) != 16) {
                return 4;
            }
            if (sizeof(struct seven_bytes) != 7) {
                return 5;
            }
            if (sizeof(struct twentyfour_bytes) != 24) {
                return 6;
            }
            if (sizeof(struct twenty_bytes) != 20) {
                return 7;
            }
            if (sizeof(struct wonky) != 19) {
                return 8;
            }
            if (sizeof(struct internal_padding) != 16) {
                return 9;
            }
            if (sizeof(struct contains_struct_array) != 28) {
                return 10;
            }
            if (sizeof(struct internal_padding[4]) != 64) {
                return 11;
            }
            if (sizeof(struct wonky[2]) != 38) {
                return 12;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 8
            tmp.0 = 8UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 2
            tmp.2 = 2UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 3
            tmp.4 = 3UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend 16
            tmp.6 = 16UL != tmp.7
            if !tmp.6 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 7
            tmp.8 = 7UL != tmp.9
            if !tmp.8 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = sign_extend 24
            tmp.10 = 24UL != tmp.11
            if !tmp.10 jump end_if_10
            return 6
        
          end_if_10:
            tmp.13 = sign_extend 20
            tmp.12 = 20UL != tmp.13
            if !tmp.12 jump end_if_12
            return 7
        
          end_if_12:
            tmp.15 = sign_extend 19
            tmp.14 = 19UL != tmp.15
            if !tmp.14 jump end_if_14
            return 8
        
          end_if_14:
            tmp.17 = sign_extend 16
            tmp.16 = 16UL != tmp.17
            if !tmp.16 jump end_if_16
            return 9
        
          end_if_16:
            tmp.19 = sign_extend 28
            tmp.18 = 28UL != tmp.19
            if !tmp.18 jump end_if_18
            return 10
        
          end_if_18:
            tmp.21 = sign_extend 64
            tmp.20 = 64UL != tmp.21
            if !tmp.20 jump end_if_20
            return 11
        
          end_if_20:
            tmp.23 = sign_extend 38
            tmp.22 = 38UL != tmp.23
            if !tmp.22 jump end_if_22
            return 12
        
          end_if_22:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_smoke_tests_simple() {
    let src = r#"
        struct pair {
            int a;
            int b;
        };
        int main(void) {
            struct pair x = {1, 2};
            if (x.a != 1 || x.b != 2) {
                return 1;
            }
            struct pair *x_ptr = &x;
            if (x_ptr->a != 1 || x_ptr->b != 2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            pair.0[0] = 1
            pair.0[4] = 2
            tmp.0 = x.1[0]
            tmp.1 = tmp.0 != 1
            if tmp.1 jump or_true_0
            tmp.4 = x.1[1]
            tmp.5 = tmp.4 != 2
            if tmp.5 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if !tmp.3 jump end_if_2
            return 1
        
          end_if_2:
            tmp.6 = &x.1
            x_ptr.2 = tmp.6
            tmp.7 = x_ptr.2 != 1
            if tmp.7 jump or_true_4
            tmp.10 = add_ptr(x_ptr.2, index=1L, scale=1)
            tmp.11 = tmp.10 != 2
            if tmp.11 jump or_true_4
            tmp.9 = 0
            jump or_end_5
        
          or_true_4:
            tmp.9 = 1
        
          or_end_5:
            if !tmp.9 jump end_if_6
            return 2
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_smoke_tests_static_vs_auto() {
    let src = r#"
        struct s {
            int a;
            int b;
        };
        int main(void) {
            for (int i = 0; i < 10; i = i + 1) {
                struct s autom = {1, 2};
                static struct s stat = {1, 2};
                autom.a = autom.a + 1;
                autom.b = autom.b + 1;
                stat.a = stat.a + 1;
                stat.b = stat.b + 1;
                if (i == 9) {
                    if (stat.a != 11 || stat.b != 12) {
                        return 1;
                    }
                    if (autom.a != 2 || autom.b != 3) {
                        return 2;
                    }
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 10
            if !tmp.0 jump break_loop_0
            s.0[0] = 1
            s.0[4] = 2
            tmp.1 = autom.2[0]
            tmp.2 = tmp.1 + 1
            autom.2[0] = tmp.2
            tmp.3 = autom.2[1]
            tmp.4 = tmp.3 + 1
            autom.2[1] = tmp.4
            tmp.5 = stat.3[0]
            tmp.6 = tmp.5 + 1
            stat.3[0] = tmp.6
            tmp.7 = stat.3[1]
            tmp.8 = tmp.7 + 1
            stat.3[1] = tmp.8
            tmp.9 = i.1 == 9
            if !tmp.9 jump end_if_0
            tmp.10 = stat.3[0]
            tmp.11 = tmp.10 != 11
            if tmp.11 jump or_true_2
            tmp.14 = stat.3[1]
            tmp.15 = tmp.14 != 12
            if tmp.15 jump or_true_2
            tmp.13 = 0
            jump or_end_3
        
          or_true_2:
            tmp.13 = 1
        
          or_end_3:
            if !tmp.13 jump end_if_4
            return 1
        
          end_if_4:
            tmp.16 = autom.2[0]
            tmp.17 = tmp.16 != 2
            if tmp.17 jump or_true_6
            tmp.20 = autom.2[1]
            tmp.21 = tmp.20 != 3
            if tmp.21 jump or_true_6
            tmp.19 = 0
            jump or_end_7
        
          or_true_6:
            tmp.19 = 1
        
          or_end_7:
            if !tmp.19 jump end_if_8
            return 2
        
          end_if_8:
        
          end_if_0:
        
          continue_loop_0:
            tmp.22 = i.1 + 1
            i.1 = tmp.22
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
        static stat.3: Struct(s.0) = [ 1, 2]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_struct_copy_copy_struct() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        void *malloc(unsigned long size);
        struct small {
            int a;
            long b;
        };
        struct s {
            char arr[3];
            struct small inner;
        };
        struct with_end_padding {
            int a;
            int b;
            char c;
        };
        int test_auto(void) {
            struct s x = {"ab", {-1, 2}};
            struct s y = {"x", {1}};
            y = x;
            if (strcmp(y.arr, "ab") || y.inner.a != -1 || y.inner.b != 2) {
                return 0;
            }
            y.inner.a = 20;
            if (y.inner.a != 20 || x.inner.a != -1) {
                return 0;
            }
            return 1;
        }
        int test_static(void) {
            static struct s x = {"ab", {1, 2}};
            static struct s y;
            y = x;
            if (strcmp(y.arr, "ab") || y.inner.a != 1 || y.inner.b != 2) {
                return 0;
            }
            return 1;
        }
        struct wonky {
            char arr[7];
        };
        int test_wonky_size(void) {
            struct wonky x = {"abcdef"};
            static struct wonky y;
            y = x;
            if (strcmp(y.arr, "abcdef")) {
                return 0;
            }
            return 1;
        }
        int true_flag(void) {
            return 1;
        }
        int test_conditional(void) {
            static struct s x = {"xy", {1234, 5678}};
            struct s y = {"!", {-10}};
            struct s z;
            z = true_flag() ? x : y;
            if (strcmp(z.arr, "xy") || z.inner.a != 1234 || z.inner.b != 5678) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_auto()) {
                return 1;
            }
            if (!test_static()) {
                return 2;
            }
            if (!test_wonky_size()) {
                return 3;
            }
            if (!test_conditional()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_auto() { 
            s.4[0] = 'a'
            s.4[1] = 'b'
            s.4[2] = '\0'
            tmp.0 = - 1
            small.3[8] = tmp.0
            tmp.1 = sign_extend 2
            small.3[16] = tmp.1
            s.4[0] = 'x'
            s.4[1] = '\0'
            s.4[2] = '\0'
            small.3[8] = 1
            small.3[16] = 0L
            y.7 = x.6
            tmp.2 = y.7[0]
            tmp.3 = &tmp.2
            tmp.4 = &string.0
            tmp.5 = strcmp(tmp.3, tmp.4)
            if tmp.5 jump or_true_0
            tmp.8 = y.7[1]
            tmp.10 = - 1
            tmp.9 = tmp.8 != tmp.10
            if tmp.9 jump or_true_0
            tmp.7 = 0
            jump or_end_1
        
          or_true_0:
            tmp.7 = 1
        
          or_end_1:
            if tmp.7 jump or_true_2
            tmp.13 = y.7[2]
            tmp.15 = sign_extend 2
            tmp.14 = tmp.13 != tmp.15
            if tmp.14 jump or_true_2
            tmp.12 = 0
            jump or_end_3
        
          or_true_2:
            tmp.12 = 1
        
          or_end_3:
            if !tmp.12 jump end_if_4
            return 0
        
          end_if_4:
            y.7[1] = 20
            tmp.16 = y.7[1]
            tmp.17 = tmp.16 != 20
            if tmp.17 jump or_true_6
            tmp.20 = x.6[1]
            tmp.22 = - 1
            tmp.21 = tmp.20 != tmp.22
            if tmp.21 jump or_true_6
            tmp.19 = 0
            jump or_end_7
        
          or_true_6:
            tmp.19 = 1
        
          or_end_7:
            if !tmp.19 jump end_if_8
            return 0
        
          end_if_8:
            return 1
            return 0
        }
        global function test_static() { 
            y.9 = x.8
            tmp.23 = y.9[0]
            tmp.24 = &tmp.23
            tmp.25 = &string.0
            tmp.26 = strcmp(tmp.24, tmp.25)
            if tmp.26 jump or_true_10
            tmp.29 = y.9[1]
            tmp.30 = tmp.29 != 1
            if tmp.30 jump or_true_10
            tmp.28 = 0
            jump or_end_11
        
          or_true_10:
            tmp.28 = 1
        
          or_end_11:
            if tmp.28 jump or_true_12
            tmp.33 = y.9[2]
            tmp.35 = sign_extend 2
            tmp.34 = tmp.33 != tmp.35
            if tmp.34 jump or_true_12
            tmp.32 = 0
            jump or_end_13
        
          or_true_12:
            tmp.32 = 1
        
          or_end_13:
            if !tmp.32 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_wonky_size() { 
            wonky.10[0] = 'a'
            wonky.10[1] = 'b'
            wonky.10[2] = 'c'
            wonky.10[3] = 'd'
            wonky.10[4] = 'e'
            wonky.10[5] = 'f'
            wonky.10[6] = '\0'
            y.12 = x.11
            tmp.36 = y.12[0]
            tmp.37 = &tmp.36
            tmp.38 = &string.1
            tmp.39 = strcmp(tmp.37, tmp.38)
            if !tmp.39 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function true_flag() { 
            return 1
            return 0
        }
        global function test_conditional() { 
            s.4[0] = '!'
            s.4[1] = '\0'
            s.4[2] = '\0'
            tmp.40 = - 10
            small.3[8] = tmp.40
            small.3[16] = 0L
            tmp.41 = true_flag()
            if !tmp.41 jump else_19
            tmp.42 = x.13
            jump end_if_18
        
          else_19:
            tmp.42 = y.14
        
          end_if_18:
            z.15 = tmp.42
            tmp.43 = z.15[0]
            tmp.44 = &tmp.43
            tmp.45 = &string.2
            tmp.46 = strcmp(tmp.44, tmp.45)
            if tmp.46 jump or_true_20
            tmp.49 = z.15[1]
            tmp.50 = tmp.49 != 1234
            if tmp.50 jump or_true_20
            tmp.48 = 0
            jump or_end_21
        
          or_true_20:
            tmp.48 = 1
        
          or_end_21:
            if tmp.48 jump or_true_22
            tmp.53 = z.15[2]
            tmp.55 = sign_extend 5678
            tmp.54 = tmp.53 != tmp.55
            if tmp.54 jump or_true_22
            tmp.52 = 0
            jump or_end_23
        
          or_true_22:
            tmp.52 = 1
        
          or_end_23:
            if !tmp.52 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function main() { 
            tmp.56 = test_auto()
            tmp.57 = ! tmp.56
            if !tmp.57 jump end_if_26
            return 1
        
          end_if_26:
            tmp.58 = test_static()
            tmp.59 = ! tmp.58
            if !tmp.59 jump end_if_28
            return 2
        
          end_if_28:
            tmp.60 = test_wonky_size()
            tmp.61 = ! tmp.60
            if !tmp.61 jump end_if_30
            return 3
        
          end_if_30:
            tmp.62 = test_conditional()
            tmp.63 = ! tmp.62
            if !tmp.63 jump end_if_32
            return 4
        
          end_if_32:
            return 0
            return 0
        }
        constant string.0: Array(3,Char) = "ab\\0"
        constant string.1: Array(7,Char) = "abcdef\\0"
        constant string.2: Array(3,Char) = "xy\\0"
        static x.13: Struct(s.4) = [ "xy\\0", zero[5], 1234, zero[4], 5678L]
        static x.8: Struct(s.4) = [ "ab\\0", zero[5], 1, zero[4], 2L]
        static y.12: Struct(wonky.10) = zero[7]
        static y.9: Struct(s.4) = zero[24]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_struct_copy_copy_struct_through_pointer() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        void *malloc(unsigned long size);
        struct small {
            int a;
            long b;
        };
        struct s {
            char arr[3];
            struct small inner;
        };
        struct with_end_padding {
            int a;
            int b;
            char c;
        };
        int test_copy_to_pointer(void) {
            struct s y = {"!?", {-20, -30}};
            struct s *x = malloc(sizeof(struct s));
            *x = y;
            if (strcmp(x->arr, "!?") || x->inner.a != -20 || x->inner.b != -30) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_pointer(void) {
            static struct s my_struct = {"()", {77, 78}};
            struct s *y = &my_struct;
            struct s x = {"", {0, 0}};
            x = *y;
            if (strcmp(x.arr, "()") || x.inner.a != 77 || x.inner.b != 78) {
                return 0;
            }
            return 1;
        }
        int test_copy_to_and_from_pointer(void) {
            struct s my_struct = {"+-", {1000, 1001}};
            struct s *y = &my_struct;
            struct s *x = malloc(sizeof(struct s));
            *x = *y;
            if (strcmp(x->arr, "+-") || x->inner.a != 1000 || x->inner.b != 1001) {
                return 0;
            }
            return 1;
        }
        int test_copy_to_array_elem(void) {
            struct s y = {"\n\t", {10000, 20000}};
            static struct s arr[3];
            arr[1] = y;
            if (strcmp(arr[1].arr, "\n\t") || arr[1].inner.a != 10000 ||
                arr[1].inner.b != 20000) {
                return 0;
            }
            if (arr[0].inner.a || arr[0].inner.b || arr[2].arr[0] || arr[2].arr[1]) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_array_elem(void) {
            struct s arr[3] = {
                {"ab", {-3000, -4000}}, {"cd", {-5000, -6000}}, {"ef", {-7000, -8000}}};
            struct s x = {"", {0, 0}};
            x = arr[1];
            if (strcmp(x.arr, "cd") || x.inner.a != -5000 || x.inner.b != -6000) {
                return 0;
            }
            return 1;
        }
        int test_copy_to_and_from_array_elem(void) {
            struct s arr[3] = {
                {"ab", {-3000, -4000}}, {"cd", {-5000, -6000}}, {"ef", {-7000, -8000}}};
            arr[0] = arr[2];
            if (strcmp(arr[0].arr, "ef") || arr[0].inner.a != -7000 ||
                arr[0].inner.b != -8000) {
                return 0;
            }
            if (strcmp(arr[1].arr, "cd") || arr[1].inner.a != -5000 ||
                arr[1].inner.b != -6000) {
                return 0;
            }
            if (strcmp(arr[2].arr, "ef") || arr[2].inner.a != -7000 ||
                arr[2].inner.b != -8000) {
                return 0;
            }
            return 1;
        }
        int test_copy_array_element_with_padding(void) {
            struct with_end_padding arr[3] = {{0, 1, 2}, {3, 4, 5}, {6, 7, 8}};
            struct with_end_padding elem = {9, 9, 9};
            arr[1] = elem;
            if (arr[0].a != 0 || arr[0].b != 1 || arr[0].c != 2 || arr[1].a != 9 ||
                arr[1].b != 9 || arr[1].c != 9 || arr[2].a != 6 || arr[2].b != 7 ||
                arr[2].c != 8) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_copy_to_pointer()) {
                return 1;
            }
            if (!test_copy_from_pointer()) {
                return 2;
            }
            if (!test_copy_to_and_from_pointer()) {
                return 3;
            }
            if (!test_copy_to_array_elem()) {
                return 4;
            }
            if (!test_copy_from_array_elem()) {
                return 5;
            }
            if (!test_copy_to_and_from_array_elem()) {
                return 6;
            }
            if (!test_copy_array_element_with_padding()) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_copy_to_pointer() { 
            s.4[0] = '!'
            s.4[1] = '?'
            s.4[2] = '\0'
            tmp.0 = - 20
            small.3[8] = tmp.0
            tmp.1 = - 30
            tmp.2 = sign_extend tmp.1
            small.3[16] = tmp.2
            tmp.3 = malloc(24UL)
            tmp.4 = tmp.3
            x.7 = tmp.4
            *x.7 = y.6
            tmp.5 = &x.7
            tmp.6 = &string.0
            tmp.7 = strcmp(tmp.5, tmp.6)
            if tmp.7 jump or_true_0
            tmp.10 = add_ptr(x.7, index=1L, scale=1)
            tmp.11 = tmp.10[0]
            tmp.13 = - 20
            tmp.12 = tmp.11 != tmp.13
            if tmp.12 jump or_true_0
            tmp.9 = 0
            jump or_end_1
        
          or_true_0:
            tmp.9 = 1
        
          or_end_1:
            if tmp.9 jump or_true_2
            tmp.16 = add_ptr(x.7, index=1L, scale=1)
            tmp.17 = tmp.16[1]
            tmp.19 = - 30
            tmp.20 = sign_extend tmp.19
            tmp.18 = tmp.17 != tmp.20
            if tmp.18 jump or_true_2
            tmp.15 = 0
            jump or_end_3
        
          or_true_2:
            tmp.15 = 1
        
          or_end_3:
            if !tmp.15 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function test_copy_from_pointer() { 
            tmp.21 = &my_struct.8
            y.9 = tmp.21
            s.4[0] = '\0'
            s.4[1] = '\0'
            s.4[2] = '\0'
            small.3[8] = 0
            tmp.22 = sign_extend 0
            small.3[16] = tmp.22
            tmp.23 = *y.9
            x.10 = tmp.23
            tmp.24 = x.10[0]
            tmp.25 = &tmp.24
            tmp.26 = &string.1
            tmp.27 = strcmp(tmp.25, tmp.26)
            if tmp.27 jump or_true_6
            tmp.30 = x.10[1]
            tmp.31 = tmp.30 != 77
            if tmp.31 jump or_true_6
            tmp.29 = 0
            jump or_end_7
        
          or_true_6:
            tmp.29 = 1
        
          or_end_7:
            if tmp.29 jump or_true_8
            tmp.34 = x.10[2]
            tmp.36 = sign_extend 78
            tmp.35 = tmp.34 != tmp.36
            if tmp.35 jump or_true_8
            tmp.33 = 0
            jump or_end_9
        
          or_true_8:
            tmp.33 = 1
        
          or_end_9:
            if !tmp.33 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_copy_to_and_from_pointer() { 
            s.4[0] = '+'
            s.4[1] = '-'
            s.4[2] = '\0'
            small.3[8] = 1000
            tmp.37 = sign_extend 1001
            small.3[16] = tmp.37
            tmp.38 = &my_struct.11
            y.12 = tmp.38
            tmp.39 = malloc(24UL)
            tmp.40 = tmp.39
            x.13 = tmp.40
            tmp.41 = *y.12
            *x.13 = tmp.41
            tmp.42 = &x.13
            tmp.43 = &string.2
            tmp.44 = strcmp(tmp.42, tmp.43)
            if tmp.44 jump or_true_12
            tmp.47 = add_ptr(x.13, index=1L, scale=1)
            tmp.48 = tmp.47[0]
            tmp.49 = tmp.48 != 1000
            if tmp.49 jump or_true_12
            tmp.46 = 0
            jump or_end_13
        
          or_true_12:
            tmp.46 = 1
        
          or_end_13:
            if tmp.46 jump or_true_14
            tmp.52 = add_ptr(x.13, index=1L, scale=1)
            tmp.53 = tmp.52[1]
            tmp.55 = sign_extend 1001
            tmp.54 = tmp.53 != tmp.55
            if tmp.54 jump or_true_14
            tmp.51 = 0
            jump or_end_15
        
          or_true_14:
            tmp.51 = 1
        
          or_end_15:
            if !tmp.51 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function test_copy_to_array_elem() { 
            s.4[0] = '\n'
            s.4[1] = '\t'
            s.4[2] = '\0'
            small.3[8] = 10000
            tmp.56 = sign_extend 20000
            small.3[16] = tmp.56
            tmp.57 = &arr.15
            tmp.58 = sign_extend 1
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=24)
            *tmp.59 = y.14
            tmp.60 = &arr.15
            tmp.61 = sign_extend 1
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=24)
            tmp.63 = &string.3
            tmp.64 = strcmp(tmp.62, tmp.63)
            if tmp.64 jump or_true_18
            tmp.67 = &arr.15
            tmp.68 = sign_extend 1
            tmp.69 = add_ptr(tmp.67, index=tmp.68, scale=24)
            tmp.70 = add_ptr(tmp.69, index=1L, scale=1)
            tmp.71 = *tmp.70
            tmp.72 = tmp.71 != 10000
            if tmp.72 jump or_true_18
            tmp.66 = 0
            jump or_end_19
        
          or_true_18:
            tmp.66 = 1
        
          or_end_19:
            if tmp.66 jump or_true_20
            tmp.75 = &arr.15
            tmp.76 = sign_extend 1
            tmp.77 = add_ptr(tmp.75, index=tmp.76, scale=24)
            tmp.78 = add_ptr(tmp.77, index=1L, scale=1)
            tmp.79 = add_ptr(tmp.78, index=1L, scale=1)
            tmp.80 = *tmp.79
            tmp.82 = sign_extend 20000
            tmp.81 = tmp.80 != tmp.82
            if tmp.81 jump or_true_20
            tmp.74 = 0
            jump or_end_21
        
          or_true_20:
            tmp.74 = 1
        
          or_end_21:
            if !tmp.74 jump end_if_22
            return 0
        
          end_if_22:
            tmp.83 = &arr.15
            tmp.84 = sign_extend 0
            tmp.85 = add_ptr(tmp.83, index=tmp.84, scale=24)
            tmp.86 = add_ptr(tmp.85, index=1L, scale=1)
            tmp.87 = *tmp.86
            if tmp.87 jump or_true_24
            tmp.90 = &arr.15
            tmp.91 = sign_extend 0
            tmp.92 = add_ptr(tmp.90, index=tmp.91, scale=24)
            tmp.93 = add_ptr(tmp.92, index=1L, scale=1)
            tmp.94 = add_ptr(tmp.93, index=1L, scale=1)
            tmp.95 = *tmp.94
            if tmp.95 jump or_true_24
            tmp.89 = 0
            jump or_end_25
        
          or_true_24:
            tmp.89 = 1
        
          or_end_25:
            if tmp.89 jump or_true_26
            tmp.98 = &arr.15
            tmp.99 = sign_extend 2
            tmp.100 = add_ptr(tmp.98, index=tmp.99, scale=24)
            tmp.101 = sign_extend 0
            tmp.102 = add_ptr(tmp.100, index=tmp.101, scale=1)
            tmp.103 = *tmp.102
            if tmp.103 jump or_true_26
            tmp.97 = 0
            jump or_end_27
        
          or_true_26:
            tmp.97 = 1
        
          or_end_27:
            if tmp.97 jump or_true_28
            tmp.106 = &arr.15
            tmp.107 = sign_extend 2
            tmp.108 = add_ptr(tmp.106, index=tmp.107, scale=24)
            tmp.109 = sign_extend 1
            tmp.110 = add_ptr(tmp.108, index=tmp.109, scale=1)
            tmp.111 = *tmp.110
            if tmp.111 jump or_true_28
            tmp.105 = 0
            jump or_end_29
        
          or_true_28:
            tmp.105 = 1
        
          or_end_29:
            if !tmp.105 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_copy_from_array_elem() { 
            s.4[0] = 'a'
            s.4[1] = 'b'
            s.4[2] = '\0'
            tmp.112 = - 3000
            small.3[8] = tmp.112
            tmp.113 = - 4000
            tmp.114 = sign_extend tmp.113
            small.3[16] = tmp.114
            s.4[24] = 'c'
            s.4[25] = 'd'
            s.4[26] = '\0'
            tmp.115 = - 5000
            small.3[32] = tmp.115
            tmp.116 = - 6000
            tmp.117 = sign_extend tmp.116
            small.3[40] = tmp.117
            s.4[48] = 'e'
            s.4[49] = 'f'
            s.4[50] = '\0'
            tmp.118 = - 7000
            small.3[56] = tmp.118
            tmp.119 = - 8000
            tmp.120 = sign_extend tmp.119
            small.3[64] = tmp.120
            s.4[0] = '\0'
            s.4[1] = '\0'
            s.4[2] = '\0'
            small.3[8] = 0
            tmp.121 = sign_extend 0
            small.3[16] = tmp.121
            tmp.122 = &arr.16
            tmp.123 = sign_extend 1
            tmp.124 = add_ptr(tmp.122, index=tmp.123, scale=24)
            tmp.125 = *tmp.124
            x.17 = tmp.125
            tmp.126 = x.17[0]
            tmp.127 = &tmp.126
            tmp.128 = &string.4
            tmp.129 = strcmp(tmp.127, tmp.128)
            if tmp.129 jump or_true_32
            tmp.132 = x.17[1]
            tmp.134 = - 5000
            tmp.133 = tmp.132 != tmp.134
            if tmp.133 jump or_true_32
            tmp.131 = 0
            jump or_end_33
        
          or_true_32:
            tmp.131 = 1
        
          or_end_33:
            if tmp.131 jump or_true_34
            tmp.137 = x.17[2]
            tmp.139 = - 6000
            tmp.140 = sign_extend tmp.139
            tmp.138 = tmp.137 != tmp.140
            if tmp.138 jump or_true_34
            tmp.136 = 0
            jump or_end_35
        
          or_true_34:
            tmp.136 = 1
        
          or_end_35:
            if !tmp.136 jump end_if_36
            return 0
        
          end_if_36:
            return 1
            return 0
        }
        global function test_copy_to_and_from_array_elem() { 
            s.4[0] = 'a'
            s.4[1] = 'b'
            s.4[2] = '\0'
            tmp.141 = - 3000
            small.3[8] = tmp.141
            tmp.142 = - 4000
            tmp.143 = sign_extend tmp.142
            small.3[16] = tmp.143
            s.4[24] = 'c'
            s.4[25] = 'd'
            s.4[26] = '\0'
            tmp.144 = - 5000
            small.3[32] = tmp.144
            tmp.145 = - 6000
            tmp.146 = sign_extend tmp.145
            small.3[40] = tmp.146
            s.4[48] = 'e'
            s.4[49] = 'f'
            s.4[50] = '\0'
            tmp.147 = - 7000
            small.3[56] = tmp.147
            tmp.148 = - 8000
            tmp.149 = sign_extend tmp.148
            small.3[64] = tmp.149
            tmp.150 = &arr.18
            tmp.151 = sign_extend 0
            tmp.152 = add_ptr(tmp.150, index=tmp.151, scale=24)
            tmp.153 = &arr.18
            tmp.154 = sign_extend 2
            tmp.155 = add_ptr(tmp.153, index=tmp.154, scale=24)
            tmp.156 = *tmp.155
            *tmp.152 = tmp.156
            tmp.157 = &arr.18
            tmp.158 = sign_extend 0
            tmp.159 = add_ptr(tmp.157, index=tmp.158, scale=24)
            tmp.160 = &string.5
            tmp.161 = strcmp(tmp.159, tmp.160)
            if tmp.161 jump or_true_38
            tmp.164 = &arr.18
            tmp.165 = sign_extend 0
            tmp.166 = add_ptr(tmp.164, index=tmp.165, scale=24)
            tmp.167 = add_ptr(tmp.166, index=1L, scale=1)
            tmp.168 = *tmp.167
            tmp.170 = - 7000
            tmp.169 = tmp.168 != tmp.170
            if tmp.169 jump or_true_38
            tmp.163 = 0
            jump or_end_39
        
          or_true_38:
            tmp.163 = 1
        
          or_end_39:
            if tmp.163 jump or_true_40
            tmp.173 = &arr.18
            tmp.174 = sign_extend 0
            tmp.175 = add_ptr(tmp.173, index=tmp.174, scale=24)
            tmp.176 = add_ptr(tmp.175, index=1L, scale=1)
            tmp.177 = add_ptr(tmp.176, index=1L, scale=1)
            tmp.178 = *tmp.177
            tmp.180 = - 8000
            tmp.181 = sign_extend tmp.180
            tmp.179 = tmp.178 != tmp.181
            if tmp.179 jump or_true_40
            tmp.172 = 0
            jump or_end_41
        
          or_true_40:
            tmp.172 = 1
        
          or_end_41:
            if !tmp.172 jump end_if_42
            return 0
        
          end_if_42:
            tmp.182 = &arr.18
            tmp.183 = sign_extend 1
            tmp.184 = add_ptr(tmp.182, index=tmp.183, scale=24)
            tmp.185 = &string.4
            tmp.186 = strcmp(tmp.184, tmp.185)
            if tmp.186 jump or_true_44
            tmp.189 = &arr.18
            tmp.190 = sign_extend 1
            tmp.191 = add_ptr(tmp.189, index=tmp.190, scale=24)
            tmp.192 = add_ptr(tmp.191, index=1L, scale=1)
            tmp.193 = *tmp.192
            tmp.195 = - 5000
            tmp.194 = tmp.193 != tmp.195
            if tmp.194 jump or_true_44
            tmp.188 = 0
            jump or_end_45
        
          or_true_44:
            tmp.188 = 1
        
          or_end_45:
            if tmp.188 jump or_true_46
            tmp.198 = &arr.18
            tmp.199 = sign_extend 1
            tmp.200 = add_ptr(tmp.198, index=tmp.199, scale=24)
            tmp.201 = add_ptr(tmp.200, index=1L, scale=1)
            tmp.202 = add_ptr(tmp.201, index=1L, scale=1)
            tmp.203 = *tmp.202
            tmp.205 = - 6000
            tmp.206 = sign_extend tmp.205
            tmp.204 = tmp.203 != tmp.206
            if tmp.204 jump or_true_46
            tmp.197 = 0
            jump or_end_47
        
          or_true_46:
            tmp.197 = 1
        
          or_end_47:
            if !tmp.197 jump end_if_48
            return 0
        
          end_if_48:
            tmp.207 = &arr.18
            tmp.208 = sign_extend 2
            tmp.209 = add_ptr(tmp.207, index=tmp.208, scale=24)
            tmp.210 = &string.5
            tmp.211 = strcmp(tmp.209, tmp.210)
            if tmp.211 jump or_true_50
            tmp.214 = &arr.18
            tmp.215 = sign_extend 2
            tmp.216 = add_ptr(tmp.214, index=tmp.215, scale=24)
            tmp.217 = add_ptr(tmp.216, index=1L, scale=1)
            tmp.218 = *tmp.217
            tmp.220 = - 7000
            tmp.219 = tmp.218 != tmp.220
            if tmp.219 jump or_true_50
            tmp.213 = 0
            jump or_end_51
        
          or_true_50:
            tmp.213 = 1
        
          or_end_51:
            if tmp.213 jump or_true_52
            tmp.223 = &arr.18
            tmp.224 = sign_extend 2
            tmp.225 = add_ptr(tmp.223, index=tmp.224, scale=24)
            tmp.226 = add_ptr(tmp.225, index=1L, scale=1)
            tmp.227 = add_ptr(tmp.226, index=1L, scale=1)
            tmp.228 = *tmp.227
            tmp.230 = - 8000
            tmp.231 = sign_extend tmp.230
            tmp.229 = tmp.228 != tmp.231
            if tmp.229 jump or_true_52
            tmp.222 = 0
            jump or_end_53
        
          or_true_52:
            tmp.222 = 1
        
          or_end_53:
            if !tmp.222 jump end_if_54
            return 0
        
          end_if_54:
            return 1
            return 0
        }
        global function test_copy_array_element_with_padding() { 
            with_end_padding.5[0] = 0
            with_end_padding.5[4] = 1
            tmp.232 = truncate 2
            with_end_padding.5[8] = tmp.232
            with_end_padding.5[12] = 3
            with_end_padding.5[16] = 4
            tmp.233 = truncate 5
            with_end_padding.5[20] = tmp.233
            with_end_padding.5[24] = 6
            with_end_padding.5[28] = 7
            tmp.234 = truncate 8
            with_end_padding.5[32] = tmp.234
            with_end_padding.5[0] = 9
            with_end_padding.5[4] = 9
            tmp.235 = truncate 9
            with_end_padding.5[8] = tmp.235
            tmp.236 = &arr.19
            tmp.237 = sign_extend 1
            tmp.238 = add_ptr(tmp.236, index=tmp.237, scale=12)
            *tmp.238 = elem.20
            tmp.239 = &arr.19
            tmp.240 = sign_extend 0
            tmp.241 = add_ptr(tmp.239, index=tmp.240, scale=12)
            tmp.242 = *tmp.241
            tmp.243 = tmp.242 != 0
            if tmp.243 jump or_true_56
            tmp.246 = &arr.19
            tmp.247 = sign_extend 0
            tmp.248 = add_ptr(tmp.246, index=tmp.247, scale=12)
            tmp.249 = add_ptr(tmp.248, index=1L, scale=1)
            tmp.250 = *tmp.249
            tmp.251 = tmp.250 != 1
            if tmp.251 jump or_true_56
            tmp.245 = 0
            jump or_end_57
        
          or_true_56:
            tmp.245 = 1
        
          or_end_57:
            if tmp.245 jump or_true_58
            tmp.254 = &arr.19
            tmp.255 = sign_extend 0
            tmp.256 = add_ptr(tmp.254, index=tmp.255, scale=12)
            tmp.257 = add_ptr(tmp.256, index=2L, scale=1)
            tmp.258 = *tmp.257
            tmp.259 = sign_extend tmp.258
            tmp.260 = tmp.259 != 2
            if tmp.260 jump or_true_58
            tmp.253 = 0
            jump or_end_59
        
          or_true_58:
            tmp.253 = 1
        
          or_end_59:
            if tmp.253 jump or_true_60
            tmp.263 = &arr.19
            tmp.264 = sign_extend 1
            tmp.265 = add_ptr(tmp.263, index=tmp.264, scale=12)
            tmp.266 = *tmp.265
            tmp.267 = tmp.266 != 9
            if tmp.267 jump or_true_60
            tmp.262 = 0
            jump or_end_61
        
          or_true_60:
            tmp.262 = 1
        
          or_end_61:
            if tmp.262 jump or_true_62
            tmp.270 = &arr.19
            tmp.271 = sign_extend 1
            tmp.272 = add_ptr(tmp.270, index=tmp.271, scale=12)
            tmp.273 = add_ptr(tmp.272, index=1L, scale=1)
            tmp.274 = *tmp.273
            tmp.275 = tmp.274 != 9
            if tmp.275 jump or_true_62
            tmp.269 = 0
            jump or_end_63
        
          or_true_62:
            tmp.269 = 1
        
          or_end_63:
            if tmp.269 jump or_true_64
            tmp.278 = &arr.19
            tmp.279 = sign_extend 1
            tmp.280 = add_ptr(tmp.278, index=tmp.279, scale=12)
            tmp.281 = add_ptr(tmp.280, index=2L, scale=1)
            tmp.282 = *tmp.281
            tmp.283 = sign_extend tmp.282
            tmp.284 = tmp.283 != 9
            if tmp.284 jump or_true_64
            tmp.277 = 0
            jump or_end_65
        
          or_true_64:
            tmp.277 = 1
        
          or_end_65:
            if tmp.277 jump or_true_66
            tmp.287 = &arr.19
            tmp.288 = sign_extend 2
            tmp.289 = add_ptr(tmp.287, index=tmp.288, scale=12)
            tmp.290 = *tmp.289
            tmp.291 = tmp.290 != 6
            if tmp.291 jump or_true_66
            tmp.286 = 0
            jump or_end_67
        
          or_true_66:
            tmp.286 = 1
        
          or_end_67:
            if tmp.286 jump or_true_68
            tmp.294 = &arr.19
            tmp.295 = sign_extend 2
            tmp.296 = add_ptr(tmp.294, index=tmp.295, scale=12)
            tmp.297 = add_ptr(tmp.296, index=1L, scale=1)
            tmp.298 = *tmp.297
            tmp.299 = tmp.298 != 7
            if tmp.299 jump or_true_68
            tmp.293 = 0
            jump or_end_69
        
          or_true_68:
            tmp.293 = 1
        
          or_end_69:
            if tmp.293 jump or_true_70
            tmp.302 = &arr.19
            tmp.303 = sign_extend 2
            tmp.304 = add_ptr(tmp.302, index=tmp.303, scale=12)
            tmp.305 = add_ptr(tmp.304, index=2L, scale=1)
            tmp.306 = *tmp.305
            tmp.307 = sign_extend tmp.306
            tmp.308 = tmp.307 != 8
            if tmp.308 jump or_true_70
            tmp.301 = 0
            jump or_end_71
        
          or_true_70:
            tmp.301 = 1
        
          or_end_71:
            if !tmp.301 jump end_if_72
            return 0
        
          end_if_72:
            return 1
            return 0
        }
        global function main() { 
            tmp.309 = test_copy_to_pointer()
            tmp.310 = ! tmp.309
            if !tmp.310 jump end_if_74
            return 1
        
          end_if_74:
            tmp.311 = test_copy_from_pointer()
            tmp.312 = ! tmp.311
            if !tmp.312 jump end_if_76
            return 2
        
          end_if_76:
            tmp.313 = test_copy_to_and_from_pointer()
            tmp.314 = ! tmp.313
            if !tmp.314 jump end_if_78
            return 3
        
          end_if_78:
            tmp.315 = test_copy_to_array_elem()
            tmp.316 = ! tmp.315
            if !tmp.316 jump end_if_80
            return 4
        
          end_if_80:
            tmp.317 = test_copy_from_array_elem()
            tmp.318 = ! tmp.317
            if !tmp.318 jump end_if_82
            return 5
        
          end_if_82:
            tmp.319 = test_copy_to_and_from_array_elem()
            tmp.320 = ! tmp.319
            if !tmp.320 jump end_if_84
            return 6
        
          end_if_84:
            tmp.321 = test_copy_array_element_with_padding()
            tmp.322 = ! tmp.321
            if !tmp.322 jump end_if_86
            return 7
        
          end_if_86:
            return 0
            return 0
        }
        static arr.15: Array(3,Struct(s.4)) = zero[72]
        static my_struct.8: Struct(s.4) = [ "()\\0", zero[5], 77, zero[4], 78L]
        constant string.0: Array(3,Char) = "!?\\0"
        constant string.1: Array(3,Char) = "()\\0"
        constant string.2: Array(3,Char) = "+-\\0"
        constant string.3: Array(3,Char) = "\n\t\\0"
        constant string.4: Array(3,Char) = "cd\\0"
        constant string.5: Array(3,Char) = "ef\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_struct_copy_copy_struct_with_arrow_operator() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *malloc(unsigned long size);
        struct inner {
            double d;
            int i;
        };
        struct outer {
            char a;
            char b;
            struct inner substruct;
        };
        struct outermost {
            int i;
            struct outer *nested_ptr;
            struct outer nested_struct;
        };
        int test_copy_from_member_pointer(void) {
            struct inner small = {0.0, 0};
            struct outer *outer_ptr = malloc(sizeof(struct outer));
            outer_ptr->a = 100;
            outer_ptr->substruct.d = 21.5;
            outer_ptr->substruct.i = 100001;
            small = outer_ptr->substruct;
            if (small.d != 21.5 || small.i != 100001) {
                return 0;
            }
            return 1;
        }
        int test_copy_to_member_pointer(void) {
            struct inner small = {99.25, 987654};
            struct outer *outer_ptr = calloc(1, sizeof(struct outer));
            outer_ptr->substruct = small;
            if (outer_ptr->substruct.d != 99.25 || outer_ptr->substruct.i != 987654) {
                return 0;
            }
            if (outer_ptr->a || outer_ptr->b) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_nested_member_pointer(void) {
            struct inner small = {99.25, 987654};
            struct outermost *outer_ptr = calloc(1, sizeof(struct outermost));
            outer_ptr->nested_ptr = calloc(1, sizeof(struct outer));
            outer_ptr->i = -5;
            outer_ptr->nested_ptr->a = 101;
            outer_ptr->nested_ptr->b = 102;
            outer_ptr->nested_ptr->substruct.d = 77.5;
            outer_ptr->nested_ptr->substruct.i = 88;
            small = outer_ptr->nested_ptr->substruct;
            if (small.d != 77.5 || small.i != 88) {
                return 0;
            }
            if (outer_ptr->i != -5 || outer_ptr->nested_struct.a) {
                return 0;
            }
            return 1;
        }
        int test_copy_to_nested_member_pointer(void) {
            struct inner small = {99.25, 987654};
            struct outermost *outer_ptr = calloc(1, sizeof(struct outermost));
            outer_ptr->nested_ptr = calloc(1, sizeof(struct outer));
            outer_ptr->nested_ptr->substruct = small;
            if (outer_ptr->nested_ptr->substruct.d != 99.25 ||
                outer_ptr->nested_ptr->substruct.i != 987654) {
                return 0;
            }
            if (outer_ptr->nested_ptr->a || outer_ptr->nested_ptr->b) {
                return 0;
            }
            return 1;
        }
        int test_mixed_nested_access(void) {
            struct outermost s1 = {100, 0, {0, 0, {0, 0}}};
            struct outermost *s2_ptr = calloc(1, sizeof(struct outermost));
            s1.i = 2147483647;
            s1.nested_ptr = calloc(1, sizeof(struct outermost));
            s1.nested_ptr->a = 125;
            s1.nested_ptr->b = 126;
            s1.nested_ptr->substruct.d = -50.;
            s1.nested_ptr->substruct.i = -70;
            s1.nested_struct.a = 101;
            s1.nested_struct.b = 102;
            s2_ptr->i = -2147483647;
            s2_ptr->nested_ptr = calloc(1, sizeof(struct outermost));
            s2_ptr->nested_ptr->a = 5;
            s2_ptr->nested_ptr->b = 6;
            s2_ptr->nested_struct.substruct.d = 8.e8;
            s2_ptr->nested_struct.substruct.i = -5;
            s1.nested_ptr->substruct = s2_ptr->nested_struct.substruct;
            if (s1.nested_ptr->substruct.d != 8.e8 ||
                s1.nested_ptr->substruct.i != -5) {
                return 0;
            }
            if (s1.nested_ptr->a != 125 || s1.nested_ptr->b != 126) {
                return 0;
            }
            return 1;
        }
        int test_member_from_cast(void) {
            struct inner small = {20.0, 10};
            void *outer_ptr = calloc(1, sizeof(struct outer));
            ((struct outer *)outer_ptr)->substruct = small;
            if (((struct outer *)outer_ptr)->substruct.d != 20.0 ||
                ((struct outer *)outer_ptr)->substruct.i != 10) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_copy_from_member_pointer()) {
                return 1;
            }
            if (!test_copy_to_member_pointer()) {
                return 2;
            }
            if (!test_copy_from_nested_member_pointer()) {
                return 3;
            }
            if (!test_copy_to_nested_member_pointer()) {
                return 4;
            }
            if (!test_mixed_nested_access()) {
                return 5;
            }
            if (!test_member_from_cast()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_copy_from_member_pointer() { 
            inner.3[0] = 0D
            inner.3[8] = 0
            tmp.0 = malloc(24UL)
            tmp.1 = tmp.0
            outer_ptr.7 = tmp.1
            tmp.2 = truncate 100
            outer_ptr.7 = tmp.2
            tmp.3 = add_ptr(outer_ptr.7, index=2L, scale=1)
            tmp.3[0] = 21.5D
            tmp.4 = add_ptr(outer_ptr.7, index=2L, scale=1)
            tmp.4[1] = 100001
            tmp.5 = add_ptr(outer_ptr.7, index=2L, scale=1)
            small.6 = tmp.5
            tmp.6 = small.6[0]
            tmp.7 = tmp.6 != 21.5D
            if tmp.7 jump or_true_0
            tmp.10 = small.6[1]
            tmp.11 = tmp.10 != 100001
            if tmp.11 jump or_true_0
            tmp.9 = 0
            jump or_end_1
        
          or_true_0:
            tmp.9 = 1
        
          or_end_1:
            if !tmp.9 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_copy_to_member_pointer() { 
            inner.3[0] = 99.25D
            inner.3[8] = 987654
            tmp.12 = sign_extend 1
            tmp.13 = calloc(tmp.12, 24UL)
            tmp.14 = tmp.13
            outer_ptr.9 = tmp.14
            tmp.15 = add_ptr(outer_ptr.9, index=2L, scale=1)
            tmp.15 = small.8
            tmp.16 = add_ptr(outer_ptr.9, index=2L, scale=1)
            tmp.17 = tmp.16[0]
            tmp.18 = tmp.17 != 99.25D
            if tmp.18 jump or_true_4
            tmp.21 = add_ptr(outer_ptr.9, index=2L, scale=1)
            tmp.22 = tmp.21[1]
            tmp.23 = tmp.22 != 987654
            if tmp.23 jump or_true_4
            tmp.20 = 0
            jump or_end_5
        
          or_true_4:
            tmp.20 = 1
        
          or_end_5:
            if !tmp.20 jump end_if_6
            return 0
        
          end_if_6:
            if outer_ptr.9 jump or_true_8
            tmp.26 = add_ptr(outer_ptr.9, index=1L, scale=1)
            if tmp.26 jump or_true_8
            tmp.25 = 0
            jump or_end_9
        
          or_true_8:
            tmp.25 = 1
        
          or_end_9:
            if !tmp.25 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_copy_from_nested_member_pointer() { 
            inner.3[0] = 99.25D
            inner.3[8] = 987654
            tmp.27 = sign_extend 1
            tmp.28 = calloc(tmp.27, 40UL)
            tmp.29 = tmp.28
            outer_ptr.11 = tmp.29
            tmp.30 = add_ptr(outer_ptr.11, index=1L, scale=1)
            tmp.31 = sign_extend 1
            tmp.32 = calloc(tmp.31, 24UL)
            tmp.33 = tmp.32
            tmp.30 = tmp.33
            tmp.34 = - 5
            outer_ptr.11 = tmp.34
            tmp.35 = add_ptr(outer_ptr.11, index=1L, scale=1)
            tmp.36 = truncate 101
            tmp.35 = tmp.36
            tmp.37 = add_ptr(outer_ptr.11, index=1L, scale=1)
            tmp.38 = add_ptr(tmp.37, index=1L, scale=1)
            tmp.39 = truncate 102
            tmp.38 = tmp.39
            tmp.40 = add_ptr(outer_ptr.11, index=1L, scale=1)
            tmp.41 = add_ptr(tmp.40, index=2L, scale=1)
            tmp.41[0] = 77.5D
            tmp.42 = add_ptr(outer_ptr.11, index=1L, scale=1)
            tmp.43 = add_ptr(tmp.42, index=2L, scale=1)
            tmp.43[1] = 88
            tmp.44 = add_ptr(outer_ptr.11, index=1L, scale=1)
            tmp.45 = add_ptr(tmp.44, index=2L, scale=1)
            small.10 = tmp.45
            tmp.46 = small.10[0]
            tmp.47 = tmp.46 != 77.5D
            if tmp.47 jump or_true_12
            tmp.50 = small.10[1]
            tmp.51 = tmp.50 != 88
            if tmp.51 jump or_true_12
            tmp.49 = 0
            jump or_end_13
        
          or_true_12:
            tmp.49 = 1
        
          or_end_13:
            if !tmp.49 jump end_if_14
            return 0
        
          end_if_14:
            tmp.53 = - 5
            tmp.52 = outer_ptr.11 != tmp.53
            if tmp.52 jump or_true_16
            tmp.56 = add_ptr(outer_ptr.11, index=2L, scale=1)
            tmp.57 = tmp.56[0]
            if tmp.57 jump or_true_16
            tmp.55 = 0
            jump or_end_17
        
          or_true_16:
            tmp.55 = 1
        
          or_end_17:
            if !tmp.55 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function test_copy_to_nested_member_pointer() { 
            inner.3[0] = 99.25D
            inner.3[8] = 987654
            tmp.58 = sign_extend 1
            tmp.59 = calloc(tmp.58, 40UL)
            tmp.60 = tmp.59
            outer_ptr.13 = tmp.60
            tmp.61 = add_ptr(outer_ptr.13, index=1L, scale=1)
            tmp.62 = sign_extend 1
            tmp.63 = calloc(tmp.62, 24UL)
            tmp.64 = tmp.63
            tmp.61 = tmp.64
            tmp.65 = add_ptr(outer_ptr.13, index=1L, scale=1)
            tmp.66 = add_ptr(tmp.65, index=2L, scale=1)
            tmp.66 = small.12
            tmp.67 = add_ptr(outer_ptr.13, index=1L, scale=1)
            tmp.68 = add_ptr(tmp.67, index=2L, scale=1)
            tmp.69 = tmp.68[0]
            tmp.70 = tmp.69 != 99.25D
            if tmp.70 jump or_true_20
            tmp.73 = add_ptr(outer_ptr.13, index=1L, scale=1)
            tmp.74 = add_ptr(tmp.73, index=2L, scale=1)
            tmp.75 = tmp.74[1]
            tmp.76 = tmp.75 != 987654
            if tmp.76 jump or_true_20
            tmp.72 = 0
            jump or_end_21
        
          or_true_20:
            tmp.72 = 1
        
          or_end_21:
            if !tmp.72 jump end_if_22
            return 0
        
          end_if_22:
            tmp.77 = add_ptr(outer_ptr.13, index=1L, scale=1)
            if tmp.77 jump or_true_24
            tmp.80 = add_ptr(outer_ptr.13, index=1L, scale=1)
            tmp.81 = add_ptr(tmp.80, index=1L, scale=1)
            if tmp.81 jump or_true_24
            tmp.79 = 0
            jump or_end_25
        
          or_true_24:
            tmp.79 = 1
        
          or_end_25:
            if !tmp.79 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function test_mixed_nested_access() { 
            outermost.5[0] = 100
            tmp.82 = sign_extend 0
            outermost.5[8] = tmp.82
            tmp.83 = truncate 0
            outer.4[16] = tmp.83
            tmp.84 = truncate 0
            outer.4[17] = tmp.84
            tmp.85 = int_to_double 0
            inner.3[24] = tmp.85
            inner.3[32] = 0
            tmp.86 = sign_extend 1
            tmp.87 = calloc(tmp.86, 40UL)
            tmp.88 = tmp.87
            s2_ptr.15 = tmp.88
            s1.14[0] = 2147483647
            tmp.89 = sign_extend 1
            tmp.90 = calloc(tmp.89, 40UL)
            tmp.91 = tmp.90
            s1.14[1] = tmp.91
            tmp.92 = s1.14[1]
            tmp.93 = truncate 125
            tmp.92 = tmp.93
            tmp.94 = s1.14[1]
            tmp.95 = add_ptr(tmp.94, index=1L, scale=1)
            tmp.96 = truncate 126
            tmp.95 = tmp.96
            tmp.97 = s1.14[1]
            tmp.98 = add_ptr(tmp.97, index=2L, scale=1)
            tmp.99 = - 50D
            tmp.98[0] = tmp.99
            tmp.100 = s1.14[1]
            tmp.101 = add_ptr(tmp.100, index=2L, scale=1)
            tmp.102 = - 70
            tmp.101[1] = tmp.102
            tmp.103 = truncate 101
            s1.14[2] = tmp.103
            tmp.104 = truncate 102
            s1.14[3] = tmp.104
            tmp.105 = - 2147483647
            s2_ptr.15 = tmp.105
            tmp.106 = add_ptr(s2_ptr.15, index=1L, scale=1)
            tmp.107 = sign_extend 1
            tmp.108 = calloc(tmp.107, 40UL)
            tmp.109 = tmp.108
            tmp.106 = tmp.109
            tmp.110 = add_ptr(s2_ptr.15, index=1L, scale=1)
            tmp.111 = truncate 5
            tmp.110 = tmp.111
            tmp.112 = add_ptr(s2_ptr.15, index=1L, scale=1)
            tmp.113 = add_ptr(tmp.112, index=1L, scale=1)
            tmp.114 = truncate 6
            tmp.113 = tmp.114
            tmp.115 = add_ptr(s2_ptr.15, index=2L, scale=1)
            tmp.115[2] = 800000000D
            tmp.116 = add_ptr(s2_ptr.15, index=2L, scale=1)
            tmp.117 = - 5
            tmp.116[3] = tmp.117
            tmp.118 = s1.14[1]
            tmp.119 = add_ptr(tmp.118, index=2L, scale=1)
            tmp.120 = add_ptr(s2_ptr.15, index=2L, scale=1)
            tmp.121 = tmp.120[2]
            tmp.119 = tmp.121
            tmp.122 = s1.14[1]
            tmp.123 = add_ptr(tmp.122, index=2L, scale=1)
            tmp.124 = tmp.123[0]
            tmp.125 = tmp.124 != 800000000D
            if tmp.125 jump or_true_28
            tmp.128 = s1.14[1]
            tmp.129 = add_ptr(tmp.128, index=2L, scale=1)
            tmp.130 = tmp.129[1]
            tmp.132 = - 5
            tmp.131 = tmp.130 != tmp.132
            if tmp.131 jump or_true_28
            tmp.127 = 0
            jump or_end_29
        
          or_true_28:
            tmp.127 = 1
        
          or_end_29:
            if !tmp.127 jump end_if_30
            return 0
        
          end_if_30:
            tmp.133 = s1.14[1]
            tmp.134 = sign_extend tmp.133
            tmp.135 = tmp.134 != 125
            if tmp.135 jump or_true_32
            tmp.138 = s1.14[1]
            tmp.139 = add_ptr(tmp.138, index=1L, scale=1)
            tmp.140 = sign_extend tmp.139
            tmp.141 = tmp.140 != 126
            if tmp.141 jump or_true_32
            tmp.137 = 0
            jump or_end_33
        
          or_true_32:
            tmp.137 = 1
        
          or_end_33:
            if !tmp.137 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function test_member_from_cast() { 
            inner.3[0] = 20D
            inner.3[8] = 10
            tmp.142 = sign_extend 1
            tmp.143 = calloc(tmp.142, 24UL)
            outer_ptr.17 = tmp.143
            tmp.144 = outer_ptr.17
            tmp.145 = add_ptr(tmp.144, index=2L, scale=1)
            tmp.145 = small.16
            tmp.146 = outer_ptr.17
            tmp.147 = add_ptr(tmp.146, index=2L, scale=1)
            tmp.148 = tmp.147[0]
            tmp.149 = tmp.148 != 20D
            if tmp.149 jump or_true_36
            tmp.152 = outer_ptr.17
            tmp.153 = add_ptr(tmp.152, index=2L, scale=1)
            tmp.154 = tmp.153[1]
            tmp.155 = tmp.154 != 10
            if tmp.155 jump or_true_36
            tmp.151 = 0
            jump or_end_37
        
          or_true_36:
            tmp.151 = 1
        
          or_end_37:
            if !tmp.151 jump end_if_38
            return 0
        
          end_if_38:
            return 1
            return 0
        }
        global function main() { 
            tmp.156 = test_copy_from_member_pointer()
            tmp.157 = ! tmp.156
            if !tmp.157 jump end_if_40
            return 1
        
          end_if_40:
            tmp.158 = test_copy_to_member_pointer()
            tmp.159 = ! tmp.158
            if !tmp.159 jump end_if_42
            return 2
        
          end_if_42:
            tmp.160 = test_copy_from_nested_member_pointer()
            tmp.161 = ! tmp.160
            if !tmp.161 jump end_if_44
            return 3
        
          end_if_44:
            tmp.162 = test_copy_to_nested_member_pointer()
            tmp.163 = ! tmp.162
            if !tmp.163 jump end_if_46
            return 4
        
          end_if_46:
            tmp.164 = test_mixed_nested_access()
            tmp.165 = ! tmp.164
            if !tmp.165 jump end_if_48
            return 5
        
          end_if_48:
            tmp.166 = test_member_from_cast()
            tmp.167 = ! tmp.166
            if !tmp.167 jump end_if_50
            return 6
        
          end_if_50:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_struct_copy_copy_struct_with_dot_operator() {
    let src = r#"
        struct inner {
            signed char a;
            signed char b;
            signed char arr[3];
        };
        struct outer {
            struct inner substruct;
            signed char x;
            signed char y;
        };
        struct outermost {
            struct outer nested;
            int i;
        };
        int test_copy_from_member(void) {
            static struct outer big_struct = {{10, 9, {8, 7, 6}}, 5, 4};
            char arr[3] = {'a', 'b', 'c'};
            struct inner substruct = {-1, -1, {-1, -1, -1}};
            char arr2[3] = {'d', 'e', 'f'};
            substruct = big_struct.substruct;
            if (substruct.a != 10 || substruct.b != 9 || substruct.arr[0] != 8 ||
                substruct.arr[1] != 7 || substruct.arr[2] != 6) {
                return 0;
            }
            if (arr[0] != 'a' || arr[1] != 'b' || arr[2] != 'c' || arr2[0] != 'd' ||
                arr2[1] != 'e' || arr2[2] != 'f') {
                return 0;
            }
            return 1;
        }
        int test_copy_to_member(void) {
            static struct outer big_struct = {{0, 0, {0, 0, 0}}, 0, 0};
            struct inner small_struct = {-1, -2, {-3, -4, -5}};
            big_struct.substruct = small_struct;
            if (big_struct.substruct.a != -1 || big_struct.substruct.b != -2 ||
                big_struct.substruct.arr[0] != -3 ||
                big_struct.substruct.arr[1] != -4 ||
                big_struct.substruct.arr[2] != -5) {
                return 0;
            }
            if (big_struct.x || big_struct.y) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_nested_member(void) {
            struct outermost biggest_struct = {{{-1, -2, {-3, -4, -5}}, -6, -7}, 0};
            static struct inner small_struct;
            small_struct = biggest_struct.nested.substruct;
            if (small_struct.a != -1 || small_struct.b != -2 ||
                small_struct.arr[0] != -3 || small_struct.arr[1] != -4 ||
                small_struct.arr[2] != -5) {
                return 0;
            }
            return 1;
        }
        int test_copy_to_nested_member(void) {
            struct outermost biggest_struct = {{{0, 0, {0, 0, 0}}, 0, 0}, -1};
            static struct inner small_struct = {50, 51, {52, 53, 54}};
            biggest_struct.nested.substruct = small_struct;
            if (biggest_struct.nested.substruct.a != 50 ||
                biggest_struct.nested.substruct.b != 51 ||
                biggest_struct.nested.substruct.arr[0] != 52 ||
                biggest_struct.nested.substruct.arr[1] != 53 ||
                biggest_struct.nested.substruct.arr[2] != 54) {
                return 0;
            }
            if (biggest_struct.nested.x || biggest_struct.nested.y) {
                return 0;
            }
            if (biggest_struct.i != -1) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_conditional(void) {
            struct outer big_struct = {{127, -128, {61, 62, 63}}, -10, -11};
            struct outer big_struct2 = {{0, 1, {2, 3, 4}}, 5, 6};
            static int t = 1;
            static int f = 0;
            struct inner small_struct = (f ? big_struct : big_struct2).substruct;
            if (small_struct.a != 0 || small_struct.b != 1 ||
                small_struct.arr[0] != 2 || small_struct.arr[1] != 3 ||
                small_struct.arr[2] != 4) {
                return 0;
            }
            small_struct = (t ? big_struct : big_struct2).substruct;
            if (small_struct.a != 127 || small_struct.b != -128 ||
                small_struct.arr[0] != 61 || small_struct.arr[1] != 62 ||
                small_struct.arr[2] != 63) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_assignment(void) {
            struct outer big_struct = {{127, -128, {61, 62, 63}}, -10, -11};
            static struct outer big_struct2;
            static struct inner small_struct;
            small_struct = (big_struct2 = big_struct).substruct;
            if (small_struct.a != 127 || small_struct.b != -128 ||
                small_struct.arr[0] != 61 || small_struct.arr[1] != 62 ||
                small_struct.arr[2] != 63) {
                return 0;
            }
            if (big_struct2.substruct.a != 127 || big_struct2.substruct.b != -128 ||
                big_struct2.substruct.arr[0] != 61 ||
                big_struct2.substruct.arr[1] != 62 ||
                big_struct2.substruct.arr[2] != 63 || big_struct2.x != -10 ||
                big_struct2.y != -11) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_copy_from_member()) {
                return 1;
            }
            if (!test_copy_to_member()) {
                return 2;
            }
            if (!test_copy_from_nested_member()) {
                return 3;
            }
            if (!test_copy_to_nested_member()) {
                return 4;
            }
            if (!test_copy_from_conditional()) {
                return 6;
            }
            if (!test_copy_from_assignment()) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_copy_from_member() { 
            tmp.0 = truncate 97
            arr.4[0] = tmp.0
            tmp.1 = truncate 98
            arr.4[1] = tmp.1
            tmp.2 = truncate 99
            arr.4[2] = tmp.2
            tmp.3 = - 1
            tmp.4 = truncate tmp.3
            inner.0[0] = tmp.4
            tmp.5 = - 1
            tmp.6 = truncate tmp.5
            inner.0[1] = tmp.6
            tmp.7 = - 1
            tmp.8 = truncate tmp.7
            inner.0[2] = tmp.8
            tmp.9 = - 1
            tmp.10 = truncate tmp.9
            inner.0[3] = tmp.10
            tmp.11 = - 1
            tmp.12 = truncate tmp.11
            inner.0[4] = tmp.12
            tmp.13 = truncate 100
            arr2.6[0] = tmp.13
            tmp.14 = truncate 101
            arr2.6[1] = tmp.14
            tmp.15 = truncate 102
            arr2.6[2] = tmp.15
            tmp.16 = big_struct.3[0]
            substruct.5 = tmp.16
            tmp.17 = substruct.5[0]
            tmp.18 = sign_extend tmp.17
            tmp.19 = tmp.18 != 10
            if tmp.19 jump or_true_0
            tmp.22 = substruct.5[1]
            tmp.23 = sign_extend tmp.22
            tmp.24 = tmp.23 != 9
            if tmp.24 jump or_true_0
            tmp.21 = 0
            jump or_end_1
        
          or_true_0:
            tmp.21 = 1
        
          or_end_1:
            if tmp.21 jump or_true_2
            tmp.27 = substruct.5[2]
            tmp.28 = &tmp.27
            tmp.29 = sign_extend 0
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=1)
            tmp.31 = *tmp.30
            tmp.32 = sign_extend tmp.31
            tmp.33 = tmp.32 != 8
            if tmp.33 jump or_true_2
            tmp.26 = 0
            jump or_end_3
        
          or_true_2:
            tmp.26 = 1
        
          or_end_3:
            if tmp.26 jump or_true_4
            tmp.36 = substruct.5[2]
            tmp.37 = &tmp.36
            tmp.38 = sign_extend 1
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = sign_extend tmp.40
            tmp.42 = tmp.41 != 7
            if tmp.42 jump or_true_4
            tmp.35 = 0
            jump or_end_5
        
          or_true_4:
            tmp.35 = 1
        
          or_end_5:
            if tmp.35 jump or_true_6
            tmp.45 = substruct.5[2]
            tmp.46 = &tmp.45
            tmp.47 = sign_extend 2
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=1)
            tmp.49 = *tmp.48
            tmp.50 = sign_extend tmp.49
            tmp.51 = tmp.50 != 6
            if tmp.51 jump or_true_6
            tmp.44 = 0
            jump or_end_7
        
          or_true_6:
            tmp.44 = 1
        
          or_end_7:
            if !tmp.44 jump end_if_8
            return 0
        
          end_if_8:
            tmp.52 = &arr.4
            tmp.53 = sign_extend 0
            tmp.54 = add_ptr(tmp.52, index=tmp.53, scale=1)
            tmp.55 = *tmp.54
            tmp.56 = sign_extend tmp.55
            tmp.57 = tmp.56 != 97
            if tmp.57 jump or_true_10
            tmp.60 = &arr.4
            tmp.61 = sign_extend 1
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=1)
            tmp.63 = *tmp.62
            tmp.64 = sign_extend tmp.63
            tmp.65 = tmp.64 != 98
            if tmp.65 jump or_true_10
            tmp.59 = 0
            jump or_end_11
        
          or_true_10:
            tmp.59 = 1
        
          or_end_11:
            if tmp.59 jump or_true_12
            tmp.68 = &arr.4
            tmp.69 = sign_extend 2
            tmp.70 = add_ptr(tmp.68, index=tmp.69, scale=1)
            tmp.71 = *tmp.70
            tmp.72 = sign_extend tmp.71
            tmp.73 = tmp.72 != 99
            if tmp.73 jump or_true_12
            tmp.67 = 0
            jump or_end_13
        
          or_true_12:
            tmp.67 = 1
        
          or_end_13:
            if tmp.67 jump or_true_14
            tmp.76 = &arr2.6
            tmp.77 = sign_extend 0
            tmp.78 = add_ptr(tmp.76, index=tmp.77, scale=1)
            tmp.79 = *tmp.78
            tmp.80 = sign_extend tmp.79
            tmp.81 = tmp.80 != 100
            if tmp.81 jump or_true_14
            tmp.75 = 0
            jump or_end_15
        
          or_true_14:
            tmp.75 = 1
        
          or_end_15:
            if tmp.75 jump or_true_16
            tmp.84 = &arr2.6
            tmp.85 = sign_extend 1
            tmp.86 = add_ptr(tmp.84, index=tmp.85, scale=1)
            tmp.87 = *tmp.86
            tmp.88 = sign_extend tmp.87
            tmp.89 = tmp.88 != 101
            if tmp.89 jump or_true_16
            tmp.83 = 0
            jump or_end_17
        
          or_true_16:
            tmp.83 = 1
        
          or_end_17:
            if tmp.83 jump or_true_18
            tmp.92 = &arr2.6
            tmp.93 = sign_extend 2
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=1)
            tmp.95 = *tmp.94
            tmp.96 = sign_extend tmp.95
            tmp.97 = tmp.96 != 102
            if tmp.97 jump or_true_18
            tmp.91 = 0
            jump or_end_19
        
          or_true_18:
            tmp.91 = 1
        
          or_end_19:
            if !tmp.91 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function test_copy_to_member() { 
            tmp.98 = - 1
            tmp.99 = truncate tmp.98
            inner.0[0] = tmp.99
            tmp.100 = - 2
            tmp.101 = truncate tmp.100
            inner.0[1] = tmp.101
            tmp.102 = - 3
            tmp.103 = truncate tmp.102
            inner.0[2] = tmp.103
            tmp.104 = - 4
            tmp.105 = truncate tmp.104
            inner.0[3] = tmp.105
            tmp.106 = - 5
            tmp.107 = truncate tmp.106
            inner.0[4] = tmp.107
            big_struct.7[0] = small_struct.8
            tmp.108 = big_struct.7[0]
            tmp.109 = sign_extend tmp.108
            tmp.111 = - 1
            tmp.110 = tmp.109 != tmp.111
            if tmp.110 jump or_true_22
            tmp.114 = big_struct.7[1]
            tmp.115 = sign_extend tmp.114
            tmp.117 = - 2
            tmp.116 = tmp.115 != tmp.117
            if tmp.116 jump or_true_22
            tmp.113 = 0
            jump or_end_23
        
          or_true_22:
            tmp.113 = 1
        
          or_end_23:
            if tmp.113 jump or_true_24
            tmp.120 = big_struct.7[2]
            tmp.121 = &tmp.120
            tmp.122 = sign_extend 0
            tmp.123 = add_ptr(tmp.121, index=tmp.122, scale=1)
            tmp.124 = *tmp.123
            tmp.125 = sign_extend tmp.124
            tmp.127 = - 3
            tmp.126 = tmp.125 != tmp.127
            if tmp.126 jump or_true_24
            tmp.119 = 0
            jump or_end_25
        
          or_true_24:
            tmp.119 = 1
        
          or_end_25:
            if tmp.119 jump or_true_26
            tmp.130 = big_struct.7[2]
            tmp.131 = &tmp.130
            tmp.132 = sign_extend 1
            tmp.133 = add_ptr(tmp.131, index=tmp.132, scale=1)
            tmp.134 = *tmp.133
            tmp.135 = sign_extend tmp.134
            tmp.137 = - 4
            tmp.136 = tmp.135 != tmp.137
            if tmp.136 jump or_true_26
            tmp.129 = 0
            jump or_end_27
        
          or_true_26:
            tmp.129 = 1
        
          or_end_27:
            if tmp.129 jump or_true_28
            tmp.140 = big_struct.7[2]
            tmp.141 = &tmp.140
            tmp.142 = sign_extend 2
            tmp.143 = add_ptr(tmp.141, index=tmp.142, scale=1)
            tmp.144 = *tmp.143
            tmp.145 = sign_extend tmp.144
            tmp.147 = - 5
            tmp.146 = tmp.145 != tmp.147
            if tmp.146 jump or_true_28
            tmp.139 = 0
            jump or_end_29
        
          or_true_28:
            tmp.139 = 1
        
          or_end_29:
            if !tmp.139 jump end_if_30
            return 0
        
          end_if_30:
            tmp.148 = big_struct.7[1]
            if tmp.148 jump or_true_32
            tmp.151 = big_struct.7[2]
            if tmp.151 jump or_true_32
            tmp.150 = 0
            jump or_end_33
        
          or_true_32:
            tmp.150 = 1
        
          or_end_33:
            if !tmp.150 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function test_copy_from_nested_member() { 
            tmp.152 = - 1
            tmp.153 = truncate tmp.152
            inner.0[0] = tmp.153
            tmp.154 = - 2
            tmp.155 = truncate tmp.154
            inner.0[1] = tmp.155
            tmp.156 = - 3
            tmp.157 = truncate tmp.156
            inner.0[2] = tmp.157
            tmp.158 = - 4
            tmp.159 = truncate tmp.158
            inner.0[3] = tmp.159
            tmp.160 = - 5
            tmp.161 = truncate tmp.160
            inner.0[4] = tmp.161
            tmp.162 = - 6
            tmp.163 = truncate tmp.162
            outer.1[5] = tmp.163
            tmp.164 = - 7
            tmp.165 = truncate tmp.164
            outer.1[6] = tmp.165
            outermost.2[8] = 0
            tmp.166 = biggest_struct.9[0]
            small_struct.10 = tmp.166
            tmp.167 = small_struct.10[0]
            tmp.168 = sign_extend tmp.167
            tmp.170 = - 1
            tmp.169 = tmp.168 != tmp.170
            if tmp.169 jump or_true_36
            tmp.173 = small_struct.10[1]
            tmp.174 = sign_extend tmp.173
            tmp.176 = - 2
            tmp.175 = tmp.174 != tmp.176
            if tmp.175 jump or_true_36
            tmp.172 = 0
            jump or_end_37
        
          or_true_36:
            tmp.172 = 1
        
          or_end_37:
            if tmp.172 jump or_true_38
            tmp.179 = small_struct.10[2]
            tmp.180 = &tmp.179
            tmp.181 = sign_extend 0
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=1)
            tmp.183 = *tmp.182
            tmp.184 = sign_extend tmp.183
            tmp.186 = - 3
            tmp.185 = tmp.184 != tmp.186
            if tmp.185 jump or_true_38
            tmp.178 = 0
            jump or_end_39
        
          or_true_38:
            tmp.178 = 1
        
          or_end_39:
            if tmp.178 jump or_true_40
            tmp.189 = small_struct.10[2]
            tmp.190 = &tmp.189
            tmp.191 = sign_extend 1
            tmp.192 = add_ptr(tmp.190, index=tmp.191, scale=1)
            tmp.193 = *tmp.192
            tmp.194 = sign_extend tmp.193
            tmp.196 = - 4
            tmp.195 = tmp.194 != tmp.196
            if tmp.195 jump or_true_40
            tmp.188 = 0
            jump or_end_41
        
          or_true_40:
            tmp.188 = 1
        
          or_end_41:
            if tmp.188 jump or_true_42
            tmp.199 = small_struct.10[2]
            tmp.200 = &tmp.199
            tmp.201 = sign_extend 2
            tmp.202 = add_ptr(tmp.200, index=tmp.201, scale=1)
            tmp.203 = *tmp.202
            tmp.204 = sign_extend tmp.203
            tmp.206 = - 5
            tmp.205 = tmp.204 != tmp.206
            if tmp.205 jump or_true_42
            tmp.198 = 0
            jump or_end_43
        
          or_true_42:
            tmp.198 = 1
        
          or_end_43:
            if !tmp.198 jump end_if_44
            return 0
        
          end_if_44:
            return 1
            return 0
        }
        global function test_copy_to_nested_member() { 
            tmp.207 = truncate 0
            inner.0[0] = tmp.207
            tmp.208 = truncate 0
            inner.0[1] = tmp.208
            tmp.209 = truncate 0
            inner.0[2] = tmp.209
            tmp.210 = truncate 0
            inner.0[3] = tmp.210
            tmp.211 = truncate 0
            inner.0[4] = tmp.211
            tmp.212 = truncate 0
            outer.1[5] = tmp.212
            tmp.213 = truncate 0
            outer.1[6] = tmp.213
            tmp.214 = - 1
            outermost.2[8] = tmp.214
            biggest_struct.11[0] = small_struct.12
            tmp.215 = biggest_struct.11[0]
            tmp.216 = sign_extend tmp.215
            tmp.217 = tmp.216 != 50
            if tmp.217 jump or_true_46
            tmp.220 = biggest_struct.11[1]
            tmp.221 = sign_extend tmp.220
            tmp.222 = tmp.221 != 51
            if tmp.222 jump or_true_46
            tmp.219 = 0
            jump or_end_47
        
          or_true_46:
            tmp.219 = 1
        
          or_end_47:
            if tmp.219 jump or_true_48
            tmp.225 = biggest_struct.11[2]
            tmp.226 = &tmp.225
            tmp.227 = sign_extend 0
            tmp.228 = add_ptr(tmp.226, index=tmp.227, scale=1)
            tmp.229 = *tmp.228
            tmp.230 = sign_extend tmp.229
            tmp.231 = tmp.230 != 52
            if tmp.231 jump or_true_48
            tmp.224 = 0
            jump or_end_49
        
          or_true_48:
            tmp.224 = 1
        
          or_end_49:
            if tmp.224 jump or_true_50
            tmp.234 = biggest_struct.11[2]
            tmp.235 = &tmp.234
            tmp.236 = sign_extend 1
            tmp.237 = add_ptr(tmp.235, index=tmp.236, scale=1)
            tmp.238 = *tmp.237
            tmp.239 = sign_extend tmp.238
            tmp.240 = tmp.239 != 53
            if tmp.240 jump or_true_50
            tmp.233 = 0
            jump or_end_51
        
          or_true_50:
            tmp.233 = 1
        
          or_end_51:
            if tmp.233 jump or_true_52
            tmp.243 = biggest_struct.11[2]
            tmp.244 = &tmp.243
            tmp.245 = sign_extend 2
            tmp.246 = add_ptr(tmp.244, index=tmp.245, scale=1)
            tmp.247 = *tmp.246
            tmp.248 = sign_extend tmp.247
            tmp.249 = tmp.248 != 54
            if tmp.249 jump or_true_52
            tmp.242 = 0
            jump or_end_53
        
          or_true_52:
            tmp.242 = 1
        
          or_end_53:
            if !tmp.242 jump end_if_54
            return 0
        
          end_if_54:
            tmp.250 = biggest_struct.11[1]
            if tmp.250 jump or_true_56
            tmp.253 = biggest_struct.11[2]
            if tmp.253 jump or_true_56
            tmp.252 = 0
            jump or_end_57
        
          or_true_56:
            tmp.252 = 1
        
          or_end_57:
            if !tmp.252 jump end_if_58
            return 0
        
          end_if_58:
            tmp.254 = biggest_struct.11[1]
            tmp.256 = - 1
            tmp.255 = tmp.254 != tmp.256
            if !tmp.255 jump end_if_60
            return 0
        
          end_if_60:
            return 1
            return 0
        }
        global function test_copy_from_conditional() { 
            tmp.257 = truncate 127
            inner.0[0] = tmp.257
            tmp.258 = - 128
            tmp.259 = truncate tmp.258
            inner.0[1] = tmp.259
            tmp.260 = truncate 61
            inner.0[2] = tmp.260
            tmp.261 = truncate 62
            inner.0[3] = tmp.261
            tmp.262 = truncate 63
            inner.0[4] = tmp.262
            tmp.263 = - 10
            tmp.264 = truncate tmp.263
            outer.1[5] = tmp.264
            tmp.265 = - 11
            tmp.266 = truncate tmp.265
            outer.1[6] = tmp.266
            tmp.267 = truncate 0
            inner.0[0] = tmp.267
            tmp.268 = truncate 1
            inner.0[1] = tmp.268
            tmp.269 = truncate 2
            inner.0[2] = tmp.269
            tmp.270 = truncate 3
            inner.0[3] = tmp.270
            tmp.271 = truncate 4
            inner.0[4] = tmp.271
            tmp.272 = truncate 5
            outer.1[5] = tmp.272
            tmp.273 = truncate 6
            outer.1[6] = tmp.273
            if !f.16 jump else_63
            tmp.274 = big_struct.13
            jump end_if_62
        
          else_63:
            tmp.274 = big_struct2.14
        
          end_if_62:
            tmp.275 = tmp.274[0]
            small_struct.17 = tmp.275
            tmp.276 = small_struct.17[0]
            tmp.277 = sign_extend tmp.276
            tmp.278 = tmp.277 != 0
            if tmp.278 jump or_true_64
            tmp.281 = small_struct.17[1]
            tmp.282 = sign_extend tmp.281
            tmp.283 = tmp.282 != 1
            if tmp.283 jump or_true_64
            tmp.280 = 0
            jump or_end_65
        
          or_true_64:
            tmp.280 = 1
        
          or_end_65:
            if tmp.280 jump or_true_66
            tmp.286 = small_struct.17[2]
            tmp.287 = &tmp.286
            tmp.288 = sign_extend 0
            tmp.289 = add_ptr(tmp.287, index=tmp.288, scale=1)
            tmp.290 = *tmp.289
            tmp.291 = sign_extend tmp.290
            tmp.292 = tmp.291 != 2
            if tmp.292 jump or_true_66
            tmp.285 = 0
            jump or_end_67
        
          or_true_66:
            tmp.285 = 1
        
          or_end_67:
            if tmp.285 jump or_true_68
            tmp.295 = small_struct.17[2]
            tmp.296 = &tmp.295
            tmp.297 = sign_extend 1
            tmp.298 = add_ptr(tmp.296, index=tmp.297, scale=1)
            tmp.299 = *tmp.298
            tmp.300 = sign_extend tmp.299
            tmp.301 = tmp.300 != 3
            if tmp.301 jump or_true_68
            tmp.294 = 0
            jump or_end_69
        
          or_true_68:
            tmp.294 = 1
        
          or_end_69:
            if tmp.294 jump or_true_70
            tmp.304 = small_struct.17[2]
            tmp.305 = &tmp.304
            tmp.306 = sign_extend 2
            tmp.307 = add_ptr(tmp.305, index=tmp.306, scale=1)
            tmp.308 = *tmp.307
            tmp.309 = sign_extend tmp.308
            tmp.310 = tmp.309 != 4
            if tmp.310 jump or_true_70
            tmp.303 = 0
            jump or_end_71
        
          or_true_70:
            tmp.303 = 1
        
          or_end_71:
            if !tmp.303 jump end_if_72
            return 0
        
          end_if_72:
            if !t.15 jump else_75
            tmp.311 = big_struct.13
            jump end_if_74
        
          else_75:
            tmp.311 = big_struct2.14
        
          end_if_74:
            tmp.312 = tmp.311[0]
            small_struct.17 = tmp.312
            tmp.313 = small_struct.17[0]
            tmp.314 = sign_extend tmp.313
            tmp.315 = tmp.314 != 127
            if tmp.315 jump or_true_76
            tmp.318 = small_struct.17[1]
            tmp.319 = sign_extend tmp.318
            tmp.321 = - 128
            tmp.320 = tmp.319 != tmp.321
            if tmp.320 jump or_true_76
            tmp.317 = 0
            jump or_end_77
        
          or_true_76:
            tmp.317 = 1
        
          or_end_77:
            if tmp.317 jump or_true_78
            tmp.324 = small_struct.17[2]
            tmp.325 = &tmp.324
            tmp.326 = sign_extend 0
            tmp.327 = add_ptr(tmp.325, index=tmp.326, scale=1)
            tmp.328 = *tmp.327
            tmp.329 = sign_extend tmp.328
            tmp.330 = tmp.329 != 61
            if tmp.330 jump or_true_78
            tmp.323 = 0
            jump or_end_79
        
          or_true_78:
            tmp.323 = 1
        
          or_end_79:
            if tmp.323 jump or_true_80
            tmp.333 = small_struct.17[2]
            tmp.334 = &tmp.333
            tmp.335 = sign_extend 1
            tmp.336 = add_ptr(tmp.334, index=tmp.335, scale=1)
            tmp.337 = *tmp.336
            tmp.338 = sign_extend tmp.337
            tmp.339 = tmp.338 != 62
            if tmp.339 jump or_true_80
            tmp.332 = 0
            jump or_end_81
        
          or_true_80:
            tmp.332 = 1
        
          or_end_81:
            if tmp.332 jump or_true_82
            tmp.342 = small_struct.17[2]
            tmp.343 = &tmp.342
            tmp.344 = sign_extend 2
            tmp.345 = add_ptr(tmp.343, index=tmp.344, scale=1)
            tmp.346 = *tmp.345
            tmp.347 = sign_extend tmp.346
            tmp.348 = tmp.347 != 63
            if tmp.348 jump or_true_82
            tmp.341 = 0
            jump or_end_83
        
          or_true_82:
            tmp.341 = 1
        
          or_end_83:
            if !tmp.341 jump end_if_84
            return 0
        
          end_if_84:
            return 1
            return 0
        }
        global function test_copy_from_assignment() { 
            tmp.349 = truncate 127
            inner.0[0] = tmp.349
            tmp.350 = - 128
            tmp.351 = truncate tmp.350
            inner.0[1] = tmp.351
            tmp.352 = truncate 61
            inner.0[2] = tmp.352
            tmp.353 = truncate 62
            inner.0[3] = tmp.353
            tmp.354 = truncate 63
            inner.0[4] = tmp.354
            tmp.355 = - 10
            tmp.356 = truncate tmp.355
            outer.1[5] = tmp.356
            tmp.357 = - 11
            tmp.358 = truncate tmp.357
            outer.1[6] = tmp.358
            big_struct2.19 = big_struct.18
            tmp.359 = big_struct.18[0]
            small_struct.20 = tmp.359
            tmp.360 = small_struct.20[0]
            tmp.361 = sign_extend tmp.360
            tmp.362 = tmp.361 != 127
            if tmp.362 jump or_true_86
            tmp.365 = small_struct.20[1]
            tmp.366 = sign_extend tmp.365
            tmp.368 = - 128
            tmp.367 = tmp.366 != tmp.368
            if tmp.367 jump or_true_86
            tmp.364 = 0
            jump or_end_87
        
          or_true_86:
            tmp.364 = 1
        
          or_end_87:
            if tmp.364 jump or_true_88
            tmp.371 = small_struct.20[2]
            tmp.372 = &tmp.371
            tmp.373 = sign_extend 0
            tmp.374 = add_ptr(tmp.372, index=tmp.373, scale=1)
            tmp.375 = *tmp.374
            tmp.376 = sign_extend tmp.375
            tmp.377 = tmp.376 != 61
            if tmp.377 jump or_true_88
            tmp.370 = 0
            jump or_end_89
        
          or_true_88:
            tmp.370 = 1
        
          or_end_89:
            if tmp.370 jump or_true_90
            tmp.380 = small_struct.20[2]
            tmp.381 = &tmp.380
            tmp.382 = sign_extend 1
            tmp.383 = add_ptr(tmp.381, index=tmp.382, scale=1)
            tmp.384 = *tmp.383
            tmp.385 = sign_extend tmp.384
            tmp.386 = tmp.385 != 62
            if tmp.386 jump or_true_90
            tmp.379 = 0
            jump or_end_91
        
          or_true_90:
            tmp.379 = 1
        
          or_end_91:
            if tmp.379 jump or_true_92
            tmp.389 = small_struct.20[2]
            tmp.390 = &tmp.389
            tmp.391 = sign_extend 2
            tmp.392 = add_ptr(tmp.390, index=tmp.391, scale=1)
            tmp.393 = *tmp.392
            tmp.394 = sign_extend tmp.393
            tmp.395 = tmp.394 != 63
            if tmp.395 jump or_true_92
            tmp.388 = 0
            jump or_end_93
        
          or_true_92:
            tmp.388 = 1
        
          or_end_93:
            if !tmp.388 jump end_if_94
            return 0
        
          end_if_94:
            tmp.396 = big_struct2.19[0]
            tmp.397 = sign_extend tmp.396
            tmp.398 = tmp.397 != 127
            if tmp.398 jump or_true_96
            tmp.401 = big_struct2.19[1]
            tmp.402 = sign_extend tmp.401
            tmp.404 = - 128
            tmp.403 = tmp.402 != tmp.404
            if tmp.403 jump or_true_96
            tmp.400 = 0
            jump or_end_97
        
          or_true_96:
            tmp.400 = 1
        
          or_end_97:
            if tmp.400 jump or_true_98
            tmp.407 = big_struct2.19[2]
            tmp.408 = &tmp.407
            tmp.409 = sign_extend 0
            tmp.410 = add_ptr(tmp.408, index=tmp.409, scale=1)
            tmp.411 = *tmp.410
            tmp.412 = sign_extend tmp.411
            tmp.413 = tmp.412 != 61
            if tmp.413 jump or_true_98
            tmp.406 = 0
            jump or_end_99
        
          or_true_98:
            tmp.406 = 1
        
          or_end_99:
            if tmp.406 jump or_true_100
            tmp.416 = big_struct2.19[2]
            tmp.417 = &tmp.416
            tmp.418 = sign_extend 1
            tmp.419 = add_ptr(tmp.417, index=tmp.418, scale=1)
            tmp.420 = *tmp.419
            tmp.421 = sign_extend tmp.420
            tmp.422 = tmp.421 != 62
            if tmp.422 jump or_true_100
            tmp.415 = 0
            jump or_end_101
        
          or_true_100:
            tmp.415 = 1
        
          or_end_101:
            if tmp.415 jump or_true_102
            tmp.425 = big_struct2.19[2]
            tmp.426 = &tmp.425
            tmp.427 = sign_extend 2
            tmp.428 = add_ptr(tmp.426, index=tmp.427, scale=1)
            tmp.429 = *tmp.428
            tmp.430 = sign_extend tmp.429
            tmp.431 = tmp.430 != 63
            if tmp.431 jump or_true_102
            tmp.424 = 0
            jump or_end_103
        
          or_true_102:
            tmp.424 = 1
        
          or_end_103:
            if tmp.424 jump or_true_104
            tmp.434 = big_struct2.19[1]
            tmp.435 = sign_extend tmp.434
            tmp.437 = - 10
            tmp.436 = tmp.435 != tmp.437
            if tmp.436 jump or_true_104
            tmp.433 = 0
            jump or_end_105
        
          or_true_104:
            tmp.433 = 1
        
          or_end_105:
            if tmp.433 jump or_true_106
            tmp.440 = big_struct2.19[2]
            tmp.441 = sign_extend tmp.440
            tmp.443 = - 11
            tmp.442 = tmp.441 != tmp.443
            if tmp.442 jump or_true_106
            tmp.439 = 0
            jump or_end_107
        
          or_true_106:
            tmp.439 = 1
        
          or_end_107:
            if !tmp.439 jump end_if_108
            return 0
        
          end_if_108:
            return 1
            return 0
        }
        global function main() { 
            tmp.444 = test_copy_from_member()
            tmp.445 = ! tmp.444
            if !tmp.445 jump end_if_110
            return 1
        
          end_if_110:
            tmp.446 = test_copy_to_member()
            tmp.447 = ! tmp.446
            if !tmp.447 jump end_if_112
            return 2
        
          end_if_112:
            tmp.448 = test_copy_from_nested_member()
            tmp.449 = ! tmp.448
            if !tmp.449 jump end_if_114
            return 3
        
          end_if_114:
            tmp.450 = test_copy_to_nested_member()
            tmp.451 = ! tmp.450
            if !tmp.451 jump end_if_116
            return 4
        
          end_if_116:
            tmp.452 = test_copy_from_conditional()
            tmp.453 = ! tmp.452
            if !tmp.453 jump end_if_118
            return 6
        
          end_if_118:
            tmp.454 = test_copy_from_assignment()
            tmp.455 = ! tmp.454
            if !tmp.455 jump end_if_120
            return 7
        
          end_if_120:
            return 0
            return 0
        }
        static big_struct.3: Struct(outer.1) = [ '\n', '\t', '\u{8}', '\u{7}', '\u{6}', '\u{5}', '\u{4}']
        static big_struct.7: Struct(outer.1) = [ '\0', '\0', '\0', '\0', '\0', '\0', '\0']
        static big_struct2.19: Struct(outer.1) = zero[7]
        static f.16: Int = 0
        static small_struct.10: Struct(inner.0) = zero[5]
        static small_struct.12: Struct(inner.0) = [ '2', '3', '4', '5', '6']
        static small_struct.20: Struct(inner.0) = zero[5]
        static t.15: Int = 1
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_structure_parameters_struct_copy_stack_clobber() {
    let src = r#"
        void exit(int status);
        struct chars {
            char char_array[3];
        };
        static struct chars y = {{0, 1, 2}};
        static struct chars *ptr;
        void validate_array(char *char_array, int start, int code) {
            for (int i = 0; i < 3; i = i + 1) {
                if (char_array[i] != start + i) {
                    exit(code);
                }
            }
            return;
        }
        void increment_y(void) {
            y.char_array[0] = y.char_array[0] + 3;
            y.char_array[1] = y.char_array[1] + 3;
            y.char_array[2] = y.char_array[2] + 3;
        }
        int test_copy(void) {
            struct chars a = {"abc"};
            struct chars b = {"xyz"};
            struct chars c = {"def"};
            b = y;
            validate_array(a.char_array, 'a', 1);
            validate_array(b.char_array, 0, 2);
            validate_array(c.char_array, 'd', 3);
            return 0;
        }
        static struct chars to_validate;
        void validate_static(int start, int code) {
            validate_array(to_validate.char_array, start, code);
        }
        int test_load(void) {
            static struct chars b;
            struct chars a = {"ghi"};
            b = *ptr;
            to_validate = a;
            validate_static('g', 4);
            to_validate = b;
            validate_static(3, 5);
            return 0;
        }
        int test_store(void) {
            struct chars struct_array[3] = {{"jkl"}, {"xyz"}, {"mno"}};
            struct chars *ptr = &struct_array[1];
            *ptr = y;
            validate_array(struct_array[0].char_array, 'j', 6);
            validate_array(struct_array[1].char_array, 6, 7);
            validate_array(struct_array[2].char_array, 'm', 8);
            return 0;
        }
        struct chars_container {
            char c;
            struct chars chars;
            char arr[3];
        };
        int test_copy_from_offset(void) {
            struct chars a = {"pqr"};
            static struct chars b = {"xyz"};
            static struct chars_container container = {100, {{9, 10, 11}}, "123"};
            b = container.chars;
            to_validate = a;
            validate_static('p', 9);
            to_validate = b;
            validate_static(9, 10);
            return 0;
        }
        int test_copy_to_offset(void) {
            struct chars_container container = {
                'x', {{0, 0, 0}}, "stu"
            };
            container.chars = y;
            if (container.c != 'x') {
                exit(11);
            }
            validate_array(container.chars.char_array, 12, 12);
            validate_array(container.arr, 's', 13);
            return 0;
        }
        int main(void) {
            ptr = &y;
            test_copy();
            increment_y();
            test_load();
            increment_y();
            test_store();
            increment_y();
            test_copy_from_offset();
            increment_y();
            test_copy_to_offset();
            return 0;
        }
    "#;
    let expected = r#"
        global function validate_array(char_array.2, start.3, code.4) { 
            i.5 = 0
        
          start_loop_0:
            tmp.0 = i.5 < 3
            if !tmp.0 jump break_loop_0
            tmp.1 = sign_extend i.5
            tmp.2 = add_ptr(char_array.2, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.6 = start.3 + i.5
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_0
            exit(code.4)
        
          end_if_0:
        
          continue_loop_0:
            tmp.7 = i.5 + 1
            i.5 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            return 
        
            return 0
        }
        global function increment_y() { 
            tmp.8 = y[0]
            tmp.9 = &tmp.8
            tmp.10 = sign_extend 0
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=1)
            tmp.12 = y[0]
            tmp.13 = &tmp.12
            tmp.14 = sign_extend 0
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=1)
            tmp.16 = *tmp.15
            tmp.17 = sign_extend tmp.16
            tmp.18 = tmp.17 + 3
            tmp.19 = truncate tmp.18
            *tmp.11 = tmp.19
            tmp.20 = y[0]
            tmp.21 = &tmp.20
            tmp.22 = sign_extend 1
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=1)
            tmp.24 = y[0]
            tmp.25 = &tmp.24
            tmp.26 = sign_extend 1
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=1)
            tmp.28 = *tmp.27
            tmp.29 = sign_extend tmp.28
            tmp.30 = tmp.29 + 3
            tmp.31 = truncate tmp.30
            *tmp.23 = tmp.31
            tmp.32 = y[0]
            tmp.33 = &tmp.32
            tmp.34 = sign_extend 2
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=1)
            tmp.36 = y[0]
            tmp.37 = &tmp.36
            tmp.38 = sign_extend 2
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = sign_extend tmp.40
            tmp.42 = tmp.41 + 3
            tmp.43 = truncate tmp.42
            *tmp.35 = tmp.43
            return 0
        }
        global function test_copy() { 
            chars.1[0] = 'a'
            chars.1[1] = 'b'
            chars.1[2] = 'c'
            chars.1[0] = 'x'
            chars.1[1] = 'y'
            chars.1[2] = 'z'
            chars.1[0] = 'd'
            chars.1[1] = 'e'
            chars.1[2] = 'f'
            b.7 = y
            tmp.44 = a.6[0]
            tmp.45 = &tmp.44
            validate_array(tmp.45, 97, 1)
            tmp.46 = b.7[0]
            tmp.47 = &tmp.46
            validate_array(tmp.47, 0, 2)
            tmp.48 = c.8[0]
            tmp.49 = &tmp.48
            validate_array(tmp.49, 100, 3)
            return 0
            return 0
        }
        global function validate_static(start.9, code.10) { 
            tmp.50 = to_validate[0]
            tmp.51 = &tmp.50
            validate_array(tmp.51, start.9, code.10)
            return 0
        }
        global function test_load() { 
            chars.1[0] = 'g'
            chars.1[1] = 'h'
            chars.1[2] = 'i'
            tmp.52 = *ptr
            b.11 = tmp.52
            to_validate = a.12
            validate_static(103, 4)
            to_validate = b.11
            validate_static(3, 5)
            return 0
            return 0
        }
        global function test_store() { 
            chars.1[0] = 'j'
            chars.1[1] = 'k'
            chars.1[2] = 'l'
            chars.1[3] = 'x'
            chars.1[4] = 'y'
            chars.1[5] = 'z'
            chars.1[6] = 'm'
            chars.1[7] = 'n'
            chars.1[8] = 'o'
            tmp.53 = &struct_array.13
            tmp.54 = sign_extend 1
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=3)
            ptr.14 = tmp.55
            *ptr.14 = y
            tmp.56 = &struct_array.13
            tmp.57 = sign_extend 0
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=3)
            validate_array(tmp.58, 106, 6)
            tmp.59 = &struct_array.13
            tmp.60 = sign_extend 1
            tmp.61 = add_ptr(tmp.59, index=tmp.60, scale=3)
            validate_array(tmp.61, 6, 7)
            tmp.62 = &struct_array.13
            tmp.63 = sign_extend 2
            tmp.64 = add_ptr(tmp.62, index=tmp.63, scale=3)
            validate_array(tmp.64, 109, 8)
            return 0
            return 0
        }
        global function test_copy_from_offset() { 
            chars.1[0] = 'p'
            chars.1[1] = 'q'
            chars.1[2] = 'r'
            tmp.65 = container.18[1]
            b.17 = tmp.65
            to_validate = a.16
            validate_static(112, 9)
            to_validate = b.17
            validate_static(9, 10)
            return 0
            return 0
        }
        global function test_copy_to_offset() { 
            tmp.66 = truncate 120
            chars_container.15[0] = tmp.66
            tmp.67 = truncate 0
            chars.1[1] = tmp.67
            tmp.68 = truncate 0
            chars.1[2] = tmp.68
            tmp.69 = truncate 0
            chars.1[3] = tmp.69
            chars_container.15[4] = 's'
            chars_container.15[5] = 't'
            chars_container.15[6] = 'u'
            container.19[1] = y
            tmp.70 = container.19[0]
            tmp.71 = sign_extend tmp.70
            tmp.72 = tmp.71 != 120
            if !tmp.72 jump end_if_2
            exit(11)
        
          end_if_2:
            tmp.73 = container.19[1]
            tmp.74 = &tmp.73
            validate_array(tmp.74, 12, 12)
            tmp.75 = container.19[2]
            tmp.76 = &tmp.75
            validate_array(tmp.76, 115, 13)
            return 0
            return 0
        }
        global function main() { 
            tmp.77 = &y
            ptr = tmp.77
            tmp.78 = test_copy()
            increment_y()
            tmp.79 = test_load()
            increment_y()
            tmp.80 = test_store()
            increment_y()
            tmp.81 = test_copy_from_offset()
            increment_y()
            tmp.82 = test_copy_to_offset()
            return 0
            return 0
        }
        static b.11: Struct(chars.1) = zero[3]
        static b.17: Struct(chars.1) = "xyz"
        static container.18: Struct(chars_container.15) = [ 'd', '\t', '\n', '\u{b}', "123"]
        static ptr: Pointer(Struct(chars.1)) = zero[8]
        static to_validate: Struct(chars.1) = zero[3]
        static y: Struct(chars.1) = [ '\0', '\u{1}', '\u{2}']
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_incomplete_param_type() {
    let src = r#"
        struct s;
        int foo(struct s blah);
        struct s {
            int a;
            int b;
        };
        int main(void) {
            struct s arg = {1, 2};
            return foo(arg);
        }
        int foo(struct s blah) {
            return blah.a + blah.b;
        }
    "#;
    let expected = r#"
        global function main() { 
            s.0[0] = 1
            s.0[4] = 2
            tmp.0 = foo(arg.3)
            return tmp.0
            return 0
        }
        global function foo(blah.4) { 
            tmp.1 = blah.4[0]
            tmp.3 = blah.4[1]
            tmp.2 = tmp.1 + tmp.3
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_classify_params() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct twelve_bytes {
            int i;
            char arr[8];
        };
        struct inner {
            int i;
            char ch2;
        };
        struct nested_ints {
            char ch1;
            struct inner nested;
        };
        struct flattened_ints {
            char c;
            int i;
            char a;
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        struct two_ints {
            int i;
            int i2;
        };
        struct nested_double {
            double array[1];
        };
        struct two_eightbytes {
            double d;
            char c;
        };
        struct pass_in_memory {
            double w;
            double x;
            int y;
            long z;
        };
        int test_twelve_bytes(struct twelve_bytes s);
        int test_nested_ints(struct nested_ints s);
        int test_flattened_ints(struct flattened_ints s);
        int test_large(struct large s);
        int test_two_ints(struct two_ints s);
        int test_nested_double(struct nested_double s);
        int test_two_eightbytes(struct two_eightbytes s);
        int test_pass_in_memory(struct pass_in_memory s);
        
        int test_twelve_bytes(struct twelve_bytes s) {
            if (s.i != 0 || strcmp(s.arr, "lmnopqr")) {
                return 0;
            }
            return 1;
        }
        int test_nested_ints(struct nested_ints s) {
            if (s.ch1 != 127 || s.nested.i != 2147483647 || s.nested.ch2 != -128) {
                return 0;
            }
            return 1;
        }
        int test_flattened_ints(struct flattened_ints s) {
            if (s.c != 127 || s.i != 2147483647 || s.a != -128) {
                return 0;
            }
            return 1;
        }
        int test_large(struct large s) {
            if (s.i != 200000 || s.d != 23.25 || strcmp(s.arr, "abcdefghi")) {
                return 0;
            }
            return 1;
        }
        int test_two_ints(struct two_ints s) {
            if (s.i != 999 || s.i2 != 888) {
                return 0;
            }
            return 1;
        }
        int test_nested_double(struct nested_double s) {
            if (s.array[0] != 25.125e3) {
                return 0;
            }
            return 1;
        }
        int test_two_eightbytes(struct two_eightbytes s) {
            if (s.d != 1000. || s.c != 'x') {
                return 0;
            }
            return 1;
        }
        int test_pass_in_memory(struct pass_in_memory s) {
            if (s.w != 1.7e308 || s.x != -1.7e308 || s.y != -2147483647 ||
                s.z != -9223372036854775807l) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function test_twelve_bytes(s.19) { 
            tmp.0 = s.19[0]
            tmp.1 = tmp.0 != 0
            if tmp.1 jump or_true_0
            tmp.4 = s.19[1]
            tmp.5 = &tmp.4
            tmp.6 = &string.0
            tmp.7 = strcmp(tmp.5, tmp.6)
            if tmp.7 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if !tmp.3 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_nested_ints(s.20) { 
            tmp.8 = s.20[0]
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 127
            if tmp.10 jump or_true_4
            tmp.13 = s.20[1]
            tmp.14 = tmp.13 != 2147483647
            if tmp.14 jump or_true_4
            tmp.12 = 0
            jump or_end_5
        
          or_true_4:
            tmp.12 = 1
        
          or_end_5:
            if tmp.12 jump or_true_6
            tmp.17 = s.20[2]
            tmp.18 = sign_extend tmp.17
            tmp.20 = - 128
            tmp.19 = tmp.18 != tmp.20
            if tmp.19 jump or_true_6
            tmp.16 = 0
            jump or_end_7
        
          or_true_6:
            tmp.16 = 1
        
          or_end_7:
            if !tmp.16 jump end_if_8
            return 0
        
          end_if_8:
            return 1
            return 0
        }
        global function test_flattened_ints(s.21) { 
            tmp.21 = s.21[0]
            tmp.22 = sign_extend tmp.21
            tmp.23 = tmp.22 != 127
            if tmp.23 jump or_true_10
            tmp.26 = s.21[1]
            tmp.27 = tmp.26 != 2147483647
            if tmp.27 jump or_true_10
            tmp.25 = 0
            jump or_end_11
        
          or_true_10:
            tmp.25 = 1
        
          or_end_11:
            if tmp.25 jump or_true_12
            tmp.30 = s.21[2]
            tmp.31 = sign_extend tmp.30
            tmp.33 = - 128
            tmp.32 = tmp.31 != tmp.33
            if tmp.32 jump or_true_12
            tmp.29 = 0
            jump or_end_13
        
          or_true_12:
            tmp.29 = 1
        
          or_end_13:
            if !tmp.29 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_large(s.22) { 
            tmp.34 = s.22[0]
            tmp.35 = tmp.34 != 200000
            if tmp.35 jump or_true_16
            tmp.38 = s.22[1]
            tmp.39 = tmp.38 != 23.25D
            if tmp.39 jump or_true_16
            tmp.37 = 0
            jump or_end_17
        
          or_true_16:
            tmp.37 = 1
        
          or_end_17:
            if tmp.37 jump or_true_18
            tmp.42 = s.22[2]
            tmp.43 = &tmp.42
            tmp.44 = &string.1
            tmp.45 = strcmp(tmp.43, tmp.44)
            if tmp.45 jump or_true_18
            tmp.41 = 0
            jump or_end_19
        
          or_true_18:
            tmp.41 = 1
        
          or_end_19:
            if !tmp.41 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function test_two_ints(s.23) { 
            tmp.46 = s.23[0]
            tmp.47 = tmp.46 != 999
            if tmp.47 jump or_true_22
            tmp.50 = s.23[1]
            tmp.51 = tmp.50 != 888
            if tmp.51 jump or_true_22
            tmp.49 = 0
            jump or_end_23
        
          or_true_22:
            tmp.49 = 1
        
          or_end_23:
            if !tmp.49 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function test_nested_double(s.24) { 
            tmp.52 = s.24[0]
            tmp.53 = &tmp.52
            tmp.54 = sign_extend 0
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=8)
            tmp.56 = *tmp.55
            tmp.57 = tmp.56 != 25125D
            if !tmp.57 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function test_two_eightbytes(s.25) { 
            tmp.58 = s.25[0]
            tmp.59 = tmp.58 != 1000D
            if tmp.59 jump or_true_28
            tmp.62 = s.25[1]
            tmp.63 = sign_extend tmp.62
            tmp.64 = tmp.63 != 120
            if tmp.64 jump or_true_28
            tmp.61 = 0
            jump or_end_29
        
          or_true_28:
            tmp.61 = 1
        
          or_end_29:
            if !tmp.61 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_pass_in_memory(s.26) { 
            tmp.65 = s.26[0]
            tmp.66 = tmp.65 != 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            if tmp.66 jump or_true_32
            tmp.69 = s.26[1]
            tmp.71 = - 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            tmp.70 = tmp.69 != tmp.71
            if tmp.70 jump or_true_32
            tmp.68 = 0
            jump or_end_33
        
          or_true_32:
            tmp.68 = 1
        
          or_end_33:
            if tmp.68 jump or_true_34
            tmp.74 = s.26[2]
            tmp.76 = - 2147483647
            tmp.75 = tmp.74 != tmp.76
            if tmp.75 jump or_true_34
            tmp.73 = 0
            jump or_end_35
        
          or_true_34:
            tmp.73 = 1
        
          or_end_35:
            if tmp.73 jump or_true_36
            tmp.79 = s.26[3]
            tmp.81 = - 9223372036854775807L
            tmp.80 = tmp.79 != tmp.81
            if tmp.80 jump or_true_36
            tmp.78 = 0
            jump or_end_37
        
          or_true_36:
            tmp.78 = 1
        
          or_end_37:
            if !tmp.78 jump end_if_38
            return 0
        
          end_if_38:
            return 1
            return 0
        }
        constant string.0: Array(8,Char) = "lmnopqr\\0"
        constant string.1: Array(10,Char) = "abcdefghi\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_classify_params_client() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        struct twelve_bytes {
            int i;
            char arr[8];
        };
        struct inner {
            int i;
            char ch2;
        };
        struct nested_ints {
            char ch1;
            struct inner nested;
        };
        struct flattened_ints {
            char c;
            int i;
            char a;
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        struct two_ints {
            int i;
            int i2;
        };
        struct nested_double {
            double array[1];
        };
        struct two_eightbytes {
            double d;
            char c;
        };
        struct pass_in_memory {
            double w;
            double x;
            int y;
            long z;
        };
        int test_twelve_bytes(struct twelve_bytes s);
        int test_nested_ints(struct nested_ints s);
        int test_flattened_ints(struct flattened_ints s);
        int test_large(struct large s);
        int test_two_ints(struct two_ints s);
        int test_nested_double(struct nested_double s);
        int test_two_eightbytes(struct two_eightbytes s);
        int test_pass_in_memory(struct pass_in_memory s);
        
        int main(void) {
            struct twelve_bytes s1 = {0, "lmnopqr"};
            if (!test_twelve_bytes(s1)) {
                return 1;
            }
            struct nested_ints s2 = {127, {2147483647, -128}};
            if (!test_nested_ints(s2)) {
                return 2;
            }
            struct flattened_ints s3 = {127, 2147483647, -128};
            if (!test_flattened_ints(s3)) {
                return 3;
            }
            struct large s4 = {200000, 23.25, "abcdefghi"};
            if (!test_large(s4)) {
                return 4;
            }
            struct two_ints s5 = {999, 888};
            if (!test_two_ints(s5)) {
                return 5;
            }
            struct nested_double s6 = {{25.125e3}};
            if (!test_nested_double(s6)) {
                return 6;
            }
            struct two_eightbytes s7 = {1000., 'x'};
            if (!test_two_eightbytes(s7)) {
                return 7;
            }
            struct pass_in_memory s8 = {1.7e308, -1.7e308, -2147483647, -9223372036854775807l};
            if (!test_pass_in_memory(s8)) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            twelve_bytes.2[0] = 0
            twelve_bytes.2[4] = 'l'
            twelve_bytes.2[5] = 'm'
            twelve_bytes.2[6] = 'n'
            twelve_bytes.2[7] = 'o'
            twelve_bytes.2[8] = 'p'
            twelve_bytes.2[9] = 'q'
            twelve_bytes.2[10] = 'r'
            twelve_bytes.2[11] = '\0'
            tmp.0 = test_twelve_bytes(s1.19)
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = truncate 127
            nested_ints.4[0] = tmp.2
            inner.3[4] = 2147483647
            tmp.3 = - 128
            tmp.4 = truncate tmp.3
            inner.3[8] = tmp.4
            tmp.5 = test_nested_ints(s2.20)
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = truncate 127
            flattened_ints.5[0] = tmp.7
            flattened_ints.5[4] = 2147483647
            tmp.8 = - 128
            tmp.9 = truncate tmp.8
            flattened_ints.5[8] = tmp.9
            tmp.10 = test_flattened_ints(s3.21)
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            large.6[0] = 200000
            large.6[8] = 23.25D
            large.6[16] = 'a'
            large.6[17] = 'b'
            large.6[18] = 'c'
            large.6[19] = 'd'
            large.6[20] = 'e'
            large.6[21] = 'f'
            large.6[22] = 'g'
            large.6[23] = 'h'
            large.6[24] = 'i'
            large.6[25] = '\0'
            tmp.12 = test_large(s4.22)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            two_ints.7[0] = 999
            two_ints.7[4] = 888
            tmp.14 = test_two_ints(s5.23)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_8
            return 5
        
          end_if_8:
            nested_double.8[0] = 25125D
            tmp.16 = test_nested_double(s6.24)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_10
            return 6
        
          end_if_10:
            two_eightbytes.9[0] = 1000D
            tmp.18 = truncate 120
            two_eightbytes.9[8] = tmp.18
            tmp.19 = test_two_eightbytes(s7.25)
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_12
            return 7
        
          end_if_12:
            pass_in_memory.10[0] = 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            tmp.21 = - 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            pass_in_memory.10[8] = tmp.21
            tmp.22 = - 2147483647
            pass_in_memory.10[16] = tmp.22
            tmp.23 = - 9223372036854775807L
            pass_in_memory.10[24] = tmp.23
            tmp.24 = test_pass_in_memory(s8.26)
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_modify_param() {
    let src = r#"
        struct inner {
            double d;
            int i;
        };
        struct outer {
            struct inner s;
            struct inner *ptr;
            long l;
        };
        int modify_simple_struct(struct inner s);
        int modify_nested_struct(struct outer s);
        
        int modify_simple_struct(struct inner s) {
            struct inner copy = s;
            s.d = 0.0;
            if (s.d || s.i != 3) {
                return 0;
            }
            if (copy.d != 2.0 || copy.i != 3) {
                return 0;
            }
            return 1;
        }
        int modify_nested_struct(struct outer s) {
            struct outer copy = s;
            s.l = 10;
            s.s.i = 200;
            s.ptr->d = 10.0;
            s.ptr->i = 11;
            if (s.s.i != 200 || s.s.d != 4.0 || s.l != 10 || s.ptr->d != 10.0 ||
                s.ptr->i != 11) {
                return 0;
            }
            if (copy.s.i != 5 || copy.s.d != 4.0 || copy.l != 1000 ||
                copy.ptr->d != 10.0 || copy.ptr->i != 11) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function modify_simple_struct(s.4) { 
            copy.5 = s.4
            s.4[0] = 0D
            tmp.0 = s.4[0]
            if tmp.0 jump or_true_0
            tmp.3 = s.4[1]
            tmp.4 = tmp.3 != 3
            if tmp.4 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if !tmp.2 jump end_if_2
            return 0
        
          end_if_2:
            tmp.5 = copy.5[0]
            tmp.6 = tmp.5 != 2D
            if tmp.6 jump or_true_4
            tmp.9 = copy.5[1]
            tmp.10 = tmp.9 != 3
            if tmp.10 jump or_true_4
            tmp.8 = 0
            jump or_end_5
        
          or_true_4:
            tmp.8 = 1
        
          or_end_5:
            if !tmp.8 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function modify_nested_struct(s.6) { 
            copy.7 = s.6
            tmp.11 = sign_extend 10
            s.6[2] = tmp.11
            s.6[1] = 200
            tmp.12 = s.6[1]
            tmp.12 = 10D
            tmp.13 = s.6[1]
            tmp.14 = add_ptr(tmp.13, index=1L, scale=1)
            tmp.14 = 11
            tmp.15 = s.6[1]
            tmp.16 = tmp.15 != 200
            if tmp.16 jump or_true_8
            tmp.19 = s.6[0]
            tmp.20 = tmp.19 != 4D
            if tmp.20 jump or_true_8
            tmp.18 = 0
            jump or_end_9
        
          or_true_8:
            tmp.18 = 1
        
          or_end_9:
            if tmp.18 jump or_true_10
            tmp.23 = s.6[2]
            tmp.25 = sign_extend 10
            tmp.24 = tmp.23 != tmp.25
            if tmp.24 jump or_true_10
            tmp.22 = 0
            jump or_end_11
        
          or_true_10:
            tmp.22 = 1
        
          or_end_11:
            if tmp.22 jump or_true_12
            tmp.28 = s.6[1]
            tmp.29 = tmp.28 != 10D
            if tmp.29 jump or_true_12
            tmp.27 = 0
            jump or_end_13
        
          or_true_12:
            tmp.27 = 1
        
          or_end_13:
            if tmp.27 jump or_true_14
            tmp.32 = s.6[1]
            tmp.33 = add_ptr(tmp.32, index=1L, scale=1)
            tmp.34 = tmp.33 != 11
            if tmp.34 jump or_true_14
            tmp.31 = 0
            jump or_end_15
        
          or_true_14:
            tmp.31 = 1
        
          or_end_15:
            if !tmp.31 jump end_if_16
            return 0
        
          end_if_16:
            tmp.35 = copy.7[1]
            tmp.36 = tmp.35 != 5
            if tmp.36 jump or_true_18
            tmp.39 = copy.7[0]
            tmp.40 = tmp.39 != 4D
            if tmp.40 jump or_true_18
            tmp.38 = 0
            jump or_end_19
        
          or_true_18:
            tmp.38 = 1
        
          or_end_19:
            if tmp.38 jump or_true_20
            tmp.43 = copy.7[2]
            tmp.45 = sign_extend 1000
            tmp.44 = tmp.43 != tmp.45
            if tmp.44 jump or_true_20
            tmp.42 = 0
            jump or_end_21
        
          or_true_20:
            tmp.42 = 1
        
          or_end_21:
            if tmp.42 jump or_true_22
            tmp.48 = copy.7[1]
            tmp.49 = tmp.48 != 10D
            if tmp.49 jump or_true_22
            tmp.47 = 0
            jump or_end_23
        
          or_true_22:
            tmp.47 = 1
        
          or_end_23:
            if tmp.47 jump or_true_24
            tmp.52 = copy.7[1]
            tmp.53 = add_ptr(tmp.52, index=1L, scale=1)
            tmp.54 = tmp.53 != 11
            if tmp.54 jump or_true_24
            tmp.51 = 0
            jump or_end_25
        
          or_true_24:
            tmp.51 = 1
        
          or_end_25:
            if !tmp.51 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_modify_param_client() {
    let src = r#"
        struct inner {
            double d;
            int i;
        };
        struct outer {
            struct inner s;
            struct inner *ptr;
            long l;
        };
        int modify_simple_struct(struct inner s);
        int modify_nested_struct(struct outer s);
        
        int main(void) {
            struct inner s_inner = {2.0, 3};
            if (!modify_simple_struct(s_inner)) {
                return 1;
            }
            if (s_inner.d != 2.0 || s_inner.i != 3) {
                return 2;
            }
            struct outer s_o = {{4.0, 5}, &s_inner, 1000l};
            if (!modify_nested_struct(s_o)) {
                return 3;
            }
            if (s_o.s.d != 4.0 || s_o.s.i != 5 || s_o.l != 1000l) {
                return 4;
            }
            if (s_o.ptr != &s_inner) {
                return 5;
            }
            if (s_o.ptr->d != 10.0 || s_o.ptr->i != 11) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            inner.0[0] = 2D
            inner.0[8] = 3
            tmp.0 = modify_simple_struct(s_inner.4)
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = s_inner.4[0]
            tmp.3 = tmp.2 != 2D
            if tmp.3 jump or_true_2
            tmp.6 = s_inner.4[1]
            tmp.7 = tmp.6 != 3
            if tmp.7 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            if !tmp.5 jump end_if_4
            return 2
        
          end_if_4:
            inner.0[0] = 4D
            inner.0[8] = 5
            tmp.8 = &s_inner.4
            outer.1[16] = tmp.8
            outer.1[24] = 1000L
            tmp.9 = modify_nested_struct(s_o.5)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_6
            return 3
        
          end_if_6:
            tmp.11 = s_o.5[0]
            tmp.12 = tmp.11 != 4D
            if tmp.12 jump or_true_8
            tmp.15 = s_o.5[1]
            tmp.16 = tmp.15 != 5
            if tmp.16 jump or_true_8
            tmp.14 = 0
            jump or_end_9
        
          or_true_8:
            tmp.14 = 1
        
          or_end_9:
            if tmp.14 jump or_true_10
            tmp.19 = s_o.5[2]
            tmp.20 = tmp.19 != 1000L
            if tmp.20 jump or_true_10
            tmp.18 = 0
            jump or_end_11
        
          or_true_10:
            tmp.18 = 1
        
          or_end_11:
            if !tmp.18 jump end_if_12
            return 4
        
          end_if_12:
            tmp.21 = s_o.5[1]
            tmp.23 = &s_inner.4
            tmp.22 = tmp.21 != tmp.23
            if !tmp.22 jump end_if_14
            return 5
        
          end_if_14:
            tmp.24 = s_o.5[1]
            tmp.25 = tmp.24 != 10D
            if tmp.25 jump or_true_16
            tmp.28 = s_o.5[1]
            tmp.29 = add_ptr(tmp.28, index=1L, scale=1)
            tmp.30 = tmp.29 != 11
            if tmp.30 jump or_true_16
            tmp.27 = 0
            jump or_end_17
        
          or_true_16:
            tmp.27 = 1
        
          or_end_17:
            if !tmp.27 jump end_if_18
            return 6
        
          end_if_18:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_param_calling_conventions() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int strncmp(char *s1, char *s2, unsigned long n);
        struct two_longs {
            long a;
            long b;
        };
        struct one_int {
            int i;
            char c;
        };
        struct one_int_exactly {
            unsigned long l;
        };
        struct two_ints {
            char c;
            int arr[3];
        };
        struct two_ints_nested {
            struct one_int a;
            struct one_int b;
        };
        struct twelve_bytes {
            int i;
            char arr[8];
        };
        struct one_xmm {
            double d;
        };
        struct two_xmm {
            double d[2];
        };
        struct int_and_xmm {
            char c;
            double d;
        };
        struct xmm_and_int {
            struct one_xmm dbl;
            char c[3];
        };
        struct odd_size {
            char arr[5];
        };
        struct memory {
            double d;
            char c[3];
            long l;
            int i;
        };
        int pass_small_structs(struct two_xmm two_xmm_struct, struct one_int int_struct,
                               struct one_xmm xmm_struct,
                               struct xmm_and_int mixed_struct,
                               struct twelve_bytes int_struct_2,
                               struct one_int_exactly another_int_struct);
        int a_bunch_of_arguments(int i0, int i1, int i2, int i3, int i4,
                                 struct two_longs param, int i5);
        int structs_and_scalars(long l, double d, struct odd_size os, struct memory mem,
                                struct one_xmm xmm_struct);
        int struct_in_mem(double a, double b, double c, struct xmm_and_int first_struct,
                          double d, struct two_xmm second_struct, long l,
                          struct int_and_xmm third_struct,
                          struct one_xmm fourth_struct);
        int pass_borderline_struct_in_memory(struct two_ints t_i, int c,
                                             struct int_and_xmm i_x, void *ptr,
                                             struct two_ints_nested t_i_n, double d);
        int pass_uneven_struct_in_mem(struct twelve_bytes struct1, long a, long b,
                                      struct twelve_bytes struct2, struct odd_size os,
                                      struct memory m);
        int pass_later_structs_in_regs(struct memory m, struct twelve_bytes struct1, struct one_xmm struct2);
        int pass_small_structs(struct two_xmm two_xmm_struct, struct one_int int_struct,
                               struct one_xmm xmm_struct,
                               struct xmm_and_int mixed_struct,
                               struct twelve_bytes int_struct_2,
                               struct one_int_exactly another_int_struct) {
            if (two_xmm_struct.d[0] != 55.5 || two_xmm_struct.d[1] != 44.4)
                return 0;
            if (int_struct.c != 'c' || int_struct.i != 54320)
                return 0;
            if (xmm_struct.d != 5.125)
                return 0;
            if (strcmp(mixed_struct.c, "hi") || mixed_struct.dbl.d != 1.234)
                return 0;
            if (strcmp(int_struct_2.arr, "string!") || int_struct_2.i != 123)
                return 0;
            if (another_int_struct.l != 567890)
                return 0;
            return 1;
        }
        int a_bunch_of_arguments(int i0, int i1, int i2, int i3, int i4,
                                 struct two_longs param, int i5) {
            if (i0 != 0 || i1 != 1 || i2 != 2 || i3 != 3 || i4 != 4 || i5 != 5) {
                return 0;
            }
            if (param.a != 1234567l || param.b != 89101112l) {
                return 0;
            }
            return 1;
        }
        int structs_and_scalars(long l, double d, struct odd_size os, struct memory mem,
                                struct one_xmm xmm_struct) {
            if (l != 10)
                return 0;
            if (d != 10.0)
                return 0;
            if (strcmp(os.arr, "lmno"))
                return 0;
            if (strcmp(mem.c, "rs") || mem.d != 15.75 || mem.i != 3333 || mem.l != 4444)
                return 0;
            if (xmm_struct.d != 5.125)
                return 0;
            return 1;
        }
        int struct_in_mem(double a, double b, double c, struct xmm_and_int first_struct,
                          double d, struct two_xmm second_struct, long l,
                          struct int_and_xmm third_struct,
                          struct one_xmm fourth_struct) {
            if (a != 10.0 || b != 11.125 || c != 12.0)
                return 0;
            if (strcmp(first_struct.c, "hi") || first_struct.dbl.d != 1.234)
                return 0;
            if (d != 13.0)
                return 0;
            if (second_struct.d[0] != 55.5 || second_struct.d[1] != 44.4)
                return 0;
            if (l)
                return 0;
            if (third_struct.c != 'p' || third_struct.d != 4.56)
                return 0;
            if (fourth_struct.d != 5.125)
                return 0;
            return 1;
        }
        int pass_borderline_struct_in_memory(struct two_ints t_i, int c,
                                             struct int_and_xmm i_x, void *ptr,
                                             struct two_ints_nested t_i_n, double d) {
            if (t_i.c != '_' || t_i.arr[0] != 5 || t_i.arr[1] != 6 || t_i.arr[2] != 7)
                return 0;
            if (c != '!')
                return 0;
            if (i_x.c != 'p' || i_x.d != 4.56)
                return 0;
            if (ptr)
                return 0;
            if (t_i_n.a.c != 'c' || t_i_n.a.i != 54320)
                return 0;
            if (t_i_n.b.c != 'c' || t_i_n.b.i != 54320)
                return 0;
            if (d != 7.8)
                return 0;
            return 1;
        }
        int pass_uneven_struct_in_mem(struct twelve_bytes struct1, long a, long b,
                                      struct twelve_bytes struct2, struct odd_size os,
                                      struct memory m) {
            if (struct1.i != -1) {
                return 0;
            }
            if (struct1.arr[0] != 127 || struct1.arr[1] != 126 ||
                struct1.arr[2] != 125) {
                return 0;
            }
            if (a != 9223372036854775805l || b != 9223372036854775800l) {
                return 0;
            }
            if (struct2.i != -5) {
                return 0;
            }
            if (struct2.arr[0] != 100 || struct2.arr[1] != 101 ||
                struct2.arr[2] != 102) {
                return 0;
            }
            for (int i = 0; i < 5; i = i + 1) {
                if (os.arr[i] != 100 - i) {
                    return 0;
                }
            }
            if (m.d != 5.345) {
                return 0;
            }
            if (m.c[0] != -1 || m.c[1] != -2 || m.c[2] != -3) {
                return 0;
            }
            if (m.l != 4294967300l) {
                return 0;
            }
            if (m.i != 10000) {
                return 0;
            }
            return 1;
        }
        int pass_later_structs_in_regs(struct memory m, struct twelve_bytes struct1,
                                       struct one_xmm struct2) {
            if (m.d != 5.345) {
                return 0;
            }
            if (m.c[0] != -1 || m.c[1] != -2 || m.c[2] != -3) {
                return 0;
            }
            if (m.l != 4294967300l) {
                return 0;
            }
            if (m.i != 10000) {
                return 0;
            }
            if (struct1.i != -1) {
                return 0;
            }
            if (struct1.arr[0] != 127 || struct1.arr[1] != 126 ||
                struct1.arr[2] != 125) {
                return 0;
            }
            if (struct2.d != 5.125) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function pass_small_structs(two_xmm_struct.59, int_struct.60, xmm_struct.61, mixed_struct.62, int_struct_2.63, another_int_struct.64) { 
            tmp.0 = two_xmm_struct.59[0]
            tmp.1 = &tmp.0
            tmp.2 = sign_extend 0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=8)
            tmp.4 = *tmp.3
            tmp.5 = tmp.4 != 55.5D
            if tmp.5 jump or_true_0
            tmp.8 = two_xmm_struct.59[0]
            tmp.9 = &tmp.8
            tmp.10 = sign_extend 1
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=8)
            tmp.12 = *tmp.11
            tmp.13 = tmp.12 != 44.4D
            if tmp.13 jump or_true_0
            tmp.7 = 0
            jump or_end_1
        
          or_true_0:
            tmp.7 = 1
        
          or_end_1:
            if !tmp.7 jump end_if_2
            return 0
        
          end_if_2:
            tmp.14 = int_struct.60[1]
            tmp.15 = sign_extend tmp.14
            tmp.16 = tmp.15 != 99
            if tmp.16 jump or_true_4
            tmp.19 = int_struct.60[0]
            tmp.20 = tmp.19 != 54320
            if tmp.20 jump or_true_4
            tmp.18 = 0
            jump or_end_5
        
          or_true_4:
            tmp.18 = 1
        
          or_end_5:
            if !tmp.18 jump end_if_6
            return 0
        
          end_if_6:
            tmp.21 = xmm_struct.61[0]
            tmp.22 = tmp.21 != 5.125D
            if !tmp.22 jump end_if_8
            return 0
        
          end_if_8:
            tmp.23 = mixed_struct.62[1]
            tmp.24 = &tmp.23
            tmp.25 = &string.0
            tmp.26 = strcmp(tmp.24, tmp.25)
            if tmp.26 jump or_true_10
            tmp.29 = mixed_struct.62[0]
            tmp.30 = tmp.29 != 1.234D
            if tmp.30 jump or_true_10
            tmp.28 = 0
            jump or_end_11
        
          or_true_10:
            tmp.28 = 1
        
          or_end_11:
            if !tmp.28 jump end_if_12
            return 0
        
          end_if_12:
            tmp.31 = int_struct_2.63[1]
            tmp.32 = &tmp.31
            tmp.33 = &string.1
            tmp.34 = strcmp(tmp.32, tmp.33)
            if tmp.34 jump or_true_14
            tmp.37 = int_struct_2.63[0]
            tmp.38 = tmp.37 != 123
            if tmp.38 jump or_true_14
            tmp.36 = 0
            jump or_end_15
        
          or_true_14:
            tmp.36 = 1
        
          or_end_15:
            if !tmp.36 jump end_if_16
            return 0
        
          end_if_16:
            tmp.39 = another_int_struct.64[0]
            tmp.41 = sign_extend 567890
            tmp.40 = tmp.39 != tmp.41
            if !tmp.40 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function a_bunch_of_arguments(i0.65, i1.66, i2.67, i3.68, i4.69, param.70, i5.71) { 
            tmp.42 = i0.65 != 0
            if tmp.42 jump or_true_20
            tmp.45 = i1.66 != 1
            if tmp.45 jump or_true_20
            tmp.44 = 0
            jump or_end_21
        
          or_true_20:
            tmp.44 = 1
        
          or_end_21:
            if tmp.44 jump or_true_22
            tmp.48 = i2.67 != 2
            if tmp.48 jump or_true_22
            tmp.47 = 0
            jump or_end_23
        
          or_true_22:
            tmp.47 = 1
        
          or_end_23:
            if tmp.47 jump or_true_24
            tmp.51 = i3.68 != 3
            if tmp.51 jump or_true_24
            tmp.50 = 0
            jump or_end_25
        
          or_true_24:
            tmp.50 = 1
        
          or_end_25:
            if tmp.50 jump or_true_26
            tmp.54 = i4.69 != 4
            if tmp.54 jump or_true_26
            tmp.53 = 0
            jump or_end_27
        
          or_true_26:
            tmp.53 = 1
        
          or_end_27:
            if tmp.53 jump or_true_28
            tmp.57 = i5.71 != 5
            if tmp.57 jump or_true_28
            tmp.56 = 0
            jump or_end_29
        
          or_true_28:
            tmp.56 = 1
        
          or_end_29:
            if !tmp.56 jump end_if_30
            return 0
        
          end_if_30:
            tmp.58 = param.70[0]
            tmp.59 = tmp.58 != 1234567L
            if tmp.59 jump or_true_32
            tmp.62 = param.70[1]
            tmp.63 = tmp.62 != 89101112L
            if tmp.63 jump or_true_32
            tmp.61 = 0
            jump or_end_33
        
          or_true_32:
            tmp.61 = 1
        
          or_end_33:
            if !tmp.61 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function structs_and_scalars(l.72, d.73, os.74, mem.75, xmm_struct.76) { 
            tmp.65 = sign_extend 10
            tmp.64 = l.72 != tmp.65
            if !tmp.64 jump end_if_36
            return 0
        
          end_if_36:
            tmp.66 = d.73 != 10D
            if !tmp.66 jump end_if_38
            return 0
        
          end_if_38:
            tmp.67 = os.74[0]
            tmp.68 = &tmp.67
            tmp.69 = &string.2
            tmp.70 = strcmp(tmp.68, tmp.69)
            if !tmp.70 jump end_if_40
            return 0
        
          end_if_40:
            tmp.71 = mem.75[1]
            tmp.72 = &tmp.71
            tmp.73 = &string.3
            tmp.74 = strcmp(tmp.72, tmp.73)
            if tmp.74 jump or_true_42
            tmp.77 = mem.75[0]
            tmp.78 = tmp.77 != 15.75D
            if tmp.78 jump or_true_42
            tmp.76 = 0
            jump or_end_43
        
          or_true_42:
            tmp.76 = 1
        
          or_end_43:
            if tmp.76 jump or_true_44
            tmp.81 = mem.75[3]
            tmp.82 = tmp.81 != 3333
            if tmp.82 jump or_true_44
            tmp.80 = 0
            jump or_end_45
        
          or_true_44:
            tmp.80 = 1
        
          or_end_45:
            if tmp.80 jump or_true_46
            tmp.85 = mem.75[2]
            tmp.87 = sign_extend 4444
            tmp.86 = tmp.85 != tmp.87
            if tmp.86 jump or_true_46
            tmp.84 = 0
            jump or_end_47
        
          or_true_46:
            tmp.84 = 1
        
          or_end_47:
            if !tmp.84 jump end_if_48
            return 0
        
          end_if_48:
            tmp.88 = xmm_struct.76[0]
            tmp.89 = tmp.88 != 5.125D
            if !tmp.89 jump end_if_50
            return 0
        
          end_if_50:
            return 1
            return 0
        }
        global function struct_in_mem(a.77, b.78, c.79, first_struct.80, d.81, second_struct.82, l.83, third_struct.84, fourth_struct.85) { 
            tmp.90 = a.77 != 10D
            if tmp.90 jump or_true_52
            tmp.93 = b.78 != 11.125D
            if tmp.93 jump or_true_52
            tmp.92 = 0
            jump or_end_53
        
          or_true_52:
            tmp.92 = 1
        
          or_end_53:
            if tmp.92 jump or_true_54
            tmp.96 = c.79 != 12D
            if tmp.96 jump or_true_54
            tmp.95 = 0
            jump or_end_55
        
          or_true_54:
            tmp.95 = 1
        
          or_end_55:
            if !tmp.95 jump end_if_56
            return 0
        
          end_if_56:
            tmp.97 = first_struct.80[1]
            tmp.98 = &tmp.97
            tmp.99 = &string.0
            tmp.100 = strcmp(tmp.98, tmp.99)
            if tmp.100 jump or_true_58
            tmp.103 = first_struct.80[0]
            tmp.104 = tmp.103 != 1.234D
            if tmp.104 jump or_true_58
            tmp.102 = 0
            jump or_end_59
        
          or_true_58:
            tmp.102 = 1
        
          or_end_59:
            if !tmp.102 jump end_if_60
            return 0
        
          end_if_60:
            tmp.105 = d.81 != 13D
            if !tmp.105 jump end_if_62
            return 0
        
          end_if_62:
            tmp.106 = second_struct.82[0]
            tmp.107 = &tmp.106
            tmp.108 = sign_extend 0
            tmp.109 = add_ptr(tmp.107, index=tmp.108, scale=8)
            tmp.110 = *tmp.109
            tmp.111 = tmp.110 != 55.5D
            if tmp.111 jump or_true_64
            tmp.114 = second_struct.82[0]
            tmp.115 = &tmp.114
            tmp.116 = sign_extend 1
            tmp.117 = add_ptr(tmp.115, index=tmp.116, scale=8)
            tmp.118 = *tmp.117
            tmp.119 = tmp.118 != 44.4D
            if tmp.119 jump or_true_64
            tmp.113 = 0
            jump or_end_65
        
          or_true_64:
            tmp.113 = 1
        
          or_end_65:
            if !tmp.113 jump end_if_66
            return 0
        
          end_if_66:
            if !l.83 jump end_if_68
            return 0
        
          end_if_68:
            tmp.120 = third_struct.84[0]
            tmp.121 = sign_extend tmp.120
            tmp.122 = tmp.121 != 112
            if tmp.122 jump or_true_70
            tmp.125 = third_struct.84[1]
            tmp.126 = tmp.125 != 4.56D
            if tmp.126 jump or_true_70
            tmp.124 = 0
            jump or_end_71
        
          or_true_70:
            tmp.124 = 1
        
          or_end_71:
            if !tmp.124 jump end_if_72
            return 0
        
          end_if_72:
            tmp.127 = fourth_struct.85[0]
            tmp.128 = tmp.127 != 5.125D
            if !tmp.128 jump end_if_74
            return 0
        
          end_if_74:
            return 1
            return 0
        }
        global function pass_borderline_struct_in_memory(t_i.86, c.87, i_x.88, ptr.89, t_i_n.90, d.91) { 
            tmp.129 = t_i.86[0]
            tmp.130 = sign_extend tmp.129
            tmp.131 = tmp.130 != 95
            if tmp.131 jump or_true_76
            tmp.134 = t_i.86[1]
            tmp.135 = &tmp.134
            tmp.136 = sign_extend 0
            tmp.137 = add_ptr(tmp.135, index=tmp.136, scale=4)
            tmp.138 = *tmp.137
            tmp.139 = tmp.138 != 5
            if tmp.139 jump or_true_76
            tmp.133 = 0
            jump or_end_77
        
          or_true_76:
            tmp.133 = 1
        
          or_end_77:
            if tmp.133 jump or_true_78
            tmp.142 = t_i.86[1]
            tmp.143 = &tmp.142
            tmp.144 = sign_extend 1
            tmp.145 = add_ptr(tmp.143, index=tmp.144, scale=4)
            tmp.146 = *tmp.145
            tmp.147 = tmp.146 != 6
            if tmp.147 jump or_true_78
            tmp.141 = 0
            jump or_end_79
        
          or_true_78:
            tmp.141 = 1
        
          or_end_79:
            if tmp.141 jump or_true_80
            tmp.150 = t_i.86[1]
            tmp.151 = &tmp.150
            tmp.152 = sign_extend 2
            tmp.153 = add_ptr(tmp.151, index=tmp.152, scale=4)
            tmp.154 = *tmp.153
            tmp.155 = tmp.154 != 7
            if tmp.155 jump or_true_80
            tmp.149 = 0
            jump or_end_81
        
          or_true_80:
            tmp.149 = 1
        
          or_end_81:
            if !tmp.149 jump end_if_82
            return 0
        
          end_if_82:
            tmp.156 = c.87 != 33
            if !tmp.156 jump end_if_84
            return 0
        
          end_if_84:
            tmp.157 = i_x.88[0]
            tmp.158 = sign_extend tmp.157
            tmp.159 = tmp.158 != 112
            if tmp.159 jump or_true_86
            tmp.162 = i_x.88[1]
            tmp.163 = tmp.162 != 4.56D
            if tmp.163 jump or_true_86
            tmp.161 = 0
            jump or_end_87
        
          or_true_86:
            tmp.161 = 1
        
          or_end_87:
            if !tmp.161 jump end_if_88
            return 0
        
          end_if_88:
            if !ptr.89 jump end_if_90
            return 0
        
          end_if_90:
            tmp.164 = t_i_n.90[1]
            tmp.165 = sign_extend tmp.164
            tmp.166 = tmp.165 != 99
            if tmp.166 jump or_true_92
            tmp.169 = t_i_n.90[0]
            tmp.170 = tmp.169 != 54320
            if tmp.170 jump or_true_92
            tmp.168 = 0
            jump or_end_93
        
          or_true_92:
            tmp.168 = 1
        
          or_end_93:
            if !tmp.168 jump end_if_94
            return 0
        
          end_if_94:
            tmp.171 = t_i_n.90[2]
            tmp.172 = sign_extend tmp.171
            tmp.173 = tmp.172 != 99
            if tmp.173 jump or_true_96
            tmp.176 = t_i_n.90[1]
            tmp.177 = tmp.176 != 54320
            if tmp.177 jump or_true_96
            tmp.175 = 0
            jump or_end_97
        
          or_true_96:
            tmp.175 = 1
        
          or_end_97:
            if !tmp.175 jump end_if_98
            return 0
        
          end_if_98:
            tmp.178 = d.91 != 7.8D
            if !tmp.178 jump end_if_100
            return 0
        
          end_if_100:
            return 1
            return 0
        }
        global function pass_uneven_struct_in_mem(struct1.92, a.93, b.94, struct2.95, os.96, m.97) { 
            tmp.179 = struct1.92[0]
            tmp.181 = - 1
            tmp.180 = tmp.179 != tmp.181
            if !tmp.180 jump end_if_102
            return 0
        
          end_if_102:
            tmp.182 = struct1.92[1]
            tmp.183 = &tmp.182
            tmp.184 = sign_extend 0
            tmp.185 = add_ptr(tmp.183, index=tmp.184, scale=1)
            tmp.186 = *tmp.185
            tmp.187 = sign_extend tmp.186
            tmp.188 = tmp.187 != 127
            if tmp.188 jump or_true_104
            tmp.191 = struct1.92[1]
            tmp.192 = &tmp.191
            tmp.193 = sign_extend 1
            tmp.194 = add_ptr(tmp.192, index=tmp.193, scale=1)
            tmp.195 = *tmp.194
            tmp.196 = sign_extend tmp.195
            tmp.197 = tmp.196 != 126
            if tmp.197 jump or_true_104
            tmp.190 = 0
            jump or_end_105
        
          or_true_104:
            tmp.190 = 1
        
          or_end_105:
            if tmp.190 jump or_true_106
            tmp.200 = struct1.92[1]
            tmp.201 = &tmp.200
            tmp.202 = sign_extend 2
            tmp.203 = add_ptr(tmp.201, index=tmp.202, scale=1)
            tmp.204 = *tmp.203
            tmp.205 = sign_extend tmp.204
            tmp.206 = tmp.205 != 125
            if tmp.206 jump or_true_106
            tmp.199 = 0
            jump or_end_107
        
          or_true_106:
            tmp.199 = 1
        
          or_end_107:
            if !tmp.199 jump end_if_108
            return 0
        
          end_if_108:
            tmp.207 = a.93 != 9223372036854775805L
            if tmp.207 jump or_true_110
            tmp.210 = b.94 != 9223372036854775800L
            if tmp.210 jump or_true_110
            tmp.209 = 0
            jump or_end_111
        
          or_true_110:
            tmp.209 = 1
        
          or_end_111:
            if !tmp.209 jump end_if_112
            return 0
        
          end_if_112:
            tmp.211 = struct2.95[0]
            tmp.213 = - 5
            tmp.212 = tmp.211 != tmp.213
            if !tmp.212 jump end_if_114
            return 0
        
          end_if_114:
            tmp.214 = struct2.95[1]
            tmp.215 = &tmp.214
            tmp.216 = sign_extend 0
            tmp.217 = add_ptr(tmp.215, index=tmp.216, scale=1)
            tmp.218 = *tmp.217
            tmp.219 = sign_extend tmp.218
            tmp.220 = tmp.219 != 100
            if tmp.220 jump or_true_116
            tmp.223 = struct2.95[1]
            tmp.224 = &tmp.223
            tmp.225 = sign_extend 1
            tmp.226 = add_ptr(tmp.224, index=tmp.225, scale=1)
            tmp.227 = *tmp.226
            tmp.228 = sign_extend tmp.227
            tmp.229 = tmp.228 != 101
            if tmp.229 jump or_true_116
            tmp.222 = 0
            jump or_end_117
        
          or_true_116:
            tmp.222 = 1
        
          or_end_117:
            if tmp.222 jump or_true_118
            tmp.232 = struct2.95[1]
            tmp.233 = &tmp.232
            tmp.234 = sign_extend 2
            tmp.235 = add_ptr(tmp.233, index=tmp.234, scale=1)
            tmp.236 = *tmp.235
            tmp.237 = sign_extend tmp.236
            tmp.238 = tmp.237 != 102
            if tmp.238 jump or_true_118
            tmp.231 = 0
            jump or_end_119
        
          or_true_118:
            tmp.231 = 1
        
          or_end_119:
            if !tmp.231 jump end_if_120
            return 0
        
          end_if_120:
            i.98 = 0
        
          start_loop_0:
            tmp.239 = i.98 < 5
            if !tmp.239 jump break_loop_0
            tmp.240 = os.96[0]
            tmp.241 = &tmp.240
            tmp.242 = sign_extend i.98
            tmp.243 = add_ptr(tmp.241, index=tmp.242, scale=1)
            tmp.244 = *tmp.243
            tmp.245 = sign_extend tmp.244
            tmp.247 = 100 - i.98
            tmp.246 = tmp.245 != tmp.247
            if !tmp.246 jump end_if_122
            return 0
        
          end_if_122:
        
          continue_loop_0:
            tmp.248 = i.98 + 1
            i.98 = tmp.248
            jump start_loop_0
        
          break_loop_0:
            tmp.249 = m.97[0]
            tmp.250 = tmp.249 != 5.345D
            if !tmp.250 jump end_if_124
            return 0
        
          end_if_124:
            tmp.251 = m.97[1]
            tmp.252 = &tmp.251
            tmp.253 = sign_extend 0
            tmp.254 = add_ptr(tmp.252, index=tmp.253, scale=1)
            tmp.255 = *tmp.254
            tmp.256 = sign_extend tmp.255
            tmp.258 = - 1
            tmp.257 = tmp.256 != tmp.258
            if tmp.257 jump or_true_126
            tmp.261 = m.97[1]
            tmp.262 = &tmp.261
            tmp.263 = sign_extend 1
            tmp.264 = add_ptr(tmp.262, index=tmp.263, scale=1)
            tmp.265 = *tmp.264
            tmp.266 = sign_extend tmp.265
            tmp.268 = - 2
            tmp.267 = tmp.266 != tmp.268
            if tmp.267 jump or_true_126
            tmp.260 = 0
            jump or_end_127
        
          or_true_126:
            tmp.260 = 1
        
          or_end_127:
            if tmp.260 jump or_true_128
            tmp.271 = m.97[1]
            tmp.272 = &tmp.271
            tmp.273 = sign_extend 2
            tmp.274 = add_ptr(tmp.272, index=tmp.273, scale=1)
            tmp.275 = *tmp.274
            tmp.276 = sign_extend tmp.275
            tmp.278 = - 3
            tmp.277 = tmp.276 != tmp.278
            if tmp.277 jump or_true_128
            tmp.270 = 0
            jump or_end_129
        
          or_true_128:
            tmp.270 = 1
        
          or_end_129:
            if !tmp.270 jump end_if_130
            return 0
        
          end_if_130:
            tmp.279 = m.97[2]
            tmp.280 = tmp.279 != 4294967300L
            if !tmp.280 jump end_if_132
            return 0
        
          end_if_132:
            tmp.281 = m.97[3]
            tmp.282 = tmp.281 != 10000
            if !tmp.282 jump end_if_134
            return 0
        
          end_if_134:
            return 1
            return 0
        }
        global function pass_later_structs_in_regs(m.99, struct1.100, struct2.101) { 
            tmp.283 = m.99[0]
            tmp.284 = tmp.283 != 5.345D
            if !tmp.284 jump end_if_136
            return 0
        
          end_if_136:
            tmp.285 = m.99[1]
            tmp.286 = &tmp.285
            tmp.287 = sign_extend 0
            tmp.288 = add_ptr(tmp.286, index=tmp.287, scale=1)
            tmp.289 = *tmp.288
            tmp.290 = sign_extend tmp.289
            tmp.292 = - 1
            tmp.291 = tmp.290 != tmp.292
            if tmp.291 jump or_true_138
            tmp.295 = m.99[1]
            tmp.296 = &tmp.295
            tmp.297 = sign_extend 1
            tmp.298 = add_ptr(tmp.296, index=tmp.297, scale=1)
            tmp.299 = *tmp.298
            tmp.300 = sign_extend tmp.299
            tmp.302 = - 2
            tmp.301 = tmp.300 != tmp.302
            if tmp.301 jump or_true_138
            tmp.294 = 0
            jump or_end_139
        
          or_true_138:
            tmp.294 = 1
        
          or_end_139:
            if tmp.294 jump or_true_140
            tmp.305 = m.99[1]
            tmp.306 = &tmp.305
            tmp.307 = sign_extend 2
            tmp.308 = add_ptr(tmp.306, index=tmp.307, scale=1)
            tmp.309 = *tmp.308
            tmp.310 = sign_extend tmp.309
            tmp.312 = - 3
            tmp.311 = tmp.310 != tmp.312
            if tmp.311 jump or_true_140
            tmp.304 = 0
            jump or_end_141
        
          or_true_140:
            tmp.304 = 1
        
          or_end_141:
            if !tmp.304 jump end_if_142
            return 0
        
          end_if_142:
            tmp.313 = m.99[2]
            tmp.314 = tmp.313 != 4294967300L
            if !tmp.314 jump end_if_144
            return 0
        
          end_if_144:
            tmp.315 = m.99[3]
            tmp.316 = tmp.315 != 10000
            if !tmp.316 jump end_if_146
            return 0
        
          end_if_146:
            tmp.317 = struct1.100[0]
            tmp.319 = - 1
            tmp.318 = tmp.317 != tmp.319
            if !tmp.318 jump end_if_148
            return 0
        
          end_if_148:
            tmp.320 = struct1.100[1]
            tmp.321 = &tmp.320
            tmp.322 = sign_extend 0
            tmp.323 = add_ptr(tmp.321, index=tmp.322, scale=1)
            tmp.324 = *tmp.323
            tmp.325 = sign_extend tmp.324
            tmp.326 = tmp.325 != 127
            if tmp.326 jump or_true_150
            tmp.329 = struct1.100[1]
            tmp.330 = &tmp.329
            tmp.331 = sign_extend 1
            tmp.332 = add_ptr(tmp.330, index=tmp.331, scale=1)
            tmp.333 = *tmp.332
            tmp.334 = sign_extend tmp.333
            tmp.335 = tmp.334 != 126
            if tmp.335 jump or_true_150
            tmp.328 = 0
            jump or_end_151
        
          or_true_150:
            tmp.328 = 1
        
          or_end_151:
            if tmp.328 jump or_true_152
            tmp.338 = struct1.100[1]
            tmp.339 = &tmp.338
            tmp.340 = sign_extend 2
            tmp.341 = add_ptr(tmp.339, index=tmp.340, scale=1)
            tmp.342 = *tmp.341
            tmp.343 = sign_extend tmp.342
            tmp.344 = tmp.343 != 125
            if tmp.344 jump or_true_152
            tmp.337 = 0
            jump or_end_153
        
          or_true_152:
            tmp.337 = 1
        
          or_end_153:
            if !tmp.337 jump end_if_154
            return 0
        
          end_if_154:
            tmp.345 = struct2.101[0]
            tmp.346 = tmp.345 != 5.125D
            if !tmp.346 jump end_if_156
            return 0
        
          end_if_156:
            return 1
            return 0
        }
        constant string.0: Array(3,Char) = "hi\\0"
        constant string.1: Array(8,Char) = "string!\\0"
        constant string.2: Array(5,Char) = "lmno\\0"
        constant string.3: Array(3,Char) = "rs\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_param_calling_conventions_client() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int strncmp(char *s1, char *s2, unsigned long n);
        struct two_longs {
            long a;
            long b;
        };
        struct one_int {
            int i;
            char c;
        };
        struct one_int_exactly {
            unsigned long l;
        };
        struct two_ints {
            char c;
            int arr[3];
        };
        struct two_ints_nested {
            struct one_int a;
            struct one_int b;
        };
        struct twelve_bytes {
            int i;
            char arr[8];
        };
        struct one_xmm {
            double d;
        };
        struct two_xmm {
            double d[2];
        };
        struct int_and_xmm {
            char c;
            double d;
        };
        struct xmm_and_int {
            struct one_xmm dbl;
            char c[3];
        };
        struct odd_size {
            char arr[5];
        };
        struct memory {
            double d;
            char c[3];
            long l;
            int i;
        };
        int pass_small_structs(struct two_xmm two_xmm_struct, struct one_int int_struct,
                               struct one_xmm xmm_struct,
                               struct xmm_and_int mixed_struct,
                               struct twelve_bytes int_struct_2,
                               struct one_int_exactly another_int_struct);
        int a_bunch_of_arguments(int i0, int i1, int i2, int i3, int i4,
                                 struct two_longs param, int i5);
        int structs_and_scalars(long l, double d, struct odd_size os, struct memory mem,
                                struct one_xmm xmm_struct);
        int struct_in_mem(double a, double b, double c, struct xmm_and_int first_struct,
                          double d, struct two_xmm second_struct, long l,
                          struct int_and_xmm third_struct,
                          struct one_xmm fourth_struct);
        int pass_borderline_struct_in_memory(struct two_ints t_i, int c,
                                             struct int_and_xmm i_x, void *ptr,
                                             struct two_ints_nested t_i_n, double d);
        int pass_uneven_struct_in_mem(struct twelve_bytes struct1, long a, long b,
                                      struct twelve_bytes struct2, struct odd_size os,
                                      struct memory m);
        int pass_later_structs_in_regs(struct memory m, struct twelve_bytes struct1, struct one_xmm struct2);
        
        int main(void) {
            struct two_longs two_longs = {1234567l, 89101112l};
            struct one_int one_int = {54320, 'c'};
            struct one_int_exactly one_long = {567890l};
            struct two_ints two_ints = {'_', {5, 6, 7}};
            struct two_ints_nested two_ints_nested = {one_int, one_int};
            struct twelve_bytes xii = {123, "string!"};
            struct one_xmm one_xmm = {5.125};
            struct two_xmm two_xmm = {{55.5, 44.4}};
            struct int_and_xmm int_and_xmm = {'p', 4.56};
            struct xmm_and_int xmm_and_int = {{1.234}, "hi"};
            struct odd_size odd = {"lmno"};
            struct memory mem = {15.75, "rs", 4444, 3333};
            if (!pass_small_structs(two_xmm, one_int, one_xmm, xmm_and_int, xii,
                                    one_long)) {
                return 1;
            }
            if (!a_bunch_of_arguments(0, 1, 2, 3, 4, two_longs, 5)) {
                return 2;
            }
            if (!structs_and_scalars(10, 10.0, odd, mem, one_xmm)) {
                return 2;
            }
            if (!struct_in_mem(10.0, 11.125, 12.0, xmm_and_int, 13.0, two_xmm, 0,
                               int_and_xmm, one_xmm)) {
                return 3;
            }
            if (!pass_borderline_struct_in_memory(two_ints, '!', int_and_xmm, 0,
                                                  two_ints_nested, 7.8)) {
                return 4;
            }
            struct twelve_bytes struct1 = {-1, {127, 126, 125}};
            struct twelve_bytes struct2 = {-5, {100, 101, 102}};
            struct odd_size os = {{100, 99, 98, 97, 96}};
            struct memory m = {5.345, {-1, -2, -3}, 4294967300l, 10000};
            if (!pass_uneven_struct_in_mem(struct1, 9223372036854775805l,
                                           9223372036854775800l, struct2, os, m)) {
                return 5;
            }
            if (!pass_later_structs_in_regs(m, struct1, one_xmm)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            two_longs.5[0] = 1234567L
            two_longs.5[8] = 89101112L
            one_int.6[0] = 54320
            tmp.0 = truncate 99
            one_int.6[4] = tmp.0
            tmp.1 = 567890L
            one_int_exactly.7[0] = tmp.1
            tmp.2 = truncate 95
            two_ints.8[0] = tmp.2
            two_ints.8[4] = 5
            two_ints.8[8] = 6
            two_ints.8[12] = 7
            two_ints_nested.9[0] = one_int.60
            two_ints_nested.9[8] = one_int.60
            twelve_bytes.10[0] = 123
            twelve_bytes.10[4] = 's'
            twelve_bytes.10[5] = 't'
            twelve_bytes.10[6] = 'r'
            twelve_bytes.10[7] = 'i'
            twelve_bytes.10[8] = 'n'
            twelve_bytes.10[9] = 'g'
            twelve_bytes.10[10] = '!'
            twelve_bytes.10[11] = '\0'
            one_xmm.11[0] = 5.125D
            two_xmm.12[0] = 55.5D
            two_xmm.12[8] = 44.4D
            tmp.3 = truncate 112
            int_and_xmm.13[0] = tmp.3
            int_and_xmm.13[8] = 4.56D
            one_xmm.11[0] = 1.234D
            xmm_and_int.14[8] = 'h'
            xmm_and_int.14[9] = 'i'
            xmm_and_int.14[10] = '\0'
            odd_size.15[0] = 'l'
            odd_size.15[1] = 'm'
            odd_size.15[2] = 'n'
            odd_size.15[3] = 'o'
            odd_size.15[4] = '\0'
            memory.16[0] = 15.75D
            memory.16[8] = 'r'
            memory.16[9] = 's'
            memory.16[10] = '\0'
            tmp.4 = sign_extend 4444
            memory.16[16] = tmp.4
            memory.16[24] = 3333
            tmp.5 = pass_small_structs(two_xmm.66, one_int.60, one_xmm.65, xmm_and_int.68, xii.64, one_long.61)
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = a_bunch_of_arguments(0, 1, 2, 3, 4, two_longs.59, 5)
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = sign_extend 10
            tmp.10 = structs_and_scalars(tmp.9, 10D, odd.69, mem.70, one_xmm.65)
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_4
            return 2
        
          end_if_4:
            tmp.12 = sign_extend 0
            tmp.13 = struct_in_mem(10D, 11.125D, 12D, xmm_and_int.68, 13D, two_xmm.66, tmp.12, int_and_xmm.67, one_xmm.65)
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_6
            return 3
        
          end_if_6:
            tmp.15 = sign_extend 0
            tmp.16 = pass_borderline_struct_in_memory(two_ints.62, 33, int_and_xmm.67, tmp.15, two_ints_nested.63, 7.8D)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_8
            return 4
        
          end_if_8:
            tmp.18 = - 1
            twelve_bytes.10[0] = tmp.18
            tmp.19 = truncate 127
            twelve_bytes.10[4] = tmp.19
            tmp.20 = truncate 126
            twelve_bytes.10[5] = tmp.20
            tmp.21 = truncate 125
            twelve_bytes.10[6] = tmp.21
            twelve_bytes.10[7] = '\0'
            twelve_bytes.10[8] = '\0'
            twelve_bytes.10[9] = '\0'
            twelve_bytes.10[10] = '\0'
            twelve_bytes.10[11] = '\0'
            tmp.22 = - 5
            twelve_bytes.10[0] = tmp.22
            tmp.23 = truncate 100
            twelve_bytes.10[4] = tmp.23
            tmp.24 = truncate 101
            twelve_bytes.10[5] = tmp.24
            tmp.25 = truncate 102
            twelve_bytes.10[6] = tmp.25
            twelve_bytes.10[7] = '\0'
            twelve_bytes.10[8] = '\0'
            twelve_bytes.10[9] = '\0'
            twelve_bytes.10[10] = '\0'
            twelve_bytes.10[11] = '\0'
            tmp.26 = truncate 100
            odd_size.15[0] = tmp.26
            tmp.27 = truncate 99
            odd_size.15[1] = tmp.27
            tmp.28 = truncate 98
            odd_size.15[2] = tmp.28
            tmp.29 = truncate 97
            odd_size.15[3] = tmp.29
            tmp.30 = truncate 96
            odd_size.15[4] = tmp.30
            memory.16[0] = 5.345D
            tmp.31 = - 1
            tmp.32 = truncate tmp.31
            memory.16[8] = tmp.32
            tmp.33 = - 2
            tmp.34 = truncate tmp.33
            memory.16[9] = tmp.34
            tmp.35 = - 3
            tmp.36 = truncate tmp.35
            memory.16[10] = tmp.36
            memory.16[16] = 4294967300L
            memory.16[24] = 10000
            tmp.37 = pass_uneven_struct_in_mem(struct1.71, 9223372036854775805L, 9223372036854775800L, struct2.72, os.73, m.74)
            tmp.38 = ! tmp.37
            if !tmp.38 jump end_if_10
            return 5
        
          end_if_10:
            tmp.39 = pass_later_structs_in_regs(m.74, struct1.71, one_xmm.65)
            tmp.40 = ! tmp.39
            if !tmp.40 jump end_if_12
            return 6
        
          end_if_12:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_pass_struct() {
    let src = r#"
        struct pair {
            int x;
            int y;
        };
        int validate_struct_param(struct pair p);
        
        int validate_struct_param(struct pair p) {
            if (p.x != 1 || p.y != 2) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function validate_struct_param(p.2) { 
            tmp.0 = p.2[0]
            tmp.1 = tmp.0 != 1
            if tmp.1 jump or_true_0
            tmp.4 = p.2[1]
            tmp.5 = tmp.4 != 2
            if tmp.5 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if !tmp.3 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_pass_struct_client() {
    let src = r#"
        struct pair {
            int x;
            int y;
        };
        int validate_struct_param(struct pair p);
        
        int main(void) {
            struct pair arg = {1, 2};
            if (!validate_struct_param(arg)) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            pair.0[0] = 1
            pair.0[4] = 2
            tmp.0 = validate_struct_param(arg.2)
            tmp.1 = ! tmp.0
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
fn test_valid_parameters_libraries_struct_sizes() {
    let src = r#"
        struct bytesize1 {
            unsigned char arr[1];
        };
        extern struct bytesize1 globvar_1;
        struct bytesize2 {
            unsigned char arr[2];
        };
        extern struct bytesize2 globvar_2;
        struct bytesize3 {
            unsigned char arr[3];
        };
        extern struct bytesize3 globvar_3;
        struct bytesize4 {
            unsigned char arr[4];
        };
        extern struct bytesize4 globvar_4;
        struct bytesize5 {
            unsigned char arr[5];
        };
        extern struct bytesize5 globvar_5;
        struct bytesize6 {
            unsigned char arr[6];
        };
        extern struct bytesize6 globvar_6;
        struct bytesize7 {
            unsigned char arr[7];
        };
        extern struct bytesize7 globvar_7;
        struct bytesize8 {
            unsigned char arr[8];
        };
        extern struct bytesize8 globvar_8;
        struct bytesize9 {
            unsigned char arr[9];
        };
        extern struct bytesize9 globvar_9;
        struct bytesize10 {
            unsigned char arr[10];
        };
        extern struct bytesize10 globvar_10;
        struct bytesize11 {
            unsigned char arr[11];
        };
        extern struct bytesize11 globvar_11;
        struct bytesize12 {
            unsigned char arr[12];
        };
        extern struct bytesize12 globvar_12;
        struct bytesize13 {
            unsigned char arr[13];
        };
        extern struct bytesize13 globvar_13;
        struct bytesize14 {
            unsigned char arr[14];
        };
        extern struct bytesize14 globvar_14;
        struct bytesize15 {
            unsigned char arr[15];
        };
        extern struct bytesize15 globvar_15;
        struct bytesize16 {
            unsigned char arr[16];
        };
        extern struct bytesize16 globvar_16;
        struct bytesize17 {
            unsigned char arr[17];
        };
        extern struct bytesize17 globvar_17;
        struct bytesize18 {
            unsigned char arr[18];
        };
        extern struct bytesize18 globvar_18;
        struct bytesize19 {
            unsigned char arr[19];
        };
        extern struct bytesize19 globvar_19;
        struct bytesize20 {
            unsigned char arr[20];
        };
        extern struct bytesize20 globvar_20;
        struct bytesize21 {
            unsigned char arr[21];
        };
        extern struct bytesize21 globvar_21;
        struct bytesize22 {
            unsigned char arr[22];
        };
        extern struct bytesize22 globvar_22;
        struct bytesize23 {
            unsigned char arr[23];
        };
        extern struct bytesize23 globvar_23;
        struct bytesize24 {
            unsigned char arr[24];
        };
        extern struct bytesize24 globvar_24;
        int fun0(struct bytesize1 a, struct bytesize2 b, struct bytesize3 c,
                 struct bytesize4 d, struct bytesize5 e, struct bytesize6 f,
                 struct bytesize7 g, struct bytesize8 h, struct bytesize9 i,
                 struct bytesize10 j, struct bytesize11 k, struct bytesize12 l,
                 struct bytesize13 m, struct bytesize14 n, struct bytesize15 o,
                 struct bytesize16 p, struct bytesize17 q, struct bytesize18 r,
                 struct bytesize19 s, struct bytesize20 t, struct bytesize21 u,
                 struct bytesize22 v, struct bytesize23 w, struct bytesize24 x,
                 unsigned char *a_expected, unsigned char *b_expected,
                 unsigned char *c_expected, unsigned char *d_expected,
                 unsigned char *e_expected, unsigned char *f_expected,
                 unsigned char *g_expected, unsigned char *h_expected,
                 unsigned char *i_expected, unsigned char *j_expected,
                 unsigned char *k_expected, unsigned char *l_expected,
                 unsigned char *m_expected, unsigned char *n_expected,
                 unsigned char *o_expected, unsigned char *p_expected,
                 unsigned char *q_expected, unsigned char *r_expected,
                 unsigned char *s_expected, unsigned char *t_expected,
                 unsigned char *u_expected, unsigned char *v_expected,
                 unsigned char *w_expected, unsigned char *x_expected);
        int fun1(struct bytesize7 a, struct bytesize8 b, struct bytesize9 c,
                 struct bytesize10 d, struct bytesize1 e, struct bytesize2 f,
                 struct bytesize3 g, struct bytesize4 h, struct bytesize5 i,
                 struct bytesize6 j, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected, unsigned char *e_expected,
                 unsigned char *f_expected, unsigned char *g_expected,
                 unsigned char *h_expected, unsigned char *i_expected,
                 unsigned char *j_expected);
        int fun2(struct bytesize11 a, struct bytesize12 b, struct bytesize13 c,
                 struct bytesize1 d, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected);
        int fun3(struct bytesize14 a, struct bytesize15 b, struct bytesize16 c,
                 struct bytesize2 d, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected);
        int memcmp(void *s1, void *s2, unsigned long n);
        int fun0(struct bytesize1 a, struct bytesize2 b, struct bytesize3 c,
                 struct bytesize4 d, struct bytesize5 e, struct bytesize6 f,
                 struct bytesize7 g, struct bytesize8 h, struct bytesize9 i,
                 struct bytesize10 j, struct bytesize11 k, struct bytesize12 l,
                 struct bytesize13 m, struct bytesize14 n, struct bytesize15 o,
                 struct bytesize16 p, struct bytesize17 q, struct bytesize18 r,
                 struct bytesize19 s, struct bytesize20 t, struct bytesize21 u,
                 struct bytesize22 v, struct bytesize23 w, struct bytesize24 x,
                 unsigned char *a_expected, unsigned char *b_expected,
                 unsigned char *c_expected, unsigned char *d_expected,
                 unsigned char *e_expected, unsigned char *f_expected,
                 unsigned char *g_expected, unsigned char *h_expected,
                 unsigned char *i_expected, unsigned char *j_expected,
                 unsigned char *k_expected, unsigned char *l_expected,
                 unsigned char *m_expected, unsigned char *n_expected,
                 unsigned char *o_expected, unsigned char *p_expected,
                 unsigned char *q_expected, unsigned char *r_expected,
                 unsigned char *s_expected, unsigned char *t_expected,
                 unsigned char *u_expected, unsigned char *v_expected,
                 unsigned char *w_expected, unsigned char *x_expected) {
            if (memcmp(&a, a_expected, sizeof a)) {
                return 0;
            }
            if (memcmp(&b, b_expected, sizeof b)) {
                return 0;
            }
            if (memcmp(&c, c_expected, sizeof c)) {
                return 0;
            }
            if (memcmp(&d, d_expected, sizeof d)) {
                return 0;
            }
            if (memcmp(&e, e_expected, sizeof e)) {
                return 0;
            }
            if (memcmp(&f, f_expected, sizeof f)) {
                return 0;
            }
            if (memcmp(&g, g_expected, sizeof g)) {
                return 0;
            }
            if (memcmp(&h, h_expected, sizeof h)) {
                return 0;
            }
            if (memcmp(&i, i_expected, sizeof i)) {
                return 0;
            }
            if (memcmp(&j, j_expected, sizeof j)) {
                return 0;
            }
            if (memcmp(&k, k_expected, sizeof k)) {
                return 0;
            }
            if (memcmp(&l, l_expected, sizeof l)) {
                return 0;
            }
            if (memcmp(&m, m_expected, sizeof m)) {
                return 0;
            }
            if (memcmp(&n, n_expected, sizeof n)) {
                return 0;
            }
            if (memcmp(&o, o_expected, sizeof o)) {
                return 0;
            }
            if (memcmp(&p, p_expected, sizeof p)) {
                return 0;
            }
            if (memcmp(&q, q_expected, sizeof q)) {
                return 0;
            }
            if (memcmp(&r, r_expected, sizeof r)) {
                return 0;
            }
            if (memcmp(&s, s_expected, sizeof s)) {
                return 0;
            }
            if (memcmp(&t, t_expected, sizeof t)) {
                return 0;
            }
            if (memcmp(&u, u_expected, sizeof u)) {
                return 0;
            }
            if (memcmp(&v, v_expected, sizeof v)) {
                return 0;
            }
            if (memcmp(&w, w_expected, sizeof w)) {
                return 0;
            }
            if (memcmp(&x, x_expected, sizeof x)) {
                return 0;
            }
            return 1;
        }
        int fun1(struct bytesize7 a, struct bytesize8 b, struct bytesize9 c,
                 struct bytesize10 d, struct bytesize1 e, struct bytesize2 f,
                 struct bytesize3 g, struct bytesize4 h, struct bytesize5 i,
                 struct bytesize6 j, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected, unsigned char *e_expected,
                 unsigned char *f_expected, unsigned char *g_expected,
                 unsigned char *h_expected, unsigned char *i_expected,
                 unsigned char *j_expected) {
            if (memcmp(&a, a_expected, sizeof a)) {
                return 0;
            }
            if (memcmp(&b, b_expected, sizeof b)) {
                return 0;
            }
            if (memcmp(&c, c_expected, sizeof c)) {
                return 0;
            }
            if (memcmp(&d, d_expected, sizeof d)) {
                return 0;
            }
            if (memcmp(&e, e_expected, sizeof e)) {
                return 0;
            }
            if (memcmp(&f, f_expected, sizeof f)) {
                return 0;
            }
            if (memcmp(&g, g_expected, sizeof g)) {
                return 0;
            }
            if (memcmp(&h, h_expected, sizeof h)) {
                return 0;
            }
            if (memcmp(&i, i_expected, sizeof i)) {
                return 0;
            }
            if (memcmp(&j, j_expected, sizeof j)) {
                return 0;
            }
            return 1;
        }
        int fun2(struct bytesize11 a, struct bytesize12 b, struct bytesize13 c,
                 struct bytesize1 d, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected) {
            if (memcmp(&a, a_expected, sizeof a)) {
                return 0;
            }
            if (memcmp(&b, b_expected, sizeof b)) {
                return 0;
            }
            if (memcmp(&c, c_expected, sizeof c)) {
                return 0;
            }
            if (memcmp(&d, d_expected, sizeof d)) {
                return 0;
            }
            return 1;
        }
        int fun3(struct bytesize14 a, struct bytesize15 b, struct bytesize16 c,
                 struct bytesize2 d, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected) {
            if (memcmp(&a, a_expected, sizeof a)) {
                return 0;
            }
            if (memcmp(&b, b_expected, sizeof b)) {
                return 0;
            }
            if (memcmp(&c, c_expected, sizeof c)) {
                return 0;
            }
            if (memcmp(&d, d_expected, sizeof d)) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function fun0(a.111, b.112, c.113, d.114, e.115, f.116, g.117, h.118, i.119, j.120, k.121, l.122, m.123, n.124, o.125, p.126, q.127, r.128, s.129, t.130, u.131, v.132, w.133, x.134, a_expected.135, b_expected.136, c_expected.137, d_expected.138, e_expected.139, f_expected.140, g_expected.141, h_expected.142, i_expected.143, j_expected.144, k_expected.145, l_expected.146, m_expected.147, n_expected.148, o_expected.149, p_expected.150, q_expected.151, r_expected.152, s_expected.153, t_expected.154, u_expected.155, v_expected.156, w_expected.157, x_expected.158) { 
            tmp.0 = &a.111
            tmp.1 = tmp.0
            tmp.2 = a_expected.135
            tmp.3 = memcmp(tmp.1, tmp.2, 1UL)
            if !tmp.3 jump end_if_0
            return 0
        
          end_if_0:
            tmp.4 = &b.112
            tmp.5 = tmp.4
            tmp.6 = b_expected.136
            tmp.7 = memcmp(tmp.5, tmp.6, 2UL)
            if !tmp.7 jump end_if_2
            return 0
        
          end_if_2:
            tmp.8 = &c.113
            tmp.9 = tmp.8
            tmp.10 = c_expected.137
            tmp.11 = memcmp(tmp.9, tmp.10, 3UL)
            if !tmp.11 jump end_if_4
            return 0
        
          end_if_4:
            tmp.12 = &d.114
            tmp.13 = tmp.12
            tmp.14 = d_expected.138
            tmp.15 = memcmp(tmp.13, tmp.14, 4UL)
            if !tmp.15 jump end_if_6
            return 0
        
          end_if_6:
            tmp.16 = &e.115
            tmp.17 = tmp.16
            tmp.18 = e_expected.139
            tmp.19 = memcmp(tmp.17, tmp.18, 5UL)
            if !tmp.19 jump end_if_8
            return 0
        
          end_if_8:
            tmp.20 = &f.116
            tmp.21 = tmp.20
            tmp.22 = f_expected.140
            tmp.23 = memcmp(tmp.21, tmp.22, 6UL)
            if !tmp.23 jump end_if_10
            return 0
        
          end_if_10:
            tmp.24 = &g.117
            tmp.25 = tmp.24
            tmp.26 = g_expected.141
            tmp.27 = memcmp(tmp.25, tmp.26, 7UL)
            if !tmp.27 jump end_if_12
            return 0
        
          end_if_12:
            tmp.28 = &h.118
            tmp.29 = tmp.28
            tmp.30 = h_expected.142
            tmp.31 = memcmp(tmp.29, tmp.30, 8UL)
            if !tmp.31 jump end_if_14
            return 0
        
          end_if_14:
            tmp.32 = &i.119
            tmp.33 = tmp.32
            tmp.34 = i_expected.143
            tmp.35 = memcmp(tmp.33, tmp.34, 9UL)
            if !tmp.35 jump end_if_16
            return 0
        
          end_if_16:
            tmp.36 = &j.120
            tmp.37 = tmp.36
            tmp.38 = j_expected.144
            tmp.39 = memcmp(tmp.37, tmp.38, 10UL)
            if !tmp.39 jump end_if_18
            return 0
        
          end_if_18:
            tmp.40 = &k.121
            tmp.41 = tmp.40
            tmp.42 = k_expected.145
            tmp.43 = memcmp(tmp.41, tmp.42, 11UL)
            if !tmp.43 jump end_if_20
            return 0
        
          end_if_20:
            tmp.44 = &l.122
            tmp.45 = tmp.44
            tmp.46 = l_expected.146
            tmp.47 = memcmp(tmp.45, tmp.46, 12UL)
            if !tmp.47 jump end_if_22
            return 0
        
          end_if_22:
            tmp.48 = &m.123
            tmp.49 = tmp.48
            tmp.50 = m_expected.147
            tmp.51 = memcmp(tmp.49, tmp.50, 13UL)
            if !tmp.51 jump end_if_24
            return 0
        
          end_if_24:
            tmp.52 = &n.124
            tmp.53 = tmp.52
            tmp.54 = n_expected.148
            tmp.55 = memcmp(tmp.53, tmp.54, 14UL)
            if !tmp.55 jump end_if_26
            return 0
        
          end_if_26:
            tmp.56 = &o.125
            tmp.57 = tmp.56
            tmp.58 = o_expected.149
            tmp.59 = memcmp(tmp.57, tmp.58, 15UL)
            if !tmp.59 jump end_if_28
            return 0
        
          end_if_28:
            tmp.60 = &p.126
            tmp.61 = tmp.60
            tmp.62 = p_expected.150
            tmp.63 = memcmp(tmp.61, tmp.62, 16UL)
            if !tmp.63 jump end_if_30
            return 0
        
          end_if_30:
            tmp.64 = &q.127
            tmp.65 = tmp.64
            tmp.66 = q_expected.151
            tmp.67 = memcmp(tmp.65, tmp.66, 17UL)
            if !tmp.67 jump end_if_32
            return 0
        
          end_if_32:
            tmp.68 = &r.128
            tmp.69 = tmp.68
            tmp.70 = r_expected.152
            tmp.71 = memcmp(tmp.69, tmp.70, 18UL)
            if !tmp.71 jump end_if_34
            return 0
        
          end_if_34:
            tmp.72 = &s.129
            tmp.73 = tmp.72
            tmp.74 = s_expected.153
            tmp.75 = memcmp(tmp.73, tmp.74, 19UL)
            if !tmp.75 jump end_if_36
            return 0
        
          end_if_36:
            tmp.76 = &t.130
            tmp.77 = tmp.76
            tmp.78 = t_expected.154
            tmp.79 = memcmp(tmp.77, tmp.78, 20UL)
            if !tmp.79 jump end_if_38
            return 0
        
          end_if_38:
            tmp.80 = &u.131
            tmp.81 = tmp.80
            tmp.82 = u_expected.155
            tmp.83 = memcmp(tmp.81, tmp.82, 21UL)
            if !tmp.83 jump end_if_40
            return 0
        
          end_if_40:
            tmp.84 = &v.132
            tmp.85 = tmp.84
            tmp.86 = v_expected.156
            tmp.87 = memcmp(tmp.85, tmp.86, 22UL)
            if !tmp.87 jump end_if_42
            return 0
        
          end_if_42:
            tmp.88 = &w.133
            tmp.89 = tmp.88
            tmp.90 = w_expected.157
            tmp.91 = memcmp(tmp.89, tmp.90, 23UL)
            if !tmp.91 jump end_if_44
            return 0
        
          end_if_44:
            tmp.92 = &x.134
            tmp.93 = tmp.92
            tmp.94 = x_expected.158
            tmp.95 = memcmp(tmp.93, tmp.94, 24UL)
            if !tmp.95 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function fun1(a.159, b.160, c.161, d.162, e.163, f.164, g.165, h.166, i.167, j.168, a_expected.169, b_expected.170, c_expected.171, d_expected.172, e_expected.173, f_expected.174, g_expected.175, h_expected.176, i_expected.177, j_expected.178) { 
            tmp.96 = &a.159
            tmp.97 = tmp.96
            tmp.98 = a_expected.169
            tmp.99 = memcmp(tmp.97, tmp.98, 7UL)
            if !tmp.99 jump end_if_48
            return 0
        
          end_if_48:
            tmp.100 = &b.160
            tmp.101 = tmp.100
            tmp.102 = b_expected.170
            tmp.103 = memcmp(tmp.101, tmp.102, 8UL)
            if !tmp.103 jump end_if_50
            return 0
        
          end_if_50:
            tmp.104 = &c.161
            tmp.105 = tmp.104
            tmp.106 = c_expected.171
            tmp.107 = memcmp(tmp.105, tmp.106, 9UL)
            if !tmp.107 jump end_if_52
            return 0
        
          end_if_52:
            tmp.108 = &d.162
            tmp.109 = tmp.108
            tmp.110 = d_expected.172
            tmp.111 = memcmp(tmp.109, tmp.110, 10UL)
            if !tmp.111 jump end_if_54
            return 0
        
          end_if_54:
            tmp.112 = &e.163
            tmp.113 = tmp.112
            tmp.114 = e_expected.173
            tmp.115 = memcmp(tmp.113, tmp.114, 1UL)
            if !tmp.115 jump end_if_56
            return 0
        
          end_if_56:
            tmp.116 = &f.164
            tmp.117 = tmp.116
            tmp.118 = f_expected.174
            tmp.119 = memcmp(tmp.117, tmp.118, 2UL)
            if !tmp.119 jump end_if_58
            return 0
        
          end_if_58:
            tmp.120 = &g.165
            tmp.121 = tmp.120
            tmp.122 = g_expected.175
            tmp.123 = memcmp(tmp.121, tmp.122, 3UL)
            if !tmp.123 jump end_if_60
            return 0
        
          end_if_60:
            tmp.124 = &h.166
            tmp.125 = tmp.124
            tmp.126 = h_expected.176
            tmp.127 = memcmp(tmp.125, tmp.126, 4UL)
            if !tmp.127 jump end_if_62
            return 0
        
          end_if_62:
            tmp.128 = &i.167
            tmp.129 = tmp.128
            tmp.130 = i_expected.177
            tmp.131 = memcmp(tmp.129, tmp.130, 5UL)
            if !tmp.131 jump end_if_64
            return 0
        
          end_if_64:
            tmp.132 = &j.168
            tmp.133 = tmp.132
            tmp.134 = j_expected.178
            tmp.135 = memcmp(tmp.133, tmp.134, 6UL)
            if !tmp.135 jump end_if_66
            return 0
        
          end_if_66:
            return 1
            return 0
        }
        global function fun2(a.179, b.180, c.181, d.182, a_expected.183, b_expected.184, c_expected.185, d_expected.186) { 
            tmp.136 = &a.179
            tmp.137 = tmp.136
            tmp.138 = a_expected.183
            tmp.139 = memcmp(tmp.137, tmp.138, 11UL)
            if !tmp.139 jump end_if_68
            return 0
        
          end_if_68:
            tmp.140 = &b.180
            tmp.141 = tmp.140
            tmp.142 = b_expected.184
            tmp.143 = memcmp(tmp.141, tmp.142, 12UL)
            if !tmp.143 jump end_if_70
            return 0
        
          end_if_70:
            tmp.144 = &c.181
            tmp.145 = tmp.144
            tmp.146 = c_expected.185
            tmp.147 = memcmp(tmp.145, tmp.146, 13UL)
            if !tmp.147 jump end_if_72
            return 0
        
          end_if_72:
            tmp.148 = &d.182
            tmp.149 = tmp.148
            tmp.150 = d_expected.186
            tmp.151 = memcmp(tmp.149, tmp.150, 1UL)
            if !tmp.151 jump end_if_74
            return 0
        
          end_if_74:
            return 1
            return 0
        }
        global function fun3(a.187, b.188, c.189, d.190, a_expected.191, b_expected.192, c_expected.193, d_expected.194) { 
            tmp.152 = &a.187
            tmp.153 = tmp.152
            tmp.154 = a_expected.191
            tmp.155 = memcmp(tmp.153, tmp.154, 14UL)
            if !tmp.155 jump end_if_76
            return 0
        
          end_if_76:
            tmp.156 = &b.188
            tmp.157 = tmp.156
            tmp.158 = b_expected.192
            tmp.159 = memcmp(tmp.157, tmp.158, 15UL)
            if !tmp.159 jump end_if_78
            return 0
        
          end_if_78:
            tmp.160 = &c.189
            tmp.161 = tmp.160
            tmp.162 = c_expected.193
            tmp.163 = memcmp(tmp.161, tmp.162, 16UL)
            if !tmp.163 jump end_if_80
            return 0
        
          end_if_80:
            tmp.164 = &d.190
            tmp.165 = tmp.164
            tmp.166 = d_expected.194
            tmp.167 = memcmp(tmp.165, tmp.166, 2UL)
            if !tmp.167 jump end_if_82
            return 0
        
          end_if_82:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_libraries_struct_sizes_client() {
    let src = r#"
        struct bytesize1 {
            unsigned char arr[1];
        };
        extern struct bytesize1 globvar_1;
        struct bytesize2 {
            unsigned char arr[2];
        };
        extern struct bytesize2 globvar_2;
        struct bytesize3 {
            unsigned char arr[3];
        };
        extern struct bytesize3 globvar_3;
        struct bytesize4 {
            unsigned char arr[4];
        };
        extern struct bytesize4 globvar_4;
        struct bytesize5 {
            unsigned char arr[5];
        };
        extern struct bytesize5 globvar_5;
        struct bytesize6 {
            unsigned char arr[6];
        };
        extern struct bytesize6 globvar_6;
        struct bytesize7 {
            unsigned char arr[7];
        };
        extern struct bytesize7 globvar_7;
        struct bytesize8 {
            unsigned char arr[8];
        };
        extern struct bytesize8 globvar_8;
        struct bytesize9 {
            unsigned char arr[9];
        };
        extern struct bytesize9 globvar_9;
        struct bytesize10 {
            unsigned char arr[10];
        };
        extern struct bytesize10 globvar_10;
        struct bytesize11 {
            unsigned char arr[11];
        };
        extern struct bytesize11 globvar_11;
        struct bytesize12 {
            unsigned char arr[12];
        };
        extern struct bytesize12 globvar_12;
        struct bytesize13 {
            unsigned char arr[13];
        };
        extern struct bytesize13 globvar_13;
        struct bytesize14 {
            unsigned char arr[14];
        };
        extern struct bytesize14 globvar_14;
        struct bytesize15 {
            unsigned char arr[15];
        };
        extern struct bytesize15 globvar_15;
        struct bytesize16 {
            unsigned char arr[16];
        };
        extern struct bytesize16 globvar_16;
        struct bytesize17 {
            unsigned char arr[17];
        };
        extern struct bytesize17 globvar_17;
        struct bytesize18 {
            unsigned char arr[18];
        };
        extern struct bytesize18 globvar_18;
        struct bytesize19 {
            unsigned char arr[19];
        };
        extern struct bytesize19 globvar_19;
        struct bytesize20 {
            unsigned char arr[20];
        };
        extern struct bytesize20 globvar_20;
        struct bytesize21 {
            unsigned char arr[21];
        };
        extern struct bytesize21 globvar_21;
        struct bytesize22 {
            unsigned char arr[22];
        };
        extern struct bytesize22 globvar_22;
        struct bytesize23 {
            unsigned char arr[23];
        };
        extern struct bytesize23 globvar_23;
        struct bytesize24 {
            unsigned char arr[24];
        };
        extern struct bytesize24 globvar_24;
        int fun0(struct bytesize1 a, struct bytesize2 b, struct bytesize3 c,
                 struct bytesize4 d, struct bytesize5 e, struct bytesize6 f,
                 struct bytesize7 g, struct bytesize8 h, struct bytesize9 i,
                 struct bytesize10 j, struct bytesize11 k, struct bytesize12 l,
                 struct bytesize13 m, struct bytesize14 n, struct bytesize15 o,
                 struct bytesize16 p, struct bytesize17 q, struct bytesize18 r,
                 struct bytesize19 s, struct bytesize20 t, struct bytesize21 u,
                 struct bytesize22 v, struct bytesize23 w, struct bytesize24 x,
                 unsigned char *a_expected, unsigned char *b_expected,
                 unsigned char *c_expected, unsigned char *d_expected,
                 unsigned char *e_expected, unsigned char *f_expected,
                 unsigned char *g_expected, unsigned char *h_expected,
                 unsigned char *i_expected, unsigned char *j_expected,
                 unsigned char *k_expected, unsigned char *l_expected,
                 unsigned char *m_expected, unsigned char *n_expected,
                 unsigned char *o_expected, unsigned char *p_expected,
                 unsigned char *q_expected, unsigned char *r_expected,
                 unsigned char *s_expected, unsigned char *t_expected,
                 unsigned char *u_expected, unsigned char *v_expected,
                 unsigned char *w_expected, unsigned char *x_expected);
        int fun1(struct bytesize7 a, struct bytesize8 b, struct bytesize9 c,
                 struct bytesize10 d, struct bytesize1 e, struct bytesize2 f,
                 struct bytesize3 g, struct bytesize4 h, struct bytesize5 i,
                 struct bytesize6 j, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected, unsigned char *e_expected,
                 unsigned char *f_expected, unsigned char *g_expected,
                 unsigned char *h_expected, unsigned char *i_expected,
                 unsigned char *j_expected);
        int fun2(struct bytesize11 a, struct bytesize12 b, struct bytesize13 c,
                 struct bytesize1 d, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected);
        int fun3(struct bytesize14 a, struct bytesize15 b, struct bytesize16 c,
                 struct bytesize2 d, unsigned char *a_expected,
                 unsigned char *b_expected, unsigned char *c_expected,
                 unsigned char *d_expected);
        
        int main(void) {
            if (!fun0(globvar_1, globvar_2, globvar_3, globvar_4, globvar_5, globvar_6,
                     globvar_7, globvar_8, globvar_9, globvar_10, globvar_11,
                     globvar_12, globvar_13, globvar_14, globvar_15, globvar_16,
                     globvar_17, globvar_18, globvar_19, globvar_20, globvar_21,
                     globvar_22, globvar_23, globvar_24, globvar_1.arr, globvar_2.arr,
                     globvar_3.arr, globvar_4.arr, globvar_5.arr, globvar_6.arr,
                     globvar_7.arr, globvar_8.arr, globvar_9.arr, globvar_10.arr,
                     globvar_11.arr, globvar_12.arr, globvar_13.arr, globvar_14.arr,
                     globvar_15.arr, globvar_16.arr, globvar_17.arr, globvar_18.arr,
                     globvar_19.arr, globvar_20.arr, globvar_21.arr, globvar_22.arr,
                     globvar_23.arr, globvar_24.arr)) {
                return 1;
            }
            if (!fun1(globvar_7, globvar_8, globvar_9, globvar_10, globvar_1, globvar_2,
                     globvar_3, globvar_4, globvar_5, globvar_6, globvar_7.arr,
                     globvar_8.arr, globvar_9.arr, globvar_10.arr, globvar_1.arr,
                     globvar_2.arr, globvar_3.arr, globvar_4.arr, globvar_5.arr,
                     globvar_6.arr)) {
                return 2;
            }
            if (!fun2(globvar_11, globvar_12, globvar_13, globvar_1, globvar_11.arr,
                     globvar_12.arr, globvar_13.arr, globvar_1.arr)) {
                return 3;
            }
            if (!fun3(globvar_14, globvar_15, globvar_16, globvar_2, globvar_14.arr,
                     globvar_15.arr, globvar_16.arr, globvar_2.arr)) {
                return 4;
            }
            struct bytesize1 locvar_1 = {{0}};
            struct bytesize2 locvar_2 = {{1, 2}};
            struct bytesize3 locvar_3 = {{3, 4, 5}};
            struct bytesize4 locvar_4 = {{6, 7, 8, 9}};
            struct bytesize5 locvar_5 = {{10, 11, 12, 13, 14}};
            struct bytesize6 locvar_6 = {{15, 16, 17, 18, 19, 20}};
            struct bytesize7 locvar_7 = {{21, 22, 23, 24, 25, 26, 27}};
            struct bytesize8 locvar_8 = {{28, 29, 30, 31, 32, 33, 34, 35}};
            struct bytesize9 locvar_9 = {{36, 37, 38, 39, 40, 41, 42, 43, 44}};
            struct bytesize10 locvar_10 = {{45, 46, 47, 48, 49, 50, 51, 52, 53, 54}};
            struct bytesize11 locvar_11 = {
                {55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65}};
            struct bytesize12 locvar_12 = {
                {66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77}};
            struct bytesize13 locvar_13 = {
                {78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90}};
            struct bytesize14 locvar_14 = {
                {91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104}};
            struct bytesize15 locvar_15 = {{105, 106, 107, 108, 109, 110, 111, 112, 113,
                                            114, 115, 116, 117, 118, 119}};
            struct bytesize16 locvar_16 = {{120, 121, 122, 123, 124, 125, 126, 127, 128,
                                            129, 130, 131, 132, 133, 134, 135}};
            struct bytesize17 locvar_17 = {{136, 137, 138, 139, 140, 141, 142, 143, 144,
                                            145, 146, 147, 148, 149, 150, 151, 152}};
            struct bytesize18 locvar_18 = {{153, 154, 155, 156, 157, 158, 159, 160, 161,
                                            162, 163, 164, 165, 166, 167, 168, 169,
                                            170}};
            struct bytesize19 locvar_19 = {{171, 172, 173, 174, 175, 176, 177, 178, 179,
                                            180, 181, 182, 183, 184, 185, 186, 187, 188,
                                            189}};
            struct bytesize20 locvar_20 = {{190, 191, 192, 193, 194, 195, 196,
                                            197, 198, 199, 200, 201, 202, 203,
                                            204, 205, 206, 207, 208, 209}};
            struct bytesize21 locvar_21 = {{210, 211, 212, 213, 214, 215, 216,
                                            217, 218, 219, 220, 221, 222, 223,
                                            224, 225, 226, 227, 228, 229, 230}};
            struct bytesize22 locvar_22 = {{231, 232, 233, 234, 235, 236, 237, 238,
                                            239, 240, 241, 242, 243, 244, 245, 246,
                                            247, 248, 249, 250, 251, 252}};
            struct bytesize23 locvar_23 = {{253, 254, 255, 0, 1, 2, 3, 4,
                                            5, 6, 7, 8, 9, 10, 11, 12,
                                            13, 14, 15, 16, 17, 18, 19}};
            struct bytesize24 locvar_24 = {{20, 21, 22, 23, 24, 25, 26, 27,
                                            28, 29, 30, 31, 32, 33, 34, 35,
                                            36, 37, 38, 39, 40, 41, 42, 43}};
            if (!fun0(locvar_1, locvar_2, locvar_3, locvar_4, locvar_5, locvar_6,
                     locvar_7, locvar_8, locvar_9, locvar_10, locvar_11, locvar_12,
                     locvar_13, locvar_14, locvar_15, locvar_16, locvar_17, locvar_18,
                     locvar_19, locvar_20, locvar_21, locvar_22, locvar_23, locvar_24,
                     locvar_1.arr, locvar_2.arr, locvar_3.arr, locvar_4.arr,
                     locvar_5.arr, locvar_6.arr, locvar_7.arr, locvar_8.arr,
                     locvar_9.arr, locvar_10.arr, locvar_11.arr, locvar_12.arr,
                     locvar_13.arr, locvar_14.arr, locvar_15.arr, locvar_16.arr,
                     locvar_17.arr, locvar_18.arr, locvar_19.arr, locvar_20.arr,
                     locvar_21.arr, locvar_22.arr, locvar_23.arr, locvar_24.arr)) {
                return 5;
            }
            if (!fun1(locvar_7, locvar_8, locvar_9, locvar_10, locvar_1, locvar_2,
                     locvar_3, locvar_4, locvar_5, locvar_6, locvar_7.arr, locvar_8.arr,
                     locvar_9.arr, locvar_10.arr, locvar_1.arr, locvar_2.arr,
                     locvar_3.arr, locvar_4.arr, locvar_5.arr, locvar_6.arr)) {
                return 6;
            }
            if (!fun2(locvar_11, locvar_12, locvar_13, locvar_1, locvar_11.arr,
                     locvar_12.arr, locvar_13.arr, locvar_1.arr)) {
                return 7;
            }
            if (!fun3(locvar_14, locvar_15, locvar_16, locvar_2, locvar_14.arr,
                     locvar_15.arr, locvar_16.arr, locvar_2.arr)) {
                return 8;
            }
            return 0;
        }
        struct bytesize1 globvar_1 = {{0}};
        struct bytesize2 globvar_2 = {{1, 2}};
        struct bytesize3 globvar_3 = {{3, 4, 5}};
        struct bytesize4 globvar_4 = {{6, 7, 8, 9}};
        struct bytesize5 globvar_5 = {{10, 11, 12, 13, 14}};
        struct bytesize6 globvar_6 = {{15, 16, 17, 18, 19, 20}};
        struct bytesize7 globvar_7 = {{21, 22, 23, 24, 25, 26, 27}};
        struct bytesize8 globvar_8 = {{28, 29, 30, 31, 32, 33, 34, 35}};
        struct bytesize9 globvar_9 = {{36, 37, 38, 39, 40, 41, 42, 43, 44}};
        struct bytesize10 globvar_10 = {{45, 46, 47, 48, 49, 50, 51, 52, 53, 54}};
        struct bytesize11 globvar_11 = {{55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65}};
        struct bytesize12 globvar_12 = {
            {66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77}};
        struct bytesize13 globvar_13 = {
            {78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90}};
        struct bytesize14 globvar_14 = {
            {91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104}};
        struct bytesize15 globvar_15 = {{105, 106, 107, 108, 109, 110, 111, 112, 113,
                                         114, 115, 116, 117, 118, 119}};
        struct bytesize16 globvar_16 = {{120, 121, 122, 123, 124, 125, 126, 127, 128,
                                         129, 130, 131, 132, 133, 134, 135}};
        struct bytesize17 globvar_17 = {{136, 137, 138, 139, 140, 141, 142, 143, 144,
                                         145, 146, 147, 148, 149, 150, 151, 152}};
        struct bytesize18 globvar_18 = {{153, 154, 155, 156, 157, 158, 159, 160, 161,
                                         162, 163, 164, 165, 166, 167, 168, 169, 170}};
        struct bytesize19 globvar_19 = {{171, 172, 173, 174, 175, 176, 177, 178, 179,
                                         180, 181, 182, 183, 184, 185, 186, 187, 188,
                                         189}};
        struct bytesize20 globvar_20 = {{190, 191, 192, 193, 194, 195, 196,
                                         197, 198, 199, 200, 201, 202, 203,
                                         204, 205, 206, 207, 208, 209}};
        struct bytesize21 globvar_21 = {{210, 211, 212, 213, 214, 215, 216,
                                         217, 218, 219, 220, 221, 222, 223,
                                         224, 225, 226, 227, 228, 229, 230}};
        struct bytesize22 globvar_22 = {{231, 232, 233, 234, 235, 236, 237, 238,
                                         239, 240, 241, 242, 243, 244, 245, 246,
                                         247, 248, 249, 250, 251, 252}};
        struct bytesize23 globvar_23 = {{253, 254, 255, 0, 1, 2, 3, 4,
                                         5, 6, 7, 8, 9, 10, 11, 12,
                                         13, 14, 15, 16, 17, 18, 19}};
        struct bytesize24 globvar_24 = {{20, 21, 22, 23, 24, 25, 26, 27,
                                         28, 29, 30, 31, 32, 33, 34, 35,
                                         36, 37, 38, 39, 40, 41, 42, 43}};
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = globvar_1[0]
            tmp.1 = &tmp.0
            tmp.2 = globvar_2[0]
            tmp.3 = &tmp.2
            tmp.4 = globvar_3[0]
            tmp.5 = &tmp.4
            tmp.6 = globvar_4[0]
            tmp.7 = &tmp.6
            tmp.8 = globvar_5[0]
            tmp.9 = &tmp.8
            tmp.10 = globvar_6[0]
            tmp.11 = &tmp.10
            tmp.12 = globvar_7[0]
            tmp.13 = &tmp.12
            tmp.14 = globvar_8[0]
            tmp.15 = &tmp.14
            tmp.16 = globvar_9[0]
            tmp.17 = &tmp.16
            tmp.18 = globvar_10[0]
            tmp.19 = &tmp.18
            tmp.20 = globvar_11[0]
            tmp.21 = &tmp.20
            tmp.22 = globvar_12[0]
            tmp.23 = &tmp.22
            tmp.24 = globvar_13[0]
            tmp.25 = &tmp.24
            tmp.26 = globvar_14[0]
            tmp.27 = &tmp.26
            tmp.28 = globvar_15[0]
            tmp.29 = &tmp.28
            tmp.30 = globvar_16[0]
            tmp.31 = &tmp.30
            tmp.32 = globvar_17[0]
            tmp.33 = &tmp.32
            tmp.34 = globvar_18[0]
            tmp.35 = &tmp.34
            tmp.36 = globvar_19[0]
            tmp.37 = &tmp.36
            tmp.38 = globvar_20[0]
            tmp.39 = &tmp.38
            tmp.40 = globvar_21[0]
            tmp.41 = &tmp.40
            tmp.42 = globvar_22[0]
            tmp.43 = &tmp.42
            tmp.44 = globvar_23[0]
            tmp.45 = &tmp.44
            tmp.46 = globvar_24[0]
            tmp.47 = &tmp.46
            tmp.48 = fun0(globvar_1, globvar_2, globvar_3, globvar_4, globvar_5, globvar_6, globvar_7, globvar_8, globvar_9, globvar_10, globvar_11, globvar_12, globvar_13, globvar_14, globvar_15, globvar_16, globvar_17, globvar_18, globvar_19, globvar_20, globvar_21, globvar_22, globvar_23, globvar_24, tmp.1, tmp.3, tmp.5, tmp.7, tmp.9, tmp.11, tmp.13, tmp.15, tmp.17, tmp.19, tmp.21, tmp.23, tmp.25, tmp.27, tmp.29, tmp.31, tmp.33, tmp.35, tmp.37, tmp.39, tmp.41, tmp.43, tmp.45, tmp.47)
            tmp.49 = ! tmp.48
            if !tmp.49 jump end_if_0
            return 1
        
          end_if_0:
            tmp.50 = globvar_7[0]
            tmp.51 = &tmp.50
            tmp.52 = globvar_8[0]
            tmp.53 = &tmp.52
            tmp.54 = globvar_9[0]
            tmp.55 = &tmp.54
            tmp.56 = globvar_10[0]
            tmp.57 = &tmp.56
            tmp.58 = globvar_1[0]
            tmp.59 = &tmp.58
            tmp.60 = globvar_2[0]
            tmp.61 = &tmp.60
            tmp.62 = globvar_3[0]
            tmp.63 = &tmp.62
            tmp.64 = globvar_4[0]
            tmp.65 = &tmp.64
            tmp.66 = globvar_5[0]
            tmp.67 = &tmp.66
            tmp.68 = globvar_6[0]
            tmp.69 = &tmp.68
            tmp.70 = fun1(globvar_7, globvar_8, globvar_9, globvar_10, globvar_1, globvar_2, globvar_3, globvar_4, globvar_5, globvar_6, tmp.51, tmp.53, tmp.55, tmp.57, tmp.59, tmp.61, tmp.63, tmp.65, tmp.67, tmp.69)
            tmp.71 = ! tmp.70
            if !tmp.71 jump end_if_2
            return 2
        
          end_if_2:
            tmp.72 = globvar_11[0]
            tmp.73 = &tmp.72
            tmp.74 = globvar_12[0]
            tmp.75 = &tmp.74
            tmp.76 = globvar_13[0]
            tmp.77 = &tmp.76
            tmp.78 = globvar_1[0]
            tmp.79 = &tmp.78
            tmp.80 = fun2(globvar_11, globvar_12, globvar_13, globvar_1, tmp.73, tmp.75, tmp.77, tmp.79)
            tmp.81 = ! tmp.80
            if !tmp.81 jump end_if_4
            return 3
        
          end_if_4:
            tmp.82 = globvar_14[0]
            tmp.83 = &tmp.82
            tmp.84 = globvar_15[0]
            tmp.85 = &tmp.84
            tmp.86 = globvar_16[0]
            tmp.87 = &tmp.86
            tmp.88 = globvar_2[0]
            tmp.89 = &tmp.88
            tmp.90 = fun3(globvar_14, globvar_15, globvar_16, globvar_2, tmp.83, tmp.85, tmp.87, tmp.89)
            tmp.91 = ! tmp.90
            if !tmp.91 jump end_if_6
            return 4
        
          end_if_6:
            tmp.92 = truncate 0
            bytesize1.0[0] = tmp.92
            tmp.93 = truncate 1
            bytesize2.1[0] = tmp.93
            tmp.94 = truncate 2
            bytesize2.1[1] = tmp.94
            tmp.95 = truncate 3
            bytesize3.2[0] = tmp.95
            tmp.96 = truncate 4
            bytesize3.2[1] = tmp.96
            tmp.97 = truncate 5
            bytesize3.2[2] = tmp.97
            tmp.98 = truncate 6
            bytesize4.3[0] = tmp.98
            tmp.99 = truncate 7
            bytesize4.3[1] = tmp.99
            tmp.100 = truncate 8
            bytesize4.3[2] = tmp.100
            tmp.101 = truncate 9
            bytesize4.3[3] = tmp.101
            tmp.102 = truncate 10
            bytesize5.4[0] = tmp.102
            tmp.103 = truncate 11
            bytesize5.4[1] = tmp.103
            tmp.104 = truncate 12
            bytesize5.4[2] = tmp.104
            tmp.105 = truncate 13
            bytesize5.4[3] = tmp.105
            tmp.106 = truncate 14
            bytesize5.4[4] = tmp.106
            tmp.107 = truncate 15
            bytesize6.5[0] = tmp.107
            tmp.108 = truncate 16
            bytesize6.5[1] = tmp.108
            tmp.109 = truncate 17
            bytesize6.5[2] = tmp.109
            tmp.110 = truncate 18
            bytesize6.5[3] = tmp.110
            tmp.111 = truncate 19
            bytesize6.5[4] = tmp.111
            tmp.112 = truncate 20
            bytesize6.5[5] = tmp.112
            tmp.113 = truncate 21
            bytesize7.6[0] = tmp.113
            tmp.114 = truncate 22
            bytesize7.6[1] = tmp.114
            tmp.115 = truncate 23
            bytesize7.6[2] = tmp.115
            tmp.116 = truncate 24
            bytesize7.6[3] = tmp.116
            tmp.117 = truncate 25
            bytesize7.6[4] = tmp.117
            tmp.118 = truncate 26
            bytesize7.6[5] = tmp.118
            tmp.119 = truncate 27
            bytesize7.6[6] = tmp.119
            tmp.120 = truncate 28
            bytesize8.7[0] = tmp.120
            tmp.121 = truncate 29
            bytesize8.7[1] = tmp.121
            tmp.122 = truncate 30
            bytesize8.7[2] = tmp.122
            tmp.123 = truncate 31
            bytesize8.7[3] = tmp.123
            tmp.124 = truncate 32
            bytesize8.7[4] = tmp.124
            tmp.125 = truncate 33
            bytesize8.7[5] = tmp.125
            tmp.126 = truncate 34
            bytesize8.7[6] = tmp.126
            tmp.127 = truncate 35
            bytesize8.7[7] = tmp.127
            tmp.128 = truncate 36
            bytesize9.8[0] = tmp.128
            tmp.129 = truncate 37
            bytesize9.8[1] = tmp.129
            tmp.130 = truncate 38
            bytesize9.8[2] = tmp.130
            tmp.131 = truncate 39
            bytesize9.8[3] = tmp.131
            tmp.132 = truncate 40
            bytesize9.8[4] = tmp.132
            tmp.133 = truncate 41
            bytesize9.8[5] = tmp.133
            tmp.134 = truncate 42
            bytesize9.8[6] = tmp.134
            tmp.135 = truncate 43
            bytesize9.8[7] = tmp.135
            tmp.136 = truncate 44
            bytesize9.8[8] = tmp.136
            tmp.137 = truncate 45
            bytesize10.9[0] = tmp.137
            tmp.138 = truncate 46
            bytesize10.9[1] = tmp.138
            tmp.139 = truncate 47
            bytesize10.9[2] = tmp.139
            tmp.140 = truncate 48
            bytesize10.9[3] = tmp.140
            tmp.141 = truncate 49
            bytesize10.9[4] = tmp.141
            tmp.142 = truncate 50
            bytesize10.9[5] = tmp.142
            tmp.143 = truncate 51
            bytesize10.9[6] = tmp.143
            tmp.144 = truncate 52
            bytesize10.9[7] = tmp.144
            tmp.145 = truncate 53
            bytesize10.9[8] = tmp.145
            tmp.146 = truncate 54
            bytesize10.9[9] = tmp.146
            tmp.147 = truncate 55
            bytesize11.10[0] = tmp.147
            tmp.148 = truncate 56
            bytesize11.10[1] = tmp.148
            tmp.149 = truncate 57
            bytesize11.10[2] = tmp.149
            tmp.150 = truncate 58
            bytesize11.10[3] = tmp.150
            tmp.151 = truncate 59
            bytesize11.10[4] = tmp.151
            tmp.152 = truncate 60
            bytesize11.10[5] = tmp.152
            tmp.153 = truncate 61
            bytesize11.10[6] = tmp.153
            tmp.154 = truncate 62
            bytesize11.10[7] = tmp.154
            tmp.155 = truncate 63
            bytesize11.10[8] = tmp.155
            tmp.156 = truncate 64
            bytesize11.10[9] = tmp.156
            tmp.157 = truncate 65
            bytesize11.10[10] = tmp.157
            tmp.158 = truncate 66
            bytesize12.11[0] = tmp.158
            tmp.159 = truncate 67
            bytesize12.11[1] = tmp.159
            tmp.160 = truncate 68
            bytesize12.11[2] = tmp.160
            tmp.161 = truncate 69
            bytesize12.11[3] = tmp.161
            tmp.162 = truncate 70
            bytesize12.11[4] = tmp.162
            tmp.163 = truncate 71
            bytesize12.11[5] = tmp.163
            tmp.164 = truncate 72
            bytesize12.11[6] = tmp.164
            tmp.165 = truncate 73
            bytesize12.11[7] = tmp.165
            tmp.166 = truncate 74
            bytesize12.11[8] = tmp.166
            tmp.167 = truncate 75
            bytesize12.11[9] = tmp.167
            tmp.168 = truncate 76
            bytesize12.11[10] = tmp.168
            tmp.169 = truncate 77
            bytesize12.11[11] = tmp.169
            tmp.170 = truncate 78
            bytesize13.12[0] = tmp.170
            tmp.171 = truncate 79
            bytesize13.12[1] = tmp.171
            tmp.172 = truncate 80
            bytesize13.12[2] = tmp.172
            tmp.173 = truncate 81
            bytesize13.12[3] = tmp.173
            tmp.174 = truncate 82
            bytesize13.12[4] = tmp.174
            tmp.175 = truncate 83
            bytesize13.12[5] = tmp.175
            tmp.176 = truncate 84
            bytesize13.12[6] = tmp.176
            tmp.177 = truncate 85
            bytesize13.12[7] = tmp.177
            tmp.178 = truncate 86
            bytesize13.12[8] = tmp.178
            tmp.179 = truncate 87
            bytesize13.12[9] = tmp.179
            tmp.180 = truncate 88
            bytesize13.12[10] = tmp.180
            tmp.181 = truncate 89
            bytesize13.12[11] = tmp.181
            tmp.182 = truncate 90
            bytesize13.12[12] = tmp.182
            tmp.183 = truncate 91
            bytesize14.13[0] = tmp.183
            tmp.184 = truncate 92
            bytesize14.13[1] = tmp.184
            tmp.185 = truncate 93
            bytesize14.13[2] = tmp.185
            tmp.186 = truncate 94
            bytesize14.13[3] = tmp.186
            tmp.187 = truncate 95
            bytesize14.13[4] = tmp.187
            tmp.188 = truncate 96
            bytesize14.13[5] = tmp.188
            tmp.189 = truncate 97
            bytesize14.13[6] = tmp.189
            tmp.190 = truncate 98
            bytesize14.13[7] = tmp.190
            tmp.191 = truncate 99
            bytesize14.13[8] = tmp.191
            tmp.192 = truncate 100
            bytesize14.13[9] = tmp.192
            tmp.193 = truncate 101
            bytesize14.13[10] = tmp.193
            tmp.194 = truncate 102
            bytesize14.13[11] = tmp.194
            tmp.195 = truncate 103
            bytesize14.13[12] = tmp.195
            tmp.196 = truncate 104
            bytesize14.13[13] = tmp.196
            tmp.197 = truncate 105
            bytesize15.14[0] = tmp.197
            tmp.198 = truncate 106
            bytesize15.14[1] = tmp.198
            tmp.199 = truncate 107
            bytesize15.14[2] = tmp.199
            tmp.200 = truncate 108
            bytesize15.14[3] = tmp.200
            tmp.201 = truncate 109
            bytesize15.14[4] = tmp.201
            tmp.202 = truncate 110
            bytesize15.14[5] = tmp.202
            tmp.203 = truncate 111
            bytesize15.14[6] = tmp.203
            tmp.204 = truncate 112
            bytesize15.14[7] = tmp.204
            tmp.205 = truncate 113
            bytesize15.14[8] = tmp.205
            tmp.206 = truncate 114
            bytesize15.14[9] = tmp.206
            tmp.207 = truncate 115
            bytesize15.14[10] = tmp.207
            tmp.208 = truncate 116
            bytesize15.14[11] = tmp.208
            tmp.209 = truncate 117
            bytesize15.14[12] = tmp.209
            tmp.210 = truncate 118
            bytesize15.14[13] = tmp.210
            tmp.211 = truncate 119
            bytesize15.14[14] = tmp.211
            tmp.212 = truncate 120
            bytesize16.15[0] = tmp.212
            tmp.213 = truncate 121
            bytesize16.15[1] = tmp.213
            tmp.214 = truncate 122
            bytesize16.15[2] = tmp.214
            tmp.215 = truncate 123
            bytesize16.15[3] = tmp.215
            tmp.216 = truncate 124
            bytesize16.15[4] = tmp.216
            tmp.217 = truncate 125
            bytesize16.15[5] = tmp.217
            tmp.218 = truncate 126
            bytesize16.15[6] = tmp.218
            tmp.219 = truncate 127
            bytesize16.15[7] = tmp.219
            tmp.220 = truncate 128
            bytesize16.15[8] = tmp.220
            tmp.221 = truncate 129
            bytesize16.15[9] = tmp.221
            tmp.222 = truncate 130
            bytesize16.15[10] = tmp.222
            tmp.223 = truncate 131
            bytesize16.15[11] = tmp.223
            tmp.224 = truncate 132
            bytesize16.15[12] = tmp.224
            tmp.225 = truncate 133
            bytesize16.15[13] = tmp.225
            tmp.226 = truncate 134
            bytesize16.15[14] = tmp.226
            tmp.227 = truncate 135
            bytesize16.15[15] = tmp.227
            tmp.228 = truncate 136
            bytesize17.16[0] = tmp.228
            tmp.229 = truncate 137
            bytesize17.16[1] = tmp.229
            tmp.230 = truncate 138
            bytesize17.16[2] = tmp.230
            tmp.231 = truncate 139
            bytesize17.16[3] = tmp.231
            tmp.232 = truncate 140
            bytesize17.16[4] = tmp.232
            tmp.233 = truncate 141
            bytesize17.16[5] = tmp.233
            tmp.234 = truncate 142
            bytesize17.16[6] = tmp.234
            tmp.235 = truncate 143
            bytesize17.16[7] = tmp.235
            tmp.236 = truncate 144
            bytesize17.16[8] = tmp.236
            tmp.237 = truncate 145
            bytesize17.16[9] = tmp.237
            tmp.238 = truncate 146
            bytesize17.16[10] = tmp.238
            tmp.239 = truncate 147
            bytesize17.16[11] = tmp.239
            tmp.240 = truncate 148
            bytesize17.16[12] = tmp.240
            tmp.241 = truncate 149
            bytesize17.16[13] = tmp.241
            tmp.242 = truncate 150
            bytesize17.16[14] = tmp.242
            tmp.243 = truncate 151
            bytesize17.16[15] = tmp.243
            tmp.244 = truncate 152
            bytesize17.16[16] = tmp.244
            tmp.245 = truncate 153
            bytesize18.17[0] = tmp.245
            tmp.246 = truncate 154
            bytesize18.17[1] = tmp.246
            tmp.247 = truncate 155
            bytesize18.17[2] = tmp.247
            tmp.248 = truncate 156
            bytesize18.17[3] = tmp.248
            tmp.249 = truncate 157
            bytesize18.17[4] = tmp.249
            tmp.250 = truncate 158
            bytesize18.17[5] = tmp.250
            tmp.251 = truncate 159
            bytesize18.17[6] = tmp.251
            tmp.252 = truncate 160
            bytesize18.17[7] = tmp.252
            tmp.253 = truncate 161
            bytesize18.17[8] = tmp.253
            tmp.254 = truncate 162
            bytesize18.17[9] = tmp.254
            tmp.255 = truncate 163
            bytesize18.17[10] = tmp.255
            tmp.256 = truncate 164
            bytesize18.17[11] = tmp.256
            tmp.257 = truncate 165
            bytesize18.17[12] = tmp.257
            tmp.258 = truncate 166
            bytesize18.17[13] = tmp.258
            tmp.259 = truncate 167
            bytesize18.17[14] = tmp.259
            tmp.260 = truncate 168
            bytesize18.17[15] = tmp.260
            tmp.261 = truncate 169
            bytesize18.17[16] = tmp.261
            tmp.262 = truncate 170
            bytesize18.17[17] = tmp.262
            tmp.263 = truncate 171
            bytesize19.18[0] = tmp.263
            tmp.264 = truncate 172
            bytesize19.18[1] = tmp.264
            tmp.265 = truncate 173
            bytesize19.18[2] = tmp.265
            tmp.266 = truncate 174
            bytesize19.18[3] = tmp.266
            tmp.267 = truncate 175
            bytesize19.18[4] = tmp.267
            tmp.268 = truncate 176
            bytesize19.18[5] = tmp.268
            tmp.269 = truncate 177
            bytesize19.18[6] = tmp.269
            tmp.270 = truncate 178
            bytesize19.18[7] = tmp.270
            tmp.271 = truncate 179
            bytesize19.18[8] = tmp.271
            tmp.272 = truncate 180
            bytesize19.18[9] = tmp.272
            tmp.273 = truncate 181
            bytesize19.18[10] = tmp.273
            tmp.274 = truncate 182
            bytesize19.18[11] = tmp.274
            tmp.275 = truncate 183
            bytesize19.18[12] = tmp.275
            tmp.276 = truncate 184
            bytesize19.18[13] = tmp.276
            tmp.277 = truncate 185
            bytesize19.18[14] = tmp.277
            tmp.278 = truncate 186
            bytesize19.18[15] = tmp.278
            tmp.279 = truncate 187
            bytesize19.18[16] = tmp.279
            tmp.280 = truncate 188
            bytesize19.18[17] = tmp.280
            tmp.281 = truncate 189
            bytesize19.18[18] = tmp.281
            tmp.282 = truncate 190
            bytesize20.19[0] = tmp.282
            tmp.283 = truncate 191
            bytesize20.19[1] = tmp.283
            tmp.284 = truncate 192
            bytesize20.19[2] = tmp.284
            tmp.285 = truncate 193
            bytesize20.19[3] = tmp.285
            tmp.286 = truncate 194
            bytesize20.19[4] = tmp.286
            tmp.287 = truncate 195
            bytesize20.19[5] = tmp.287
            tmp.288 = truncate 196
            bytesize20.19[6] = tmp.288
            tmp.289 = truncate 197
            bytesize20.19[7] = tmp.289
            tmp.290 = truncate 198
            bytesize20.19[8] = tmp.290
            tmp.291 = truncate 199
            bytesize20.19[9] = tmp.291
            tmp.292 = truncate 200
            bytesize20.19[10] = tmp.292
            tmp.293 = truncate 201
            bytesize20.19[11] = tmp.293
            tmp.294 = truncate 202
            bytesize20.19[12] = tmp.294
            tmp.295 = truncate 203
            bytesize20.19[13] = tmp.295
            tmp.296 = truncate 204
            bytesize20.19[14] = tmp.296
            tmp.297 = truncate 205
            bytesize20.19[15] = tmp.297
            tmp.298 = truncate 206
            bytesize20.19[16] = tmp.298
            tmp.299 = truncate 207
            bytesize20.19[17] = tmp.299
            tmp.300 = truncate 208
            bytesize20.19[18] = tmp.300
            tmp.301 = truncate 209
            bytesize20.19[19] = tmp.301
            tmp.302 = truncate 210
            bytesize21.20[0] = tmp.302
            tmp.303 = truncate 211
            bytesize21.20[1] = tmp.303
            tmp.304 = truncate 212
            bytesize21.20[2] = tmp.304
            tmp.305 = truncate 213
            bytesize21.20[3] = tmp.305
            tmp.306 = truncate 214
            bytesize21.20[4] = tmp.306
            tmp.307 = truncate 215
            bytesize21.20[5] = tmp.307
            tmp.308 = truncate 216
            bytesize21.20[6] = tmp.308
            tmp.309 = truncate 217
            bytesize21.20[7] = tmp.309
            tmp.310 = truncate 218
            bytesize21.20[8] = tmp.310
            tmp.311 = truncate 219
            bytesize21.20[9] = tmp.311
            tmp.312 = truncate 220
            bytesize21.20[10] = tmp.312
            tmp.313 = truncate 221
            bytesize21.20[11] = tmp.313
            tmp.314 = truncate 222
            bytesize21.20[12] = tmp.314
            tmp.315 = truncate 223
            bytesize21.20[13] = tmp.315
            tmp.316 = truncate 224
            bytesize21.20[14] = tmp.316
            tmp.317 = truncate 225
            bytesize21.20[15] = tmp.317
            tmp.318 = truncate 226
            bytesize21.20[16] = tmp.318
            tmp.319 = truncate 227
            bytesize21.20[17] = tmp.319
            tmp.320 = truncate 228
            bytesize21.20[18] = tmp.320
            tmp.321 = truncate 229
            bytesize21.20[19] = tmp.321
            tmp.322 = truncate 230
            bytesize21.20[20] = tmp.322
            tmp.323 = truncate 231
            bytesize22.21[0] = tmp.323
            tmp.324 = truncate 232
            bytesize22.21[1] = tmp.324
            tmp.325 = truncate 233
            bytesize22.21[2] = tmp.325
            tmp.326 = truncate 234
            bytesize22.21[3] = tmp.326
            tmp.327 = truncate 235
            bytesize22.21[4] = tmp.327
            tmp.328 = truncate 236
            bytesize22.21[5] = tmp.328
            tmp.329 = truncate 237
            bytesize22.21[6] = tmp.329
            tmp.330 = truncate 238
            bytesize22.21[7] = tmp.330
            tmp.331 = truncate 239
            bytesize22.21[8] = tmp.331
            tmp.332 = truncate 240
            bytesize22.21[9] = tmp.332
            tmp.333 = truncate 241
            bytesize22.21[10] = tmp.333
            tmp.334 = truncate 242
            bytesize22.21[11] = tmp.334
            tmp.335 = truncate 243
            bytesize22.21[12] = tmp.335
            tmp.336 = truncate 244
            bytesize22.21[13] = tmp.336
            tmp.337 = truncate 245
            bytesize22.21[14] = tmp.337
            tmp.338 = truncate 246
            bytesize22.21[15] = tmp.338
            tmp.339 = truncate 247
            bytesize22.21[16] = tmp.339
            tmp.340 = truncate 248
            bytesize22.21[17] = tmp.340
            tmp.341 = truncate 249
            bytesize22.21[18] = tmp.341
            tmp.342 = truncate 250
            bytesize22.21[19] = tmp.342
            tmp.343 = truncate 251
            bytesize22.21[20] = tmp.343
            tmp.344 = truncate 252
            bytesize22.21[21] = tmp.344
            tmp.345 = truncate 253
            bytesize23.22[0] = tmp.345
            tmp.346 = truncate 254
            bytesize23.22[1] = tmp.346
            tmp.347 = truncate 255
            bytesize23.22[2] = tmp.347
            tmp.348 = truncate 0
            bytesize23.22[3] = tmp.348
            tmp.349 = truncate 1
            bytesize23.22[4] = tmp.349
            tmp.350 = truncate 2
            bytesize23.22[5] = tmp.350
            tmp.351 = truncate 3
            bytesize23.22[6] = tmp.351
            tmp.352 = truncate 4
            bytesize23.22[7] = tmp.352
            tmp.353 = truncate 5
            bytesize23.22[8] = tmp.353
            tmp.354 = truncate 6
            bytesize23.22[9] = tmp.354
            tmp.355 = truncate 7
            bytesize23.22[10] = tmp.355
            tmp.356 = truncate 8
            bytesize23.22[11] = tmp.356
            tmp.357 = truncate 9
            bytesize23.22[12] = tmp.357
            tmp.358 = truncate 10
            bytesize23.22[13] = tmp.358
            tmp.359 = truncate 11
            bytesize23.22[14] = tmp.359
            tmp.360 = truncate 12
            bytesize23.22[15] = tmp.360
            tmp.361 = truncate 13
            bytesize23.22[16] = tmp.361
            tmp.362 = truncate 14
            bytesize23.22[17] = tmp.362
            tmp.363 = truncate 15
            bytesize23.22[18] = tmp.363
            tmp.364 = truncate 16
            bytesize23.22[19] = tmp.364
            tmp.365 = truncate 17
            bytesize23.22[20] = tmp.365
            tmp.366 = truncate 18
            bytesize23.22[21] = tmp.366
            tmp.367 = truncate 19
            bytesize23.22[22] = tmp.367
            tmp.368 = truncate 20
            bytesize24.23[0] = tmp.368
            tmp.369 = truncate 21
            bytesize24.23[1] = tmp.369
            tmp.370 = truncate 22
            bytesize24.23[2] = tmp.370
            tmp.371 = truncate 23
            bytesize24.23[3] = tmp.371
            tmp.372 = truncate 24
            bytesize24.23[4] = tmp.372
            tmp.373 = truncate 25
            bytesize24.23[5] = tmp.373
            tmp.374 = truncate 26
            bytesize24.23[6] = tmp.374
            tmp.375 = truncate 27
            bytesize24.23[7] = tmp.375
            tmp.376 = truncate 28
            bytesize24.23[8] = tmp.376
            tmp.377 = truncate 29
            bytesize24.23[9] = tmp.377
            tmp.378 = truncate 30
            bytesize24.23[10] = tmp.378
            tmp.379 = truncate 31
            bytesize24.23[11] = tmp.379
            tmp.380 = truncate 32
            bytesize24.23[12] = tmp.380
            tmp.381 = truncate 33
            bytesize24.23[13] = tmp.381
            tmp.382 = truncate 34
            bytesize24.23[14] = tmp.382
            tmp.383 = truncate 35
            bytesize24.23[15] = tmp.383
            tmp.384 = truncate 36
            bytesize24.23[16] = tmp.384
            tmp.385 = truncate 37
            bytesize24.23[17] = tmp.385
            tmp.386 = truncate 38
            bytesize24.23[18] = tmp.386
            tmp.387 = truncate 39
            bytesize24.23[19] = tmp.387
            tmp.388 = truncate 40
            bytesize24.23[20] = tmp.388
            tmp.389 = truncate 41
            bytesize24.23[21] = tmp.389
            tmp.390 = truncate 42
            bytesize24.23[22] = tmp.390
            tmp.391 = truncate 43
            bytesize24.23[23] = tmp.391
            tmp.392 = locvar_1.108[0]
            tmp.393 = &tmp.392
            tmp.394 = locvar_2.109[0]
            tmp.395 = &tmp.394
            tmp.396 = locvar_3.110[0]
            tmp.397 = &tmp.396
            tmp.398 = locvar_4.111[0]
            tmp.399 = &tmp.398
            tmp.400 = locvar_5.112[0]
            tmp.401 = &tmp.400
            tmp.402 = locvar_6.113[0]
            tmp.403 = &tmp.402
            tmp.404 = locvar_7.114[0]
            tmp.405 = &tmp.404
            tmp.406 = locvar_8.115[0]
            tmp.407 = &tmp.406
            tmp.408 = locvar_9.116[0]
            tmp.409 = &tmp.408
            tmp.410 = locvar_10.117[0]
            tmp.411 = &tmp.410
            tmp.412 = locvar_11.118[0]
            tmp.413 = &tmp.412
            tmp.414 = locvar_12.119[0]
            tmp.415 = &tmp.414
            tmp.416 = locvar_13.120[0]
            tmp.417 = &tmp.416
            tmp.418 = locvar_14.121[0]
            tmp.419 = &tmp.418
            tmp.420 = locvar_15.122[0]
            tmp.421 = &tmp.420
            tmp.422 = locvar_16.123[0]
            tmp.423 = &tmp.422
            tmp.424 = locvar_17.124[0]
            tmp.425 = &tmp.424
            tmp.426 = locvar_18.125[0]
            tmp.427 = &tmp.426
            tmp.428 = locvar_19.126[0]
            tmp.429 = &tmp.428
            tmp.430 = locvar_20.127[0]
            tmp.431 = &tmp.430
            tmp.432 = locvar_21.128[0]
            tmp.433 = &tmp.432
            tmp.434 = locvar_22.129[0]
            tmp.435 = &tmp.434
            tmp.436 = locvar_23.130[0]
            tmp.437 = &tmp.436
            tmp.438 = locvar_24.131[0]
            tmp.439 = &tmp.438
            tmp.440 = fun0(locvar_1.108, locvar_2.109, locvar_3.110, locvar_4.111, locvar_5.112, locvar_6.113, locvar_7.114, locvar_8.115, locvar_9.116, locvar_10.117, locvar_11.118, locvar_12.119, locvar_13.120, locvar_14.121, locvar_15.122, locvar_16.123, locvar_17.124, locvar_18.125, locvar_19.126, locvar_20.127, locvar_21.128, locvar_22.129, locvar_23.130, locvar_24.131, tmp.393, tmp.395, tmp.397, tmp.399, tmp.401, tmp.403, tmp.405, tmp.407, tmp.409, tmp.411, tmp.413, tmp.415, tmp.417, tmp.419, tmp.421, tmp.423, tmp.425, tmp.427, tmp.429, tmp.431, tmp.433, tmp.435, tmp.437, tmp.439)
            tmp.441 = ! tmp.440
            if !tmp.441 jump end_if_8
            return 5
        
          end_if_8:
            tmp.442 = locvar_7.114[0]
            tmp.443 = &tmp.442
            tmp.444 = locvar_8.115[0]
            tmp.445 = &tmp.444
            tmp.446 = locvar_9.116[0]
            tmp.447 = &tmp.446
            tmp.448 = locvar_10.117[0]
            tmp.449 = &tmp.448
            tmp.450 = locvar_1.108[0]
            tmp.451 = &tmp.450
            tmp.452 = locvar_2.109[0]
            tmp.453 = &tmp.452
            tmp.454 = locvar_3.110[0]
            tmp.455 = &tmp.454
            tmp.456 = locvar_4.111[0]
            tmp.457 = &tmp.456
            tmp.458 = locvar_5.112[0]
            tmp.459 = &tmp.458
            tmp.460 = locvar_6.113[0]
            tmp.461 = &tmp.460
            tmp.462 = fun1(locvar_7.114, locvar_8.115, locvar_9.116, locvar_10.117, locvar_1.108, locvar_2.109, locvar_3.110, locvar_4.111, locvar_5.112, locvar_6.113, tmp.443, tmp.445, tmp.447, tmp.449, tmp.451, tmp.453, tmp.455, tmp.457, tmp.459, tmp.461)
            tmp.463 = ! tmp.462
            if !tmp.463 jump end_if_10
            return 6
        
          end_if_10:
            tmp.464 = locvar_11.118[0]
            tmp.465 = &tmp.464
            tmp.466 = locvar_12.119[0]
            tmp.467 = &tmp.466
            tmp.468 = locvar_13.120[0]
            tmp.469 = &tmp.468
            tmp.470 = locvar_1.108[0]
            tmp.471 = &tmp.470
            tmp.472 = fun2(locvar_11.118, locvar_12.119, locvar_13.120, locvar_1.108, tmp.465, tmp.467, tmp.469, tmp.471)
            tmp.473 = ! tmp.472
            if !tmp.473 jump end_if_12
            return 7
        
          end_if_12:
            tmp.474 = locvar_14.121[0]
            tmp.475 = &tmp.474
            tmp.476 = locvar_15.122[0]
            tmp.477 = &tmp.476
            tmp.478 = locvar_16.123[0]
            tmp.479 = &tmp.478
            tmp.480 = locvar_2.109[0]
            tmp.481 = &tmp.480
            tmp.482 = fun3(locvar_14.121, locvar_15.122, locvar_16.123, locvar_2.109, tmp.475, tmp.477, tmp.479, tmp.481)
            tmp.483 = ! tmp.482
            if !tmp.483 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        static global globvar_1: Struct(bytesize1.0) = 0UC
        static global globvar_10: Struct(bytesize10.9) = [ 45UC, 46UC, 47UC, 48UC, 49UC, 50UC, 51UC, 52UC, 53UC, 54UC]
        static global globvar_11: Struct(bytesize11.10) = [ 55UC, 56UC, 57UC, 58UC, 59UC, 60UC, 61UC, 62UC, 63UC, 64UC, 65UC]
        static global globvar_12: Struct(bytesize12.11) = [ 66UC, 67UC, 68UC, 69UC, 70UC, 71UC, 72UC, 73UC, 74UC, 75UC, 76UC, 77UC]
        static global globvar_13: Struct(bytesize13.12) = [ 78UC, 79UC, 80UC, 81UC, 82UC, 83UC, 84UC, 85UC, 86UC, 87UC, 88UC, 89UC, 90UC]
        static global globvar_14: Struct(bytesize14.13) = [ 91UC, 92UC, 93UC, 94UC, 95UC, 96UC, 97UC, 98UC, 99UC, 100UC, 101UC, 102UC, 103UC, 104UC]
        static global globvar_15: Struct(bytesize15.14) = [ 105UC, 106UC, 107UC, 108UC, 109UC, 110UC, 111UC, 112UC, 113UC, 114UC, 115UC, 116UC, 117UC, 118UC, 119UC]
        static global globvar_16: Struct(bytesize16.15) = [ 120UC, 121UC, 122UC, 123UC, 124UC, 125UC, 126UC, 127UC, 128UC, 129UC, 130UC, 131UC, 132UC, 133UC, 134UC, 135UC]
        static global globvar_17: Struct(bytesize17.16) = [ 136UC, 137UC, 138UC, 139UC, 140UC, 141UC, 142UC, 143UC, 144UC, 145UC, 146UC, 147UC, 148UC, 149UC, 150UC, 151UC, 152UC]
        static global globvar_18: Struct(bytesize18.17) = [ 153UC, 154UC, 155UC, 156UC, 157UC, 158UC, 159UC, 160UC, 161UC, 162UC, 163UC, 164UC, 165UC, 166UC, 167UC, 168UC, 169UC, 170UC]
        static global globvar_19: Struct(bytesize19.18) = [ 171UC, 172UC, 173UC, 174UC, 175UC, 176UC, 177UC, 178UC, 179UC, 180UC, 181UC, 182UC, 183UC, 184UC, 185UC, 186UC, 187UC, 188UC, 189UC]
        static global globvar_2: Struct(bytesize2.1) = [ 1UC, 2UC]
        static global globvar_20: Struct(bytesize20.19) = [ 190UC, 191UC, 192UC, 193UC, 194UC, 195UC, 196UC, 197UC, 198UC, 199UC, 200UC, 201UC, 202UC, 203UC, 204UC, 205UC, 206UC, 207UC, 208UC, 209UC]
        static global globvar_21: Struct(bytesize21.20) = [ 210UC, 211UC, 212UC, 213UC, 214UC, 215UC, 216UC, 217UC, 218UC, 219UC, 220UC, 221UC, 222UC, 223UC, 224UC, 225UC, 226UC, 227UC, 228UC, 229UC, 230UC]
        static global globvar_22: Struct(bytesize22.21) = [ 231UC, 232UC, 233UC, 234UC, 235UC, 236UC, 237UC, 238UC, 239UC, 240UC, 241UC, 242UC, 243UC, 244UC, 245UC, 246UC, 247UC, 248UC, 249UC, 250UC, 251UC, 252UC]
        static global globvar_23: Struct(bytesize23.22) = [ 253UC, 254UC, 255UC, 0UC, 1UC, 2UC, 3UC, 4UC, 5UC, 6UC, 7UC, 8UC, 9UC, 10UC, 11UC, 12UC, 13UC, 14UC, 15UC, 16UC, 17UC, 18UC, 19UC]
        static global globvar_24: Struct(bytesize24.23) = [ 20UC, 21UC, 22UC, 23UC, 24UC, 25UC, 26UC, 27UC, 28UC, 29UC, 30UC, 31UC, 32UC, 33UC, 34UC, 35UC, 36UC, 37UC, 38UC, 39UC, 40UC, 41UC, 42UC, 43UC]
        static global globvar_3: Struct(bytesize3.2) = [ 3UC, 4UC, 5UC]
        static global globvar_4: Struct(bytesize4.3) = [ 6UC, 7UC, 8UC, 9UC]
        static global globvar_5: Struct(bytesize5.4) = [ 10UC, 11UC, 12UC, 13UC, 14UC]
        static global globvar_6: Struct(bytesize6.5) = [ 15UC, 16UC, 17UC, 18UC, 19UC, 20UC]
        static global globvar_7: Struct(bytesize7.6) = [ 21UC, 22UC, 23UC, 24UC, 25UC, 26UC, 27UC]
        static global globvar_8: Struct(bytesize8.7) = [ 28UC, 29UC, 30UC, 31UC, 32UC, 33UC, 34UC, 35UC]
        static global globvar_9: Struct(bytesize9.8) = [ 36UC, 37UC, 38UC, 39UC, 40UC, 41UC, 42UC, 43UC, 44UC]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_pass_args_on_page_boundary() {
    let src = r#"
        struct nine_bytes {
            char arr[11];
        };
        extern struct nine_bytes on_page_boundary;
        int f(struct nine_bytes in_reg, int a, int b, int c, int d, int e,
              struct nine_bytes on_stack) {
            for (int i = 0; i < 9; i = i + 1) {
                char in_reg_c = in_reg.arr[i];
                char on_stack_c = on_stack.arr[i];
                if (i == 2) {
                    if (in_reg_c != 4 || on_stack_c != 4) {
                        return 1;
                    }
                } else if (i == 3) {
                    if (in_reg_c != 5 || on_stack_c != 5) {
                        return 2;
                    }
                } else if (i == 8) {
                    if (in_reg_c != 6 || on_stack_c != 6) {
                        return 3;
                    }
                } else {
                    if (in_reg_c || on_stack_c) {
                        return 4;
                    }
                }
            }
            if (a != 101 || b != 102 || c != 103 || d != 104 || e != 105) {
                return 5;
            }
            return 0;
        }
        int main(void) {
            on_page_boundary.arr[2] = 4;
            on_page_boundary.arr[3] = 5;
            on_page_boundary.arr[8] = 6;
            return f(on_page_boundary, 101, 102, 103, 104, 105,
                     on_page_boundary);
        }
    "#;
    let expected = r#"
        global function f(in_reg.1, a.2, b.3, c.4, d.5, e.6, on_stack.7) { 
            i.8 = 0
        
          start_loop_0:
            tmp.0 = i.8 < 9
            if !tmp.0 jump break_loop_0
            tmp.1 = in_reg.1[0]
            tmp.2 = &tmp.1
            tmp.3 = sign_extend i.8
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=1)
            tmp.5 = *tmp.4
            in_reg_c.9 = tmp.5
            tmp.6 = on_stack.7[0]
            tmp.7 = &tmp.6
            tmp.8 = sign_extend i.8
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=1)
            tmp.10 = *tmp.9
            on_stack_c.10 = tmp.10
            tmp.11 = i.8 == 2
            if !tmp.11 jump else_1
            tmp.12 = sign_extend in_reg_c.9
            tmp.13 = tmp.12 != 4
            if tmp.13 jump or_true_2
            tmp.16 = sign_extend on_stack_c.10
            tmp.17 = tmp.16 != 4
            if tmp.17 jump or_true_2
            tmp.15 = 0
            jump or_end_3
        
          or_true_2:
            tmp.15 = 1
        
          or_end_3:
            if !tmp.15 jump end_if_4
            return 1
        
          end_if_4:
            jump end_if_0
        
          else_1:
            tmp.18 = i.8 == 3
            if !tmp.18 jump else_7
            tmp.19 = sign_extend in_reg_c.9
            tmp.20 = tmp.19 != 5
            if tmp.20 jump or_true_8
            tmp.23 = sign_extend on_stack_c.10
            tmp.24 = tmp.23 != 5
            if tmp.24 jump or_true_8
            tmp.22 = 0
            jump or_end_9
        
          or_true_8:
            tmp.22 = 1
        
          or_end_9:
            if !tmp.22 jump end_if_10
            return 2
        
          end_if_10:
            jump end_if_6
        
          else_7:
            tmp.25 = i.8 == 8
            if !tmp.25 jump else_13
            tmp.26 = sign_extend in_reg_c.9
            tmp.27 = tmp.26 != 6
            if tmp.27 jump or_true_14
            tmp.30 = sign_extend on_stack_c.10
            tmp.31 = tmp.30 != 6
            if tmp.31 jump or_true_14
            tmp.29 = 0
            jump or_end_15
        
          or_true_14:
            tmp.29 = 1
        
          or_end_15:
            if !tmp.29 jump end_if_16
            return 3
        
          end_if_16:
            jump end_if_12
        
          else_13:
            if in_reg_c.9 jump or_true_18
            if on_stack_c.10 jump or_true_18
            tmp.33 = 0
            jump or_end_19
        
          or_true_18:
            tmp.33 = 1
        
          or_end_19:
            if !tmp.33 jump end_if_20
            return 4
        
          end_if_20:
        
          end_if_12:
        
          end_if_6:
        
          end_if_0:
        
          continue_loop_0:
            tmp.34 = i.8 + 1
            i.8 = tmp.34
            jump start_loop_0
        
          break_loop_0:
            tmp.35 = a.2 != 101
            if tmp.35 jump or_true_22
            tmp.38 = b.3 != 102
            if tmp.38 jump or_true_22
            tmp.37 = 0
            jump or_end_23
        
          or_true_22:
            tmp.37 = 1
        
          or_end_23:
            if tmp.37 jump or_true_24
            tmp.41 = c.4 != 103
            if tmp.41 jump or_true_24
            tmp.40 = 0
            jump or_end_25
        
          or_true_24:
            tmp.40 = 1
        
          or_end_25:
            if tmp.40 jump or_true_26
            tmp.44 = d.5 != 104
            if tmp.44 jump or_true_26
            tmp.43 = 0
            jump or_end_27
        
          or_true_26:
            tmp.43 = 1
        
          or_end_27:
            if tmp.43 jump or_true_28
            tmp.47 = e.6 != 105
            if tmp.47 jump or_true_28
            tmp.46 = 0
            jump or_end_29
        
          or_true_28:
            tmp.46 = 1
        
          or_end_29:
            if !tmp.46 jump end_if_30
            return 5
        
          end_if_30:
            return 0
            return 0
        }
        global function main() { 
            tmp.48 = on_page_boundary[0]
            tmp.49 = &tmp.48
            tmp.50 = sign_extend 2
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=1)
            tmp.52 = truncate 4
            *tmp.51 = tmp.52
            tmp.53 = on_page_boundary[0]
            tmp.54 = &tmp.53
            tmp.55 = sign_extend 3
            tmp.56 = add_ptr(tmp.54, index=tmp.55, scale=1)
            tmp.57 = truncate 5
            *tmp.56 = tmp.57
            tmp.58 = on_page_boundary[0]
            tmp.59 = &tmp.58
            tmp.60 = sign_extend 8
            tmp.61 = add_ptr(tmp.59, index=tmp.60, scale=1)
            tmp.62 = truncate 6
            *tmp.61 = tmp.62
            tmp.63 = f(on_page_boundary, 101, 102, 103, 104, 105, on_page_boundary)
            return tmp.63
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_simple() {
    let src = r#"
        
        struct pair {
            int x;
            double y;
        };
        double test_struct_param(struct pair p) {
            if (p.x != 1 || p.y != 2.0) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            struct pair x = {1, 2.0};
            if (!test_struct_param(x)) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_struct_param(p.1) { 
            tmp.0 = p.1[0]
            tmp.1 = tmp.0 != 1
            if tmp.1 jump or_true_0
            tmp.4 = p.1[1]
            tmp.5 = tmp.4 != 2D
            if tmp.5 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if !tmp.3 jump end_if_2
            tmp.6 = int_to_double 0
            return tmp.6
        
          end_if_2:
            tmp.7 = int_to_double 1
            return tmp.7
            return 0
        }
        global function main() { 
            pair.0[0] = 1
            pair.0[8] = 2D
            tmp.8 = test_struct_param(x.2)
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parameters_stack_clobber() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        void exit(int status);
        struct stack_bytes {
            char bytes[16];
        };
        static struct stack_bytes to_validate;
        void validate_stack_bytes(int code) {
            if (strcmp(to_validate.bytes, "efghijklmnopqrs")) {
                exit(code);
            }
            return;
        }
        struct one_longword {
            int i;
        };
        void take_longword(struct one_longword s, int code) {
            if (s.i != 10) {
                exit(code);
            }
            return;
        }
        int pass_longword(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct one_longword my_var = {10};
            take_longword(my_var, 1);
            to_validate = bytes;
            validate_stack_bytes(2);
            return 0;
        }
        struct one_quadword {
            long l;
        };
        void take_quadword(struct one_quadword s, int code) {
            if (s.l != 10) {
                exit(code);
            }
            return;
        }
        int pass_quadword(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct one_quadword my_var = {10};
            take_quadword(my_var, 3);
            to_validate = bytes;
            validate_stack_bytes(4);
            return 0;
        }
        struct one_double {
            double d;
        };
        void take_double(struct one_double s, int code) {
            if (s.d != 10) {
                exit(code);
            }
            return;
        }
        int pass_double(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct one_double my_var = {10};
            take_double(my_var, 5);
            to_validate = bytes;
            validate_stack_bytes(6);
            return 0;
        }
        struct twelve_bytes {
            char arr[12];
        };
        void take_twelve_bytes(struct twelve_bytes s, int code) {
            if (strcmp(s.arr, "abcdefghijk")) {
                exit(code);
            }
            return;
        }
        int pass_twelve_bytes(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct twelve_bytes my_var = {"abcdefghijk"};
            take_twelve_bytes(my_var, 7);
            to_validate = bytes;
            validate_stack_bytes(8);
            return 0;
        }
        struct memory {
            char arr[32];
        };
        void take_struct_in_mem(struct memory s, int code) {
            if (strcmp(s.arr, "Here's the thing: I'm a string.")) {
                exit(code);
            }
            return;
        }
        int pass_struct_in_mem(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct memory my_var = {"Here's the thing: I'm a string."};
            take_struct_in_mem(my_var, 9);
            to_validate = bytes;
            validate_stack_bytes(10);
            return 0;
        }
        struct irregular {
            char arr[3];
        };
        void take_irregular_struct(struct irregular s, int code) {
            if (strcmp(s.arr, "12")) {
                exit(code);
            }
            return;
        }
        int pass_irregular_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct irregular my_var = {"12"};
            take_irregular_struct(my_var, 11);
            to_validate = bytes;
            validate_stack_bytes(12);
            return 0;
        }
        struct irregular_memory {
            char arr[27];
        };
        void take_irregular_memory_struct(struct irregular_memory s, int code) {
            if (strcmp(s.arr, "The quick brown fox jumped")) {
                exit(code);
            }
            return;
        }
        int pass_irregular_memory_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            static struct irregular_memory my_var = {"The quick brown fox jumped"};
            take_irregular_memory_struct(my_var, 13);
            to_validate = bytes;
            validate_stack_bytes(14);
            return 0;
        }
        int main(void) {
            pass_longword();
            pass_quadword();
            pass_double();
            pass_twelve_bytes();
            pass_struct_in_mem();
            pass_irregular_struct();
            pass_irregular_memory_struct();
            return 0;
        }
    "#;
    let expected = r#"
        global function validate_stack_bytes(code.4) { 
            tmp.0 = to_validate[0]
            tmp.1 = &tmp.0
            tmp.2 = &string.0
            tmp.3 = strcmp(tmp.1, tmp.2)
            if !tmp.3 jump end_if_0
            exit(code.4)
        
          end_if_0:
            return 
        
            return 0
        }
        global function take_longword(s.6, code.7) { 
            tmp.4 = s.6[0]
            tmp.5 = tmp.4 != 10
            if !tmp.5 jump end_if_2
            exit(code.7)
        
          end_if_2:
            return 
        
            return 0
        }
        global function pass_longword() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_longword(my_var.9, 1)
            to_validate = bytes.8
            validate_stack_bytes(2)
            return 0
            return 0
        }
        global function take_quadword(s.11, code.12) { 
            tmp.6 = s.11[0]
            tmp.8 = sign_extend 10
            tmp.7 = tmp.6 != tmp.8
            if !tmp.7 jump end_if_4
            exit(code.12)
        
          end_if_4:
            return 
        
            return 0
        }
        global function pass_quadword() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_quadword(my_var.14, 3)
            to_validate = bytes.13
            validate_stack_bytes(4)
            return 0
            return 0
        }
        global function take_double(s.16, code.17) { 
            tmp.9 = s.16[0]
            tmp.11 = int_to_double 10
            tmp.10 = tmp.9 != tmp.11
            if !tmp.10 jump end_if_6
            exit(code.17)
        
          end_if_6:
            return 
        
            return 0
        }
        global function pass_double() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_double(my_var.19, 5)
            to_validate = bytes.18
            validate_stack_bytes(6)
            return 0
            return 0
        }
        global function take_twelve_bytes(s.21, code.22) { 
            tmp.12 = s.21[0]
            tmp.13 = &tmp.12
            tmp.14 = &string.1
            tmp.15 = strcmp(tmp.13, tmp.14)
            if !tmp.15 jump end_if_8
            exit(code.22)
        
          end_if_8:
            return 
        
            return 0
        }
        global function pass_twelve_bytes() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_twelve_bytes(my_var.24, 7)
            to_validate = bytes.23
            validate_stack_bytes(8)
            return 0
            return 0
        }
        global function take_struct_in_mem(s.26, code.27) { 
            tmp.16 = s.26[0]
            tmp.17 = &tmp.16
            tmp.18 = &string.2
            tmp.19 = strcmp(tmp.17, tmp.18)
            if !tmp.19 jump end_if_10
            exit(code.27)
        
          end_if_10:
            return 
        
            return 0
        }
        global function pass_struct_in_mem() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_struct_in_mem(my_var.29, 9)
            to_validate = bytes.28
            validate_stack_bytes(10)
            return 0
            return 0
        }
        global function take_irregular_struct(s.31, code.32) { 
            tmp.20 = s.31[0]
            tmp.21 = &tmp.20
            tmp.22 = &string.3
            tmp.23 = strcmp(tmp.21, tmp.22)
            if !tmp.23 jump end_if_12
            exit(code.32)
        
          end_if_12:
            return 
        
            return 0
        }
        global function pass_irregular_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_irregular_struct(my_var.34, 11)
            to_validate = bytes.33
            validate_stack_bytes(12)
            return 0
            return 0
        }
        global function take_irregular_memory_struct(s.36, code.37) { 
            tmp.24 = s.36[0]
            tmp.25 = &tmp.24
            tmp.26 = &string.4
            tmp.27 = strcmp(tmp.25, tmp.26)
            if !tmp.27 jump end_if_14
            exit(code.37)
        
          end_if_14:
            return 
        
            return 0
        }
        global function pass_irregular_memory_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            take_irregular_memory_struct(my_var.39, 13)
            to_validate = bytes.38
            validate_stack_bytes(14)
            return 0
            return 0
        }
        global function main() { 
            tmp.28 = pass_longword()
            tmp.29 = pass_quadword()
            tmp.30 = pass_double()
            tmp.31 = pass_twelve_bytes()
            tmp.32 = pass_struct_in_mem()
            tmp.33 = pass_irregular_struct()
            tmp.34 = pass_irregular_memory_struct()
            return 0
            return 0
        }
        static my_var.14: Struct(one_quadword.10) = 10L
        static my_var.19: Struct(one_double.15) = 10D
        static my_var.24: Struct(twelve_bytes.20) = "abcdefghijk\\0"
        static my_var.29: Struct(memory.25) = "Here's the thing: I'm a string.\\0"
        static my_var.34: Struct(irregular.30) = "12\\0"
        static my_var.39: Struct(irregular_memory.35) = "The quick brown fox jumped\\0"
        static my_var.9: Struct(one_longword.5) = 10
        constant string.0: Array(16,Char) = "efghijklmnopqrs\\0"
        constant string.1: Array(12,Char) = "abcdefghijk\\0"
        constant string.2: Array(32,Char) = "Here's the thing: I'm a string.\\0"
        constant string.3: Array(3,Char) = "12\\0"
        constant string.4: Array(27,Char) = "The quick brown fox jumped\\0"
        static to_validate: Struct(stack_bytes.3) = zero[16]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_ignore_retval() {
    let src = r#"
        struct small {
            int x;
        };
        struct big {
            double d;
            int x;
            long l;
        };
        struct small globl = {0};
        struct small return_in_reg(void) {
            globl.x = globl.x + 1;
            return globl;
        }
        struct big globl2 = {1.25, 2, 300};
        struct big return_in_mem(void) {
            globl2.d = globl2.d * 2;
            globl2.x = globl2.x * 3;
            globl2.l = globl2.l * 4;
            return globl2;
        }
        int main(void) {
            (void)return_in_reg();
            return_in_reg();
            if (globl.x != 2) {
                return 1;
            }
            return_in_mem();
            (void)return_in_mem();
            if (globl2.d != 5.0 || globl2.x != 18 || globl2.l != 4800) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_in_reg() { 
            tmp.0 = globl[0]
            tmp.1 = tmp.0 + 1
            globl[0] = tmp.1
            return globl
            return 0
        }
        global function return_in_mem() { 
            tmp.2 = globl2[0]
            tmp.4 = int_to_double 2
            tmp.3 = tmp.2 * tmp.4
            globl2[0] = tmp.3
            tmp.5 = globl2[1]
            tmp.6 = tmp.5 * 3
            globl2[1] = tmp.6
            tmp.7 = globl2[2]
            tmp.9 = sign_extend 4
            tmp.8 = tmp.7 * tmp.9
            globl2[2] = tmp.8
            return globl2
            return 0
        }
        global function main() { 
            tmp.10 = return_in_reg()
            tmp.11 = return_in_reg()
            tmp.12 = globl[0]
            tmp.13 = tmp.12 != 2
            if !tmp.13 jump end_if_0
            return 1
        
          end_if_0:
            tmp.14 = return_in_mem()
            tmp.15 = return_in_mem()
            tmp.16 = globl2[0]
            tmp.17 = tmp.16 != 5D
            if tmp.17 jump or_true_2
            tmp.20 = globl2[1]
            tmp.21 = tmp.20 != 18
            if tmp.21 jump or_true_2
            tmp.19 = 0
            jump or_end_3
        
          or_true_2:
            tmp.19 = 1
        
          or_end_3:
            if tmp.19 jump or_true_4
            tmp.24 = globl2[2]
            tmp.26 = sign_extend 4800
            tmp.25 = tmp.24 != tmp.26
            if tmp.25 jump or_true_4
            tmp.23 = 0
            jump or_end_5
        
          or_true_4:
            tmp.23 = 1
        
          or_end_5:
            if !tmp.23 jump end_if_6
            return 2
        
          end_if_6:
            return 0
            return 0
        }
        static global globl: Struct(small.0) = 0
        static global globl2: Struct(big.1) = [ 1.25D, 2, zero[4], 300L]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_access_retval_members() {
    let src = r#"
        
        
        struct inner {
            char x;
            long y;
        };
        struct outer {
            double d;
            struct inner *ptr;
            struct inner s;
        };
        void *calloc(unsigned long nmemb, unsigned long size);
        struct inner return_small_struct(void);
        struct outer return_nested_struct(void);
        
        struct inner return_small_struct(void) {
            struct inner i = {101, 102};
            return i;
        }
        struct outer return_nested_struct(void) {
            static struct outer ret = {2.0, 0, {10, 11}};
            if (!ret.ptr) {
                ret.ptr = calloc(1, sizeof(struct inner));
                ret.ptr->x = 12;
                ret.ptr->y = 13;
            }
            return ret;
        }
    "#;
    let expected = r#"
        global function return_small_struct() { 
            tmp.0 = truncate 101
            inner.0[0] = tmp.0
            tmp.1 = sign_extend 102
            inner.0[8] = tmp.1
            return i.4
            return 0
        }
        global function return_nested_struct() { 
            tmp.2 = ret.5[1]
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_0
            tmp.4 = sign_extend 1
            tmp.5 = calloc(tmp.4, 16UL)
            tmp.6 = tmp.5
            ret.5[1] = tmp.6
            tmp.7 = ret.5[1]
            tmp.8 = truncate 12
            tmp.7 = tmp.8
            tmp.9 = ret.5[1]
            tmp.10 = add_ptr(tmp.9, index=1L, scale=1)
            tmp.11 = sign_extend 13
            tmp.10 = tmp.11
        
          end_if_0:
            return ret.5
            return 0
        }
        static ret.5: Struct(outer.1) = [ 2D, 0UL, '\n', zero[7], 11L]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_access_retval_members_client() {
    let src = r#"
        
        
        struct inner {
            char x;
            long y;
        };
        struct outer {
            double d;
            struct inner *ptr;
            struct inner s;
        };
        void *calloc(unsigned long nmemb, unsigned long size);
        struct inner return_small_struct(void);
        struct outer return_nested_struct(void);
        
        int main(void) {
            if (return_small_struct().y != 102) {
                return 1;
            }
            if (return_nested_struct().d != 2.0 || return_nested_struct().s.x != 10 ||
                return_nested_struct().s.y != 11) {
                return 3;
            }
            if (return_nested_struct().ptr->x != 12 ||
                return_nested_struct().ptr->y != 13) {
                return 4;
            }
            return_nested_struct().ptr->x = 70;
            return_nested_struct().ptr->y = 71;
            if (return_nested_struct().ptr->x != 70 ||
                return_nested_struct().ptr->y != 71) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = return_small_struct()
            tmp.1 = tmp.0[1]
            tmp.3 = sign_extend 102
            tmp.2 = tmp.1 != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = return_nested_struct()
            tmp.5 = tmp.4[0]
            tmp.6 = tmp.5 != 2D
            if tmp.6 jump or_true_2
            tmp.9 = return_nested_struct()
            tmp.10 = tmp.9[2]
            tmp.11 = sign_extend tmp.10
            tmp.12 = tmp.11 != 10
            if tmp.12 jump or_true_2
            tmp.8 = 0
            jump or_end_3
        
          or_true_2:
            tmp.8 = 1
        
          or_end_3:
            if tmp.8 jump or_true_4
            tmp.15 = return_nested_struct()
            tmp.16 = tmp.15[3]
            tmp.18 = sign_extend 11
            tmp.17 = tmp.16 != tmp.18
            if tmp.17 jump or_true_4
            tmp.14 = 0
            jump or_end_5
        
          or_true_4:
            tmp.14 = 1
        
          or_end_5:
            if !tmp.14 jump end_if_6
            return 3
        
          end_if_6:
            tmp.19 = return_nested_struct()
            tmp.20 = tmp.19[1]
            tmp.21 = sign_extend tmp.20
            tmp.22 = tmp.21 != 12
            if tmp.22 jump or_true_8
            tmp.25 = return_nested_struct()
            tmp.26 = tmp.25[1]
            tmp.27 = add_ptr(tmp.26, index=1L, scale=1)
            tmp.29 = sign_extend 13
            tmp.28 = tmp.27 != tmp.29
            if tmp.28 jump or_true_8
            tmp.24 = 0
            jump or_end_9
        
          or_true_8:
            tmp.24 = 1
        
          or_end_9:
            if !tmp.24 jump end_if_10
            return 4
        
          end_if_10:
            tmp.30 = return_nested_struct()
            tmp.31 = tmp.30[1]
            tmp.32 = truncate 70
            tmp.31 = tmp.32
            tmp.33 = return_nested_struct()
            tmp.34 = tmp.33[1]
            tmp.35 = add_ptr(tmp.34, index=1L, scale=1)
            tmp.36 = sign_extend 71
            tmp.35 = tmp.36
            tmp.37 = return_nested_struct()
            tmp.38 = tmp.37[1]
            tmp.39 = sign_extend tmp.38
            tmp.40 = tmp.39 != 70
            if tmp.40 jump or_true_12
            tmp.43 = return_nested_struct()
            tmp.44 = tmp.43[1]
            tmp.45 = add_ptr(tmp.44, index=1L, scale=1)
            tmp.47 = sign_extend 71
            tmp.46 = tmp.45 != tmp.47
            if tmp.46 jump or_true_12
            tmp.42 = 0
            jump or_end_13
        
          or_true_12:
            tmp.42 = 1
        
          or_end_13:
            if !tmp.42 jump end_if_14
            return 5
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_missing_retval() {
    let src = r#"
        struct big {
            char arr[25];
        };
        struct big missing_return_value(int *i);
        struct big missing_return_value(int *i) {
            *i = 10;
        }
    "#;
    let expected = r#"
        global function missing_return_value(i.2) { 
            *i.2 = 10
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_missing_retval_client() {
    let src = r#"
        struct big {
            char arr[25];
        };
        struct big missing_return_value(int *i);
        
        int main(void) {
            int array[4] = {1, 2, 3, 4};
            missing_return_value(array + 2);
            return array[2] == 10;
        }
    "#;
    let expected = r#"
        global function main() { 
            array.2[0] = 1
            array.2[4] = 2
            array.2[8] = 3
            array.2[12] = 4
            tmp.0 = &array.2
            tmp.2 = sign_extend 2
            tmp.1 = add_ptr(tmp.0, index=tmp.2, scale=4)
            tmp.3 = missing_return_value(tmp.1)
            tmp.4 = &array.2
            tmp.5 = sign_extend 2
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=4)
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 == 10
            return tmp.8
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_return_calling_conventions() {
    let src = r#"
        
        int strcmp(char *s1, char *s2);
        int strncmp(char *s1, char *s2, unsigned long n);
        struct one_int {
            int i;
            char c;
        };
        struct one_int_exactly {
            unsigned long l;
        };
        struct two_ints {
            char c;
            int arr[3];
        };
        struct two_ints_nested {
            struct one_int a;
            struct one_int b;
        };
        struct twelve_bytes {
            int i;
            char arr[8];
        };
        struct one_xmm {
            double d;
        };
        struct two_xmm {
            double d[2];
        };
        struct int_and_xmm {
            char c;
            double d;
        };
        struct xmm_and_int {
            struct one_xmm dbl;
            char c[3];
        };
        struct odd_size {
            char arr[5];
        };
        struct memory {
            double d;
            char c[3];
            long l;
            int i;
        };
        struct one_int return_int_struct(void);
        struct twelve_bytes return_two_int_struct(void);
        struct one_xmm return_double_struct(void);
        struct two_xmm return_two_double_struct(void);
        struct xmm_and_int return_mixed(void);
        struct int_and_xmm return_mixed2(void);
        struct memory return_on_stack(void);
        struct memory pass_and_return_regs(int i, double d, struct int_and_xmm strct,
                                           int c, struct two_ints t_i, long l,
                                           struct one_int_exactly o_i_e, int c2);
        
        struct one_int return_int_struct(void) {
            struct one_int retval = {1, 2};
            return retval;
        }
        struct twelve_bytes return_two_int_struct(void) {
            struct twelve_bytes retval = {10, "12345678"};
            return retval;
        }
        struct one_xmm return_double_struct(void) {
            struct one_xmm retval = {100.625};
            return retval;
        }
        struct two_xmm return_two_double_struct(void) {
            struct two_xmm retval = {{8.8, 7.8}};
            return retval;
        }
        struct xmm_and_int return_mixed(void) {
            struct xmm_and_int retval = {{10.0}, "ab"};
            return retval;
        }
        struct int_and_xmm return_mixed2(void) {
            struct int_and_xmm retval = {127, 34e43};
            return retval;
        }
        struct memory return_on_stack(void) {
            struct memory retval = {1.25, "xy", 100l, 44};
            return retval;
        }
        int leaf_call(struct two_ints t_i, int c, double d) {
            if (t_i.c != '_' || t_i.arr[0] != 5 || t_i.arr[1] != 6 || t_i.arr[2] != 7) {
                return 0;
            }
            if (c != 'p' || d != 4.56) {
                return 0;
            }
            return 1;
        }
        struct memory pass_and_return_regs(int i, double d, struct int_and_xmm strct,
                                           int c, struct two_ints t_i, long l,
                                           struct one_int_exactly o_i_e, int c2) {
            char stackbytes[8] = "zyxwvut";
            struct memory retval = {0, {0, 0, 0}, 0, 0};
            if (!leaf_call(t_i, strct.c, strct.d)) {
                retval.i = 1;
                return retval;
            }
            if (i != 6 || d != 4.0 || c != 5 || l != 77 || c2 != 99) {
                retval.i = 2;
                return retval;
            }
            if (o_i_e.l != 567890) {
                retval.i = 3;
                return retval;
            }
            if (strcmp(stackbytes, "zyxwvut")) {
                retval.i = 4;
                return retval;
            }
            retval.l = 100;
            return retval;
        }
    "#;
    let expected = r#"
        global function return_int_struct() { 
            one_int.5[0] = 1
            tmp.0 = truncate 2
            one_int.5[4] = tmp.0
            return retval.24
            return 0
        }
        global function return_two_int_struct() { 
            twelve_bytes.9[0] = 10
            twelve_bytes.9[4] = '1'
            twelve_bytes.9[5] = '2'
            twelve_bytes.9[6] = '3'
            twelve_bytes.9[7] = '4'
            twelve_bytes.9[8] = '5'
            twelve_bytes.9[9] = '6'
            twelve_bytes.9[10] = '7'
            twelve_bytes.9[11] = '8'
            return retval.25
            return 0
        }
        global function return_double_struct() { 
            one_xmm.10[0] = 100.625D
            return retval.26
            return 0
        }
        global function return_two_double_struct() { 
            two_xmm.11[0] = 8.8D
            two_xmm.11[8] = 7.8D
            return retval.27
            return 0
        }
        global function return_mixed() { 
            one_xmm.10[0] = 10D
            xmm_and_int.13[8] = 'a'
            xmm_and_int.13[9] = 'b'
            xmm_and_int.13[10] = '\0'
            return retval.28
            return 0
        }
        global function return_mixed2() { 
            tmp.1 = truncate 127
            int_and_xmm.12[0] = tmp.1
            int_and_xmm.12[8] = 340000000000000000000000000000000000000000000D
            return retval.29
            return 0
        }
        global function return_on_stack() { 
            memory.15[0] = 1.25D
            memory.15[8] = 'x'
            memory.15[9] = 'y'
            memory.15[10] = '\0'
            memory.15[16] = 100L
            memory.15[24] = 44
            return retval.30
            return 0
        }
        global function leaf_call(t_i.31, c.32, d.33) { 
            tmp.2 = t_i.31[0]
            tmp.3 = sign_extend tmp.2
            tmp.4 = tmp.3 != 95
            if tmp.4 jump or_true_0
            tmp.7 = t_i.31[1]
            tmp.8 = &tmp.7
            tmp.9 = sign_extend 0
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=4)
            tmp.11 = *tmp.10
            tmp.12 = tmp.11 != 5
            if tmp.12 jump or_true_0
            tmp.6 = 0
            jump or_end_1
        
          or_true_0:
            tmp.6 = 1
        
          or_end_1:
            if tmp.6 jump or_true_2
            tmp.15 = t_i.31[1]
            tmp.16 = &tmp.15
            tmp.17 = sign_extend 1
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=4)
            tmp.19 = *tmp.18
            tmp.20 = tmp.19 != 6
            if tmp.20 jump or_true_2
            tmp.14 = 0
            jump or_end_3
        
          or_true_2:
            tmp.14 = 1
        
          or_end_3:
            if tmp.14 jump or_true_4
            tmp.23 = t_i.31[1]
            tmp.24 = &tmp.23
            tmp.25 = sign_extend 2
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=4)
            tmp.27 = *tmp.26
            tmp.28 = tmp.27 != 7
            if tmp.28 jump or_true_4
            tmp.22 = 0
            jump or_end_5
        
          or_true_4:
            tmp.22 = 1
        
          or_end_5:
            if !tmp.22 jump end_if_6
            return 0
        
          end_if_6:
            tmp.29 = c.32 != 112
            if tmp.29 jump or_true_8
            tmp.32 = d.33 != 4.56D
            if tmp.32 jump or_true_8
            tmp.31 = 0
            jump or_end_9
        
          or_true_8:
            tmp.31 = 1
        
          or_end_9:
            if !tmp.31 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function pass_and_return_regs(i.34, d.35, strct.36, c.37, t_i.38, l.39, o_i_e.40, c2.41) { 
            stackbytes.42[0] = 'z'
            stackbytes.42[1] = 'y'
            stackbytes.42[2] = 'x'
            stackbytes.42[3] = 'w'
            stackbytes.42[4] = 'v'
            stackbytes.42[5] = 'u'
            stackbytes.42[6] = 't'
            stackbytes.42[7] = '\0'
            tmp.33 = int_to_double 0
            memory.15[0] = tmp.33
            tmp.34 = truncate 0
            memory.15[8] = tmp.34
            tmp.35 = truncate 0
            memory.15[9] = tmp.35
            tmp.36 = truncate 0
            memory.15[10] = tmp.36
            tmp.37 = sign_extend 0
            memory.15[16] = tmp.37
            memory.15[24] = 0
            tmp.38 = strct.36[0]
            tmp.39 = sign_extend tmp.38
            tmp.40 = strct.36[1]
            tmp.41 = leaf_call(t_i.38, tmp.39, tmp.40)
            tmp.42 = ! tmp.41
            if !tmp.42 jump end_if_12
            retval.43[3] = 1
            return retval.43
        
          end_if_12:
            tmp.43 = i.34 != 6
            if tmp.43 jump or_true_14
            tmp.46 = d.35 != 4D
            if tmp.46 jump or_true_14
            tmp.45 = 0
            jump or_end_15
        
          or_true_14:
            tmp.45 = 1
        
          or_end_15:
            if tmp.45 jump or_true_16
            tmp.49 = c.37 != 5
            if tmp.49 jump or_true_16
            tmp.48 = 0
            jump or_end_17
        
          or_true_16:
            tmp.48 = 1
        
          or_end_17:
            if tmp.48 jump or_true_18
            tmp.53 = sign_extend 77
            tmp.52 = l.39 != tmp.53
            if tmp.52 jump or_true_18
            tmp.51 = 0
            jump or_end_19
        
          or_true_18:
            tmp.51 = 1
        
          or_end_19:
            if tmp.51 jump or_true_20
            tmp.56 = c2.41 != 99
            if tmp.56 jump or_true_20
            tmp.55 = 0
            jump or_end_21
        
          or_true_20:
            tmp.55 = 1
        
          or_end_21:
            if !tmp.55 jump end_if_22
            retval.43[3] = 2
            return retval.43
        
          end_if_22:
            tmp.57 = o_i_e.40[0]
            tmp.59 = sign_extend 567890
            tmp.58 = tmp.57 != tmp.59
            if !tmp.58 jump end_if_24
            retval.43[3] = 3
            return retval.43
        
          end_if_24:
            tmp.60 = &stackbytes.42
            tmp.61 = &string.0
            tmp.62 = strcmp(tmp.60, tmp.61)
            if !tmp.62 jump end_if_26
            retval.43[3] = 4
            return retval.43
        
          end_if_26:
            tmp.63 = sign_extend 100
            retval.43[2] = tmp.63
            return retval.43
            return 0
        }
        constant string.0: Array(8,Char) = "zyxwvut\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_return_calling_conventions_client() {
    let src = r#"
        
        int strcmp(char *s1, char *s2);
        int strncmp(char *s1, char *s2, unsigned long n);
        struct one_int {
            int i;
            char c;
        };
        struct one_int_exactly {
            unsigned long l;
        };
        struct two_ints {
            char c;
            int arr[3];
        };
        struct two_ints_nested {
            struct one_int a;
            struct one_int b;
        };
        struct twelve_bytes {
            int i;
            char arr[8];
        };
        struct one_xmm {
            double d;
        };
        struct two_xmm {
            double d[2];
        };
        struct int_and_xmm {
            char c;
            double d;
        };
        struct xmm_and_int {
            struct one_xmm dbl;
            char c[3];
        };
        struct odd_size {
            char arr[5];
        };
        struct memory {
            double d;
            char c[3];
            long l;
            int i;
        };
        struct one_int return_int_struct(void);
        struct twelve_bytes return_two_int_struct(void);
        struct one_xmm return_double_struct(void);
        struct two_xmm return_two_double_struct(void);
        struct xmm_and_int return_mixed(void);
        struct int_and_xmm return_mixed2(void);
        struct memory return_on_stack(void);
        struct memory pass_and_return_regs(int i, double d, struct int_and_xmm strct,
                                           int c, struct two_ints t_i, long l,
                                           struct one_int_exactly o_i_e, int c2);
        
        int main(void) {
            struct one_int_exactly one_long = {567890l};
            struct two_ints two_ints = {'_', {5, 6, 7}};
            struct int_and_xmm int_and_xmm = {'p', 4.56};
            struct one_int s1 = return_int_struct();
            if (s1.i != 1 || s1.c != 2) {
                return 1;
            }
            struct twelve_bytes s2 = return_two_int_struct();
            if (s2.i != 10 || strncmp(s2.arr, "12345678", sizeof s2.arr))
                return 2;
            struct one_xmm s3 = return_double_struct();
            if (s3.d != 100.625)
                return 3;
            struct two_xmm s4 = return_two_double_struct();
            if (s4.d[0] != 8.8 || s4.d[1] != 7.8)
                return 4;
            struct xmm_and_int s5 = return_mixed();
            if (s5.dbl.d != 10.0 || strcmp(s5.c, "ab"))
                return 5;
            struct int_and_xmm s6 = return_mixed2();
            if (s6.c != 127 || s6.d != 34e43)
                return 6;
            struct memory s7 = return_on_stack();
            if (s7.d != 1.25 || strcmp(s7.c, "xy") || s7.l != 100l || s7.i != 44)
                return 7;
            s7 = pass_and_return_regs(6, 4.0, int_and_xmm, 5, two_ints, 77, one_long,
                                      99);
            if (s7.d || s7.c[0] || s7.c[1] || s7.c[2])
                return 8;
            if (s7.i)
                return 9;
            if (s7.l != 100)
                return 10;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 567890L
            one_int_exactly.6[0] = tmp.0
            tmp.1 = truncate 95
            two_ints.7[0] = tmp.1
            two_ints.7[4] = 5
            two_ints.7[8] = 6
            two_ints.7[12] = 7
            tmp.2 = truncate 112
            int_and_xmm.12[0] = tmp.2
            int_and_xmm.12[8] = 4.56D
            tmp.3 = return_int_struct()
            s1.27 = tmp.3
            tmp.4 = s1.27[0]
            tmp.5 = tmp.4 != 1
            if tmp.5 jump or_true_0
            tmp.8 = s1.27[1]
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 2
            if tmp.10 jump or_true_0
            tmp.7 = 0
            jump or_end_1
        
          or_true_0:
            tmp.7 = 1
        
          or_end_1:
            if !tmp.7 jump end_if_2
            return 1
        
          end_if_2:
            tmp.11 = return_two_int_struct()
            s2.28 = tmp.11
            tmp.12 = s2.28[0]
            tmp.13 = tmp.12 != 10
            if tmp.13 jump or_true_4
            tmp.16 = s2.28[1]
            tmp.17 = &tmp.16
            tmp.18 = &string.0
            tmp.19 = strncmp(tmp.17, tmp.18, 8UL)
            if tmp.19 jump or_true_4
            tmp.15 = 0
            jump or_end_5
        
          or_true_4:
            tmp.15 = 1
        
          or_end_5:
            if !tmp.15 jump end_if_6
            return 2
        
          end_if_6:
            tmp.20 = return_double_struct()
            s3.29 = tmp.20
            tmp.21 = s3.29[0]
            tmp.22 = tmp.21 != 100.625D
            if !tmp.22 jump end_if_8
            return 3
        
          end_if_8:
            tmp.23 = return_two_double_struct()
            s4.30 = tmp.23
            tmp.24 = s4.30[0]
            tmp.25 = &tmp.24
            tmp.26 = sign_extend 0
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=8)
            tmp.28 = *tmp.27
            tmp.29 = tmp.28 != 8.8D
            if tmp.29 jump or_true_10
            tmp.32 = s4.30[0]
            tmp.33 = &tmp.32
            tmp.34 = sign_extend 1
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=8)
            tmp.36 = *tmp.35
            tmp.37 = tmp.36 != 7.8D
            if tmp.37 jump or_true_10
            tmp.31 = 0
            jump or_end_11
        
          or_true_10:
            tmp.31 = 1
        
          or_end_11:
            if !tmp.31 jump end_if_12
            return 4
        
          end_if_12:
            tmp.38 = return_mixed()
            s5.31 = tmp.38
            tmp.39 = s5.31[0]
            tmp.40 = tmp.39 != 10D
            if tmp.40 jump or_true_14
            tmp.43 = s5.31[1]
            tmp.44 = &tmp.43
            tmp.45 = &string.1
            tmp.46 = strcmp(tmp.44, tmp.45)
            if tmp.46 jump or_true_14
            tmp.42 = 0
            jump or_end_15
        
          or_true_14:
            tmp.42 = 1
        
          or_end_15:
            if !tmp.42 jump end_if_16
            return 5
        
          end_if_16:
            tmp.47 = return_mixed2()
            s6.32 = tmp.47
            tmp.48 = s6.32[0]
            tmp.49 = sign_extend tmp.48
            tmp.50 = tmp.49 != 127
            if tmp.50 jump or_true_18
            tmp.53 = s6.32[1]
            tmp.54 = tmp.53 != 340000000000000000000000000000000000000000000D
            if tmp.54 jump or_true_18
            tmp.52 = 0
            jump or_end_19
        
          or_true_18:
            tmp.52 = 1
        
          or_end_19:
            if !tmp.52 jump end_if_20
            return 6
        
          end_if_20:
            tmp.55 = return_on_stack()
            s7.33 = tmp.55
            tmp.56 = s7.33[0]
            tmp.57 = tmp.56 != 1.25D
            if tmp.57 jump or_true_22
            tmp.60 = s7.33[1]
            tmp.61 = &tmp.60
            tmp.62 = &string.2
            tmp.63 = strcmp(tmp.61, tmp.62)
            if tmp.63 jump or_true_22
            tmp.59 = 0
            jump or_end_23
        
          or_true_22:
            tmp.59 = 1
        
          or_end_23:
            if tmp.59 jump or_true_24
            tmp.66 = s7.33[2]
            tmp.67 = tmp.66 != 100L
            if tmp.67 jump or_true_24
            tmp.65 = 0
            jump or_end_25
        
          or_true_24:
            tmp.65 = 1
        
          or_end_25:
            if tmp.65 jump or_true_26
            tmp.70 = s7.33[3]
            tmp.71 = tmp.70 != 44
            if tmp.71 jump or_true_26
            tmp.69 = 0
            jump or_end_27
        
          or_true_26:
            tmp.69 = 1
        
          or_end_27:
            if !tmp.69 jump end_if_28
            return 7
        
          end_if_28:
            tmp.72 = sign_extend 77
            tmp.73 = pass_and_return_regs(6, 4D, int_and_xmm.26, 5, two_ints.25, tmp.72, one_long.24, 99)
            s7.33 = tmp.73
            tmp.74 = s7.33[0]
            if tmp.74 jump or_true_30
            tmp.77 = s7.33[1]
            tmp.78 = &tmp.77
            tmp.79 = sign_extend 0
            tmp.80 = add_ptr(tmp.78, index=tmp.79, scale=1)
            tmp.81 = *tmp.80
            if tmp.81 jump or_true_30
            tmp.76 = 0
            jump or_end_31
        
          or_true_30:
            tmp.76 = 1
        
          or_end_31:
            if tmp.76 jump or_true_32
            tmp.84 = s7.33[1]
            tmp.85 = &tmp.84
            tmp.86 = sign_extend 1
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=1)
            tmp.88 = *tmp.87
            if tmp.88 jump or_true_32
            tmp.83 = 0
            jump or_end_33
        
          or_true_32:
            tmp.83 = 1
        
          or_end_33:
            if tmp.83 jump or_true_34
            tmp.91 = s7.33[1]
            tmp.92 = &tmp.91
            tmp.93 = sign_extend 2
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=1)
            tmp.95 = *tmp.94
            if tmp.95 jump or_true_34
            tmp.90 = 0
            jump or_end_35
        
          or_true_34:
            tmp.90 = 1
        
          or_end_35:
            if !tmp.90 jump end_if_36
            return 8
        
          end_if_36:
            tmp.96 = s7.33[3]
            if !tmp.96 jump end_if_38
            return 9
        
          end_if_38:
            tmp.97 = s7.33[2]
            tmp.99 = sign_extend 100
            tmp.98 = tmp.97 != tmp.99
            if !tmp.98 jump end_if_40
            return 10
        
          end_if_40:
            return 0
            return 0
        }
        constant string.0: Array(9,Char) = "12345678\\0"
        constant string.1: Array(3,Char) = "ab\\0"
        constant string.2: Array(3,Char) = "xy\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_retval_struct_sizes() {
    let src = r#"
        struct bytesize1 {
            unsigned char arr[1];
        };
        extern struct bytesize1 globvar_1;
        struct bytesize1 fun1(void);
        struct bytesize2 {
            unsigned char arr[2];
        };
        extern struct bytesize2 globvar_2;
        struct bytesize2 fun2(void);
        struct bytesize3 {
            unsigned char arr[3];
        };
        extern struct bytesize3 globvar_3;
        struct bytesize3 fun3(void);
        struct bytesize4 {
            unsigned char arr[4];
        };
        extern struct bytesize4 globvar_4;
        struct bytesize4 fun4(void);
        struct bytesize5 {
            unsigned char arr[5];
        };
        extern struct bytesize5 globvar_5;
        struct bytesize5 fun5(void);
        struct bytesize6 {
            unsigned char arr[6];
        };
        extern struct bytesize6 globvar_6;
        struct bytesize6 fun6(void);
        struct bytesize7 {
            unsigned char arr[7];
        };
        extern struct bytesize7 globvar_7;
        struct bytesize7 fun7(void);
        struct bytesize8 {
            unsigned char arr[8];
        };
        extern struct bytesize8 globvar_8;
        struct bytesize8 fun8(void);
        struct bytesize9 {
            unsigned char arr[9];
        };
        extern struct bytesize9 globvar_9;
        struct bytesize9 fun9(void);
        struct bytesize10 {
            unsigned char arr[10];
        };
        extern struct bytesize10 globvar_10;
        struct bytesize10 fun10(void);
        struct bytesize11 {
            unsigned char arr[11];
        };
        extern struct bytesize11 globvar_11;
        struct bytesize11 fun11(void);
        struct bytesize12 {
            unsigned char arr[12];
        };
        extern struct bytesize12 globvar_12;
        struct bytesize12 fun12(void);
        struct bytesize13 {
            unsigned char arr[13];
        };
        extern struct bytesize13 globvar_13;
        struct bytesize13 fun13(void);
        struct bytesize14 {
            unsigned char arr[14];
        };
        extern struct bytesize14 globvar_14;
        struct bytesize14 fun14(void);
        struct bytesize15 {
            unsigned char arr[15];
        };
        extern struct bytesize15 globvar_15;
        struct bytesize15 fun15(void);
        struct bytesize16 {
            unsigned char arr[16];
        };
        extern struct bytesize16 globvar_16;
        struct bytesize16 fun16(void);
        struct bytesize17 {
            unsigned char arr[17];
        };
        extern struct bytesize17 globvar_17;
        struct bytesize17 fun17(void);
        struct bytesize18 {
            unsigned char arr[18];
        };
        extern struct bytesize18 globvar_18;
        struct bytesize18 fun18(void);
        struct bytesize19 {
            unsigned char arr[19];
        };
        extern struct bytesize19 globvar_19;
        struct bytesize19 fun19(void);
        struct bytesize20 {
            unsigned char arr[20];
        };
        extern struct bytesize20 globvar_20;
        struct bytesize20 fun20(void);
        struct bytesize21 {
            unsigned char arr[21];
        };
        extern struct bytesize21 globvar_21;
        struct bytesize21 fun21(void);
        struct bytesize22 {
            unsigned char arr[22];
        };
        extern struct bytesize22 globvar_22;
        struct bytesize22 fun22(void);
        struct bytesize23 {
            unsigned char arr[23];
        };
        extern struct bytesize23 globvar_23;
        struct bytesize23 fun23(void);
        struct bytesize24 {
            unsigned char arr[24];
        };
        extern struct bytesize24 globvar_24;
        struct bytesize24 fun24(void);
        
        struct bytesize1 fun1(void) {
            return globvar_1;
        }
        struct bytesize2 fun2(void) {
            return globvar_2;
        }
        struct bytesize3 fun3(void) {
            return globvar_3;
        }
        struct bytesize4 fun4(void) {
            return globvar_4;
        }
        struct bytesize5 fun5(void) {
            return globvar_5;
        }
        struct bytesize6 fun6(void) {
            return globvar_6;
        }
        struct bytesize7 fun7(void) {
            return globvar_7;
        }
        struct bytesize8 fun8(void) {
            return globvar_8;
        }
        struct bytesize9 fun9(void) {
            return globvar_9;
        }
        struct bytesize10 fun10(void) {
            return globvar_10;
        }
        struct bytesize11 fun11(void) {
            return globvar_11;
        }
        struct bytesize12 fun12(void) {
            return globvar_12;
        }
        struct bytesize13 fun13(void) {
            return globvar_13;
        }
        struct bytesize14 fun14(void) {
            return globvar_14;
        }
        struct bytesize15 fun15(void) {
            return globvar_15;
        }
        struct bytesize16 fun16(void) {
            return globvar_16;
        }
        struct bytesize17 fun17(void) {
            return globvar_17;
        }
        struct bytesize18 fun18(void) {
            return globvar_18;
        }
        struct bytesize19 fun19(void) {
            return globvar_19;
        }
        struct bytesize20 fun20(void) {
            return globvar_20;
        }
        struct bytesize21 fun21(void) {
            return globvar_21;
        }
        struct bytesize22 fun22(void) {
            return globvar_22;
        }
        struct bytesize23 fun23(void) {
            return globvar_23;
        }
        struct bytesize24 fun24(void) {
            return globvar_24;
        }
    "#;
    let expected = r#"
        global function fun1() { 
            return globvar_1
            return 0
        }
        global function fun2() { 
            return globvar_2
            return 0
        }
        global function fun3() { 
            return globvar_3
            return 0
        }
        global function fun4() { 
            return globvar_4
            return 0
        }
        global function fun5() { 
            return globvar_5
            return 0
        }
        global function fun6() { 
            return globvar_6
            return 0
        }
        global function fun7() { 
            return globvar_7
            return 0
        }
        global function fun8() { 
            return globvar_8
            return 0
        }
        global function fun9() { 
            return globvar_9
            return 0
        }
        global function fun10() { 
            return globvar_10
            return 0
        }
        global function fun11() { 
            return globvar_11
            return 0
        }
        global function fun12() { 
            return globvar_12
            return 0
        }
        global function fun13() { 
            return globvar_13
            return 0
        }
        global function fun14() { 
            return globvar_14
            return 0
        }
        global function fun15() { 
            return globvar_15
            return 0
        }
        global function fun16() { 
            return globvar_16
            return 0
        }
        global function fun17() { 
            return globvar_17
            return 0
        }
        global function fun18() { 
            return globvar_18
            return 0
        }
        global function fun19() { 
            return globvar_19
            return 0
        }
        global function fun20() { 
            return globvar_20
            return 0
        }
        global function fun21() { 
            return globvar_21
            return 0
        }
        global function fun22() { 
            return globvar_22
            return 0
        }
        global function fun23() { 
            return globvar_23
            return 0
        }
        global function fun24() { 
            return globvar_24
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_libraries_retval_struct_sizes_client() {
    let src = r#"
        
        struct bytesize1 {
            unsigned char arr[1];
        };
        extern struct bytesize1 globvar_1;
        struct bytesize1 fun1(void);
        struct bytesize2 {
            unsigned char arr[2];
        };
        extern struct bytesize2 globvar_2;
        struct bytesize2 fun2(void);
        struct bytesize3 {
            unsigned char arr[3];
        };
        extern struct bytesize3 globvar_3;
        struct bytesize3 fun3(void);
        struct bytesize4 {
            unsigned char arr[4];
        };
        extern struct bytesize4 globvar_4;
        struct bytesize4 fun4(void);
        struct bytesize5 {
            unsigned char arr[5];
        };
        extern struct bytesize5 globvar_5;
        struct bytesize5 fun5(void);
        struct bytesize6 {
            unsigned char arr[6];
        };
        extern struct bytesize6 globvar_6;
        struct bytesize6 fun6(void);
        struct bytesize7 {
            unsigned char arr[7];
        };
        extern struct bytesize7 globvar_7;
        struct bytesize7 fun7(void);
        struct bytesize8 {
            unsigned char arr[8];
        };
        extern struct bytesize8 globvar_8;
        struct bytesize8 fun8(void);
        struct bytesize9 {
            unsigned char arr[9];
        };
        extern struct bytesize9 globvar_9;
        struct bytesize9 fun9(void);
        struct bytesize10 {
            unsigned char arr[10];
        };
        extern struct bytesize10 globvar_10;
        struct bytesize10 fun10(void);
        struct bytesize11 {
            unsigned char arr[11];
        };
        extern struct bytesize11 globvar_11;
        struct bytesize11 fun11(void);
        struct bytesize12 {
            unsigned char arr[12];
        };
        extern struct bytesize12 globvar_12;
        struct bytesize12 fun12(void);
        struct bytesize13 {
            unsigned char arr[13];
        };
        extern struct bytesize13 globvar_13;
        struct bytesize13 fun13(void);
        struct bytesize14 {
            unsigned char arr[14];
        };
        extern struct bytesize14 globvar_14;
        struct bytesize14 fun14(void);
        struct bytesize15 {
            unsigned char arr[15];
        };
        extern struct bytesize15 globvar_15;
        struct bytesize15 fun15(void);
        struct bytesize16 {
            unsigned char arr[16];
        };
        extern struct bytesize16 globvar_16;
        struct bytesize16 fun16(void);
        struct bytesize17 {
            unsigned char arr[17];
        };
        extern struct bytesize17 globvar_17;
        struct bytesize17 fun17(void);
        struct bytesize18 {
            unsigned char arr[18];
        };
        extern struct bytesize18 globvar_18;
        struct bytesize18 fun18(void);
        struct bytesize19 {
            unsigned char arr[19];
        };
        extern struct bytesize19 globvar_19;
        struct bytesize19 fun19(void);
        struct bytesize20 {
            unsigned char arr[20];
        };
        extern struct bytesize20 globvar_20;
        struct bytesize20 fun20(void);
        struct bytesize21 {
            unsigned char arr[21];
        };
        extern struct bytesize21 globvar_21;
        struct bytesize21 fun21(void);
        struct bytesize22 {
            unsigned char arr[22];
        };
        extern struct bytesize22 globvar_22;
        struct bytesize22 fun22(void);
        struct bytesize23 {
            unsigned char arr[23];
        };
        extern struct bytesize23 globvar_23;
        struct bytesize23 fun23(void);
        struct bytesize24 {
            unsigned char arr[24];
        };
        extern struct bytesize24 globvar_24;
        struct bytesize24 fun24(void);
        int memcmp(void *s1, void *s2, unsigned long n);
        int main(void) {
            struct bytesize1 s1 = fun1();
            if (memcmp(&s1, &globvar_1, sizeof s1)) {
                return 1;
            }
            struct bytesize2 s2 = fun2();
            if (memcmp(&s2, &globvar_2, sizeof s2)) {
                return 2;
            }
            struct bytesize3 s3 = fun3();
            if (memcmp(&s3, &globvar_3, sizeof s3)) {
                return 3;
            }
            struct bytesize4 s4 = fun4();
            if (memcmp(&s4, &globvar_4, sizeof s4)) {
                return 4;
            }
            struct bytesize5 s5 = fun5();
            if (memcmp(&s5, &globvar_5, sizeof s5)) {
                return 5;
            }
            struct bytesize6 s6 = fun6();
            if (memcmp(&s6, &globvar_6, sizeof s6)) {
                return 6;
            }
            struct bytesize7 s7 = fun7();
            if (memcmp(&s7, &globvar_7, sizeof s7)) {
                return 7;
            }
            struct bytesize8 s8 = fun8();
            if (memcmp(&s8, &globvar_8, sizeof s8)) {
                return 8;
            }
            struct bytesize9 s9 = fun9();
            if (memcmp(&s9, &globvar_9, sizeof s9)) {
                return 9;
            }
            struct bytesize10 s10 = fun10();
            if (memcmp(&s10, &globvar_10, sizeof s10)) {
                return 10;
            }
            struct bytesize11 s11 = fun11();
            if (memcmp(&s11, &globvar_11, sizeof s11)) {
                return 11;
            }
            struct bytesize12 s12 = fun12();
            if (memcmp(&s12, &globvar_12, sizeof s12)) {
                return 12;
            }
            struct bytesize13 s13 = fun13();
            if (memcmp(&s13, &globvar_13, sizeof s13)) {
                return 13;
            }
            struct bytesize14 s14 = fun14();
            if (memcmp(&s14, &globvar_14, sizeof s14)) {
                return 14;
            }
            struct bytesize15 s15 = fun15();
            if (memcmp(&s15, &globvar_15, sizeof s15)) {
                return 15;
            }
            struct bytesize16 s16 = fun16();
            if (memcmp(&s16, &globvar_16, sizeof s16)) {
                return 16;
            }
            struct bytesize17 s17 = fun17();
            if (memcmp(&s17, &globvar_17, sizeof s17)) {
                return 17;
            }
            struct bytesize18 s18 = fun18();
            if (memcmp(&s18, &globvar_18, sizeof s18)) {
                return 18;
            }
            struct bytesize19 s19 = fun19();
            if (memcmp(&s19, &globvar_19, sizeof s19)) {
                return 19;
            }
            struct bytesize20 s20 = fun20();
            if (memcmp(&s20, &globvar_20, sizeof s20)) {
                return 20;
            }
            struct bytesize21 s21 = fun21();
            if (memcmp(&s21, &globvar_21, sizeof s21)) {
                return 21;
            }
            struct bytesize22 s22 = fun22();
            if (memcmp(&s22, &globvar_22, sizeof s22)) {
                return 22;
            }
            struct bytesize23 s23 = fun23();
            if (memcmp(&s23, &globvar_23, sizeof s23)) {
                return 23;
            }
            struct bytesize24 s24 = fun24();
            if (memcmp(&s24, &globvar_24, sizeof s24)) {
                return 24;
            }
            return 0;
        }
        struct bytesize1 globvar_1 = {{0}};
        struct bytesize2 globvar_2 = {{1, 2}};
        struct bytesize3 globvar_3 = {{3, 4, 5}};
        struct bytesize4 globvar_4 = {{6, 7, 8, 9}};
        struct bytesize5 globvar_5 = {{10, 11, 12, 13, 14}};
        struct bytesize6 globvar_6 = {{15, 16, 17, 18, 19, 20}};
        struct bytesize7 globvar_7 = {{21, 22, 23, 24, 25, 26, 27}};
        struct bytesize8 globvar_8 = {{28, 29, 30, 31, 32, 33, 34, 35}};
        struct bytesize9 globvar_9 = {{36, 37, 38, 39, 40, 41, 42, 43, 44}};
        struct bytesize10 globvar_10 = {{45, 46, 47, 48, 49, 50, 51, 52, 53, 54}};
        struct bytesize11 globvar_11 = {{55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65}};
        struct bytesize12 globvar_12 = {
            {66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77}};
        struct bytesize13 globvar_13 = {
            {78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90}};
        struct bytesize14 globvar_14 = {
            {91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104}};
        struct bytesize15 globvar_15 = {{105, 106, 107, 108, 109, 110, 111, 112, 113,
                                         114, 115, 116, 117, 118, 119}};
        struct bytesize16 globvar_16 = {{120, 121, 122, 123, 124, 125, 126, 127, 128,
                                         129, 130, 131, 132, 133, 134, 135}};
        struct bytesize17 globvar_17 = {{136, 137, 138, 139, 140, 141, 142, 143, 144,
                                         145, 146, 147, 148, 149, 150, 151, 152}};
        struct bytesize18 globvar_18 = {{153, 154, 155, 156, 157, 158, 159, 160, 161,
                                         162, 163, 164, 165, 166, 167, 168, 169, 170}};
        struct bytesize19 globvar_19 = {{171, 172, 173, 174, 175, 176, 177, 178, 179,
                                         180, 181, 182, 183, 184, 185, 186, 187, 188,
                                         189}};
        struct bytesize20 globvar_20 = {{190, 191, 192, 193, 194, 195, 196,
                                         197, 198, 199, 200, 201, 202, 203,
                                         204, 205, 206, 207, 208, 209}};
        struct bytesize21 globvar_21 = {{210, 211, 212, 213, 214, 215, 216,
                                         217, 218, 219, 220, 221, 222, 223,
                                         224, 225, 226, 227, 228, 229, 230}};
        struct bytesize22 globvar_22 = {{231, 232, 233, 234, 235, 236, 237, 238,
                                         239, 240, 241, 242, 243, 244, 245, 246,
                                         247, 248, 249, 250, 251, 252}};
        struct bytesize23 globvar_23 = {{253, 254, 255, 0, 1, 2, 3, 4,
                                         5, 6, 7, 8, 9, 10, 11, 12,
                                         13, 14, 15, 16, 17, 18, 19}};
        struct bytesize24 globvar_24 = {{20, 21, 22, 23, 24, 25, 26, 27,
                                         28, 29, 30, 31, 32, 33, 34, 35,
                                         36, 37, 38, 39, 40, 41, 42, 43}};
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fun1()
            s1.27 = tmp.0
            tmp.1 = &s1.27
            tmp.2 = tmp.1
            tmp.3 = &globvar_1
            tmp.4 = tmp.3
            tmp.5 = memcmp(tmp.2, tmp.4, 1UL)
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = fun2()
            s2.28 = tmp.6
            tmp.7 = &s2.28
            tmp.8 = tmp.7
            tmp.9 = &globvar_2
            tmp.10 = tmp.9
            tmp.11 = memcmp(tmp.8, tmp.10, 2UL)
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = fun3()
            s3.29 = tmp.12
            tmp.13 = &s3.29
            tmp.14 = tmp.13
            tmp.15 = &globvar_3
            tmp.16 = tmp.15
            tmp.17 = memcmp(tmp.14, tmp.16, 3UL)
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            tmp.18 = fun4()
            s4.30 = tmp.18
            tmp.19 = &s4.30
            tmp.20 = tmp.19
            tmp.21 = &globvar_4
            tmp.22 = tmp.21
            tmp.23 = memcmp(tmp.20, tmp.22, 4UL)
            if !tmp.23 jump end_if_6
            return 4
        
          end_if_6:
            tmp.24 = fun5()
            s5.31 = tmp.24
            tmp.25 = &s5.31
            tmp.26 = tmp.25
            tmp.27 = &globvar_5
            tmp.28 = tmp.27
            tmp.29 = memcmp(tmp.26, tmp.28, 5UL)
            if !tmp.29 jump end_if_8
            return 5
        
          end_if_8:
            tmp.30 = fun6()
            s6.32 = tmp.30
            tmp.31 = &s6.32
            tmp.32 = tmp.31
            tmp.33 = &globvar_6
            tmp.34 = tmp.33
            tmp.35 = memcmp(tmp.32, tmp.34, 6UL)
            if !tmp.35 jump end_if_10
            return 6
        
          end_if_10:
            tmp.36 = fun7()
            s7.33 = tmp.36
            tmp.37 = &s7.33
            tmp.38 = tmp.37
            tmp.39 = &globvar_7
            tmp.40 = tmp.39
            tmp.41 = memcmp(tmp.38, tmp.40, 7UL)
            if !tmp.41 jump end_if_12
            return 7
        
          end_if_12:
            tmp.42 = fun8()
            s8.34 = tmp.42
            tmp.43 = &s8.34
            tmp.44 = tmp.43
            tmp.45 = &globvar_8
            tmp.46 = tmp.45
            tmp.47 = memcmp(tmp.44, tmp.46, 8UL)
            if !tmp.47 jump end_if_14
            return 8
        
          end_if_14:
            tmp.48 = fun9()
            s9.35 = tmp.48
            tmp.49 = &s9.35
            tmp.50 = tmp.49
            tmp.51 = &globvar_9
            tmp.52 = tmp.51
            tmp.53 = memcmp(tmp.50, tmp.52, 9UL)
            if !tmp.53 jump end_if_16
            return 9
        
          end_if_16:
            tmp.54 = fun10()
            s10.36 = tmp.54
            tmp.55 = &s10.36
            tmp.56 = tmp.55
            tmp.57 = &globvar_10
            tmp.58 = tmp.57
            tmp.59 = memcmp(tmp.56, tmp.58, 10UL)
            if !tmp.59 jump end_if_18
            return 10
        
          end_if_18:
            tmp.60 = fun11()
            s11.37 = tmp.60
            tmp.61 = &s11.37
            tmp.62 = tmp.61
            tmp.63 = &globvar_11
            tmp.64 = tmp.63
            tmp.65 = memcmp(tmp.62, tmp.64, 11UL)
            if !tmp.65 jump end_if_20
            return 11
        
          end_if_20:
            tmp.66 = fun12()
            s12.38 = tmp.66
            tmp.67 = &s12.38
            tmp.68 = tmp.67
            tmp.69 = &globvar_12
            tmp.70 = tmp.69
            tmp.71 = memcmp(tmp.68, tmp.70, 12UL)
            if !tmp.71 jump end_if_22
            return 12
        
          end_if_22:
            tmp.72 = fun13()
            s13.39 = tmp.72
            tmp.73 = &s13.39
            tmp.74 = tmp.73
            tmp.75 = &globvar_13
            tmp.76 = tmp.75
            tmp.77 = memcmp(tmp.74, tmp.76, 13UL)
            if !tmp.77 jump end_if_24
            return 13
        
          end_if_24:
            tmp.78 = fun14()
            s14.40 = tmp.78
            tmp.79 = &s14.40
            tmp.80 = tmp.79
            tmp.81 = &globvar_14
            tmp.82 = tmp.81
            tmp.83 = memcmp(tmp.80, tmp.82, 14UL)
            if !tmp.83 jump end_if_26
            return 14
        
          end_if_26:
            tmp.84 = fun15()
            s15.41 = tmp.84
            tmp.85 = &s15.41
            tmp.86 = tmp.85
            tmp.87 = &globvar_15
            tmp.88 = tmp.87
            tmp.89 = memcmp(tmp.86, tmp.88, 15UL)
            if !tmp.89 jump end_if_28
            return 15
        
          end_if_28:
            tmp.90 = fun16()
            s16.42 = tmp.90
            tmp.91 = &s16.42
            tmp.92 = tmp.91
            tmp.93 = &globvar_16
            tmp.94 = tmp.93
            tmp.95 = memcmp(tmp.92, tmp.94, 16UL)
            if !tmp.95 jump end_if_30
            return 16
        
          end_if_30:
            tmp.96 = fun17()
            s17.43 = tmp.96
            tmp.97 = &s17.43
            tmp.98 = tmp.97
            tmp.99 = &globvar_17
            tmp.100 = tmp.99
            tmp.101 = memcmp(tmp.98, tmp.100, 17UL)
            if !tmp.101 jump end_if_32
            return 17
        
          end_if_32:
            tmp.102 = fun18()
            s18.44 = tmp.102
            tmp.103 = &s18.44
            tmp.104 = tmp.103
            tmp.105 = &globvar_18
            tmp.106 = tmp.105
            tmp.107 = memcmp(tmp.104, tmp.106, 18UL)
            if !tmp.107 jump end_if_34
            return 18
        
          end_if_34:
            tmp.108 = fun19()
            s19.45 = tmp.108
            tmp.109 = &s19.45
            tmp.110 = tmp.109
            tmp.111 = &globvar_19
            tmp.112 = tmp.111
            tmp.113 = memcmp(tmp.110, tmp.112, 19UL)
            if !tmp.113 jump end_if_36
            return 19
        
          end_if_36:
            tmp.114 = fun20()
            s20.46 = tmp.114
            tmp.115 = &s20.46
            tmp.116 = tmp.115
            tmp.117 = &globvar_20
            tmp.118 = tmp.117
            tmp.119 = memcmp(tmp.116, tmp.118, 20UL)
            if !tmp.119 jump end_if_38
            return 20
        
          end_if_38:
            tmp.120 = fun21()
            s21.47 = tmp.120
            tmp.121 = &s21.47
            tmp.122 = tmp.121
            tmp.123 = &globvar_21
            tmp.124 = tmp.123
            tmp.125 = memcmp(tmp.122, tmp.124, 21UL)
            if !tmp.125 jump end_if_40
            return 21
        
          end_if_40:
            tmp.126 = fun22()
            s22.48 = tmp.126
            tmp.127 = &s22.48
            tmp.128 = tmp.127
            tmp.129 = &globvar_22
            tmp.130 = tmp.129
            tmp.131 = memcmp(tmp.128, tmp.130, 22UL)
            if !tmp.131 jump end_if_42
            return 22
        
          end_if_42:
            tmp.132 = fun23()
            s23.49 = tmp.132
            tmp.133 = &s23.49
            tmp.134 = tmp.133
            tmp.135 = &globvar_23
            tmp.136 = tmp.135
            tmp.137 = memcmp(tmp.134, tmp.136, 23UL)
            if !tmp.137 jump end_if_44
            return 23
        
          end_if_44:
            tmp.138 = fun24()
            s24.50 = tmp.138
            tmp.139 = &s24.50
            tmp.140 = tmp.139
            tmp.141 = &globvar_24
            tmp.142 = tmp.141
            tmp.143 = memcmp(tmp.140, tmp.142, 24UL)
            if !tmp.143 jump end_if_46
            return 24
        
          end_if_46:
            return 0
            return 0
        }
        static global globvar_1: Struct(bytesize1.0) = 0UC
        static global globvar_10: Struct(bytesize10.9) = [ 45UC, 46UC, 47UC, 48UC, 49UC, 50UC, 51UC, 52UC, 53UC, 54UC]
        static global globvar_11: Struct(bytesize11.10) = [ 55UC, 56UC, 57UC, 58UC, 59UC, 60UC, 61UC, 62UC, 63UC, 64UC, 65UC]
        static global globvar_12: Struct(bytesize12.11) = [ 66UC, 67UC, 68UC, 69UC, 70UC, 71UC, 72UC, 73UC, 74UC, 75UC, 76UC, 77UC]
        static global globvar_13: Struct(bytesize13.12) = [ 78UC, 79UC, 80UC, 81UC, 82UC, 83UC, 84UC, 85UC, 86UC, 87UC, 88UC, 89UC, 90UC]
        static global globvar_14: Struct(bytesize14.13) = [ 91UC, 92UC, 93UC, 94UC, 95UC, 96UC, 97UC, 98UC, 99UC, 100UC, 101UC, 102UC, 103UC, 104UC]
        static global globvar_15: Struct(bytesize15.14) = [ 105UC, 106UC, 107UC, 108UC, 109UC, 110UC, 111UC, 112UC, 113UC, 114UC, 115UC, 116UC, 117UC, 118UC, 119UC]
        static global globvar_16: Struct(bytesize16.15) = [ 120UC, 121UC, 122UC, 123UC, 124UC, 125UC, 126UC, 127UC, 128UC, 129UC, 130UC, 131UC, 132UC, 133UC, 134UC, 135UC]
        static global globvar_17: Struct(bytesize17.16) = [ 136UC, 137UC, 138UC, 139UC, 140UC, 141UC, 142UC, 143UC, 144UC, 145UC, 146UC, 147UC, 148UC, 149UC, 150UC, 151UC, 152UC]
        static global globvar_18: Struct(bytesize18.17) = [ 153UC, 154UC, 155UC, 156UC, 157UC, 158UC, 159UC, 160UC, 161UC, 162UC, 163UC, 164UC, 165UC, 166UC, 167UC, 168UC, 169UC, 170UC]
        static global globvar_19: Struct(bytesize19.18) = [ 171UC, 172UC, 173UC, 174UC, 175UC, 176UC, 177UC, 178UC, 179UC, 180UC, 181UC, 182UC, 183UC, 184UC, 185UC, 186UC, 187UC, 188UC, 189UC]
        static global globvar_2: Struct(bytesize2.1) = [ 1UC, 2UC]
        static global globvar_20: Struct(bytesize20.19) = [ 190UC, 191UC, 192UC, 193UC, 194UC, 195UC, 196UC, 197UC, 198UC, 199UC, 200UC, 201UC, 202UC, 203UC, 204UC, 205UC, 206UC, 207UC, 208UC, 209UC]
        static global globvar_21: Struct(bytesize21.20) = [ 210UC, 211UC, 212UC, 213UC, 214UC, 215UC, 216UC, 217UC, 218UC, 219UC, 220UC, 221UC, 222UC, 223UC, 224UC, 225UC, 226UC, 227UC, 228UC, 229UC, 230UC]
        static global globvar_22: Struct(bytesize22.21) = [ 231UC, 232UC, 233UC, 234UC, 235UC, 236UC, 237UC, 238UC, 239UC, 240UC, 241UC, 242UC, 243UC, 244UC, 245UC, 246UC, 247UC, 248UC, 249UC, 250UC, 251UC, 252UC]
        static global globvar_23: Struct(bytesize23.22) = [ 253UC, 254UC, 255UC, 0UC, 1UC, 2UC, 3UC, 4UC, 5UC, 6UC, 7UC, 8UC, 9UC, 10UC, 11UC, 12UC, 13UC, 14UC, 15UC, 16UC, 17UC, 18UC, 19UC]
        static global globvar_24: Struct(bytesize24.23) = [ 20UC, 21UC, 22UC, 23UC, 24UC, 25UC, 26UC, 27UC, 28UC, 29UC, 30UC, 31UC, 32UC, 33UC, 34UC, 35UC, 36UC, 37UC, 38UC, 39UC, 40UC, 41UC, 42UC, 43UC]
        static global globvar_3: Struct(bytesize3.2) = [ 3UC, 4UC, 5UC]
        static global globvar_4: Struct(bytesize4.3) = [ 6UC, 7UC, 8UC, 9UC]
        static global globvar_5: Struct(bytesize5.4) = [ 10UC, 11UC, 12UC, 13UC, 14UC]
        static global globvar_6: Struct(bytesize6.5) = [ 15UC, 16UC, 17UC, 18UC, 19UC, 20UC]
        static global globvar_7: Struct(bytesize7.6) = [ 21UC, 22UC, 23UC, 24UC, 25UC, 26UC, 27UC]
        static global globvar_8: Struct(bytesize8.7) = [ 28UC, 29UC, 30UC, 31UC, 32UC, 33UC, 34UC, 35UC]
        static global globvar_9: Struct(bytesize9.8) = [ 36UC, 37UC, 38UC, 39UC, 40UC, 41UC, 42UC, 43UC, 44UC]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_return_big_struct_on_page_boundary() {
    let src = r#"
        struct eighteen_bytes {
            char arr[18];
        };
        extern struct eighteen_bytes on_page_boundary;
        struct eighteen_bytes return_struct(void) {
            on_page_boundary.arr[17] = 12;
            on_page_boundary.arr[9] = -1;
            on_page_boundary.arr[8] = -2;
            on_page_boundary.arr[7] = -3;
            return on_page_boundary;
        }
        int main(void) {
            struct eighteen_bytes x = return_struct();
            for (int i = 0; i < 18; i = i + 1) {
                char val = x.arr[i];
                if (i == 7) {
                    if (val != -3) {
                        return 1;
                    }
                } else if (i == 8) {
                    if (val != -2) {
                        return 2;
                    }
                } else if (i == 9) {
                    if (val != -1) {
                        return 3;
                    }
                } else if (i == 17) {
                    if (val != 12) {
                        return 4;
                    }
                } else if (x.arr[i]) {
                    return 5;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_struct() { 
            tmp.0 = on_page_boundary[0]
            tmp.1 = &tmp.0
            tmp.2 = sign_extend 17
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=1)
            tmp.4 = truncate 12
            *tmp.3 = tmp.4
            tmp.5 = on_page_boundary[0]
            tmp.6 = &tmp.5
            tmp.7 = sign_extend 9
            tmp.8 = add_ptr(tmp.6, index=tmp.7, scale=1)
            tmp.9 = - 1
            tmp.10 = truncate tmp.9
            *tmp.8 = tmp.10
            tmp.11 = on_page_boundary[0]
            tmp.12 = &tmp.11
            tmp.13 = sign_extend 8
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = - 2
            tmp.16 = truncate tmp.15
            *tmp.14 = tmp.16
            tmp.17 = on_page_boundary[0]
            tmp.18 = &tmp.17
            tmp.19 = sign_extend 7
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=1)
            tmp.21 = - 3
            tmp.22 = truncate tmp.21
            *tmp.20 = tmp.22
            return on_page_boundary
            return 0
        }
        global function main() { 
            tmp.23 = return_struct()
            x.1 = tmp.23
            i.2 = 0
        
          start_loop_0:
            tmp.24 = i.2 < 18
            if !tmp.24 jump break_loop_0
            tmp.25 = x.1[0]
            tmp.26 = &tmp.25
            tmp.27 = sign_extend i.2
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=1)
            tmp.29 = *tmp.28
            val.3 = tmp.29
            tmp.30 = i.2 == 7
            if !tmp.30 jump else_1
            tmp.31 = sign_extend val.3
            tmp.33 = - 3
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_2
            return 1
        
          end_if_2:
            jump end_if_0
        
          else_1:
            tmp.34 = i.2 == 8
            if !tmp.34 jump else_5
            tmp.35 = sign_extend val.3
            tmp.37 = - 2
            tmp.36 = tmp.35 != tmp.37
            if !tmp.36 jump end_if_6
            return 2
        
          end_if_6:
            jump end_if_4
        
          else_5:
            tmp.38 = i.2 == 9
            if !tmp.38 jump else_9
            tmp.39 = sign_extend val.3
            tmp.41 = - 1
            tmp.40 = tmp.39 != tmp.41
            if !tmp.40 jump end_if_10
            return 3
        
          end_if_10:
            jump end_if_8
        
          else_9:
            tmp.42 = i.2 == 17
            if !tmp.42 jump else_13
            tmp.43 = sign_extend val.3
            tmp.44 = tmp.43 != 12
            if !tmp.44 jump end_if_14
            return 4
        
          end_if_14:
            jump end_if_12
        
          else_13:
            tmp.45 = x.1[0]
            tmp.46 = &tmp.45
            tmp.47 = sign_extend i.2
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=1)
            tmp.49 = *tmp.48
            if !tmp.49 jump end_if_16
            return 5
        
          end_if_16:
        
          end_if_12:
        
          end_if_8:
        
          end_if_4:
        
          end_if_0:
        
          continue_loop_0:
            tmp.50 = i.2 + 1
            i.2 = tmp.50
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_return_incomplete_type() {
    let src = r#"
        struct s;
        struct s increment_struct(struct s param);
        struct s {
            int a;
            int b;
        };
        int main(void) {
            struct s arg = {1, 2};
            struct s val = increment_struct(arg);
            if (val.a != 2 || val.b != 3) {
                return 1;
            }
            return 0;
        }
        struct s increment_struct(struct s param) {
            param.a = param.a + 1;
            param.b = param.b + 1;
            return param;
        }
    "#;
    let expected = r#"
        global function main() { 
            s.0[0] = 1
            s.0[4] = 2
            tmp.0 = increment_struct(arg.3)
            val.4 = tmp.0
            tmp.1 = val.4[0]
            tmp.2 = tmp.1 != 2
            if tmp.2 jump or_true_0
            tmp.5 = val.4[1]
            tmp.6 = tmp.5 != 3
            if tmp.6 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if !tmp.4 jump end_if_2
            return 1
        
          end_if_2:
            return 0
            return 0
        }
        global function increment_struct(param.5) { 
            tmp.7 = param.5[0]
            tmp.8 = tmp.7 + 1
            param.5[0] = tmp.8
            tmp.9 = param.5[1]
            tmp.10 = tmp.9 + 1
            param.5[1] = tmp.10
            return param.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_return_pointer_in_rax() {
    let src = r#"
        struct s {
            long l1;
            long l2;
            long l3;
        };
        struct s return_in_mem(void) {
            struct s result = {1, 2, 3};
            return result;
        }
    "#;
    let expected = r#"
        global function return_in_mem() { 
            tmp.0 = sign_extend 1
            s.0[0] = tmp.0
            tmp.1 = sign_extend 2
            s.0[8] = tmp.1
            tmp.2 = sign_extend 3
            s.0[16] = tmp.2
            return result.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_return_space_overlap() {
    let src = r#"
        struct s {
            long l1;
            long l2;
            long l3;
        };
        extern struct s globvar;
        struct s overlap_with_globvar(void);
        struct s overlap_with_pointer(struct s *ptr);
        int main(void) {
            globvar = overlap_with_globvar();
            if (globvar.l1 != 400l || globvar.l2 != 500l || globvar.l3 != 600l) {
                return 2;
            }
            struct s my_struct = {10l, 9l, 8l};
            my_struct = overlap_with_pointer(&my_struct);
            if (my_struct.l1 != 20l || my_struct.l2 != 18l || my_struct.l3 != 16l) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = overlap_with_globvar()
            globvar = tmp.0
            tmp.1 = globvar[0]
            tmp.2 = tmp.1 != 400L
            if tmp.2 jump or_true_0
            tmp.5 = globvar[1]
            tmp.6 = tmp.5 != 500L
            if tmp.6 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if tmp.4 jump or_true_2
            tmp.9 = globvar[2]
            tmp.10 = tmp.9 != 600L
            if tmp.10 jump or_true_2
            tmp.8 = 0
            jump or_end_3
        
          or_true_2:
            tmp.8 = 1
        
          or_end_3:
            if !tmp.8 jump end_if_4
            return 2
        
          end_if_4:
            s.0[0] = 10L
            s.0[8] = 9L
            s.0[16] = 8L
            tmp.11 = &my_struct.2
            tmp.12 = overlap_with_pointer(tmp.11)
            my_struct.2 = tmp.12
            tmp.13 = my_struct.2[0]
            tmp.14 = tmp.13 != 20L
            if tmp.14 jump or_true_6
            tmp.17 = my_struct.2[1]
            tmp.18 = tmp.17 != 18L
            if tmp.18 jump or_true_6
            tmp.16 = 0
            jump or_end_7
        
          or_true_6:
            tmp.16 = 1
        
          or_end_7:
            if tmp.16 jump or_true_8
            tmp.21 = my_struct.2[2]
            tmp.22 = tmp.21 != 16L
            if tmp.22 jump or_true_8
            tmp.20 = 0
            jump or_end_9
        
          or_true_8:
            tmp.20 = 1
        
          or_end_9:
            if !tmp.20 jump end_if_10
            return 4
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_return_struct_on_page_boundary() {
    let src = r#"
        struct ten_bytes {
            char arr[10];
        };
        extern struct ten_bytes on_page_boundary;
        struct ten_bytes return_struct(void) {
            on_page_boundary.arr[9] = -1;
            on_page_boundary.arr[8] = -2;
            on_page_boundary.arr[7] = -3;
            return on_page_boundary;
        }
        int main(void) {
            struct ten_bytes x = return_struct();
            for (int i = 0; i < 7; i = i + 1) {
                if (x.arr[i]) {
                    return 1;
                }
            }
            if (x.arr[7] != -3) {
                return 2;
            }
            if (x.arr[8] != -2) {
                return 2;
            }
            if (x.arr[9] != -1) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_struct() { 
            tmp.0 = on_page_boundary[0]
            tmp.1 = &tmp.0
            tmp.2 = sign_extend 9
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=1)
            tmp.4 = - 1
            tmp.5 = truncate tmp.4
            *tmp.3 = tmp.5
            tmp.6 = on_page_boundary[0]
            tmp.7 = &tmp.6
            tmp.8 = sign_extend 8
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=1)
            tmp.10 = - 2
            tmp.11 = truncate tmp.10
            *tmp.9 = tmp.11
            tmp.12 = on_page_boundary[0]
            tmp.13 = &tmp.12
            tmp.14 = sign_extend 7
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=1)
            tmp.16 = - 3
            tmp.17 = truncate tmp.16
            *tmp.15 = tmp.17
            return on_page_boundary
            return 0
        }
        global function main() { 
            tmp.18 = return_struct()
            x.1 = tmp.18
            i.2 = 0
        
          start_loop_0:
            tmp.19 = i.2 < 7
            if !tmp.19 jump break_loop_0
            tmp.20 = x.1[0]
            tmp.21 = &tmp.20
            tmp.22 = sign_extend i.2
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            if !tmp.24 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.25 = i.2 + 1
            i.2 = tmp.25
            jump start_loop_0
        
          break_loop_0:
            tmp.26 = x.1[0]
            tmp.27 = &tmp.26
            tmp.28 = sign_extend 7
            tmp.29 = add_ptr(tmp.27, index=tmp.28, scale=1)
            tmp.30 = *tmp.29
            tmp.31 = sign_extend tmp.30
            tmp.33 = - 3
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_2
            return 2
        
          end_if_2:
            tmp.34 = x.1[0]
            tmp.35 = &tmp.34
            tmp.36 = sign_extend 8
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=1)
            tmp.38 = *tmp.37
            tmp.39 = sign_extend tmp.38
            tmp.41 = - 2
            tmp.40 = tmp.39 != tmp.41
            if !tmp.40 jump end_if_4
            return 2
        
          end_if_4:
            tmp.42 = x.1[0]
            tmp.43 = &tmp.42
            tmp.44 = sign_extend 9
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=1)
            tmp.46 = *tmp.45
            tmp.47 = sign_extend tmp.46
            tmp.49 = - 1
            tmp.48 = tmp.47 != tmp.49
            if !tmp.48 jump end_if_6
            return 3
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_simple() {
    let src = r#"
        struct pair {
            int x;
            char y;
        };
        struct pair2 {
            double d;
            long l;
        };
        struct pair2 double_members(struct pair p) {
            struct pair2 retval = {p.x * 2, p.y * 2};
            return retval;
        }
        int main(void) {
            struct pair arg = {1, 4};
            struct pair2 result = double_members(arg);
            if (result.d != 2.0 || result.l != 8) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function double_members(p.2) { 
            tmp.0 = p.2[0]
            tmp.1 = tmp.0 * 2
            tmp.2 = int_to_double tmp.1
            pair2.1[0] = tmp.2
            tmp.3 = p.2[1]
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 * 2
            tmp.6 = sign_extend tmp.5
            pair2.1[8] = tmp.6
            return retval.3
            return 0
        }
        global function main() { 
            pair.0[0] = 1
            tmp.7 = truncate 4
            pair.0[4] = tmp.7
            tmp.8 = double_members(arg.4)
            result.5 = tmp.8
            tmp.9 = result.5[0]
            tmp.10 = tmp.9 != 2D
            if tmp.10 jump or_true_0
            tmp.13 = result.5[1]
            tmp.15 = sign_extend 8
            tmp.14 = tmp.13 != tmp.15
            if tmp.14 jump or_true_0
            tmp.12 = 0
            jump or_end_1
        
          or_true_0:
            tmp.12 = 1
        
          or_end_1:
            if !tmp.12 jump end_if_2
            return 1
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_stack_clobber() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        void exit(int status);
        struct stack_bytes {
            char bytes[16];
        };
        static struct stack_bytes to_validate;
        void validate_stack_bytes(int code) {
            if (strcmp(to_validate.bytes, "efghijklmnopqrs")) {
                exit(code);
            }
            return;
        }
        struct one_int_reg {
            char cs[7];
        };
        struct one_int_reg return_int_struct(void) {
            struct one_int_reg retval = {{0, 0, 0, 0, 0, 0, 0}};
            return retval;
        }
        static struct one_int_reg one_int_struct;
        void validate_one_int_struct(int code) {
            for (int i = 0; i < 7; i = i + 1) {
                if (one_int_struct.cs[i]) {
                    exit(code);
                }
            }
        }
        int test_int_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            one_int_struct = return_int_struct();
            to_validate = bytes;
            validate_stack_bytes(1);
            validate_one_int_struct(2);
            return 0;
        }
        struct two_int_regs {
            char cs[15];
        };
        struct two_int_regs return_two_int_struct(void) {
            struct two_int_regs retval = {
                {20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34}};
            return retval;
        }
        static struct two_int_regs two_int_struct;
        void validate_two_int_struct(int code) {
            for (int i = 0; i < 15; i = i + 1)
                if (two_int_struct.cs[i] != i + 20) {
                    exit(code);
                }
        }
        int test_two_int_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            two_int_struct = return_two_int_struct();
            to_validate = bytes;
            validate_stack_bytes(3);
            validate_two_int_struct(4);
            return 0;
        }
        struct one_xmm_reg {
            double d;
        };
        struct one_xmm_reg return_one_xmm_struct(void) {
            struct one_xmm_reg retval = {234.5};
            return retval;
        }
        static struct one_xmm_reg one_double_struct;
        void validate_one_double_struct(int code) {
            if (one_double_struct.d != 234.5) {
                exit(code);
            }
        }
        int test_one_double_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            one_double_struct = return_one_xmm_struct();
            to_validate = bytes;
            validate_stack_bytes(5);
            validate_one_double_struct(6);
            return 0;
        }
        struct two_xmm_regs {
            double d1;
            double d2;
        };
        struct two_xmm_regs return_two_xmm_struct(void) {
            struct two_xmm_regs retval = {234.5, 678.25};
            return retval;
        }
        static struct two_xmm_regs two_doubles_struct;
        void validate_two_doubles_struct(int code) {
            if (two_doubles_struct.d1 != 234.5 || two_doubles_struct.d2 != 678.25) {
                exit(code);
            }
        }
        int test_two_doubles_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            two_doubles_struct = return_two_xmm_struct();
            to_validate = bytes;
            validate_stack_bytes(7);
            validate_two_doubles_struct(8);
            return 0;
        }
        struct int_and_xmm {
            char c;
            double d;
        };
        struct int_and_xmm return_mixed_struct(void) {
            struct int_and_xmm retval = {125, 678.25};
            return retval;
        }
        static struct int_and_xmm mixed_struct;
        void validate_mixed_struct(int code) {
            if (mixed_struct.c != 125 || mixed_struct.d != 678.25) {
                exit(code);
            }
        }
        int test_mixed_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            mixed_struct = return_mixed_struct();
            to_validate = bytes;
            validate_stack_bytes(9);
            validate_mixed_struct(10);
            return 0;
        }
        struct stack {
            char cs[28];
        };
        struct stack return_stack_struct(void) {
            struct stack retval = {{90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
                                    100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
                                    110, 111, 112, 113, 114, 115, 116, 117}};
            return retval;
        }
        static struct stack stack_struct;
        void validate_stack_struct(int code) {
            for (int i = 0; i < 28; i = i + 1) {
                if (stack_struct.cs[i] != i + 90) {
                    exit(code);
                }
            }
        }
        int test_stack_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            stack_struct = return_stack_struct();
            to_validate = bytes;
            validate_stack_bytes(11);
            validate_stack_struct(12);
            return 0;
        }
        struct stack_irregular {
            char cs[19];
        };
        struct stack_irregular return_irregular_stack_struct(void) {
            struct stack_irregular retval = {{70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
                                              80, 81, 82, 83, 84, 85, 86, 87, 88}};
            return retval;
        }
        static struct stack_irregular irregular_stack_struct;
        void validate_irregular_stack_struct(int code) {
            for (int i = 0; i < 19; i = i + 1) {
                if (irregular_stack_struct.cs[i] != i + 70) {
                    exit(code);
                }
            }
        }
        int test_irregular_stack_struct(void) {
            struct stack_bytes bytes = {"efghijklmnopqrs"};
            irregular_stack_struct = return_irregular_stack_struct();
            to_validate = bytes;
            validate_stack_bytes(13);
            validate_irregular_stack_struct(14);
            return 0;
        }
        int main(void) {
            test_int_struct();
            test_two_int_struct();
            test_one_double_struct();
            test_two_doubles_struct();
            test_mixed_struct();
            test_stack_struct();
            test_irregular_stack_struct();
            return 0;
        }
    "#;
    let expected = r#"
        global function validate_stack_bytes(code.4) { 
            tmp.0 = to_validate[0]
            tmp.1 = &tmp.0
            tmp.2 = &string.0
            tmp.3 = strcmp(tmp.1, tmp.2)
            if !tmp.3 jump end_if_0
            exit(code.4)
        
          end_if_0:
            return 
        
            return 0
        }
        global function return_int_struct() { 
            tmp.4 = truncate 0
            one_int_reg.5[0] = tmp.4
            tmp.5 = truncate 0
            one_int_reg.5[1] = tmp.5
            tmp.6 = truncate 0
            one_int_reg.5[2] = tmp.6
            tmp.7 = truncate 0
            one_int_reg.5[3] = tmp.7
            tmp.8 = truncate 0
            one_int_reg.5[4] = tmp.8
            tmp.9 = truncate 0
            one_int_reg.5[5] = tmp.9
            tmp.10 = truncate 0
            one_int_reg.5[6] = tmp.10
            return retval.6
            return 0
        }
        global function validate_one_int_struct(code.7) { 
            i.8 = 0
        
          start_loop_0:
            tmp.11 = i.8 < 7
            if !tmp.11 jump break_loop_0
            tmp.12 = one_int_struct[0]
            tmp.13 = &tmp.12
            tmp.14 = sign_extend i.8
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=1)
            tmp.16 = *tmp.15
            if !tmp.16 jump end_if_2
            exit(code.7)
        
          end_if_2:
        
          continue_loop_0:
            tmp.17 = i.8 + 1
            i.8 = tmp.17
            jump start_loop_0
        
          break_loop_0:
            return 0
        }
        global function test_int_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.18 = return_int_struct()
            one_int_struct = tmp.18
            to_validate = bytes.9
            validate_stack_bytes(1)
            validate_one_int_struct(2)
            return 0
            return 0
        }
        global function return_two_int_struct() { 
            tmp.19 = truncate 20
            two_int_regs.10[0] = tmp.19
            tmp.20 = truncate 21
            two_int_regs.10[1] = tmp.20
            tmp.21 = truncate 22
            two_int_regs.10[2] = tmp.21
            tmp.22 = truncate 23
            two_int_regs.10[3] = tmp.22
            tmp.23 = truncate 24
            two_int_regs.10[4] = tmp.23
            tmp.24 = truncate 25
            two_int_regs.10[5] = tmp.24
            tmp.25 = truncate 26
            two_int_regs.10[6] = tmp.25
            tmp.26 = truncate 27
            two_int_regs.10[7] = tmp.26
            tmp.27 = truncate 28
            two_int_regs.10[8] = tmp.27
            tmp.28 = truncate 29
            two_int_regs.10[9] = tmp.28
            tmp.29 = truncate 30
            two_int_regs.10[10] = tmp.29
            tmp.30 = truncate 31
            two_int_regs.10[11] = tmp.30
            tmp.31 = truncate 32
            two_int_regs.10[12] = tmp.31
            tmp.32 = truncate 33
            two_int_regs.10[13] = tmp.32
            tmp.33 = truncate 34
            two_int_regs.10[14] = tmp.33
            return retval.11
            return 0
        }
        global function validate_two_int_struct(code.12) { 
            i.13 = 0
        
          start_loop_1:
            tmp.34 = i.13 < 15
            if !tmp.34 jump break_loop_1
            tmp.35 = two_int_struct[0]
            tmp.36 = &tmp.35
            tmp.37 = sign_extend i.13
            tmp.38 = add_ptr(tmp.36, index=tmp.37, scale=1)
            tmp.39 = *tmp.38
            tmp.40 = sign_extend tmp.39
            tmp.42 = i.13 + 20
            tmp.41 = tmp.40 != tmp.42
            if !tmp.41 jump end_if_4
            exit(code.12)
        
          end_if_4:
        
          continue_loop_1:
            tmp.43 = i.13 + 1
            i.13 = tmp.43
            jump start_loop_1
        
          break_loop_1:
            return 0
        }
        global function test_two_int_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.44 = return_two_int_struct()
            two_int_struct = tmp.44
            to_validate = bytes.14
            validate_stack_bytes(3)
            validate_two_int_struct(4)
            return 0
            return 0
        }
        global function return_one_xmm_struct() { 
            one_xmm_reg.15[0] = 234.5D
            return retval.16
            return 0
        }
        global function validate_one_double_struct(code.17) { 
            tmp.45 = one_double_struct[0]
            tmp.46 = tmp.45 != 234.5D
            if !tmp.46 jump end_if_6
            exit(code.17)
        
          end_if_6:
            return 0
        }
        global function test_one_double_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.47 = return_one_xmm_struct()
            one_double_struct = tmp.47
            to_validate = bytes.18
            validate_stack_bytes(5)
            validate_one_double_struct(6)
            return 0
            return 0
        }
        global function return_two_xmm_struct() { 
            two_xmm_regs.19[0] = 234.5D
            two_xmm_regs.19[8] = 678.25D
            return retval.20
            return 0
        }
        global function validate_two_doubles_struct(code.21) { 
            tmp.48 = two_doubles_struct[0]
            tmp.49 = tmp.48 != 234.5D
            if tmp.49 jump or_true_8
            tmp.52 = two_doubles_struct[1]
            tmp.53 = tmp.52 != 678.25D
            if tmp.53 jump or_true_8
            tmp.51 = 0
            jump or_end_9
        
          or_true_8:
            tmp.51 = 1
        
          or_end_9:
            if !tmp.51 jump end_if_10
            exit(code.21)
        
          end_if_10:
            return 0
        }
        global function test_two_doubles_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.54 = return_two_xmm_struct()
            two_doubles_struct = tmp.54
            to_validate = bytes.22
            validate_stack_bytes(7)
            validate_two_doubles_struct(8)
            return 0
            return 0
        }
        global function return_mixed_struct() { 
            tmp.55 = truncate 125
            int_and_xmm.23[0] = tmp.55
            int_and_xmm.23[8] = 678.25D
            return retval.24
            return 0
        }
        global function validate_mixed_struct(code.25) { 
            tmp.56 = mixed_struct[0]
            tmp.57 = sign_extend tmp.56
            tmp.58 = tmp.57 != 125
            if tmp.58 jump or_true_12
            tmp.61 = mixed_struct[1]
            tmp.62 = tmp.61 != 678.25D
            if tmp.62 jump or_true_12
            tmp.60 = 0
            jump or_end_13
        
          or_true_12:
            tmp.60 = 1
        
          or_end_13:
            if !tmp.60 jump end_if_14
            exit(code.25)
        
          end_if_14:
            return 0
        }
        global function test_mixed_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.63 = return_mixed_struct()
            mixed_struct = tmp.63
            to_validate = bytes.26
            validate_stack_bytes(9)
            validate_mixed_struct(10)
            return 0
            return 0
        }
        global function return_stack_struct() { 
            tmp.64 = truncate 90
            stack.27[0] = tmp.64
            tmp.65 = truncate 91
            stack.27[1] = tmp.65
            tmp.66 = truncate 92
            stack.27[2] = tmp.66
            tmp.67 = truncate 93
            stack.27[3] = tmp.67
            tmp.68 = truncate 94
            stack.27[4] = tmp.68
            tmp.69 = truncate 95
            stack.27[5] = tmp.69
            tmp.70 = truncate 96
            stack.27[6] = tmp.70
            tmp.71 = truncate 97
            stack.27[7] = tmp.71
            tmp.72 = truncate 98
            stack.27[8] = tmp.72
            tmp.73 = truncate 99
            stack.27[9] = tmp.73
            tmp.74 = truncate 100
            stack.27[10] = tmp.74
            tmp.75 = truncate 101
            stack.27[11] = tmp.75
            tmp.76 = truncate 102
            stack.27[12] = tmp.76
            tmp.77 = truncate 103
            stack.27[13] = tmp.77
            tmp.78 = truncate 104
            stack.27[14] = tmp.78
            tmp.79 = truncate 105
            stack.27[15] = tmp.79
            tmp.80 = truncate 106
            stack.27[16] = tmp.80
            tmp.81 = truncate 107
            stack.27[17] = tmp.81
            tmp.82 = truncate 108
            stack.27[18] = tmp.82
            tmp.83 = truncate 109
            stack.27[19] = tmp.83
            tmp.84 = truncate 110
            stack.27[20] = tmp.84
            tmp.85 = truncate 111
            stack.27[21] = tmp.85
            tmp.86 = truncate 112
            stack.27[22] = tmp.86
            tmp.87 = truncate 113
            stack.27[23] = tmp.87
            tmp.88 = truncate 114
            stack.27[24] = tmp.88
            tmp.89 = truncate 115
            stack.27[25] = tmp.89
            tmp.90 = truncate 116
            stack.27[26] = tmp.90
            tmp.91 = truncate 117
            stack.27[27] = tmp.91
            return retval.28
            return 0
        }
        global function validate_stack_struct(code.29) { 
            i.30 = 0
        
          start_loop_2:
            tmp.92 = i.30 < 28
            if !tmp.92 jump break_loop_2
            tmp.93 = stack_struct[0]
            tmp.94 = &tmp.93
            tmp.95 = sign_extend i.30
            tmp.96 = add_ptr(tmp.94, index=tmp.95, scale=1)
            tmp.97 = *tmp.96
            tmp.98 = sign_extend tmp.97
            tmp.100 = i.30 + 90
            tmp.99 = tmp.98 != tmp.100
            if !tmp.99 jump end_if_16
            exit(code.29)
        
          end_if_16:
        
          continue_loop_2:
            tmp.101 = i.30 + 1
            i.30 = tmp.101
            jump start_loop_2
        
          break_loop_2:
            return 0
        }
        global function test_stack_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.102 = return_stack_struct()
            stack_struct = tmp.102
            to_validate = bytes.31
            validate_stack_bytes(11)
            validate_stack_struct(12)
            return 0
            return 0
        }
        global function return_irregular_stack_struct() { 
            tmp.103 = truncate 70
            stack_irregular.32[0] = tmp.103
            tmp.104 = truncate 71
            stack_irregular.32[1] = tmp.104
            tmp.105 = truncate 72
            stack_irregular.32[2] = tmp.105
            tmp.106 = truncate 73
            stack_irregular.32[3] = tmp.106
            tmp.107 = truncate 74
            stack_irregular.32[4] = tmp.107
            tmp.108 = truncate 75
            stack_irregular.32[5] = tmp.108
            tmp.109 = truncate 76
            stack_irregular.32[6] = tmp.109
            tmp.110 = truncate 77
            stack_irregular.32[7] = tmp.110
            tmp.111 = truncate 78
            stack_irregular.32[8] = tmp.111
            tmp.112 = truncate 79
            stack_irregular.32[9] = tmp.112
            tmp.113 = truncate 80
            stack_irregular.32[10] = tmp.113
            tmp.114 = truncate 81
            stack_irregular.32[11] = tmp.114
            tmp.115 = truncate 82
            stack_irregular.32[12] = tmp.115
            tmp.116 = truncate 83
            stack_irregular.32[13] = tmp.116
            tmp.117 = truncate 84
            stack_irregular.32[14] = tmp.117
            tmp.118 = truncate 85
            stack_irregular.32[15] = tmp.118
            tmp.119 = truncate 86
            stack_irregular.32[16] = tmp.119
            tmp.120 = truncate 87
            stack_irregular.32[17] = tmp.120
            tmp.121 = truncate 88
            stack_irregular.32[18] = tmp.121
            return retval.33
            return 0
        }
        global function validate_irregular_stack_struct(code.34) { 
            i.35 = 0
        
          start_loop_3:
            tmp.122 = i.35 < 19
            if !tmp.122 jump break_loop_3
            tmp.123 = irregular_stack_struct[0]
            tmp.124 = &tmp.123
            tmp.125 = sign_extend i.35
            tmp.126 = add_ptr(tmp.124, index=tmp.125, scale=1)
            tmp.127 = *tmp.126
            tmp.128 = sign_extend tmp.127
            tmp.130 = i.35 + 70
            tmp.129 = tmp.128 != tmp.130
            if !tmp.129 jump end_if_18
            exit(code.34)
        
          end_if_18:
        
          continue_loop_3:
            tmp.131 = i.35 + 1
            i.35 = tmp.131
            jump start_loop_3
        
          break_loop_3:
            return 0
        }
        global function test_irregular_stack_struct() { 
            stack_bytes.3[0] = 'e'
            stack_bytes.3[1] = 'f'
            stack_bytes.3[2] = 'g'
            stack_bytes.3[3] = 'h'
            stack_bytes.3[4] = 'i'
            stack_bytes.3[5] = 'j'
            stack_bytes.3[6] = 'k'
            stack_bytes.3[7] = 'l'
            stack_bytes.3[8] = 'm'
            stack_bytes.3[9] = 'n'
            stack_bytes.3[10] = 'o'
            stack_bytes.3[11] = 'p'
            stack_bytes.3[12] = 'q'
            stack_bytes.3[13] = 'r'
            stack_bytes.3[14] = 's'
            stack_bytes.3[15] = '\0'
            tmp.132 = return_irregular_stack_struct()
            irregular_stack_struct = tmp.132
            to_validate = bytes.36
            validate_stack_bytes(13)
            validate_irregular_stack_struct(14)
            return 0
            return 0
        }
        global function main() { 
            tmp.133 = test_int_struct()
            tmp.134 = test_two_int_struct()
            tmp.135 = test_one_double_struct()
            tmp.136 = test_two_doubles_struct()
            tmp.137 = test_mixed_struct()
            tmp.138 = test_stack_struct()
            tmp.139 = test_irregular_stack_struct()
            return 0
            return 0
        }
        static irregular_stack_struct: Struct(stack_irregular.32) = zero[19]
        static mixed_struct: Struct(int_and_xmm.23) = zero[16]
        static one_double_struct: Struct(one_xmm_reg.15) = zero[8]
        static one_int_struct: Struct(one_int_reg.5) = zero[7]
        static stack_struct: Struct(stack.27) = zero[28]
        constant string.0: Array(16,Char) = "efghijklmnopqrs\\0"
        static to_validate: Struct(stack_bytes.3) = zero[16]
        static two_doubles_struct: Struct(two_xmm_regs.19) = zero[16]
        static two_int_struct: Struct(two_int_regs.10) = zero[15]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_params_and_returns_temporary_lifetime() {
    let src = r#"
        struct s {
            int arr[3];
        };
        struct s f(void) {
            struct s retval = {{1, 2, 3}};
            return retval;
        }
        int main(void) {
            int i = f().arr[0];
            int j = f().arr[1];
            int k = f().arr[2];
            if (i != 1) {
                return 1;
            }
            if (j != 2) {
                return 2;
            }
            if (k != 3) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function f() { 
            s.0[0] = 1
            s.0[4] = 2
            s.0[8] = 3
            return retval.1
            return 0
        }
        global function main() { 
            tmp.0 = f()
            tmp.1 = tmp.0[0]
            tmp.2 = &tmp.1
            tmp.3 = sign_extend 0
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=4)
            tmp.5 = *tmp.4
            i.2 = tmp.5
            tmp.6 = f()
            tmp.7 = tmp.6[0]
            tmp.8 = &tmp.7
            tmp.9 = sign_extend 1
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=4)
            tmp.11 = *tmp.10
            j.3 = tmp.11
            tmp.12 = f()
            tmp.13 = tmp.12[0]
            tmp.14 = &tmp.13
            tmp.15 = sign_extend 2
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=4)
            tmp.17 = *tmp.16
            k.4 = tmp.17
            tmp.18 = i.2 != 1
            if !tmp.18 jump end_if_0
            return 1
        
          end_if_0:
            tmp.19 = j.3 != 2
            if !tmp.19 jump end_if_2
            return 2
        
          end_if_2:
            tmp.20 = k.4 != 3
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
