use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_extra_credit_libraries_classify_unions() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        void exit(int status);
        void *malloc(unsigned long size);
        union one_double {
            double d1;
            double d2;
        };
        struct has_union_with_double {
            union one_double member;
        };
        union has_struct_with_double {
            struct has_union_with_double s;
            double arr[1];
        };
        union one_int {
            double d;
            char c;
        };
        union one_int_nested {
            union one_int oi;
            union one_double od;
        };
        union char_int_mixed {
            char arr[7];
            union char_int_mixed* union_ptr;
            unsigned int ui;
        };
        union char_int_short {
            char c;
            int i;
        };
        struct has_union {
            unsigned int i;
            union char_int_short u;
        };
        union has_struct_with_ints {
            double d;
            struct has_union s;
            unsigned long ul;
        };
        union two_doubles {
            double arr[2];
            double single;
        };
        union has_xmm_union {
            union one_double u;
            union two_doubles u2;
        };
        struct dbl_struct {
            union one_double member1;
            double member2;
        };
        union has_dbl_struct {
            struct dbl_struct member1;
        };
        union char_arr {
            char arr[11];
            int i;
        };
        union two_arrs {
            double dbl_arr[2];
            long long_arr[2];
        };
        union two_eightbyte_has_struct {
            int arr[3];
            struct dbl_struct member1;
        };
        struct char_first_eightbyte {
            char c;
            double d;
        };
        struct int_second_eightbyte {
            double d;
            int i;
        };
        union two_structs {
            struct char_first_eightbyte member1;
            struct int_second_eightbyte member2;
        };
        struct nine_bytes {
            int i;
            char arr[5];
        };
        union has_nine_byte_struct {
            char c;
            long l;
            struct nine_bytes s;
        };
        union uneven {
            char arr[5];
            unsigned char uc;
        };
        struct has_uneven_union {
            int i;
            union uneven u;
        };
        union has_other_unions {
            union uneven u;
            union two_doubles d;
            union has_nine_byte_struct n;
        };
        union union_array {
            union one_int u_arr[2];
        };
        union uneven_union_array {
            union uneven u_arr[2];
        };
        struct small {
            char arr[3];
            signed char sc;
        };
        union has_small_struct_array {
            struct small arr[3];
        };
        union gp_and_xmm {
            double d_arr[2];
            char c;
        };
        union scalar_and_struct {
            long* ptr;
            struct char_first_eightbyte cfe;
        };
        struct has_two_unions {
            union char_int_mixed member1;
            union one_double member2;
        };
        union small_struct_arr_and_dbl {
            struct small arr[2];
            union two_doubles d;
        };
        union xmm_and_gp {
            double d;
            struct int_second_eightbyte ise;
        };
        union xmm_and_gp_nested {
            union xmm_and_gp member1;
            double arr[2];
            union two_doubles d;
        };
        union lotsa_doubles {
            double arr[3];
            int i;
        };
        union lotsa_chars {
            char more_chars[18];
            char fewer_chars[5];
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        union contains_large_struct {
            int i;
            unsigned long ul;
            struct large l;
        };
        union contains_union_array {
            union gp_and_xmm arr[2];
        };
        int test_one_double(union one_double u);
        int test_has_union_with_double(struct has_union_with_double s);
        int test_has_struct_with_double(union has_struct_with_double u);
        int test_one_int(union one_int u);
        int test_one_int_nested(union one_int_nested u);
        int test_char_int_mixed(union char_int_mixed u);
        int test_has_union(struct has_union s);
        int test_has_struct_with_ints(union has_struct_with_ints u);
        int test_two_doubles(union two_doubles u);
        int test_has_xmm_union(union has_xmm_union u);
        int test_dbl_struct(struct dbl_struct s);
        int test_has_dbl_struct(union has_dbl_struct u);
        int test_char_arr(union char_arr u);
        int test_two_arrs(union two_arrs u);
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u);
        int test_two_structs(union two_structs u);
        int test_has_nine_byte_struct(union has_nine_byte_struct u);
        int test_has_uneven_union(struct has_uneven_union s);
        int test_has_other_unions(union has_other_unions u);
        int test_union_array(union union_array u);
        int test_uneven_union_array(union uneven_union_array u);
        int test_has_small_struct_array(union has_small_struct_array u);
        int test_gp_and_xmm(union gp_and_xmm u);
        int test_scalar_and_struct(union scalar_and_struct u);
        int test_has_two_unions(struct has_two_unions s);
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u);
        int test_xmm_and_gp(union xmm_and_gp u);
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u);
        int test_lotsa_doubles(union lotsa_doubles u);
        int test_lotsa_chars(union lotsa_chars u);
        int test_contains_large_struct(union contains_large_struct u);
        int test_contains_union_array(union contains_union_array u);
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5);
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp);
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2);
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp);
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2);
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs);
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven);
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct);
        union one_double return_one_double(void);
        union one_int_nested return_one_int_nested(void);
        union has_dbl_struct return_has_dbl_struct(void);
        union two_arrs return_two_arrs(void);
        union scalar_and_struct return_scalar_and_struct(void);
        union xmm_and_gp return_xmm_and_gp(void);
        union contains_union_array return_contains_union_array(void);
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin);
        struct has_uneven_union return_struct_with_union(void);
        
        int test_one_double(union one_double u) {
            return (u.d1 == -2.345e6 && u.d2 == -2.345e6);
        }
        int test_has_union_with_double(struct has_union_with_double s) {
            return (s.member.d1 == 9887.54321e44 && s.member.d2 == 9887.54321e44);
        }
        int test_has_struct_with_double(union has_struct_with_double u) {
            return (u.s.member.d1 == 9887.54321e44
                && u.arr[0] == 9887.54321e44 && u.s.member.d2 == 9887.54321e44);
        }
        int test_one_int(union one_int u) {
            return (u.d == -80. && u.c == 0);
        }
        int test_one_int_nested(union one_int_nested u) {
            return u.oi.d == 44e55 && u.oi.c == 109 && u.od.d1 == 44e55
                && u.od.d2 == 44e55;
        }
        int test_char_int_mixed(union char_int_mixed u) {
            return (strcmp(u.arr, "WXYZ") == 0 && u.ui == 1515804759);
        }
        int test_has_union(struct has_union s) {
            return (s.i == 4294954951u && s.u.c == -60);
        }
        int test_has_struct_with_ints(union has_struct_with_ints u) {
            return (u.s.i == 4294954951u && u.s.u.c == -60);
        }
        int test_two_doubles(union two_doubles u) {
            return (u.arr[0] == 10.0 && u.arr[1] == 11.0 && u.single == 10.0);
        }
        int test_has_xmm_union(union has_xmm_union u) {
            return u.u.d1 == 10.0 && u.u.d2 == 10.0 && u.u2.single == 10.0
                && u.u2.arr[0] == 10.0 && u.u2.arr[1] == 11.0;
        }
        int test_dbl_struct(struct dbl_struct s) {
            return s.member1.d1 == -2.345e6 && s.member1.d2 == -2.345e6
                && s.member2 == 123.45;
        }
        int test_has_dbl_struct(union has_dbl_struct u) {
            return u.member1.member1.d1 == -2.345e6 && u.member1.member1.d2 == -2.345e6
                && u.member1.member2 == 123.45;
        }
        int test_char_arr(union char_arr u) {
            return (strcmp(u.arr, "Chars!") == 0 && u.i == 1918986307);
        }
        int test_two_arrs(union two_arrs u) {
            return (u.dbl_arr[0] == 13e4 && u.dbl_arr[1] == 14.5
                && u.long_arr[0] == 4683669945186254848 && u.long_arr[1] == 4624352392379367424);
        }
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u) {
            return (u.arr[0] == 100 && u.arr[1] == 200 && u.arr[2] == 300
                && u.member1.member1.d1 == 4.24399158242461027606e-312);
        }
        int test_two_structs(union two_structs u) {
            return (u.member1.c == 'x' && u.member1.d == 55.5e5 && u.member2.i == 0);
        }
        int test_has_nine_byte_struct(union has_nine_byte_struct u) {
            if (u.l != -71777214294589696l || u.c != 0) {
                return 0;
            }
            if (u.s.i != -16711936) {
                return 0;
            }
            for (int i = 0; i < 5; i = i + 1) {
                int expected = i % 2 ? -1 : 0;
                if (u.s.arr[i] != expected) {
                    return 0;
                }
            }
            return 1;
        }
        int test_has_uneven_union(struct has_uneven_union s) {
            return s.i == -2147483647 && strcmp(s.u.arr, "!@#$") == 0 && s.u.uc == 33;
        }
        int test_has_other_unions(union has_other_unions u) {
            if (u.n.l != -71777214294589696l) {
                return 0;
            }
            for (int i = 0; i < 5; i = i + 1) {
                int expected = i % 2 ? -1 : 0;
                if (u.n.s.arr[i] != expected) {
                    return 0;
                }
            }
            return 1;
        }
        int test_union_array(union union_array u) {
            return (u.u_arr->d == -20. && u.u_arr[1].d == -30.);
        }
        int test_uneven_union_array(union uneven_union_array u) {
            return (strcmp(u.u_arr[0].arr, "QWER") == 0 && strcmp(u.u_arr[1].arr, "TYUI") == 0);
        }
        int test_has_small_struct_array(union has_small_struct_array u) {
            return strcmp(u.arr[0].arr, "AS") == 0 && u.arr[0].sc == 10
                && strcmp(u.arr[1].arr, "DF") == 0 && u.arr[1].sc == 11
                && strcmp(u.arr[2].arr, "GH") == 0 && u.arr[2].sc == 12;
        }
        int test_gp_and_xmm(union gp_and_xmm u) {
            return u.d_arr[0] == 11. && u.d_arr[1] == 12.;
        }
        int test_scalar_and_struct(union scalar_and_struct u) {
            return u.cfe.c == -5 && u.cfe.d == -88.8;
        }
        int test_has_two_unions(struct has_two_unions s) {
            if (strcmp(s.member1.arr, "WXYZ")) {
                return 0;
            }
            if (s.member2.d1 != -2.345e6) {
                return 0;
            }
            return 1;
        }
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u) {
            return (u.d.arr[0] == -22. && u.d.arr[1] == -32.);
        }
        int test_xmm_and_gp(union xmm_and_gp u) {
            return (u.ise.d == -8. && u.ise.i == -8);
        }
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u) {
            return (u.member1.ise.d == -8. && u.member1.ise.i == -8);
        }
        int test_lotsa_doubles(union lotsa_doubles u) {
            return u.arr[0] == 99. && u.arr[1] == 98. && u.arr[2] == 97;
        }
        int test_lotsa_chars(union lotsa_chars u) {
            return !strcmp(u.more_chars, "asflakjsdflkjs");
        }
        int test_contains_large_struct(union contains_large_struct u) {
            return u.l.i == 100 && u.l.d == 100. && !strcmp(u.l.arr, "A struct!");
        }
        int test_contains_union_array(union contains_union_array u) {
            union gp_and_xmm a = u.arr[0];
            union gp_and_xmm b = u.arr[1];
            if (a.d_arr[0] != 11. || a.d_arr[1] != 12.) {
                return 0;
            }
            if (b.d_arr[1] != -1 || b.c != 0) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function test_one_double(u.142) { 
            tmp.0 = u.142[0]
            tmp.2 = - 2345000D
            tmp.1 = tmp.0 == tmp.2
            if !tmp.1 jump and_false_0
            tmp.5 = u.142[0]
            tmp.7 = - 2345000D
            tmp.6 = tmp.5 == tmp.7
            if !tmp.6 jump and_false_0
            tmp.4 = 1
            jump and_end_1
        
          and_false_0:
            tmp.4 = 0
        
          and_end_1:
            return tmp.4
            return 0
        }
        global function test_has_union_with_double(s.143) { 
            tmp.8 = s.143[0]
            tmp.9 = tmp.8 == 988754321000000000000000000000000000000000000000D
            if !tmp.9 jump and_false_2
            tmp.12 = s.143[0]
            tmp.13 = tmp.12 == 988754321000000000000000000000000000000000000000D
            if !tmp.13 jump and_false_2
            tmp.11 = 1
            jump and_end_3
        
          and_false_2:
            tmp.11 = 0
        
          and_end_3:
            return tmp.11
            return 0
        }
        global function test_has_struct_with_double(u.144) { 
            tmp.14 = u.144[0]
            tmp.15 = tmp.14 == 988754321000000000000000000000000000000000000000D
            if !tmp.15 jump and_false_4
            tmp.18 = &u.144
            tmp.19 = sign_extend 0
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=8)
            tmp.21 = *tmp.20
            tmp.22 = tmp.21 == 988754321000000000000000000000000000000000000000D
            if !tmp.22 jump and_false_4
            tmp.17 = 1
            jump and_end_5
        
          and_false_4:
            tmp.17 = 0
        
          and_end_5:
            if !tmp.17 jump and_false_6
            tmp.25 = u.144[0]
            tmp.26 = tmp.25 == 988754321000000000000000000000000000000000000000D
            if !tmp.26 jump and_false_6
            tmp.24 = 1
            jump and_end_7
        
          and_false_6:
            tmp.24 = 0
        
          and_end_7:
            return tmp.24
            return 0
        }
        global function test_one_int(u.145) { 
            tmp.27 = u.145[0]
            tmp.29 = - 80D
            tmp.28 = tmp.27 == tmp.29
            if !tmp.28 jump and_false_8
            tmp.32 = u.145[0]
            tmp.33 = sign_extend tmp.32
            tmp.34 = tmp.33 == 0
            if !tmp.34 jump and_false_8
            tmp.31 = 1
            jump and_end_9
        
          and_false_8:
            tmp.31 = 0
        
          and_end_9:
            return tmp.31
            return 0
        }
        global function test_one_int_nested(u.146) { 
            tmp.35 = u.146[0]
            tmp.36 = tmp.35 == 440000000000000000000000000000000000000000000000000000000D
            if !tmp.36 jump and_false_10
            tmp.39 = u.146[0]
            tmp.40 = sign_extend tmp.39
            tmp.41 = tmp.40 == 109
            if !tmp.41 jump and_false_10
            tmp.38 = 1
            jump and_end_11
        
          and_false_10:
            tmp.38 = 0
        
          and_end_11:
            if !tmp.38 jump and_false_12
            tmp.44 = u.146[0]
            tmp.45 = tmp.44 == 440000000000000000000000000000000000000000000000000000000D
            if !tmp.45 jump and_false_12
            tmp.43 = 1
            jump and_end_13
        
          and_false_12:
            tmp.43 = 0
        
          and_end_13:
            if !tmp.43 jump and_false_14
            tmp.48 = u.146[0]
            tmp.49 = tmp.48 == 440000000000000000000000000000000000000000000000000000000D
            if !tmp.49 jump and_false_14
            tmp.47 = 1
            jump and_end_15
        
          and_false_14:
            tmp.47 = 0
        
          and_end_15:
            return tmp.47
            return 0
        }
        global function test_char_int_mixed(u.147) { 
            tmp.50 = &u.147
            tmp.51 = &string.0
            tmp.52 = strcmp(tmp.50, tmp.51)
            tmp.53 = tmp.52 == 0
            if !tmp.53 jump and_false_16
            tmp.56 = u.147[0]
            tmp.58 = 1515804759
            tmp.57 = tmp.56 == tmp.58
            if !tmp.57 jump and_false_16
            tmp.55 = 1
            jump and_end_17
        
          and_false_16:
            tmp.55 = 0
        
          and_end_17:
            return tmp.55
            return 0
        }
        global function test_has_union(s.148) { 
            tmp.59 = s.148[0]
            tmp.60 = tmp.59 == 4294954951U
            if !tmp.60 jump and_false_18
            tmp.63 = s.148[4]
            tmp.64 = sign_extend tmp.63
            tmp.66 = - 60
            tmp.65 = tmp.64 == tmp.66
            if !tmp.65 jump and_false_18
            tmp.62 = 1
            jump and_end_19
        
          and_false_18:
            tmp.62 = 0
        
          and_end_19:
            return tmp.62
            return 0
        }
        global function test_has_struct_with_ints(u.149) { 
            tmp.67 = u.149[0]
            tmp.68 = tmp.67 == 4294954951U
            if !tmp.68 jump and_false_20
            tmp.71 = u.149[4]
            tmp.72 = sign_extend tmp.71
            tmp.74 = - 60
            tmp.73 = tmp.72 == tmp.74
            if !tmp.73 jump and_false_20
            tmp.70 = 1
            jump and_end_21
        
          and_false_20:
            tmp.70 = 0
        
          and_end_21:
            return tmp.70
            return 0
        }
        global function test_two_doubles(u.150) { 
            tmp.75 = &u.150
            tmp.76 = sign_extend 0
            tmp.77 = add_ptr(tmp.75, index=tmp.76, scale=8)
            tmp.78 = *tmp.77
            tmp.79 = tmp.78 == 10D
            if !tmp.79 jump and_false_22
            tmp.82 = &u.150
            tmp.83 = sign_extend 1
            tmp.84 = add_ptr(tmp.82, index=tmp.83, scale=8)
            tmp.85 = *tmp.84
            tmp.86 = tmp.85 == 11D
            if !tmp.86 jump and_false_22
            tmp.81 = 1
            jump and_end_23
        
          and_false_22:
            tmp.81 = 0
        
          and_end_23:
            if !tmp.81 jump and_false_24
            tmp.89 = u.150[0]
            tmp.90 = tmp.89 == 10D
            if !tmp.90 jump and_false_24
            tmp.88 = 1
            jump and_end_25
        
          and_false_24:
            tmp.88 = 0
        
          and_end_25:
            return tmp.88
            return 0
        }
        global function test_has_xmm_union(u.151) { 
            tmp.91 = u.151[0]
            tmp.92 = tmp.91 == 10D
            if !tmp.92 jump and_false_26
            tmp.95 = u.151[0]
            tmp.96 = tmp.95 == 10D
            if !tmp.96 jump and_false_26
            tmp.94 = 1
            jump and_end_27
        
          and_false_26:
            tmp.94 = 0
        
          and_end_27:
            if !tmp.94 jump and_false_28
            tmp.99 = u.151[0]
            tmp.100 = tmp.99 == 10D
            if !tmp.100 jump and_false_28
            tmp.98 = 1
            jump and_end_29
        
          and_false_28:
            tmp.98 = 0
        
          and_end_29:
            if !tmp.98 jump and_false_30
            tmp.103 = &u.151
            tmp.104 = sign_extend 0
            tmp.105 = add_ptr(tmp.103, index=tmp.104, scale=8)
            tmp.106 = *tmp.105
            tmp.107 = tmp.106 == 10D
            if !tmp.107 jump and_false_30
            tmp.102 = 1
            jump and_end_31
        
          and_false_30:
            tmp.102 = 0
        
          and_end_31:
            if !tmp.102 jump and_false_32
            tmp.110 = &u.151
            tmp.111 = sign_extend 1
            tmp.112 = add_ptr(tmp.110, index=tmp.111, scale=8)
            tmp.113 = *tmp.112
            tmp.114 = tmp.113 == 11D
            if !tmp.114 jump and_false_32
            tmp.109 = 1
            jump and_end_33
        
          and_false_32:
            tmp.109 = 0
        
          and_end_33:
            return tmp.109
            return 0
        }
        global function test_dbl_struct(s.152) { 
            tmp.115 = s.152[0]
            tmp.117 = - 2345000D
            tmp.116 = tmp.115 == tmp.117
            if !tmp.116 jump and_false_34
            tmp.120 = s.152[0]
            tmp.122 = - 2345000D
            tmp.121 = tmp.120 == tmp.122
            if !tmp.121 jump and_false_34
            tmp.119 = 1
            jump and_end_35
        
          and_false_34:
            tmp.119 = 0
        
          and_end_35:
            if !tmp.119 jump and_false_36
            tmp.125 = s.152[8]
            tmp.126 = tmp.125 == 123.45D
            if !tmp.126 jump and_false_36
            tmp.124 = 1
            jump and_end_37
        
          and_false_36:
            tmp.124 = 0
        
          and_end_37:
            return tmp.124
            return 0
        }
        global function test_has_dbl_struct(u.153) { 
            tmp.127 = u.153[0]
            tmp.129 = - 2345000D
            tmp.128 = tmp.127 == tmp.129
            if !tmp.128 jump and_false_38
            tmp.132 = u.153[0]
            tmp.134 = - 2345000D
            tmp.133 = tmp.132 == tmp.134
            if !tmp.133 jump and_false_38
            tmp.131 = 1
            jump and_end_39
        
          and_false_38:
            tmp.131 = 0
        
          and_end_39:
            if !tmp.131 jump and_false_40
            tmp.137 = u.153[8]
            tmp.138 = tmp.137 == 123.45D
            if !tmp.138 jump and_false_40
            tmp.136 = 1
            jump and_end_41
        
          and_false_40:
            tmp.136 = 0
        
          and_end_41:
            return tmp.136
            return 0
        }
        global function test_char_arr(u.154) { 
            tmp.139 = &u.154
            tmp.140 = &string.1
            tmp.141 = strcmp(tmp.139, tmp.140)
            tmp.142 = tmp.141 == 0
            if !tmp.142 jump and_false_42
            tmp.145 = u.154[0]
            tmp.146 = tmp.145 == 1918986307
            if !tmp.146 jump and_false_42
            tmp.144 = 1
            jump and_end_43
        
          and_false_42:
            tmp.144 = 0
        
          and_end_43:
            return tmp.144
            return 0
        }
        global function test_two_arrs(u.155) { 
            tmp.147 = &u.155
            tmp.148 = sign_extend 0
            tmp.149 = add_ptr(tmp.147, index=tmp.148, scale=8)
            tmp.150 = *tmp.149
            tmp.151 = tmp.150 == 130000D
            if !tmp.151 jump and_false_44
            tmp.154 = &u.155
            tmp.155 = sign_extend 1
            tmp.156 = add_ptr(tmp.154, index=tmp.155, scale=8)
            tmp.157 = *tmp.156
            tmp.158 = tmp.157 == 14.5D
            if !tmp.158 jump and_false_44
            tmp.153 = 1
            jump and_end_45
        
          and_false_44:
            tmp.153 = 0
        
          and_end_45:
            if !tmp.153 jump and_false_46
            tmp.161 = &u.155
            tmp.162 = sign_extend 0
            tmp.163 = add_ptr(tmp.161, index=tmp.162, scale=8)
            tmp.164 = *tmp.163
            tmp.165 = tmp.164 == 4683669945186254848L
            if !tmp.165 jump and_false_46
            tmp.160 = 1
            jump and_end_47
        
          and_false_46:
            tmp.160 = 0
        
          and_end_47:
            if !tmp.160 jump and_false_48
            tmp.168 = &u.155
            tmp.169 = sign_extend 1
            tmp.170 = add_ptr(tmp.168, index=tmp.169, scale=8)
            tmp.171 = *tmp.170
            tmp.172 = tmp.171 == 4624352392379367424L
            if !tmp.172 jump and_false_48
            tmp.167 = 1
            jump and_end_49
        
          and_false_48:
            tmp.167 = 0
        
          and_end_49:
            return tmp.167
            return 0
        }
        global function test_two_eightbyte_has_struct(u.156) { 
            tmp.173 = &u.156
            tmp.174 = sign_extend 0
            tmp.175 = add_ptr(tmp.173, index=tmp.174, scale=4)
            tmp.176 = *tmp.175
            tmp.177 = tmp.176 == 100
            if !tmp.177 jump and_false_50
            tmp.180 = &u.156
            tmp.181 = sign_extend 1
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=4)
            tmp.183 = *tmp.182
            tmp.184 = tmp.183 == 200
            if !tmp.184 jump and_false_50
            tmp.179 = 1
            jump and_end_51
        
          and_false_50:
            tmp.179 = 0
        
          and_end_51:
            if !tmp.179 jump and_false_52
            tmp.187 = &u.156
            tmp.188 = sign_extend 2
            tmp.189 = add_ptr(tmp.187, index=tmp.188, scale=4)
            tmp.190 = *tmp.189
            tmp.191 = tmp.190 == 300
            if !tmp.191 jump and_false_52
            tmp.186 = 1
            jump and_end_53
        
          and_false_52:
            tmp.186 = 0
        
          and_end_53:
            if !tmp.186 jump and_false_54
            tmp.194 = u.156[0]
            tmp.195 = tmp.194 == 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004243991582425D
            if !tmp.195 jump and_false_54
            tmp.193 = 1
            jump and_end_55
        
          and_false_54:
            tmp.193 = 0
        
          and_end_55:
            return tmp.193
            return 0
        }
        global function test_two_structs(u.157) { 
            tmp.196 = u.157[0]
            tmp.197 = sign_extend tmp.196
            tmp.198 = tmp.197 == 120
            if !tmp.198 jump and_false_56
            tmp.201 = u.157[8]
            tmp.202 = tmp.201 == 5550000D
            if !tmp.202 jump and_false_56
            tmp.200 = 1
            jump and_end_57
        
          and_false_56:
            tmp.200 = 0
        
          and_end_57:
            if !tmp.200 jump and_false_58
            tmp.205 = u.157[8]
            tmp.206 = tmp.205 == 0
            if !tmp.206 jump and_false_58
            tmp.204 = 1
            jump and_end_59
        
          and_false_58:
            tmp.204 = 0
        
          and_end_59:
            return tmp.204
            return 0
        }
        global function test_has_nine_byte_struct(u.158) { 
            tmp.207 = u.158[0]
            tmp.209 = - 71777214294589696L
            tmp.208 = tmp.207 != tmp.209
            if tmp.208 jump or_true_60
            tmp.212 = u.158[0]
            tmp.213 = sign_extend tmp.212
            tmp.214 = tmp.213 != 0
            if tmp.214 jump or_true_60
            tmp.211 = 0
            jump or_end_61
        
          or_true_60:
            tmp.211 = 1
        
          or_end_61:
            if !tmp.211 jump end_if_62
            return 0
        
          end_if_62:
            tmp.215 = u.158[0]
            tmp.217 = - 16711936
            tmp.216 = tmp.215 != tmp.217
            if !tmp.216 jump end_if_64
            return 0
        
          end_if_64:
            i.159 = 0
        
          start_loop_0:
            tmp.218 = i.159 < 5
            if !tmp.218 jump break_loop_0
            tmp.219 = i.159 % 2
            if !tmp.219 jump else_67
            tmp.221 = - 1
            tmp.220 = tmp.221
            jump end_if_66
        
          else_67:
            tmp.220 = 0
        
          end_if_66:
            expected.160 = tmp.220
            tmp.222 = &u.158
            tmp.222 = add_ptr(tmp.222, index=4L, scale=1)
            tmp.223 = sign_extend i.159
            tmp.224 = add_ptr(tmp.222, index=tmp.223, scale=1)
            tmp.225 = *tmp.224
            tmp.226 = sign_extend tmp.225
            tmp.227 = tmp.226 != expected.160
            if !tmp.227 jump end_if_68
            return 0
        
          end_if_68:
        
          continue_loop_0:
            tmp.228 = i.159 + 1
            i.159 = tmp.228
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
        global function test_has_uneven_union(s.161) { 
            tmp.229 = s.161[0]
            tmp.231 = - 2147483647
            tmp.230 = tmp.229 == tmp.231
            if !tmp.230 jump and_false_70
            tmp.234 = &s.161
            tmp.234 = add_ptr(tmp.234, index=4L, scale=1)
            tmp.235 = &string.2
            tmp.236 = strcmp(tmp.234, tmp.235)
            tmp.237 = tmp.236 == 0
            if !tmp.237 jump and_false_70
            tmp.233 = 1
            jump and_end_71
        
          and_false_70:
            tmp.233 = 0
        
          and_end_71:
            if !tmp.233 jump and_false_72
            tmp.240 = s.161[4]
            tmp.241 = zero_extend tmp.240
            tmp.242 = tmp.241 == 33
            if !tmp.242 jump and_false_72
            tmp.239 = 1
            jump and_end_73
        
          and_false_72:
            tmp.239 = 0
        
          and_end_73:
            return tmp.239
            return 0
        }
        global function test_has_other_unions(u.162) { 
            tmp.243 = u.162[0]
            tmp.245 = - 71777214294589696L
            tmp.244 = tmp.243 != tmp.245
            if !tmp.244 jump end_if_74
            return 0
        
          end_if_74:
            i.163 = 0
        
          start_loop_1:
            tmp.246 = i.163 < 5
            if !tmp.246 jump break_loop_1
            tmp.247 = i.163 % 2
            if !tmp.247 jump else_77
            tmp.249 = - 1
            tmp.248 = tmp.249
            jump end_if_76
        
          else_77:
            tmp.248 = 0
        
          end_if_76:
            expected.164 = tmp.248
            tmp.250 = &u.162
            tmp.250 = add_ptr(tmp.250, index=4L, scale=1)
            tmp.251 = sign_extend i.163
            tmp.252 = add_ptr(tmp.250, index=tmp.251, scale=1)
            tmp.253 = *tmp.252
            tmp.254 = sign_extend tmp.253
            tmp.255 = tmp.254 != expected.164
            if !tmp.255 jump end_if_78
            return 0
        
          end_if_78:
        
          continue_loop_1:
            tmp.256 = i.163 + 1
            i.163 = tmp.256
            jump start_loop_1
        
          break_loop_1:
            return 1
            return 0
        }
        global function test_union_array(u.165) { 
            tmp.257 = &u.165
            tmp.258 = *tmp.257
            tmp.260 = - 20D
            tmp.259 = tmp.258 == tmp.260
            if !tmp.259 jump and_false_80
            tmp.263 = &u.165
            tmp.264 = sign_extend 1
            tmp.265 = add_ptr(tmp.263, index=tmp.264, scale=8)
            tmp.266 = *tmp.265
            tmp.268 = - 30D
            tmp.267 = tmp.266 == tmp.268
            if !tmp.267 jump and_false_80
            tmp.262 = 1
            jump and_end_81
        
          and_false_80:
            tmp.262 = 0
        
          and_end_81:
            return tmp.262
            return 0
        }
        global function test_uneven_union_array(u.166) { 
            tmp.269 = &u.166
            tmp.270 = sign_extend 0
            tmp.271 = add_ptr(tmp.269, index=tmp.270, scale=5)
            tmp.272 = &string.3
            tmp.273 = strcmp(tmp.271, tmp.272)
            tmp.274 = tmp.273 == 0
            if !tmp.274 jump and_false_82
            tmp.277 = &u.166
            tmp.278 = sign_extend 1
            tmp.279 = add_ptr(tmp.277, index=tmp.278, scale=5)
            tmp.280 = &string.4
            tmp.281 = strcmp(tmp.279, tmp.280)
            tmp.282 = tmp.281 == 0
            if !tmp.282 jump and_false_82
            tmp.276 = 1
            jump and_end_83
        
          and_false_82:
            tmp.276 = 0
        
          and_end_83:
            return tmp.276
            return 0
        }
        global function test_has_small_struct_array(u.167) { 
            tmp.283 = &u.167
            tmp.284 = sign_extend 0
            tmp.285 = add_ptr(tmp.283, index=tmp.284, scale=4)
            tmp.286 = &string.5
            tmp.287 = strcmp(tmp.285, tmp.286)
            tmp.288 = tmp.287 == 0
            if !tmp.288 jump and_false_84
            tmp.291 = &u.167
            tmp.292 = sign_extend 0
            tmp.293 = add_ptr(tmp.291, index=tmp.292, scale=4)
            tmp.294 = add_ptr(tmp.293, index=3L, scale=1)
            tmp.295 = *tmp.294
            tmp.296 = sign_extend tmp.295
            tmp.297 = tmp.296 == 10
            if !tmp.297 jump and_false_84
            tmp.290 = 1
            jump and_end_85
        
          and_false_84:
            tmp.290 = 0
        
          and_end_85:
            if !tmp.290 jump and_false_86
            tmp.300 = &u.167
            tmp.301 = sign_extend 1
            tmp.302 = add_ptr(tmp.300, index=tmp.301, scale=4)
            tmp.303 = &string.6
            tmp.304 = strcmp(tmp.302, tmp.303)
            tmp.305 = tmp.304 == 0
            if !tmp.305 jump and_false_86
            tmp.299 = 1
            jump and_end_87
        
          and_false_86:
            tmp.299 = 0
        
          and_end_87:
            if !tmp.299 jump and_false_88
            tmp.308 = &u.167
            tmp.309 = sign_extend 1
            tmp.310 = add_ptr(tmp.308, index=tmp.309, scale=4)
            tmp.311 = add_ptr(tmp.310, index=3L, scale=1)
            tmp.312 = *tmp.311
            tmp.313 = sign_extend tmp.312
            tmp.314 = tmp.313 == 11
            if !tmp.314 jump and_false_88
            tmp.307 = 1
            jump and_end_89
        
          and_false_88:
            tmp.307 = 0
        
          and_end_89:
            if !tmp.307 jump and_false_90
            tmp.317 = &u.167
            tmp.318 = sign_extend 2
            tmp.319 = add_ptr(tmp.317, index=tmp.318, scale=4)
            tmp.320 = &string.7
            tmp.321 = strcmp(tmp.319, tmp.320)
            tmp.322 = tmp.321 == 0
            if !tmp.322 jump and_false_90
            tmp.316 = 1
            jump and_end_91
        
          and_false_90:
            tmp.316 = 0
        
          and_end_91:
            if !tmp.316 jump and_false_92
            tmp.325 = &u.167
            tmp.326 = sign_extend 2
            tmp.327 = add_ptr(tmp.325, index=tmp.326, scale=4)
            tmp.328 = add_ptr(tmp.327, index=3L, scale=1)
            tmp.329 = *tmp.328
            tmp.330 = sign_extend tmp.329
            tmp.331 = tmp.330 == 12
            if !tmp.331 jump and_false_92
            tmp.324 = 1
            jump and_end_93
        
          and_false_92:
            tmp.324 = 0
        
          and_end_93:
            return tmp.324
            return 0
        }
        global function test_gp_and_xmm(u.168) { 
            tmp.332 = &u.168
            tmp.333 = sign_extend 0
            tmp.334 = add_ptr(tmp.332, index=tmp.333, scale=8)
            tmp.335 = *tmp.334
            tmp.336 = tmp.335 == 11D
            if !tmp.336 jump and_false_94
            tmp.339 = &u.168
            tmp.340 = sign_extend 1
            tmp.341 = add_ptr(tmp.339, index=tmp.340, scale=8)
            tmp.342 = *tmp.341
            tmp.343 = tmp.342 == 12D
            if !tmp.343 jump and_false_94
            tmp.338 = 1
            jump and_end_95
        
          and_false_94:
            tmp.338 = 0
        
          and_end_95:
            return tmp.338
            return 0
        }
        global function test_scalar_and_struct(u.169) { 
            tmp.344 = u.169[0]
            tmp.345 = sign_extend tmp.344
            tmp.347 = - 5
            tmp.346 = tmp.345 == tmp.347
            if !tmp.346 jump and_false_96
            tmp.350 = u.169[8]
            tmp.352 = - 88.8D
            tmp.351 = tmp.350 == tmp.352
            if !tmp.351 jump and_false_96
            tmp.349 = 1
            jump and_end_97
        
          and_false_96:
            tmp.349 = 0
        
          and_end_97:
            return tmp.349
            return 0
        }
        global function test_has_two_unions(s.170) { 
            tmp.353 = &s.170
            tmp.354 = &string.0
            tmp.355 = strcmp(tmp.353, tmp.354)
            if !tmp.355 jump end_if_98
            return 0
        
          end_if_98:
            tmp.356 = s.170[8]
            tmp.358 = - 2345000D
            tmp.357 = tmp.356 != tmp.358
            if !tmp.357 jump end_if_100
            return 0
        
          end_if_100:
            return 1
            return 0
        }
        global function test_small_struct_arr_and_dbl(u.171) { 
            tmp.359 = &u.171
            tmp.360 = sign_extend 0
            tmp.361 = add_ptr(tmp.359, index=tmp.360, scale=8)
            tmp.362 = *tmp.361
            tmp.364 = - 22D
            tmp.363 = tmp.362 == tmp.364
            if !tmp.363 jump and_false_102
            tmp.367 = &u.171
            tmp.368 = sign_extend 1
            tmp.369 = add_ptr(tmp.367, index=tmp.368, scale=8)
            tmp.370 = *tmp.369
            tmp.372 = - 32D
            tmp.371 = tmp.370 == tmp.372
            if !tmp.371 jump and_false_102
            tmp.366 = 1
            jump and_end_103
        
          and_false_102:
            tmp.366 = 0
        
          and_end_103:
            return tmp.366
            return 0
        }
        global function test_xmm_and_gp(u.172) { 
            tmp.373 = u.172[0]
            tmp.375 = - 8D
            tmp.374 = tmp.373 == tmp.375
            if !tmp.374 jump and_false_104
            tmp.378 = u.172[8]
            tmp.380 = - 8
            tmp.379 = tmp.378 == tmp.380
            if !tmp.379 jump and_false_104
            tmp.377 = 1
            jump and_end_105
        
          and_false_104:
            tmp.377 = 0
        
          and_end_105:
            return tmp.377
            return 0
        }
        global function test_xmm_and_gp_nested(u.173) { 
            tmp.381 = u.173[0]
            tmp.383 = - 8D
            tmp.382 = tmp.381 == tmp.383
            if !tmp.382 jump and_false_106
            tmp.386 = u.173[8]
            tmp.388 = - 8
            tmp.387 = tmp.386 == tmp.388
            if !tmp.387 jump and_false_106
            tmp.385 = 1
            jump and_end_107
        
          and_false_106:
            tmp.385 = 0
        
          and_end_107:
            return tmp.385
            return 0
        }
        global function test_lotsa_doubles(u.174) { 
            tmp.389 = &u.174
            tmp.390 = sign_extend 0
            tmp.391 = add_ptr(tmp.389, index=tmp.390, scale=8)
            tmp.392 = *tmp.391
            tmp.393 = tmp.392 == 99D
            if !tmp.393 jump and_false_108
            tmp.396 = &u.174
            tmp.397 = sign_extend 1
            tmp.398 = add_ptr(tmp.396, index=tmp.397, scale=8)
            tmp.399 = *tmp.398
            tmp.400 = tmp.399 == 98D
            if !tmp.400 jump and_false_108
            tmp.395 = 1
            jump and_end_109
        
          and_false_108:
            tmp.395 = 0
        
          and_end_109:
            if !tmp.395 jump and_false_110
            tmp.403 = &u.174
            tmp.404 = sign_extend 2
            tmp.405 = add_ptr(tmp.403, index=tmp.404, scale=8)
            tmp.406 = *tmp.405
            tmp.408 = int_to_double 97
            tmp.407 = tmp.406 == tmp.408
            if !tmp.407 jump and_false_110
            tmp.402 = 1
            jump and_end_111
        
          and_false_110:
            tmp.402 = 0
        
          and_end_111:
            return tmp.402
            return 0
        }
        global function test_lotsa_chars(u.175) { 
            tmp.409 = &u.175
            tmp.410 = &string.8
            tmp.411 = strcmp(tmp.409, tmp.410)
            tmp.412 = ! tmp.411
            return tmp.412
            return 0
        }
        global function test_contains_large_struct(u.176) { 
            tmp.413 = u.176[0]
            tmp.414 = tmp.413 == 100
            if !tmp.414 jump and_false_112
            tmp.417 = u.176[8]
            tmp.418 = tmp.417 == 100D
            if !tmp.418 jump and_false_112
            tmp.416 = 1
            jump and_end_113
        
          and_false_112:
            tmp.416 = 0
        
          and_end_113:
            if !tmp.416 jump and_false_114
            tmp.421 = &u.176
            tmp.421 = add_ptr(tmp.421, index=16L, scale=1)
            tmp.422 = &string.9
            tmp.423 = strcmp(tmp.421, tmp.422)
            tmp.424 = ! tmp.423
            if !tmp.424 jump and_false_114
            tmp.420 = 1
            jump and_end_115
        
          and_false_114:
            tmp.420 = 0
        
          and_end_115:
            return tmp.420
            return 0
        }
        global function test_contains_union_array(u.177) { 
            tmp.425 = &u.177
            tmp.426 = sign_extend 0
            tmp.427 = add_ptr(tmp.425, index=tmp.426, scale=16)
            tmp.428 = *tmp.427
            a.178 = tmp.428
            tmp.429 = &u.177
            tmp.430 = sign_extend 1
            tmp.431 = add_ptr(tmp.429, index=tmp.430, scale=16)
            tmp.432 = *tmp.431
            b.179 = tmp.432
            tmp.433 = &a.178
            tmp.434 = sign_extend 0
            tmp.435 = add_ptr(tmp.433, index=tmp.434, scale=8)
            tmp.436 = *tmp.435
            tmp.437 = tmp.436 != 11D
            if tmp.437 jump or_true_116
            tmp.440 = &a.178
            tmp.441 = sign_extend 1
            tmp.442 = add_ptr(tmp.440, index=tmp.441, scale=8)
            tmp.443 = *tmp.442
            tmp.444 = tmp.443 != 12D
            if tmp.444 jump or_true_116
            tmp.439 = 0
            jump or_end_117
        
          or_true_116:
            tmp.439 = 1
        
          or_end_117:
            if !tmp.439 jump end_if_118
            return 0
        
          end_if_118:
            tmp.445 = &b.179
            tmp.446 = sign_extend 1
            tmp.447 = add_ptr(tmp.445, index=tmp.446, scale=8)
            tmp.448 = *tmp.447
            tmp.450 = - 1
            tmp.451 = int_to_double tmp.450
            tmp.449 = tmp.448 != tmp.451
            if tmp.449 jump or_true_120
            tmp.454 = b.179[0]
            tmp.455 = sign_extend tmp.454
            tmp.456 = tmp.455 != 0
            if tmp.456 jump or_true_120
            tmp.453 = 0
            jump or_end_121
        
          or_true_120:
            tmp.453 = 1
        
          or_end_121:
            if !tmp.453 jump end_if_122
            return 0
        
          end_if_122:
            return 1
            return 0
        }
        constant string.0: Array(5,Char) = "WXYZ\\0"
        constant string.1: Array(7,Char) = "Chars!\\0"
        constant string.2: Array(5,Char) = "!@#$\\0"
        constant string.3: Array(5,Char) = "QWER\\0"
        constant string.4: Array(5,Char) = "TYUI\\0"
        constant string.5: Array(3,Char) = "AS\\0"
        constant string.6: Array(3,Char) = "DF\\0"
        constant string.7: Array(3,Char) = "GH\\0"
        constant string.8: Array(15,Char) = "asflakjsdflkjs\\0"
        constant string.9: Array(10,Char) = "A struct!\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_classify_unions_client() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        void exit(int status);
        void *malloc(unsigned long size);
        union one_double {
            double d1;
            double d2;
        };
        struct has_union_with_double {
            union one_double member;
        };
        union has_struct_with_double {
            struct has_union_with_double s;
            double arr[1];
        };
        union one_int {
            double d;
            char c;
        };
        union one_int_nested {
            union one_int oi;
            union one_double od;
        };
        union char_int_mixed {
            char arr[7];
            union char_int_mixed* union_ptr;
            unsigned int ui;
        };
        union char_int_short {
            char c;
            int i;
        };
        struct has_union {
            unsigned int i;
            union char_int_short u;
        };
        union has_struct_with_ints {
            double d;
            struct has_union s;
            unsigned long ul;
        };
        union two_doubles {
            double arr[2];
            double single;
        };
        union has_xmm_union {
            union one_double u;
            union two_doubles u2;
        };
        struct dbl_struct {
            union one_double member1;
            double member2;
        };
        union has_dbl_struct {
            struct dbl_struct member1;
        };
        union char_arr {
            char arr[11];
            int i;
        };
        union two_arrs {
            double dbl_arr[2];
            long long_arr[2];
        };
        union two_eightbyte_has_struct {
            int arr[3];
            struct dbl_struct member1;
        };
        struct char_first_eightbyte {
            char c;
            double d;
        };
        struct int_second_eightbyte {
            double d;
            int i;
        };
        union two_structs {
            struct char_first_eightbyte member1;
            struct int_second_eightbyte member2;
        };
        struct nine_bytes {
            int i;
            char arr[5];
        };
        union has_nine_byte_struct {
            char c;
            long l;
            struct nine_bytes s;
        };
        union uneven {
            char arr[5];
            unsigned char uc;
        };
        struct has_uneven_union {
            int i;
            union uneven u;
        };
        union has_other_unions {
            union uneven u;
            union two_doubles d;
            union has_nine_byte_struct n;
        };
        union union_array {
            union one_int u_arr[2];
        };
        union uneven_union_array {
            union uneven u_arr[2];
        };
        struct small {
            char arr[3];
            signed char sc;
        };
        union has_small_struct_array {
            struct small arr[3];
        };
        union gp_and_xmm {
            double d_arr[2];
            char c;
        };
        union scalar_and_struct {
            long* ptr;
            struct char_first_eightbyte cfe;
        };
        struct has_two_unions {
            union char_int_mixed member1;
            union one_double member2;
        };
        union small_struct_arr_and_dbl {
            struct small arr[2];
            union two_doubles d;
        };
        union xmm_and_gp {
            double d;
            struct int_second_eightbyte ise;
        };
        union xmm_and_gp_nested {
            union xmm_and_gp member1;
            double arr[2];
            union two_doubles d;
        };
        union lotsa_doubles {
            double arr[3];
            int i;
        };
        union lotsa_chars {
            char more_chars[18];
            char fewer_chars[5];
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        union contains_large_struct {
            int i;
            unsigned long ul;
            struct large l;
        };
        union contains_union_array {
            union gp_and_xmm arr[2];
        };
        int test_one_double(union one_double u);
        int test_has_union_with_double(struct has_union_with_double s);
        int test_has_struct_with_double(union has_struct_with_double u);
        int test_one_int(union one_int u);
        int test_one_int_nested(union one_int_nested u);
        int test_char_int_mixed(union char_int_mixed u);
        int test_has_union(struct has_union s);
        int test_has_struct_with_ints(union has_struct_with_ints u);
        int test_two_doubles(union two_doubles u);
        int test_has_xmm_union(union has_xmm_union u);
        int test_dbl_struct(struct dbl_struct s);
        int test_has_dbl_struct(union has_dbl_struct u);
        int test_char_arr(union char_arr u);
        int test_two_arrs(union two_arrs u);
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u);
        int test_two_structs(union two_structs u);
        int test_has_nine_byte_struct(union has_nine_byte_struct u);
        int test_has_uneven_union(struct has_uneven_union s);
        int test_has_other_unions(union has_other_unions u);
        int test_union_array(union union_array u);
        int test_uneven_union_array(union uneven_union_array u);
        int test_has_small_struct_array(union has_small_struct_array u);
        int test_gp_and_xmm(union gp_and_xmm u);
        int test_scalar_and_struct(union scalar_and_struct u);
        int test_has_two_unions(struct has_two_unions s);
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u);
        int test_xmm_and_gp(union xmm_and_gp u);
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u);
        int test_lotsa_doubles(union lotsa_doubles u);
        int test_lotsa_chars(union lotsa_chars u);
        int test_contains_large_struct(union contains_large_struct u);
        int test_contains_union_array(union contains_union_array u);
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5);
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp);
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2);
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp);
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2);
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs);
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven);
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct);
        union one_double return_one_double(void);
        union one_int_nested return_one_int_nested(void);
        union has_dbl_struct return_has_dbl_struct(void);
        union two_arrs return_two_arrs(void);
        union scalar_and_struct return_scalar_and_struct(void);
        union xmm_and_gp return_xmm_and_gp(void);
        union contains_union_array return_contains_union_array(void);
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin);
        struct has_uneven_union return_struct_with_union(void);
        
        int main(void) {
            union one_double od = { -2.345e6 };
            if (!test_one_double(od)) {
                return 1;
            }
            struct has_union_with_double huwd = { {9887.54321e44} };
            if (!test_has_union_with_double(huwd)) {
                return 2;
            }
            union has_struct_with_double hswd = { huwd };
            if (!test_has_struct_with_double(hswd)) {
                return 3;
            }
            union one_int oi = { -80. };
            if (!test_one_int(oi)) {
                return 4;
            }
            union one_int_nested oin = { {44e55} };
            if (!test_one_int_nested(oin)) {
                return 5;
            }
            union char_int_mixed cim = { "WXYZ" };
            if (!test_char_int_mixed(cim)) {
                return 6;
            }
            struct has_union hu = { 4294954951u, {-60} };
            if (!test_has_union(hu)) {
                return 7;
            }
            union has_struct_with_ints hswi;
            hswi.s = hu;
            if (!test_has_struct_with_ints(hswi)) {
                return 8;
            }
            union two_doubles td = { {10.0, 11.0} };
            if (!test_two_doubles(td)) {
                return 9;
            }
            union has_xmm_union hxu;
            hxu.u2 = td;
            if (!test_has_xmm_union(hxu)) {
                return 10;
            }
            struct dbl_struct ds = { od, 123.45 };
            if (!test_dbl_struct(ds)) {
                return 11;
            }
            union has_dbl_struct hds = { ds };
            if (!test_has_dbl_struct(hds)) {
                return 12;
            }
            union char_arr ca = { "Chars!" };
            if (!test_char_arr(ca)) {
                return 13;
            }
            union two_arrs two_arr_var = { {13e4, 14.5} };
            if (!test_two_arrs(two_arr_var)) {
                return 14;
            }
            union two_eightbyte_has_struct tehs = { {100, 200, 300} };
            if (!test_two_eightbyte_has_struct(tehs)) {
                return 15;
            }
            union two_structs ts = { {'x', 55.5e5} };
            if (!test_two_structs(ts)) {
                return 16;
            }
            union has_nine_byte_struct hnbs;
            hnbs.s.i = -16711936;
            for (int i = 0; i < 5; i = i + 1) {
                char byte = i % 2 ? -1 : 0;
                hnbs.s.arr[i] = byte;
            }
            hnbs.s.arr[4] = 0;
            if (!test_has_nine_byte_struct(hnbs)) {
                return 17;
            }
            struct has_uneven_union huu = { -2147483647, {"!@#$"} };
            if (!test_has_uneven_union(huu)) {
                return 18;
            }
            union has_other_unions hou;
            hou.n = hnbs;
            hou.n.s.arr[4] = 0;
            if (!test_has_other_unions(hou)) {
                return 19;
            }
            union union_array ua = { {{-20.}, {-30.}} };
            if (!test_union_array(ua)) {
                return 20;
            }
            union uneven_union_array uua = { {{"QWER"},{"TYUI"}} };
            if (!test_uneven_union_array(uua)) {
                return 21;
            }
            union has_small_struct_array hssa = { {
                {"AS", 10}, {"DF", 11}, {"GH", 12}
            } };
            if (!test_has_small_struct_array(hssa)) {
                return 22;
            }
            union gp_and_xmm gax = { {11., 12} };
            if (!test_gp_and_xmm(gax)) {
                return 23;
            }
            union scalar_and_struct sas;
            sas.cfe.c = -5;
            sas.cfe.d = -88.8;
            if (!test_scalar_and_struct(sas)) {
                return 24;
            }
            struct has_two_unions htu = {
                cim, od
            };
            if (!test_has_two_unions(htu)) {
                return 25;
            }
            union small_struct_arr_and_dbl ssaad;
            ssaad.d.arr[0] = -22.;
            ssaad.d.arr[1] = -32.;
            if (!test_small_struct_arr_and_dbl(ssaad)) {
                return 26;
            }
            union xmm_and_gp xag;
            xag.ise.d = -8.;
            xag.ise.i = -8;
            if (!test_xmm_and_gp(xag)) {
                return 27;
            }
            union xmm_and_gp_nested xagn = { xag };
            if (!test_xmm_and_gp_nested(xagn)) {
                return 28;
            }
            union lotsa_doubles dbls = { {99., 98., 97.} };
            if (!test_lotsa_doubles(dbls)) {
                return 29;
            }
            union lotsa_chars chars = { "asflakjsdflkjs" };
            if (!test_lotsa_chars(chars)) {
                return 30;
            }
            struct large large_struct = { 100, 100., "A struct!" };
            union contains_large_struct cls;
            cls.l = large_struct;
            if (!test_contains_large_struct(cls)) {
                return 31;
            }
            union gp_and_xmm gax2 = gax;
            gax2.d_arr[0] = -2.0;
            gax2.d_arr[1] = -1.0;
            union contains_union_array cua = {
                {gax, gax2}
            };
            if (!test_contains_union_array(cua)) {
                return 32;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 2345000D
            od.142[0] = tmp.0
            tmp.1 = test_one_double(od.142)
            tmp.2 = ! tmp.1
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            huwd.143[0] = 988754321000000000000000000000000000000000000000D
            tmp.3 = test_has_union_with_double(huwd.143)
            tmp.4 = ! tmp.3
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            hswd.144[0] = huwd.143
            tmp.5 = test_has_struct_with_double(hswd.144)
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = - 80D
            oi.145[0] = tmp.7
            tmp.8 = test_one_int(oi.145)
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_6
            return 4
        
          end_if_6:
            oin.146[0] = 440000000000000000000000000000000000000000000000000000000D
            tmp.10 = test_one_int_nested(oin.146)
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_8
            return 5
        
          end_if_8:
            cim.147[0] = 'W'
            cim.147[1] = 'X'
            cim.147[2] = 'Y'
            cim.147[3] = 'Z'
            cim.147[4] = '\0'
            cim.147[5] = '\0'
            cim.147[6] = '\0'
            tmp.12 = test_char_int_mixed(cim.147)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            hu.148[0] = 4294954951U
            tmp.14 = - 60
            tmp.15 = truncate tmp.14
            hu.148[4] = tmp.15
            tmp.16 = test_has_union(hu.148)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_12
            return 7
        
          end_if_12:
            hswi.149[0] = hu.148
            tmp.18 = hswi.149[0]
            tmp.19 = test_has_struct_with_ints(hswi.149)
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_14
            return 8
        
          end_if_14:
            td.150[0] = 10D
            td.150[8] = 11D
            tmp.21 = test_two_doubles(td.150)
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_16
            return 9
        
          end_if_16:
            hxu.151[0] = td.150
            tmp.23 = hxu.151[0]
            tmp.24 = test_has_xmm_union(hxu.151)
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_18
            return 10
        
          end_if_18:
            ds.152[0] = od.142
            ds.152[8] = 123.45D
            tmp.26 = test_dbl_struct(ds.152)
            tmp.27 = ! tmp.26
            if !tmp.27 jump end_if_20
            return 11
        
          end_if_20:
            hds.153[0] = ds.152
            tmp.28 = test_has_dbl_struct(hds.153)
            tmp.29 = ! tmp.28
            if !tmp.29 jump end_if_22
            return 12
        
          end_if_22:
            ca.154[0] = 'C'
            ca.154[1] = 'h'
            ca.154[2] = 'a'
            ca.154[3] = 'r'
            ca.154[4] = 's'
            ca.154[5] = '!'
            ca.154[6] = '\0'
            ca.154[7] = '\0'
            ca.154[8] = '\0'
            ca.154[9] = '\0'
            ca.154[10] = '\0'
            tmp.30 = test_char_arr(ca.154)
            tmp.31 = ! tmp.30
            if !tmp.31 jump end_if_24
            return 13
        
          end_if_24:
            two_arr_var.155[0] = 130000D
            two_arr_var.155[8] = 14.5D
            tmp.32 = test_two_arrs(two_arr_var.155)
            tmp.33 = ! tmp.32
            if !tmp.33 jump end_if_26
            return 14
        
          end_if_26:
            tehs.156[0] = 100
            tehs.156[4] = 200
            tehs.156[8] = 300
            tmp.34 = test_two_eightbyte_has_struct(tehs.156)
            tmp.35 = ! tmp.34
            if !tmp.35 jump end_if_28
            return 15
        
          end_if_28:
            tmp.36 = truncate 120
            ts.157[0] = tmp.36
            ts.157[8] = 5550000D
            tmp.37 = test_two_structs(ts.157)
            tmp.38 = ! tmp.37
            if !tmp.38 jump end_if_30
            return 16
        
          end_if_30:
            tmp.39 = - 16711936
            hnbs.158[0] = tmp.39
            tmp.40 = hnbs.158[0]
            i.159 = 0
        
          start_loop_0:
            tmp.41 = i.159 < 5
            if !tmp.41 jump break_loop_0
            tmp.42 = i.159 % 2
            if !tmp.42 jump else_33
            tmp.44 = - 1
            tmp.43 = tmp.44
            jump end_if_32
        
          else_33:
            tmp.43 = 0
        
          end_if_32:
            tmp.45 = truncate tmp.43
            byte.160 = tmp.45
            tmp.46 = &hnbs.158
            tmp.46 = add_ptr(tmp.46, index=4L, scale=1)
            tmp.47 = sign_extend i.159
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=1)
            *tmp.48 = byte.160
            tmp.49 = *tmp.48
        
          continue_loop_0:
            tmp.50 = i.159 + 1
            i.159 = tmp.50
            jump start_loop_0
        
          break_loop_0:
            tmp.51 = &hnbs.158
            tmp.51 = add_ptr(tmp.51, index=4L, scale=1)
            tmp.52 = sign_extend 4
            tmp.53 = add_ptr(tmp.51, index=tmp.52, scale=1)
            tmp.54 = truncate 0
            *tmp.53 = tmp.54
            tmp.55 = *tmp.53
            tmp.56 = test_has_nine_byte_struct(hnbs.158)
            tmp.57 = ! tmp.56
            if !tmp.57 jump end_if_34
            return 17
        
          end_if_34:
            tmp.58 = - 2147483647
            huu.161[0] = tmp.58
            huu.161[4] = '!'
            huu.161[5] = '@'
            huu.161[6] = '#'
            huu.161[7] = '$'
            huu.161[8] = '\0'
            tmp.59 = test_has_uneven_union(huu.161)
            tmp.60 = ! tmp.59
            if !tmp.60 jump end_if_36
            return 18
        
          end_if_36:
            hou.162[0] = hnbs.158
            tmp.61 = hou.162[0]
            tmp.62 = &hou.162
            tmp.62 = add_ptr(tmp.62, index=4L, scale=1)
            tmp.63 = sign_extend 4
            tmp.64 = add_ptr(tmp.62, index=tmp.63, scale=1)
            tmp.65 = truncate 0
            *tmp.64 = tmp.65
            tmp.66 = *tmp.64
            tmp.67 = test_has_other_unions(hou.162)
            tmp.68 = ! tmp.67
            if !tmp.68 jump end_if_38
            return 19
        
          end_if_38:
            tmp.69 = - 20D
            ua.163[0] = tmp.69
            tmp.70 = - 30D
            ua.163[8] = tmp.70
            tmp.71 = test_union_array(ua.163)
            tmp.72 = ! tmp.71
            if !tmp.72 jump end_if_40
            return 20
        
          end_if_40:
            uua.164[0] = 'Q'
            uua.164[1] = 'W'
            uua.164[2] = 'E'
            uua.164[3] = 'R'
            uua.164[4] = '\0'
            uua.164[5] = 'T'
            uua.164[6] = 'Y'
            uua.164[7] = 'U'
            uua.164[8] = 'I'
            uua.164[9] = '\0'
            tmp.73 = test_uneven_union_array(uua.164)
            tmp.74 = ! tmp.73
            if !tmp.74 jump end_if_42
            return 21
        
          end_if_42:
            hssa.165[0] = 'A'
            hssa.165[1] = 'S'
            hssa.165[2] = '\0'
            tmp.75 = truncate 10
            hssa.165[3] = tmp.75
            hssa.165[4] = 'D'
            hssa.165[5] = 'F'
            hssa.165[6] = '\0'
            tmp.76 = truncate 11
            hssa.165[7] = tmp.76
            hssa.165[8] = 'G'
            hssa.165[9] = 'H'
            hssa.165[10] = '\0'
            tmp.77 = truncate 12
            hssa.165[11] = tmp.77
            tmp.78 = test_has_small_struct_array(hssa.165)
            tmp.79 = ! tmp.78
            if !tmp.79 jump end_if_44
            return 22
        
          end_if_44:
            gax.166[0] = 11D
            tmp.80 = int_to_double 12
            gax.166[8] = tmp.80
            tmp.81 = test_gp_and_xmm(gax.166)
            tmp.82 = ! tmp.81
            if !tmp.82 jump end_if_46
            return 23
        
          end_if_46:
            tmp.83 = - 5
            tmp.84 = truncate tmp.83
            sas.167[0] = tmp.84
            tmp.85 = sas.167[0]
            tmp.86 = - 88.8D
            sas.167[8] = tmp.86
            tmp.87 = sas.167[8]
            tmp.88 = test_scalar_and_struct(sas.167)
            tmp.89 = ! tmp.88
            if !tmp.89 jump end_if_48
            return 24
        
          end_if_48:
            htu.168[0] = cim.147
            htu.168[8] = od.142
            tmp.90 = test_has_two_unions(htu.168)
            tmp.91 = ! tmp.90
            if !tmp.91 jump end_if_50
            return 25
        
          end_if_50:
            tmp.92 = &ssaad.169
            tmp.93 = sign_extend 0
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=8)
            tmp.95 = - 22D
            *tmp.94 = tmp.95
            tmp.96 = *tmp.94
            tmp.97 = &ssaad.169
            tmp.98 = sign_extend 1
            tmp.99 = add_ptr(tmp.97, index=tmp.98, scale=8)
            tmp.100 = - 32D
            *tmp.99 = tmp.100
            tmp.101 = *tmp.99
            tmp.102 = test_small_struct_arr_and_dbl(ssaad.169)
            tmp.103 = ! tmp.102
            if !tmp.103 jump end_if_52
            return 26
        
          end_if_52:
            tmp.104 = - 8D
            xag.170[0] = tmp.104
            tmp.105 = xag.170[0]
            tmp.106 = - 8
            xag.170[8] = tmp.106
            tmp.107 = xag.170[8]
            tmp.108 = test_xmm_and_gp(xag.170)
            tmp.109 = ! tmp.108
            if !tmp.109 jump end_if_54
            return 27
        
          end_if_54:
            xagn.171[0] = xag.170
            tmp.110 = test_xmm_and_gp_nested(xagn.171)
            tmp.111 = ! tmp.110
            if !tmp.111 jump end_if_56
            return 28
        
          end_if_56:
            dbls.172[0] = 99D
            dbls.172[8] = 98D
            dbls.172[16] = 97D
            tmp.112 = test_lotsa_doubles(dbls.172)
            tmp.113 = ! tmp.112
            if !tmp.113 jump end_if_58
            return 29
        
          end_if_58:
            chars.173[0] = 'a'
            chars.173[1] = 's'
            chars.173[2] = 'f'
            chars.173[3] = 'l'
            chars.173[4] = 'a'
            chars.173[5] = 'k'
            chars.173[6] = 'j'
            chars.173[7] = 's'
            chars.173[8] = 'd'
            chars.173[9] = 'f'
            chars.173[10] = 'l'
            chars.173[11] = 'k'
            chars.173[12] = 'j'
            chars.173[13] = 's'
            chars.173[14] = '\0'
            chars.173[15] = '\0'
            chars.173[16] = '\0'
            chars.173[17] = '\0'
            tmp.114 = test_lotsa_chars(chars.173)
            tmp.115 = ! tmp.114
            if !tmp.115 jump end_if_60
            return 30
        
          end_if_60:
            large_struct.174[0] = 100
            large_struct.174[8] = 100D
            large_struct.174[16] = 'A'
            large_struct.174[17] = ' '
            large_struct.174[18] = 's'
            large_struct.174[19] = 't'
            large_struct.174[20] = 'r'
            large_struct.174[21] = 'u'
            large_struct.174[22] = 'c'
            large_struct.174[23] = 't'
            large_struct.174[24] = '!'
            large_struct.174[25] = '\0'
            cls.175[0] = large_struct.174
            tmp.116 = cls.175[0]
            tmp.117 = test_contains_large_struct(cls.175)
            tmp.118 = ! tmp.117
            if !tmp.118 jump end_if_62
            return 31
        
          end_if_62:
            gax2.176 = gax.166
            tmp.119 = &gax2.176
            tmp.120 = sign_extend 0
            tmp.121 = add_ptr(tmp.119, index=tmp.120, scale=8)
            tmp.122 = - 2D
            *tmp.121 = tmp.122
            tmp.123 = *tmp.121
            tmp.124 = &gax2.176
            tmp.125 = sign_extend 1
            tmp.126 = add_ptr(tmp.124, index=tmp.125, scale=8)
            tmp.127 = - 1D
            *tmp.126 = tmp.127
            tmp.128 = *tmp.126
            cua.177[0] = gax.166
            cua.177[16] = gax2.176
            tmp.129 = test_contains_union_array(cua.177)
            tmp.130 = ! tmp.129
            if !tmp.130 jump end_if_64
            return 32
        
          end_if_64:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_param_passing() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        void exit(int status);
        void *malloc(unsigned long size);
        union one_double {
            double d1;
            double d2;
        };
        struct has_union_with_double {
            union one_double member;
        };
        union has_struct_with_double {
            struct has_union_with_double s;
            double arr[1];
        };
        union one_int {
            double d;
            char c;
        };
        union one_int_nested {
            union one_int oi;
            union one_double od;
        };
        union char_int_mixed {
            char arr[7];
            union char_int_mixed* union_ptr;
            unsigned int ui;
        };
        union char_int_short {
            char c;
            int i;
        };
        struct has_union {
            unsigned int i;
            union char_int_short u;
        };
        union has_struct_with_ints {
            double d;
            struct has_union s;
            unsigned long ul;
        };
        union two_doubles {
            double arr[2];
            double single;
        };
        union has_xmm_union {
            union one_double u;
            union two_doubles u2;
        };
        struct dbl_struct {
            union one_double member1;
            double member2;
        };
        union has_dbl_struct {
            struct dbl_struct member1;
        };
        union char_arr {
            char arr[11];
            int i;
        };
        union two_arrs {
            double dbl_arr[2];
            long long_arr[2];
        };
        union two_eightbyte_has_struct {
            int arr[3];
            struct dbl_struct member1;
        };
        struct char_first_eightbyte {
            char c;
            double d;
        };
        struct int_second_eightbyte {
            double d;
            int i;
        };
        union two_structs {
            struct char_first_eightbyte member1;
            struct int_second_eightbyte member2;
        };
        struct nine_bytes {
            int i;
            char arr[5];
        };
        union has_nine_byte_struct {
            char c;
            long l;
            struct nine_bytes s;
        };
        union uneven {
            char arr[5];
            unsigned char uc;
        };
        struct has_uneven_union {
            int i;
            union uneven u;
        };
        union has_other_unions {
            union uneven u;
            union two_doubles d;
            union has_nine_byte_struct n;
        };
        union union_array {
            union one_int u_arr[2];
        };
        union uneven_union_array {
            union uneven u_arr[2];
        };
        struct small {
            char arr[3];
            signed char sc;
        };
        union has_small_struct_array {
            struct small arr[3];
        };
        union gp_and_xmm {
            double d_arr[2];
            char c;
        };
        union scalar_and_struct {
            long* ptr;
            struct char_first_eightbyte cfe;
        };
        struct has_two_unions {
            union char_int_mixed member1;
            union one_double member2;
        };
        union small_struct_arr_and_dbl {
            struct small arr[2];
            union two_doubles d;
        };
        union xmm_and_gp {
            double d;
            struct int_second_eightbyte ise;
        };
        union xmm_and_gp_nested {
            union xmm_and_gp member1;
            double arr[2];
            union two_doubles d;
        };
        union lotsa_doubles {
            double arr[3];
            int i;
        };
        union lotsa_chars {
            char more_chars[18];
            char fewer_chars[5];
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        union contains_large_struct {
            int i;
            unsigned long ul;
            struct large l;
        };
        union contains_union_array {
            union gp_and_xmm arr[2];
        };
        int test_one_double(union one_double u);
        int test_has_union_with_double(struct has_union_with_double s);
        int test_has_struct_with_double(union has_struct_with_double u);
        int test_one_int(union one_int u);
        int test_one_int_nested(union one_int_nested u);
        int test_char_int_mixed(union char_int_mixed u);
        int test_has_union(struct has_union s);
        int test_has_struct_with_ints(union has_struct_with_ints u);
        int test_two_doubles(union two_doubles u);
        int test_has_xmm_union(union has_xmm_union u);
        int test_dbl_struct(struct dbl_struct s);
        int test_has_dbl_struct(union has_dbl_struct u);
        int test_char_arr(union char_arr u);
        int test_two_arrs(union two_arrs u);
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u);
        int test_two_structs(union two_structs u);
        int test_has_nine_byte_struct(union has_nine_byte_struct u);
        int test_has_uneven_union(struct has_uneven_union s);
        int test_has_other_unions(union has_other_unions u);
        int test_union_array(union union_array u);
        int test_uneven_union_array(union uneven_union_array u);
        int test_has_small_struct_array(union has_small_struct_array u);
        int test_gp_and_xmm(union gp_and_xmm u);
        int test_scalar_and_struct(union scalar_and_struct u);
        int test_has_two_unions(struct has_two_unions s);
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u);
        int test_xmm_and_gp(union xmm_and_gp u);
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u);
        int test_lotsa_doubles(union lotsa_doubles u);
        int test_lotsa_chars(union lotsa_chars u);
        int test_contains_large_struct(union contains_large_struct u);
        int test_contains_union_array(union contains_union_array u);
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5);
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp);
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2);
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp);
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2);
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs);
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven);
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct);
        union one_double return_one_double(void);
        union one_int_nested return_one_int_nested(void);
        union has_dbl_struct return_has_dbl_struct(void);
        union two_arrs return_two_arrs(void);
        union scalar_and_struct return_scalar_and_struct(void);
        union xmm_and_gp return_xmm_and_gp(void);
        union contains_union_array return_contains_union_array(void);
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin);
        struct has_uneven_union return_struct_with_union(void);
        
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5) {
            if (!(i1 == 1 && i2 == 2 && d1 == 4.0 && i3 == 100 && i4 == 120 && i5 == 130)) {
                return 0;
            }
            if (!(one_gp_struct.i == (unsigned int)-24 && one_gp_struct.u.i == 123456789)) {
                return 0;
            }
            if (!(two_xmm.arr[0] == -10. && two_xmm.arr[1] == -11.)) {
                return 0;
            }
            if (!(one_gp.d == 13.)) {
                return 0;
            }
            return 1;
        }
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp) {
            if (!(i1 == -1 && i2 == -2 && i3 == -3 && i4 == -4 && i5 == -5 && i6 == -6)) {
                return 0;
            }
            if (!(two_xmm.arr[0] == -10. && two_xmm.arr[1] == -11.)) {
                return 0;
            }
            if (!(one_gp_struct.i == (unsigned int)-24 && one_gp_struct.u.i == 123456789)) {
                return 0;
            }
            if (!(one_gp.d == 13.)) {
                return 0;
            }
            return 1;
        }
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2) {
            if (!(d1 == 1.0 && d2 == 2.0 && d3 == 3.0 && d4 == 4.0)) {
                return 0;
            }
            if (!(two_xmm.arr[0] == -10. && two_xmm.arr[1] == -11.)) {
                return 0;
            }
            if (!(two_xmm_copy.arr[0] == -10. && two_xmm_copy.arr[1] == -11.)) {
                return 0;
            }
            if (!(two_xmm_2.arr[0] == 33e4 && two_xmm_2.arr[1] == 55e6)) {
                return 0;
            }
            return 1;
        }
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp) {
            if (!(i1 == 1 && i2 == 2 && i3 == 3 && i4 == 4 && i5 == 5)) {
                return 0;
            }
            if (strcmp(two_gp.arr, "+_)(*&^%$#") != 0) {
                return 0;
            }
            return 1;
        }
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2) {
            if (!(d1 == 9.0 && d2 == 8.0 && d3 == 7.0 && d4 == 6.0 && d5 == 5.0)) {
                return 0;
            }
            if (!(two_xmm.arr[0] == -10. && two_xmm.arr[1] == -11.)) {
                return 0;
            }
            if (!(two_xmm_2.arr[0] == 66e4 && two_xmm_2.arr[1] == 110e6)) {
                return 0;
            }
            return 1;
        }
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs) {
            if (!(d1 == 101.2 && d2 == 102.3 && d3 == 103.4 && d4 == 104.5 && i1 == 75 && i2 == 76 && i3 == 77 && i4 == 78 && i5 == 79 && i6 == 80)) {
                return 0;
            }
            if (!(mixed_regs.d_arr[0] == 0 && mixed_regs.d_arr[1] == 150.5)) {
                return 0;
            }
            return 1;
        }
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven) {
            if (!(i1 == 1100 && i2 == 2200 && i3 == 3300 && i4 == 4400 && i5 == 5500)) {
                return 0;
            }
            if (!(mixed_regs.d_arr[0] == 0 && mixed_regs.d_arr[1] == 150.5)) {
                return 0;
            }
            if (!(one_gp.d == 13.)) {
                return 0;
            }
            if (strcmp(uneven.arr, "boop") != 0) {
                return 0;
            }
            return 1;
        }
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct) {
            if (!(mem.arr[0] == 66. && mem.arr[1] == 77. && mem.arr[2] == 88.)) {
                return 0;
            }
            if (!(mixed_regs.d_arr[0] == 0 && mixed_regs.d_arr[1] == 150.5)) {
                return 0;
            }
            if (strcmp(two_gp.arr, "+_)(*&^%$#") != 0) {
                return 0;
            }
            if (!(one_gp_struct.i == (unsigned int)-24 && one_gp_struct.u.i == 123456789)) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function pass_unions_and_structs(i1.142, i2.143, one_gp_struct.144, d1.145, two_xmm.146, one_gp.147, i3.148, i4.149, i5.150) { 
            tmp.0 = i1.142 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = i2.143 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = d1.145 == 4D
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = i3.148 == 100
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = i4.149 == 120
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = i5.150 == 130
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            tmp.16 = ! tmp.14
            if !tmp.16 jump end_if_10
            return 0
        
          end_if_10:
            tmp.17 = one_gp_struct.144[0]
            tmp.19 = - 24
            tmp.20 = tmp.19
            tmp.18 = tmp.17 == tmp.20
            if !tmp.18 jump and_false_12
            tmp.23 = one_gp_struct.144[4]
            tmp.24 = tmp.23 == 123456789
            if !tmp.24 jump and_false_12
            tmp.22 = 1
            jump and_end_13
        
          and_false_12:
            tmp.22 = 0
        
          and_end_13:
            tmp.25 = ! tmp.22
            if !tmp.25 jump end_if_14
            return 0
        
          end_if_14:
            tmp.26 = &two_xmm.146
            tmp.27 = sign_extend 0
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=8)
            tmp.29 = *tmp.28
            tmp.31 = - 10D
            tmp.30 = tmp.29 == tmp.31
            if !tmp.30 jump and_false_16
            tmp.34 = &two_xmm.146
            tmp.35 = sign_extend 1
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=8)
            tmp.37 = *tmp.36
            tmp.39 = - 11D
            tmp.38 = tmp.37 == tmp.39
            if !tmp.38 jump and_false_16
            tmp.33 = 1
            jump and_end_17
        
          and_false_16:
            tmp.33 = 0
        
          and_end_17:
            tmp.40 = ! tmp.33
            if !tmp.40 jump end_if_18
            return 0
        
          end_if_18:
            tmp.41 = one_gp.147[0]
            tmp.42 = tmp.41 == 13D
            tmp.43 = ! tmp.42
            if !tmp.43 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function pass_gp_union_in_memory(two_xmm.151, one_gp_struct.152, i1.153, i2.154, i3.155, i4.156, i5.157, i6.158, one_gp.159) { 
            tmp.45 = - 1
            tmp.44 = i1.153 == tmp.45
            if !tmp.44 jump and_false_22
            tmp.49 = - 2
            tmp.48 = i2.154 == tmp.49
            if !tmp.48 jump and_false_22
            tmp.47 = 1
            jump and_end_23
        
          and_false_22:
            tmp.47 = 0
        
          and_end_23:
            if !tmp.47 jump and_false_24
            tmp.53 = - 3
            tmp.52 = i3.155 == tmp.53
            if !tmp.52 jump and_false_24
            tmp.51 = 1
            jump and_end_25
        
          and_false_24:
            tmp.51 = 0
        
          and_end_25:
            if !tmp.51 jump and_false_26
            tmp.57 = - 4
            tmp.56 = i4.156 == tmp.57
            if !tmp.56 jump and_false_26
            tmp.55 = 1
            jump and_end_27
        
          and_false_26:
            tmp.55 = 0
        
          and_end_27:
            if !tmp.55 jump and_false_28
            tmp.61 = - 5
            tmp.60 = i5.157 == tmp.61
            if !tmp.60 jump and_false_28
            tmp.59 = 1
            jump and_end_29
        
          and_false_28:
            tmp.59 = 0
        
          and_end_29:
            if !tmp.59 jump and_false_30
            tmp.65 = - 6
            tmp.64 = i6.158 == tmp.65
            if !tmp.64 jump and_false_30
            tmp.63 = 1
            jump and_end_31
        
          and_false_30:
            tmp.63 = 0
        
          and_end_31:
            tmp.66 = ! tmp.63
            if !tmp.66 jump end_if_32
            return 0
        
          end_if_32:
            tmp.67 = &two_xmm.151
            tmp.68 = sign_extend 0
            tmp.69 = add_ptr(tmp.67, index=tmp.68, scale=8)
            tmp.70 = *tmp.69
            tmp.72 = - 10D
            tmp.71 = tmp.70 == tmp.72
            if !tmp.71 jump and_false_34
            tmp.75 = &two_xmm.151
            tmp.76 = sign_extend 1
            tmp.77 = add_ptr(tmp.75, index=tmp.76, scale=8)
            tmp.78 = *tmp.77
            tmp.80 = - 11D
            tmp.79 = tmp.78 == tmp.80
            if !tmp.79 jump and_false_34
            tmp.74 = 1
            jump and_end_35
        
          and_false_34:
            tmp.74 = 0
        
          and_end_35:
            tmp.81 = ! tmp.74
            if !tmp.81 jump end_if_36
            return 0
        
          end_if_36:
            tmp.82 = one_gp_struct.152[0]
            tmp.84 = - 24
            tmp.85 = tmp.84
            tmp.83 = tmp.82 == tmp.85
            if !tmp.83 jump and_false_38
            tmp.88 = one_gp_struct.152[4]
            tmp.89 = tmp.88 == 123456789
            if !tmp.89 jump and_false_38
            tmp.87 = 1
            jump and_end_39
        
          and_false_38:
            tmp.87 = 0
        
          and_end_39:
            tmp.90 = ! tmp.87
            if !tmp.90 jump end_if_40
            return 0
        
          end_if_40:
            tmp.91 = one_gp.159[0]
            tmp.92 = tmp.91 == 13D
            tmp.93 = ! tmp.92
            if !tmp.93 jump end_if_42
            return 0
        
          end_if_42:
            return 1
            return 0
        }
        global function pass_xmm_union_in_memory(d1.160, d2.161, two_xmm.162, two_xmm_copy.163, d3.164, d4.165, two_xmm_2.166) { 
            tmp.94 = d1.160 == 1D
            if !tmp.94 jump and_false_44
            tmp.97 = d2.161 == 2D
            if !tmp.97 jump and_false_44
            tmp.96 = 1
            jump and_end_45
        
          and_false_44:
            tmp.96 = 0
        
          and_end_45:
            if !tmp.96 jump and_false_46
            tmp.100 = d3.164 == 3D
            if !tmp.100 jump and_false_46
            tmp.99 = 1
            jump and_end_47
        
          and_false_46:
            tmp.99 = 0
        
          and_end_47:
            if !tmp.99 jump and_false_48
            tmp.103 = d4.165 == 4D
            if !tmp.103 jump and_false_48
            tmp.102 = 1
            jump and_end_49
        
          and_false_48:
            tmp.102 = 0
        
          and_end_49:
            tmp.104 = ! tmp.102
            if !tmp.104 jump end_if_50
            return 0
        
          end_if_50:
            tmp.105 = &two_xmm.162
            tmp.106 = sign_extend 0
            tmp.107 = add_ptr(tmp.105, index=tmp.106, scale=8)
            tmp.108 = *tmp.107
            tmp.110 = - 10D
            tmp.109 = tmp.108 == tmp.110
            if !tmp.109 jump and_false_52
            tmp.113 = &two_xmm.162
            tmp.114 = sign_extend 1
            tmp.115 = add_ptr(tmp.113, index=tmp.114, scale=8)
            tmp.116 = *tmp.115
            tmp.118 = - 11D
            tmp.117 = tmp.116 == tmp.118
            if !tmp.117 jump and_false_52
            tmp.112 = 1
            jump and_end_53
        
          and_false_52:
            tmp.112 = 0
        
          and_end_53:
            tmp.119 = ! tmp.112
            if !tmp.119 jump end_if_54
            return 0
        
          end_if_54:
            tmp.120 = &two_xmm_copy.163
            tmp.121 = sign_extend 0
            tmp.122 = add_ptr(tmp.120, index=tmp.121, scale=8)
            tmp.123 = *tmp.122
            tmp.125 = - 10D
            tmp.124 = tmp.123 == tmp.125
            if !tmp.124 jump and_false_56
            tmp.128 = &two_xmm_copy.163
            tmp.129 = sign_extend 1
            tmp.130 = add_ptr(tmp.128, index=tmp.129, scale=8)
            tmp.131 = *tmp.130
            tmp.133 = - 11D
            tmp.132 = tmp.131 == tmp.133
            if !tmp.132 jump and_false_56
            tmp.127 = 1
            jump and_end_57
        
          and_false_56:
            tmp.127 = 0
        
          and_end_57:
            tmp.134 = ! tmp.127
            if !tmp.134 jump end_if_58
            return 0
        
          end_if_58:
            tmp.135 = &two_xmm_2.166
            tmp.136 = sign_extend 0
            tmp.137 = add_ptr(tmp.135, index=tmp.136, scale=8)
            tmp.138 = *tmp.137
            tmp.139 = tmp.138 == 330000D
            if !tmp.139 jump and_false_60
            tmp.142 = &two_xmm_2.166
            tmp.143 = sign_extend 1
            tmp.144 = add_ptr(tmp.142, index=tmp.143, scale=8)
            tmp.145 = *tmp.144
            tmp.146 = tmp.145 == 55000000D
            if !tmp.146 jump and_false_60
            tmp.141 = 1
            jump and_end_61
        
          and_false_60:
            tmp.141 = 0
        
          and_end_61:
            tmp.147 = ! tmp.141
            if !tmp.147 jump end_if_62
            return 0
        
          end_if_62:
            return 1
            return 0
        }
        global function pass_borderline_union(i1.167, i2.168, i3.169, i4.170, i5.171, two_gp.172) { 
            tmp.148 = i1.167 == 1
            if !tmp.148 jump and_false_64
            tmp.151 = i2.168 == 2
            if !tmp.151 jump and_false_64
            tmp.150 = 1
            jump and_end_65
        
          and_false_64:
            tmp.150 = 0
        
          and_end_65:
            if !tmp.150 jump and_false_66
            tmp.154 = i3.169 == 3
            if !tmp.154 jump and_false_66
            tmp.153 = 1
            jump and_end_67
        
          and_false_66:
            tmp.153 = 0
        
          and_end_67:
            if !tmp.153 jump and_false_68
            tmp.157 = i4.170 == 4
            if !tmp.157 jump and_false_68
            tmp.156 = 1
            jump and_end_69
        
          and_false_68:
            tmp.156 = 0
        
          and_end_69:
            if !tmp.156 jump and_false_70
            tmp.160 = i5.171 == 5
            if !tmp.160 jump and_false_70
            tmp.159 = 1
            jump and_end_71
        
          and_false_70:
            tmp.159 = 0
        
          and_end_71:
            tmp.161 = ! tmp.159
            if !tmp.161 jump end_if_72
            return 0
        
          end_if_72:
            tmp.162 = &two_gp.172
            tmp.163 = &string.0
            tmp.164 = strcmp(tmp.162, tmp.163)
            tmp.165 = tmp.164 != 0
            if !tmp.165 jump end_if_74
            return 0
        
          end_if_74:
            return 1
            return 0
        }
        global function pass_borderline_xmm_union(two_xmm.173, d1.174, d2.175, d3.176, d4.177, d5.178, two_xmm_2.179) { 
            tmp.166 = d1.174 == 9D
            if !tmp.166 jump and_false_76
            tmp.169 = d2.175 == 8D
            if !tmp.169 jump and_false_76
            tmp.168 = 1
            jump and_end_77
        
          and_false_76:
            tmp.168 = 0
        
          and_end_77:
            if !tmp.168 jump and_false_78
            tmp.172 = d3.176 == 7D
            if !tmp.172 jump and_false_78
            tmp.171 = 1
            jump and_end_79
        
          and_false_78:
            tmp.171 = 0
        
          and_end_79:
            if !tmp.171 jump and_false_80
            tmp.175 = d4.177 == 6D
            if !tmp.175 jump and_false_80
            tmp.174 = 1
            jump and_end_81
        
          and_false_80:
            tmp.174 = 0
        
          and_end_81:
            if !tmp.174 jump and_false_82
            tmp.178 = d5.178 == 5D
            if !tmp.178 jump and_false_82
            tmp.177 = 1
            jump and_end_83
        
          and_false_82:
            tmp.177 = 0
        
          and_end_83:
            tmp.179 = ! tmp.177
            if !tmp.179 jump end_if_84
            return 0
        
          end_if_84:
            tmp.180 = &two_xmm.173
            tmp.181 = sign_extend 0
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=8)
            tmp.183 = *tmp.182
            tmp.185 = - 10D
            tmp.184 = tmp.183 == tmp.185
            if !tmp.184 jump and_false_86
            tmp.188 = &two_xmm.173
            tmp.189 = sign_extend 1
            tmp.190 = add_ptr(tmp.188, index=tmp.189, scale=8)
            tmp.191 = *tmp.190
            tmp.193 = - 11D
            tmp.192 = tmp.191 == tmp.193
            if !tmp.192 jump and_false_86
            tmp.187 = 1
            jump and_end_87
        
          and_false_86:
            tmp.187 = 0
        
          and_end_87:
            tmp.194 = ! tmp.187
            if !tmp.194 jump end_if_88
            return 0
        
          end_if_88:
            tmp.195 = &two_xmm_2.179
            tmp.196 = sign_extend 0
            tmp.197 = add_ptr(tmp.195, index=tmp.196, scale=8)
            tmp.198 = *tmp.197
            tmp.199 = tmp.198 == 660000D
            if !tmp.199 jump and_false_90
            tmp.202 = &two_xmm_2.179
            tmp.203 = sign_extend 1
            tmp.204 = add_ptr(tmp.202, index=tmp.203, scale=8)
            tmp.205 = *tmp.204
            tmp.206 = tmp.205 == 110000000D
            if !tmp.206 jump and_false_90
            tmp.201 = 1
            jump and_end_91
        
          and_false_90:
            tmp.201 = 0
        
          and_end_91:
            tmp.207 = ! tmp.201
            if !tmp.207 jump end_if_92
            return 0
        
          end_if_92:
            return 1
            return 0
        }
        global function pass_mixed_reg_in_memory(d1.180, d2.181, d3.182, d4.183, i1.184, i2.185, i3.186, i4.187, i5.188, i6.189, mixed_regs.190) { 
            tmp.208 = d1.180 == 101.2D
            if !tmp.208 jump and_false_94
            tmp.211 = d2.181 == 102.3D
            if !tmp.211 jump and_false_94
            tmp.210 = 1
            jump and_end_95
        
          and_false_94:
            tmp.210 = 0
        
          and_end_95:
            if !tmp.210 jump and_false_96
            tmp.214 = d3.182 == 103.4D
            if !tmp.214 jump and_false_96
            tmp.213 = 1
            jump and_end_97
        
          and_false_96:
            tmp.213 = 0
        
          and_end_97:
            if !tmp.213 jump and_false_98
            tmp.217 = d4.183 == 104.5D
            if !tmp.217 jump and_false_98
            tmp.216 = 1
            jump and_end_99
        
          and_false_98:
            tmp.216 = 0
        
          and_end_99:
            if !tmp.216 jump and_false_100
            tmp.220 = i1.184 == 75
            if !tmp.220 jump and_false_100
            tmp.219 = 1
            jump and_end_101
        
          and_false_100:
            tmp.219 = 0
        
          and_end_101:
            if !tmp.219 jump and_false_102
            tmp.223 = i2.185 == 76
            if !tmp.223 jump and_false_102
            tmp.222 = 1
            jump and_end_103
        
          and_false_102:
            tmp.222 = 0
        
          and_end_103:
            if !tmp.222 jump and_false_104
            tmp.226 = i3.186 == 77
            if !tmp.226 jump and_false_104
            tmp.225 = 1
            jump and_end_105
        
          and_false_104:
            tmp.225 = 0
        
          and_end_105:
            if !tmp.225 jump and_false_106
            tmp.229 = i4.187 == 78
            if !tmp.229 jump and_false_106
            tmp.228 = 1
            jump and_end_107
        
          and_false_106:
            tmp.228 = 0
        
          and_end_107:
            if !tmp.228 jump and_false_108
            tmp.232 = i5.188 == 79
            if !tmp.232 jump and_false_108
            tmp.231 = 1
            jump and_end_109
        
          and_false_108:
            tmp.231 = 0
        
          and_end_109:
            if !tmp.231 jump and_false_110
            tmp.235 = i6.189 == 80
            if !tmp.235 jump and_false_110
            tmp.234 = 1
            jump and_end_111
        
          and_false_110:
            tmp.234 = 0
        
          and_end_111:
            tmp.236 = ! tmp.234
            if !tmp.236 jump end_if_112
            return 0
        
          end_if_112:
            tmp.237 = &mixed_regs.190
            tmp.238 = sign_extend 0
            tmp.239 = add_ptr(tmp.237, index=tmp.238, scale=8)
            tmp.240 = *tmp.239
            tmp.242 = int_to_double 0
            tmp.241 = tmp.240 == tmp.242
            if !tmp.241 jump and_false_114
            tmp.245 = &mixed_regs.190
            tmp.246 = sign_extend 1
            tmp.247 = add_ptr(tmp.245, index=tmp.246, scale=8)
            tmp.248 = *tmp.247
            tmp.249 = tmp.248 == 150.5D
            if !tmp.249 jump and_false_114
            tmp.244 = 1
            jump and_end_115
        
          and_false_114:
            tmp.244 = 0
        
          and_end_115:
            tmp.250 = ! tmp.244
            if !tmp.250 jump end_if_116
            return 0
        
          end_if_116:
            return 1
            return 0
        }
        global function pass_uneven_union_in_memory(i1.191, i2.192, i3.193, i4.194, i5.195, mixed_regs.196, one_gp.197, uneven.198) { 
            tmp.251 = i1.191 == 1100
            if !tmp.251 jump and_false_118
            tmp.254 = i2.192 == 2200
            if !tmp.254 jump and_false_118
            tmp.253 = 1
            jump and_end_119
        
          and_false_118:
            tmp.253 = 0
        
          and_end_119:
            if !tmp.253 jump and_false_120
            tmp.257 = i3.193 == 3300
            if !tmp.257 jump and_false_120
            tmp.256 = 1
            jump and_end_121
        
          and_false_120:
            tmp.256 = 0
        
          and_end_121:
            if !tmp.256 jump and_false_122
            tmp.260 = i4.194 == 4400
            if !tmp.260 jump and_false_122
            tmp.259 = 1
            jump and_end_123
        
          and_false_122:
            tmp.259 = 0
        
          and_end_123:
            if !tmp.259 jump and_false_124
            tmp.263 = i5.195 == 5500
            if !tmp.263 jump and_false_124
            tmp.262 = 1
            jump and_end_125
        
          and_false_124:
            tmp.262 = 0
        
          and_end_125:
            tmp.264 = ! tmp.262
            if !tmp.264 jump end_if_126
            return 0
        
          end_if_126:
            tmp.265 = &mixed_regs.196
            tmp.266 = sign_extend 0
            tmp.267 = add_ptr(tmp.265, index=tmp.266, scale=8)
            tmp.268 = *tmp.267
            tmp.270 = int_to_double 0
            tmp.269 = tmp.268 == tmp.270
            if !tmp.269 jump and_false_128
            tmp.273 = &mixed_regs.196
            tmp.274 = sign_extend 1
            tmp.275 = add_ptr(tmp.273, index=tmp.274, scale=8)
            tmp.276 = *tmp.275
            tmp.277 = tmp.276 == 150.5D
            if !tmp.277 jump and_false_128
            tmp.272 = 1
            jump and_end_129
        
          and_false_128:
            tmp.272 = 0
        
          and_end_129:
            tmp.278 = ! tmp.272
            if !tmp.278 jump end_if_130
            return 0
        
          end_if_130:
            tmp.279 = one_gp.197[0]
            tmp.280 = tmp.279 == 13D
            tmp.281 = ! tmp.280
            if !tmp.281 jump end_if_132
            return 0
        
          end_if_132:
            tmp.282 = &uneven.198
            tmp.283 = &string.1
            tmp.284 = strcmp(tmp.282, tmp.283)
            tmp.285 = tmp.284 != 0
            if !tmp.285 jump end_if_134
            return 0
        
          end_if_134:
            return 1
            return 0
        }
        global function pass_in_mem_first(mem.199, mixed_regs.200, two_gp.201, one_gp_struct.202) { 
            tmp.286 = &mem.199
            tmp.287 = sign_extend 0
            tmp.288 = add_ptr(tmp.286, index=tmp.287, scale=8)
            tmp.289 = *tmp.288
            tmp.290 = tmp.289 == 66D
            if !tmp.290 jump and_false_136
            tmp.293 = &mem.199
            tmp.294 = sign_extend 1
            tmp.295 = add_ptr(tmp.293, index=tmp.294, scale=8)
            tmp.296 = *tmp.295
            tmp.297 = tmp.296 == 77D
            if !tmp.297 jump and_false_136
            tmp.292 = 1
            jump and_end_137
        
          and_false_136:
            tmp.292 = 0
        
          and_end_137:
            if !tmp.292 jump and_false_138
            tmp.300 = &mem.199
            tmp.301 = sign_extend 2
            tmp.302 = add_ptr(tmp.300, index=tmp.301, scale=8)
            tmp.303 = *tmp.302
            tmp.304 = tmp.303 == 88D
            if !tmp.304 jump and_false_138
            tmp.299 = 1
            jump and_end_139
        
          and_false_138:
            tmp.299 = 0
        
          and_end_139:
            tmp.305 = ! tmp.299
            if !tmp.305 jump end_if_140
            return 0
        
          end_if_140:
            tmp.306 = &mixed_regs.200
            tmp.307 = sign_extend 0
            tmp.308 = add_ptr(tmp.306, index=tmp.307, scale=8)
            tmp.309 = *tmp.308
            tmp.311 = int_to_double 0
            tmp.310 = tmp.309 == tmp.311
            if !tmp.310 jump and_false_142
            tmp.314 = &mixed_regs.200
            tmp.315 = sign_extend 1
            tmp.316 = add_ptr(tmp.314, index=tmp.315, scale=8)
            tmp.317 = *tmp.316
            tmp.318 = tmp.317 == 150.5D
            if !tmp.318 jump and_false_142
            tmp.313 = 1
            jump and_end_143
        
          and_false_142:
            tmp.313 = 0
        
          and_end_143:
            tmp.319 = ! tmp.313
            if !tmp.319 jump end_if_144
            return 0
        
          end_if_144:
            tmp.320 = &two_gp.201
            tmp.321 = &string.0
            tmp.322 = strcmp(tmp.320, tmp.321)
            tmp.323 = tmp.322 != 0
            if !tmp.323 jump end_if_146
            return 0
        
          end_if_146:
            tmp.324 = one_gp_struct.202[0]
            tmp.326 = - 24
            tmp.327 = tmp.326
            tmp.325 = tmp.324 == tmp.327
            if !tmp.325 jump and_false_148
            tmp.330 = one_gp_struct.202[4]
            tmp.331 = tmp.330 == 123456789
            if !tmp.331 jump and_false_148
            tmp.329 = 1
            jump and_end_149
        
          and_false_148:
            tmp.329 = 0
        
          and_end_149:
            tmp.332 = ! tmp.329
            if !tmp.332 jump end_if_150
            return 0
        
          end_if_150:
            return 1
            return 0
        }
        constant string.0: Array(11,Char) = "+_)(*&^%$#\\0"
        constant string.1: Array(5,Char) = "boop\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_param_passing_client() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        void exit(int status);
        void *malloc(unsigned long size);
        union one_double {
            double d1;
            double d2;
        };
        struct has_union_with_double {
            union one_double member;
        };
        union has_struct_with_double {
            struct has_union_with_double s;
            double arr[1];
        };
        union one_int {
            double d;
            char c;
        };
        union one_int_nested {
            union one_int oi;
            union one_double od;
        };
        union char_int_mixed {
            char arr[7];
            union char_int_mixed* union_ptr;
            unsigned int ui;
        };
        union char_int_short {
            char c;
            int i;
        };
        struct has_union {
            unsigned int i;
            union char_int_short u;
        };
        union has_struct_with_ints {
            double d;
            struct has_union s;
            unsigned long ul;
        };
        union two_doubles {
            double arr[2];
            double single;
        };
        union has_xmm_union {
            union one_double u;
            union two_doubles u2;
        };
        struct dbl_struct {
            union one_double member1;
            double member2;
        };
        union has_dbl_struct {
            struct dbl_struct member1;
        };
        union char_arr {
            char arr[11];
            int i;
        };
        union two_arrs {
            double dbl_arr[2];
            long long_arr[2];
        };
        union two_eightbyte_has_struct {
            int arr[3];
            struct dbl_struct member1;
        };
        struct char_first_eightbyte {
            char c;
            double d;
        };
        struct int_second_eightbyte {
            double d;
            int i;
        };
        union two_structs {
            struct char_first_eightbyte member1;
            struct int_second_eightbyte member2;
        };
        struct nine_bytes {
            int i;
            char arr[5];
        };
        union has_nine_byte_struct {
            char c;
            long l;
            struct nine_bytes s;
        };
        union uneven {
            char arr[5];
            unsigned char uc;
        };
        struct has_uneven_union {
            int i;
            union uneven u;
        };
        union has_other_unions {
            union uneven u;
            union two_doubles d;
            union has_nine_byte_struct n;
        };
        union union_array {
            union one_int u_arr[2];
        };
        union uneven_union_array {
            union uneven u_arr[2];
        };
        struct small {
            char arr[3];
            signed char sc;
        };
        union has_small_struct_array {
            struct small arr[3];
        };
        union gp_and_xmm {
            double d_arr[2];
            char c;
        };
        union scalar_and_struct {
            long* ptr;
            struct char_first_eightbyte cfe;
        };
        struct has_two_unions {
            union char_int_mixed member1;
            union one_double member2;
        };
        union small_struct_arr_and_dbl {
            struct small arr[2];
            union two_doubles d;
        };
        union xmm_and_gp {
            double d;
            struct int_second_eightbyte ise;
        };
        union xmm_and_gp_nested {
            union xmm_and_gp member1;
            double arr[2];
            union two_doubles d;
        };
        union lotsa_doubles {
            double arr[3];
            int i;
        };
        union lotsa_chars {
            char more_chars[18];
            char fewer_chars[5];
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        union contains_large_struct {
            int i;
            unsigned long ul;
            struct large l;
        };
        union contains_union_array {
            union gp_and_xmm arr[2];
        };
        int test_one_double(union one_double u);
        int test_has_union_with_double(struct has_union_with_double s);
        int test_has_struct_with_double(union has_struct_with_double u);
        int test_one_int(union one_int u);
        int test_one_int_nested(union one_int_nested u);
        int test_char_int_mixed(union char_int_mixed u);
        int test_has_union(struct has_union s);
        int test_has_struct_with_ints(union has_struct_with_ints u);
        int test_two_doubles(union two_doubles u);
        int test_has_xmm_union(union has_xmm_union u);
        int test_dbl_struct(struct dbl_struct s);
        int test_has_dbl_struct(union has_dbl_struct u);
        int test_char_arr(union char_arr u);
        int test_two_arrs(union two_arrs u);
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u);
        int test_two_structs(union two_structs u);
        int test_has_nine_byte_struct(union has_nine_byte_struct u);
        int test_has_uneven_union(struct has_uneven_union s);
        int test_has_other_unions(union has_other_unions u);
        int test_union_array(union union_array u);
        int test_uneven_union_array(union uneven_union_array u);
        int test_has_small_struct_array(union has_small_struct_array u);
        int test_gp_and_xmm(union gp_and_xmm u);
        int test_scalar_and_struct(union scalar_and_struct u);
        int test_has_two_unions(struct has_two_unions s);
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u);
        int test_xmm_and_gp(union xmm_and_gp u);
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u);
        int test_lotsa_doubles(union lotsa_doubles u);
        int test_lotsa_chars(union lotsa_chars u);
        int test_contains_large_struct(union contains_large_struct u);
        int test_contains_union_array(union contains_union_array u);
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5);
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp);
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2);
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp);
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2);
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs);
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven);
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct);
        union one_double return_one_double(void);
        union one_int_nested return_one_int_nested(void);
        union has_dbl_struct return_has_dbl_struct(void);
        union two_arrs return_two_arrs(void);
        union scalar_and_struct return_scalar_and_struct(void);
        union xmm_and_gp return_xmm_and_gp(void);
        union contains_union_array return_contains_union_array(void);
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin);
        struct has_uneven_union return_struct_with_union(void);
        
        int main(void) {
            union two_doubles two_xmm = { {-10.0, -11.0} };
            union one_int one_gp = { 13.0 };
            struct has_union one_gp_struct = { -24, {0} };
            one_gp_struct.u.i = 123456789;
            if (!pass_unions_and_structs(1, 2, one_gp_struct, 4.0, two_xmm, one_gp, 100, 120, 130)) {
                return 1;
            }
            if (!pass_gp_union_in_memory(two_xmm, one_gp_struct, -1, -2, -3, -4, -5, -6, one_gp)) {
                return 2;
            }
            union two_doubles two_xmm_2 = { {33e4, 55e6 } };
            if (!pass_xmm_union_in_memory(1.0, 2.0, two_xmm, two_xmm, 3.0, 4.0, two_xmm_2)) {
                return 3;
            }
            union char_arr two_gp = { "+_)(*&^%$#" };
            if (!pass_borderline_union(1, 2, 3, 4, 5, two_gp)) {
                return 4;
            }
            two_xmm_2.arr[0] = two_xmm_2.arr[0] * 2;
            two_xmm_2.arr[1] = two_xmm_2.arr[1] * 2;
            if (!pass_borderline_xmm_union(two_xmm, 9.0, 8.0, 7.0, 6.0, 5.0, two_xmm_2)) {
                return 5;
            }
            union gp_and_xmm mixed_regs = { {0, 150.5} };
            if (!pass_mixed_reg_in_memory(101.2, 102.3, 103.4, 104.5, 75, 76, 77, 78, 79, 80, mixed_regs)) {
                return 6;
            }
            union uneven uneven = { "boop" };
            if (!pass_uneven_union_in_memory(1100, 2200, 3300, 4400, 5500, mixed_regs, one_gp, uneven)) {
                return 7;
            }
            union lotsa_doubles mem = { {66., 77., 88.} };
            if (!pass_in_mem_first(mem, mixed_regs, two_gp, one_gp_struct)) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 10D
            two_xmm.142[0] = tmp.0
            tmp.1 = - 11D
            two_xmm.142[8] = tmp.1
            one_gp.143[0] = 13D
            tmp.2 = - 24
            tmp.3 = tmp.2
            one_gp_struct.144[0] = tmp.3
            tmp.4 = truncate 0
            one_gp_struct.144[4] = tmp.4
            one_gp_struct.144[4] = 123456789
            tmp.5 = one_gp_struct.144[4]
            tmp.6 = pass_unions_and_structs(1, 2, one_gp_struct.144, 4D, two_xmm.142, one_gp.143, 100, 120, 130)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = - 1
            tmp.9 = - 2
            tmp.10 = - 3
            tmp.11 = - 4
            tmp.12 = - 5
            tmp.13 = - 6
            tmp.14 = pass_gp_union_in_memory(two_xmm.142, one_gp_struct.144, tmp.8, tmp.9, tmp.10, tmp.11, tmp.12, tmp.13, one_gp.143)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_2
            return 2
        
          end_if_2:
            two_xmm_2.145[0] = 330000D
            two_xmm_2.145[8] = 55000000D
            tmp.16 = pass_xmm_union_in_memory(1D, 2D, two_xmm.142, two_xmm.142, 3D, 4D, two_xmm_2.145)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            two_gp.146[0] = '+'
            two_gp.146[1] = '_'
            two_gp.146[2] = ')'
            two_gp.146[3] = '('
            two_gp.146[4] = '*'
            two_gp.146[5] = '&'
            two_gp.146[6] = '^'
            two_gp.146[7] = '%'
            two_gp.146[8] = '$'
            two_gp.146[9] = '#'
            two_gp.146[10] = '\0'
            tmp.18 = pass_borderline_union(1, 2, 3, 4, 5, two_gp.146)
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = &two_xmm_2.145
            tmp.21 = sign_extend 0
            tmp.22 = add_ptr(tmp.20, index=tmp.21, scale=8)
            tmp.23 = &two_xmm_2.145
            tmp.24 = sign_extend 0
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=8)
            tmp.26 = *tmp.25
            tmp.28 = int_to_double 2
            tmp.27 = tmp.26 * tmp.28
            *tmp.22 = tmp.27
            tmp.29 = *tmp.22
            tmp.30 = &two_xmm_2.145
            tmp.31 = sign_extend 1
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=8)
            tmp.33 = &two_xmm_2.145
            tmp.34 = sign_extend 1
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=8)
            tmp.36 = *tmp.35
            tmp.38 = int_to_double 2
            tmp.37 = tmp.36 * tmp.38
            *tmp.32 = tmp.37
            tmp.39 = *tmp.32
            tmp.40 = pass_borderline_xmm_union(two_xmm.142, 9D, 8D, 7D, 6D, 5D, two_xmm_2.145)
            tmp.41 = ! tmp.40
            if !tmp.41 jump end_if_8
            return 5
        
          end_if_8:
            tmp.42 = int_to_double 0
            mixed_regs.147[0] = tmp.42
            mixed_regs.147[8] = 150.5D
            tmp.43 = pass_mixed_reg_in_memory(101.2D, 102.3D, 103.4D, 104.5D, 75, 76, 77, 78, 79, 80, mixed_regs.147)
            tmp.44 = ! tmp.43
            if !tmp.44 jump end_if_10
            return 6
        
          end_if_10:
            uneven.148[0] = 'b'
            uneven.148[1] = 'o'
            uneven.148[2] = 'o'
            uneven.148[3] = 'p'
            uneven.148[4] = '\0'
            tmp.45 = pass_uneven_union_in_memory(1100, 2200, 3300, 4400, 5500, mixed_regs.147, one_gp.143, uneven.148)
            tmp.46 = ! tmp.45
            if !tmp.46 jump end_if_12
            return 7
        
          end_if_12:
            mem.149[0] = 66D
            mem.149[8] = 77D
            mem.149[16] = 88D
            tmp.47 = pass_in_mem_first(mem.149, mixed_regs.147, two_gp.146, one_gp_struct.144)
            tmp.48 = ! tmp.47
            if !tmp.48 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_static_union_inits() {
    let src = r#"
        
        int strcmp(char* s1, char* s2);
        union simple {
            int i;
            char c;
            double d;
        };
        extern union simple s;
        int validate_simple(void);
        union has_union {
            union simple u;
            char c;
        };
        extern union has_union h;
        int validate_has_union(void);
        struct has_union_array {
            union has_union union_array[4];
            char c;
            union simple s;
        };
        extern struct has_union_array my_struct;
        int validate_has_union_array(void);
        extern union has_union all_zeros;
        int validate_uninitialized(void);
        union with_padding {
            char arr[13];
            long l;
        };
        extern union with_padding padded_union_array[3];
        int validate_padded_union_array(void);
        int validate_simple(void) {
            return (s.c == -39 && s.i == 217);
        }
        int validate_has_union(void) {
            return (h.u.c == 77 && h.c == 77 && h.u.i == 77);
        }
        int validate_has_union_array(void) {
            for (int i = 0; i < 3; i = i + 1) {
                int expected = 'a' + i;
                if (my_struct.union_array[i].u.c != expected
                    || my_struct.union_array[i].c != expected
                    || my_struct.union_array[i].u.i != expected) {
                    return 0;
                }
            }
            if (my_struct.union_array[3].u.d != 0.0) {
                return 0;
            }
            if (my_struct.c != '#') {
                return 0;
            }
            if (my_struct.s.c != '!' || my_struct.s.i != '!') {
                return 0;
            }
            return 1;
        }
        int validate_uninitialized(void) {
            if (all_zeros.u.d != 0.0) {
                return 0;
            }
            return 1;
        }
        int validate_padded_union_array(void) {
            if (strcmp(padded_union_array[0].arr, "first string") != 0) {
                return 0;
            }
            if (strcmp(padded_union_array[1].arr, "string #2") != 0) {
                return 0;
            }
            if (strcmp(padded_union_array[2].arr, "string #3") != 0) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function validate_simple() { 
            tmp.0 = s[0]
            tmp.1 = sign_extend tmp.0
            tmp.3 = - 39
            tmp.2 = tmp.1 == tmp.3
            if !tmp.2 jump and_false_0
            tmp.6 = s[0]
            tmp.7 = tmp.6 == 217
            if !tmp.7 jump and_false_0
            tmp.5 = 1
            jump and_end_1
        
          and_false_0:
            tmp.5 = 0
        
          and_end_1:
            return tmp.5
            return 0
        }
        global function validate_has_union() { 
            tmp.8 = h[0]
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 == 77
            if !tmp.10 jump and_false_2
            tmp.13 = h[0]
            tmp.14 = sign_extend tmp.13
            tmp.15 = tmp.14 == 77
            if !tmp.15 jump and_false_2
            tmp.12 = 1
            jump and_end_3
        
          and_false_2:
            tmp.12 = 0
        
          and_end_3:
            if !tmp.12 jump and_false_4
            tmp.18 = h[0]
            tmp.19 = tmp.18 == 77
            if !tmp.19 jump and_false_4
            tmp.17 = 1
            jump and_end_5
        
          and_false_4:
            tmp.17 = 0
        
          and_end_5:
            return tmp.17
            return 0
        }
        global function validate_has_union_array() { 
            i.6 = 0
        
          start_loop_0:
            tmp.20 = i.6 < 3
            if !tmp.20 jump break_loop_0
            tmp.21 = 97 + i.6
            expected.7 = tmp.21
            tmp.22 = &my_struct
            tmp.23 = sign_extend i.6
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=8)
            tmp.25 = *tmp.24
            tmp.26 = sign_extend tmp.25
            tmp.27 = tmp.26 != expected.7
            if tmp.27 jump or_true_6
            tmp.30 = &my_struct
            tmp.31 = sign_extend i.6
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=8)
            tmp.33 = *tmp.32
            tmp.34 = sign_extend tmp.33
            tmp.35 = tmp.34 != expected.7
            if tmp.35 jump or_true_6
            tmp.29 = 0
            jump or_end_7
        
          or_true_6:
            tmp.29 = 1
        
          or_end_7:
            if tmp.29 jump or_true_8
            tmp.38 = &my_struct
            tmp.39 = sign_extend i.6
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=8)
            tmp.41 = *tmp.40
            tmp.42 = tmp.41 != expected.7
            if tmp.42 jump or_true_8
            tmp.37 = 0
            jump or_end_9
        
          or_true_8:
            tmp.37 = 1
        
          or_end_9:
            if !tmp.37 jump end_if_10
            return 0
        
          end_if_10:
        
          continue_loop_0:
            tmp.43 = i.6 + 1
            i.6 = tmp.43
            jump start_loop_0
        
          break_loop_0:
            tmp.44 = &my_struct
            tmp.45 = sign_extend 3
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=8)
            tmp.47 = *tmp.46
            tmp.48 = tmp.47 != 0D
            if !tmp.48 jump end_if_12
            return 0
        
          end_if_12:
            tmp.49 = my_struct[32]
            tmp.50 = sign_extend tmp.49
            tmp.51 = tmp.50 != 35
            if !tmp.51 jump end_if_14
            return 0
        
          end_if_14:
            tmp.52 = my_struct[40]
            tmp.53 = sign_extend tmp.52
            tmp.54 = tmp.53 != 33
            if tmp.54 jump or_true_16
            tmp.57 = my_struct[40]
            tmp.58 = tmp.57 != 33
            if tmp.58 jump or_true_16
            tmp.56 = 0
            jump or_end_17
        
          or_true_16:
            tmp.56 = 1
        
          or_end_17:
            if !tmp.56 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function validate_uninitialized() { 
            tmp.59 = all_zeros[0]
            tmp.60 = tmp.59 != 0D
            if !tmp.60 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function validate_padded_union_array() { 
            tmp.61 = &padded_union_array
            tmp.62 = sign_extend 0
            tmp.63 = add_ptr(tmp.61, index=tmp.62, scale=16)
            tmp.64 = &string.0
            tmp.65 = strcmp(tmp.63, tmp.64)
            tmp.66 = tmp.65 != 0
            if !tmp.66 jump end_if_22
            return 0
        
          end_if_22:
            tmp.67 = &padded_union_array
            tmp.68 = sign_extend 1
            tmp.69 = add_ptr(tmp.67, index=tmp.68, scale=16)
            tmp.70 = &string.1
            tmp.71 = strcmp(tmp.69, tmp.70)
            tmp.72 = tmp.71 != 0
            if !tmp.72 jump end_if_24
            return 0
        
          end_if_24:
            tmp.73 = &padded_union_array
            tmp.74 = sign_extend 2
            tmp.75 = add_ptr(tmp.73, index=tmp.74, scale=16)
            tmp.76 = &string.2
            tmp.77 = strcmp(tmp.75, tmp.76)
            tmp.78 = tmp.77 != 0
            if !tmp.78 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        constant string.0: Array(13,Char) = "first string\\0"
        constant string.1: Array(10,Char) = "string #2\\0"
        constant string.2: Array(10,Char) = "string #3\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_static_union_inits_client() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        union simple {
            int i;
            char c;
            double d;
        };
        extern union simple s;
        int validate_simple(void);
        union has_union {
            union simple u;
            char c;
        };
        extern union has_union h;
        int validate_has_union(void);
        struct has_union_array {
            union has_union union_array[4];
            char c;
            union simple s;
        };
        extern struct has_union_array my_struct;
        int validate_has_union_array(void);
        extern union has_union all_zeros;
        int validate_uninitialized(void);
        union with_padding {
            char arr[13];
            long l;
        };
        extern union with_padding padded_union_array[3];
        int validate_padded_union_array(void);
        union simple s = {217};
        union has_union h = {{77}};
        struct has_union_array my_struct = {
            {{{'a'}}, {{'b'}}, {{'c'}}}, '#', {'!'}
        };
        union has_union all_zeros;
        union with_padding padded_union_array[3] = {
            {"first string"}, {"string #2"}, {
                "string #3"
            }
        };
        int main(void) {
            if (!validate_simple()) {
                return 1;
            }
            if (!validate_has_union()){
                return 2;
            }
            if (!validate_has_union_array()) {
                return 3;
            }
            if (!validate_uninitialized()) {
                return 4;
            }
            if (!validate_padded_union_array()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = validate_simple()
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = validate_has_union()
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = validate_has_union_array()
            tmp.5 = ! tmp.4
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.6 = validate_uninitialized()
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            tmp.8 = validate_padded_union_array()
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static global all_zeros: Union(has_union.3) = zero[8]
        static global h: Union(has_union.3) = [ 77, zero[4]]
        static global my_struct: Struct(has_union_array.4) = [ 97, zero[4], 98, zero[4], 99, zero[4], zero[8], '#', zero[7], 33, zero[4]]
        static global padded_union_array: Array(3,Union(with_padding.5)) = [ "first string\\0", zero[3], "string #2\\0", zero[3], zero[3], "string #3\\0", zero[3], zero[3]]
        static global s: Union(simple.2) = [ 217, zero[4]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_union_inits() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        union simple {
            double d;
            char c;
            int *ptr;
        };
        union inner {
            char arr[9];
        };
        struct my_struct {
            long l;
            union inner u;
            int i;
        };
        union nested {
            struct my_struct str;
            union simple s;
            long l;
        };
        int validate_simple(union simple *ptr);
        int validate_simple_converted(union simple *ptr);
        int validate_nested(union nested *ptr);
        int validate_nested_partial(union nested *ptr);
        
        int validate_simple(union simple* ptr) {
            return (ptr->d == 123.45);
        }
        int validate_simple_converted(union simple* ptr) {
            return (ptr->d == 18446744073709549568.);
        }
        int validate_nested(union nested* ptr) {
            if (ptr->str.l != 4294967395l) {
                return 0;
            }
            for (int i = 0; i < 9; i = i + 1) {
                if (ptr->str.u.arr[i] != -1 - i) {
                    return 0;
                }
            }
            return 1;
        }
        int validate_nested_partial(union nested* ptr) {
            if (ptr->str.l != 9000372036854775800l) {
                return 0;
            }
            if (strcmp(ptr->str.u.arr, "string")) {
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function validate_simple(ptr.10) { 
            tmp.0 = *ptr.10
            tmp.1 = tmp.0 == 123.45D
            return tmp.1
            return 0
        }
        global function validate_simple_converted(ptr.11) { 
            tmp.2 = *ptr.11
            tmp.3 = tmp.2 == 18446744073709550000D
            return tmp.3
            return 0
        }
        global function validate_nested(ptr.12) { 
            tmp.4 = *ptr.12
            tmp.5 = tmp.4 != 4294967395L
            if !tmp.5 jump end_if_0
            return 0
        
          end_if_0:
            i.13 = 0
        
          start_loop_0:
            tmp.6 = i.13 < 9
            if !tmp.6 jump break_loop_0
            tmp.7 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.8 = sign_extend i.13
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=1)
            tmp.10 = *tmp.9
            tmp.11 = sign_extend tmp.10
            tmp.13 = - 1
            tmp.14 = tmp.13 - i.13
            tmp.12 = tmp.11 != tmp.14
            if !tmp.12 jump end_if_2
            return 0
        
          end_if_2:
        
          continue_loop_0:
            tmp.15 = i.13 + 1
            i.13 = tmp.15
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
        global function validate_nested_partial(ptr.14) { 
            tmp.16 = *ptr.14
            tmp.17 = tmp.16 != 9000372036854775800L
            if !tmp.17 jump end_if_4
            return 0
        
          end_if_4:
            tmp.18 = add_ptr(ptr.14, index=8L, scale=1)
            tmp.19 = &string.0
            tmp.20 = strcmp(tmp.18, tmp.19)
            if !tmp.20 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        constant string.0: Array(7,Char) = "string\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_union_inits_client() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        union simple {
            double d;
            char c;
            int *ptr;
        };
        union inner {
            char arr[9];
        };
        struct my_struct {
            long l;
            union inner u;
            int i;
        };
        union nested {
            struct my_struct str;
            union simple s;
            long l;
        };
        int validate_simple(union simple *ptr);
        int validate_simple_converted(union simple *ptr);
        int validate_nested(union nested *ptr);
        int validate_nested_partial(union nested *ptr);
        int test_simple(void) {
            union simple x = { 123.45 };
            return validate_simple(&x);
        }
        int test_simple_converted(void) {
            union simple x = { 18446744073709550315UL };
            return validate_simple_converted(&x);
        }
        int test_nested(void) {
            union nested x = { {4294967395l, {{-1, -2, -3, -4, -5, -6, -7, -8, -9}}} };
            return validate_nested(&x);
        }
        int test_nested_partial_init(void) {
            union nested x = { {9000372036854775800l, {"string"}} };
            return validate_nested_partial(&x);
        }
        int main(void) {
            if (!test_simple()) {
                return 1;
            }
            if (!test_simple_converted()) {
                return 2;
            }
            if (!test_nested()) {
                return 3;
            }
            if (!test_nested_partial_init()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_simple() { 
            x.10[0] = 123.45D
            tmp.0 = &x.10
            tmp.1 = validate_simple(tmp.0)
            return tmp.1
            return 0
        }
        global function test_simple_converted() { 
            tmp.2 = uint_to_double 18446744073709550315UL
            x.11[0] = tmp.2
            tmp.3 = &x.11
            tmp.4 = validate_simple_converted(tmp.3)
            return tmp.4
            return 0
        }
        global function test_nested() { 
            x.12[0] = 4294967395L
            tmp.5 = - 1
            tmp.6 = truncate tmp.5
            x.12[8] = tmp.6
            tmp.7 = - 2
            tmp.8 = truncate tmp.7
            x.12[9] = tmp.8
            tmp.9 = - 3
            tmp.10 = truncate tmp.9
            x.12[10] = tmp.10
            tmp.11 = - 4
            tmp.12 = truncate tmp.11
            x.12[11] = tmp.12
            tmp.13 = - 5
            tmp.14 = truncate tmp.13
            x.12[12] = tmp.14
            tmp.15 = - 6
            tmp.16 = truncate tmp.15
            x.12[13] = tmp.16
            tmp.17 = - 7
            tmp.18 = truncate tmp.17
            x.12[14] = tmp.18
            tmp.19 = - 8
            tmp.20 = truncate tmp.19
            x.12[15] = tmp.20
            tmp.21 = - 9
            tmp.22 = truncate tmp.21
            x.12[16] = tmp.22
            x.12[20] = 0
            tmp.23 = &x.12
            tmp.24 = validate_nested(tmp.23)
            return tmp.24
            return 0
        }
        global function test_nested_partial_init() { 
            x.13[0] = 9000372036854775800L
            x.13[8] = 's'
            x.13[9] = 't'
            x.13[10] = 'r'
            x.13[11] = 'i'
            x.13[12] = 'n'
            x.13[13] = 'g'
            x.13[14] = '\0'
            x.13[15] = '\0'
            x.13[16] = '\0'
            x.13[20] = 0
            tmp.25 = &x.13
            tmp.26 = validate_nested_partial(tmp.25)
            return tmp.26
            return 0
        }
        global function main() { 
            tmp.27 = test_simple()
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_0
            return 1
        
          end_if_0:
            tmp.29 = test_simple_converted()
            tmp.30 = ! tmp.29
            if !tmp.30 jump end_if_2
            return 2
        
          end_if_2:
            tmp.31 = test_nested()
            tmp.32 = ! tmp.31
            if !tmp.32 jump end_if_4
            return 3
        
          end_if_4:
            tmp.33 = test_nested_partial_init()
            tmp.34 = ! tmp.33
            if !tmp.34 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_union_retvals() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        void exit(int status);
        void *malloc(unsigned long size);
        union one_double {
            double d1;
            double d2;
        };
        struct has_union_with_double {
            union one_double member;
        };
        union has_struct_with_double {
            struct has_union_with_double s;
            double arr[1];
        };
        union one_int {
            double d;
            char c;
        };
        union one_int_nested {
            union one_int oi;
            union one_double od;
        };
        union char_int_mixed {
            char arr[7];
            union char_int_mixed* union_ptr;
            unsigned int ui;
        };
        union char_int_short {
            char c;
            int i;
        };
        struct has_union {
            unsigned int i;
            union char_int_short u;
        };
        union has_struct_with_ints {
            double d;
            struct has_union s;
            unsigned long ul;
        };
        union two_doubles {
            double arr[2];
            double single;
        };
        union has_xmm_union {
            union one_double u;
            union two_doubles u2;
        };
        struct dbl_struct {
            union one_double member1;
            double member2;
        };
        union has_dbl_struct {
            struct dbl_struct member1;
        };
        union char_arr {
            char arr[11];
            int i;
        };
        union two_arrs {
            double dbl_arr[2];
            long long_arr[2];
        };
        union two_eightbyte_has_struct {
            int arr[3];
            struct dbl_struct member1;
        };
        struct char_first_eightbyte {
            char c;
            double d;
        };
        struct int_second_eightbyte {
            double d;
            int i;
        };
        union two_structs {
            struct char_first_eightbyte member1;
            struct int_second_eightbyte member2;
        };
        struct nine_bytes {
            int i;
            char arr[5];
        };
        union has_nine_byte_struct {
            char c;
            long l;
            struct nine_bytes s;
        };
        union uneven {
            char arr[5];
            unsigned char uc;
        };
        struct has_uneven_union {
            int i;
            union uneven u;
        };
        union has_other_unions {
            union uneven u;
            union two_doubles d;
            union has_nine_byte_struct n;
        };
        union union_array {
            union one_int u_arr[2];
        };
        union uneven_union_array {
            union uneven u_arr[2];
        };
        struct small {
            char arr[3];
            signed char sc;
        };
        union has_small_struct_array {
            struct small arr[3];
        };
        union gp_and_xmm {
            double d_arr[2];
            char c;
        };
        union scalar_and_struct {
            long* ptr;
            struct char_first_eightbyte cfe;
        };
        struct has_two_unions {
            union char_int_mixed member1;
            union one_double member2;
        };
        union small_struct_arr_and_dbl {
            struct small arr[2];
            union two_doubles d;
        };
        union xmm_and_gp {
            double d;
            struct int_second_eightbyte ise;
        };
        union xmm_and_gp_nested {
            union xmm_and_gp member1;
            double arr[2];
            union two_doubles d;
        };
        union lotsa_doubles {
            double arr[3];
            int i;
        };
        union lotsa_chars {
            char more_chars[18];
            char fewer_chars[5];
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        union contains_large_struct {
            int i;
            unsigned long ul;
            struct large l;
        };
        union contains_union_array {
            union gp_and_xmm arr[2];
        };
        int test_one_double(union one_double u);
        int test_has_union_with_double(struct has_union_with_double s);
        int test_has_struct_with_double(union has_struct_with_double u);
        int test_one_int(union one_int u);
        int test_one_int_nested(union one_int_nested u);
        int test_char_int_mixed(union char_int_mixed u);
        int test_has_union(struct has_union s);
        int test_has_struct_with_ints(union has_struct_with_ints u);
        int test_two_doubles(union two_doubles u);
        int test_has_xmm_union(union has_xmm_union u);
        int test_dbl_struct(struct dbl_struct s);
        int test_has_dbl_struct(union has_dbl_struct u);
        int test_char_arr(union char_arr u);
        int test_two_arrs(union two_arrs u);
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u);
        int test_two_structs(union two_structs u);
        int test_has_nine_byte_struct(union has_nine_byte_struct u);
        int test_has_uneven_union(struct has_uneven_union s);
        int test_has_other_unions(union has_other_unions u);
        int test_union_array(union union_array u);
        int test_uneven_union_array(union uneven_union_array u);
        int test_has_small_struct_array(union has_small_struct_array u);
        int test_gp_and_xmm(union gp_and_xmm u);
        int test_scalar_and_struct(union scalar_and_struct u);
        int test_has_two_unions(struct has_two_unions s);
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u);
        int test_xmm_and_gp(union xmm_and_gp u);
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u);
        int test_lotsa_doubles(union lotsa_doubles u);
        int test_lotsa_chars(union lotsa_chars u);
        int test_contains_large_struct(union contains_large_struct u);
        int test_contains_union_array(union contains_union_array u);
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5);
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp);
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2);
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp);
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2);
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs);
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven);
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct);
        union one_double return_one_double(void);
        union one_int_nested return_one_int_nested(void);
        union has_dbl_struct return_has_dbl_struct(void);
        union two_arrs return_two_arrs(void);
        union scalar_and_struct return_scalar_and_struct(void);
        union xmm_and_gp return_xmm_and_gp(void);
        union contains_union_array return_contains_union_array(void);
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin);
        struct has_uneven_union return_struct_with_union(void);
        
        union one_double return_one_double(void) {
            union one_double result = { 245.5 };
            return result;
        }
        union one_int_nested return_one_int_nested(void) {
            union one_int_nested result = { {-9876.5} };
            return result;
        }
        union has_dbl_struct return_has_dbl_struct(void) {
            union has_dbl_struct result = {
                {
                    {1234.5}, 6789.
                }
            };
            return result;
        }
        union two_arrs return_two_arrs(void) {
            union two_arrs result;
            result.dbl_arr[0] = 66.75;
            result.long_arr[1] = -4294967300l;
            return result;
        }
        union scalar_and_struct return_scalar_and_struct(void) {
            union scalar_and_struct result;
            result.cfe.c = -115;
            result.cfe.d = 222222.25;
            return result;
        }
        union xmm_and_gp return_xmm_and_gp(void) {
            union xmm_and_gp result;
            result.ise.d = -50000.125;
            result.ise.i = -3000;
            return result;
        }
        union contains_union_array return_contains_union_array(void) {
            union contains_union_array result = {
                {
                    {{-2000e-4, -3000e-4}}, {{20000e10, 5000e11}}
                }
            };
            return result;
        }
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin) {
            if (i1 != 1 || i2 != 25) {
                exit(-1);
            }
            if (int_and_dbl.cfe.c != -115 || int_and_dbl.cfe.d != 222222.25) {
                exit(-2);
            }
            if (two_arrs.dbl_arr[0] != 66.75 || two_arrs.long_arr[1] != -4294967300l) {
                exit(-3);
            }
            if (!(big_union.arr[0].d_arr[0] == -2000e-4 && big_union.arr[0].d_arr[1] == -3000e-4
                && big_union.arr[1].d_arr[0] == 20000e10 && big_union.arr[1].d_arr[1] == 5000e11)) {
                exit(-4);
            }
            if (oin.oi.d != -9876.5) {
                exit(-5);
            }
            union lotsa_chars result = { "ABCDEFGHIJKLMNOPQ" };
            return result;
        }
        struct has_uneven_union return_struct_with_union(void) {
            struct has_uneven_union result = {
                -8765, {"done"}
            };
            return result;
        }
    "#;
    let expected = r#"
        global function return_one_double() { 
            result.142[0] = 245.5D
            return result.142
            return 0
        }
        global function return_one_int_nested() { 
            tmp.0 = - 9876.5D
            result.143[0] = tmp.0
            return result.143
            return 0
        }
        global function return_has_dbl_struct() { 
            result.144[0] = 1234.5D
            result.144[8] = 6789D
            return result.144
            return 0
        }
        global function return_two_arrs() { 
            tmp.1 = &result.145
            tmp.2 = sign_extend 0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=8)
            *tmp.3 = 66.75D
            tmp.4 = *tmp.3
            tmp.5 = &result.145
            tmp.6 = sign_extend 1
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=8)
            tmp.8 = - 4294967300L
            *tmp.7 = tmp.8
            tmp.9 = *tmp.7
            return result.145
            return 0
        }
        global function return_scalar_and_struct() { 
            tmp.10 = - 115
            tmp.11 = truncate tmp.10
            result.146[0] = tmp.11
            tmp.12 = result.146[0]
            result.146[8] = 222222.25D
            tmp.13 = result.146[8]
            return result.146
            return 0
        }
        global function return_xmm_and_gp() { 
            tmp.14 = - 50000.125D
            result.147[0] = tmp.14
            tmp.15 = result.147[0]
            tmp.16 = - 3000
            result.147[8] = tmp.16
            tmp.17 = result.147[8]
            return result.147
            return 0
        }
        global function return_contains_union_array() { 
            tmp.18 = - 0.2D
            result.148[0] = tmp.18
            tmp.19 = - 0.3D
            result.148[8] = tmp.19
            result.148[16] = 200000000000000D
            result.148[24] = 500000000000000D
            return result.148
            return 0
        }
        global function pass_params_and_return_in_mem(i1.149, int_and_dbl.150, two_arrs.151, i2.152, big_union.153, oin.154) { 
            tmp.20 = i1.149 != 1
            if tmp.20 jump or_true_0
            tmp.23 = i2.152 != 25
            if tmp.23 jump or_true_0
            tmp.22 = 0
            jump or_end_1
        
          or_true_0:
            tmp.22 = 1
        
          or_end_1:
            if !tmp.22 jump end_if_2
            tmp.24 = - 1
            exit(tmp.24)
        
          end_if_2:
            tmp.25 = int_and_dbl.150[0]
            tmp.26 = sign_extend tmp.25
            tmp.28 = - 115
            tmp.27 = tmp.26 != tmp.28
            if tmp.27 jump or_true_4
            tmp.31 = int_and_dbl.150[8]
            tmp.32 = tmp.31 != 222222.25D
            if tmp.32 jump or_true_4
            tmp.30 = 0
            jump or_end_5
        
          or_true_4:
            tmp.30 = 1
        
          or_end_5:
            if !tmp.30 jump end_if_6
            tmp.33 = - 2
            exit(tmp.33)
        
          end_if_6:
            tmp.34 = &two_arrs.151
            tmp.35 = sign_extend 0
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=8)
            tmp.37 = *tmp.36
            tmp.38 = tmp.37 != 66.75D
            if tmp.38 jump or_true_8
            tmp.41 = &two_arrs.151
            tmp.42 = sign_extend 1
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=8)
            tmp.44 = *tmp.43
            tmp.46 = - 4294967300L
            tmp.45 = tmp.44 != tmp.46
            if tmp.45 jump or_true_8
            tmp.40 = 0
            jump or_end_9
        
          or_true_8:
            tmp.40 = 1
        
          or_end_9:
            if !tmp.40 jump end_if_10
            tmp.47 = - 3
            exit(tmp.47)
        
          end_if_10:
            tmp.48 = &big_union.153
            tmp.49 = sign_extend 0
            tmp.50 = add_ptr(tmp.48, index=tmp.49, scale=16)
            tmp.51 = sign_extend 0
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=8)
            tmp.53 = *tmp.52
            tmp.55 = - 0.2D
            tmp.54 = tmp.53 == tmp.55
            if !tmp.54 jump and_false_12
            tmp.58 = &big_union.153
            tmp.59 = sign_extend 0
            tmp.60 = add_ptr(tmp.58, index=tmp.59, scale=16)
            tmp.61 = sign_extend 1
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=8)
            tmp.63 = *tmp.62
            tmp.65 = - 0.3D
            tmp.64 = tmp.63 == tmp.65
            if !tmp.64 jump and_false_12
            tmp.57 = 1
            jump and_end_13
        
          and_false_12:
            tmp.57 = 0
        
          and_end_13:
            if !tmp.57 jump and_false_14
            tmp.68 = &big_union.153
            tmp.69 = sign_extend 1
            tmp.70 = add_ptr(tmp.68, index=tmp.69, scale=16)
            tmp.71 = sign_extend 0
            tmp.72 = add_ptr(tmp.70, index=tmp.71, scale=8)
            tmp.73 = *tmp.72
            tmp.74 = tmp.73 == 200000000000000D
            if !tmp.74 jump and_false_14
            tmp.67 = 1
            jump and_end_15
        
          and_false_14:
            tmp.67 = 0
        
          and_end_15:
            if !tmp.67 jump and_false_16
            tmp.77 = &big_union.153
            tmp.78 = sign_extend 1
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=16)
            tmp.80 = sign_extend 1
            tmp.81 = add_ptr(tmp.79, index=tmp.80, scale=8)
            tmp.82 = *tmp.81
            tmp.83 = tmp.82 == 500000000000000D
            if !tmp.83 jump and_false_16
            tmp.76 = 1
            jump and_end_17
        
          and_false_16:
            tmp.76 = 0
        
          and_end_17:
            tmp.84 = ! tmp.76
            if !tmp.84 jump end_if_18
            tmp.85 = - 4
            exit(tmp.85)
        
          end_if_18:
            tmp.86 = oin.154[0]
            tmp.88 = - 9876.5D
            tmp.87 = tmp.86 != tmp.88
            if !tmp.87 jump end_if_20
            tmp.89 = - 5
            exit(tmp.89)
        
          end_if_20:
            result.155[0] = 'A'
            result.155[1] = 'B'
            result.155[2] = 'C'
            result.155[3] = 'D'
            result.155[4] = 'E'
            result.155[5] = 'F'
            result.155[6] = 'G'
            result.155[7] = 'H'
            result.155[8] = 'I'
            result.155[9] = 'J'
            result.155[10] = 'K'
            result.155[11] = 'L'
            result.155[12] = 'M'
            result.155[13] = 'N'
            result.155[14] = 'O'
            result.155[15] = 'P'
            result.155[16] = 'Q'
            result.155[17] = '\0'
            return result.155
            return 0
        }
        global function return_struct_with_union() { 
            tmp.90 = - 8765
            result.156[0] = tmp.90
            result.156[4] = 'd'
            result.156[5] = 'o'
            result.156[6] = 'n'
            result.156[7] = 'e'
            result.156[8] = '\0'
            return result.156
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_union_retvals_client() {
    let src = r#"
        int strcmp(char* s1, char* s2);
        void exit(int status);
        void *malloc(unsigned long size);
        union one_double {
            double d1;
            double d2;
        };
        struct has_union_with_double {
            union one_double member;
        };
        union has_struct_with_double {
            struct has_union_with_double s;
            double arr[1];
        };
        union one_int {
            double d;
            char c;
        };
        union one_int_nested {
            union one_int oi;
            union one_double od;
        };
        union char_int_mixed {
            char arr[7];
            union char_int_mixed* union_ptr;
            unsigned int ui;
        };
        union char_int_short {
            char c;
            int i;
        };
        struct has_union {
            unsigned int i;
            union char_int_short u;
        };
        union has_struct_with_ints {
            double d;
            struct has_union s;
            unsigned long ul;
        };
        union two_doubles {
            double arr[2];
            double single;
        };
        union has_xmm_union {
            union one_double u;
            union two_doubles u2;
        };
        struct dbl_struct {
            union one_double member1;
            double member2;
        };
        union has_dbl_struct {
            struct dbl_struct member1;
        };
        union char_arr {
            char arr[11];
            int i;
        };
        union two_arrs {
            double dbl_arr[2];
            long long_arr[2];
        };
        union two_eightbyte_has_struct {
            int arr[3];
            struct dbl_struct member1;
        };
        struct char_first_eightbyte {
            char c;
            double d;
        };
        struct int_second_eightbyte {
            double d;
            int i;
        };
        union two_structs {
            struct char_first_eightbyte member1;
            struct int_second_eightbyte member2;
        };
        struct nine_bytes {
            int i;
            char arr[5];
        };
        union has_nine_byte_struct {
            char c;
            long l;
            struct nine_bytes s;
        };
        union uneven {
            char arr[5];
            unsigned char uc;
        };
        struct has_uneven_union {
            int i;
            union uneven u;
        };
        union has_other_unions {
            union uneven u;
            union two_doubles d;
            union has_nine_byte_struct n;
        };
        union union_array {
            union one_int u_arr[2];
        };
        union uneven_union_array {
            union uneven u_arr[2];
        };
        struct small {
            char arr[3];
            signed char sc;
        };
        union has_small_struct_array {
            struct small arr[3];
        };
        union gp_and_xmm {
            double d_arr[2];
            char c;
        };
        union scalar_and_struct {
            long* ptr;
            struct char_first_eightbyte cfe;
        };
        struct has_two_unions {
            union char_int_mixed member1;
            union one_double member2;
        };
        union small_struct_arr_and_dbl {
            struct small arr[2];
            union two_doubles d;
        };
        union xmm_and_gp {
            double d;
            struct int_second_eightbyte ise;
        };
        union xmm_and_gp_nested {
            union xmm_and_gp member1;
            double arr[2];
            union two_doubles d;
        };
        union lotsa_doubles {
            double arr[3];
            int i;
        };
        union lotsa_chars {
            char more_chars[18];
            char fewer_chars[5];
        };
        struct large {
            int i;
            double d;
            char arr[10];
        };
        union contains_large_struct {
            int i;
            unsigned long ul;
            struct large l;
        };
        union contains_union_array {
            union gp_and_xmm arr[2];
        };
        int test_one_double(union one_double u);
        int test_has_union_with_double(struct has_union_with_double s);
        int test_has_struct_with_double(union has_struct_with_double u);
        int test_one_int(union one_int u);
        int test_one_int_nested(union one_int_nested u);
        int test_char_int_mixed(union char_int_mixed u);
        int test_has_union(struct has_union s);
        int test_has_struct_with_ints(union has_struct_with_ints u);
        int test_two_doubles(union two_doubles u);
        int test_has_xmm_union(union has_xmm_union u);
        int test_dbl_struct(struct dbl_struct s);
        int test_has_dbl_struct(union has_dbl_struct u);
        int test_char_arr(union char_arr u);
        int test_two_arrs(union two_arrs u);
        int test_two_eightbyte_has_struct(union two_eightbyte_has_struct u);
        int test_two_structs(union two_structs u);
        int test_has_nine_byte_struct(union has_nine_byte_struct u);
        int test_has_uneven_union(struct has_uneven_union s);
        int test_has_other_unions(union has_other_unions u);
        int test_union_array(union union_array u);
        int test_uneven_union_array(union uneven_union_array u);
        int test_has_small_struct_array(union has_small_struct_array u);
        int test_gp_and_xmm(union gp_and_xmm u);
        int test_scalar_and_struct(union scalar_and_struct u);
        int test_has_two_unions(struct has_two_unions s);
        int test_small_struct_arr_and_dbl(union small_struct_arr_and_dbl u);
        int test_xmm_and_gp(union xmm_and_gp u);
        int test_xmm_and_gp_nested(union xmm_and_gp_nested u);
        int test_lotsa_doubles(union lotsa_doubles u);
        int test_lotsa_chars(union lotsa_chars u);
        int test_contains_large_struct(union contains_large_struct u);
        int test_contains_union_array(union contains_union_array u);
        int pass_unions_and_structs(int i1, int i2, struct has_union one_gp_struct,
            double d1, union two_doubles two_xmm, union one_int one_gp, int i3, int i4,
            int i5);
        int pass_gp_union_in_memory(union two_doubles two_xmm,
            struct has_union one_gp_struct, int i1, int i2, int i3,
            int i4, int i5, int i6, union one_int one_gp);
        int pass_xmm_union_in_memory(double d1, double d2, union two_doubles two_xmm,
            union two_doubles two_xmm_copy, double d3, double d4,
            union two_doubles two_xmm_2);
        int pass_borderline_union(int i1, int i2, int i3, int i4, int i5,
            union char_arr two_gp);
        int pass_borderline_xmm_union(union two_doubles two_xmm, double d1, double d2,
            double d3, double d4, double d5, union two_doubles two_xmm_2);
        int pass_mixed_reg_in_memory(double d1, double d2, double d3, double d4,
            int i1, int i2, int i3, int i4, int i5, int i6,
            union gp_and_xmm mixed_regs);
        int pass_uneven_union_in_memory(int i1, int i2, int i3, int i4, int i5,
            union gp_and_xmm mixed_regs, union one_int one_gp, union uneven uneven);
        int pass_in_mem_first(union lotsa_doubles mem, union gp_and_xmm mixed_regs,
            union char_arr two_gp, struct has_union one_gp_struct);
        union one_double return_one_double(void);
        union one_int_nested return_one_int_nested(void);
        union has_dbl_struct return_has_dbl_struct(void);
        union two_arrs return_two_arrs(void);
        union scalar_and_struct return_scalar_and_struct(void);
        union xmm_and_gp return_xmm_and_gp(void);
        union contains_union_array return_contains_union_array(void);
        union lotsa_chars pass_params_and_return_in_mem(int i1,
            union scalar_and_struct int_and_dbl, union two_arrs two_arrs, int i2,
            union contains_union_array big_union, union one_int_nested oin);
        struct has_uneven_union return_struct_with_union(void);
        
        int main(void) {
            union one_double od = return_one_double();
            if (!(od.d1 == 245.5 && od.d2 == 245.5)) {
                return 1;
            }
            union one_int_nested oin = return_one_int_nested();
            if (oin.oi.d != -9876.5) {
                return 2;
            }
            union has_dbl_struct two_xmm = return_has_dbl_struct();
            if (!(two_xmm.member1.member1.d1 == 1234.5 && two_xmm.member1.member2 == 6789.)) {
                return 3;
            }
            union two_arrs two_arrs = return_two_arrs();
            if (two_arrs.dbl_arr[0] != 66.75 || two_arrs.long_arr[1] != -4294967300l) {
                return 4;
            }
            union scalar_and_struct int_and_dbl = return_scalar_and_struct();
            if (int_and_dbl.cfe.c != -115 || int_and_dbl.cfe.d != 222222.25) {
                return 5;
            }
            union xmm_and_gp dbl_and_int = return_xmm_and_gp();
            if (dbl_and_int.d != -50000.125 || dbl_and_int.ise.d != -50000.125
                || dbl_and_int.ise.i != -3000) {
                return 6;
            }
            union contains_union_array big_union = return_contains_union_array();
            if (!(big_union.arr[0].d_arr[0] == -2000e-4 && big_union.arr[0].d_arr[1] == -3000e-4
                && big_union.arr[1].d_arr[0] == 20000e10 && big_union.arr[1].d_arr[1] == 5000e11)) {
                return 7;
            }
            union lotsa_chars chars_union = pass_params_and_return_in_mem(1,
                int_and_dbl, two_arrs, 25, big_union, oin);
            if (strcmp(chars_union.more_chars, "ABCDEFGHIJKLMNOPQ") != 0) {
                return 8;
            }
            struct has_uneven_union s = return_struct_with_union();
            if (s.i != -8765 || strcmp(s.u.arr, "done") != 0) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = return_one_double()
            od.142 = tmp.0
            tmp.1 = od.142[0]
            tmp.2 = tmp.1 == 245.5D
            if !tmp.2 jump and_false_0
            tmp.5 = od.142[0]
            tmp.6 = tmp.5 == 245.5D
            if !tmp.6 jump and_false_0
            tmp.4 = 1
            jump and_end_1
        
          and_false_0:
            tmp.4 = 0
        
          and_end_1:
            tmp.7 = ! tmp.4
            if !tmp.7 jump end_if_2
            return 1
        
          end_if_2:
            tmp.8 = return_one_int_nested()
            oin.143 = tmp.8
            tmp.9 = oin.143[0]
            tmp.11 = - 9876.5D
            tmp.10 = tmp.9 != tmp.11
            if !tmp.10 jump end_if_4
            return 2
        
          end_if_4:
            tmp.12 = return_has_dbl_struct()
            two_xmm.144 = tmp.12
            tmp.13 = two_xmm.144[0]
            tmp.14 = tmp.13 == 1234.5D
            if !tmp.14 jump and_false_6
            tmp.17 = two_xmm.144[8]
            tmp.18 = tmp.17 == 6789D
            if !tmp.18 jump and_false_6
            tmp.16 = 1
            jump and_end_7
        
          and_false_6:
            tmp.16 = 0
        
          and_end_7:
            tmp.19 = ! tmp.16
            if !tmp.19 jump end_if_8
            return 3
        
          end_if_8:
            tmp.20 = return_two_arrs()
            two_arrs.145 = tmp.20
            tmp.21 = &two_arrs.145
            tmp.22 = sign_extend 0
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=8)
            tmp.24 = *tmp.23
            tmp.25 = tmp.24 != 66.75D
            if tmp.25 jump or_true_10
            tmp.28 = &two_arrs.145
            tmp.29 = sign_extend 1
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=8)
            tmp.31 = *tmp.30
            tmp.33 = - 4294967300L
            tmp.32 = tmp.31 != tmp.33
            if tmp.32 jump or_true_10
            tmp.27 = 0
            jump or_end_11
        
          or_true_10:
            tmp.27 = 1
        
          or_end_11:
            if !tmp.27 jump end_if_12
            return 4
        
          end_if_12:
            tmp.34 = return_scalar_and_struct()
            int_and_dbl.146 = tmp.34
            tmp.35 = int_and_dbl.146[0]
            tmp.36 = sign_extend tmp.35
            tmp.38 = - 115
            tmp.37 = tmp.36 != tmp.38
            if tmp.37 jump or_true_14
            tmp.41 = int_and_dbl.146[8]
            tmp.42 = tmp.41 != 222222.25D
            if tmp.42 jump or_true_14
            tmp.40 = 0
            jump or_end_15
        
          or_true_14:
            tmp.40 = 1
        
          or_end_15:
            if !tmp.40 jump end_if_16
            return 5
        
          end_if_16:
            tmp.43 = return_xmm_and_gp()
            dbl_and_int.147 = tmp.43
            tmp.44 = dbl_and_int.147[0]
            tmp.46 = - 50000.125D
            tmp.45 = tmp.44 != tmp.46
            if tmp.45 jump or_true_18
            tmp.49 = dbl_and_int.147[0]
            tmp.51 = - 50000.125D
            tmp.50 = tmp.49 != tmp.51
            if tmp.50 jump or_true_18
            tmp.48 = 0
            jump or_end_19
        
          or_true_18:
            tmp.48 = 1
        
          or_end_19:
            if tmp.48 jump or_true_20
            tmp.54 = dbl_and_int.147[8]
            tmp.56 = - 3000
            tmp.55 = tmp.54 != tmp.56
            if tmp.55 jump or_true_20
            tmp.53 = 0
            jump or_end_21
        
          or_true_20:
            tmp.53 = 1
        
          or_end_21:
            if !tmp.53 jump end_if_22
            return 6
        
          end_if_22:
            tmp.57 = return_contains_union_array()
            big_union.148 = tmp.57
            tmp.58 = &big_union.148
            tmp.59 = sign_extend 0
            tmp.60 = add_ptr(tmp.58, index=tmp.59, scale=16)
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=8)
            tmp.63 = *tmp.62
            tmp.65 = - 0.2D
            tmp.64 = tmp.63 == tmp.65
            if !tmp.64 jump and_false_24
            tmp.68 = &big_union.148
            tmp.69 = sign_extend 0
            tmp.70 = add_ptr(tmp.68, index=tmp.69, scale=16)
            tmp.71 = sign_extend 1
            tmp.72 = add_ptr(tmp.70, index=tmp.71, scale=8)
            tmp.73 = *tmp.72
            tmp.75 = - 0.3D
            tmp.74 = tmp.73 == tmp.75
            if !tmp.74 jump and_false_24
            tmp.67 = 1
            jump and_end_25
        
          and_false_24:
            tmp.67 = 0
        
          and_end_25:
            if !tmp.67 jump and_false_26
            tmp.78 = &big_union.148
            tmp.79 = sign_extend 1
            tmp.80 = add_ptr(tmp.78, index=tmp.79, scale=16)
            tmp.81 = sign_extend 0
            tmp.82 = add_ptr(tmp.80, index=tmp.81, scale=8)
            tmp.83 = *tmp.82
            tmp.84 = tmp.83 == 200000000000000D
            if !tmp.84 jump and_false_26
            tmp.77 = 1
            jump and_end_27
        
          and_false_26:
            tmp.77 = 0
        
          and_end_27:
            if !tmp.77 jump and_false_28
            tmp.87 = &big_union.148
            tmp.88 = sign_extend 1
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=16)
            tmp.90 = sign_extend 1
            tmp.91 = add_ptr(tmp.89, index=tmp.90, scale=8)
            tmp.92 = *tmp.91
            tmp.93 = tmp.92 == 500000000000000D
            if !tmp.93 jump and_false_28
            tmp.86 = 1
            jump and_end_29
        
          and_false_28:
            tmp.86 = 0
        
          and_end_29:
            tmp.94 = ! tmp.86
            if !tmp.94 jump end_if_30
            return 7
        
          end_if_30:
            tmp.95 = pass_params_and_return_in_mem(1, int_and_dbl.146, two_arrs.145, 25, big_union.148, oin.143)
            chars_union.149 = tmp.95
            tmp.96 = &chars_union.149
            tmp.97 = &string.0
            tmp.98 = strcmp(tmp.96, tmp.97)
            tmp.99 = tmp.98 != 0
            if !tmp.99 jump end_if_32
            return 8
        
          end_if_32:
            tmp.100 = return_struct_with_union()
            s.150 = tmp.100
            tmp.101 = s.150[0]
            tmp.103 = - 8765
            tmp.102 = tmp.101 != tmp.103
            if tmp.102 jump or_true_34
            tmp.106 = &s.150
            tmp.106 = add_ptr(tmp.106, index=4L, scale=1)
            tmp.107 = &string.1
            tmp.108 = strcmp(tmp.106, tmp.107)
            tmp.109 = tmp.108 != 0
            if tmp.109 jump or_true_34
            tmp.105 = 0
            jump or_end_35
        
          or_true_34:
            tmp.105 = 1
        
          or_end_35:
            if !tmp.105 jump end_if_36
            return 9
        
          end_if_36:
            return 0
            return 0
        }
        constant string.0: Array(18,Char) = "ABCDEFGHIJKLMNOPQ\\0"
        constant string.1: Array(5,Char) = "done\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_member_access_nested_union_access() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *malloc(unsigned long size);
        union simple {
            int i;
            long l;
            char c;
            unsigned char uc_arr[3];
        };
        union has_union {
            double d;
            union simple u;
            union simple *u_ptr;
        };
        struct simple_struct {
            long l;
            double d;
            unsigned int u;
        };
        union has_struct {
            long l;
            struct simple_struct s;
        };
        struct struct_with_union {
            union simple u;
            unsigned long ul;
        };
        union complex_union {
            double d_arr[2];
            struct struct_with_union s;
            union has_union *u_ptr;
        };
        
        int test_auto_dot(void) {
            union has_union x;
            x.u.l = 200000u;
            if (x.u.i != 200000) {
                return 0;
            }
            union has_struct y;
            y.s.l = -5555l;
            y.s.d = 10.0;
            y.s.u = 100;
            if (y.l != -5555l) {
                return 0;
            }
            union complex_union z;
            z.s.u.i = 12345;
            z.s.ul = 0;
            if (z.s.u.c != 57) {
                return 0;
            }
            if (z.d_arr[1]) {
                return 0;
            }
            unsigned int *some_int_ptr = &y.s.u;
            union simple *some_union_ptr = &z.s.u;
            if (*some_int_ptr != 100 || (*some_union_ptr).i != 12345) {
                return 0;
            }
            return 1;
        }
        int test_static_dot(void) {
            static union has_union x;
            x.u.l = 200000u;
            if (x.u.i != 200000) {
                return 0;
            }
            static union has_struct y;
            y.s.l = -5555l;
            y.s.d = 10.0;
            y.s.u = 100;
            if (y.l != -5555l) {
                return 0;
            }
            static union complex_union z;
            z.s.u.i = 12345;
            z.s.ul = 0;
            if (z.s.u.c != 57) {
                return 0;
            }
            if (z.d_arr[1]) {
                return 0;
            }
            return 1;
        }
        int test_auto_arrow(void) {
            union simple inner = {100};
            union has_union outer;
            union has_union *outer_ptr = &outer;
            outer_ptr->u_ptr = &inner;
            if (outer_ptr->u_ptr->i != 100) {
                return 0;
            }
            outer_ptr->u_ptr->l = -10;
            if (outer_ptr->u_ptr->c != -10 || outer_ptr->u_ptr->i != -10 || outer_ptr->u_ptr->l != -10) {
                return 0;
            }
            if (outer_ptr->u_ptr->uc_arr[0] != 246 || outer_ptr->u_ptr->uc_arr[1] != 255 || outer_ptr->u_ptr->uc_arr[2] != 255) {
                return 0;
            }
            return 1;
        }
        int test_static_arrow(void) {
            static union simple inner = {100};
            static union has_union outer;
            static union has_union *outer_ptr;
            outer_ptr = &outer;
            outer_ptr->u_ptr = &inner;
            if (outer_ptr->u_ptr->i != 100) {
                return 0;
            }
            outer_ptr->u_ptr->l = -10;
            if (outer_ptr->u_ptr->c != -10 || outer_ptr->u_ptr->i != -10 || outer_ptr->u_ptr->l != -10) {
                return 0;
            }
            if (outer_ptr->u_ptr->uc_arr[0] != 246 || outer_ptr->u_ptr->uc_arr[1] != 255 || outer_ptr->u_ptr->uc_arr[2] != 255) {
                return 0;
            }
            return 1;
        }
        int test_array_of_unions(void) {
            union has_union arr[3];
            arr[0].u.l = -10000;
            arr[1].u.i = 200;
            arr[2].u.c = -120;
            if (arr[0].u.l != -10000 || arr[1].u.c != -56 || arr[2].u.uc_arr[0] != 136) {
                return 0;
            }
            return 1;
        }
        int test_array_of_union_pointers(void) {
            union has_union *ptr_arr[3];
            for (int i = 0; i < 3; i = i + 1) {
                ptr_arr[i] = calloc(1, sizeof(union has_union));
                ptr_arr[i]->u_ptr = calloc(1, sizeof (union simple));
                ptr_arr[i]->u_ptr->l = i;
            }
            if (ptr_arr[0]->u_ptr->l != 0 || ptr_arr[1]->u_ptr->l != 1 || ptr_arr[2]->u_ptr->l != 2) {
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
            if (!test_array_of_unions()) {
                return 5;
            }
            if (!test_array_of_union_pointers()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_auto_dot() { 
            tmp.0 = zero_extend 200000U
            x.9[0] = tmp.0
            tmp.1 = x.9[0]
            tmp.2 = x.9[0]
            tmp.3 = tmp.2 != 200000
            if !tmp.3 jump end_if_0
            return 0
        
          end_if_0:
            tmp.4 = - 5555L
            y.10[0] = tmp.4
            tmp.5 = y.10[0]
            y.10[8] = 10D
            tmp.6 = y.10[8]
            tmp.7 = 100
            y.10[16] = tmp.7
            tmp.8 = y.10[16]
            tmp.9 = y.10[0]
            tmp.11 = - 5555L
            tmp.10 = tmp.9 != tmp.11
            if !tmp.10 jump end_if_2
            return 0
        
          end_if_2:
            z.11[0] = 12345
            tmp.12 = z.11[0]
            tmp.13 = sign_extend 0
            z.11[8] = tmp.13
            tmp.14 = z.11[8]
            tmp.15 = z.11[0]
            tmp.16 = sign_extend tmp.15
            tmp.17 = tmp.16 != 57
            if !tmp.17 jump end_if_4
            return 0
        
          end_if_4:
            tmp.18 = &z.11
            tmp.19 = sign_extend 1
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=8)
            tmp.21 = *tmp.20
            tmp.22 = tmp.21 != 0D
            if !tmp.22 jump end_if_6
            return 0
        
          end_if_6:
            tmp.23 = &y.10
            tmp.23 = add_ptr(tmp.23, index=16L, scale=1)
            some_int_ptr.12 = tmp.23
            tmp.24 = &z.11
            some_union_ptr.13 = tmp.24
            tmp.25 = *some_int_ptr.12
            tmp.27 = 100
            tmp.26 = tmp.25 != tmp.27
            if tmp.26 jump or_true_8
            tmp.30 = *some_union_ptr.13
            tmp.31 = tmp.30 != 12345
            if tmp.31 jump or_true_8
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
        global function test_static_dot() { 
            tmp.32 = zero_extend 200000U
            x.14[0] = tmp.32
            tmp.33 = x.14[0]
            tmp.34 = x.14[0]
            tmp.35 = tmp.34 != 200000
            if !tmp.35 jump end_if_12
            return 0
        
          end_if_12:
            tmp.36 = - 5555L
            y.15[0] = tmp.36
            tmp.37 = y.15[0]
            y.15[8] = 10D
            tmp.38 = y.15[8]
            tmp.39 = 100
            y.15[16] = tmp.39
            tmp.40 = y.15[16]
            tmp.41 = y.15[0]
            tmp.43 = - 5555L
            tmp.42 = tmp.41 != tmp.43
            if !tmp.42 jump end_if_14
            return 0
        
          end_if_14:
            z.16[0] = 12345
            tmp.44 = z.16[0]
            tmp.45 = sign_extend 0
            z.16[8] = tmp.45
            tmp.46 = z.16[8]
            tmp.47 = z.16[0]
            tmp.48 = sign_extend tmp.47
            tmp.49 = tmp.48 != 57
            if !tmp.49 jump end_if_16
            return 0
        
          end_if_16:
            tmp.50 = &z.16
            tmp.51 = sign_extend 1
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=8)
            tmp.53 = *tmp.52
            tmp.54 = tmp.53 != 0D
            if !tmp.54 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function test_auto_arrow() { 
            inner.17[0] = 100
            tmp.55 = &outer.18
            outer_ptr.19 = tmp.55
            tmp.56 = &inner.17
            *outer_ptr.19 = tmp.56
            tmp.57 = *outer_ptr.19
            tmp.58 = *outer_ptr.19
            tmp.59 = *tmp.58
            tmp.60 = tmp.59 != 100
            if !tmp.60 jump end_if_20
            return 0
        
          end_if_20:
            tmp.61 = *outer_ptr.19
            tmp.62 = - 10
            tmp.63 = sign_extend tmp.62
            *tmp.61 = tmp.63
            tmp.64 = *tmp.61
            tmp.65 = *outer_ptr.19
            tmp.66 = *tmp.65
            tmp.67 = sign_extend tmp.66
            tmp.69 = - 10
            tmp.68 = tmp.67 != tmp.69
            if tmp.68 jump or_true_22
            tmp.72 = *outer_ptr.19
            tmp.73 = *tmp.72
            tmp.75 = - 10
            tmp.74 = tmp.73 != tmp.75
            if tmp.74 jump or_true_22
            tmp.71 = 0
            jump or_end_23
        
          or_true_22:
            tmp.71 = 1
        
          or_end_23:
            if tmp.71 jump or_true_24
            tmp.78 = *outer_ptr.19
            tmp.79 = *tmp.78
            tmp.81 = - 10
            tmp.82 = sign_extend tmp.81
            tmp.80 = tmp.79 != tmp.82
            if tmp.80 jump or_true_24
            tmp.77 = 0
            jump or_end_25
        
          or_true_24:
            tmp.77 = 1
        
          or_end_25:
            if !tmp.77 jump end_if_26
            return 0
        
          end_if_26:
            tmp.83 = *outer_ptr.19
            tmp.84 = sign_extend 0
            tmp.85 = add_ptr(tmp.83, index=tmp.84, scale=1)
            tmp.86 = *tmp.85
            tmp.87 = zero_extend tmp.86
            tmp.88 = tmp.87 != 246
            if tmp.88 jump or_true_28
            tmp.91 = *outer_ptr.19
            tmp.92 = sign_extend 1
            tmp.93 = add_ptr(tmp.91, index=tmp.92, scale=1)
            tmp.94 = *tmp.93
            tmp.95 = zero_extend tmp.94
            tmp.96 = tmp.95 != 255
            if tmp.96 jump or_true_28
            tmp.90 = 0
            jump or_end_29
        
          or_true_28:
            tmp.90 = 1
        
          or_end_29:
            if tmp.90 jump or_true_30
            tmp.99 = *outer_ptr.19
            tmp.100 = sign_extend 2
            tmp.101 = add_ptr(tmp.99, index=tmp.100, scale=1)
            tmp.102 = *tmp.101
            tmp.103 = zero_extend tmp.102
            tmp.104 = tmp.103 != 255
            if tmp.104 jump or_true_30
            tmp.98 = 0
            jump or_end_31
        
          or_true_30:
            tmp.98 = 1
        
          or_end_31:
            if !tmp.98 jump end_if_32
            return 0
        
          end_if_32:
            return 1
            return 0
        }
        global function test_static_arrow() { 
            tmp.105 = &outer.21
            outer_ptr.22 = tmp.105
            tmp.106 = &inner.20
            *outer_ptr.22 = tmp.106
            tmp.107 = *outer_ptr.22
            tmp.108 = *outer_ptr.22
            tmp.109 = *tmp.108
            tmp.110 = tmp.109 != 100
            if !tmp.110 jump end_if_34
            return 0
        
          end_if_34:
            tmp.111 = *outer_ptr.22
            tmp.112 = - 10
            tmp.113 = sign_extend tmp.112
            *tmp.111 = tmp.113
            tmp.114 = *tmp.111
            tmp.115 = *outer_ptr.22
            tmp.116 = *tmp.115
            tmp.117 = sign_extend tmp.116
            tmp.119 = - 10
            tmp.118 = tmp.117 != tmp.119
            if tmp.118 jump or_true_36
            tmp.122 = *outer_ptr.22
            tmp.123 = *tmp.122
            tmp.125 = - 10
            tmp.124 = tmp.123 != tmp.125
            if tmp.124 jump or_true_36
            tmp.121 = 0
            jump or_end_37
        
          or_true_36:
            tmp.121 = 1
        
          or_end_37:
            if tmp.121 jump or_true_38
            tmp.128 = *outer_ptr.22
            tmp.129 = *tmp.128
            tmp.131 = - 10
            tmp.132 = sign_extend tmp.131
            tmp.130 = tmp.129 != tmp.132
            if tmp.130 jump or_true_38
            tmp.127 = 0
            jump or_end_39
        
          or_true_38:
            tmp.127 = 1
        
          or_end_39:
            if !tmp.127 jump end_if_40
            return 0
        
          end_if_40:
            tmp.133 = *outer_ptr.22
            tmp.134 = sign_extend 0
            tmp.135 = add_ptr(tmp.133, index=tmp.134, scale=1)
            tmp.136 = *tmp.135
            tmp.137 = zero_extend tmp.136
            tmp.138 = tmp.137 != 246
            if tmp.138 jump or_true_42
            tmp.141 = *outer_ptr.22
            tmp.142 = sign_extend 1
            tmp.143 = add_ptr(tmp.141, index=tmp.142, scale=1)
            tmp.144 = *tmp.143
            tmp.145 = zero_extend tmp.144
            tmp.146 = tmp.145 != 255
            if tmp.146 jump or_true_42
            tmp.140 = 0
            jump or_end_43
        
          or_true_42:
            tmp.140 = 1
        
          or_end_43:
            if tmp.140 jump or_true_44
            tmp.149 = *outer_ptr.22
            tmp.150 = sign_extend 2
            tmp.151 = add_ptr(tmp.149, index=tmp.150, scale=1)
            tmp.152 = *tmp.151
            tmp.153 = zero_extend tmp.152
            tmp.154 = tmp.153 != 255
            if tmp.154 jump or_true_44
            tmp.148 = 0
            jump or_end_45
        
          or_true_44:
            tmp.148 = 1
        
          or_end_45:
            if !tmp.148 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function test_array_of_unions() { 
            tmp.155 = &arr.23
            tmp.156 = sign_extend 0
            tmp.157 = add_ptr(tmp.155, index=tmp.156, scale=8)
            tmp.158 = - 10000
            tmp.159 = sign_extend tmp.158
            *tmp.157 = tmp.159
            tmp.160 = *tmp.157
            tmp.161 = &arr.23
            tmp.162 = sign_extend 1
            tmp.163 = add_ptr(tmp.161, index=tmp.162, scale=8)
            *tmp.163 = 200
            tmp.164 = *tmp.163
            tmp.165 = &arr.23
            tmp.166 = sign_extend 2
            tmp.167 = add_ptr(tmp.165, index=tmp.166, scale=8)
            tmp.168 = - 120
            tmp.169 = truncate tmp.168
            *tmp.167 = tmp.169
            tmp.170 = *tmp.167
            tmp.171 = &arr.23
            tmp.172 = sign_extend 0
            tmp.173 = add_ptr(tmp.171, index=tmp.172, scale=8)
            tmp.174 = *tmp.173
            tmp.176 = - 10000
            tmp.177 = sign_extend tmp.176
            tmp.175 = tmp.174 != tmp.177
            if tmp.175 jump or_true_48
            tmp.180 = &arr.23
            tmp.181 = sign_extend 1
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=8)
            tmp.183 = *tmp.182
            tmp.184 = sign_extend tmp.183
            tmp.186 = - 56
            tmp.185 = tmp.184 != tmp.186
            if tmp.185 jump or_true_48
            tmp.179 = 0
            jump or_end_49
        
          or_true_48:
            tmp.179 = 1
        
          or_end_49:
            if tmp.179 jump or_true_50
            tmp.189 = &arr.23
            tmp.190 = sign_extend 2
            tmp.191 = add_ptr(tmp.189, index=tmp.190, scale=8)
            tmp.192 = sign_extend 0
            tmp.193 = add_ptr(tmp.191, index=tmp.192, scale=1)
            tmp.194 = *tmp.193
            tmp.195 = zero_extend tmp.194
            tmp.196 = tmp.195 != 136
            if tmp.196 jump or_true_50
            tmp.188 = 0
            jump or_end_51
        
          or_true_50:
            tmp.188 = 1
        
          or_end_51:
            if !tmp.188 jump end_if_52
            return 0
        
          end_if_52:
            return 1
            return 0
        }
        global function test_array_of_union_pointers() { 
            i.25 = 0
        
          start_loop_0:
            tmp.197 = i.25 < 3
            if !tmp.197 jump break_loop_0
            tmp.198 = &ptr_arr.24
            tmp.199 = sign_extend i.25
            tmp.200 = add_ptr(tmp.198, index=tmp.199, scale=8)
            tmp.201 = sign_extend 1
            tmp.202 = calloc(tmp.201, 8UL)
            tmp.203 = tmp.202
            *tmp.200 = tmp.203
            tmp.204 = *tmp.200
            tmp.205 = &ptr_arr.24
            tmp.206 = sign_extend i.25
            tmp.207 = add_ptr(tmp.205, index=tmp.206, scale=8)
            tmp.208 = *tmp.207
            tmp.209 = sign_extend 1
            tmp.210 = calloc(tmp.209, 8UL)
            tmp.211 = tmp.210
            *tmp.208 = tmp.211
            tmp.212 = *tmp.208
            tmp.213 = &ptr_arr.24
            tmp.214 = sign_extend i.25
            tmp.215 = add_ptr(tmp.213, index=tmp.214, scale=8)
            tmp.216 = *tmp.215
            tmp.217 = *tmp.216
            tmp.218 = sign_extend i.25
            *tmp.217 = tmp.218
            tmp.219 = *tmp.217
        
          continue_loop_0:
            tmp.220 = i.25 + 1
            i.25 = tmp.220
            jump start_loop_0
        
          break_loop_0:
            tmp.221 = &ptr_arr.24
            tmp.222 = sign_extend 0
            tmp.223 = add_ptr(tmp.221, index=tmp.222, scale=8)
            tmp.224 = *tmp.223
            tmp.225 = *tmp.224
            tmp.226 = *tmp.225
            tmp.228 = sign_extend 0
            tmp.227 = tmp.226 != tmp.228
            if tmp.227 jump or_true_54
            tmp.231 = &ptr_arr.24
            tmp.232 = sign_extend 1
            tmp.233 = add_ptr(tmp.231, index=tmp.232, scale=8)
            tmp.234 = *tmp.233
            tmp.235 = *tmp.234
            tmp.236 = *tmp.235
            tmp.238 = sign_extend 1
            tmp.237 = tmp.236 != tmp.238
            if tmp.237 jump or_true_54
            tmp.230 = 0
            jump or_end_55
        
          or_true_54:
            tmp.230 = 1
        
          or_end_55:
            if tmp.230 jump or_true_56
            tmp.241 = &ptr_arr.24
            tmp.242 = sign_extend 2
            tmp.243 = add_ptr(tmp.241, index=tmp.242, scale=8)
            tmp.244 = *tmp.243
            tmp.245 = *tmp.244
            tmp.246 = *tmp.245
            tmp.248 = sign_extend 2
            tmp.247 = tmp.246 != tmp.248
            if tmp.247 jump or_true_56
            tmp.240 = 0
            jump or_end_57
        
          or_true_56:
            tmp.240 = 1
        
          or_end_57:
            if !tmp.240 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function main() { 
            tmp.249 = test_auto_dot()
            tmp.250 = ! tmp.249
            if !tmp.250 jump end_if_60
            return 1
        
          end_if_60:
            tmp.251 = test_static_dot()
            tmp.252 = ! tmp.251
            if !tmp.252 jump end_if_62
            return 2
        
          end_if_62:
            tmp.253 = test_auto_arrow()
            tmp.254 = ! tmp.253
            if !tmp.254 jump end_if_64
            return 3
        
          end_if_64:
            tmp.255 = test_static_arrow()
            tmp.256 = ! tmp.255
            if !tmp.256 jump end_if_66
            return 4
        
          end_if_66:
            tmp.257 = test_array_of_unions()
            tmp.258 = ! tmp.257
            if !tmp.258 jump end_if_68
            return 5
        
          end_if_68:
            tmp.259 = test_array_of_union_pointers()
            tmp.260 = ! tmp.259
            if !tmp.260 jump end_if_70
            return 6
        
          end_if_70:
            return 0
            return 0
        }
        static inner.20: Union(simple.3) = [ 100, zero[4]]
        static outer.21: Union(has_union.4) = zero[8]
        static outer_ptr.22: Pointer(Union(has_union.4)) = zero[8]
        static x.14: Union(has_union.4) = zero[8]
        static y.15: Union(has_struct.6) = zero[24]
        static z.16: Union(complex_union.8) = zero[16]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_member_access_static_union_access() {
    let src = r#"
        
        union u {
            unsigned long l;
            double d;
            char arr[8];
        };
        static union u my_union = { 18446744073709551615UL };
        static union u* union_ptr = 0;
        int main(void) {
            union_ptr = &my_union;
            if (my_union.l != 18446744073709551615UL) {
                return 1;
            }
            for (int i = 0; i < 8; i = i + 1) {
                if (my_union.arr[i] != -1) {
                    return 2;
                }
            }
            union_ptr->d = -1.0;
            if (union_ptr->l != 13830554455654793216ul) {
                return 3;
            }
            for (int i = 0; i < 6; i = i + 1) {
                if (my_union.arr[i]) {
                    return 4;
                }
            }
            if (union_ptr->arr[6] != -16) {
                return 5;
            }
            if (union_ptr->arr[7] != -65) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &my_union
            union_ptr = tmp.0
            tmp.1 = my_union[0]
            tmp.2 = tmp.1 != 18446744073709551615UL
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            i.1 = 0
        
          start_loop_0:
            tmp.3 = i.1 < 8
            if !tmp.3 jump break_loop_0
            tmp.4 = &my_union
            tmp.5 = sign_extend i.1
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=1)
            tmp.7 = *tmp.6
            tmp.8 = sign_extend tmp.7
            tmp.10 = - 1
            tmp.9 = tmp.8 != tmp.10
            if !tmp.9 jump end_if_2
            return 2
        
          end_if_2:
        
          continue_loop_0:
            tmp.11 = i.1 + 1
            i.1 = tmp.11
            jump start_loop_0
        
          break_loop_0:
            tmp.12 = - 1D
            *union_ptr = tmp.12
            tmp.13 = *union_ptr
            tmp.14 = *union_ptr
            tmp.15 = tmp.14 != 13830554455654793216UL
            if !tmp.15 jump end_if_4
            return 3
        
          end_if_4:
            i.2 = 0
        
          start_loop_1:
            tmp.16 = i.2 < 6
            if !tmp.16 jump break_loop_1
            tmp.17 = &my_union
            tmp.18 = sign_extend i.2
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=1)
            tmp.20 = *tmp.19
            if !tmp.20 jump end_if_6
            return 4
        
          end_if_6:
        
          continue_loop_1:
            tmp.21 = i.2 + 1
            i.2 = tmp.21
            jump start_loop_1
        
          break_loop_1:
            tmp.22 = sign_extend 6
            tmp.23 = add_ptr(union_ptr, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            tmp.25 = sign_extend tmp.24
            tmp.27 = - 16
            tmp.26 = tmp.25 != tmp.27
            if !tmp.26 jump end_if_8
            return 5
        
          end_if_8:
            tmp.28 = sign_extend 7
            tmp.29 = add_ptr(union_ptr, index=tmp.28, scale=1)
            tmp.30 = *tmp.29
            tmp.31 = sign_extend tmp.30
            tmp.33 = - 65
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static my_union: Union(u.0) = 18446744073709551615UL
        static union_ptr: Pointer(Union(u.0)) = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_member_access_union_init_and_member_access() {
    let src = r#"
        union u {
            double d;
            long l;
            unsigned long ul;
            char c;
        };
        int main(void) {
            union u x = {20};
            if (x.d != 20.0) {
                return 1;
            }
            union u *ptr = &x;
            ptr->l = -1l;
            if (ptr->l != -1l) {
                return 2;
            }
            if (ptr->ul != 18446744073709551615UL) {
                return 3;
            }
            if (x.c != -1) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = int_to_double 20
            x.1[0] = tmp.0
            tmp.1 = x.1[0]
            tmp.2 = tmp.1 != 20D
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = &x.1
            ptr.2 = tmp.3
            tmp.4 = - 1L
            *ptr.2 = tmp.4
            tmp.5 = *ptr.2
            tmp.6 = *ptr.2
            tmp.8 = - 1L
            tmp.7 = tmp.6 != tmp.8
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = *ptr.2
            tmp.10 = tmp.9 != 18446744073709551615UL
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = x.1[0]
            tmp.12 = sign_extend tmp.11
            tmp.14 = - 1
            tmp.13 = tmp.12 != tmp.14
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_member_access_union_temp_lifetime() {
    let src = r#"
        struct has_char_array {
            char arr[8];
        };
        union has_array {
            long l;
            struct has_char_array s;
        };
        int get_flag(void) {
            static int flag = 0;
            flag = !flag;
            return flag;
        }
        int main(void) {
            union has_array union1 = {9876543210l};
            union has_array union2 = {1234567890l};
            if ((get_flag() ? union1 : union2).s.arr[0] != -22) {
                return 1;
            }
            if ((get_flag() ? union1 : union2).s.arr[0] != -46) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_flag() { 
            tmp.0 = ! flag.2
            flag.2 = tmp.0
            return flag.2
            return 0
        }
        global function main() { 
            union1.3[0] = 9876543210L
            union2.4[0] = 1234567890L
            tmp.1 = get_flag()
            if !tmp.1 jump else_1
            tmp.2 = union1.3
            jump end_if_0
        
          else_1:
            tmp.2 = union2.4
        
          end_if_0:
            tmp.3 = &tmp.2
            tmp.4 = sign_extend 0
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=1)
            tmp.6 = *tmp.5
            tmp.7 = sign_extend tmp.6
            tmp.9 = - 22
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_2
            return 1
        
          end_if_2:
            tmp.10 = get_flag()
            if !tmp.10 jump else_5
            tmp.11 = union1.3
            jump end_if_4
        
          else_5:
            tmp.11 = union2.4
        
          end_if_4:
            tmp.12 = &tmp.11
            tmp.13 = sign_extend 0
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = sign_extend tmp.15
            tmp.18 = - 46
            tmp.17 = tmp.16 != tmp.18
            if !tmp.17 jump end_if_6
            return 2
        
          end_if_6:
            return 0
            return 0
        }
        static flag.2: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

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
            i.2[0] = tmp.0
            i.2[4] = 100000U
            o.3[0] = 9223372036854775810UL
            tmp.1 = &i.2
            o.3[8] = tmp.1
            o.3[16] = 100
            tmp.2 = - 80
            tmp.3 = truncate tmp.2
            o.3[20] = tmp.3
            o.3[24] = 4294967295U
            tmp.4 = i.2[0]
            tmp.5 = sign_extend tmp.4
            tmp.7 = o.3[0]
            tmp.6 = tmp.5 | tmp.7
            tmp.8 = tmp.6 != 9223372036854775907UL
            if !tmp.8 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = o.3[16]
            tmp.10 = tmp.9
            tmp.12 = i.2[4]
            tmp.11 = tmp.10 ^ tmp.12
            tmp.13 = tmp.11 != 100036U
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            tmp.14 = o.3[8]
            tmp.15 = *tmp.14
            tmp.16 = sign_extend tmp.15
            tmp.18 = o.3[20]
            tmp.19 = sign_extend tmp.18
            tmp.17 = tmp.16 & tmp.19
            tmp.20 = tmp.17 != 32
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
            tmp.21 = o.3[0]
            tmp.22 = tmp.21 >> 26
            tmp.23 = tmp.22 != 137438953472UL
            if !tmp.23 jump end_if_6
            return 4
        
          end_if_6:
            o.3[16] = 12
            tmp.24 = o.3[16]
            tmp.25 = i.2[0]
            tmp.26 = sign_extend tmp.25
            tmp.28 = o.3[16]
            tmp.27 = tmp.26 << tmp.28
            tmp.29 = tmp.27 != 397312
            if !tmp.29 jump end_if_8
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
            si.4[0] = 150D
            tmp.2 = - 12
            tmp.3 = truncate tmp.2
            si.4[8] = tmp.3
            tmp.4 = &i.2
            si.4[16] = tmp.4
            o.5[0] = 18446744073709551615UL
            tmp.5 = &si.4
            o.5[8] = tmp.5
            o.5[16] = si.4
            tmp.6 = - 2000000000000000000000D
            o.5[40] = tmp.6
            tmp.7 = truncate 120
            o.5[48] = tmp.7
            tmp.8 = sign_extend 0
            o.5[56] = tmp.8
            tmp.9 = int_to_double 0
            o.5[64] = tmp.9
            tmp.10 = truncate 0
            o.5[72] = tmp.10
            tmp.11 = sign_extend 0
            o.5[80] = tmp.11
            tmp.12 = int_to_double 1
            o.5[88] = tmp.12
            tmp.13 = truncate 1
            o.5[96] = tmp.13
            tmp.14 = &i2.3
            o.5[104] = tmp.14
            o.5[112] = 2000
            tmp.15 = si.4[0]
            tmp.17 = int_to_double 10
            tmp.16 = tmp.15 + tmp.17
            si.4[0] = tmp.16
            tmp.18 = si.4[0]
            tmp.19 = si.4[0]
            tmp.21 = int_to_double 160
            tmp.20 = tmp.19 != tmp.21
            if !tmp.20 jump end_if_0
            return 1
        
          end_if_0:
            tmp.22 = &o.5
            tmp.22 = add_ptr(tmp.22, index=16L, scale=1)
            tmp.23 = sign_extend 0
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=24)
            tmp.25 = add_ptr(tmp.24, index=8L, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = sign_extend tmp.26
            tmp.28 = tmp.27 - 460
            tmp.29 = truncate tmp.28
            *tmp.25 = tmp.29
            tmp.30 = *tmp.25
            tmp.31 = &o.5
            tmp.31 = add_ptr(tmp.31, index=16L, scale=1)
            tmp.32 = sign_extend 0
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=24)
            tmp.34 = add_ptr(tmp.33, index=8L, scale=1)
            tmp.35 = *tmp.34
            tmp.36 = sign_extend tmp.35
            tmp.37 = tmp.36 != 40
            if !tmp.37 jump end_if_2
            return 2
        
          end_if_2:
            tmp.38 = &o.5
            tmp.38 = add_ptr(tmp.38, index=16L, scale=1)
            tmp.39 = sign_extend 1
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=24)
            tmp.41 = *tmp.40
            tmp.43 = - 4
            tmp.44 = int_to_double tmp.43
            tmp.42 = tmp.41 * tmp.44
            *tmp.40 = tmp.42
            tmp.45 = *tmp.40
            tmp.46 = &o.5
            tmp.46 = add_ptr(tmp.46, index=16L, scale=1)
            tmp.47 = sign_extend 1
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=24)
            tmp.49 = *tmp.48
            tmp.50 = tmp.49 != 8000000000000000000000D
            if !tmp.50 jump end_if_4
            return 4
        
          end_if_4:
            tmp.51 = o.5[8]
            tmp.52 = *tmp.51
            tmp.54 = int_to_double 5
            tmp.53 = tmp.52 / tmp.54
            *tmp.51 = tmp.53
            tmp.55 = *tmp.51
            tmp.56 = si.4[0]
            tmp.58 = int_to_double 32
            tmp.57 = tmp.56 != tmp.58
            if !tmp.57 jump end_if_6
            return 5
        
          end_if_6:
            tmp.59 = &o.5
            tmp.60 = *tmp.59
            tmp.62 = o.5[112]
            tmp.63 = sign_extend tmp.62
            tmp.61 = tmp.60 % tmp.63
            *tmp.59 = tmp.61
            tmp.64 = *tmp.59
            tmp.65 = o.5[0]
            tmp.67 = sign_extend 1615
            tmp.66 = tmp.65 != tmp.67
            if !tmp.66 jump end_if_8
            return 6
        
          end_if_8:
            tmp.68 = &o.5
            tmp.68 = add_ptr(tmp.68, index=16L, scale=1)
            o.5[8] = tmp.68
            tmp.69 = o.5[8]
            tmp.70 = o.5[8]
            tmp.72 = sign_extend 3
            tmp.71 = add_ptr(tmp.70, index=tmp.72, scale=24)
            o.5[8] = tmp.71
            tmp.73 = o.5[8]
            tmp.74 = *tmp.73
            tmp.76 = int_to_double 1
            tmp.75 = tmp.74 != tmp.76
            if !tmp.75 jump end_if_10
            return 7
        
          end_if_10:
            tmp.77 = o.5[8]
            tmp.78 = add_ptr(tmp.77, index=16L, scale=1)
            tmp.79 = *tmp.78
            tmp.80 = *tmp.79
            tmp.82 = - 2
            tmp.81 = tmp.80 != tmp.82
            if !tmp.81 jump end_if_12
            return 8
        
          end_if_12:
            tmp.83 = o.5[8]
            tmp.85 = zero_extend 1U
            tmp.86 = - tmp.85
            tmp.84 = add_ptr(tmp.83, index=tmp.86, scale=24)
            o.5[8] = tmp.84
            tmp.87 = o.5[8]
            tmp.88 = o.5[8]
            tmp.89 = *tmp.88
            if tmp.89 jump or_true_14
            tmp.92 = o.5[8]
            tmp.93 = add_ptr(tmp.92, index=8L, scale=1)
            tmp.94 = *tmp.93
            if tmp.94 jump or_true_14
            tmp.91 = 0
            jump or_end_15
        
          or_true_14:
            tmp.91 = 1
        
          or_end_15:
            if tmp.91 jump or_true_16
            tmp.97 = o.5[8]
            tmp.98 = add_ptr(tmp.97, index=16L, scale=1)
            tmp.99 = *tmp.98
            if tmp.99 jump or_true_16
            tmp.96 = 0
            jump or_end_17
        
          or_true_16:
            tmp.96 = 1
        
          or_end_17:
            if !tmp.96 jump end_if_18
            return 9
        
          end_if_18:
            tmp.100 = si.4[0]
            tmp.102 = int_to_double 32
            tmp.101 = tmp.100 != tmp.102
            if tmp.101 jump or_true_20
            tmp.105 = si.4[8]
            tmp.106 = sign_extend tmp.105
            tmp.108 = - 12
            tmp.107 = tmp.106 != tmp.108
            if tmp.107 jump or_true_20
            tmp.104 = 0
            jump or_end_21
        
          or_true_20:
            tmp.104 = 1
        
          or_end_21:
            if tmp.104 jump or_true_22
            tmp.111 = si.4[16]
            tmp.113 = &i.2
            tmp.112 = tmp.111 != tmp.113
            if tmp.112 jump or_true_22
            tmp.110 = 0
            jump or_end_23
        
          or_true_22:
            tmp.110 = 1
        
          or_end_23:
            if !tmp.110 jump end_if_24
            return 10
        
          end_if_24:
            tmp.114 = o.5[0]
            tmp.116 = sign_extend 1615
            tmp.115 = tmp.114 != tmp.116
            if !tmp.115 jump end_if_26
            return 11
        
          end_if_26:
            tmp.117 = o.5[8]
            tmp.119 = &o.5
            tmp.119 = add_ptr(tmp.119, index=16L, scale=1)
            tmp.120 = sign_extend 2
            tmp.121 = add_ptr(tmp.119, index=tmp.120, scale=24)
            tmp.118 = tmp.117 != tmp.121
            if !tmp.118 jump end_if_28
            return 12
        
          end_if_28:
            tmp.122 = &o.5
            tmp.122 = add_ptr(tmp.122, index=16L, scale=1)
            tmp.123 = sign_extend 0
            tmp.124 = add_ptr(tmp.122, index=tmp.123, scale=24)
            tmp.125 = *tmp.124
            tmp.126 = tmp.125 != 150D
            if tmp.126 jump or_true_30
            tmp.129 = &o.5
            tmp.129 = add_ptr(tmp.129, index=16L, scale=1)
            tmp.130 = sign_extend 0
            tmp.131 = add_ptr(tmp.129, index=tmp.130, scale=24)
            tmp.132 = add_ptr(tmp.131, index=8L, scale=1)
            tmp.133 = *tmp.132
            tmp.134 = sign_extend tmp.133
            tmp.135 = tmp.134 != 40
            if tmp.135 jump or_true_30
            tmp.128 = 0
            jump or_end_31
        
          or_true_30:
            tmp.128 = 1
        
          or_end_31:
            if tmp.128 jump or_true_32
            tmp.138 = &o.5
            tmp.138 = add_ptr(tmp.138, index=16L, scale=1)
            tmp.139 = sign_extend 0
            tmp.140 = add_ptr(tmp.138, index=tmp.139, scale=24)
            tmp.141 = add_ptr(tmp.140, index=16L, scale=1)
            tmp.142 = *tmp.141
            tmp.144 = &i.2
            tmp.143 = tmp.142 != tmp.144
            if tmp.143 jump or_true_32
            tmp.137 = 0
            jump or_end_33
        
          or_true_32:
            tmp.137 = 1
        
          or_end_33:
            if !tmp.137 jump end_if_34
            return 13
        
          end_if_34:
            tmp.145 = &o.5
            tmp.145 = add_ptr(tmp.145, index=16L, scale=1)
            tmp.146 = sign_extend 1
            tmp.147 = add_ptr(tmp.145, index=tmp.146, scale=24)
            tmp.148 = *tmp.147
            tmp.149 = tmp.148 != 8000000000000000000000D
            if tmp.149 jump or_true_36
            tmp.152 = &o.5
            tmp.152 = add_ptr(tmp.152, index=16L, scale=1)
            tmp.153 = sign_extend 1
            tmp.154 = add_ptr(tmp.152, index=tmp.153, scale=24)
            tmp.155 = add_ptr(tmp.154, index=8L, scale=1)
            tmp.156 = *tmp.155
            tmp.157 = sign_extend tmp.156
            tmp.158 = tmp.157 != 120
            if tmp.158 jump or_true_36
            tmp.151 = 0
            jump or_end_37
        
          or_true_36:
            tmp.151 = 1
        
          or_end_37:
            if tmp.151 jump or_true_38
            tmp.161 = &o.5
            tmp.161 = add_ptr(tmp.161, index=16L, scale=1)
            tmp.162 = sign_extend 1
            tmp.163 = add_ptr(tmp.161, index=tmp.162, scale=24)
            tmp.164 = add_ptr(tmp.163, index=16L, scale=1)
            tmp.165 = *tmp.164
            if tmp.165 jump or_true_38
            tmp.160 = 0
            jump or_end_39
        
          or_true_38:
            tmp.160 = 1
        
          or_end_39:
            if !tmp.160 jump end_if_40
            return 14
        
          end_if_40:
            tmp.166 = &o.5
            tmp.166 = add_ptr(tmp.166, index=16L, scale=1)
            tmp.167 = sign_extend 2
            tmp.168 = add_ptr(tmp.166, index=tmp.167, scale=24)
            tmp.169 = *tmp.168
            if tmp.169 jump or_true_42
            tmp.172 = &o.5
            tmp.172 = add_ptr(tmp.172, index=16L, scale=1)
            tmp.173 = sign_extend 2
            tmp.174 = add_ptr(tmp.172, index=tmp.173, scale=24)
            tmp.175 = add_ptr(tmp.174, index=8L, scale=1)
            tmp.176 = *tmp.175
            if tmp.176 jump or_true_42
            tmp.171 = 0
            jump or_end_43
        
          or_true_42:
            tmp.171 = 1
        
          or_end_43:
            if tmp.171 jump or_true_44
            tmp.179 = &o.5
            tmp.179 = add_ptr(tmp.179, index=16L, scale=1)
            tmp.180 = sign_extend 2
            tmp.181 = add_ptr(tmp.179, index=tmp.180, scale=24)
            tmp.182 = add_ptr(tmp.181, index=16L, scale=1)
            tmp.183 = *tmp.182
            if tmp.183 jump or_true_44
            tmp.178 = 0
            jump or_end_45
        
          or_true_44:
            tmp.178 = 1
        
          or_end_45:
            if !tmp.178 jump end_if_46
            return 15
        
          end_if_46:
            tmp.184 = &o.5
            tmp.184 = add_ptr(tmp.184, index=16L, scale=1)
            tmp.185 = sign_extend 3
            tmp.186 = add_ptr(tmp.184, index=tmp.185, scale=24)
            tmp.187 = *tmp.186
            tmp.189 = int_to_double 1
            tmp.188 = tmp.187 != tmp.189
            if tmp.188 jump or_true_48
            tmp.192 = &o.5
            tmp.192 = add_ptr(tmp.192, index=16L, scale=1)
            tmp.193 = sign_extend 3
            tmp.194 = add_ptr(tmp.192, index=tmp.193, scale=24)
            tmp.195 = add_ptr(tmp.194, index=8L, scale=1)
            tmp.196 = *tmp.195
            tmp.197 = sign_extend tmp.196
            tmp.198 = tmp.197 != 1
            if tmp.198 jump or_true_48
            tmp.191 = 0
            jump or_end_49
        
          or_true_48:
            tmp.191 = 1
        
          or_end_49:
            if tmp.191 jump or_true_50
            tmp.201 = &o.5
            tmp.201 = add_ptr(tmp.201, index=16L, scale=1)
            tmp.202 = sign_extend 3
            tmp.203 = add_ptr(tmp.201, index=tmp.202, scale=24)
            tmp.204 = add_ptr(tmp.203, index=16L, scale=1)
            tmp.205 = *tmp.204
            tmp.207 = &i2.3
            tmp.206 = tmp.205 != tmp.207
            if tmp.206 jump or_true_50
            tmp.200 = 0
            jump or_end_51
        
          or_true_50:
            tmp.200 = 1
        
          or_end_51:
            if !tmp.200 jump end_if_52
            return 16
        
          end_if_52:
            tmp.208 = o.5[112]
            tmp.209 = tmp.208 != 2000
            if !tmp.209 jump end_if_54
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
            my_struct.4[0] = 9223372036854775900UL
            tmp.0 = sign_extend 3
            tmp.1 = calloc(tmp.0, 8UL)
            tmp.2 = tmp.1
            my_struct.4[8] = tmp.2
            tmp.3 = - 1000
            my_struct.4[16] = tmp.3
            tmp.4 = - 2000
            my_struct.4[20] = tmp.4
            tmp.5 = - 3000
            my_struct.4[24] = tmp.5
            tmp.6 = &my_struct.4
            my_struct_ptr.5 = tmp.6
            tmp.7 = my_struct.4[0]
            tmp.8 = inc tmp.7
            my_struct.4[0] = tmp.8
            tmp.9 = tmp.8 != 9223372036854775901UL
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.10 = my_struct.4[8]
            tmp.11 = sign_extend 0
            tmp.12 = add_ptr(tmp.10, index=tmp.11, scale=8)
            tmp.13 = add_ptr(tmp.12, index=4L, scale=1)
            tmp.14 = *tmp.13
            tmp.15 = dec tmp.14
            *tmp.13 = tmp.15
            tmp.16 = tmp.15 != 4294967295U
            if !tmp.16 jump end_if_2
            return 2
        
          end_if_2:
            tmp.17 = my_struct.4[8]
            tmp.18 = *tmp.17
            tmp.19 = tmp.18
            tmp.20 = inc tmp.18
            *tmp.17 = tmp.20
            if !tmp.19 jump end_if_4
            return 3
        
          end_if_4:
            tmp.21 = add_ptr(my_struct_ptr.5, index=16L, scale=1)
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
            tmp.29 = *my_struct_ptr.5
            tmp.30 = tmp.29 != 9223372036854775901UL
            if !tmp.30 jump end_if_8
            return 5
        
          end_if_8:
            tmp.31 = my_struct.4[8]
            tmp.32 = *tmp.31
            tmp.33 = sign_extend tmp.32
            tmp.34 = tmp.33 != 1
            if !tmp.34 jump end_if_10
            return 6
        
          end_if_10:
            tmp.35 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = add_ptr(tmp.36, index=4L, scale=1)
            tmp.38 = *tmp.37
            tmp.39 = tmp.38 != 4294967295U
            if !tmp.39 jump end_if_12
            return 7
        
          end_if_12:
            tmp.40 = add_ptr(my_struct_ptr.5, index=16L, scale=1)
            tmp.41 = sign_extend 1
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=4)
            tmp.43 = *tmp.42
            tmp.45 = - 2001
            tmp.44 = tmp.43 != tmp.45
            if !tmp.44 jump end_if_14
            return 8
        
          end_if_14:
            tmp.46 = add_ptr(my_struct_ptr.5, index=16L, scale=1)
            tmp.47 = sign_extend 0
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=4)
            tmp.49 = *tmp.48
            tmp.51 = - 1000
            tmp.50 = tmp.49 != tmp.51
            if tmp.50 jump or_true_16
            tmp.54 = add_ptr(my_struct_ptr.5, index=16L, scale=1)
            tmp.55 = sign_extend 2
            tmp.56 = add_ptr(tmp.54, index=tmp.55, scale=4)
            tmp.57 = *tmp.56
            tmp.59 = - 3000
            tmp.58 = tmp.57 != tmp.59
            if tmp.58 jump or_true_16
            tmp.53 = 0
            jump or_end_17
        
          or_true_16:
            tmp.53 = 1
        
          or_end_17:
            if !tmp.53 jump end_if_18
            return 9
        
          end_if_18:
            tmp.60 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.61 = *tmp.60
            tmp.62 = sign_extend 1
            tmp.63 = add_ptr(tmp.61, index=tmp.62, scale=8)
            tmp.64 = - 1
            tmp.65 = truncate tmp.64
            *tmp.63 = tmp.65
            tmp.66 = *tmp.63
            tmp.67 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.68 = *tmp.67
            tmp.69 = sign_extend 1
            tmp.70 = add_ptr(tmp.68, index=tmp.69, scale=8)
            tmp.71 = add_ptr(tmp.70, index=4L, scale=1)
            *tmp.71 = 1U
            tmp.72 = *tmp.71
            tmp.73 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.74 = *tmp.73
            tmp.75 = sign_extend 2
            tmp.76 = add_ptr(tmp.74, index=tmp.75, scale=8)
            tmp.77 = truncate 88
            *tmp.76 = tmp.77
            tmp.78 = *tmp.76
            tmp.79 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.80 = *tmp.79
            tmp.81 = sign_extend 2
            tmp.82 = add_ptr(tmp.80, index=tmp.81, scale=8)
            tmp.83 = add_ptr(tmp.82, index=4L, scale=1)
            *tmp.83 = 100000U
            tmp.84 = *tmp.83
            tmp.85 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.86 = *tmp.85
            tmp.87 = add_ptr(tmp.86, index=1L, scale=8)
            *tmp.85 = tmp.87
            tmp.88 = *tmp.87
            tmp.89 = tmp.88
            tmp.90 = dec tmp.88
            *tmp.87 = tmp.90
            tmp.91 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.92 = *tmp.91
            tmp.93 = tmp.92
            tmp.94 = add_ptr(tmp.92, index=1L, scale=8)
            *tmp.91 = tmp.94
            tmp.95 = add_ptr(tmp.93, index=4L, scale=1)
            tmp.96 = *tmp.95
            tmp.97 = tmp.96
            tmp.98 = inc tmp.96
            *tmp.95 = tmp.98
            tmp.99 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.100 = *tmp.99
            tmp.101 = - 2
            tmp.102 = sign_extend tmp.101
            tmp.103 = add_ptr(tmp.100, index=tmp.102, scale=8)
            tmp.104 = *tmp.103
            tmp.105 = sign_extend tmp.104
            tmp.106 = tmp.105 != 1
            if tmp.106 jump or_true_20
            tmp.109 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.110 = *tmp.109
            tmp.111 = - 2
            tmp.112 = sign_extend tmp.111
            tmp.113 = add_ptr(tmp.110, index=tmp.112, scale=8)
            tmp.114 = add_ptr(tmp.113, index=4L, scale=1)
            tmp.115 = *tmp.114
            tmp.116 = tmp.115 != 4294967295U
            if tmp.116 jump or_true_20
            tmp.108 = 0
            jump or_end_21
        
          or_true_20:
            tmp.108 = 1
        
          or_end_21:
            if !tmp.108 jump end_if_22
            return 10
        
          end_if_22:
            tmp.117 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.118 = *tmp.117
            tmp.119 = - 1
            tmp.120 = sign_extend tmp.119
            tmp.121 = add_ptr(tmp.118, index=tmp.120, scale=8)
            tmp.122 = *tmp.121
            tmp.123 = sign_extend tmp.122
            tmp.125 = - 2
            tmp.124 = tmp.123 != tmp.125
            if !tmp.124 jump end_if_24
            return 11
        
          end_if_24:
            tmp.126 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.127 = *tmp.126
            tmp.128 = - 1
            tmp.129 = sign_extend tmp.128
            tmp.130 = add_ptr(tmp.127, index=tmp.129, scale=8)
            tmp.131 = add_ptr(tmp.130, index=4L, scale=1)
            tmp.132 = *tmp.131
            tmp.134 = 2
            tmp.133 = tmp.132 != tmp.134
            if !tmp.133 jump end_if_26
            return 12
        
          end_if_26:
            tmp.135 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.136 = *tmp.135
            tmp.137 = sign_extend 0
            tmp.138 = add_ptr(tmp.136, index=tmp.137, scale=8)
            tmp.139 = *tmp.138
            tmp.140 = sign_extend tmp.139
            tmp.141 = tmp.140 != 88
            if tmp.141 jump or_true_28
            tmp.144 = add_ptr(my_struct_ptr.5, index=8L, scale=1)
            tmp.145 = *tmp.144
            tmp.146 = sign_extend 0
            tmp.147 = add_ptr(tmp.145, index=tmp.146, scale=8)
            tmp.148 = add_ptr(tmp.147, index=4L, scale=1)
            tmp.149 = *tmp.148
            tmp.150 = tmp.149 != 100000U
            if tmp.150 jump or_true_28
            tmp.143 = 0
            jump or_end_29
        
          or_true_28:
            tmp.143 = 1
        
          or_end_29:
            if !tmp.143 jump end_if_30
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
            x.1[0] = 10
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
            my_struct.1[0] = 1
            my_struct.1[4] = 2
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
            tmp.3 = my_struct.4[0]
            my_struct.4[8] = 30D
            tmp.4 = my_struct.4[8]
            tmp.5 = my_struct.4[0]
            tmp.7 = my_struct.4[8]
            tmp.6 = tmp.5 + tmp.7
            tmp.8 = double_to_int tmp.6
            result.2 = tmp.8
            jump break_switch_0
        
          switch_0_case__2:
            my_struct.4[0] = 11D
            tmp.9 = my_struct.4[0]
            my_struct.4[8] = 12D
            tmp.10 = my_struct.4[8]
            tmp.11 = my_struct.4[0]
            tmp.13 = my_struct.4[8]
            tmp.12 = tmp.11 + tmp.13
            tmp.14 = double_to_int tmp.12
            result.2 = tmp.14
            jump break_switch_0
        
          switch_0_default_3:
            my_struct.4[0] = 0D
            tmp.15 = my_struct.4[0]
            my_struct.4[8] = 0D
            tmp.16 = my_struct.4[8]
            tmp.17 = my_struct.4[0]
            tmp.19 = my_struct.4[8]
            tmp.18 = tmp.17 + tmp.19
            tmp.20 = double_to_int tmp.18
            result.2 = tmp.20
        
          break_switch_0:
            return result.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_cast_union_to_void() {
    let src = r#"
        union u {
            long l;
            double d;
        };
        int main(void) {
            union u x = {1000};
            (void) x;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 1000
            x.1[0] = tmp.0
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_decl_shadows_decl() {
    let src = r#"
        int main(void) {
            struct tag;
            struct tag *struct_ptr = 0;
            {
                union tag;
                union tag *union_ptr = 0;
                if (struct_ptr || union_ptr) {
                    return 1;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 0
            struct_ptr.1 = tmp.0
            tmp.1 = sign_extend 0
            union_ptr.3 = tmp.1
            if struct_ptr.1 jump or_true_0
            if union_ptr.3 jump or_true_0
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
fn test_valid_extra_credit_semantic_analysis_incomplete_union_types() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        int puts(char *s);
        union never_used;
        union never_used incomplete_fun(union never_used x);
        int test_block_scope_forward_decl(void) {
            union u;
            union u* u_ptr = 0;
            union u {
                long x;
                char y;
            };
            union u val = { -100000000l };
            u_ptr = &val;
            if (u_ptr->x != -100000000l || u_ptr->y != 0) {
                return 0;
            }
            return 1;
        }
        union opaque_union;
        union opaque_union* use_union_pointers(union opaque_union* param) {
            if (param == 0) {
                puts("null pointer");
            }
            return 0;
        }
        int test_use_incomplete_union_pointers(void) {
            union opaque_union* ptr1 = calloc(1, 4);
            union opaque_union* ptr2 = calloc(1, 4);
            char* ptr1_bytes = (char*)ptr1;
            if (ptr1_bytes[0] || ptr1_bytes[1]) {
                return 0;
            }
            if (ptr1 == 0 || ptr2 == 0 || ptr1 == ptr2) {
                return 0;
            }
            static int flse = 0;
            union opaque_union* ptr3 = flse ? ptr1 : ptr2;
            if (ptr3 != ptr2) {
                return 0;
            }
            if (use_union_pointers(ptr3)) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_block_scope_forward_decl()) {
                return 1;
            }
            if (!test_use_incomplete_union_pointers()) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_block_scope_forward_decl() { 
            tmp.0 = sign_extend 0
            u_ptr.6 = tmp.0
            tmp.1 = - 100000000L
            val.8[0] = tmp.1
            tmp.2 = &val.8
            u_ptr.6 = tmp.2
            tmp.3 = *u_ptr.6
            tmp.5 = - 100000000L
            tmp.4 = tmp.3 != tmp.5
            if tmp.4 jump or_true_0
            tmp.8 = *u_ptr.6
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 0
            if tmp.10 jump or_true_0
            tmp.7 = 0
            jump or_end_1
        
          or_true_0:
            tmp.7 = 1
        
          or_end_1:
            if !tmp.7 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function use_union_pointers(param.10) { 
            tmp.12 = sign_extend 0
            tmp.11 = param.10 == tmp.12
            if !tmp.11 jump end_if_4
            tmp.13 = &string.0
            tmp.14 = puts(tmp.13)
        
          end_if_4:
            tmp.15 = sign_extend 0
            return tmp.15
            return 0
        }
        global function test_use_incomplete_union_pointers() { 
            tmp.16 = sign_extend 1
            tmp.17 = sign_extend 4
            tmp.18 = calloc(tmp.16, tmp.17)
            tmp.19 = tmp.18
            ptr1.11 = tmp.19
            tmp.20 = sign_extend 1
            tmp.21 = sign_extend 4
            tmp.22 = calloc(tmp.20, tmp.21)
            tmp.23 = tmp.22
            ptr2.12 = tmp.23
            tmp.24 = ptr1.11
            ptr1_bytes.13 = tmp.24
            tmp.25 = sign_extend 0
            tmp.26 = add_ptr(ptr1_bytes.13, index=tmp.25, scale=1)
            tmp.27 = *tmp.26
            if tmp.27 jump or_true_6
            tmp.30 = sign_extend 1
            tmp.31 = add_ptr(ptr1_bytes.13, index=tmp.30, scale=1)
            tmp.32 = *tmp.31
            if tmp.32 jump or_true_6
            tmp.29 = 0
            jump or_end_7
        
          or_true_6:
            tmp.29 = 1
        
          or_end_7:
            if !tmp.29 jump end_if_8
            return 0
        
          end_if_8:
            tmp.34 = sign_extend 0
            tmp.33 = ptr1.11 == tmp.34
            if tmp.33 jump or_true_10
            tmp.38 = sign_extend 0
            tmp.37 = ptr2.12 == tmp.38
            if tmp.37 jump or_true_10
            tmp.36 = 0
            jump or_end_11
        
          or_true_10:
            tmp.36 = 1
        
          or_end_11:
            if tmp.36 jump or_true_12
            tmp.41 = ptr1.11 == ptr2.12
            if tmp.41 jump or_true_12
            tmp.40 = 0
            jump or_end_13
        
          or_true_12:
            tmp.40 = 1
        
          or_end_13:
            if !tmp.40 jump end_if_14
            return 0
        
          end_if_14:
            if !flse.14 jump else_17
            tmp.42 = ptr1.11
            jump end_if_16
        
          else_17:
            tmp.42 = ptr2.12
        
          end_if_16:
            ptr3.15 = tmp.42
            tmp.43 = ptr3.15 != ptr2.12
            if !tmp.43 jump end_if_18
            return 0
        
          end_if_18:
            tmp.44 = use_union_pointers(ptr3.15)
            if !tmp.44 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function main() { 
            tmp.45 = test_block_scope_forward_decl()
            tmp.46 = ! tmp.45
            if !tmp.46 jump end_if_22
            return 1
        
          end_if_22:
            tmp.47 = test_use_incomplete_union_pointers()
            tmp.48 = ! tmp.47
            if !tmp.48 jump end_if_24
            return 2
        
          end_if_24:
            return 0
            return 0
        }
        static flse.14: Int = 0
        constant string.0: Array(13,Char) = "null pointer\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_redeclare_union() {
    let src = r#"
        
        int main(void) {
            union u {
                int a;
            };
            union u;
            union u my_union = {1};
            return my_union.a;
        }
    "#;
    let expected = r#"
        global function main() { 
            my_union.2[0] = 1
            tmp.0 = my_union.2[0]
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_struct_shadows_union() {
    let src = r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            struct s {int a; int b;};
            struct s my_struct = {12, 13};
            {
                union u;
                union u *ptr = malloc(4);
                union u {int i; unsigned int u;};
                ptr->i = 10;
                if (ptr->u != 10) {
                    return 1;
                }
                if (my_struct.b != 13) {
                    return 2;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            my_struct.2[0] = 12
            my_struct.2[4] = 13
            tmp.0 = sign_extend 4
            tmp.1 = malloc(tmp.0)
            tmp.2 = tmp.1
            ptr.4 = tmp.2
            *ptr.4 = 10
            tmp.3 = *ptr.4
            tmp.4 = *ptr.4
            tmp.6 = 10
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = my_struct.2[4]
            tmp.8 = tmp.7 != 13
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
fn test_valid_extra_credit_semantic_analysis_union_members_same_type() {
    let src = r#"
        
        union u {
            int a;
            int b;
        };
        int main(void) {
            union u my_union = {0};
            my_union.a = -1;
            if (my_union.b != -1){
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            my_union.1[0] = 0
            tmp.0 = - 1
            my_union.1[0] = tmp.0
            tmp.1 = my_union.1[0]
            tmp.2 = my_union.1[0]
            tmp.4 = - 1
            tmp.3 = tmp.2 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_union_namespace() {
    let src = r#"
        int test_shared_member_names(void) {
            union u1 {
                int a;
            };
            union u2 {
                long l;
                double a;
            };
            struct s {
                char a[2];
            };
            union u1 var1 = {10};
            union u2 var2 = {-9223372036854775807l - 1};
            struct s var3 = {{-1, -2}};
            if (var1.a != 10 || var2.a != -0.0 || var3.a[0] != -1) {
                return 0;
            }
            return 1;
        }
        int test_same_name_var_member_and_tag(void) {
            union u {
                int u;
            };
            union u u = {100};
            if (u.u != 100) {
                return 0;
            }
            return 1;
        }
        int f(void) {
            return 10;
        }
        union f {
            int f;
        };
        int test_same_name_fun_and_tag(void) {
            union f x;
            x.f = f();
            if (x.f != 10) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_shared_member_names()) {
                return 1;
            }
            if (!test_same_name_var_member_and_tag()) {
                return 2;
            }
            if (!test_same_name_fun_and_tag()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_shared_member_names() { 
            var1.3[0] = 10
            tmp.0 = - 9223372036854775807L
            tmp.2 = sign_extend 1
            tmp.1 = tmp.0 - tmp.2
            var2.4[0] = tmp.1
            tmp.3 = - 1
            tmp.4 = truncate tmp.3
            var3.5[0] = tmp.4
            tmp.5 = - 2
            tmp.6 = truncate tmp.5
            var3.5[1] = tmp.6
            tmp.7 = var1.3[0]
            tmp.8 = tmp.7 != 10
            if tmp.8 jump or_true_0
            tmp.11 = var2.4[0]
            tmp.13 = - 0D
            tmp.12 = tmp.11 != tmp.13
            if tmp.12 jump or_true_0
            tmp.10 = 0
            jump or_end_1
        
          or_true_0:
            tmp.10 = 1
        
          or_end_1:
            if tmp.10 jump or_true_2
            tmp.16 = &var3.5
            tmp.17 = sign_extend 0
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            tmp.20 = sign_extend tmp.19
            tmp.22 = - 1
            tmp.21 = tmp.20 != tmp.22
            if tmp.21 jump or_true_2
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
        global function test_same_name_var_member_and_tag() { 
            u.7[0] = 100
            tmp.23 = u.7[0]
            tmp.24 = tmp.23 != 100
            if !tmp.24 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function f() { 
            return 10
            return 0
        }
        global function test_same_name_fun_and_tag() { 
            tmp.25 = f()
            x.9[0] = tmp.25
            tmp.26 = x.9[0]
            tmp.27 = x.9[0]
            tmp.28 = tmp.27 != 10
            if !tmp.28 jump end_if_8
            return 0
        
          end_if_8:
            return 1
            return 0
        }
        global function main() { 
            tmp.29 = test_shared_member_names()
            tmp.30 = ! tmp.29
            if !tmp.30 jump end_if_10
            return 1
        
          end_if_10:
            tmp.31 = test_same_name_var_member_and_tag()
            tmp.32 = ! tmp.31
            if !tmp.32 jump end_if_12
            return 2
        
          end_if_12:
            tmp.33 = test_same_name_fun_and_tag()
            tmp.34 = ! tmp.33
            if !tmp.34 jump end_if_14
            return 3
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_union_self_pointer() {
    let src = r#"
        union self_ptr {
            union self_ptr *ptr;
            long l;
        };
        int main(void) {
            union self_ptr u = {&u};
            if (&u != u.ptr) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &u.1
            u.1[0] = tmp.0
            tmp.1 = &u.1
            tmp.3 = u.1[0]
            tmp.2 = tmp.1 != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_semantic_analysis_union_shadows_struct() {
    let src = r#"
        
        struct tag {
            int a;
            int b;
        };
        struct tag global_struct = {1, 2};
        int main(void) {
            union tag {
                int x;
                long y;
            };
            union tag local_union = {100};
            if (global_struct.a != 1) {
                return 1;
            }
            if (local_union.x != 100) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            local_union.2[0] = 100
            tmp.0 = global_struct[0]
            tmp.1 = tmp.0 != 1
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = local_union.2[0]
            tmp.3 = tmp.2 != 100
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global global_struct: Struct(tag.0) = [ 1, 2]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_size_and_offset_compare_union_pointers() {
    let src = r#"
        
        struct s {
            int i;
        };
        union u {
            char arr[3];
            double d;
            struct s my_struct;
        };
        union u my_union;
        int main(void) {
            union u* u_ptr = &my_union;
            if ((void*)u_ptr != (void*)&(u_ptr->arr)) {
                return 1;
            }
            if (!((void*)u_ptr == (void*)&(u_ptr->d))) {
                return 2;
            }
            if ((void*)&(u_ptr->my_struct) != u_ptr) {
                return 3;
            }
            if (my_union.arr != (char*)&my_union.d) {
                return 4;
            }
            if (!(&my_union.arr[0] >= (char *) &my_union.my_struct.i)) {
                return 5;
            }
            if (! ((char *) (&u_ptr->d) <= (char *) &u_ptr->my_struct)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &my_union
            u_ptr.2 = tmp.0
            tmp.1 = u_ptr.2
            tmp.3 = u_ptr.2
            tmp.2 = tmp.1 != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = u_ptr.2
            tmp.6 = u_ptr.2
            tmp.5 = tmp.4 == tmp.6
            tmp.7 = ! tmp.5
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = u_ptr.2
            tmp.10 = u_ptr.2
            tmp.9 = tmp.8 != tmp.10
            if !tmp.9 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = &my_union
            tmp.13 = &my_union
            tmp.14 = tmp.13
            tmp.12 = tmp.11 != tmp.14
            if !tmp.12 jump end_if_6
            return 4
        
          end_if_6:
            tmp.15 = &my_union
            tmp.16 = sign_extend 0
            tmp.17 = add_ptr(tmp.15, index=tmp.16, scale=1)
            tmp.19 = &my_union
            tmp.20 = tmp.19
            tmp.18 = tmp.17 >= tmp.20
            tmp.21 = ! tmp.18
            if !tmp.21 jump end_if_8
            return 5
        
          end_if_8:
            tmp.22 = u_ptr.2
            tmp.24 = u_ptr.2
            tmp.23 = tmp.22 <= tmp.24
            tmp.25 = ! tmp.23
            if !tmp.25 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static global my_union: Union(u.1) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_size_and_offset_union_sizes() {
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
        union no_padding {
            char c;
            unsigned char uc;
            signed char arr[11];
        };
        union with_padding {
            signed char arr[10];
            unsigned int ui;
        };
        union contains_array {
            union with_padding arr1[2];
            union no_padding arr[3];
        };
        union double_and_int {
            int i;
            double d;
        };
        union contains_structs {
            struct wonky x;
            struct eight_bytes y;
        };
        int main(void) {
            if (sizeof(union no_padding) != 11) {
                return 1;
            }
            if (sizeof(union with_padding) != 12) {
                return 2;
            }
            if (sizeof(union contains_array) != 36) {
                return 3;
            }
            if (sizeof(union double_and_int) != 8) {
                return 4;
            }
            if (sizeof(union contains_structs) != 20) {
                return 5;
            }
            union no_padding x = { 1 };
            union contains_array y = { {{{-1, 2}} }};
            union contains_structs* get_union_ptr(void);
            if (sizeof x != 11) {
                return 6;
            }
            if (sizeof y.arr1 != 24) {
                return 7;
            }
            if (sizeof * get_union_ptr() != 20) {
                return 8;
            }
            return 0;
        }
        union contains_structs* get_union_ptr(void) {
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 11
            tmp.0 = 11UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 12
            tmp.2 = 12UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 36
            tmp.4 = 36UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend 8
            tmp.6 = 8UL != tmp.7
            if !tmp.6 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 20
            tmp.8 = 20UL != tmp.9
            if !tmp.8 jump end_if_8
            return 5
        
          end_if_8:
            tmp.10 = truncate 1
            x.15[0] = tmp.10
            tmp.11 = - 1
            tmp.12 = truncate tmp.11
            y.16[0] = tmp.12
            tmp.13 = truncate 2
            y.16[1] = tmp.13
            y.16[2] = '\0'
            y.16[3] = '\0'
            y.16[4] = '\0'
            y.16[5] = '\0'
            y.16[6] = '\0'
            y.16[7] = '\0'
            y.16[8] = '\0'
            y.16[9] = '\0'
            y.16[12] = '\0'
            y.16[13] = '\0'
            y.16[14] = '\0'
            y.16[15] = '\0'
            y.16[16] = '\0'
            y.16[17] = '\0'
            y.16[18] = '\0'
            y.16[19] = '\0'
            y.16[20] = '\0'
            y.16[21] = '\0'
            y.16[12] = 0U
            tmp.15 = sign_extend 11
            tmp.14 = 11UL != tmp.15
            if !tmp.14 jump end_if_10
            return 6
        
          end_if_10:
            tmp.17 = sign_extend 24
            tmp.16 = 24UL != tmp.17
            if !tmp.16 jump end_if_12
            return 7
        
          end_if_12:
            tmp.19 = sign_extend 20
            tmp.18 = 20UL != tmp.19
            if !tmp.18 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        global function get_union_ptr() { 
            tmp.20 = sign_extend 0
            return tmp.20
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_union_copy_assign_to_union() {
    let src = r#"
        
        struct s {
            int a;
            int b;
        };
        union u {
            struct s str;
            long l;
            double arr[3];
        };
        int main(void) {
            union u x = { {1, 2} };
            union u y = { {0, 0} };
            y = x;
            if (y.str.a != 1) {
                return 1;
            }
            if (y.str.b != 2) {
                return 2;
            }
            x.arr[0] = -20.;
            x.arr[1] = -30.;
            x.arr[2] = -40.;
            y = x;
            if (y.arr[0] != -20.) {
                return 3;
            }
            if (y.arr[1] != -30.) {
                return 4;
            }
            if (y.arr[2] != -40.) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.2[0] = 1
            x.2[4] = 2
            y.3[0] = 0
            y.3[4] = 0
            y.3 = x.2
            tmp.0 = y.3[0]
            tmp.1 = tmp.0 != 1
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = y.3[4]
            tmp.3 = tmp.2 != 2
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = &x.2
            tmp.5 = sign_extend 0
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=8)
            tmp.7 = - 20D
            *tmp.6 = tmp.7
            tmp.8 = *tmp.6
            tmp.9 = &x.2
            tmp.10 = sign_extend 1
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=8)
            tmp.12 = - 30D
            *tmp.11 = tmp.12
            tmp.13 = *tmp.11
            tmp.14 = &x.2
            tmp.15 = sign_extend 2
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=8)
            tmp.17 = - 40D
            *tmp.16 = tmp.17
            tmp.18 = *tmp.16
            y.3 = x.2
            tmp.19 = &y.3
            tmp.20 = sign_extend 0
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=8)
            tmp.22 = *tmp.21
            tmp.24 = - 20D
            tmp.23 = tmp.22 != tmp.24
            if !tmp.23 jump end_if_4
            return 3
        
          end_if_4:
            tmp.25 = &y.3
            tmp.26 = sign_extend 1
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=8)
            tmp.28 = *tmp.27
            tmp.30 = - 30D
            tmp.29 = tmp.28 != tmp.30
            if !tmp.29 jump end_if_6
            return 4
        
          end_if_6:
            tmp.31 = &y.3
            tmp.32 = sign_extend 2
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=8)
            tmp.34 = *tmp.33
            tmp.36 = - 40D
            tmp.35 = tmp.34 != tmp.36
            if !tmp.35 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_union_copy_copy_non_scalar_members() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *malloc(unsigned long size);
        union simple {
            int i;
            long l;
            char c;
            unsigned char uc_arr[3];
        };
        union has_union {
            double d;
            union simple u;
            union simple *u_ptr;
        };
        struct simple_struct {
            long l;
            double d;
            unsigned int u;
        };
        union has_struct {
            long l;
            struct simple_struct s;
        };
        struct struct_with_union {
            union simple u;
            unsigned long ul;
        };
        union complex_union {
            double d_arr[2];
            struct struct_with_union s;
            union has_union *u_ptr;
        };
        
        void* calloc(unsigned long nmemb, unsigned long size);
        int test_dot(void) {
            struct struct_with_union my_struct = { {0}, 100000l };
            union simple my_simple_union;
            my_simple_union.l = -1;
            my_struct.u = my_simple_union;
            static union complex_union my_union;
            my_union.s = my_struct;
            if (my_struct.ul != 100000l || my_struct.u.l != -1) {
                return 0;
            }
            if (my_union.s.ul != 100000l) {
                return 0;
            }
            my_union.s.u.i = 45;
            my_simple_union = my_union.s.u;
            if (my_simple_union.i != 45) {
                return 0;
            }
            static struct struct_with_union another_struct;
            another_struct = my_union.s;
            if (another_struct.ul != 100000l || another_struct.u.i != 45) {
                return 0;
            }
            return 1;
        }
        int test_arrow(void) {
            union complex_union* my_union_ptr = calloc(1, sizeof(union complex_union));
            my_union_ptr->u_ptr = calloc(1, sizeof(union has_union));
            my_union_ptr->u_ptr->u_ptr = calloc(1, sizeof(union simple));
            my_union_ptr->u_ptr->u_ptr->i = 987654321;
            union has_union another_union = *my_union_ptr->u_ptr;
            if (another_union.u_ptr != my_union_ptr->u_ptr->u_ptr || another_union.u_ptr->c != my_union_ptr->u_ptr->u_ptr->c) {
                return 0;
            }
            union simple small_union = { -9999 };
            my_union_ptr->u_ptr->u = small_union;
            if (my_union_ptr->u_ptr->u.i != -9999) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_dot()) {
                return 1;
            }
            if (!test_arrow()) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_dot() { 
            my_struct.11[0] = 0
            tmp.0 = 100000L
            my_struct.11[8] = tmp.0
            tmp.1 = - 1
            tmp.2 = sign_extend tmp.1
            my_simple_union.12[0] = tmp.2
            tmp.3 = my_simple_union.12[0]
            my_struct.11[0] = my_simple_union.12
            tmp.4 = my_struct.11[0]
            my_union.13[0] = my_struct.11
            tmp.5 = my_union.13[0]
            tmp.6 = my_struct.11[8]
            tmp.8 = 100000L
            tmp.7 = tmp.6 != tmp.8
            if tmp.7 jump or_true_0
            tmp.11 = my_struct.11[0]
            tmp.13 = - 1
            tmp.14 = sign_extend tmp.13
            tmp.12 = tmp.11 != tmp.14
            if tmp.12 jump or_true_0
            tmp.10 = 0
            jump or_end_1
        
          or_true_0:
            tmp.10 = 1
        
          or_end_1:
            if !tmp.10 jump end_if_2
            return 0
        
          end_if_2:
            tmp.15 = my_union.13[8]
            tmp.17 = 100000L
            tmp.16 = tmp.15 != tmp.17
            if !tmp.16 jump end_if_4
            return 0
        
          end_if_4:
            my_union.13[0] = 45
            tmp.18 = my_union.13[0]
            tmp.19 = my_union.13[0]
            my_simple_union.12 = tmp.19
            tmp.20 = my_simple_union.12[0]
            tmp.21 = tmp.20 != 45
            if !tmp.21 jump end_if_6
            return 0
        
          end_if_6:
            tmp.22 = my_union.13[0]
            another_struct.14 = tmp.22
            tmp.23 = another_struct.14[8]
            tmp.25 = 100000L
            tmp.24 = tmp.23 != tmp.25
            if tmp.24 jump or_true_8
            tmp.28 = another_struct.14[0]
            tmp.29 = tmp.28 != 45
            if tmp.29 jump or_true_8
            tmp.27 = 0
            jump or_end_9
        
          or_true_8:
            tmp.27 = 1
        
          or_end_9:
            if !tmp.27 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_arrow() { 
            tmp.30 = sign_extend 1
            tmp.31 = calloc(tmp.30, 16UL)
            tmp.32 = tmp.31
            my_union_ptr.15 = tmp.32
            tmp.33 = sign_extend 1
            tmp.34 = calloc(tmp.33, 8UL)
            tmp.35 = tmp.34
            *my_union_ptr.15 = tmp.35
            tmp.36 = *my_union_ptr.15
            tmp.37 = *my_union_ptr.15
            tmp.38 = sign_extend 1
            tmp.39 = calloc(tmp.38, 8UL)
            tmp.40 = tmp.39
            *tmp.37 = tmp.40
            tmp.41 = *tmp.37
            tmp.42 = *my_union_ptr.15
            tmp.43 = *tmp.42
            *tmp.43 = 987654321
            tmp.44 = *tmp.43
            tmp.45 = *my_union_ptr.15
            tmp.46 = *tmp.45
            another_union.16 = tmp.46
            tmp.47 = another_union.16[0]
            tmp.49 = *my_union_ptr.15
            tmp.50 = *tmp.49
            tmp.48 = tmp.47 != tmp.50
            if tmp.48 jump or_true_12
            tmp.53 = another_union.16[0]
            tmp.54 = *tmp.53
            tmp.55 = sign_extend tmp.54
            tmp.57 = *my_union_ptr.15
            tmp.58 = *tmp.57
            tmp.59 = *tmp.58
            tmp.60 = sign_extend tmp.59
            tmp.56 = tmp.55 != tmp.60
            if tmp.56 jump or_true_12
            tmp.52 = 0
            jump or_end_13
        
          or_true_12:
            tmp.52 = 1
        
          or_end_13:
            if !tmp.52 jump end_if_14
            return 0
        
          end_if_14:
            tmp.61 = - 9999
            small_union.17[0] = tmp.61
            tmp.62 = *my_union_ptr.15
            *tmp.62 = small_union.17
            tmp.63 = *tmp.62
            tmp.64 = *my_union_ptr.15
            tmp.65 = *tmp.64
            tmp.67 = - 9999
            tmp.66 = tmp.65 != tmp.67
            if !tmp.66 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function main() { 
            tmp.68 = test_dot()
            tmp.69 = ! tmp.68
            if !tmp.69 jump end_if_18
            return 1
        
          end_if_18:
            tmp.70 = test_arrow()
            tmp.71 = ! tmp.70
            if !tmp.71 jump end_if_20
            return 2
        
          end_if_20:
            return 0
            return 0
        }
        static another_struct.14: Struct(struct_with_union.7) = zero[16]
        static my_union.13: Union(complex_union.8) = zero[16]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_union_copy_copy_thru_pointer() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *malloc(unsigned long size);
        union simple {
            int i;
            long l;
            char c;
            unsigned char uc_arr[3];
        };
        union has_union {
            double d;
            union simple u;
            union simple *u_ptr;
        };
        struct simple_struct {
            long l;
            double d;
            unsigned int u;
        };
        union has_struct {
            long l;
            struct simple_struct s;
        };
        struct struct_with_union {
            union simple u;
            unsigned long ul;
        };
        union complex_union {
            double d_arr[2];
            struct struct_with_union s;
            union has_union *u_ptr;
        };
        
        int strcmp(char* s1, char* s2);
        int test_copy_to_pointer(void) {
            union simple y;
            y.l = -20;
            union simple* x = malloc(sizeof(union simple));
            *x = y;
            if (x->l != -20 || x->i != -20 || x->uc_arr[0] != 236 || x->uc_arr[1] != 255 || x->uc_arr[2] != 255) {
                return 0;
            }
            return 1;
        }
        int test_copy_from_pointer(void) {
            struct simple_struct my_struct = { 8223372036854775807l, 20e3, 2147483650u };
            static union has_struct my_union;
            my_union.s = my_struct;
            union has_struct* union_ptr;
            union_ptr = &my_union;
            union has_struct another_union = *union_ptr;
            if (another_union.s.l != 8223372036854775807l || another_union.s.d != 20e3 || another_union.s.u != 2147483650u) {
                return 0;
            }
            return 1;
        }
        union with_padding {
            char arr[10];
            unsigned int ui;
        };
        int test_copy_array_members(void) {
            union with_padding union_array[3] = { {"foobar"}, {"hello"}, {"itsaunion"} };
            union with_padding another_union = union_array[0];
            union with_padding yet_another_union = { "blahblah" };
            union_array[2] = yet_another_union;
            if (strcmp(union_array[0].arr, "foobar") || strcmp(union_array[1].arr, "hello") || strcmp(union_array[2].arr, "blahblah")) {
                return 0;
            }
            if (strcmp(another_union.arr, "foobar")) {
                return 0;
            }
            if (strcmp(yet_another_union.arr, "blahblah")) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_copy_to_pointer()){
                return 1;
            }
            if (!test_copy_from_pointer()) {
                return 2;
            }
            if (!test_copy_array_members()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_copy_to_pointer() { 
            tmp.0 = - 20
            tmp.1 = sign_extend tmp.0
            y.11[0] = tmp.1
            tmp.2 = y.11[0]
            tmp.3 = malloc(8UL)
            tmp.4 = tmp.3
            x.12 = tmp.4
            *x.12 = y.11
            tmp.5 = *x.12
            tmp.6 = *x.12
            tmp.8 = - 20
            tmp.9 = sign_extend tmp.8
            tmp.7 = tmp.6 != tmp.9
            if tmp.7 jump or_true_0
            tmp.12 = *x.12
            tmp.14 = - 20
            tmp.13 = tmp.12 != tmp.14
            if tmp.13 jump or_true_0
            tmp.11 = 0
            jump or_end_1
        
          or_true_0:
            tmp.11 = 1
        
          or_end_1:
            if tmp.11 jump or_true_2
            tmp.17 = sign_extend 0
            tmp.18 = add_ptr(x.12, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            tmp.20 = zero_extend tmp.19
            tmp.21 = tmp.20 != 236
            if tmp.21 jump or_true_2
            tmp.16 = 0
            jump or_end_3
        
          or_true_2:
            tmp.16 = 1
        
          or_end_3:
            if tmp.16 jump or_true_4
            tmp.24 = sign_extend 1
            tmp.25 = add_ptr(x.12, index=tmp.24, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = zero_extend tmp.26
            tmp.28 = tmp.27 != 255
            if tmp.28 jump or_true_4
            tmp.23 = 0
            jump or_end_5
        
          or_true_4:
            tmp.23 = 1
        
          or_end_5:
            if tmp.23 jump or_true_6
            tmp.31 = sign_extend 2
            tmp.32 = add_ptr(x.12, index=tmp.31, scale=1)
            tmp.33 = *tmp.32
            tmp.34 = zero_extend tmp.33
            tmp.35 = tmp.34 != 255
            if tmp.35 jump or_true_6
            tmp.30 = 0
            jump or_end_7
        
          or_true_6:
            tmp.30 = 1
        
          or_end_7:
            if !tmp.30 jump end_if_8
            return 0
        
          end_if_8:
            return 1
            return 0
        }
        global function test_copy_from_pointer() { 
            my_struct.13[0] = 8223372036854775807L
            my_struct.13[8] = 20000D
            my_struct.13[16] = 2147483650U
            my_union.14[0] = my_struct.13
            tmp.36 = my_union.14[0]
            tmp.37 = &my_union.14
            union_ptr.15 = tmp.37
            tmp.38 = *union_ptr.15
            another_union.16 = tmp.38
            tmp.39 = another_union.16[0]
            tmp.40 = tmp.39 != 8223372036854775807L
            if tmp.40 jump or_true_10
            tmp.43 = another_union.16[8]
            tmp.44 = tmp.43 != 20000D
            if tmp.44 jump or_true_10
            tmp.42 = 0
            jump or_end_11
        
          or_true_10:
            tmp.42 = 1
        
          or_end_11:
            if tmp.42 jump or_true_12
            tmp.47 = another_union.16[16]
            tmp.48 = tmp.47 != 2147483650U
            if tmp.48 jump or_true_12
            tmp.46 = 0
            jump or_end_13
        
          or_true_12:
            tmp.46 = 1
        
          or_end_13:
            if !tmp.46 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_copy_array_members() { 
            union_array.18[0] = 'f'
            union_array.18[1] = 'o'
            union_array.18[2] = 'o'
            union_array.18[3] = 'b'
            union_array.18[4] = 'a'
            union_array.18[5] = 'r'
            union_array.18[6] = '\0'
            union_array.18[7] = '\0'
            union_array.18[8] = '\0'
            union_array.18[9] = '\0'
            union_array.18[12] = 'h'
            union_array.18[13] = 'e'
            union_array.18[14] = 'l'
            union_array.18[15] = 'l'
            union_array.18[16] = 'o'
            union_array.18[17] = '\0'
            union_array.18[18] = '\0'
            union_array.18[19] = '\0'
            union_array.18[20] = '\0'
            union_array.18[21] = '\0'
            union_array.18[24] = 'i'
            union_array.18[25] = 't'
            union_array.18[26] = 's'
            union_array.18[27] = 'a'
            union_array.18[28] = 'u'
            union_array.18[29] = 'n'
            union_array.18[30] = 'i'
            union_array.18[31] = 'o'
            union_array.18[32] = 'n'
            union_array.18[33] = '\0'
            tmp.49 = &union_array.18
            tmp.50 = sign_extend 0
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=12)
            tmp.52 = *tmp.51
            another_union.19 = tmp.52
            yet_another_union.20[0] = 'b'
            yet_another_union.20[1] = 'l'
            yet_another_union.20[2] = 'a'
            yet_another_union.20[3] = 'h'
            yet_another_union.20[4] = 'b'
            yet_another_union.20[5] = 'l'
            yet_another_union.20[6] = 'a'
            yet_another_union.20[7] = 'h'
            yet_another_union.20[8] = '\0'
            yet_another_union.20[9] = '\0'
            tmp.53 = &union_array.18
            tmp.54 = sign_extend 2
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=12)
            *tmp.55 = yet_another_union.20
            tmp.56 = *tmp.55
            tmp.57 = &union_array.18
            tmp.58 = sign_extend 0
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=12)
            tmp.60 = &string.0
            tmp.61 = strcmp(tmp.59, tmp.60)
            if tmp.61 jump or_true_16
            tmp.64 = &union_array.18
            tmp.65 = sign_extend 1
            tmp.66 = add_ptr(tmp.64, index=tmp.65, scale=12)
            tmp.67 = &string.1
            tmp.68 = strcmp(tmp.66, tmp.67)
            if tmp.68 jump or_true_16
            tmp.63 = 0
            jump or_end_17
        
          or_true_16:
            tmp.63 = 1
        
          or_end_17:
            if tmp.63 jump or_true_18
            tmp.71 = &union_array.18
            tmp.72 = sign_extend 2
            tmp.73 = add_ptr(tmp.71, index=tmp.72, scale=12)
            tmp.74 = &string.2
            tmp.75 = strcmp(tmp.73, tmp.74)
            if tmp.75 jump or_true_18
            tmp.70 = 0
            jump or_end_19
        
          or_true_18:
            tmp.70 = 1
        
          or_end_19:
            if !tmp.70 jump end_if_20
            return 0
        
          end_if_20:
            tmp.76 = &another_union.19
            tmp.77 = &string.0
            tmp.78 = strcmp(tmp.76, tmp.77)
            if !tmp.78 jump end_if_22
            return 0
        
          end_if_22:
            tmp.79 = &yet_another_union.20
            tmp.80 = &string.2
            tmp.81 = strcmp(tmp.79, tmp.80)
            if !tmp.81 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function main() { 
            tmp.82 = test_copy_to_pointer()
            tmp.83 = ! tmp.82
            if !tmp.83 jump end_if_26
            return 1
        
          end_if_26:
            tmp.84 = test_copy_from_pointer()
            tmp.85 = ! tmp.84
            if !tmp.85 jump end_if_28
            return 2
        
          end_if_28:
            tmp.86 = test_copy_array_members()
            tmp.87 = ! tmp.86
            if !tmp.87 jump end_if_30
            return 3
        
          end_if_30:
            return 0
            return 0
        }
        static my_union.14: Union(has_struct.6) = zero[24]
        constant string.0: Array(7,Char) = "foobar\\0"
        constant string.1: Array(6,Char) = "hello\\0"
        constant string.2: Array(9,Char) = "blahblah\\0"
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_union_copy_unions_in_conditionals() {
    let src = r#"
        union u {
            long l;
            int i;
            char c;
        };
        int choose_union(int flag) {
            union u one;
            union u two;
            one.l = -1;
            two.i = 100;
            return (flag ? one : two).c;
        }
        int main(void) {
            if (choose_union(1) != -1) {
                return 1;
            }
            if (choose_union(0) != 100) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function choose_union(flag.1) { 
            tmp.0 = - 1
            tmp.1 = sign_extend tmp.0
            one.2[0] = tmp.1
            tmp.2 = one.2[0]
            two.3[0] = 100
            tmp.3 = two.3[0]
            if !flag.1 jump else_1
            tmp.4 = one.2
            jump end_if_0
        
          else_1:
            tmp.4 = two.3
        
          end_if_0:
            tmp.5 = tmp.4[0]
            tmp.6 = sign_extend tmp.5
            return tmp.6
            return 0
        }
        global function main() { 
            tmp.7 = choose_union(1)
            tmp.9 = - 1
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_2
            return 1
        
          end_if_2:
            tmp.10 = choose_union(0)
            tmp.11 = tmp.10 != 100
            if !tmp.11 jump end_if_4
            return 2
        
          end_if_4:
            return 0
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
            tmp.9 = add_ptr(tmp.8, index=8L, scale=1)
            tmp.10 = *tmp.9
            tmp.12 = i.4 * 3
            tmp.13 = sign_extend tmp.12
            tmp.11 = tmp.10 != tmp.13
            if !tmp.11 jump end_if_2
            return 0
        
          end_if_2:
            tmp.14 = sign_extend i.4
            tmp.15 = add_ptr(struct_array.3, index=tmp.14, scale=24)
            tmp.16 = add_ptr(tmp.15, index=8L, scale=1)
            tmp.17 = add_ptr(tmp.16, index=8L, scale=1)
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
            tmp.26 = add_ptr(tmp.25, index=8L, scale=1)
            tmp.27 = add_ptr(tmp.26, index=8L, scale=1)
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
            auto_array.3[0] = tmp.0
            tmp.1 = sign_extend 0
            auto_array.3[8] = tmp.1
            tmp.2 = truncate 0
            auto_array.3[16] = tmp.2
            tmp.3 = truncate 0
            auto_array.3[17] = tmp.3
            tmp.4 = truncate 2
            auto_array.3[24] = tmp.4
            tmp.5 = sign_extend 3
            auto_array.3[32] = tmp.5
            tmp.6 = truncate 4
            auto_array.3[40] = tmp.6
            tmp.7 = truncate 5
            auto_array.3[41] = tmp.7
            tmp.8 = truncate 4
            auto_array.3[48] = tmp.8
            tmp.9 = sign_extend 6
            auto_array.3[56] = tmp.9
            tmp.10 = truncate 8
            auto_array.3[64] = tmp.10
            tmp.11 = truncate 10
            auto_array.3[65] = tmp.11
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
            tmp.0 = &global
            tmp.0 = add_ptr(tmp.0, index=4L, scale=1)
            tmp.1 = sign_extend 1
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = &global
            tmp.3 = add_ptr(tmp.3, index=4L, scale=1)
            tmp.4 = sign_extend 0
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=1)
            tmp.6 = *tmp.5
            tmp.7 = sign_extend tmp.6
            tmp.8 = tmp.7 * 2
            tmp.9 = truncate tmp.8
            *tmp.2 = tmp.9
            tmp.10 = *tmp.2
            global[8] = 5D
            tmp.11 = global[8]
            return 0
        }
        global function update_outer_struct() { 
            inner.2[0] = 0
            tmp.12 = - 1
            tmp.13 = truncate tmp.12
            inner.2[4] = tmp.13
            tmp.14 = - 1
            tmp.15 = truncate tmp.14
            inner.2[5] = tmp.15
            tmp.16 = int_to_double 0
            inner.2[8] = tmp.16
            global_outer[8] = inner.2
            tmp.17 = global_outer[8]
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
            tmp.0 = &global
            tmp.0 = add_ptr(tmp.0, index=4L, scale=1)
            tmp.1 = sign_extend 1
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = *tmp.2
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 != 4
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = global[8]
            tmp.7 = tmp.6 != 5D
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            update_outer_struct()
            tmp.8 = global_outer[0]
            tmp.9 = sign_extend tmp.8
            tmp.10 = tmp.9 != 5
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = global_outer[8]
            if tmp.11 jump or_true_6
            tmp.14 = global_outer[16]
            if tmp.14 jump or_true_6
            tmp.13 = 0
            jump or_end_7
        
          or_true_6:
            tmp.13 = 1
        
          or_end_7:
            if !tmp.13 jump end_if_8
            return 4
        
          end_if_8:
            tmp.15 = &global_outer
            tmp.15 = add_ptr(tmp.15, index=12L, scale=1)
            tmp.16 = sign_extend 0
            tmp.17 = add_ptr(tmp.15, index=tmp.16, scale=1)
            tmp.18 = *tmp.17
            tmp.19 = sign_extend tmp.18
            tmp.21 = - 1
            tmp.20 = tmp.19 != tmp.21
            if tmp.20 jump or_true_10
            tmp.24 = &global_outer
            tmp.24 = add_ptr(tmp.24, index=12L, scale=1)
            tmp.25 = sign_extend 1
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=1)
            tmp.27 = *tmp.26
            tmp.28 = sign_extend tmp.27
            tmp.30 = - 1
            tmp.29 = tmp.28 != tmp.30
            if tmp.29 jump or_true_10
            tmp.23 = 0
            jump or_end_11
        
          or_true_10:
            tmp.23 = 1
        
          or_end_11:
            if !tmp.23 jump end_if_12
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
            tmp.0 = *ptr.12
            tmp.1 = &string.0
            tmp.2 = strcmp(tmp.0, tmp.1)
            if tmp.2 jump or_true_0
            tmp.5 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.6 = sign_extend 0
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=1)
            tmp.8 = *tmp.7
            tmp.9 = zero_extend tmp.8
            tmp.10 = tmp.9 != 115
            if tmp.10 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if tmp.4 jump or_true_2
            tmp.13 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.14 = sign_extend 1
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=1)
            tmp.16 = *tmp.15
            tmp.17 = zero_extend tmp.16
            tmp.18 = tmp.17 != 117
            if tmp.18 jump or_true_2
            tmp.12 = 0
            jump or_end_3
        
          or_true_2:
            tmp.12 = 1
        
          or_end_3:
            if tmp.12 jump or_true_4
            tmp.21 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.22 = sign_extend 2
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            tmp.25 = zero_extend tmp.24
            tmp.26 = tmp.25 != 112
            if tmp.26 jump or_true_4
            tmp.20 = 0
            jump or_end_5
        
          or_true_4:
            tmp.20 = 1
        
          or_end_5:
            if tmp.20 jump or_true_6
            tmp.29 = add_ptr(ptr.12, index=16L, scale=1)
            tmp.30 = *tmp.29
            tmp.31 = tmp.30 != ptr.12
            if tmp.31 jump or_true_6
            tmp.28 = 0
            jump or_end_7
        
          or_true_6:
            tmp.28 = 1
        
          or_end_7:
            if tmp.28 jump or_true_8
            tmp.34 = add_ptr(ptr.12, index=24L, scale=1)
            tmp.35 = *tmp.34
            tmp.36 = tmp.35 != 2000000000000D
            if tmp.36 jump or_true_8
            tmp.33 = 0
            jump or_end_9
        
          or_true_8:
            tmp.33 = 1
        
          or_end_9:
            if tmp.33 jump or_true_10
            tmp.39 = add_ptr(ptr.12, index=32L, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = *tmp.40
            tmp.42 = tmp.41 != 2000000000000D
            if tmp.42 jump or_true_10
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
            tmp.43 = *ptr.13
            tmp.44 = tmp.43 != expected_msg.14
            if tmp.44 jump or_true_14
            tmp.47 = add_ptr(ptr.13, index=8L, scale=1)
            tmp.48 = sign_extend 0
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=1)
            tmp.50 = *tmp.49
            tmp.51 = zero_extend tmp.50
            tmp.52 = tmp.51 != 97
            if tmp.52 jump or_true_14
            tmp.46 = 0
            jump or_end_15
        
          or_true_14:
            tmp.46 = 1
        
          or_end_15:
            if tmp.46 jump or_true_16
            tmp.55 = add_ptr(ptr.13, index=8L, scale=1)
            tmp.56 = sign_extend 1
            tmp.57 = add_ptr(tmp.55, index=tmp.56, scale=1)
            tmp.58 = *tmp.57
            tmp.59 = zero_extend tmp.58
            tmp.60 = tmp.59 != 98
            if tmp.60 jump or_true_16
            tmp.54 = 0
            jump or_end_17
        
          or_true_16:
            tmp.54 = 1
        
          or_end_17:
            if !tmp.54 jump end_if_18
            return 0
        
          end_if_18:
            tmp.61 = add_ptr(ptr.13, index=16L, scale=1)
            tmp.62 = *tmp.61
            tmp.63 = *tmp.62
            if !tmp.63 jump end_if_20
            return 0
        
          end_if_20:
            tmp.64 = add_ptr(ptr.13, index=8L, scale=1)
            tmp.65 = sign_extend 2
            tmp.66 = add_ptr(tmp.64, index=tmp.65, scale=1)
            tmp.67 = *tmp.66
            if tmp.67 jump or_true_22
            tmp.70 = add_ptr(ptr.13, index=24L, scale=1)
            tmp.71 = *tmp.70
            if tmp.71 jump or_true_22
            tmp.69 = 0
            jump or_end_23
        
          or_true_22:
            tmp.69 = 1
        
          or_end_23:
            if tmp.69 jump or_true_24
            tmp.74 = add_ptr(ptr.13, index=32L, scale=1)
            tmp.75 = *tmp.74
            if tmp.75 jump or_true_24
            tmp.73 = 0
            jump or_end_25
        
          or_true_24:
            tmp.73 = 1
        
          or_end_25:
            if !tmp.73 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function validate_converted(ptr.15) { 
            tmp.76 = *ptr.15
            tmp.77 = ! tmp.76
            if tmp.77 jump or_true_28
            tmp.80 = add_ptr(ptr.15, index=8L, scale=1)
            tmp.81 = sign_extend 0
            tmp.82 = add_ptr(tmp.80, index=tmp.81, scale=1)
            tmp.83 = *tmp.82
            tmp.84 = zero_extend tmp.83
            tmp.85 = tmp.84 != 220
            if tmp.85 jump or_true_28
            tmp.79 = 0
            jump or_end_29
        
          or_true_28:
            tmp.79 = 1
        
          or_end_29:
            if tmp.79 jump or_true_30
            tmp.88 = add_ptr(ptr.15, index=8L, scale=1)
            tmp.89 = sign_extend 1
            tmp.90 = add_ptr(tmp.88, index=tmp.89, scale=1)
            tmp.91 = *tmp.90
            tmp.92 = zero_extend tmp.91
            tmp.93 = tmp.92 != 232
            if tmp.93 jump or_true_30
            tmp.87 = 0
            jump or_end_31
        
          or_true_30:
            tmp.87 = 1
        
          or_end_31:
            if tmp.87 jump or_true_32
            tmp.96 = add_ptr(ptr.15, index=8L, scale=1)
            tmp.97 = sign_extend 2
            tmp.98 = add_ptr(tmp.96, index=tmp.97, scale=1)
            tmp.99 = *tmp.98
            tmp.100 = zero_extend tmp.99
            tmp.101 = tmp.100 != 224
            if tmp.101 jump or_true_32
            tmp.95 = 0
            jump or_end_33
        
          or_true_32:
            tmp.95 = 1
        
          or_end_33:
            if tmp.95 jump or_true_34
            tmp.104 = add_ptr(ptr.15, index=16L, scale=1)
            tmp.105 = *tmp.104
            if tmp.105 jump or_true_34
            tmp.103 = 0
            jump or_end_35
        
          or_true_34:
            tmp.103 = 1
        
          or_end_35:
            if tmp.103 jump or_true_36
            tmp.108 = add_ptr(ptr.15, index=24L, scale=1)
            tmp.109 = *tmp.108
            tmp.110 = tmp.109 != 2999D
            if tmp.110 jump or_true_36
            tmp.107 = 0
            jump or_end_37
        
          or_true_36:
            tmp.107 = 1
        
          or_end_37:
            if tmp.107 jump or_true_38
            tmp.113 = add_ptr(ptr.15, index=32L, scale=1)
            tmp.114 = *tmp.113
            tmp.115 = *tmp.114
            tmp.116 = tmp.115 != 0D
            if tmp.116 jump or_true_38
            tmp.112 = 0
            jump or_end_39
        
          or_true_38:
            tmp.112 = 1
        
          or_end_39:
            if !tmp.112 jump end_if_40
            return 0
        
          end_if_40:
            return 1
            return 0
        }
        global function validate_two_structs(ptr1.16, ptr2.17) { 
            tmp.117 = *ptr2.17
            tmp.118 = &string.1
            tmp.119 = strcmp(tmp.117, tmp.118)
            if tmp.119 jump or_true_42
            tmp.122 = *ptr2.17
            tmp.124 = *ptr1.16
            tmp.123 = tmp.122 != tmp.124
            if tmp.123 jump or_true_42
            tmp.121 = 0
            jump or_end_43
        
          or_true_42:
            tmp.121 = 1
        
          or_end_43:
            if tmp.121 jump or_true_44
            tmp.127 = add_ptr(ptr2.17, index=8L, scale=1)
            tmp.128 = sign_extend 0
            tmp.129 = add_ptr(tmp.127, index=tmp.128, scale=1)
            tmp.130 = *tmp.129
            tmp.131 = zero_extend tmp.130
            tmp.132 = tmp.131 != 120
            if tmp.132 jump or_true_44
            tmp.126 = 0
            jump or_end_45
        
          or_true_44:
            tmp.126 = 1
        
          or_end_45:
            if tmp.126 jump or_true_46
            tmp.135 = add_ptr(ptr2.17, index=8L, scale=1)
            tmp.136 = sign_extend 1
            tmp.137 = add_ptr(tmp.135, index=tmp.136, scale=1)
            tmp.138 = *tmp.137
            tmp.139 = zero_extend tmp.138
            tmp.140 = tmp.139 != 121
            if tmp.140 jump or_true_46
            tmp.134 = 0
            jump or_end_47
        
          or_true_46:
            tmp.134 = 1
        
          or_end_47:
            if tmp.134 jump or_true_48
            tmp.143 = add_ptr(ptr2.17, index=16L, scale=1)
            tmp.144 = *tmp.143
            tmp.145 = tmp.144 != ptr1.16
            if tmp.145 jump or_true_48
            tmp.142 = 0
            jump or_end_49
        
          or_true_48:
            tmp.142 = 1
        
          or_end_49:
            if tmp.142 jump or_true_50
            tmp.148 = add_ptr(ptr2.17, index=24L, scale=1)
            tmp.149 = *tmp.148
            tmp.150 = tmp.149 != 150D
            if tmp.150 jump or_true_50
            tmp.147 = 0
            jump or_end_51
        
          or_true_50:
            tmp.147 = 1
        
          or_end_51:
            if tmp.147 jump or_true_52
            tmp.153 = add_ptr(ptr1.16, index=32L, scale=1)
            tmp.154 = *tmp.153
            tmp.155 = *tmp.154
            tmp.156 = tmp.155 != 123.4D
            if tmp.156 jump or_true_52
            tmp.152 = 0
            jump or_end_53
        
          or_true_52:
            tmp.152 = 1
        
          or_end_53:
            if !tmp.152 jump end_if_54
            return 0
        
          end_if_54:
            tmp.157 = add_ptr(ptr1.16, index=8L, scale=1)
            tmp.159 = add_ptr(ptr2.17, index=8L, scale=1)
            tmp.158 = tmp.157 == tmp.159
            if !tmp.158 jump end_if_56
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
            tmp.0 = &string.1
            full.12[0] = tmp.0
            full.12[8] = 115UC
            full.12[9] = 117UC
            full.12[10] = 112UC
            tmp.1 = &full.12
            full.12[16] = tmp.1
            tmp.2 = get_double()
            full.12[24] = tmp.2
            tmp.3 = &full.12
            tmp.3 = add_ptr(tmp.3, index=24L, scale=1)
            full.12[32] = tmp.3
            tmp.4 = &full.12
            tmp.5 = validate_full_initialization(tmp.4)
            return tmp.5
            return 0
        }
        global function test_partial_initialization() { 
            partial.14[0] = msg.13
            tmp.6 = truncate 97
            partial.14[8] = tmp.6
            tmp.7 = truncate 98
            partial.14[9] = tmp.7
            partial.14[10] = 0UC
            tmp.8 = sign_extend 1
            tmp.9 = calloc(tmp.8, 40UL)
            tmp.10 = tmp.9
            partial.14[16] = tmp.10
            partial.14[24] = 0D
            partial.14[32] = 0UL
            tmp.11 = &partial.14
            tmp.12 = validate_partial_initialization(tmp.11, msg.13)
            return tmp.12
            return 0
        }
        global function test_implicit_type_conversions() { 
            tmp.13 = sign_extend 5
            tmp.14 = malloc(tmp.13)
            tmp.15 = tmp.14
            converted.16[0] = tmp.15
            tmp.16 = i.15 / 2
            tmp.17 = truncate tmp.16
            converted.16[8] = tmp.17
            tmp.18 = i.15 / 3
            tmp.19 = truncate tmp.18
            converted.16[9] = tmp.19
            tmp.20 = i.15 * 4
            tmp.21 = truncate tmp.20
            converted.16[10] = tmp.21
            tmp.22 = 0L
            converted.16[16] = tmp.22
            tmp.23 = i.15 - 1
            tmp.24 = int_to_double tmp.23
            converted.16[24] = tmp.24
            tmp.25 = sign_extend 1
            tmp.26 = calloc(tmp.25, 8UL)
            tmp.27 = tmp.26
            converted.16[32] = tmp.27
            tmp.28 = &converted.16
            tmp.29 = validate_converted(tmp.28)
            return tmp.29
            return 0
        }
        global function test_single_exp_initializer() { 
            d.17 = 123.4D
            tmp.30 = &string.2
            s1.18[0] = tmp.30
            s1.18[8] = 120UC
            s1.18[9] = 121UC
            s1.18[10] = '\0'
            tmp.31 = &s1.18
            s1.18[16] = tmp.31
            s1.18[24] = 150D
            tmp.32 = &d.17
            s1.18[32] = tmp.32
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
        static msg.13: Pointer(Char) = &string.0
        constant string.0: Array(23,Char) = "Another string literal\\0"
        constant string.1: Array(14,Char) = "I'm a struct!\\0"
        constant string.2: Array(19,Char) = "Yet another string\\0"
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
            tmp.0 = *ptr.9
            tmp.2 = - 200L
            tmp.1 = tmp.0 != tmp.2
            if tmp.1 jump or_true_0
            tmp.5 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.6 = *tmp.5
            tmp.8 = - 171
            tmp.7 = tmp.6 != tmp.8
            if tmp.7 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if tmp.4 jump or_true_2
            tmp.11 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.12 = add_ptr(tmp.11, index=4L, scale=1)
            tmp.13 = sign_extend 0
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = zero_extend tmp.15
            tmp.17 = tmp.16 != 200
            if tmp.17 jump or_true_2
            tmp.10 = 0
            jump or_end_3
        
          or_true_2:
            tmp.10 = 1
        
          or_end_3:
            if tmp.10 jump or_true_4
            tmp.20 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.21 = add_ptr(tmp.20, index=4L, scale=1)
            tmp.22 = sign_extend 1
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            tmp.25 = zero_extend tmp.24
            tmp.26 = tmp.25 != 202
            if tmp.26 jump or_true_4
            tmp.19 = 0
            jump or_end_5
        
          or_true_4:
            tmp.19 = 1
        
          or_end_5:
            if tmp.19 jump or_true_6
            tmp.29 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.30 = add_ptr(tmp.29, index=4L, scale=1)
            tmp.31 = sign_extend 2
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=1)
            tmp.33 = *tmp.32
            tmp.34 = zero_extend tmp.33
            tmp.35 = tmp.34 != 203
            if tmp.35 jump or_true_6
            tmp.28 = 0
            jump or_end_7
        
          or_true_6:
            tmp.28 = 1
        
          or_end_7:
            if tmp.28 jump or_true_8
            tmp.38 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.39 = add_ptr(tmp.38, index=8L, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = tmp.40 != 40U
            if tmp.41 jump or_true_8
            tmp.37 = 0
            jump or_end_9
        
          or_true_8:
            tmp.37 = 1
        
          or_end_9:
            if tmp.37 jump or_true_10
            tmp.44 = add_ptr(ptr.9, index=24L, scale=1)
            tmp.45 = *tmp.44
            tmp.46 = &string.0
            tmp.47 = strcmp(tmp.45, tmp.46)
            if tmp.47 jump or_true_10
            tmp.43 = 0
            jump or_end_11
        
          or_true_10:
            tmp.43 = 1
        
          or_end_11:
            if tmp.43 jump or_true_12
            tmp.50 = add_ptr(ptr.9, index=32L, scale=1)
            tmp.51 = *tmp.50
            tmp.53 = - 22D
            tmp.52 = tmp.51 != tmp.53
            if tmp.52 jump or_true_12
            tmp.49 = 0
            jump or_end_13
        
          or_true_12:
            tmp.49 = 1
        
          or_end_13:
            if tmp.49 jump or_true_14
            tmp.56 = add_ptr(ptr.9, index=40L, scale=1)
            tmp.57 = *tmp.56
            tmp.58 = tmp.57 != 1
            if tmp.58 jump or_true_14
            tmp.55 = 0
            jump or_end_15
        
          or_true_14:
            tmp.55 = 1
        
          or_end_15:
            if tmp.55 jump or_true_16
            tmp.61 = add_ptr(ptr.9, index=40L, scale=1)
            tmp.62 = add_ptr(tmp.61, index=4L, scale=1)
            tmp.63 = *tmp.62
            tmp.64 = tmp.63 != 2
            if tmp.64 jump or_true_16
            tmp.60 = 0
            jump or_end_17
        
          or_true_16:
            tmp.60 = 1
        
          or_end_17:
            if !tmp.60 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function validate_partial_initialization(ptr.10) { 
            tmp.65 = *ptr.10
            tmp.67 = sign_extend 1000
            tmp.66 = tmp.65 != tmp.67
            if tmp.66 jump or_true_20
            tmp.70 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.71 = *tmp.70
            tmp.72 = tmp.71 != 1
            if tmp.72 jump or_true_20
            tmp.69 = 0
            jump or_end_21
        
          or_true_20:
            tmp.69 = 1
        
          or_end_21:
            if tmp.69 jump or_true_22
            tmp.75 = add_ptr(ptr.10, index=24L, scale=1)
            tmp.76 = *tmp.75
            tmp.77 = &string.1
            tmp.78 = strcmp(tmp.76, tmp.77)
            if tmp.78 jump or_true_22
            tmp.74 = 0
            jump or_end_23
        
          or_true_22:
            tmp.74 = 1
        
          or_end_23:
            if !tmp.74 jump end_if_24
            return 0
        
          end_if_24:
            tmp.79 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.80 = add_ptr(tmp.79, index=4L, scale=1)
            tmp.81 = sign_extend 0
            tmp.82 = add_ptr(tmp.80, index=tmp.81, scale=1)
            tmp.83 = *tmp.82
            if tmp.83 jump or_true_26
            tmp.86 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.87 = add_ptr(tmp.86, index=4L, scale=1)
            tmp.88 = sign_extend 1
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=1)
            tmp.90 = *tmp.89
            if tmp.90 jump or_true_26
            tmp.85 = 0
            jump or_end_27
        
          or_true_26:
            tmp.85 = 1
        
          or_end_27:
            if tmp.85 jump or_true_28
            tmp.93 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.94 = add_ptr(tmp.93, index=4L, scale=1)
            tmp.95 = sign_extend 2
            tmp.96 = add_ptr(tmp.94, index=tmp.95, scale=1)
            tmp.97 = *tmp.96
            if tmp.97 jump or_true_28
            tmp.92 = 0
            jump or_end_29
        
          or_true_28:
            tmp.92 = 1
        
          or_end_29:
            if tmp.92 jump or_true_30
            tmp.100 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.101 = add_ptr(tmp.100, index=8L, scale=1)
            tmp.102 = *tmp.101
            if tmp.102 jump or_true_30
            tmp.99 = 0
            jump or_end_31
        
          or_true_30:
            tmp.99 = 1
        
          or_end_31:
            if tmp.99 jump or_true_32
            tmp.105 = add_ptr(ptr.10, index=32L, scale=1)
            tmp.106 = *tmp.105
            if tmp.106 jump or_true_32
            tmp.104 = 0
            jump or_end_33
        
          or_true_32:
            tmp.104 = 1
        
          or_end_33:
            if tmp.104 jump or_true_34
            tmp.109 = add_ptr(ptr.10, index=40L, scale=1)
            tmp.110 = *tmp.109
            if tmp.110 jump or_true_34
            tmp.108 = 0
            jump or_end_35
        
          or_true_34:
            tmp.108 = 1
        
          or_end_35:
            if tmp.108 jump or_true_36
            tmp.113 = add_ptr(ptr.10, index=40L, scale=1)
            tmp.114 = add_ptr(tmp.113, index=4L, scale=1)
            tmp.115 = *tmp.114
            if tmp.115 jump or_true_36
            tmp.112 = 0
            jump or_end_37
        
          or_true_36:
            tmp.112 = 1
        
          or_end_37:
            if !tmp.112 jump end_if_38
            return 0
        
          end_if_38:
            return 1
            return 0
        }
        global function validate_mixed_initialization(ptr.11) { 
            tmp.116 = *ptr.11
            tmp.118 = sign_extend 200
            tmp.117 = tmp.116 != tmp.118
            if tmp.117 jump or_true_40
            tmp.121 = add_ptr(ptr.11, index=8L, scale=1)
            tmp.122 = *tmp.121
            tmp.123 = tmp.122 != 20
            if tmp.123 jump or_true_40
            tmp.120 = 0
            jump or_end_41
        
          or_true_40:
            tmp.120 = 1
        
          or_end_41:
            if tmp.120 jump or_true_42
            tmp.126 = add_ptr(ptr.11, index=8L, scale=1)
            tmp.127 = add_ptr(tmp.126, index=4L, scale=1)
            tmp.128 = sign_extend 0
            tmp.129 = add_ptr(tmp.127, index=tmp.128, scale=1)
            tmp.130 = *tmp.129
            tmp.131 = zero_extend tmp.130
            tmp.132 = tmp.131 != 21
            if tmp.132 jump or_true_42
            tmp.125 = 0
            jump or_end_43
        
          or_true_42:
            tmp.125 = 1
        
          or_end_43:
            if tmp.125 jump or_true_44
            tmp.135 = add_ptr(ptr.11, index=8L, scale=1)
            tmp.136 = add_ptr(tmp.135, index=8L, scale=1)
            tmp.137 = *tmp.136
            tmp.138 = tmp.137 != 22U
            if tmp.138 jump or_true_44
            tmp.134 = 0
            jump or_end_45
        
          or_true_44:
            tmp.134 = 1
        
          or_end_45:
            if tmp.134 jump or_true_46
            tmp.141 = add_ptr(ptr.11, index=24L, scale=1)
            tmp.142 = *tmp.141
            tmp.143 = &string.2
            tmp.144 = strcmp(tmp.142, tmp.143)
            if tmp.144 jump or_true_46
            tmp.140 = 0
            jump or_end_47
        
          or_true_46:
            tmp.140 = 1
        
          or_end_47:
            if tmp.140 jump or_true_48
            tmp.147 = add_ptr(ptr.11, index=32L, scale=1)
            tmp.148 = *tmp.147
            tmp.149 = tmp.148 != 10D
            if tmp.149 jump or_true_48
            tmp.146 = 0
            jump or_end_49
        
          or_true_48:
            tmp.146 = 1
        
          or_end_49:
            if tmp.146 jump or_true_50
            tmp.152 = add_ptr(ptr.11, index=40L, scale=1)
            tmp.153 = *tmp.152
            tmp.154 = tmp.153 != 99
            if tmp.154 jump or_true_50
            tmp.151 = 0
            jump or_end_51
        
          or_true_50:
            tmp.151 = 1
        
          or_end_51:
            if tmp.151 jump or_true_52
            tmp.157 = add_ptr(ptr.11, index=40L, scale=1)
            tmp.158 = add_ptr(tmp.157, index=4L, scale=1)
            tmp.159 = *tmp.158
            tmp.160 = tmp.159 != 100
            if tmp.160 jump or_true_52
            tmp.156 = 0
            jump or_end_53
        
          or_true_52:
            tmp.156 = 1
        
          or_end_53:
            if !tmp.156 jump end_if_54
            return 0
        
          end_if_54:
            tmp.161 = add_ptr(ptr.11, index=8L, scale=1)
            tmp.162 = add_ptr(tmp.161, index=4L, scale=1)
            tmp.163 = sign_extend 1
            tmp.164 = add_ptr(tmp.162, index=tmp.163, scale=1)
            tmp.165 = *tmp.164
            if tmp.165 jump or_true_56
            tmp.168 = add_ptr(ptr.11, index=8L, scale=1)
            tmp.169 = add_ptr(tmp.168, index=4L, scale=1)
            tmp.170 = sign_extend 2
            tmp.171 = add_ptr(tmp.169, index=tmp.170, scale=1)
            tmp.172 = *tmp.171
            if tmp.172 jump or_true_56
            tmp.167 = 0
            jump or_end_57
        
          or_true_56:
            tmp.167 = 1
        
          or_end_57:
            if !tmp.167 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function validate_array_of_structs(struct_array.12) { 
            tmp.173 = sign_extend 0
            tmp.174 = add_ptr(struct_array.12, index=tmp.173, scale=48)
            tmp.175 = *tmp.174
            tmp.177 = sign_extend 1
            tmp.176 = tmp.175 != tmp.177
            if tmp.176 jump or_true_60
            tmp.180 = sign_extend 0
            tmp.181 = add_ptr(struct_array.12, index=tmp.180, scale=48)
            tmp.182 = add_ptr(tmp.181, index=8L, scale=1)
            tmp.183 = *tmp.182
            tmp.184 = tmp.183 != 2
            if tmp.184 jump or_true_60
            tmp.179 = 0
            jump or_end_61
        
          or_true_60:
            tmp.179 = 1
        
          or_end_61:
            if tmp.179 jump or_true_62
            tmp.187 = sign_extend 0
            tmp.188 = add_ptr(struct_array.12, index=tmp.187, scale=48)
            tmp.189 = add_ptr(tmp.188, index=8L, scale=1)
            tmp.190 = add_ptr(tmp.189, index=4L, scale=1)
            tmp.191 = sign_extend 0
            tmp.192 = add_ptr(tmp.190, index=tmp.191, scale=1)
            tmp.193 = *tmp.192
            tmp.194 = zero_extend tmp.193
            tmp.195 = tmp.194 != 3
            if tmp.195 jump or_true_62
            tmp.186 = 0
            jump or_end_63
        
          or_true_62:
            tmp.186 = 1
        
          or_end_63:
            if tmp.186 jump or_true_64
            tmp.198 = sign_extend 0
            tmp.199 = add_ptr(struct_array.12, index=tmp.198, scale=48)
            tmp.200 = add_ptr(tmp.199, index=8L, scale=1)
            tmp.201 = add_ptr(tmp.200, index=4L, scale=1)
            tmp.202 = sign_extend 1
            tmp.203 = add_ptr(tmp.201, index=tmp.202, scale=1)
            tmp.204 = *tmp.203
            tmp.205 = zero_extend tmp.204
            tmp.206 = tmp.205 != 4
            if tmp.206 jump or_true_64
            tmp.197 = 0
            jump or_end_65
        
          or_true_64:
            tmp.197 = 1
        
          or_end_65:
            if tmp.197 jump or_true_66
            tmp.209 = sign_extend 0
            tmp.210 = add_ptr(struct_array.12, index=tmp.209, scale=48)
            tmp.211 = add_ptr(tmp.210, index=8L, scale=1)
            tmp.212 = add_ptr(tmp.211, index=4L, scale=1)
            tmp.213 = sign_extend 2
            tmp.214 = add_ptr(tmp.212, index=tmp.213, scale=1)
            tmp.215 = *tmp.214
            tmp.216 = zero_extend tmp.215
            tmp.217 = tmp.216 != 5
            if tmp.217 jump or_true_66
            tmp.208 = 0
            jump or_end_67
        
          or_true_66:
            tmp.208 = 1
        
          or_end_67:
            if tmp.208 jump or_true_68
            tmp.220 = sign_extend 0
            tmp.221 = add_ptr(struct_array.12, index=tmp.220, scale=48)
            tmp.222 = add_ptr(tmp.221, index=8L, scale=1)
            tmp.223 = add_ptr(tmp.222, index=8L, scale=1)
            tmp.224 = *tmp.223
            tmp.226 = 6
            tmp.225 = tmp.224 != tmp.226
            if tmp.225 jump or_true_68
            tmp.219 = 0
            jump or_end_69
        
          or_true_68:
            tmp.219 = 1
        
          or_end_69:
            if tmp.219 jump or_true_70
            tmp.229 = sign_extend 0
            tmp.230 = add_ptr(struct_array.12, index=tmp.229, scale=48)
            tmp.231 = add_ptr(tmp.230, index=24L, scale=1)
            tmp.232 = *tmp.231
            tmp.233 = &string.3
            tmp.234 = strcmp(tmp.232, tmp.233)
            if tmp.234 jump or_true_70
            tmp.228 = 0
            jump or_end_71
        
          or_true_70:
            tmp.228 = 1
        
          or_end_71:
            if tmp.228 jump or_true_72
            tmp.237 = sign_extend 0
            tmp.238 = add_ptr(struct_array.12, index=tmp.237, scale=48)
            tmp.239 = add_ptr(tmp.238, index=32L, scale=1)
            tmp.240 = *tmp.239
            tmp.241 = tmp.240 != 8D
            if tmp.241 jump or_true_72
            tmp.236 = 0
            jump or_end_73
        
          or_true_72:
            tmp.236 = 1
        
          or_end_73:
            if tmp.236 jump or_true_74
            tmp.244 = sign_extend 0
            tmp.245 = add_ptr(struct_array.12, index=tmp.244, scale=48)
            tmp.246 = add_ptr(tmp.245, index=40L, scale=1)
            tmp.247 = *tmp.246
            tmp.248 = tmp.247 != 9
            if tmp.248 jump or_true_74
            tmp.243 = 0
            jump or_end_75
        
          or_true_74:
            tmp.243 = 1
        
          or_end_75:
            if tmp.243 jump or_true_76
            tmp.251 = sign_extend 0
            tmp.252 = add_ptr(struct_array.12, index=tmp.251, scale=48)
            tmp.253 = add_ptr(tmp.252, index=40L, scale=1)
            tmp.254 = add_ptr(tmp.253, index=4L, scale=1)
            tmp.255 = *tmp.254
            tmp.256 = tmp.255 != 10
            if tmp.256 jump or_true_76
            tmp.250 = 0
            jump or_end_77
        
          or_true_76:
            tmp.250 = 1
        
          or_end_77:
            if !tmp.250 jump end_if_78
            return 0
        
          end_if_78:
            tmp.257 = sign_extend 1
            tmp.258 = add_ptr(struct_array.12, index=tmp.257, scale=48)
            tmp.259 = *tmp.258
            tmp.261 = sign_extend 101
            tmp.260 = tmp.259 != tmp.261
            if tmp.260 jump or_true_80
            tmp.264 = sign_extend 1
            tmp.265 = add_ptr(struct_array.12, index=tmp.264, scale=48)
            tmp.266 = add_ptr(tmp.265, index=8L, scale=1)
            tmp.267 = *tmp.266
            tmp.268 = tmp.267 != 102
            if tmp.268 jump or_true_80
            tmp.263 = 0
            jump or_end_81
        
          or_true_80:
            tmp.263 = 1
        
          or_end_81:
            if tmp.263 jump or_true_82
            tmp.271 = sign_extend 1
            tmp.272 = add_ptr(struct_array.12, index=tmp.271, scale=48)
            tmp.273 = add_ptr(tmp.272, index=8L, scale=1)
            tmp.274 = add_ptr(tmp.273, index=4L, scale=1)
            tmp.275 = sign_extend 0
            tmp.276 = add_ptr(tmp.274, index=tmp.275, scale=1)
            tmp.277 = *tmp.276
            tmp.278 = zero_extend tmp.277
            tmp.279 = tmp.278 != 103
            if tmp.279 jump or_true_82
            tmp.270 = 0
            jump or_end_83
        
          or_true_82:
            tmp.270 = 1
        
          or_end_83:
            if tmp.270 jump or_true_84
            tmp.282 = sign_extend 1
            tmp.283 = add_ptr(struct_array.12, index=tmp.282, scale=48)
            tmp.284 = add_ptr(tmp.283, index=8L, scale=1)
            tmp.285 = add_ptr(tmp.284, index=4L, scale=1)
            tmp.286 = sign_extend 1
            tmp.287 = add_ptr(tmp.285, index=tmp.286, scale=1)
            tmp.288 = *tmp.287
            tmp.289 = zero_extend tmp.288
            tmp.290 = tmp.289 != 104
            if tmp.290 jump or_true_84
            tmp.281 = 0
            jump or_end_85
        
          or_true_84:
            tmp.281 = 1
        
          or_end_85:
            if tmp.281 jump or_true_86
            tmp.293 = sign_extend 1
            tmp.294 = add_ptr(struct_array.12, index=tmp.293, scale=48)
            tmp.295 = add_ptr(tmp.294, index=8L, scale=1)
            tmp.296 = add_ptr(tmp.295, index=4L, scale=1)
            tmp.297 = sign_extend 2
            tmp.298 = add_ptr(tmp.296, index=tmp.297, scale=1)
            tmp.299 = *tmp.298
            tmp.300 = zero_extend tmp.299
            tmp.301 = tmp.300 != 105
            if tmp.301 jump or_true_86
            tmp.292 = 0
            jump or_end_87
        
          or_true_86:
            tmp.292 = 1
        
          or_end_87:
            if tmp.292 jump or_true_88
            tmp.304 = sign_extend 1
            tmp.305 = add_ptr(struct_array.12, index=tmp.304, scale=48)
            tmp.306 = add_ptr(tmp.305, index=8L, scale=1)
            tmp.307 = add_ptr(tmp.306, index=8L, scale=1)
            tmp.308 = *tmp.307
            tmp.310 = 106
            tmp.309 = tmp.308 != tmp.310
            if tmp.309 jump or_true_88
            tmp.303 = 0
            jump or_end_89
        
          or_true_88:
            tmp.303 = 1
        
          or_end_89:
            if tmp.303 jump or_true_90
            tmp.313 = sign_extend 1
            tmp.314 = add_ptr(struct_array.12, index=tmp.313, scale=48)
            tmp.315 = add_ptr(tmp.314, index=24L, scale=1)
            tmp.316 = *tmp.315
            tmp.317 = &string.4
            tmp.318 = strcmp(tmp.316, tmp.317)
            if tmp.318 jump or_true_90
            tmp.312 = 0
            jump or_end_91
        
          or_true_90:
            tmp.312 = 1
        
          or_end_91:
            if tmp.312 jump or_true_92
            tmp.321 = sign_extend 1
            tmp.322 = add_ptr(struct_array.12, index=tmp.321, scale=48)
            tmp.323 = add_ptr(tmp.322, index=32L, scale=1)
            tmp.324 = *tmp.323
            tmp.325 = tmp.324 != 108D
            if tmp.325 jump or_true_92
            tmp.320 = 0
            jump or_end_93
        
          or_true_92:
            tmp.320 = 1
        
          or_end_93:
            if tmp.320 jump or_true_94
            tmp.328 = sign_extend 1
            tmp.329 = add_ptr(struct_array.12, index=tmp.328, scale=48)
            tmp.330 = add_ptr(tmp.329, index=40L, scale=1)
            tmp.331 = *tmp.330
            tmp.332 = tmp.331 != 109
            if tmp.332 jump or_true_94
            tmp.327 = 0
            jump or_end_95
        
          or_true_94:
            tmp.327 = 1
        
          or_end_95:
            if tmp.327 jump or_true_96
            tmp.335 = sign_extend 1
            tmp.336 = add_ptr(struct_array.12, index=tmp.335, scale=48)
            tmp.337 = add_ptr(tmp.336, index=40L, scale=1)
            tmp.338 = add_ptr(tmp.337, index=4L, scale=1)
            tmp.339 = *tmp.338
            tmp.340 = tmp.339 != 110
            if tmp.340 jump or_true_96
            tmp.334 = 0
            jump or_end_97
        
          or_true_96:
            tmp.334 = 1
        
          or_end_97:
            if !tmp.334 jump end_if_98
            return 0
        
          end_if_98:
            tmp.341 = sign_extend 2
            tmp.342 = add_ptr(struct_array.12, index=tmp.341, scale=48)
            tmp.343 = *tmp.342
            tmp.345 = sign_extend 201
            tmp.344 = tmp.343 != tmp.345
            if tmp.344 jump or_true_100
            tmp.348 = sign_extend 2
            tmp.349 = add_ptr(struct_array.12, index=tmp.348, scale=48)
            tmp.350 = add_ptr(tmp.349, index=8L, scale=1)
            tmp.351 = *tmp.350
            tmp.352 = tmp.351 != 202
            if tmp.352 jump or_true_100
            tmp.347 = 0
            jump or_end_101
        
          or_true_100:
            tmp.347 = 1
        
          or_end_101:
            if tmp.347 jump or_true_102
            tmp.355 = sign_extend 2
            tmp.356 = add_ptr(struct_array.12, index=tmp.355, scale=48)
            tmp.357 = add_ptr(tmp.356, index=8L, scale=1)
            tmp.358 = add_ptr(tmp.357, index=4L, scale=1)
            tmp.359 = sign_extend 0
            tmp.360 = add_ptr(tmp.358, index=tmp.359, scale=1)
            tmp.361 = *tmp.360
            tmp.362 = zero_extend tmp.361
            tmp.363 = tmp.362 != 203
            if tmp.363 jump or_true_102
            tmp.354 = 0
            jump or_end_103
        
          or_true_102:
            tmp.354 = 1
        
          or_end_103:
            if tmp.354 jump or_true_104
            tmp.366 = sign_extend 2
            tmp.367 = add_ptr(struct_array.12, index=tmp.366, scale=48)
            tmp.368 = add_ptr(tmp.367, index=8L, scale=1)
            tmp.369 = add_ptr(tmp.368, index=4L, scale=1)
            tmp.370 = sign_extend 1
            tmp.371 = add_ptr(tmp.369, index=tmp.370, scale=1)
            tmp.372 = *tmp.371
            if tmp.372 jump or_true_104
            tmp.365 = 0
            jump or_end_105
        
          or_true_104:
            tmp.365 = 1
        
          or_end_105:
            if tmp.365 jump or_true_106
            tmp.375 = sign_extend 2
            tmp.376 = add_ptr(struct_array.12, index=tmp.375, scale=48)
            tmp.377 = add_ptr(tmp.376, index=8L, scale=1)
            tmp.378 = add_ptr(tmp.377, index=4L, scale=1)
            tmp.379 = sign_extend 2
            tmp.380 = add_ptr(tmp.378, index=tmp.379, scale=1)
            tmp.381 = *tmp.380
            if tmp.381 jump or_true_106
            tmp.374 = 0
            jump or_end_107
        
          or_true_106:
            tmp.374 = 1
        
          or_end_107:
            if tmp.374 jump or_true_108
            tmp.384 = sign_extend 2
            tmp.385 = add_ptr(struct_array.12, index=tmp.384, scale=48)
            tmp.386 = add_ptr(tmp.385, index=8L, scale=1)
            tmp.387 = add_ptr(tmp.386, index=8L, scale=1)
            tmp.388 = *tmp.387
            if tmp.388 jump or_true_108
            tmp.383 = 0
            jump or_end_109
        
          or_true_108:
            tmp.383 = 1
        
          or_end_109:
            if tmp.383 jump or_true_110
            tmp.391 = sign_extend 2
            tmp.392 = add_ptr(struct_array.12, index=tmp.391, scale=48)
            tmp.393 = add_ptr(tmp.392, index=24L, scale=1)
            tmp.394 = *tmp.393
            tmp.395 = &string.5
            tmp.396 = strcmp(tmp.394, tmp.395)
            if tmp.396 jump or_true_110
            tmp.390 = 0
            jump or_end_111
        
          or_true_110:
            tmp.390 = 1
        
          or_end_111:
            if tmp.390 jump or_true_112
            tmp.399 = sign_extend 2
            tmp.400 = add_ptr(struct_array.12, index=tmp.399, scale=48)
            tmp.401 = add_ptr(tmp.400, index=32L, scale=1)
            tmp.402 = *tmp.401
            tmp.403 = tmp.402 != 208D
            if tmp.403 jump or_true_112
            tmp.398 = 0
            jump or_end_113
        
          or_true_112:
            tmp.398 = 1
        
          or_end_113:
            if tmp.398 jump or_true_114
            tmp.406 = sign_extend 2
            tmp.407 = add_ptr(struct_array.12, index=tmp.406, scale=48)
            tmp.408 = add_ptr(tmp.407, index=40L, scale=1)
            tmp.409 = *tmp.408
            tmp.410 = tmp.409 != 209
            if tmp.410 jump or_true_114
            tmp.405 = 0
            jump or_end_115
        
          or_true_114:
            tmp.405 = 1
        
          or_end_115:
            if tmp.405 jump or_true_116
            tmp.413 = sign_extend 2
            tmp.414 = add_ptr(struct_array.12, index=tmp.413, scale=48)
            tmp.415 = add_ptr(tmp.414, index=40L, scale=1)
            tmp.416 = add_ptr(tmp.415, index=4L, scale=1)
            tmp.417 = *tmp.416
            if tmp.417 jump or_true_116
            tmp.412 = 0
            jump or_end_117
        
          or_true_116:
            tmp.412 = 1
        
          or_end_117:
            if !tmp.412 jump end_if_118
            return 0
        
          end_if_118:
            tmp.418 = sign_extend 3
            tmp.419 = add_ptr(struct_array.12, index=tmp.418, scale=48)
            tmp.420 = *tmp.419
            tmp.422 = sign_extend 301
            tmp.421 = tmp.420 != tmp.422
            if tmp.421 jump or_true_120
            tmp.425 = sign_extend 3
            tmp.426 = add_ptr(struct_array.12, index=tmp.425, scale=48)
            tmp.427 = add_ptr(tmp.426, index=8L, scale=1)
            tmp.428 = *tmp.427
            if tmp.428 jump or_true_120
            tmp.424 = 0
            jump or_end_121
        
          or_true_120:
            tmp.424 = 1
        
          or_end_121:
            if tmp.424 jump or_true_122
            tmp.431 = sign_extend 3
            tmp.432 = add_ptr(struct_array.12, index=tmp.431, scale=48)
            tmp.433 = add_ptr(tmp.432, index=8L, scale=1)
            tmp.434 = add_ptr(tmp.433, index=4L, scale=1)
            tmp.435 = sign_extend 0
            tmp.436 = add_ptr(tmp.434, index=tmp.435, scale=1)
            tmp.437 = *tmp.436
            if tmp.437 jump or_true_122
            tmp.430 = 0
            jump or_end_123
        
          or_true_122:
            tmp.430 = 1
        
          or_end_123:
            if tmp.430 jump or_true_124
            tmp.440 = sign_extend 3
            tmp.441 = add_ptr(struct_array.12, index=tmp.440, scale=48)
            tmp.442 = add_ptr(tmp.441, index=8L, scale=1)
            tmp.443 = add_ptr(tmp.442, index=4L, scale=1)
            tmp.444 = sign_extend 1
            tmp.445 = add_ptr(tmp.443, index=tmp.444, scale=1)
            tmp.446 = *tmp.445
            if tmp.446 jump or_true_124
            tmp.439 = 0
            jump or_end_125
        
          or_true_124:
            tmp.439 = 1
        
          or_end_125:
            if tmp.439 jump or_true_126
            tmp.449 = sign_extend 3
            tmp.450 = add_ptr(struct_array.12, index=tmp.449, scale=48)
            tmp.451 = add_ptr(tmp.450, index=8L, scale=1)
            tmp.452 = add_ptr(tmp.451, index=4L, scale=1)
            tmp.453 = sign_extend 2
            tmp.454 = add_ptr(tmp.452, index=tmp.453, scale=1)
            tmp.455 = *tmp.454
            if tmp.455 jump or_true_126
            tmp.448 = 0
            jump or_end_127
        
          or_true_126:
            tmp.448 = 1
        
          or_end_127:
            if tmp.448 jump or_true_128
            tmp.458 = sign_extend 3
            tmp.459 = add_ptr(struct_array.12, index=tmp.458, scale=48)
            tmp.460 = add_ptr(tmp.459, index=8L, scale=1)
            tmp.461 = add_ptr(tmp.460, index=8L, scale=1)
            tmp.462 = *tmp.461
            if tmp.462 jump or_true_128
            tmp.457 = 0
            jump or_end_129
        
          or_true_128:
            tmp.457 = 1
        
          or_end_129:
            if tmp.457 jump or_true_130
            tmp.465 = sign_extend 3
            tmp.466 = add_ptr(struct_array.12, index=tmp.465, scale=48)
            tmp.467 = add_ptr(tmp.466, index=24L, scale=1)
            tmp.468 = *tmp.467
            if tmp.468 jump or_true_130
            tmp.464 = 0
            jump or_end_131
        
          or_true_130:
            tmp.464 = 1
        
          or_end_131:
            if tmp.464 jump or_true_132
            tmp.471 = sign_extend 3
            tmp.472 = add_ptr(struct_array.12, index=tmp.471, scale=48)
            tmp.473 = add_ptr(tmp.472, index=32L, scale=1)
            tmp.474 = *tmp.473
            if tmp.474 jump or_true_132
            tmp.470 = 0
            jump or_end_133
        
          or_true_132:
            tmp.470 = 1
        
          or_end_133:
            if tmp.470 jump or_true_134
            tmp.477 = sign_extend 3
            tmp.478 = add_ptr(struct_array.12, index=tmp.477, scale=48)
            tmp.479 = add_ptr(tmp.478, index=40L, scale=1)
            tmp.480 = *tmp.479
            if tmp.480 jump or_true_134
            tmp.476 = 0
            jump or_end_135
        
          or_true_134:
            tmp.476 = 1
        
          or_end_135:
            if tmp.476 jump or_true_136
            tmp.483 = sign_extend 3
            tmp.484 = add_ptr(struct_array.12, index=tmp.483, scale=48)
            tmp.485 = add_ptr(tmp.484, index=40L, scale=1)
            tmp.486 = add_ptr(tmp.485, index=4L, scale=1)
            tmp.487 = *tmp.486
            if tmp.487 jump or_true_136
            tmp.482 = 0
            jump or_end_137
        
          or_true_136:
            tmp.482 = 1
        
          or_end_137:
            if !tmp.482 jump end_if_138
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
            full.9[0] = tmp.1
            tmp.2 = - 171L
            tmp.3 = truncate tmp.2
            full.9[8] = tmp.3
            tmp.4 = - 56
            tmp.5 = truncate tmp.4
            full.9[12] = tmp.5
            tmp.6 = - 54
            tmp.7 = truncate tmp.6
            full.9[13] = tmp.7
            tmp.8 = - 53
            tmp.9 = truncate tmp.8
            full.9[14] = tmp.9
            tmp.10 = double_to_uint 40.5D
            full.9[16] = tmp.10
            tmp.11 = &string.0
            full.9[24] = tmp.11
            tmp.12 = - 22
            tmp.13 = int_to_double tmp.12
            full.9[32] = tmp.13
            full.9[40] = 1
            full.9[44] = 2
            tmp.14 = &full.9
            tmp.15 = validate_full_initialization(tmp.14)
            return tmp.15
            return 0
        }
        global function test_partial_initialization() { 
            tmp.16 = sign_extend 1000
            partial.10[0] = tmp.16
            partial.10[8] = 1
            partial.10[12] = 0UC
            partial.10[13] = 0UC
            partial.10[14] = 0UC
            partial.10[16] = 0U
            tmp.17 = &string.1
            partial.10[24] = tmp.17
            partial.10[32] = 0D
            partial.10[40] = 0
            partial.10[44] = 0
            tmp.18 = &partial.10
            tmp.19 = validate_partial_initialization(tmp.18)
            return tmp.19
            return 0
        }
        global function test_mixed_initialization() { 
            inner1.11[0] = 10
            inner1.11[4] = 0UC
            inner1.11[5] = 0UC
            inner1.11[6] = 0UC
            inner1.11[8] = 0U
            inner2.12[0] = 20
            tmp.20 = truncate 21
            inner2.12[4] = tmp.20
            inner2.12[5] = 0UC
            inner2.12[6] = 0UC
            inner2.12[8] = 22U
            tmp.21 = sign_extend 200
            mixed.14[0] = tmp.21
            if !flag.13 jump else_1
            tmp.22 = inner1.11
            jump end_if_0
        
          else_1:
            tmp.22 = inner2.12
        
          end_if_0:
            mixed.14[8] = tmp.22
            tmp.23 = &string.2
            mixed.14[24] = tmp.23
            mixed.14[32] = 10D
            mixed.14[40] = 99
            mixed.14[44] = 100
            tmp.24 = &mixed.14
            tmp.25 = validate_mixed_initialization(tmp.24)
            return tmp.25
            return 0
        }
        global function test_array_of_structs() { 
            tmp.26 = sign_extend 1
            s0.15[0] = tmp.26
            s0.15[8] = 2
            tmp.27 = truncate 3
            s0.15[12] = tmp.27
            tmp.28 = truncate 4
            s0.15[13] = tmp.28
            tmp.29 = truncate 5
            s0.15[14] = tmp.29
            tmp.30 = 6
            s0.15[16] = tmp.30
            tmp.31 = &string.3
            s0.15[24] = tmp.31
            s0.15[32] = 8D
            s0.15[40] = 9
            s0.15[44] = 10
            in1.16[0] = 102
            tmp.32 = truncate 103
            in1.16[4] = tmp.32
            tmp.33 = truncate 104
            in1.16[5] = tmp.33
            tmp.34 = truncate 105
            in1.16[6] = tmp.34
            tmp.35 = 106
            in1.16[8] = tmp.35
            pair1.17[0] = 109
            pair1.17[4] = 110
            pair2.18[0] = 209
            pair2.18[4] = 0
            tmp.36 = sign_extend 301
            s3.19[0] = tmp.36
            s3.19[8] = 0
            s3.19[12] = 0UC
            s3.19[13] = 0UC
            s3.19[14] = 0UC
            s3.19[16] = 0U
            s3.19[24] = 0UL
            s3.19[32] = 0D
            s3.19[40] = 0
            s3.19[44] = 0
            struct_array.20[0] = s0.15
            tmp.37 = sign_extend 101
            struct_array.20[48] = tmp.37
            struct_array.20[56] = in1.16
            tmp.38 = &string.4
            struct_array.20[72] = tmp.38
            struct_array.20[80] = 108D
            struct_array.20[88] = pair1.17
            tmp.39 = sign_extend 201
            struct_array.20[96] = tmp.39
            struct_array.20[104] = 202
            tmp.40 = truncate 203
            struct_array.20[108] = tmp.40
            struct_array.20[109] = 0UC
            struct_array.20[110] = 0UC
            struct_array.20[112] = 0U
            tmp.41 = &string.5
            struct_array.20[120] = tmp.41
            struct_array.20[128] = 208D
            struct_array.20[136] = pair2.18
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
            tmp.3 = all_zeros[24]
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if tmp.2 jump or_true_2
            tmp.6 = all_zeros[32]
            if tmp.6 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            if !tmp.5 jump end_if_4
            return 0
        
          end_if_4:
            tmp.7 = all_zeros[8]
            if tmp.7 jump or_true_6
            tmp.10 = &all_zeros
            tmp.10 = add_ptr(tmp.10, index=12L, scale=1)
            tmp.11 = sign_extend 0
            tmp.12 = add_ptr(tmp.10, index=tmp.11, scale=1)
            tmp.13 = *tmp.12
            if tmp.13 jump or_true_6
            tmp.9 = 0
            jump or_end_7
        
          or_true_6:
            tmp.9 = 1
        
          or_end_7:
            if tmp.9 jump or_true_8
            tmp.16 = &all_zeros
            tmp.16 = add_ptr(tmp.16, index=12L, scale=1)
            tmp.17 = sign_extend 1
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=1)
            tmp.19 = *tmp.18
            if tmp.19 jump or_true_8
            tmp.15 = 0
            jump or_end_9
        
          or_true_8:
            tmp.15 = 1
        
          or_end_9:
            if tmp.15 jump or_true_10
            tmp.22 = &all_zeros
            tmp.22 = add_ptr(tmp.22, index=12L, scale=1)
            tmp.23 = sign_extend 2
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=1)
            tmp.25 = *tmp.24
            if tmp.25 jump or_true_10
            tmp.21 = 0
            jump or_end_11
        
          or_true_10:
            tmp.21 = 1
        
          or_end_11:
            if tmp.21 jump or_true_12
            tmp.28 = all_zeros[16]
            if tmp.28 jump or_true_12
            tmp.27 = 0
            jump or_end_13
        
          or_true_12:
            tmp.27 = 1
        
          or_end_13:
            if !tmp.27 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_partially_initialized() { 
            tmp.29 = partial[0]
            tmp.30 = tmp.29 != 100L
            if tmp.30 jump or_true_16
            tmp.33 = partial[24]
            tmp.34 = &string.0
            tmp.35 = strcmp(tmp.33, tmp.34)
            if tmp.35 jump or_true_16
            tmp.32 = 0
            jump or_end_17
        
          or_true_16:
            tmp.32 = 1
        
          or_end_17:
            if !tmp.32 jump end_if_18
            return 0
        
          end_if_18:
            tmp.36 = partial[32]
            tmp.37 = tmp.36 != 0D
            if !tmp.37 jump end_if_20
            return 0
        
          end_if_20:
            tmp.38 = partial[8]
            tmp.39 = tmp.38 != 10
            if tmp.39 jump or_true_22
            tmp.42 = &partial
            tmp.42 = add_ptr(tmp.42, index=12L, scale=1)
            tmp.43 = sign_extend 0
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=1)
            tmp.45 = *tmp.44
            tmp.46 = sign_extend tmp.45
            tmp.47 = tmp.46 != 10
            if tmp.47 jump or_true_22
            tmp.41 = 0
            jump or_end_23
        
          or_true_22:
            tmp.41 = 1
        
          or_end_23:
            if !tmp.41 jump end_if_24
            return 0
        
          end_if_24:
            tmp.48 = &partial
            tmp.48 = add_ptr(tmp.48, index=12L, scale=1)
            tmp.49 = sign_extend 1
            tmp.50 = add_ptr(tmp.48, index=tmp.49, scale=1)
            tmp.51 = *tmp.50
            if tmp.51 jump or_true_26
            tmp.54 = &partial
            tmp.54 = add_ptr(tmp.54, index=12L, scale=1)
            tmp.55 = sign_extend 2
            tmp.56 = add_ptr(tmp.54, index=tmp.55, scale=1)
            tmp.57 = *tmp.56
            if tmp.57 jump or_true_26
            tmp.53 = 0
            jump or_end_27
        
          or_true_26:
            tmp.53 = 1
        
          or_end_27:
            if tmp.53 jump or_true_28
            tmp.60 = partial[16]
            if tmp.60 jump or_true_28
            tmp.59 = 0
            jump or_end_29
        
          or_true_28:
            tmp.59 = 1
        
          or_end_29:
            if !tmp.59 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_fully_intialized() { 
            tmp.61 = full[0]
            tmp.62 = tmp.61 != 18014398509481979L
            if tmp.62 jump or_true_32
            tmp.65 = full[24]
            tmp.66 = &string.1
            tmp.67 = strcmp(tmp.65, tmp.66)
            if tmp.67 jump or_true_32
            tmp.64 = 0
            jump or_end_33
        
          or_true_32:
            tmp.64 = 1
        
          or_end_33:
            if tmp.64 jump or_true_34
            tmp.70 = full[32]
            tmp.71 = tmp.70 != 2000000000000D
            if tmp.71 jump or_true_34
            tmp.69 = 0
            jump or_end_35
        
          or_true_34:
            tmp.69 = 1
        
          or_end_35:
            if !tmp.69 jump end_if_36
            return 0
        
          end_if_36:
            tmp.72 = full[8]
            tmp.73 = tmp.72 != 1000
            if tmp.73 jump or_true_38
            tmp.76 = &full
            tmp.76 = add_ptr(tmp.76, index=12L, scale=1)
            tmp.77 = sign_extend 0
            tmp.78 = add_ptr(tmp.76, index=tmp.77, scale=1)
            tmp.79 = *tmp.78
            tmp.80 = sign_extend tmp.79
            tmp.81 = tmp.80 != 111
            if tmp.81 jump or_true_38
            tmp.75 = 0
            jump or_end_39
        
          or_true_38:
            tmp.75 = 1
        
          or_end_39:
            if tmp.75 jump or_true_40
            tmp.84 = &full
            tmp.84 = add_ptr(tmp.84, index=12L, scale=1)
            tmp.85 = sign_extend 1
            tmp.86 = add_ptr(tmp.84, index=tmp.85, scale=1)
            tmp.87 = *tmp.86
            tmp.88 = sign_extend tmp.87
            tmp.89 = tmp.88 != 107
            if tmp.89 jump or_true_40
            tmp.83 = 0
            jump or_end_41
        
          or_true_40:
            tmp.83 = 1
        
          or_end_41:
            if tmp.83 jump or_true_42
            tmp.92 = &full
            tmp.92 = add_ptr(tmp.92, index=12L, scale=1)
            tmp.93 = sign_extend 2
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=1)
            tmp.95 = *tmp.94
            tmp.96 = sign_extend tmp.95
            tmp.97 = tmp.96 != 0
            if tmp.97 jump or_true_42
            tmp.91 = 0
            jump or_end_43
        
          or_true_42:
            tmp.91 = 1
        
          or_end_43:
            if tmp.91 jump or_true_44
            tmp.100 = full[16]
            tmp.101 = tmp.100 != 4292870144U
            if tmp.101 jump or_true_44
            tmp.99 = 0
            jump or_end_45
        
          or_true_44:
            tmp.99 = 1
        
          or_end_45:
            if !tmp.99 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function test_implicit_conversions() { 
            tmp.102 = converted[0]
            tmp.103 = tmp.102 != 10L
            if tmp.103 jump or_true_48
            tmp.106 = converted[24]
            tmp.108 = sign_extend 0
            tmp.107 = tmp.106 != tmp.108
            if tmp.107 jump or_true_48
            tmp.105 = 0
            jump or_end_49
        
          or_true_48:
            tmp.105 = 1
        
          or_end_49:
            if tmp.105 jump or_true_50
            tmp.111 = converted[32]
            tmp.112 = tmp.111 != 9223372036854778000D
            if tmp.112 jump or_true_50
            tmp.110 = 0
            jump or_end_51
        
          or_true_50:
            tmp.110 = 1
        
          or_end_51:
            if !tmp.110 jump end_if_52
            return 0
        
          end_if_52:
            tmp.113 = converted[8]
            tmp.115 = - 2147483646
            tmp.114 = tmp.113 != tmp.115
            if tmp.114 jump or_true_54
            tmp.118 = &converted
            tmp.118 = add_ptr(tmp.118, index=12L, scale=1)
            tmp.119 = sign_extend 0
            tmp.120 = add_ptr(tmp.118, index=tmp.119, scale=1)
            tmp.121 = *tmp.120
            tmp.122 = sign_extend tmp.121
            tmp.123 = tmp.122 != 15
            if tmp.123 jump or_true_54
            tmp.117 = 0
            jump or_end_55
        
          or_true_54:
            tmp.117 = 1
        
          or_end_55:
            if tmp.117 jump or_true_56
            tmp.126 = &converted
            tmp.126 = add_ptr(tmp.126, index=12L, scale=1)
            tmp.127 = sign_extend 1
            tmp.128 = add_ptr(tmp.126, index=tmp.127, scale=1)
            tmp.129 = *tmp.128
            tmp.130 = sign_extend tmp.129
            tmp.131 = tmp.130 != 3
            if tmp.131 jump or_true_56
            tmp.125 = 0
            jump or_end_57
        
          or_true_56:
            tmp.125 = 1
        
          or_end_57:
            if tmp.125 jump or_true_58
            tmp.134 = &converted
            tmp.134 = add_ptr(tmp.134, index=12L, scale=1)
            tmp.135 = sign_extend 2
            tmp.136 = add_ptr(tmp.134, index=tmp.135, scale=1)
            tmp.137 = *tmp.136
            tmp.138 = sign_extend tmp.137
            tmp.140 = - 127
            tmp.139 = tmp.138 != tmp.140
            if tmp.139 jump or_true_58
            tmp.133 = 0
            jump or_end_59
        
          or_true_58:
            tmp.133 = 1
        
          or_end_59:
            if tmp.133 jump or_true_60
            tmp.143 = converted[16]
            tmp.144 = tmp.143 != 2147483648U
            if tmp.144 jump or_true_60
            tmp.142 = 0
            jump or_end_61
        
          or_true_60:
            tmp.142 = 1
        
          or_end_61:
            if !tmp.142 jump end_if_62
            return 0
        
          end_if_62:
            return 1
            return 0
        }
        global function test_array_of_structs() { 
            tmp.145 = &struct_array
            tmp.146 = sign_extend 0
            tmp.147 = add_ptr(tmp.145, index=tmp.146, scale=40)
            tmp.148 = *tmp.147
            tmp.150 = sign_extend 1
            tmp.149 = tmp.148 != tmp.150
            if tmp.149 jump or_true_64
            tmp.153 = &struct_array
            tmp.154 = sign_extend 0
            tmp.155 = add_ptr(tmp.153, index=tmp.154, scale=40)
            tmp.156 = add_ptr(tmp.155, index=24L, scale=1)
            tmp.157 = *tmp.156
            tmp.159 = sign_extend 0
            tmp.158 = tmp.157 != tmp.159
            if tmp.158 jump or_true_64
            tmp.152 = 0
            jump or_end_65
        
          or_true_64:
            tmp.152 = 1
        
          or_end_65:
            if tmp.152 jump or_true_66
            tmp.162 = &struct_array
            tmp.163 = sign_extend 0
            tmp.164 = add_ptr(tmp.162, index=tmp.163, scale=40)
            tmp.165 = add_ptr(tmp.164, index=32L, scale=1)
            tmp.166 = *tmp.165
            tmp.168 = int_to_double 5
            tmp.167 = tmp.166 != tmp.168
            if tmp.167 jump or_true_66
            tmp.161 = 0
            jump or_end_67
        
          or_true_66:
            tmp.161 = 1
        
          or_end_67:
            if !tmp.161 jump end_if_68
            return 0
        
          end_if_68:
            tmp.169 = &struct_array
            tmp.170 = sign_extend 0
            tmp.171 = add_ptr(tmp.169, index=tmp.170, scale=40)
            tmp.172 = add_ptr(tmp.171, index=8L, scale=1)
            tmp.173 = *tmp.172
            tmp.174 = tmp.173 != 2
            if tmp.174 jump or_true_70
            tmp.177 = &struct_array
            tmp.178 = sign_extend 0
            tmp.179 = add_ptr(tmp.177, index=tmp.178, scale=40)
            tmp.180 = add_ptr(tmp.179, index=8L, scale=1)
            tmp.181 = add_ptr(tmp.180, index=4L, scale=1)
            tmp.182 = tmp.181
            tmp.183 = &string.2
            tmp.184 = strcmp(tmp.182, tmp.183)
            if tmp.184 jump or_true_70
            tmp.176 = 0
            jump or_end_71
        
          or_true_70:
            tmp.176 = 1
        
          or_end_71:
            if tmp.176 jump or_true_72
            tmp.187 = &struct_array
            tmp.188 = sign_extend 0
            tmp.189 = add_ptr(tmp.187, index=tmp.188, scale=40)
            tmp.190 = add_ptr(tmp.189, index=8L, scale=1)
            tmp.191 = add_ptr(tmp.190, index=8L, scale=1)
            tmp.192 = *tmp.191
            tmp.194 = 3
            tmp.193 = tmp.192 != tmp.194
            if tmp.193 jump or_true_72
            tmp.186 = 0
            jump or_end_73
        
          or_true_72:
            tmp.186 = 1
        
          or_end_73:
            if !tmp.186 jump end_if_74
            return 0
        
          end_if_74:
            tmp.195 = &struct_array
            tmp.196 = sign_extend 1
            tmp.197 = add_ptr(tmp.195, index=tmp.196, scale=40)
            tmp.198 = *tmp.197
            tmp.200 = sign_extend 6
            tmp.199 = tmp.198 != tmp.200
            if tmp.199 jump or_true_76
            tmp.203 = &struct_array
            tmp.204 = sign_extend 1
            tmp.205 = add_ptr(tmp.203, index=tmp.204, scale=40)
            tmp.206 = add_ptr(tmp.205, index=24L, scale=1)
            tmp.207 = *tmp.206
            tmp.208 = &string.3
            tmp.209 = strcmp(tmp.207, tmp.208)
            if tmp.209 jump or_true_76
            tmp.202 = 0
            jump or_end_77
        
          or_true_76:
            tmp.202 = 1
        
          or_end_77:
            if tmp.202 jump or_true_78
            tmp.212 = &struct_array
            tmp.213 = sign_extend 1
            tmp.214 = add_ptr(tmp.212, index=tmp.213, scale=40)
            tmp.215 = add_ptr(tmp.214, index=32L, scale=1)
            tmp.216 = *tmp.215
            tmp.218 = int_to_double 9
            tmp.217 = tmp.216 != tmp.218
            if tmp.217 jump or_true_78
            tmp.211 = 0
            jump or_end_79
        
          or_true_78:
            tmp.211 = 1
        
          or_end_79:
            if !tmp.211 jump end_if_80
            return 0
        
          end_if_80:
            tmp.219 = &struct_array
            tmp.220 = sign_extend 1
            tmp.221 = add_ptr(tmp.219, index=tmp.220, scale=40)
            tmp.222 = add_ptr(tmp.221, index=8L, scale=1)
            tmp.223 = *tmp.222
            tmp.224 = tmp.223 != 7
            if tmp.224 jump or_true_82
            tmp.227 = &struct_array
            tmp.228 = sign_extend 1
            tmp.229 = add_ptr(tmp.227, index=tmp.228, scale=40)
            tmp.230 = add_ptr(tmp.229, index=8L, scale=1)
            tmp.231 = add_ptr(tmp.230, index=4L, scale=1)
            tmp.232 = tmp.231
            tmp.233 = &string.4
            tmp.234 = strcmp(tmp.232, tmp.233)
            if tmp.234 jump or_true_82
            tmp.226 = 0
            jump or_end_83
        
          or_true_82:
            tmp.226 = 1
        
          or_end_83:
            if tmp.226 jump or_true_84
            tmp.237 = &struct_array
            tmp.238 = sign_extend 1
            tmp.239 = add_ptr(tmp.237, index=tmp.238, scale=40)
            tmp.240 = add_ptr(tmp.239, index=8L, scale=1)
            tmp.241 = add_ptr(tmp.240, index=8L, scale=1)
            tmp.242 = *tmp.241
            tmp.244 = 8
            tmp.243 = tmp.242 != tmp.244
            if tmp.243 jump or_true_84
            tmp.236 = 0
            jump or_end_85
        
          or_true_84:
            tmp.236 = 1
        
          or_end_85:
            if !tmp.236 jump end_if_86
            return 0
        
          end_if_86:
            tmp.245 = &struct_array
            tmp.246 = sign_extend 2
            tmp.247 = add_ptr(tmp.245, index=tmp.246, scale=40)
            tmp.248 = *tmp.247
            if tmp.248 jump or_true_88
            tmp.251 = &struct_array
            tmp.252 = sign_extend 2
            tmp.253 = add_ptr(tmp.251, index=tmp.252, scale=40)
            tmp.254 = add_ptr(tmp.253, index=24L, scale=1)
            tmp.255 = *tmp.254
            if tmp.255 jump or_true_88
            tmp.250 = 0
            jump or_end_89
        
          or_true_88:
            tmp.250 = 1
        
          or_end_89:
            if tmp.250 jump or_true_90
            tmp.258 = &struct_array
            tmp.259 = sign_extend 2
            tmp.260 = add_ptr(tmp.258, index=tmp.259, scale=40)
            tmp.261 = add_ptr(tmp.260, index=32L, scale=1)
            tmp.262 = *tmp.261
            if tmp.262 jump or_true_90
            tmp.257 = 0
            jump or_end_91
        
          or_true_90:
            tmp.257 = 1
        
          or_end_91:
            if !tmp.257 jump end_if_92
            return 0
        
          end_if_92:
            tmp.263 = &struct_array
            tmp.264 = sign_extend 2
            tmp.265 = add_ptr(tmp.263, index=tmp.264, scale=40)
            tmp.266 = add_ptr(tmp.265, index=8L, scale=1)
            tmp.267 = *tmp.266
            if tmp.267 jump or_true_94
            tmp.270 = &struct_array
            tmp.271 = sign_extend 2
            tmp.272 = add_ptr(tmp.270, index=tmp.271, scale=40)
            tmp.273 = add_ptr(tmp.272, index=8L, scale=1)
            tmp.274 = add_ptr(tmp.273, index=4L, scale=1)
            tmp.275 = sign_extend 0
            tmp.276 = add_ptr(tmp.274, index=tmp.275, scale=1)
            tmp.277 = *tmp.276
            if tmp.277 jump or_true_94
            tmp.269 = 0
            jump or_end_95
        
          or_true_94:
            tmp.269 = 1
        
          or_end_95:
            if tmp.269 jump or_true_96
            tmp.280 = &struct_array
            tmp.281 = sign_extend 2
            tmp.282 = add_ptr(tmp.280, index=tmp.281, scale=40)
            tmp.283 = add_ptr(tmp.282, index=8L, scale=1)
            tmp.284 = add_ptr(tmp.283, index=4L, scale=1)
            tmp.285 = sign_extend 1
            tmp.286 = add_ptr(tmp.284, index=tmp.285, scale=1)
            tmp.287 = *tmp.286
            if tmp.287 jump or_true_96
            tmp.279 = 0
            jump or_end_97
        
          or_true_96:
            tmp.279 = 1
        
          or_end_97:
            if tmp.279 jump or_true_98
            tmp.290 = &struct_array
            tmp.291 = sign_extend 2
            tmp.292 = add_ptr(tmp.290, index=tmp.291, scale=40)
            tmp.293 = add_ptr(tmp.292, index=8L, scale=1)
            tmp.294 = add_ptr(tmp.293, index=4L, scale=1)
            tmp.295 = sign_extend 2
            tmp.296 = add_ptr(tmp.294, index=tmp.295, scale=1)
            tmp.297 = *tmp.296
            if tmp.297 jump or_true_98
            tmp.289 = 0
            jump or_end_99
        
          or_true_98:
            tmp.289 = 1
        
          or_end_99:
            if tmp.289 jump or_true_100
            tmp.300 = &struct_array
            tmp.301 = sign_extend 2
            tmp.302 = add_ptr(tmp.300, index=tmp.301, scale=40)
            tmp.303 = add_ptr(tmp.302, index=8L, scale=1)
            tmp.304 = add_ptr(tmp.303, index=8L, scale=1)
            tmp.305 = *tmp.304
            if tmp.305 jump or_true_100
            tmp.299 = 0
            jump or_end_101
        
          or_true_100:
            tmp.299 = 1
        
          or_end_101:
            if !tmp.299 jump end_if_102
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
        static global converted: Struct(outer.3) = [ 10L, -2147483646, '\u{f}', '\u{3}', '\u{81}', zero[1], 2147483648U, zero[4], 0UL, 9223372036854778000D]
        static global full: Struct(outer.3) = [ 18014398509481979L, 1000, "ok\\0", zero[1], 4292870144U, zero[4], &string.1, 2000000000000D]
        static global partial: Struct(outer.3) = [ 100L, 10, '\n', zero[2], zero[5], zero[4], &string.0, zero[8]]
        constant string.0: Array(7,Char) = "Hello!\\0"
        constant string.1: Array(16,Char) = "Another message\\0"
        constant string.2: Array(8,Char) = "Message\\0"
        static global struct_array: Array(3,Struct(outer.3)) = [ 1L, 2, "ab\\0", zero[1], 3U, zero[4], 0UL, 5D, 6L, 7, "cd\\0", zero[1], 8U, zero[4], &string.2, 9D, zero[40]]
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
            tmp.3 = uninitialized[8]
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if tmp.2 jump or_true_2
            tmp.6 = &uninitialized
            tmp.6 = add_ptr(tmp.6, index=16L, scale=1)
            tmp.7 = sign_extend 0
            tmp.8 = add_ptr(tmp.6, index=tmp.7, scale=1)
            tmp.9 = *tmp.8
            if tmp.9 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            if tmp.5 jump or_true_4
            tmp.12 = &uninitialized
            tmp.12 = add_ptr(tmp.12, index=16L, scale=1)
            tmp.13 = sign_extend 1
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            if tmp.15 jump or_true_4
            tmp.11 = 0
            jump or_end_5
        
          or_true_4:
            tmp.11 = 1
        
          or_end_5:
            if tmp.11 jump or_true_6
            tmp.18 = &uninitialized
            tmp.18 = add_ptr(tmp.18, index=16L, scale=1)
            tmp.19 = sign_extend 2
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=1)
            tmp.21 = *tmp.20
            if tmp.21 jump or_true_6
            tmp.17 = 0
            jump or_end_7
        
          or_true_6:
            tmp.17 = 1
        
          or_end_7:
            if tmp.17 jump or_true_8
            tmp.24 = uninitialized[20]
            if tmp.24 jump or_true_8
            tmp.23 = 0
            jump or_end_9
        
          or_true_8:
            tmp.23 = 1
        
          or_end_9:
            if !tmp.23 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_partially_initialized() { 
            tmp.25 = partial[0]
            tmp.26 = tmp.25 != 1D
            if tmp.26 jump or_true_12
            tmp.29 = partial[8]
            tmp.30 = &string.0
            tmp.31 = strcmp(tmp.29, tmp.30)
            if tmp.31 jump or_true_12
            tmp.28 = 0
            jump or_end_13
        
          or_true_12:
            tmp.28 = 1
        
          or_end_13:
            if !tmp.28 jump end_if_14
            return 0
        
          end_if_14:
            tmp.32 = &partial
            tmp.32 = add_ptr(tmp.32, index=16L, scale=1)
            tmp.33 = sign_extend 0
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=1)
            tmp.35 = *tmp.34
            if tmp.35 jump or_true_16
            tmp.38 = &partial
            tmp.38 = add_ptr(tmp.38, index=16L, scale=1)
            tmp.39 = sign_extend 1
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=1)
            tmp.41 = *tmp.40
            if tmp.41 jump or_true_16
            tmp.37 = 0
            jump or_end_17
        
          or_true_16:
            tmp.37 = 1
        
          or_end_17:
            if tmp.37 jump or_true_18
            tmp.44 = &partial
            tmp.44 = add_ptr(tmp.44, index=16L, scale=1)
            tmp.45 = sign_extend 2
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=1)
            tmp.47 = *tmp.46
            if tmp.47 jump or_true_18
            tmp.43 = 0
            jump or_end_19
        
          or_true_18:
            tmp.43 = 1
        
          or_end_19:
            if tmp.43 jump or_true_20
            tmp.50 = partial[20]
            if tmp.50 jump or_true_20
            tmp.49 = 0
            jump or_end_21
        
          or_true_20:
            tmp.49 = 1
        
          or_end_21:
            if !tmp.49 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function test_partial_inner_init() { 
            tmp.51 = partial_with_array[0]
            tmp.52 = tmp.51 != 3D
            if tmp.52 jump or_true_24
            tmp.55 = partial_with_array[8]
            tmp.56 = &string.1
            tmp.57 = strcmp(tmp.55, tmp.56)
            if tmp.57 jump or_true_24
            tmp.54 = 0
            jump or_end_25
        
          or_true_24:
            tmp.54 = 1
        
          or_end_25:
            if tmp.54 jump or_true_26
            tmp.60 = &partial_with_array
            tmp.60 = add_ptr(tmp.60, index=16L, scale=1)
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=1)
            tmp.63 = *tmp.62
            tmp.64 = zero_extend tmp.63
            tmp.65 = tmp.64 != 1
            if tmp.65 jump or_true_26
            tmp.59 = 0
            jump or_end_27
        
          or_true_26:
            tmp.59 = 1
        
          or_end_27:
            if tmp.59 jump or_true_28
            tmp.68 = partial_with_array[20]
            tmp.69 = tmp.68 != 2
            if tmp.69 jump or_true_28
            tmp.67 = 0
            jump or_end_29
        
          or_true_28:
            tmp.67 = 1
        
          or_end_29:
            if !tmp.67 jump end_if_30
            return 0
        
          end_if_30:
            tmp.70 = &partial_with_array
            tmp.70 = add_ptr(tmp.70, index=16L, scale=1)
            tmp.71 = sign_extend 1
            tmp.72 = add_ptr(tmp.70, index=tmp.71, scale=1)
            tmp.73 = *tmp.72
            if tmp.73 jump or_true_32
            tmp.76 = &partial_with_array
            tmp.76 = add_ptr(tmp.76, index=16L, scale=1)
            tmp.77 = sign_extend 2
            tmp.78 = add_ptr(tmp.76, index=tmp.77, scale=1)
            tmp.79 = *tmp.78
            if tmp.79 jump or_true_32
            tmp.75 = 0
            jump or_end_33
        
          or_true_32:
            tmp.75 = 1
        
          or_end_33:
            if !tmp.75 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function test_implicit_conversion() { 
            tmp.80 = converted[0]
            tmp.81 = tmp.80 != 1152921504606847000D
            if tmp.81 jump or_true_36
            tmp.84 = converted[8]
            if tmp.84 jump or_true_36
            tmp.83 = 0
            jump or_end_37
        
          or_true_36:
            tmp.83 = 1
        
          or_end_37:
            if tmp.83 jump or_true_38
            tmp.87 = &converted
            tmp.87 = add_ptr(tmp.87, index=16L, scale=1)
            tmp.88 = sign_extend 0
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=1)
            tmp.90 = *tmp.89
            tmp.91 = zero_extend tmp.90
            tmp.92 = tmp.91 != 97
            if tmp.92 jump or_true_38
            tmp.86 = 0
            jump or_end_39
        
          or_true_38:
            tmp.86 = 1
        
          or_end_39:
            if tmp.86 jump or_true_40
            tmp.95 = &converted
            tmp.95 = add_ptr(tmp.95, index=16L, scale=1)
            tmp.96 = sign_extend 1
            tmp.97 = add_ptr(tmp.95, index=tmp.96, scale=1)
            tmp.98 = *tmp.97
            tmp.99 = zero_extend tmp.98
            tmp.100 = tmp.99 != 98
            if tmp.100 jump or_true_40
            tmp.94 = 0
            jump or_end_41
        
          or_true_40:
            tmp.94 = 1
        
          or_end_41:
            if tmp.94 jump or_true_42
            tmp.103 = &converted
            tmp.103 = add_ptr(tmp.103, index=16L, scale=1)
            tmp.104 = sign_extend 2
            tmp.105 = add_ptr(tmp.103, index=tmp.104, scale=1)
            tmp.106 = *tmp.105
            tmp.107 = zero_extend tmp.106
            tmp.108 = tmp.107 != 99
            if tmp.108 jump or_true_42
            tmp.102 = 0
            jump or_end_43
        
          or_true_42:
            tmp.102 = 1
        
          or_end_43:
            if tmp.102 jump or_true_44
            tmp.111 = converted[20]
            tmp.112 = tmp.111 != 5
            if tmp.112 jump or_true_44
            tmp.110 = 0
            jump or_end_45
        
          or_true_44:
            tmp.110 = 1
        
          or_end_45:
            if !tmp.110 jump end_if_46
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
        static global partial: Struct(s.2) = [ 1D, &string.0, zero[8]]
        static global partial_with_array: Struct(s.2) = [ 3D, &string.1, 1UC, zero[2], zero[1], 2]
        constant string.0: Array(6,Char) = "Hello\\0"
        constant string.1: Array(2,Char) = "!\\0"
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
            *ptr.8 = i.5
            tmp.2 = *ptr.8
            tmp.3 = add_ptr(ptr.8, index=8L, scale=1)
            *tmp.3 = d.6
            tmp.4 = *tmp.3
            tmp.5 = add_ptr(ptr.8, index=16L, scale=1)
            *tmp.5 = s.7
            tmp.6 = *tmp.5
            return ptr.8
            return 0
        }
        global function increment_struct(ptr.9) { 
            tmp.7 = *ptr.9
            tmp.8 = tmp.7 + 1
            *ptr.9 = tmp.8
            tmp.9 = *ptr.9
            tmp.10 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.11 = add_ptr(ptr.9, index=8L, scale=1)
            tmp.12 = *tmp.11
            tmp.14 = int_to_double 1
            tmp.13 = tmp.12 + tmp.14
            *tmp.10 = tmp.13
            tmp.15 = *tmp.10
            tmp.16 = add_ptr(ptr.9, index=16L, scale=1)
            tmp.17 = add_ptr(ptr.9, index=16L, scale=1)
            tmp.18 = *tmp.17
            *tmp.16 = tmp.18
            tmp.19 = *tmp.16
            return 0
        }
        global function check_struct(ptr.10, expected_i.11, expected_d.12, expected_s.13) { 
            tmp.20 = *ptr.10
            tmp.21 = tmp.20 != expected_i.11
            if !tmp.21 jump end_if_0
            return 0
        
          end_if_0:
            tmp.22 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.23 = *tmp.22
            tmp.24 = tmp.23 != expected_d.12
            if !tmp.24 jump end_if_2
            return 0
        
          end_if_2:
            tmp.25 = add_ptr(ptr.10, index=16L, scale=1)
            tmp.26 = *tmp.25
            tmp.27 = strcmp(tmp.26, expected_s.13)
            if !tmp.27 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function print_struct_msg(ptr.14) { 
            tmp.28 = add_ptr(ptr.14, index=16L, scale=1)
            tmp.29 = *tmp.28
            tmp.30 = puts(tmp.29)
            return 0
        }
        global function get_internal_struct() { 
            tmp.31 = &internal
            return tmp.31
            return 0
        }
        static global incomplete_var: Struct(s.4) = [ 3, zero[4], 4D, &string.1]
        static internal: Struct(s.4) = [ 1, zero[4], 2D, &string.0]
        constant string.0: Array(14,Char) = "static struct\\0"
        constant string.1: Array(14,Char) = "global struct\\0"
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
            tmp.0 = *ptr.12
            tmp.1 = sign_extend tmp.0
            tmp.2 = tmp.1 != expected_a.13
            if !tmp.2 jump end_if_0
            return 0
        
          end_if_0:
            tmp.3 = add_ptr(ptr.12, index=1L, scale=1)
            tmp.4 = *tmp.3
            tmp.5 = sign_extend tmp.4
            tmp.6 = tmp.5 != expected_b.14
            if !tmp.6 jump end_if_2
            return 0
        
          end_if_2:
            tmp.7 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.8 = *tmp.7
            tmp.9 = tmp.8 != expected_d.15
            if !tmp.9 jump end_if_4
            return 0
        
          end_if_4:
            tmp.10 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.11 = add_ptr(tmp.10, index=8L, scale=1)
            tmp.12 = *tmp.11
            tmp.13 = tmp.12 != expected_i.16
            if !tmp.13 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function update_members_through_pointer(ptr.17, a.18, b.19, inner_ptr.20) { 
            tmp.14 = truncate a.18
            *ptr.17 = tmp.14
            tmp.15 = *ptr.17
            tmp.16 = add_ptr(ptr.17, index=1L, scale=1)
            tmp.17 = truncate b.19
            *tmp.16 = tmp.17
            tmp.18 = *tmp.16
            tmp.19 = add_ptr(ptr.17, index=8L, scale=1)
            tmp.20 = *inner_ptr.20
            *tmp.19 = tmp.20
            tmp.21 = *tmp.19
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
            s.12[0] = tmp.0
            tmp.1 = truncate 2
            s.12[1] = tmp.1
            s.12[8] = 3D
            s.12[16] = 4
            tmp.2 = &s.12
            tmp.3 = access_members_through_pointer(tmp.2, 1, 2, 3D, 4)
            tmp.4 = ! tmp.3
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = int_to_double 7
            inner_struct.13[0] = tmp.5
            inner_struct.13[8] = 8
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
            tmp.18 = s.12[8]
            tmp.20 = int_to_double 7
            tmp.19 = tmp.18 != tmp.20
            if tmp.19 jump or_true_4
            tmp.17 = 0
            jump or_end_5
        
          or_true_4:
            tmp.17 = 1
        
          or_end_5:
            if tmp.17 jump or_true_6
            tmp.23 = s.12[16]
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
            *ptr.8 = tmp.2
            tmp.3 = *ptr.8
            tmp.4 = add_ptr(ptr.8, index=8L, scale=1)
            *tmp.4 = seed.7
            tmp.5 = *tmp.4
            return ptr.8
            return 0
        }
        global function make_struct_outer(seed.9) { 
            tmp.6 = malloc(24UL)
            tmp.7 = tmp.6
            ptr.10 = tmp.7
            tmp.8 = truncate seed.9
            *ptr.10 = tmp.8
            tmp.9 = *ptr.10
            tmp.10 = add_ptr(ptr.10, index=1L, scale=1)
            tmp.11 = seed.9 + 1
            tmp.12 = truncate tmp.11
            *tmp.10 = tmp.12
            tmp.13 = *tmp.10
            tmp.14 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.15 = seed.9 + 2
            tmp.16 = int_to_double tmp.15
            *tmp.14 = tmp.16
            tmp.17 = *tmp.14
            tmp.18 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.19 = add_ptr(tmp.18, index=8L, scale=1)
            tmp.20 = seed.9 + 3
            *tmp.19 = tmp.20
            tmp.21 = *tmp.19
            return ptr.10
            return 0
        }
        global function make_struct_outermost(seed.11) { 
            tmp.22 = malloc(40UL)
            tmp.23 = tmp.22
            ptr.12 = tmp.23
            *ptr.12 = seed.11
            tmp.24 = *ptr.12
            tmp.25 = add_ptr(ptr.12, index=8L, scale=1)
            tmp.26 = seed.11 + 1
            tmp.27 = make_struct_outer(tmp.26)
            *tmp.25 = tmp.27
            tmp.28 = *tmp.25
            tmp.29 = add_ptr(ptr.12, index=16L, scale=1)
            tmp.30 = seed.11 + 5
            tmp.31 = truncate tmp.30
            *tmp.29 = tmp.31
            tmp.32 = *tmp.29
            tmp.33 = add_ptr(ptr.12, index=16L, scale=1)
            tmp.34 = add_ptr(tmp.33, index=1L, scale=1)
            tmp.35 = seed.11 + 6
            tmp.36 = truncate tmp.35
            *tmp.34 = tmp.36
            tmp.37 = *tmp.34
            tmp.38 = add_ptr(ptr.12, index=16L, scale=1)
            tmp.39 = add_ptr(tmp.38, index=8L, scale=1)
            tmp.40 = seed.11 + 7
            tmp.41 = int_to_double tmp.40
            *tmp.39 = tmp.41
            tmp.42 = *tmp.39
            tmp.43 = add_ptr(ptr.12, index=16L, scale=1)
            tmp.44 = add_ptr(tmp.43, index=8L, scale=1)
            tmp.45 = add_ptr(tmp.44, index=8L, scale=1)
            tmp.46 = seed.11 + 8
            *tmp.45 = tmp.46
            tmp.47 = *tmp.45
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
            tmp.1 = *inner_ptr.7
            tmp.3 = int_to_double 11
            tmp.2 = tmp.1 != tmp.3
            if tmp.2 jump or_true_0
            tmp.6 = add_ptr(inner_ptr.7, index=8L, scale=1)
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 != 11
            if tmp.8 jump or_true_0
            tmp.5 = 0
            jump or_end_1
        
          or_true_0:
            tmp.5 = 1
        
          or_end_1:
            if !tmp.5 jump end_if_2
            return 0
        
          end_if_2:
            o.8[0] = 0
            tmp.9 = sign_extend 0
            o.8[8] = tmp.9
            tmp.10 = truncate 0
            o.8[16] = tmp.10
            tmp.11 = truncate 0
            o.8[17] = tmp.11
            tmp.12 = int_to_double 0
            o.8[24] = tmp.12
            o.8[32] = 0
            tmp.13 = make_struct_outer(20)
            o.8[8] = tmp.13
            tmp.14 = o.8[8]
            tmp.15 = o.8[8]
            tmp.16 = *tmp.15
            tmp.17 = sign_extend tmp.16
            tmp.18 = tmp.17 != 20
            if tmp.18 jump or_true_4
            tmp.21 = o.8[8]
            tmp.22 = add_ptr(tmp.21, index=1L, scale=1)
            tmp.23 = *tmp.22
            tmp.24 = sign_extend tmp.23
            tmp.25 = tmp.24 != 21
            if tmp.25 jump or_true_4
            tmp.20 = 0
            jump or_end_5
        
          or_true_4:
            tmp.20 = 1
        
          or_end_5:
            if tmp.20 jump or_true_6
            tmp.28 = o.8[8]
            tmp.29 = add_ptr(tmp.28, index=8L, scale=1)
            tmp.30 = *tmp.29
            tmp.32 = int_to_double 22
            tmp.31 = tmp.30 != tmp.32
            if tmp.31 jump or_true_6
            tmp.27 = 0
            jump or_end_7
        
          or_true_6:
            tmp.27 = 1
        
          or_end_7:
            if tmp.27 jump or_true_8
            tmp.35 = o.8[8]
            tmp.36 = add_ptr(tmp.35, index=8L, scale=1)
            tmp.37 = add_ptr(tmp.36, index=8L, scale=1)
            tmp.38 = *tmp.37
            tmp.39 = tmp.38 != 23
            if tmp.39 jump or_true_8
            tmp.34 = 0
            jump or_end_9
        
          or_true_8:
            tmp.34 = 1
        
          or_end_9:
            if !tmp.34 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_get_struct_pointer_member() { 
            tmp.40 = make_struct_inner(2)
            tmp.41 = *tmp.40
            tmp.43 = int_to_double 2
            tmp.42 = tmp.41 != tmp.43
            if !tmp.42 jump end_if_12
            return 0
        
          end_if_12:
            tmp.44 = make_struct_outer(2)
            tmp.45 = add_ptr(tmp.44, index=8L, scale=1)
            tmp.46 = *tmp.45
            tmp.48 = int_to_double 4
            tmp.47 = tmp.46 != tmp.48
            if !tmp.47 jump end_if_14
            return 0
        
          end_if_14:
            tmp.49 = make_struct_outermost(0)
            tmp.50 = add_ptr(tmp.49, index=8L, scale=1)
            tmp.51 = *tmp.50
            tmp.52 = *tmp.51
            tmp.53 = sign_extend tmp.52
            tmp.54 = tmp.53 != 1
            if !tmp.54 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function get_static_struct_ptr() { 
            tmp.55 = &s.9
            return tmp.55
            return 0
        }
        global function test_update_member_thru_retval() { 
            tmp.56 = get_static_struct_ptr()
            tmp.57 = truncate 10
            *tmp.56 = tmp.57
            tmp.58 = *tmp.56
            tmp.59 = get_static_struct_ptr()
            tmp.60 = add_ptr(tmp.59, index=8L, scale=1)
            *tmp.60 = 20D
            tmp.61 = *tmp.60
            tmp.62 = get_static_struct_ptr()
            ptr.10 = tmp.62
            tmp.63 = *ptr.10
            tmp.64 = sign_extend tmp.63
            tmp.65 = tmp.64 != 10
            if tmp.65 jump or_true_18
            tmp.68 = add_ptr(ptr.10, index=8L, scale=1)
            tmp.69 = *tmp.68
            tmp.70 = tmp.69 != 20D
            if tmp.70 jump or_true_18
            tmp.67 = 0
            jump or_end_19
        
          or_true_18:
            tmp.67 = 1
        
          or_end_19:
            if !tmp.67 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function test_update_nested_struct_thru_retval() { 
            small.11[0] = 12D
            small.11[8] = 13
            tmp.71 = get_static_struct_ptr()
            tmp.72 = add_ptr(tmp.71, index=8L, scale=1)
            *tmp.72 = small.11
            tmp.73 = *tmp.72
            tmp.74 = get_static_struct_ptr()
            tmp.75 = add_ptr(tmp.74, index=8L, scale=1)
            tmp.76 = *tmp.75
            tmp.77 = tmp.76 != 12D
            if !tmp.77 jump end_if_22
            return 0
        
          end_if_22:
            tmp.78 = get_static_struct_ptr()
            tmp.79 = add_ptr(tmp.78, index=8L, scale=1)
            tmp.80 = add_ptr(tmp.79, index=8L, scale=1)
            tmp.81 = *tmp.80
            tmp.82 = tmp.81 != 13
            if !tmp.82 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function main() { 
            tmp.83 = test_get_struct_ptr()
            tmp.84 = ! tmp.83
            if !tmp.84 jump end_if_26
            return 1
        
          end_if_26:
            tmp.85 = test_get_struct_pointer_member()
            tmp.86 = ! tmp.85
            if !tmp.86 jump end_if_28
            return 2
        
          end_if_28:
            tmp.87 = test_update_member_thru_retval()
            tmp.88 = ! tmp.87
            if !tmp.88 jump end_if_30
            return 3
        
          end_if_30:
            tmp.89 = test_update_nested_struct_thru_retval()
            tmp.90 = ! tmp.89
            if !tmp.90 jump end_if_32
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
            array.2[0] = 1
            array.2[4] = 2
            array.2[8] = 3
            array.2[12] = 4
            array.2[16] = 5
            array.2[20] = 6
            array.2[24] = 7
            array.2[28] = 8
            array.2[32] = 9
            array.2[36] = 10
            array.2[40] = 11
            array.2[44] = 12
            array.2[48] = 13
            array.2[52] = 14
            array.2[56] = 15
            array.2[60] = 16
            tmp.0 = &array.2
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=16)
            tmp.3 = add_ptr(tmp.2, index=4L, scale=1)
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
            tmp.1 = foo.1[0]
            b.2 = tmp.1
            tmp.2 = foo.1[0]
            tmp.3 = tmp.2 == b.2
            return tmp.3
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
            x1.1[0] = 3
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
            x.1[0] = 1
            x.1[4] = 2
            tmp.0 = x.1[0]
            tmp.1 = tmp.0 != 1
            if tmp.1 jump or_true_0
            tmp.4 = x.1[4]
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
            *autom_ptr.12 = tmp.31
            tmp.34 = *autom_ptr.12
            tmp.35 = add_ptr(autom_ptr.12, index=8L, scale=1)
            tmp.36 = truncate 127
            *tmp.35 = tmp.36
            tmp.37 = *tmp.35
            tmp.38 = add_ptr(autom_ptr.12, index=16L, scale=1)
            tmp.39 = get_double()
            tmp.41 = int_to_double l
            tmp.40 = tmp.39 / tmp.41
            tmp.42 = double_to_int tmp.40
            *tmp.38 = tmp.42
            tmp.43 = *tmp.38
            tmp.44 = truncate 100
            chr.13 = tmp.44
            tmp.45 = add_ptr(autom_ptr.12, index=24L, scale=1)
            tmp.46 = &chr.13
            *tmp.45 = tmp.46
            tmp.47 = *tmp.45
            tmp.48 = *autom_ptr.12
            tmp.50 = - 1845381177299D
            tmp.49 = tmp.48 != tmp.50
            if tmp.49 jump or_true_16
            tmp.53 = add_ptr(autom_ptr.12, index=8L, scale=1)
            tmp.54 = *tmp.53
            tmp.55 = sign_extend tmp.54
            tmp.56 = tmp.55 != 127
            if tmp.56 jump or_true_16
            tmp.52 = 0
            jump or_end_17
        
          or_true_16:
            tmp.52 = 1
        
          or_end_17:
            if tmp.52 jump or_true_18
            tmp.59 = add_ptr(autom_ptr.12, index=16L, scale=1)
            tmp.60 = *tmp.59
            tmp.62 = sign_extend 58
            tmp.61 = tmp.60 != tmp.62
            if tmp.61 jump or_true_18
            tmp.58 = 0
            jump or_end_19
        
          or_true_18:
            tmp.58 = 1
        
          or_end_19:
            if tmp.58 jump or_true_20
            tmp.65 = add_ptr(autom_ptr.12, index=24L, scale=1)
            tmp.66 = *tmp.65
            tmp.68 = &chr.13
            tmp.67 = tmp.66 != tmp.68
            if tmp.67 jump or_true_20
            tmp.64 = 0
            jump or_end_21
        
          or_true_20:
            tmp.64 = 1
        
          or_end_21:
            if !tmp.64 jump end_if_22
            return 0
        
          end_if_22:
            d_ptr.14 = autom_ptr.12
            tmp.69 = add_ptr(autom_ptr.12, index=8L, scale=1)
            c_ptr.15 = tmp.69
            tmp.70 = *d_ptr.14
            tmp.72 = - 1845381177299D
            tmp.71 = tmp.70 != tmp.72
            if tmp.71 jump or_true_24
            tmp.75 = *c_ptr.15
            tmp.76 = sign_extend tmp.75
            tmp.77 = tmp.76 != 127
            if tmp.77 jump or_true_24
            tmp.74 = 0
            jump or_end_25
        
          or_true_24:
            tmp.74 = 1
        
          or_end_25:
            if !tmp.74 jump end_if_26
            return 0
        
          end_if_26:
            tmp.78 = add_ptr(autom_ptr.12, index=24L, scale=1)
            tmp.79 = *tmp.78
            tmp.80 = *tmp.79
            tmp.81 = sign_extend tmp.80
            tmp.82 = tmp.81 != 100
            if !tmp.82 jump end_if_28
            return 0
        
          end_if_28:
            tmp.83 = autom.11[0]
            tmp.85 = int_to_double 2000
            tmp.84 = tmp.83 / tmp.85
            tmp.86 = double_to_int tmp.84
            tmp.87 = autom.11[8]
            tmp.88 = sign_extend tmp.87
            tmp.89 = tmp.88 * 2
            tmp.90 = autom.11[16]
            tmp.91 = int_to_double tmp.90
            tmp.92 = autom.11[24]
            tmp.93 = *tmp.92
            tmp.94 = sign_extend tmp.93
            tmp.95 = autom.11[0]
            tmp.96 = autom.11[8]
            tmp.97 = sign_extend tmp.96
            tmp.98 = autom.11[16]
            tmp.99 = autom.11[24]
            tmp.100 = accept_params(tmp.86, tmp.89, tmp.91, tmp.94, tmp.95, tmp.97, tmp.98, tmp.99)
            tmp.101 = ! tmp.100
            if !tmp.101 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_static() { 
            tmp.102 = &stat.16
            stat_ptr.17 = tmp.102
            tmp.103 = int_to_double l
            tmp.105 = get_double()
            tmp.104 = tmp.103 - tmp.105
            tmp.107 = int_to_double l
            tmp.108 = tmp.107 * 3.5D
            tmp.106 = tmp.104 + tmp.108
            *stat_ptr.17 = tmp.106
            tmp.109 = *stat_ptr.17
            tmp.110 = add_ptr(stat_ptr.17, index=8L, scale=1)
            tmp.111 = truncate 127
            *tmp.110 = tmp.111
            tmp.112 = *tmp.110
            tmp.113 = add_ptr(stat_ptr.17, index=16L, scale=1)
            tmp.114 = get_double()
            tmp.116 = int_to_double l
            tmp.115 = tmp.114 / tmp.116
            tmp.117 = double_to_int tmp.115
            *tmp.113 = tmp.117
            tmp.118 = *tmp.113
            tmp.119 = add_ptr(stat_ptr.17, index=24L, scale=1)
            tmp.120 = &chr.18
            *tmp.119 = tmp.120
            tmp.121 = *tmp.119
            tmp.122 = *stat_ptr.17
            tmp.124 = - 1845381177299D
            tmp.123 = tmp.122 != tmp.124
            if tmp.123 jump or_true_32
            tmp.127 = add_ptr(stat_ptr.17, index=8L, scale=1)
            tmp.128 = *tmp.127
            tmp.129 = sign_extend tmp.128
            tmp.130 = tmp.129 != 127
            if tmp.130 jump or_true_32
            tmp.126 = 0
            jump or_end_33
        
          or_true_32:
            tmp.126 = 1
        
          or_end_33:
            if tmp.126 jump or_true_34
            tmp.133 = add_ptr(stat_ptr.17, index=16L, scale=1)
            tmp.134 = *tmp.133
            tmp.136 = sign_extend 58
            tmp.135 = tmp.134 != tmp.136
            if tmp.135 jump or_true_34
            tmp.132 = 0
            jump or_end_35
        
          or_true_34:
            tmp.132 = 1
        
          or_end_35:
            if tmp.132 jump or_true_36
            tmp.139 = add_ptr(stat_ptr.17, index=24L, scale=1)
            tmp.140 = *tmp.139
            tmp.142 = &chr.18
            tmp.141 = tmp.140 != tmp.142
            if tmp.141 jump or_true_36
            tmp.138 = 0
            jump or_end_37
        
          or_true_36:
            tmp.138 = 1
        
          or_end_37:
            if !tmp.138 jump end_if_38
            return 0
        
          end_if_38:
            d_ptr.19 = stat_ptr.17
            tmp.143 = add_ptr(stat_ptr.17, index=8L, scale=1)
            c_ptr.20 = tmp.143
            tmp.144 = *d_ptr.19
            tmp.146 = - 1845381177299D
            tmp.145 = tmp.144 != tmp.146
            if tmp.145 jump or_true_40
            tmp.149 = *c_ptr.20
            tmp.150 = sign_extend tmp.149
            tmp.151 = tmp.150 != 127
            if tmp.151 jump or_true_40
            tmp.148 = 0
            jump or_end_41
        
          or_true_40:
            tmp.148 = 1
        
          or_end_41:
            if !tmp.148 jump end_if_42
            return 0
        
          end_if_42:
            tmp.152 = add_ptr(stat_ptr.17, index=24L, scale=1)
            tmp.153 = *tmp.152
            tmp.154 = *tmp.153
            tmp.155 = sign_extend tmp.154
            tmp.156 = tmp.155 != 100
            if !tmp.156 jump end_if_44
            return 0
        
          end_if_44:
            tmp.157 = stat.16[0]
            tmp.159 = int_to_double 2000
            tmp.158 = tmp.157 / tmp.159
            tmp.160 = double_to_int tmp.158
            tmp.161 = stat.16[8]
            tmp.162 = sign_extend tmp.161
            tmp.163 = tmp.162 * 2
            tmp.164 = stat.16[16]
            tmp.165 = int_to_double tmp.164
            tmp.166 = stat.16[24]
            tmp.167 = *tmp.166
            tmp.168 = sign_extend tmp.167
            tmp.169 = stat.16[0]
            tmp.170 = stat.16[8]
            tmp.171 = sign_extend tmp.170
            tmp.172 = stat.16[16]
            tmp.173 = stat.16[24]
            tmp.174 = accept_params(tmp.160, tmp.163, tmp.165, tmp.168, tmp.169, tmp.171, tmp.172, tmp.173)
            tmp.175 = ! tmp.174
            if !tmp.175 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function test_exp_result_member() { 
            s1.22[0] = 10D
            tmp.176 = s1.22[0]
            tmp.177 = truncate 99
            s1.22[8] = tmp.177
            tmp.178 = s1.22[8]
            s1.22[16] = 9223372036854775807L
            tmp.179 = s1.22[16]
            tmp.180 = sign_extend 0
            s1.22[24] = tmp.180
            tmp.181 = s1.22[24]
            s2.23[0] = 12D
            tmp.182 = s2.23[0]
            tmp.183 = truncate 98
            s2.23[8] = tmp.183
            tmp.184 = s2.23[8]
            tmp.185 = - 9223372036854775807L
            s2.23[16] = tmp.185
            tmp.186 = s2.23[16]
            tmp.187 = sign_extend 0
            s2.23[24] = tmp.187
            tmp.188 = s2.23[24]
            tmp.189 = &s1.22
            s1_ptr.24 = tmp.189
            tmp.190 = &s2.23
            s2_ptr.25 = tmp.190
            if !flag.21 jump else_49
            tmp.191 = s1_ptr.24
            jump end_if_48
        
          else_49:
            tmp.191 = s2_ptr.25
        
          end_if_48:
            tmp.192 = add_ptr(tmp.191, index=8L, scale=1)
            tmp.193 = truncate 127
            *tmp.192 = tmp.193
            tmp.194 = *tmp.192
            tmp.195 = s1.22[8]
            tmp.196 = sign_extend tmp.195
            tmp.197 = tmp.196 != 127
            if !tmp.197 jump end_if_50
            return 0
        
          end_if_50:
            tmp.198 = s2.23[8]
            tmp.199 = sign_extend tmp.198
            tmp.200 = tmp.199 != 98
            if !tmp.200 jump end_if_52
            return 0
        
          end_if_52:
            tmp.201 = sign_extend 0
            result_ptr.26 = tmp.201
            result_ptr.26 = s2_ptr.25
            tmp.202 = *result_ptr.26
            tmp.203 = tmp.202 != 12D
            if tmp.203 jump or_true_54
            tmp.206 = add_ptr(result_ptr.26, index=16L, scale=1)
            tmp.207 = *tmp.206
            tmp.209 = - 9223372036854775807L
            tmp.208 = tmp.207 != tmp.209
            if tmp.208 jump or_true_54
            tmp.205 = 0
            jump or_end_55
        
          or_true_54:
            tmp.205 = 1
        
          or_end_55:
            if !tmp.205 jump end_if_56
            return 0
        
          end_if_56:
            tmp.210 = sign_extend 1
            tmp.211 = calloc(tmp.210, 32UL)
            void_ptr.27 = tmp.211
            tmp.212 = void_ptr.27
            tmp.213 = add_ptr(tmp.212, index=8L, scale=1)
            tmp.214 = truncate 80
            *tmp.213 = tmp.214
            tmp.215 = *tmp.213
            tmp.216 = void_ptr.27
            result_ptr.26 = tmp.216
            tmp.217 = add_ptr(result_ptr.26, index=8L, scale=1)
            tmp.218 = *tmp.217
            tmp.219 = sign_extend tmp.218
            tmp.220 = tmp.219 != 80
            if !tmp.220 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function main() { 
            tmp.221 = test_auto()
            tmp.222 = ! tmp.221
            if !tmp.222 jump end_if_60
            return 1
        
          end_if_60:
            tmp.223 = test_static()
            tmp.224 = ! tmp.223
            if !tmp.224 jump end_if_62
            return 2
        
          end_if_62:
            tmp.225 = test_exp_result_member()
            tmp.226 = ! tmp.225
            if !tmp.226 jump end_if_64
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
            tmp.27 = autom.9[0]
            tmp.28 = truncate 127
            autom.9[8] = tmp.28
            tmp.29 = autom.9[8]
            tmp.31 = sign_extend 4
            tmp.30 = l / tmp.31
            autom.9[16] = tmp.30
            tmp.32 = autom.9[16]
            tmp.33 = truncate 100
            chr.10 = tmp.33
            tmp.34 = &chr.10
            autom.9[24] = tmp.34
            tmp.35 = autom.9[24]
            tmp.36 = autom.9[0]
            tmp.37 = tmp.36 != 4000000000000D
            if tmp.37 jump or_true_16
            tmp.40 = autom.9[8]
            tmp.41 = sign_extend tmp.40
            tmp.42 = tmp.41 != 127
            if tmp.42 jump or_true_16
            tmp.39 = 0
            jump or_end_17
        
          or_true_16:
            tmp.39 = 1
        
          or_end_17:
            if tmp.39 jump or_true_18
            tmp.45 = autom.9[16]
            tmp.46 = tmp.45 != 8589934594L
            if tmp.46 jump or_true_18
            tmp.44 = 0
            jump or_end_19
        
          or_true_18:
            tmp.44 = 1
        
          or_end_19:
            if tmp.44 jump or_true_20
            tmp.49 = autom.9[24]
            tmp.51 = &chr.10
            tmp.50 = tmp.49 != tmp.51
            if tmp.50 jump or_true_20
            tmp.48 = 0
            jump or_end_21
        
          or_true_20:
            tmp.48 = 1
        
          or_end_21:
            if !tmp.48 jump end_if_22
            return 0
        
          end_if_22:
            tmp.52 = &autom.9
            d_ptr.11 = tmp.52
            tmp.53 = &autom.9
            tmp.53 = add_ptr(tmp.53, index=8L, scale=1)
            c_ptr.12 = tmp.53
            tmp.54 = *d_ptr.11
            tmp.55 = tmp.54 != 4000000000000D
            if tmp.55 jump or_true_24
            tmp.58 = *c_ptr.12
            tmp.59 = sign_extend tmp.58
            tmp.60 = tmp.59 != 127
            if tmp.60 jump or_true_24
            tmp.57 = 0
            jump or_end_25
        
          or_true_24:
            tmp.57 = 1
        
          or_end_25:
            if !tmp.57 jump end_if_26
            return 0
        
          end_if_26:
            tmp.61 = autom.9[24]
            tmp.62 = *tmp.61
            tmp.63 = sign_extend tmp.62
            tmp.64 = tmp.63 != 100
            if !tmp.64 jump end_if_28
            return 0
        
          end_if_28:
            tmp.65 = autom.9[0]
            tmp.66 = tmp.65 / 40000000000D
            tmp.67 = double_to_int tmp.66
            tmp.68 = autom.9[8]
            tmp.69 = sign_extend tmp.68
            tmp.70 = tmp.69 * 2
            tmp.71 = autom.9[16]
            tmp.72 = int_to_double tmp.71
            tmp.73 = autom.9[24]
            tmp.74 = *tmp.73
            tmp.75 = sign_extend tmp.74
            tmp.76 = autom.9[0]
            tmp.77 = autom.9[8]
            tmp.78 = sign_extend tmp.77
            tmp.79 = autom.9[16]
            tmp.80 = autom.9[24]
            tmp.81 = accept_params(tmp.67, tmp.70, tmp.72, tmp.75, tmp.76, tmp.78, tmp.79, tmp.80)
            tmp.82 = ! tmp.81
            if !tmp.82 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_static() { 
            tmp.83 = get_double()
            tmp.84 = tmp.83 * 2D
            stat.13[0] = tmp.84
            tmp.85 = stat.13[0]
            tmp.86 = truncate 127
            stat.13[8] = tmp.86
            tmp.87 = stat.13[8]
            tmp.89 = sign_extend 4
            tmp.88 = l / tmp.89
            stat.13[16] = tmp.88
            tmp.90 = stat.13[16]
            tmp.91 = &chr.14
            stat.13[24] = tmp.91
            tmp.92 = stat.13[24]
            tmp.93 = stat.13[0]
            tmp.94 = tmp.93 != 4000000000000D
            if tmp.94 jump or_true_32
            tmp.97 = stat.13[8]
            tmp.98 = sign_extend tmp.97
            tmp.99 = tmp.98 != 127
            if tmp.99 jump or_true_32
            tmp.96 = 0
            jump or_end_33
        
          or_true_32:
            tmp.96 = 1
        
          or_end_33:
            if tmp.96 jump or_true_34
            tmp.102 = stat.13[16]
            tmp.103 = tmp.102 != 8589934594L
            if tmp.103 jump or_true_34
            tmp.101 = 0
            jump or_end_35
        
          or_true_34:
            tmp.101 = 1
        
          or_end_35:
            if tmp.101 jump or_true_36
            tmp.106 = stat.13[24]
            tmp.108 = &chr.14
            tmp.107 = tmp.106 != tmp.108
            if tmp.107 jump or_true_36
            tmp.105 = 0
            jump or_end_37
        
          or_true_36:
            tmp.105 = 1
        
          or_end_37:
            if !tmp.105 jump end_if_38
            return 0
        
          end_if_38:
            tmp.109 = &stat.13
            d_ptr.15 = tmp.109
            tmp.110 = &stat.13
            tmp.110 = add_ptr(tmp.110, index=8L, scale=1)
            c_ptr.16 = tmp.110
            tmp.111 = *d_ptr.15
            tmp.112 = tmp.111 != 4000000000000D
            if tmp.112 jump or_true_40
            tmp.115 = *c_ptr.16
            tmp.116 = sign_extend tmp.115
            tmp.117 = tmp.116 != 127
            if tmp.117 jump or_true_40
            tmp.114 = 0
            jump or_end_41
        
          or_true_40:
            tmp.114 = 1
        
          or_end_41:
            if !tmp.114 jump end_if_42
            return 0
        
          end_if_42:
            tmp.118 = stat.13[24]
            tmp.119 = *tmp.118
            tmp.120 = sign_extend tmp.119
            tmp.121 = tmp.120 != 100
            if !tmp.121 jump end_if_44
            return 0
        
          end_if_44:
            tmp.122 = stat.13[0]
            tmp.123 = tmp.122 / 40000000000D
            tmp.124 = double_to_int tmp.123
            tmp.125 = stat.13[8]
            tmp.126 = sign_extend tmp.125
            tmp.127 = tmp.126 * 2
            tmp.128 = stat.13[16]
            tmp.129 = int_to_double tmp.128
            tmp.130 = stat.13[24]
            tmp.131 = *tmp.130
            tmp.132 = sign_extend tmp.131
            tmp.133 = stat.13[0]
            tmp.134 = stat.13[8]
            tmp.135 = sign_extend tmp.134
            tmp.136 = stat.13[16]
            tmp.137 = stat.13[24]
            tmp.138 = accept_params(tmp.124, tmp.127, tmp.129, tmp.132, tmp.133, tmp.135, tmp.136, tmp.137)
            tmp.139 = ! tmp.138
            if !tmp.139 jump end_if_46
            return 0
        
          end_if_46:
            return 1
            return 0
        }
        global function main() { 
            tmp.140 = test_auto()
            tmp.141 = ! tmp.140
            if !tmp.141 jump end_if_48
            return 1
        
          end_if_48:
            tmp.142 = test_static()
            tmp.143 = ! tmp.142
            if !tmp.143 jump end_if_50
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
            *head.4 = tmp.4
            tmp.5 = *head.4
            tmp.6 = add_ptr(head.4, index=8L, scale=1)
            tmp.7 = sign_extend 0
            *tmp.6 = tmp.7
            tmp.8 = *tmp.6
            current.5 = head.4
            i.6 = 1
        
          start_loop_0:
            tmp.9 = i.6 < count.3
            if !tmp.9 jump break_loop_0
            tmp.10 = add_ptr(current.5, index=8L, scale=1)
            tmp.11 = malloc(16UL)
            tmp.12 = tmp.11
            *tmp.10 = tmp.12
            tmp.13 = *tmp.10
            tmp.14 = add_ptr(current.5, index=8L, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = add_ptr(tmp.15, index=8L, scale=1)
            tmp.17 = sign_extend 0
            *tmp.16 = tmp.17
            tmp.18 = *tmp.16
            tmp.19 = add_ptr(current.5, index=8L, scale=1)
            tmp.20 = *tmp.19
            tmp.21 = sign_extend i.6
            tmp.22 = add_ptr(array.2, index=tmp.21, scale=4)
            tmp.23 = *tmp.22
            *tmp.20 = tmp.23
            tmp.24 = *tmp.20
            tmp.25 = add_ptr(current.5, index=8L, scale=1)
            tmp.26 = *tmp.25
            current.5 = tmp.26
        
          continue_loop_0:
            tmp.27 = i.6 + 1
            i.6 = tmp.27
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
            tmp.28 = &arr.7
            tmp.29 = array_to_list(tmp.28, 4)
            elem.8 = tmp.29
            i.9 = 0
        
          start_loop_1:
            tmp.30 = i.9 < 4
            if !tmp.30 jump break_loop_1
            tmp.31 = &arr.7
            tmp.32 = sign_extend i.9
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=4)
            tmp.34 = *tmp.33
            expected.10 = tmp.34
            tmp.35 = *elem.8
            tmp.36 = tmp.35 != expected.10
            if !tmp.36 jump end_if_0
            tmp.37 = i.9 + 1
            return tmp.37
        
          end_if_0:
            tmp.38 = add_ptr(elem.8, index=8L, scale=1)
            tmp.39 = *tmp.38
            elem.8 = tmp.39
        
          continue_loop_1:
            tmp.40 = i.9 + 1
            i.9 = tmp.40
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
            s.5[120] = 1D
            tmp.0 = s.5[120]
            tmp.1 = truncate 2
            s.5[128] = tmp.1
            tmp.2 = s.5[128]
            tmp.3 = &ptr_target
            s.5[136] = tmp.3
            tmp.4 = s.5[136]
            tmp.5 = s.5[120]
            tmp.6 = tmp.5 != 1D
            if tmp.6 jump or_true_0
            tmp.9 = s.5[128]
            tmp.10 = sign_extend tmp.9
            tmp.11 = tmp.10 != 2
            if tmp.11 jump or_true_0
            tmp.8 = 0
            jump or_end_1
        
          or_true_0:
            tmp.8 = 1
        
          or_end_1:
            if tmp.8 jump or_true_2
            tmp.14 = s.5[136]
            tmp.16 = &ptr_target
            tmp.15 = tmp.14 != tmp.16
            if tmp.15 jump or_true_2
            tmp.13 = 0
            jump or_end_3
        
          or_true_2:
            tmp.13 = 1
        
          or_end_3:
            if !tmp.13 jump end_if_4
            return 0
        
          end_if_4:
            tmp.17 = &s.5
            tmp.17 = add_ptr(tmp.17, index=128L, scale=1)
            char_ptr.6 = tmp.17
            tmp.18 = *char_ptr.6
            tmp.19 = sign_extend tmp.18
            tmp.20 = tmp.19 != 2
            if !tmp.20 jump end_if_6
            return 0
        
          end_if_6:
            tmp.21 = s.5[136]
            *tmp.21 = 5
            tmp.22 = *tmp.21
            tmp.23 = ptr_target != 5
            if !tmp.23 jump end_if_8
            return 0
        
          end_if_8:
            tmp.24 = s.5[128]
            tmp.25 = int_to_double tmp.24
            s.5[120] = tmp.25
            tmp.26 = s.5[120]
            tmp.27 = s.5[120]
            tmp.28 = tmp.27 != 2D
            if !tmp.28 jump end_if_10
            return 0
        
          end_if_10:
            return 1
            return 0
        }
        global function test_static_dot() { 
            s.7[120] = 1D
            tmp.29 = s.7[120]
            tmp.30 = truncate 2
            s.7[128] = tmp.30
            tmp.31 = s.7[128]
            tmp.32 = &ptr_target
            s.7[136] = tmp.32
            tmp.33 = s.7[136]
            tmp.34 = s.7[120]
            tmp.35 = tmp.34 != 1D
            if tmp.35 jump or_true_12
            tmp.38 = s.7[128]
            tmp.39 = sign_extend tmp.38
            tmp.40 = tmp.39 != 2
            if tmp.40 jump or_true_12
            tmp.37 = 0
            jump or_end_13
        
          or_true_12:
            tmp.37 = 1
        
          or_end_13:
            if tmp.37 jump or_true_14
            tmp.43 = s.7[136]
            tmp.45 = &ptr_target
            tmp.44 = tmp.43 != tmp.45
            if tmp.44 jump or_true_14
            tmp.42 = 0
            jump or_end_15
        
          or_true_14:
            tmp.42 = 1
        
          or_end_15:
            if !tmp.42 jump end_if_16
            return 0
        
          end_if_16:
            tmp.46 = &s.7
            tmp.46 = add_ptr(tmp.46, index=128L, scale=1)
            char_ptr.8 = tmp.46
            tmp.47 = *char_ptr.8
            tmp.48 = sign_extend tmp.47
            tmp.49 = tmp.48 != 2
            if !tmp.49 jump end_if_18
            return 0
        
          end_if_18:
            tmp.50 = s.7[136]
            *tmp.50 = 6
            tmp.51 = *tmp.50
            tmp.52 = ptr_target != 6
            if !tmp.52 jump end_if_20
            return 0
        
          end_if_20:
            tmp.53 = s.7[128]
            tmp.54 = int_to_double tmp.53
            s.7[120] = tmp.54
            tmp.55 = s.7[120]
            tmp.56 = s.7[120]
            tmp.57 = tmp.56 != 2D
            if !tmp.57 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function test_auto_arrow() { 
            tmp.58 = &s.10
            s_ptr.11 = tmp.58
            tmp.59 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.60 = &in.9
            *tmp.59 = tmp.60
            tmp.61 = *tmp.59
            *s_ptr.11 = 4294967295UL
            tmp.62 = *s_ptr.11
            tmp.63 = add_ptr(s_ptr.11, index=112L, scale=1)
            tmp.64 = - 5
            *tmp.63 = tmp.64
            tmp.65 = *tmp.63
            tmp.66 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.67 = *tmp.66
            *tmp.67 = 10D
            tmp.68 = *tmp.67
            tmp.69 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.70 = *tmp.69
            tmp.71 = add_ptr(tmp.70, index=8L, scale=1)
            tmp.72 = truncate 120
            *tmp.71 = tmp.72
            tmp.73 = *tmp.71
            tmp.74 = add_ptr(s_ptr.11, index=16L, scale=1)
            *tmp.74 = 11D
            tmp.75 = *tmp.74
            tmp.76 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.78 = sign_extend 3
            tmp.77 = add_ptr(tmp.76, index=tmp.78, scale=24)
            *tmp.77 = 12D
            tmp.79 = *tmp.77
            tmp.80 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.81 = add_ptr(tmp.80, index=16L, scale=1)
            tmp.82 = add_ptr(s_ptr.11, index=112L, scale=1)
            tmp.83 = tmp.82
            *tmp.81 = tmp.83
            tmp.84 = *tmp.81
            tmp.85 = *s_ptr.11
            tmp.86 = tmp.85 != 4294967295UL
            if tmp.86 jump or_true_24
            tmp.89 = add_ptr(s_ptr.11, index=112L, scale=1)
            tmp.90 = *tmp.89
            tmp.92 = - 5
            tmp.91 = tmp.90 != tmp.92
            if tmp.91 jump or_true_24
            tmp.88 = 0
            jump or_end_25
        
          or_true_24:
            tmp.88 = 1
        
          or_end_25:
            if !tmp.88 jump end_if_26
            return 0
        
          end_if_26:
            tmp.93 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.94 = *tmp.93
            tmp.95 = *tmp.94
            tmp.96 = tmp.95 != 10D
            if tmp.96 jump or_true_28
            tmp.99 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.100 = *tmp.99
            tmp.101 = add_ptr(tmp.100, index=8L, scale=1)
            tmp.102 = *tmp.101
            tmp.103 = sign_extend tmp.102
            tmp.104 = tmp.103 != 120
            if tmp.104 jump or_true_28
            tmp.98 = 0
            jump or_end_29
        
          or_true_28:
            tmp.98 = 1
        
          or_end_29:
            if tmp.98 jump or_true_30
            tmp.107 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.108 = *tmp.107
            tmp.109 = tmp.108 != 11D
            if tmp.109 jump or_true_30
            tmp.106 = 0
            jump or_end_31
        
          or_true_30:
            tmp.106 = 1
        
          or_end_31:
            if tmp.106 jump or_true_32
            tmp.112 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.114 = sign_extend 3
            tmp.113 = add_ptr(tmp.112, index=tmp.114, scale=24)
            tmp.115 = *tmp.113
            tmp.116 = tmp.115 != 12D
            if tmp.116 jump or_true_32
            tmp.111 = 0
            jump or_end_33
        
          or_true_32:
            tmp.111 = 1
        
          or_end_33:
            if !tmp.111 jump end_if_34
            return 0
        
          end_if_34:
            tmp.117 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.118 = *tmp.117
            tmp.119 = add_ptr(tmp.118, index=8L, scale=1)
            char_ptr.12 = tmp.119
            tmp.120 = *char_ptr.12
            tmp.121 = sign_extend tmp.120
            tmp.122 = tmp.121 != 120
            if !tmp.122 jump end_if_36
            return 0
        
          end_if_36:
            tmp.123 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.124 = add_ptr(tmp.123, index=16L, scale=1)
            tmp.125 = *tmp.124
            *tmp.125 = 123
            tmp.126 = *tmp.125
            tmp.127 = add_ptr(s_ptr.11, index=112L, scale=1)
            tmp.128 = *tmp.127
            tmp.129 = tmp.128 != 123
            if !tmp.129 jump end_if_38
            return 0
        
          end_if_38:
            tmp.130 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.131 = add_ptr(tmp.130, index=8L, scale=1)
            tmp.132 = add_ptr(s_ptr.11, index=8L, scale=1)
            tmp.133 = *tmp.132
            tmp.134 = add_ptr(tmp.133, index=8L, scale=1)
            tmp.135 = *tmp.134
            *tmp.131 = tmp.135
            tmp.136 = *tmp.131
            tmp.137 = add_ptr(s_ptr.11, index=16L, scale=1)
            tmp.138 = sign_extend 0
            tmp.139 = add_ptr(tmp.137, index=tmp.138, scale=24)
            tmp.140 = add_ptr(tmp.139, index=8L, scale=1)
            tmp.141 = *tmp.140
            tmp.142 = sign_extend tmp.141
            tmp.143 = tmp.142 != 120
            if !tmp.143 jump end_if_40
            return 0
        
          end_if_40:
            return 1
            return 0
        }
        global function test_static_arrow() { 
            tmp.144 = &s.14
            s_ptr.15 = tmp.144
            tmp.145 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.146 = &in.13
            *tmp.145 = tmp.146
            tmp.147 = *tmp.145
            *s_ptr.15 = 4294967295UL
            tmp.148 = *s_ptr.15
            tmp.149 = add_ptr(s_ptr.15, index=112L, scale=1)
            tmp.150 = - 5
            *tmp.149 = tmp.150
            tmp.151 = *tmp.149
            tmp.152 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.153 = *tmp.152
            *tmp.153 = 10D
            tmp.154 = *tmp.153
            tmp.155 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.156 = *tmp.155
            tmp.157 = add_ptr(tmp.156, index=8L, scale=1)
            tmp.158 = truncate 120
            *tmp.157 = tmp.158
            tmp.159 = *tmp.157
            tmp.160 = add_ptr(s_ptr.15, index=16L, scale=1)
            *tmp.160 = 11D
            tmp.161 = *tmp.160
            tmp.162 = add_ptr(s_ptr.15, index=16L, scale=1)
            tmp.164 = sign_extend 3
            tmp.163 = add_ptr(tmp.162, index=tmp.164, scale=24)
            *tmp.163 = 12D
            tmp.165 = *tmp.163
            tmp.166 = add_ptr(s_ptr.15, index=16L, scale=1)
            tmp.167 = add_ptr(tmp.166, index=16L, scale=1)
            tmp.168 = add_ptr(s_ptr.15, index=112L, scale=1)
            tmp.169 = tmp.168
            *tmp.167 = tmp.169
            tmp.170 = *tmp.167
            tmp.171 = *s_ptr.15
            tmp.172 = tmp.171 != 4294967295UL
            if tmp.172 jump or_true_42
            tmp.175 = add_ptr(s_ptr.15, index=112L, scale=1)
            tmp.176 = *tmp.175
            tmp.178 = - 5
            tmp.177 = tmp.176 != tmp.178
            if tmp.177 jump or_true_42
            tmp.174 = 0
            jump or_end_43
        
          or_true_42:
            tmp.174 = 1
        
          or_end_43:
            if !tmp.174 jump end_if_44
            return 0
        
          end_if_44:
            tmp.179 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.180 = *tmp.179
            tmp.181 = *tmp.180
            tmp.182 = tmp.181 != 10D
            if tmp.182 jump or_true_46
            tmp.185 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.186 = *tmp.185
            tmp.187 = add_ptr(tmp.186, index=8L, scale=1)
            tmp.188 = *tmp.187
            tmp.189 = sign_extend tmp.188
            tmp.190 = tmp.189 != 120
            if tmp.190 jump or_true_46
            tmp.184 = 0
            jump or_end_47
        
          or_true_46:
            tmp.184 = 1
        
          or_end_47:
            if tmp.184 jump or_true_48
            tmp.193 = add_ptr(s_ptr.15, index=16L, scale=1)
            tmp.194 = *tmp.193
            tmp.195 = tmp.194 != 11D
            if tmp.195 jump or_true_48
            tmp.192 = 0
            jump or_end_49
        
          or_true_48:
            tmp.192 = 1
        
          or_end_49:
            if tmp.192 jump or_true_50
            tmp.198 = add_ptr(s_ptr.15, index=16L, scale=1)
            tmp.200 = sign_extend 3
            tmp.199 = add_ptr(tmp.198, index=tmp.200, scale=24)
            tmp.201 = *tmp.199
            tmp.202 = tmp.201 != 12D
            if tmp.202 jump or_true_50
            tmp.197 = 0
            jump or_end_51
        
          or_true_50:
            tmp.197 = 1
        
          or_end_51:
            if !tmp.197 jump end_if_52
            return 0
        
          end_if_52:
            tmp.203 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.204 = *tmp.203
            tmp.205 = add_ptr(tmp.204, index=8L, scale=1)
            char_ptr.16 = tmp.205
            tmp.206 = *char_ptr.16
            tmp.207 = sign_extend tmp.206
            tmp.208 = tmp.207 != 120
            if !tmp.208 jump end_if_54
            return 0
        
          end_if_54:
            tmp.209 = add_ptr(s_ptr.15, index=16L, scale=1)
            tmp.210 = add_ptr(tmp.209, index=16L, scale=1)
            tmp.211 = *tmp.210
            *tmp.211 = 123
            tmp.212 = *tmp.211
            tmp.213 = add_ptr(s_ptr.15, index=112L, scale=1)
            tmp.214 = *tmp.213
            tmp.215 = tmp.214 != 123
            if !tmp.215 jump end_if_56
            return 0
        
          end_if_56:
            tmp.216 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.217 = *tmp.216
            tmp.218 = add_ptr(tmp.217, index=8L, scale=1)
            tmp.219 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.220 = *tmp.219
            tmp.221 = *tmp.220
            tmp.222 = double_to_int tmp.221
            *tmp.218 = tmp.222
            tmp.223 = *tmp.218
            tmp.224 = add_ptr(s_ptr.15, index=8L, scale=1)
            tmp.225 = *tmp.224
            tmp.226 = add_ptr(tmp.225, index=8L, scale=1)
            tmp.227 = *tmp.226
            tmp.228 = sign_extend tmp.227
            tmp.229 = tmp.228 != 10
            if !tmp.229 jump end_if_58
            return 0
        
          end_if_58:
            return 1
            return 0
        }
        global function test_mixed() { 
            tmp.230 = malloc(24UL)
            tmp.231 = tmp.230
            in_ptr.17 = tmp.231
            out.18[8] = in_ptr.17
            tmp.232 = out.18[8]
            tmp.233 = &out.18
            out_ptr.19 = tmp.233
            tmp.234 = sign_extend 10
            out.18[0] = tmp.234
            tmp.235 = out.18[0]
            out.18[112] = 20
            tmp.236 = out.18[112]
            tmp.237 = out.18[8]
            tmp.238 = - 1D
            *tmp.237 = tmp.238
            tmp.239 = *tmp.237
            tmp.240 = out.18[8]
            tmp.241 = add_ptr(tmp.240, index=8L, scale=1)
            tmp.242 = truncate 33
            *tmp.241 = tmp.242
            tmp.243 = *tmp.241
            tmp.244 = out.18[8]
            tmp.245 = add_ptr(tmp.244, index=16L, scale=1)
            tmp.246 = sign_extend 0
            *tmp.245 = tmp.246
            tmp.247 = *tmp.245
            tmp.248 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.249 = sign_extend 0
            tmp.250 = add_ptr(tmp.248, index=tmp.249, scale=24)
            tmp.251 = - 2D
            *tmp.250 = tmp.251
            tmp.252 = *tmp.250
            tmp.253 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.254 = sign_extend 0
            tmp.255 = add_ptr(tmp.253, index=tmp.254, scale=24)
            tmp.256 = add_ptr(tmp.255, index=8L, scale=1)
            tmp.257 = truncate 63
            *tmp.256 = tmp.257
            tmp.258 = *tmp.256
            tmp.259 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.260 = sign_extend 0
            tmp.261 = add_ptr(tmp.259, index=tmp.260, scale=24)
            tmp.262 = add_ptr(tmp.261, index=16L, scale=1)
            tmp.263 = sign_extend 0
            *tmp.262 = tmp.263
            tmp.264 = *tmp.262
            tmp.265 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.266 = sign_extend 3
            tmp.267 = add_ptr(tmp.265, index=tmp.266, scale=24)
            tmp.268 = - 3D
            *tmp.267 = tmp.268
            tmp.269 = *tmp.267
            tmp.270 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.271 = sign_extend 3
            tmp.272 = add_ptr(tmp.270, index=tmp.271, scale=24)
            tmp.273 = add_ptr(tmp.272, index=8L, scale=1)
            tmp.274 = truncate 42
            *tmp.273 = tmp.274
            tmp.275 = *tmp.273
            tmp.276 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.277 = sign_extend 3
            tmp.278 = add_ptr(tmp.276, index=tmp.277, scale=24)
            tmp.279 = add_ptr(tmp.278, index=16L, scale=1)
            tmp.280 = malloc(4UL)
            tmp.281 = tmp.280
            *tmp.279 = tmp.281
            tmp.282 = *tmp.279
            tmp.283 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.284 = - 3D
            *tmp.283 = tmp.284
            tmp.285 = *tmp.283
            tmp.286 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.287 = add_ptr(tmp.286, index=8L, scale=1)
            tmp.288 = truncate 38
            *tmp.287 = tmp.288
            tmp.289 = *tmp.287
            i.20 = 9
            tmp.290 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.291 = add_ptr(tmp.290, index=16L, scale=1)
            tmp.292 = &i.20
            *tmp.291 = tmp.292
            tmp.293 = *tmp.291
            tmp.294 = out.18[0]
            tmp.296 = sign_extend 10
            tmp.295 = tmp.294 != tmp.296
            if tmp.295 jump or_true_60
            tmp.299 = out.18[112]
            tmp.300 = tmp.299 != 20
            if tmp.300 jump or_true_60
            tmp.298 = 0
            jump or_end_61
        
          or_true_60:
            tmp.298 = 1
        
          or_end_61:
            if !tmp.298 jump end_if_62
            return 0
        
          end_if_62:
            tmp.301 = out.18[8]
            tmp.302 = *tmp.301
            tmp.304 = - 1D
            tmp.303 = tmp.302 != tmp.304
            if tmp.303 jump or_true_64
            tmp.307 = out.18[8]
            tmp.308 = add_ptr(tmp.307, index=8L, scale=1)
            tmp.309 = *tmp.308
            tmp.310 = sign_extend tmp.309
            tmp.311 = tmp.310 != 33
            if tmp.311 jump or_true_64
            tmp.306 = 0
            jump or_end_65
        
          or_true_64:
            tmp.306 = 1
        
          or_end_65:
            if tmp.306 jump or_true_66
            tmp.314 = out.18[8]
            tmp.315 = add_ptr(tmp.314, index=16L, scale=1)
            tmp.316 = *tmp.315
            if tmp.316 jump or_true_66
            tmp.313 = 0
            jump or_end_67
        
          or_true_66:
            tmp.313 = 1
        
          or_end_67:
            if !tmp.313 jump end_if_68
            return 0
        
          end_if_68:
            tmp.317 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.318 = sign_extend 0
            tmp.319 = add_ptr(tmp.317, index=tmp.318, scale=24)
            tmp.320 = *tmp.319
            tmp.322 = - 2D
            tmp.321 = tmp.320 != tmp.322
            if tmp.321 jump or_true_70
            tmp.325 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.326 = sign_extend 0
            tmp.327 = add_ptr(tmp.325, index=tmp.326, scale=24)
            tmp.328 = add_ptr(tmp.327, index=8L, scale=1)
            tmp.329 = *tmp.328
            tmp.330 = sign_extend tmp.329
            tmp.331 = tmp.330 != 63
            if tmp.331 jump or_true_70
            tmp.324 = 0
            jump or_end_71
        
          or_true_70:
            tmp.324 = 1
        
          or_end_71:
            if tmp.324 jump or_true_72
            tmp.334 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.335 = sign_extend 0
            tmp.336 = add_ptr(tmp.334, index=tmp.335, scale=24)
            tmp.337 = add_ptr(tmp.336, index=16L, scale=1)
            tmp.338 = *tmp.337
            if tmp.338 jump or_true_72
            tmp.333 = 0
            jump or_end_73
        
          or_true_72:
            tmp.333 = 1
        
          or_end_73:
            if tmp.333 jump or_true_74
            tmp.341 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.342 = sign_extend 3
            tmp.343 = add_ptr(tmp.341, index=tmp.342, scale=24)
            tmp.344 = *tmp.343
            tmp.346 = - 3D
            tmp.345 = tmp.344 != tmp.346
            if tmp.345 jump or_true_74
            tmp.340 = 0
            jump or_end_75
        
          or_true_74:
            tmp.340 = 1
        
          or_end_75:
            if tmp.340 jump or_true_76
            tmp.349 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.350 = sign_extend 3
            tmp.351 = add_ptr(tmp.349, index=tmp.350, scale=24)
            tmp.352 = add_ptr(tmp.351, index=8L, scale=1)
            tmp.353 = *tmp.352
            tmp.354 = sign_extend tmp.353
            tmp.355 = tmp.354 != 42
            if tmp.355 jump or_true_76
            tmp.348 = 0
            jump or_end_77
        
          or_true_76:
            tmp.348 = 1
        
          or_end_77:
            if tmp.348 jump or_true_78
            tmp.358 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.359 = *tmp.358
            tmp.361 = - 3D
            tmp.360 = tmp.359 != tmp.361
            if tmp.360 jump or_true_78
            tmp.357 = 0
            jump or_end_79
        
          or_true_78:
            tmp.357 = 1
        
          or_end_79:
            if tmp.357 jump or_true_80
            tmp.364 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.365 = add_ptr(tmp.364, index=8L, scale=1)
            tmp.366 = *tmp.365
            tmp.367 = sign_extend tmp.366
            tmp.368 = tmp.367 != 38
            if tmp.368 jump or_true_80
            tmp.363 = 0
            jump or_end_81
        
          or_true_80:
            tmp.363 = 1
        
          or_end_81:
            if tmp.363 jump or_true_82
            tmp.371 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.372 = add_ptr(tmp.371, index=16L, scale=1)
            tmp.373 = *tmp.372
            tmp.375 = &i.20
            tmp.374 = tmp.373 != tmp.375
            if tmp.374 jump or_true_82
            tmp.370 = 0
            jump or_end_83
        
          or_true_82:
            tmp.370 = 1
        
          or_end_83:
            if !tmp.370 jump end_if_84
            return 0
        
          end_if_84:
            tmp.376 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.377 = sign_extend 3
            tmp.378 = add_ptr(tmp.376, index=tmp.377, scale=24)
            tmp.379 = add_ptr(tmp.378, index=16L, scale=1)
            tmp.380 = *tmp.379
            *tmp.380 = 5
            tmp.381 = *tmp.380
            tmp.382 = add_ptr(out_ptr.19, index=16L, scale=1)
            tmp.383 = sign_extend 3
            tmp.384 = add_ptr(tmp.382, index=tmp.383, scale=24)
            tmp.385 = add_ptr(tmp.384, index=16L, scale=1)
            tmp.386 = *tmp.385
            tmp.387 = *tmp.386
            tmp.388 = tmp.387 != 5
            if !tmp.388 jump end_if_86
            return 0
        
          end_if_86:
            tmp.389 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.390 = add_ptr(tmp.389, index=8L, scale=1)
            tmp.391 = out.18[8]
            tmp.392 = add_ptr(tmp.391, index=8L, scale=1)
            tmp.393 = *tmp.392
            *tmp.390 = tmp.393
            tmp.394 = *tmp.390
            tmp.395 = add_ptr(out_ptr.19, index=120L, scale=1)
            tmp.396 = add_ptr(tmp.395, index=8L, scale=1)
            tmp.397 = *tmp.396
            tmp.398 = sign_extend tmp.397
            tmp.400 = out.18[8]
            tmp.401 = add_ptr(tmp.400, index=8L, scale=1)
            tmp.402 = *tmp.401
            tmp.403 = sign_extend tmp.402
            tmp.399 = tmp.398 != tmp.403
            if !tmp.399 jump end_if_88
            return 0
        
          end_if_88:
            return 1
            return 0
        }
        global function test_array_of_structs() { 
            tmp.404 = malloc(24UL)
            tmp.405 = tmp.404
            in_ptr.22 = tmp.405
            tmp.406 = &struct_array.21
            tmp.407 = sign_extend 0
            tmp.408 = add_ptr(tmp.406, index=tmp.407, scale=144)
            tmp.409 = add_ptr(tmp.408, index=8L, scale=1)
            *tmp.409 = in_ptr.22
            tmp.410 = *tmp.409
            tmp.411 = &struct_array.21
            tmp.412 = sign_extend 1
            tmp.413 = add_ptr(tmp.411, index=tmp.412, scale=144)
            tmp.414 = add_ptr(tmp.413, index=8L, scale=1)
            *tmp.414 = in_ptr.22
            tmp.415 = *tmp.414
            tmp.416 = &struct_array.21
            tmp.417 = sign_extend 0
            tmp.418 = add_ptr(tmp.416, index=tmp.417, scale=144)
            tmp.419 = add_ptr(tmp.418, index=8L, scale=1)
            tmp.420 = *tmp.419
            *tmp.420 = 20D
            tmp.421 = *tmp.420
            tmp.422 = &struct_array.21
            tmp.423 = sign_extend 1
            tmp.424 = add_ptr(tmp.422, index=tmp.423, scale=144)
            tmp.425 = add_ptr(tmp.424, index=8L, scale=1)
            tmp.426 = *tmp.425
            tmp.427 = add_ptr(tmp.426, index=8L, scale=1)
            tmp.428 = truncate 64
            *tmp.427 = tmp.428
            tmp.429 = *tmp.427
            tmp.430 = &struct_array.21
            tmp.431 = sign_extend 0
            tmp.432 = add_ptr(tmp.430, index=tmp.431, scale=144)
            tmp.433 = add_ptr(tmp.432, index=8L, scale=1)
            tmp.434 = *tmp.433
            tmp.435 = add_ptr(tmp.434, index=16L, scale=1)
            tmp.436 = sign_extend 0
            *tmp.435 = tmp.436
            tmp.437 = *tmp.435
            tmp.438 = &struct_array.21
            tmp.439 = sign_extend 1
            tmp.440 = add_ptr(tmp.438, index=tmp.439, scale=144)
            tmp.441 = add_ptr(tmp.440, index=16L, scale=1)
            tmp.442 = sign_extend 1
            tmp.443 = add_ptr(tmp.441, index=tmp.442, scale=24)
            *tmp.443 = 30D
            tmp.444 = *tmp.443
            tmp.445 = &struct_array.21
            tmp.446 = sign_extend 1
            tmp.447 = add_ptr(tmp.445, index=tmp.446, scale=144)
            tmp.448 = add_ptr(tmp.447, index=16L, scale=1)
            tmp.449 = sign_extend 0
            tmp.450 = add_ptr(tmp.448, index=tmp.449, scale=24)
            tmp.451 = add_ptr(tmp.450, index=8L, scale=1)
            tmp.452 = truncate 35
            *tmp.451 = tmp.452
            tmp.453 = *tmp.451
            tmp.454 = &struct_array.21
            tmp.455 = sign_extend 2
            tmp.456 = add_ptr(tmp.454, index=tmp.455, scale=144)
            tmp.457 = add_ptr(tmp.456, index=120L, scale=1)
            *tmp.457 = 40D
            tmp.458 = *tmp.457
            tmp.459 = &struct_array.21
            tmp.460 = sign_extend 2
            tmp.461 = add_ptr(tmp.459, index=tmp.460, scale=144)
            tmp.462 = add_ptr(tmp.461, index=120L, scale=1)
            tmp.463 = add_ptr(tmp.462, index=8L, scale=1)
            tmp.464 = truncate 36
            *tmp.463 = tmp.464
            tmp.465 = *tmp.463
            tmp.466 = &struct_array.21
            tmp.467 = sign_extend 1
            tmp.468 = add_ptr(tmp.466, index=tmp.467, scale=144)
            tmp.469 = add_ptr(tmp.468, index=8L, scale=1)
            tmp.470 = *tmp.469
            tmp.471 = *tmp.470
            tmp.472 = tmp.471 != 20D
            if tmp.472 jump or_true_90
            tmp.475 = &struct_array.21
            tmp.476 = sign_extend 0
            tmp.477 = add_ptr(tmp.475, index=tmp.476, scale=144)
            tmp.478 = add_ptr(tmp.477, index=8L, scale=1)
            tmp.479 = *tmp.478
            tmp.480 = add_ptr(tmp.479, index=8L, scale=1)
            tmp.481 = *tmp.480
            tmp.482 = sign_extend tmp.481
            tmp.483 = tmp.482 != 64
            if tmp.483 jump or_true_90
            tmp.474 = 0
            jump or_end_91
        
          or_true_90:
            tmp.474 = 1
        
          or_end_91:
            if tmp.474 jump or_true_92
            tmp.486 = &struct_array.21
            tmp.487 = sign_extend 1
            tmp.488 = add_ptr(tmp.486, index=tmp.487, scale=144)
            tmp.489 = add_ptr(tmp.488, index=8L, scale=1)
            tmp.490 = *tmp.489
            tmp.491 = add_ptr(tmp.490, index=16L, scale=1)
            tmp.492 = *tmp.491
            if tmp.492 jump or_true_92
            tmp.485 = 0
            jump or_end_93
        
          or_true_92:
            tmp.485 = 1
        
          or_end_93:
            if !tmp.485 jump end_if_94
            return 0
        
          end_if_94:
            tmp.493 = &struct_array.21
            tmp.494 = sign_extend 1
            tmp.495 = add_ptr(tmp.493, index=tmp.494, scale=144)
            tmp.496 = add_ptr(tmp.495, index=16L, scale=1)
            tmp.497 = sign_extend 1
            tmp.498 = add_ptr(tmp.496, index=tmp.497, scale=24)
            tmp.499 = *tmp.498
            tmp.500 = tmp.499 != 30D
            if tmp.500 jump or_true_96
            tmp.503 = &struct_array.21
            tmp.504 = sign_extend 1
            tmp.505 = add_ptr(tmp.503, index=tmp.504, scale=144)
            tmp.506 = add_ptr(tmp.505, index=16L, scale=1)
            tmp.507 = sign_extend 0
            tmp.508 = add_ptr(tmp.506, index=tmp.507, scale=24)
            tmp.509 = add_ptr(tmp.508, index=8L, scale=1)
            tmp.510 = *tmp.509
            tmp.511 = sign_extend tmp.510
            tmp.512 = tmp.511 != 35
            if tmp.512 jump or_true_96
            tmp.502 = 0
            jump or_end_97
        
          or_true_96:
            tmp.502 = 1
        
          or_end_97:
            if tmp.502 jump or_true_98
            tmp.515 = &struct_array.21
            tmp.516 = sign_extend 2
            tmp.517 = add_ptr(tmp.515, index=tmp.516, scale=144)
            tmp.518 = add_ptr(tmp.517, index=120L, scale=1)
            tmp.519 = *tmp.518
            tmp.520 = tmp.519 != 40D
            if tmp.520 jump or_true_98
            tmp.514 = 0
            jump or_end_99
        
          or_true_98:
            tmp.514 = 1
        
          or_end_99:
            if tmp.514 jump or_true_100
            tmp.523 = &struct_array.21
            tmp.524 = sign_extend 2
            tmp.525 = add_ptr(tmp.523, index=tmp.524, scale=144)
            tmp.526 = add_ptr(tmp.525, index=120L, scale=1)
            tmp.527 = add_ptr(tmp.526, index=8L, scale=1)
            tmp.528 = *tmp.527
            tmp.529 = sign_extend tmp.528
            tmp.530 = tmp.529 != 36
            if tmp.530 jump or_true_100
            tmp.522 = 0
            jump or_end_101
        
          or_true_100:
            tmp.522 = 1
        
          or_end_101:
            if !tmp.522 jump end_if_102
            return 0
        
          end_if_102:
            return 1
            return 0
        }
        global function test_array_of_struct_pointers() { 
            tmp.531 = &ptr_array.23
            tmp.532 = sign_extend 0
            tmp.533 = add_ptr(tmp.531, index=tmp.532, scale=8)
            tmp.534 = sign_extend 1
            tmp.535 = calloc(tmp.534, 144UL)
            tmp.536 = tmp.535
            *tmp.533 = tmp.536
            tmp.537 = *tmp.533
            tmp.538 = &ptr_array.23
            tmp.539 = sign_extend 1
            tmp.540 = add_ptr(tmp.538, index=tmp.539, scale=8)
            tmp.541 = sign_extend 1
            tmp.542 = calloc(tmp.541, 144UL)
            tmp.543 = tmp.542
            *tmp.540 = tmp.543
            tmp.544 = *tmp.540
            tmp.545 = &ptr_array.23
            tmp.546 = sign_extend 1
            tmp.547 = add_ptr(tmp.545, index=tmp.546, scale=8)
            tmp.548 = *tmp.547
            tmp.549 = add_ptr(tmp.548, index=8L, scale=1)
            tmp.550 = sign_extend 1
            tmp.551 = calloc(tmp.550, 24UL)
            tmp.552 = tmp.551
            *tmp.549 = tmp.552
            tmp.553 = *tmp.549
            tmp.554 = &ptr_array.23
            tmp.555 = sign_extend 1
            tmp.556 = add_ptr(tmp.554, index=tmp.555, scale=8)
            tmp.557 = *tmp.556
            tmp.558 = add_ptr(tmp.557, index=8L, scale=1)
            tmp.559 = *tmp.558
            tmp.560 = add_ptr(tmp.559, index=16L, scale=1)
            tmp.561 = sign_extend 0
            *tmp.560 = tmp.561
            tmp.562 = *tmp.560
            tmp.563 = &ptr_array.23
            tmp.564 = sign_extend 1
            tmp.565 = add_ptr(tmp.563, index=tmp.564, scale=8)
            tmp.566 = *tmp.565
            tmp.567 = add_ptr(tmp.566, index=8L, scale=1)
            tmp.568 = *tmp.567
            tmp.569 = add_ptr(tmp.568, index=8L, scale=1)
            tmp.570 = truncate 37
            *tmp.569 = tmp.570
            tmp.571 = *tmp.569
            tmp.572 = &ptr_array.23
            tmp.573 = sign_extend 1
            tmp.574 = add_ptr(tmp.572, index=tmp.573, scale=8)
            tmp.575 = *tmp.574
            tmp.576 = add_ptr(tmp.575, index=8L, scale=1)
            tmp.577 = *tmp.576
            *tmp.577 = 876.5D
            tmp.578 = *tmp.577
            tmp.579 = &ptr_array.23
            tmp.580 = sign_extend 1
            tmp.581 = add_ptr(tmp.579, index=tmp.580, scale=8)
            tmp.582 = *tmp.581
            tmp.583 = add_ptr(tmp.582, index=16L, scale=1)
            tmp.584 = sign_extend 2
            tmp.585 = add_ptr(tmp.583, index=tmp.584, scale=24)
            *tmp.585 = 1000.5D
            tmp.586 = *tmp.585
            tmp.587 = &ptr_array.23
            tmp.588 = sign_extend 1
            tmp.589 = add_ptr(tmp.587, index=tmp.588, scale=8)
            tmp.590 = *tmp.589
            tmp.591 = add_ptr(tmp.590, index=120L, scale=1)
            *tmp.591 = 7000000D
            tmp.592 = *tmp.591
            tmp.593 = &ptr_array.23
            tmp.594 = sign_extend 0
            tmp.595 = add_ptr(tmp.593, index=tmp.594, scale=8)
            tmp.596 = *tmp.595
            tmp.597 = add_ptr(tmp.596, index=8L, scale=1)
            tmp.598 = sign_extend 1
            tmp.599 = calloc(tmp.598, 24UL)
            tmp.600 = tmp.599
            *tmp.597 = tmp.600
            tmp.601 = *tmp.597
            tmp.602 = &ptr_array.23
            tmp.603 = sign_extend 0
            tmp.604 = add_ptr(tmp.602, index=tmp.603, scale=8)
            tmp.605 = *tmp.604
            tmp.606 = add_ptr(tmp.605, index=8L, scale=1)
            tmp.607 = *tmp.606
            tmp.608 = add_ptr(tmp.607, index=16L, scale=1)
            tmp.609 = sign_extend 0
            *tmp.608 = tmp.609
            tmp.610 = *tmp.608
            tmp.611 = &ptr_array.23
            tmp.612 = sign_extend 0
            tmp.613 = add_ptr(tmp.611, index=tmp.612, scale=8)
            tmp.614 = *tmp.613
            tmp.615 = add_ptr(tmp.614, index=8L, scale=1)
            tmp.616 = *tmp.615
            tmp.617 = add_ptr(tmp.616, index=8L, scale=1)
            tmp.618 = truncate 94
            *tmp.617 = tmp.618
            tmp.619 = *tmp.617
            tmp.620 = &ptr_array.23
            tmp.621 = sign_extend 0
            tmp.622 = add_ptr(tmp.620, index=tmp.621, scale=8)
            tmp.623 = *tmp.622
            tmp.624 = add_ptr(tmp.623, index=8L, scale=1)
            tmp.625 = *tmp.624
            *tmp.625 = 123.4D
            tmp.626 = *tmp.625
            tmp.627 = &ptr_array.23
            tmp.628 = sign_extend 0
            tmp.629 = add_ptr(tmp.627, index=tmp.628, scale=8)
            tmp.630 = *tmp.629
            tmp.631 = add_ptr(tmp.630, index=16L, scale=1)
            tmp.632 = sign_extend 1
            tmp.633 = add_ptr(tmp.631, index=tmp.632, scale=24)
            tmp.634 = add_ptr(tmp.633, index=8L, scale=1)
            tmp.635 = truncate 38
            *tmp.634 = tmp.635
            tmp.636 = *tmp.634
            tmp.637 = &ptr_array.23
            tmp.638 = sign_extend 0
            tmp.639 = add_ptr(tmp.637, index=tmp.638, scale=8)
            tmp.640 = *tmp.639
            tmp.641 = add_ptr(tmp.640, index=120L, scale=1)
            tmp.642 = add_ptr(tmp.641, index=16L, scale=1)
            tmp.643 = &ptr_array.23
            tmp.644 = sign_extend 0
            tmp.645 = add_ptr(tmp.643, index=tmp.644, scale=8)
            tmp.646 = *tmp.645
            tmp.647 = add_ptr(tmp.646, index=112L, scale=1)
            tmp.648 = tmp.647
            *tmp.642 = tmp.648
            tmp.649 = *tmp.642
            tmp.650 = &ptr_array.23
            tmp.651 = sign_extend 0
            tmp.652 = add_ptr(tmp.650, index=tmp.651, scale=8)
            tmp.653 = *tmp.652
            tmp.654 = add_ptr(tmp.653, index=112L, scale=1)
            *tmp.654 = 900
            tmp.655 = *tmp.654
            tmp.656 = &ptr_array.23
            tmp.657 = sign_extend 0
            tmp.658 = add_ptr(tmp.656, index=tmp.657, scale=8)
            tmp.659 = *tmp.658
            tmp.660 = add_ptr(tmp.659, index=16L, scale=1)
            tmp.661 = sign_extend 1
            tmp.662 = add_ptr(tmp.660, index=tmp.661, scale=24)
            tmp.663 = add_ptr(tmp.662, index=8L, scale=1)
            tmp.664 = *tmp.663
            tmp.665 = sign_extend tmp.664
            tmp.666 = tmp.665 != 38
            if !tmp.666 jump end_if_104
            return 0
        
          end_if_104:
            tmp.667 = &ptr_array.23
            tmp.668 = sign_extend 0
            tmp.669 = add_ptr(tmp.667, index=tmp.668, scale=8)
            tmp.670 = *tmp.669
            tmp.671 = add_ptr(tmp.670, index=8L, scale=1)
            tmp.672 = *tmp.671
            tmp.673 = *tmp.672
            tmp.674 = tmp.673 != 123.4D
            if tmp.674 jump or_true_106
            tmp.677 = &ptr_array.23
            tmp.678 = sign_extend 0
            tmp.679 = add_ptr(tmp.677, index=tmp.678, scale=8)
            tmp.680 = *tmp.679
            tmp.681 = add_ptr(tmp.680, index=8L, scale=1)
            tmp.682 = *tmp.681
            tmp.683 = add_ptr(tmp.682, index=8L, scale=1)
            tmp.684 = *tmp.683
            tmp.685 = sign_extend tmp.684
            tmp.686 = tmp.685 != 94
            if tmp.686 jump or_true_106
            tmp.676 = 0
            jump or_end_107
        
          or_true_106:
            tmp.676 = 1
        
          or_end_107:
            if tmp.676 jump or_true_108
            tmp.689 = &ptr_array.23
            tmp.690 = sign_extend 0
            tmp.691 = add_ptr(tmp.689, index=tmp.690, scale=8)
            tmp.692 = *tmp.691
            tmp.693 = add_ptr(tmp.692, index=8L, scale=1)
            tmp.694 = *tmp.693
            tmp.695 = add_ptr(tmp.694, index=16L, scale=1)
            tmp.696 = *tmp.695
            if tmp.696 jump or_true_108
            tmp.688 = 0
            jump or_end_109
        
          or_true_108:
            tmp.688 = 1
        
          or_end_109:
            if !tmp.688 jump end_if_110
            return 0
        
          end_if_110:
            tmp.697 = &ptr_array.23
            tmp.698 = sign_extend 1
            tmp.699 = add_ptr(tmp.697, index=tmp.698, scale=8)
            tmp.700 = *tmp.699
            tmp.701 = add_ptr(tmp.700, index=120L, scale=1)
            tmp.702 = *tmp.701
            tmp.703 = tmp.702 != 7000000D
            if !tmp.703 jump end_if_112
            return 0
        
          end_if_112:
            tmp.704 = &ptr_array.23
            tmp.705 = sign_extend 1
            tmp.706 = add_ptr(tmp.704, index=tmp.705, scale=8)
            tmp.707 = *tmp.706
            tmp.708 = add_ptr(tmp.707, index=16L, scale=1)
            tmp.709 = sign_extend 2
            tmp.710 = add_ptr(tmp.708, index=tmp.709, scale=24)
            tmp.711 = *tmp.710
            tmp.712 = tmp.711 != 1000.5D
            if !tmp.712 jump end_if_114
            return 0
        
          end_if_114:
            tmp.713 = &ptr_array.23
            tmp.714 = sign_extend 1
            tmp.715 = add_ptr(tmp.713, index=tmp.714, scale=8)
            tmp.716 = *tmp.715
            tmp.717 = add_ptr(tmp.716, index=8L, scale=1)
            tmp.718 = *tmp.717
            tmp.719 = *tmp.718
            tmp.720 = tmp.719 != 876.5D
            if tmp.720 jump or_true_116
            tmp.723 = &ptr_array.23
            tmp.724 = sign_extend 1
            tmp.725 = add_ptr(tmp.723, index=tmp.724, scale=8)
            tmp.726 = *tmp.725
            tmp.727 = add_ptr(tmp.726, index=8L, scale=1)
            tmp.728 = *tmp.727
            tmp.729 = add_ptr(tmp.728, index=8L, scale=1)
            tmp.730 = *tmp.729
            tmp.731 = sign_extend tmp.730
            tmp.732 = tmp.731 != 37
            if tmp.732 jump or_true_116
            tmp.722 = 0
            jump or_end_117
        
          or_true_116:
            tmp.722 = 1
        
          or_end_117:
            if tmp.722 jump or_true_118
            tmp.735 = &ptr_array.23
            tmp.736 = sign_extend 1
            tmp.737 = add_ptr(tmp.735, index=tmp.736, scale=8)
            tmp.738 = *tmp.737
            tmp.739 = add_ptr(tmp.738, index=8L, scale=1)
            tmp.740 = *tmp.739
            tmp.741 = add_ptr(tmp.740, index=16L, scale=1)
            tmp.742 = *tmp.741
            if tmp.742 jump or_true_118
            tmp.734 = 0
            jump or_end_119
        
          or_true_118:
            tmp.734 = 1
        
          or_end_119:
            if !tmp.734 jump end_if_120
            return 0
        
          end_if_120:
            tmp.743 = &ptr_array.23
            tmp.744 = sign_extend 0
            tmp.745 = add_ptr(tmp.743, index=tmp.744, scale=8)
            tmp.746 = *tmp.745
            tmp.747 = add_ptr(tmp.746, index=120L, scale=1)
            tmp.748 = add_ptr(tmp.747, index=16L, scale=1)
            tmp.749 = *tmp.748
            tmp.750 = *tmp.749
            tmp.751 = tmp.750 != 900
            if !tmp.751 jump end_if_122
            return 0
        
          end_if_122:
            tmp.752 = &ptr_array.23
            tmp.753 = sign_extend 0
            tmp.754 = add_ptr(tmp.752, index=tmp.753, scale=8)
            tmp.755 = *tmp.754
            tmp.756 = *tmp.755
            if !tmp.756 jump end_if_124
            return 0
        
          end_if_124:
            i.24 = 0
        
          start_loop_0:
            tmp.757 = i.24 < 4
            if !tmp.757 jump break_loop_0
            tmp.758 = &ptr_array.23
            tmp.759 = sign_extend 0
            tmp.760 = add_ptr(tmp.758, index=tmp.759, scale=8)
            tmp.761 = *tmp.760
            tmp.762 = add_ptr(tmp.761, index=16L, scale=1)
            tmp.763 = sign_extend i.24
            tmp.764 = add_ptr(tmp.762, index=tmp.763, scale=24)
            elem_ptr.25 = tmp.764
            tmp.765 = *elem_ptr.25
            if tmp.765 jump or_true_126
            tmp.768 = add_ptr(elem_ptr.25, index=16L, scale=1)
            tmp.769 = *tmp.768
            if tmp.769 jump or_true_126
            tmp.767 = 0
            jump or_end_127
        
          or_true_126:
            tmp.767 = 1
        
          or_end_127:
            if !tmp.767 jump end_if_128
            return 0
        
          end_if_128:
            tmp.770 = add_ptr(elem_ptr.25, index=8L, scale=1)
            tmp.771 = *tmp.770
            if !tmp.771 jump and_false_130
            tmp.774 = i.24 != 1
            if !tmp.774 jump and_false_130
            tmp.773 = 1
            jump and_end_131
        
          and_false_130:
            tmp.773 = 0
        
          and_end_131:
            if !tmp.773 jump end_if_132
            return 0
        
          end_if_132:
        
          continue_loop_0:
            tmp.775 = i.24 + 1
            i.24 = tmp.775
            jump start_loop_0
        
          break_loop_0:
            tmp.776 = &ptr_array.23
            tmp.777 = sign_extend 0
            tmp.778 = add_ptr(tmp.776, index=tmp.777, scale=8)
            tmp.779 = *tmp.778
            tmp.780 = add_ptr(tmp.779, index=120L, scale=1)
            tmp.781 = *tmp.780
            if tmp.781 jump or_true_134
            tmp.784 = &ptr_array.23
            tmp.785 = sign_extend 0
            tmp.786 = add_ptr(tmp.784, index=tmp.785, scale=8)
            tmp.787 = *tmp.786
            tmp.788 = add_ptr(tmp.787, index=120L, scale=1)
            tmp.789 = add_ptr(tmp.788, index=8L, scale=1)
            tmp.790 = *tmp.789
            if tmp.790 jump or_true_134
            tmp.783 = 0
            jump or_end_135
        
          or_true_134:
            tmp.783 = 1
        
          or_end_135:
            if !tmp.783 jump end_if_136
            return 0
        
          end_if_136:
            tmp.791 = &ptr_array.23
            tmp.792 = sign_extend 1
            tmp.793 = add_ptr(tmp.791, index=tmp.792, scale=8)
            tmp.794 = *tmp.793
            tmp.795 = *tmp.794
            if tmp.795 jump or_true_138
            tmp.798 = &ptr_array.23
            tmp.799 = sign_extend 1
            tmp.800 = add_ptr(tmp.798, index=tmp.799, scale=8)
            tmp.801 = *tmp.800
            tmp.802 = add_ptr(tmp.801, index=112L, scale=1)
            tmp.803 = *tmp.802
            if tmp.803 jump or_true_138
            tmp.797 = 0
            jump or_end_139
        
          or_true_138:
            tmp.797 = 1
        
          or_end_139:
            if !tmp.797 jump end_if_140
            return 0
        
          end_if_140:
            i.26 = 0
        
          start_loop_1:
            tmp.804 = i.26 < 4
            if !tmp.804 jump break_loop_1
            tmp.805 = &ptr_array.23
            tmp.806 = sign_extend 1
            tmp.807 = add_ptr(tmp.805, index=tmp.806, scale=8)
            tmp.808 = *tmp.807
            tmp.809 = add_ptr(tmp.808, index=16L, scale=1)
            tmp.810 = sign_extend i.26
            tmp.811 = add_ptr(tmp.809, index=tmp.810, scale=24)
            elem_ptr.27 = tmp.811
            tmp.812 = add_ptr(elem_ptr.27, index=8L, scale=1)
            tmp.813 = *tmp.812
            if tmp.813 jump or_true_142
            tmp.816 = add_ptr(elem_ptr.27, index=16L, scale=1)
            tmp.817 = *tmp.816
            if tmp.817 jump or_true_142
            tmp.815 = 0
            jump or_end_143
        
          or_true_142:
            tmp.815 = 1
        
          or_end_143:
            if !tmp.815 jump end_if_144
            return 0
        
          end_if_144:
            tmp.818 = *elem_ptr.27
            if !tmp.818 jump and_false_146
            tmp.821 = i.26 != 2
            if !tmp.821 jump and_false_146
            tmp.820 = 1
            jump and_end_147
        
          and_false_146:
            tmp.820 = 0
        
          and_end_147:
            if !tmp.820 jump end_if_148
            return 0
        
          end_if_148:
        
          continue_loop_1:
            tmp.822 = i.26 + 1
            i.26 = tmp.822
            jump start_loop_1
        
          break_loop_1:
            tmp.823 = &ptr_array.23
            tmp.824 = sign_extend 1
            tmp.825 = add_ptr(tmp.823, index=tmp.824, scale=8)
            tmp.826 = *tmp.825
            tmp.827 = add_ptr(tmp.826, index=120L, scale=1)
            tmp.828 = add_ptr(tmp.827, index=8L, scale=1)
            tmp.829 = *tmp.828
            if tmp.829 jump or_true_150
            tmp.832 = &ptr_array.23
            tmp.833 = sign_extend 1
            tmp.834 = add_ptr(tmp.832, index=tmp.833, scale=8)
            tmp.835 = *tmp.834
            tmp.836 = add_ptr(tmp.835, index=120L, scale=1)
            tmp.837 = add_ptr(tmp.836, index=16L, scale=1)
            tmp.838 = *tmp.837
            if tmp.838 jump or_true_150
            tmp.831 = 0
            jump or_end_151
        
          or_true_150:
            tmp.831 = 1
        
          or_end_151:
            if !tmp.831 jump end_if_152
            return 0
        
          end_if_152:
            return 1
            return 0
        }
        global function main() { 
            tmp.839 = test_auto_dot()
            tmp.840 = ! tmp.839
            if !tmp.840 jump end_if_154
            return 1
        
          end_if_154:
            tmp.841 = test_static_dot()
            tmp.842 = ! tmp.841
            if !tmp.842 jump end_if_156
            return 2
        
          end_if_156:
            tmp.843 = test_auto_arrow()
            tmp.844 = ! tmp.843
            if !tmp.844 jump end_if_158
            return 3
        
          end_if_158:
            tmp.845 = test_static_arrow()
            tmp.846 = ! tmp.845
            if !tmp.846 jump end_if_160
            return 4
        
          end_if_160:
            tmp.847 = test_mixed()
            tmp.848 = ! tmp.847
            if !tmp.848 jump end_if_162
            return 5
        
          end_if_162:
            tmp.849 = test_array_of_structs()
            tmp.850 = ! tmp.849
            if !tmp.850 jump end_if_164
            return 6
        
          end_if_164:
            tmp.851 = test_array_of_struct_pointers()
            tmp.852 = ! tmp.851
            if !tmp.852 jump end_if_166
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
            tmp.3 = static_struct.6[4]
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
            tmp.9 = static_struct.6[4]
            tmp.10 = putchar(tmp.9)
            tmp.11 = putchar(10)
        
          end_if_2:
            static_struct.6[0] = a.3
            tmp.12 = static_struct.6[0]
            static_struct.6[4] = b.4
            tmp.13 = static_struct.6[4]
            return 0
        }
        global function test_static_local_pointer(a.7, b.8) { 
            tmp.14 = ! struct_ptr.10
            if !tmp.14 jump else_5
            tmp.15 = malloc(8UL)
            tmp.16 = tmp.15
            struct_ptr.10 = tmp.16
            jump end_if_4
        
          else_5:
            tmp.17 = *struct_ptr.10
            tmp.18 = putchar(tmp.17)
            tmp.19 = add_ptr(struct_ptr.10, index=4L, scale=1)
            tmp.20 = *tmp.19
            tmp.21 = putchar(tmp.20)
            tmp.22 = putchar(10)
        
          end_if_4:
            *struct_ptr.10 = a.7
            tmp.23 = *struct_ptr.10
            tmp.24 = add_ptr(struct_ptr.10, index=4L, scale=1)
            *tmp.24 = b.8
            tmp.25 = *tmp.24
            return 0
        }
        global function f1() { 
            tmp.26 = g[0]
            tmp.27 = sign_extend tmp.26
            tmp.28 = tmp.27 + 1
            tmp.29 = truncate tmp.28
            g[0] = tmp.29
            tmp.30 = g[0]
            tmp.31 = g[1]
            tmp.32 = sign_extend tmp.31
            tmp.33 = tmp.32 + 1
            tmp.34 = truncate tmp.33
            g[1] = tmp.34
            tmp.35 = g[1]
            tmp.36 = g[2]
            tmp.37 = sign_extend tmp.36
            tmp.38 = tmp.37 + 1
            tmp.39 = truncate tmp.38
            g[2] = tmp.39
            tmp.40 = g[2]
            return 0
        }
        global function f2() { 
            tmp.41 = g[0]
            tmp.42 = sign_extend tmp.41
            tmp.43 = putchar(tmp.42)
            tmp.44 = g[1]
            tmp.45 = sign_extend tmp.44
            tmp.46 = putchar(tmp.45)
            tmp.47 = g[2]
            tmp.48 = sign_extend tmp.47
            tmp.49 = putchar(tmp.48)
            tmp.50 = putchar(10)
            return 0
        }
        global function test_global_struct() { 
            tmp.51 = truncate 65
            g[0] = tmp.51
            tmp.52 = g[0]
            tmp.53 = truncate 66
            g[1] = tmp.53
            tmp.54 = g[1]
            tmp.55 = truncate 67
            g[2] = tmp.55
            tmp.56 = g[2]
            f1()
            f2()
            f1()
            f2()
            return 0
        }
        global function f3() { 
            tmp.57 = *g_ptr
            tmp.58 = sign_extend tmp.57
            tmp.59 = tmp.58 + 1
            tmp.60 = truncate tmp.59
            *g_ptr = tmp.60
            tmp.61 = *g_ptr
            tmp.62 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.63 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.64 = *tmp.63
            tmp.65 = sign_extend tmp.64
            tmp.66 = tmp.65 + 1
            tmp.67 = truncate tmp.66
            *tmp.62 = tmp.67
            tmp.68 = *tmp.62
            tmp.69 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.70 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.71 = *tmp.70
            tmp.72 = sign_extend tmp.71
            tmp.73 = tmp.72 + 1
            tmp.74 = truncate tmp.73
            *tmp.69 = tmp.74
            tmp.75 = *tmp.69
            return 0
        }
        global function f4() { 
            tmp.76 = *g_ptr
            tmp.77 = sign_extend tmp.76
            tmp.78 = putchar(tmp.77)
            tmp.79 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.80 = *tmp.79
            tmp.81 = sign_extend tmp.80
            tmp.82 = putchar(tmp.81)
            tmp.83 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.84 = *tmp.83
            tmp.85 = sign_extend tmp.84
            tmp.86 = putchar(tmp.85)
            tmp.87 = putchar(10)
            return 0
        }
        global function test_global_struct_pointer() { 
            tmp.88 = &g
            g_ptr = tmp.88
            f3()
            f4()
            f3()
            f4()
            tmp.89 = malloc(3UL)
            tmp.90 = tmp.89
            g_ptr = tmp.90
            tmp.91 = truncate 97
            *g_ptr = tmp.91
            tmp.92 = *g_ptr
            tmp.93 = add_ptr(g_ptr, index=1L, scale=1)
            tmp.94 = truncate 98
            *tmp.93 = tmp.94
            tmp.95 = *tmp.93
            tmp.96 = add_ptr(g_ptr, index=2L, scale=1)
            tmp.97 = truncate 99
            *tmp.96 = tmp.97
            tmp.98 = *tmp.96
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
            x.1[0] = 1
            x.1[4] = 2
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
            val.11[0] = 1
            val.11[4] = 2
            tmp.1 = &val.11
            s_ptr.9 = tmp.1
            tmp.2 = *s_ptr.9
            tmp.3 = tmp.2 != 1
            if tmp.3 jump or_true_0
            tmp.6 = add_ptr(s_ptr.9, index=4L, scale=1)
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 != 2
            if tmp.8 jump or_true_0
            tmp.5 = 0
            jump or_end_1
        
          or_true_0:
            tmp.5 = 1
        
          or_end_1:
            if !tmp.5 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_file_scope_forward_decl() { 
            tmp.9 = make_struct()
            my_struct.14 = tmp.9
            tmp.10 = validate_struct(my_struct.14)
            return tmp.10
            return 0
        }
        global function make_struct() { 
            tmp.11 = malloc(16UL)
            tmp.12 = tmp.11
            retval.16 = tmp.12
            tmp.13 = sign_extend 100
            *retval.16 = tmp.13
            tmp.14 = *retval.16
            tmp.15 = add_ptr(retval.16, index=8L, scale=1)
            tmp.16 = sign_extend 200
            *tmp.15 = tmp.16
            tmp.17 = *tmp.15
            return retval.16
            return 0
        }
        global function validate_struct(ptr.17) { 
            tmp.18 = *ptr.17
            tmp.20 = sign_extend 100
            tmp.19 = tmp.18 == tmp.20
            if !tmp.19 jump and_false_4
            tmp.23 = add_ptr(ptr.17, index=8L, scale=1)
            tmp.24 = *tmp.23
            tmp.26 = sign_extend 200
            tmp.25 = tmp.24 == tmp.26
            if !tmp.25 jump and_false_4
            tmp.22 = 1
            jump and_end_5
        
          and_false_4:
            tmp.22 = 0
        
          and_end_5:
            return tmp.22
            return 0
        }
        global function test_incomplete_var() { 
            tmp.27 = &incomplete_var
            print_msg(tmp.27)
            tmp.28 = validate_incomplete_var()
            return tmp.28
            return 0
        }
        global function validate_incomplete_var() { 
            tmp.29 = incomplete_var[0]
            tmp.30 = &string.0
            tmp.31 = strcmp(tmp.29, tmp.30)
            if !tmp.31 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function print_msg(param.21) { 
            tmp.32 = *param.21
            tmp.33 = puts(tmp.32)
            return 0
        }
        global function test_deref_incomplete_var() { 
            tmp.34 = sign_extend 4
            tmp.35 = malloc(tmp.34)
            tmp.36 = tmp.35
            ptr.23 = tmp.36
            tmp.37 = ptr.23 == ptr.23
            return tmp.37
            return 0
        }
        global function use_struct_pointers(param.25) { 
            tmp.39 = sign_extend 0
            tmp.38 = param.25 == tmp.39
            if !tmp.38 jump end_if_8
            tmp.40 = &string.1
            tmp.41 = puts(tmp.40)
        
          end_if_8:
            tmp.42 = sign_extend 0
            return tmp.42
            return 0
        }
        global function test_use_incomplete_struct_pointers() { 
            tmp.43 = sign_extend 1
            tmp.44 = sign_extend 4
            tmp.45 = calloc(tmp.43, tmp.44)
            tmp.46 = tmp.45
            ptr1.26 = tmp.46
            tmp.47 = sign_extend 1
            tmp.48 = sign_extend 4
            tmp.49 = calloc(tmp.47, tmp.48)
            tmp.50 = tmp.49
            ptr2.27 = tmp.50
            tmp.51 = ptr1.26
            ptr1_bytes.28 = tmp.51
            tmp.52 = sign_extend 0
            tmp.53 = add_ptr(ptr1_bytes.28, index=tmp.52, scale=1)
            tmp.54 = *tmp.53
            if tmp.54 jump or_true_10
            tmp.57 = sign_extend 1
            tmp.58 = add_ptr(ptr1_bytes.28, index=tmp.57, scale=1)
            tmp.59 = *tmp.58
            if tmp.59 jump or_true_10
            tmp.56 = 0
            jump or_end_11
        
          or_true_10:
            tmp.56 = 1
        
          or_end_11:
            if !tmp.56 jump end_if_12
            return 0
        
          end_if_12:
            tmp.61 = sign_extend 0
            tmp.60 = ptr1.26 == tmp.61
            if tmp.60 jump or_true_14
            tmp.65 = sign_extend 0
            tmp.64 = ptr2.27 == tmp.65
            if tmp.64 jump or_true_14
            tmp.63 = 0
            jump or_end_15
        
          or_true_14:
            tmp.63 = 1
        
          or_end_15:
            if tmp.63 jump or_true_16
            tmp.68 = ptr1.26 == ptr2.27
            if tmp.68 jump or_true_16
            tmp.67 = 0
            jump or_end_17
        
          or_true_16:
            tmp.67 = 1
        
          or_end_17:
            if !tmp.67 jump end_if_18
            return 0
        
          end_if_18:
            if !flse.29 jump else_21
            tmp.69 = ptr1.26
            jump end_if_20
        
          else_21:
            tmp.69 = ptr2.27
        
          end_if_20:
            ptr3.30 = tmp.69
            tmp.70 = ptr3.30 != ptr2.27
            if !tmp.70 jump end_if_22
            return 0
        
          end_if_22:
            tmp.71 = use_struct_pointers(ptr3.30)
            if !tmp.71 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function main() { 
            tmp.72 = test_block_scope_forward_decl()
            tmp.73 = ! tmp.72
            if !tmp.73 jump end_if_26
            return 2
        
          end_if_26:
            tmp.74 = test_file_scope_forward_decl()
            tmp.75 = ! tmp.74
            if !tmp.75 jump end_if_28
            return 3
        
          end_if_28:
            tmp.76 = test_incomplete_var()
            tmp.77 = ! tmp.76
            if !tmp.77 jump end_if_30
            return 4
        
          end_if_30:
            tmp.78 = test_deref_incomplete_var()
            tmp.79 = ! tmp.78
            if !tmp.79 jump end_if_32
            return 5
        
          end_if_32:
            tmp.80 = test_use_incomplete_struct_pointers()
            tmp.81 = ! tmp.80
            if !tmp.81 jump end_if_34
            return 6
        
          end_if_34:
            return 0
            return 0
        }
        static flse.29: Int = 0
        static global incomplete_var: Struct(msg_holder.18) = &string.0
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
            p1.2[0] = 1
            p1.2[4] = 2
            p2.3[0] = 3D
            tmp.0 = truncate 4
            p2.3[8] = tmp.0
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
            p1.6[0] = 3
            tmp.7 = &p1.6
            p1.6[8] = tmp.7
            tmp.8 = &p1.6
            tmp.9 = tmp.8
            p2.7[0] = tmp.9
            p2.7[8] = 1D
            p2.7[16] = 2D
            p2.7[24] = 3D
            p2.7[32] = 4D
            tmp.10 = p2.7[0]
            tmp.11 = tmp.10
            tmp.12 = *tmp.11
            tmp.13 = tmp.12 != 3
            if !tmp.13 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function test_same_name_var_member_and_tag() { 
            x.9[0] = 10
            tmp.14 = x.9[0]
            tmp.15 = tmp.14 != 10
            if !tmp.15 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function test_same_name_fun_member_and_tag() { 
            tmp.16 = f()
            my_struct.11[0] = tmp.16
            tmp.17 = my_struct.11[0]
            tmp.18 = my_struct.11[0]
            tmp.19 = tmp.18 != 10
            if !tmp.19 jump end_if_8
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
            tmp.20 = test_shared_member_names()
            tmp.21 = ! tmp.20
            if !tmp.21 jump end_if_10
            return 1
        
          end_if_10:
            tmp.22 = test_shared_nested_member_names()
            tmp.23 = ! tmp.22
            if !tmp.23 jump end_if_12
            return 2
        
          end_if_12:
            tmp.24 = test_same_name_var_member_and_tag()
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_14
            return 3
        
          end_if_14:
            tmp.26 = test_same_name_fun_member_and_tag()
            tmp.27 = ! tmp.26
            if !tmp.27 jump end_if_16
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
            tmp.0 = outer.5[0]
            inner.7[0] = 3
            tmp.1 = inner.7[0]
            tmp.2 = outer.5[0]
            tmp.3 = tmp.2 != 2
            if !tmp.3 jump end_if_0
            return 0
        
          end_if_0:
            tmp.4 = inner.7[0]
            tmp.5 = tmp.4 != 3
            if !tmp.5 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_member_declaration() { 
            my_struct.9[0] = 123
            tmp.6 = sign_extend 0
            my_struct.9[8] = tmp.6
            tmp.7 = &my_struct.9
            my_struct.9[8] = tmp.7
            tmp.8 = my_struct.9[8]
            tmp.9 = my_struct.9[8]
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 != 123
            if !tmp.11 jump end_if_4
            return 0
        
          end_if_4:
            return 1
            return 0
        }
        global function test_function_declaration() { 
            outer_struct.10[0] = 1
            tmp.12 = &outer_struct.10
            tmp.13 = copy_struct(tmp.12)
            copy.13 = tmp.13
            tmp.14 = *copy.13
            tmp.16 = outer_struct.10[0]
            tmp.15 = tmp.14 != tmp.16
            if !tmp.15 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function copy_struct(arg.14) { 
            tmp.17 = sign_extend 4
            tmp.18 = malloc(tmp.17)
            tmp.19 = tmp.18
            ptr.15 = tmp.19
            tmp.20 = *arg.14
            *ptr.15 = tmp.20
            tmp.21 = *ptr.15
            return ptr.15
            return 0
        }
        global function test_for_loop() { 
            loop_struct.16[0] = 10
        
          start_loop_0:
            tmp.22 = loop_struct.16[0]
            tmp.23 = tmp.22 > 0
            if !tmp.23 jump break_loop_0
            tmp.24 = loop_body_struct.18[0]
            tmp.26 = int_to_double 1
            tmp.25 = tmp.24 + tmp.26
            loop_body_struct.18[0] = tmp.25
            tmp.27 = loop_body_struct.18[0]
            tmp.28 = loop_struct.16[0]
            tmp.29 = tmp.28 == 1
            if !tmp.29 jump end_if_8
            tmp.30 = loop_body_struct.18[0]
            tmp.31 = tmp.30 != 10D
            if !tmp.31 jump end_if_10
            return 0
        
          end_if_10:
        
          end_if_8:
        
          continue_loop_0:
            tmp.32 = loop_struct.16[0]
            tmp.33 = tmp.32 - 1
            loop_struct.16[0] = tmp.33
            tmp.34 = loop_struct.16[0]
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
        global function test_cast() { 
            tmp.35 = sign_extend 10
            tmp.36 = malloc(tmp.35)
            ptr.19 = tmp.36
            if !ptr.19 jump end_if_12
            tmp.37 = ptr.19
            tmp.38 = sign_extend 2
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=1)
            tmp.40 = truncate 10
            *tmp.39 = tmp.40
            tmp.41 = *tmp.39
            tmp.42 = ptr.19
            tmp.43 = sign_extend 2
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=1)
            tmp.45 = *tmp.44
            byte.21 = tmp.45
            tmp.46 = sign_extend byte.21
            tmp.47 = tmp.46 != 10
            if !tmp.47 jump end_if_14
            return 0
        
          end_if_14:
        
          end_if_12:
            tmp.48 = sign_extend 4
            tmp.49 = malloc(tmp.48)
            second_ptr.22 = tmp.49
            tmp.50 = second_ptr.22
            *tmp.50 = 10
            tmp.51 = *tmp.50
            tmp.52 = second_ptr.22
            tmp.53 = sign_extend 0
            tmp.54 = add_ptr(tmp.52, index=tmp.53, scale=1)
            tmp.55 = *tmp.54
            lowest_byte.23 = tmp.55
            tmp.56 = sign_extend lowest_byte.23
            tmp.57 = tmp.56 != 10
            if !tmp.57 jump end_if_16
            return 0
        
          end_if_16:
            return 1
            return 0
        }
        global function test_sizeof() { 
            tmp.59 = sign_extend 8
            tmp.58 = 8UL != tmp.59
            if !tmp.58 jump end_if_18
            return 0
        
          end_if_18:
            tmp.61 = sign_extend 15
            tmp.60 = 15UL != tmp.61
            if !tmp.60 jump end_if_20
            return 0
        
          end_if_20:
            tmp.63 = sign_extend 8
            tmp.62 = 8UL != tmp.63
            if !tmp.62 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function test_derived_types() { 
            outer_struct.27[0] = 1
            tmp.64 = sign_extend 3
            tmp.65 = calloc(tmp.64, 8UL)
            tmp.66 = tmp.65
            outer_arr.28 = tmp.66
            inner_struct.30[0] = 2
            tmp.67 = sign_extend 3
            tmp.68 = calloc(tmp.67, 8UL)
            tmp.69 = tmp.68
            inner_arr.31 = tmp.69
            tmp.70 = sign_extend 0
            tmp.71 = add_ptr(outer_arr.28, index=tmp.70, scale=24)
            tmp.72 = sign_extend 0
            tmp.73 = add_ptr(tmp.71, index=tmp.72, scale=8)
            tmp.74 = &outer_struct.27
            *tmp.73 = tmp.74
            tmp.75 = *tmp.73
            tmp.76 = sign_extend 0
            tmp.77 = add_ptr(outer_arr.28, index=tmp.76, scale=24)
            tmp.78 = sign_extend 1
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=8)
            tmp.80 = &outer_struct.27
            *tmp.79 = tmp.80
            tmp.81 = *tmp.79
            tmp.82 = sign_extend 0
            tmp.83 = add_ptr(inner_arr.31, index=tmp.82, scale=24)
            tmp.84 = sign_extend 0
            tmp.85 = add_ptr(tmp.83, index=tmp.84, scale=8)
            tmp.86 = &inner_struct.30
            *tmp.85 = tmp.86
            tmp.87 = *tmp.85
            tmp.88 = sign_extend 0
            tmp.89 = add_ptr(inner_arr.31, index=tmp.88, scale=24)
            tmp.90 = sign_extend 2
            tmp.91 = add_ptr(tmp.89, index=tmp.90, scale=8)
            tmp.92 = &inner_struct.30
            *tmp.91 = tmp.92
            tmp.93 = *tmp.91
            tmp.94 = sign_extend 0
            tmp.95 = add_ptr(outer_arr.28, index=tmp.94, scale=24)
            tmp.96 = sign_extend 0
            tmp.97 = add_ptr(tmp.95, index=tmp.96, scale=8)
            tmp.98 = *tmp.97
            tmp.99 = *tmp.98
            tmp.100 = tmp.99 != 1
            if !tmp.100 jump end_if_24
            return 0
        
          end_if_24:
            tmp.101 = sign_extend 0
            tmp.102 = add_ptr(inner_arr.31, index=tmp.101, scale=24)
            tmp.103 = sign_extend 0
            tmp.104 = add_ptr(tmp.102, index=tmp.103, scale=8)
            tmp.105 = *tmp.104
            tmp.106 = *tmp.105
            tmp.107 = tmp.106 != 2
            if !tmp.107 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function test_contentless_tag_noop() { 
            var.34[0] = 10
            tmp.108 = var.34[0]
            var.34[4] = 11
            tmp.109 = var.34[4]
            tmp.110 = var.34[0]
            tmp.111 = tmp.110 != 10
            if tmp.111 jump or_true_28
            tmp.114 = var.34[4]
            tmp.115 = tmp.114 != 11
            if tmp.115 jump or_true_28
            tmp.113 = 0
            jump or_end_29
        
          or_true_28:
            tmp.113 = 1
        
          or_end_29:
            if !tmp.113 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function main() { 
            tmp.116 = test_var_declaration()
            tmp.117 = ! tmp.116
            if !tmp.117 jump end_if_32
            return 1
        
          end_if_32:
            tmp.118 = test_member_declaration()
            tmp.119 = ! tmp.118
            if !tmp.119 jump end_if_34
            return 2
        
          end_if_34:
            tmp.120 = test_function_declaration()
            tmp.121 = ! tmp.120
            if !tmp.121 jump end_if_36
            return 3
        
          end_if_36:
            tmp.122 = test_for_loop()
            tmp.123 = ! tmp.122
            if !tmp.123 jump end_if_38
            return 4
        
          end_if_38:
            tmp.124 = test_cast()
            tmp.125 = ! tmp.124
            if !tmp.125 jump end_if_40
            return 5
        
          end_if_40:
            tmp.126 = test_sizeof()
            tmp.127 = ! tmp.126
            if !tmp.127 jump end_if_42
            return 6
        
          end_if_42:
            tmp.128 = test_derived_types()
            tmp.129 = ! tmp.128
            if !tmp.129 jump end_if_44
            return 7
        
          end_if_44:
            tmp.130 = test_contentless_tag_noop()
            tmp.131 = ! tmp.130
            if !tmp.131 jump end_if_46
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
            tmp.5 = my_struct.3
            tmp.4 = tmp.3 != tmp.5
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = my_struct.3
            tmp.7 = tmp.6 <= my_struct.3
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = add_ptr(my_struct.3, index=8L, scale=1)
            tmp.10 = tmp.9 <= my_struct.3
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = add_ptr(my_struct.3, index=4L, scale=1)
            tmp.13 = add_ptr(my_struct.3, index=8L, scale=1)
            tmp.12 = tmp.11 > tmp.13
            if !tmp.12 jump end_if_6
            return 4
        
          end_if_6:
            tmp.14 = add_ptr(my_struct.3, index=4L, scale=1)
            tmp.15 = tmp.14 > my_struct.3
            tmp.16 = ! tmp.15
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
            tmp.4 = add_ptr(tmp.4, index=4L, scale=1)
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
            tmp.24 = s_ptr.16
            c_addr.18 = tmp.24
            tmp.25 = add_ptr(s_ptr.16, index=8L, scale=1)
            tmp.26 = tmp.25
            d_addr.19 = tmp.26
            tmp.28 = sign_extend 1
            tmp.27 = add_ptr(s_ptr.16, index=tmp.28, scale=16)
            tmp.29 = tmp.27
            end_ptr.20 = tmp.29
            tmp.31 = sign_extend 8
            tmp.30 = start_addr.17 % tmp.31
            tmp.33 = sign_extend 0
            tmp.32 = tmp.30 != tmp.33
            if !tmp.32 jump end_if_8
            return 0
        
          end_if_8:
            tmp.34 = start_addr.17 != c_addr.18
            if !tmp.34 jump end_if_10
            return 0
        
          end_if_10:
            tmp.35 = d_addr.19 - c_addr.18
            tmp.37 = sign_extend 8
            tmp.36 = tmp.35 != tmp.37
            if !tmp.36 jump end_if_12
            return 0
        
          end_if_12:
            tmp.38 = end_ptr.20 - start_addr.17
            tmp.40 = sign_extend 16
            tmp.39 = tmp.38 != tmp.40
            if !tmp.39 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_three_bytes() { 
            tmp.41 = &s.21
            tmp.42 = tmp.41
            start_addr.22 = tmp.42
            tmp.43 = &s.21
            tmp.44 = tmp.43
            arr_addr.23 = tmp.44
            tmp.45 = &s.21
            tmp.46 = sign_extend 0
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=1)
            tmp.48 = tmp.47
            arr0_addr.24 = tmp.48
            tmp.49 = &s.21
            tmp.50 = sign_extend 1
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=1)
            tmp.52 = tmp.51
            arr1_addr.25 = tmp.52
            tmp.53 = &s.21
            tmp.55 = sign_extend 1
            tmp.54 = add_ptr(tmp.53, index=tmp.55, scale=1)
            tmp.56 = tmp.54
            arr1_addr_alt.26 = tmp.56
            tmp.57 = &s.21
            tmp.58 = sign_extend 2
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=1)
            tmp.60 = tmp.59
            arr2_addr.27 = tmp.60
            tmp.61 = &s.21
            tmp.63 = sign_extend 1
            tmp.62 = add_ptr(tmp.61, index=tmp.63, scale=3)
            tmp.64 = tmp.62
            arr_end.28 = tmp.64
            tmp.65 = &s.21
            tmp.67 = sign_extend 1
            tmp.66 = add_ptr(tmp.65, index=tmp.67, scale=3)
            tmp.68 = tmp.66
            struct_end.29 = tmp.68
            tmp.69 = start_addr.22 != arr_addr.23
            if !tmp.69 jump end_if_16
            return 0
        
          end_if_16:
            tmp.70 = start_addr.22 != arr0_addr.24
            if !tmp.70 jump end_if_18
            return 0
        
          end_if_18:
            tmp.71 = arr1_addr.25 - start_addr.22
            tmp.73 = sign_extend 1
            tmp.72 = tmp.71 != tmp.73
            if !tmp.72 jump end_if_20
            return 0
        
          end_if_20:
            tmp.74 = arr1_addr.25 != arr1_addr_alt.26
            if !tmp.74 jump end_if_22
            return 0
        
          end_if_22:
            tmp.75 = arr2_addr.27 - start_addr.22
            tmp.77 = sign_extend 2
            tmp.76 = tmp.75 != tmp.77
            if !tmp.76 jump end_if_24
            return 0
        
          end_if_24:
            tmp.78 = arr_end.28 - start_addr.22
            tmp.80 = sign_extend 3
            tmp.79 = tmp.78 != tmp.80
            if !tmp.79 jump end_if_26
            return 0
        
          end_if_26:
            tmp.81 = struct_end.29 - start_addr.22
            tmp.83 = sign_extend 3
            tmp.82 = tmp.81 != tmp.83
            if !tmp.82 jump end_if_28
            return 0
        
          end_if_28:
            return 1
            return 0
        }
        global function test_sixteen_bytes() { 
            tmp.84 = &s.30
            s_ptr.31 = tmp.84
            tmp.85 = s_ptr.31
            start_addr.32 = tmp.85
            tmp.86 = s_ptr.31
            eight_addr.33 = tmp.86
            tmp.87 = s_ptr.31
            eight_i_addr.34 = tmp.87
            tmp.88 = add_ptr(s_ptr.31, index=4L, scale=1)
            tmp.89 = tmp.88
            eight_c_addr.35 = tmp.89
            tmp.90 = add_ptr(s_ptr.31, index=8L, scale=1)
            tmp.91 = tmp.90
            two.36 = tmp.91
            tmp.92 = add_ptr(s_ptr.31, index=8L, scale=1)
            tmp.93 = tmp.92
            two_arr.37 = tmp.93
            tmp.94 = add_ptr(s_ptr.31, index=8L, scale=1)
            tmp.95 = sign_extend 0
            tmp.96 = add_ptr(tmp.94, index=tmp.95, scale=1)
            tmp.97 = tmp.96
            two_arr0.38 = tmp.97
            tmp.98 = add_ptr(s_ptr.31, index=8L, scale=1)
            tmp.99 = sign_extend 1
            tmp.100 = add_ptr(tmp.98, index=tmp.99, scale=1)
            tmp.101 = tmp.100
            two_arr1.39 = tmp.101
            tmp.102 = add_ptr(s_ptr.31, index=8L, scale=1)
            tmp.104 = sign_extend 1
            tmp.103 = add_ptr(tmp.102, index=tmp.104, scale=2)
            tmp.105 = tmp.103
            two_arr_end.40 = tmp.105
            tmp.106 = add_ptr(s_ptr.31, index=8L, scale=1)
            tmp.108 = sign_extend 1
            tmp.107 = add_ptr(tmp.106, index=tmp.108, scale=2)
            tmp.109 = tmp.107
            two_end.41 = tmp.109
            tmp.110 = add_ptr(s_ptr.31, index=10L, scale=1)
            tmp.111 = tmp.110
            three.42 = tmp.111
            tmp.112 = add_ptr(s_ptr.31, index=10L, scale=1)
            tmp.114 = sign_extend 1
            tmp.113 = add_ptr(tmp.112, index=tmp.114, scale=3)
            tmp.115 = tmp.113
            three_end.43 = tmp.115
            tmp.117 = sign_extend 1
            tmp.116 = add_ptr(s_ptr.31, index=tmp.117, scale=16)
            tmp.118 = tmp.116
            struct_end.44 = tmp.118
            tmp.120 = sign_extend 4
            tmp.119 = start_addr.32 % tmp.120
            tmp.122 = sign_extend 0
            tmp.121 = tmp.119 != tmp.122
            if !tmp.121 jump end_if_30
            return 0
        
          end_if_30:
            tmp.123 = start_addr.32 != eight_addr.33
            if !tmp.123 jump end_if_32
            return 0
        
          end_if_32:
            tmp.124 = start_addr.32 != eight_i_addr.34
            if !tmp.124 jump end_if_34
            return 0
        
          end_if_34:
            tmp.125 = eight_c_addr.35 - start_addr.32
            tmp.127 = sign_extend 4
            tmp.126 = tmp.125 != tmp.127
            if !tmp.126 jump end_if_36
            return 0
        
          end_if_36:
            tmp.128 = two.36 - start_addr.32
            tmp.130 = sign_extend 8
            tmp.129 = tmp.128 != tmp.130
            if !tmp.129 jump end_if_38
            return 0
        
          end_if_38:
            tmp.131 = two_arr.37 - start_addr.32
            tmp.133 = sign_extend 8
            tmp.132 = tmp.131 != tmp.133
            if !tmp.132 jump end_if_40
            return 0
        
          end_if_40:
            tmp.134 = two_arr0.38 - start_addr.32
            tmp.136 = sign_extend 8
            tmp.135 = tmp.134 != tmp.136
            if !tmp.135 jump end_if_42
            return 0
        
          end_if_42:
            tmp.137 = two_arr1.39 - start_addr.32
            tmp.139 = sign_extend 9
            tmp.138 = tmp.137 != tmp.139
            if !tmp.138 jump end_if_44
            return 0
        
          end_if_44:
            tmp.140 = two_arr_end.40 - start_addr.32
            tmp.142 = sign_extend 10
            tmp.141 = tmp.140 != tmp.142
            if !tmp.141 jump end_if_46
            return 0
        
          end_if_46:
            tmp.143 = two_arr_end.40 != two_end.41
            if !tmp.143 jump end_if_48
            return 0
        
          end_if_48:
            tmp.144 = three.42 - start_addr.32
            tmp.146 = sign_extend 10
            tmp.145 = tmp.144 != tmp.146
            if !tmp.145 jump end_if_50
            return 0
        
          end_if_50:
            tmp.147 = three_end.43 - start_addr.32
            tmp.149 = sign_extend 13
            tmp.148 = tmp.147 != tmp.149
            if !tmp.148 jump end_if_52
            return 0
        
          end_if_52:
            tmp.150 = struct_end.44 - start_addr.32
            tmp.152 = sign_extend 16
            tmp.151 = tmp.150 != tmp.152
            if !tmp.151 jump end_if_54
            return 0
        
          end_if_54:
            tmp.153 = &s.30
            tmp.154 = tmp.153
            eight_i_addr_alt.45 = tmp.154
            tmp.155 = &s.30
            tmp.155 = add_ptr(tmp.155, index=4L, scale=1)
            tmp.156 = tmp.155
            eight_c_addr_alt.46 = tmp.156
            tmp.157 = &s.30
            tmp.157 = add_ptr(tmp.157, index=8L, scale=1)
            tmp.158 = tmp.157
            two_arr_alt.47 = tmp.158
            tmp.159 = &s.30
            tmp.159 = add_ptr(tmp.159, index=8L, scale=1)
            tmp.160 = sign_extend 1
            tmp.161 = add_ptr(tmp.159, index=tmp.160, scale=1)
            tmp.162 = tmp.161
            two_arr1_alt.48 = tmp.162
            tmp.163 = &s.30
            tmp.163 = add_ptr(tmp.163, index=10L, scale=1)
            tmp.164 = tmp.163
            three_alt.49 = tmp.164
            tmp.165 = eight_i_addr_alt.45 != eight_i_addr.34
            if !tmp.165 jump end_if_56
            return 0
        
          end_if_56:
            tmp.166 = eight_c_addr_alt.46 != eight_c_addr.35
            if !tmp.166 jump end_if_58
            return 0
        
          end_if_58:
            tmp.167 = two_arr_alt.47 != two_arr.37
            if !tmp.167 jump end_if_60
            return 0
        
          end_if_60:
            tmp.168 = two_arr1_alt.48 != two_arr1.39
            if !tmp.168 jump end_if_62
            return 0
        
          end_if_62:
            tmp.169 = three_alt.49 != three.42
            if !tmp.169 jump end_if_64
            return 0
        
          end_if_64:
            return 1
            return 0
        }
        global function test_wonky_array() { 
            tmp.170 = &wonky_array.50
            tmp.171 = tmp.170
            array_start.51 = tmp.171
            tmp.172 = &wonky_array.50
            tmp.174 = sign_extend 3
            tmp.173 = add_ptr(tmp.172, index=tmp.174, scale=19)
            tmp.175 = tmp.173
            elem3.52 = tmp.175
            tmp.176 = &wonky_array.50
            tmp.177 = sign_extend 3
            tmp.178 = add_ptr(tmp.176, index=tmp.177, scale=19)
            tmp.179 = tmp.178
            elem3_arr.53 = tmp.179
            tmp.180 = &wonky_array.50
            tmp.181 = sign_extend 2
            tmp.182 = add_ptr(tmp.180, index=tmp.181, scale=19)
            tmp.183 = sign_extend 2
            tmp.184 = add_ptr(tmp.182, index=tmp.183, scale=1)
            tmp.185 = tmp.184
            elem2_arr2.54 = tmp.185
            tmp.186 = &wonky_array.50
            tmp.187 = sign_extend 2
            tmp.188 = add_ptr(tmp.186, index=tmp.187, scale=19)
            tmp.190 = sign_extend 19
            tmp.189 = add_ptr(tmp.188, index=tmp.190, scale=1)
            tmp.191 = tmp.189
            elem2_arr_end.55 = tmp.191
            tmp.192 = &wonky_array.50
            tmp.193 = sign_extend 4
            tmp.194 = add_ptr(tmp.192, index=tmp.193, scale=19)
            tmp.196 = sign_extend 19
            tmp.195 = add_ptr(tmp.194, index=tmp.196, scale=1)
            tmp.197 = tmp.195
            elem4_arr_end.56 = tmp.197
            tmp.198 = &wonky_array.50
            tmp.200 = sign_extend 5
            tmp.199 = add_ptr(tmp.198, index=tmp.200, scale=19)
            tmp.201 = tmp.199
            array_end.57 = tmp.201
            tmp.202 = elem3.52 - array_start.51
            tmp.204 = 19 * 3
            tmp.205 = sign_extend tmp.204
            tmp.203 = tmp.202 != tmp.205
            if !tmp.203 jump end_if_66
            return 0
        
          end_if_66:
            tmp.206 = elem3_arr.53 != elem3.52
            if !tmp.206 jump end_if_68
            return 0
        
          end_if_68:
            tmp.207 = elem2_arr2.54 - array_start.51
            tmp.209 = 19 * 2
            tmp.210 = tmp.209 + 2
            tmp.211 = sign_extend tmp.210
            tmp.208 = tmp.207 != tmp.211
            if !tmp.208 jump end_if_70
            return 0
        
          end_if_70:
            tmp.212 = elem2_arr_end.55 != elem3.52
            if !tmp.212 jump end_if_72
            return 0
        
          end_if_72:
            tmp.213 = elem4_arr_end.56 != array_end.57
            if !tmp.213 jump end_if_74
            return 0
        
          end_if_74:
            return 1
            return 0
        }
        global function test_contains_struct_array_array() { 
            tmp.214 = &arr.58
            tmp.215 = tmp.214
            array_start.59 = tmp.215
            tmp.216 = &arr.58
            tmp.217 = sign_extend 0
            tmp.218 = add_ptr(tmp.216, index=tmp.217, scale=28)
            tmp.219 = tmp.218
            first_scalar_elem.60 = tmp.219
            tmp.220 = &arr.58
            tmp.221 = sign_extend 0
            tmp.222 = add_ptr(tmp.220, index=tmp.221, scale=28)
            tmp.223 = add_ptr(tmp.222, index=4L, scale=1)
            tmp.224 = tmp.223
            outer0_inner0_i.61 = tmp.224
            tmp.225 = &arr.58
            tmp.226 = add_ptr(tmp.225, index=4L, scale=1)
            tmp.227 = add_ptr(tmp.226, index=4L, scale=1)
            tmp.228 = tmp.227
            outer0_inner0_c.62 = tmp.228
            tmp.229 = &arr.58
            tmp.230 = add_ptr(tmp.229, index=4L, scale=1)
            tmp.232 = sign_extend 3
            tmp.231 = add_ptr(tmp.230, index=tmp.232, scale=8)
            tmp.233 = tmp.231
            outer0_end.63 = tmp.233
            tmp.234 = &arr.58
            tmp.235 = sign_extend 1
            tmp.236 = add_ptr(tmp.234, index=tmp.235, scale=28)
            tmp.237 = tmp.236
            outer1.64 = tmp.237
            tmp.238 = &arr.58
            tmp.239 = sign_extend 1
            tmp.240 = add_ptr(tmp.238, index=tmp.239, scale=28)
            tmp.241 = add_ptr(tmp.240, index=4L, scale=1)
            tmp.242 = tmp.241
            outer1_arr.65 = tmp.242
            tmp.243 = &arr.58
            tmp.245 = sign_extend 1
            tmp.244 = add_ptr(tmp.243, index=tmp.245, scale=28)
            tmp.246 = add_ptr(tmp.244, index=4L, scale=1)
            tmp.248 = sign_extend 1
            tmp.247 = add_ptr(tmp.246, index=tmp.248, scale=8)
            tmp.249 = tmp.247
            outer1_inner1_i.66 = tmp.249
            tmp.250 = &arr.58
            tmp.252 = sign_extend 2
            tmp.251 = add_ptr(tmp.250, index=tmp.252, scale=28)
            tmp.253 = add_ptr(tmp.251, index=4L, scale=1)
            tmp.254 = add_ptr(tmp.253, index=4L, scale=1)
            tmp.255 = tmp.254
            outer2_inner0_c.67 = tmp.255
            tmp.257 = sign_extend 4
            tmp.256 = array_start.59 % tmp.257
            tmp.259 = sign_extend 0
            tmp.258 = tmp.256 != tmp.259
            if !tmp.258 jump end_if_76
            return 0
        
          end_if_76:
            tmp.260 = first_scalar_elem.60 != array_start.59
            if !tmp.260 jump end_if_78
            return 0
        
          end_if_78:
            tmp.261 = outer0_inner0_i.61 - array_start.59
            tmp.263 = sign_extend 4
            tmp.262 = tmp.261 != tmp.263
            if !tmp.262 jump end_if_80
            return 0
        
          end_if_80:
            tmp.264 = outer0_inner0_c.62 - array_start.59
            tmp.266 = sign_extend 8
            tmp.265 = tmp.264 != tmp.266
            if !tmp.265 jump end_if_82
            return 0
        
          end_if_82:
            tmp.267 = outer0_end.63 != outer1.64
            if !tmp.267 jump end_if_84
            return 0
        
          end_if_84:
            tmp.268 = outer1_arr.65 - array_start.59
            tmp.270 = sign_extend 32
            tmp.269 = tmp.268 != tmp.270
            if !tmp.269 jump end_if_86
            return 0
        
          end_if_86:
            tmp.271 = outer1_arr.65 - outer1.64
            tmp.273 = sign_extend 4
            tmp.272 = tmp.271 != tmp.273
            if !tmp.272 jump end_if_88
            return 0
        
          end_if_88:
            tmp.274 = outer1_inner1_i.66 - array_start.59
            tmp.276 = sign_extend 40
            tmp.275 = tmp.274 != tmp.276
            if !tmp.275 jump end_if_90
            return 0
        
          end_if_90:
            tmp.277 = outer2_inner0_c.67 - array_start.59
            tmp.279 = sign_extend 64
            tmp.278 = tmp.277 != tmp.279
            if !tmp.278 jump end_if_92
            return 0
        
          end_if_92:
            return 1
            return 0
        }
        global function main() { 
            tmp.280 = test_eightbytes()
            tmp.281 = ! tmp.280
            if !tmp.281 jump end_if_94
            return 1
        
          end_if_94:
            tmp.282 = test_internal_padding()
            tmp.283 = ! tmp.282
            if !tmp.283 jump end_if_96
            return 2
        
          end_if_96:
            tmp.284 = test_three_bytes()
            tmp.285 = ! tmp.284
            if !tmp.285 jump end_if_98
            return 3
        
          end_if_98:
            tmp.286 = test_sixteen_bytes()
            tmp.287 = ! tmp.286
            if !tmp.287 jump end_if_100
            return 4
        
          end_if_100:
            tmp.288 = test_wonky_array()
            tmp.289 = ! tmp.288
            if !tmp.289 jump end_if_102
            return 5
        
          end_if_102:
            tmp.290 = test_contains_struct_array_array()
            tmp.291 = ! tmp.290
            if !tmp.291 jump end_if_104
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
            x.1[0] = 1
            x.1[4] = 2
            tmp.0 = x.1[0]
            tmp.1 = tmp.0 != 1
            if tmp.1 jump or_true_0
            tmp.4 = x.1[4]
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
            tmp.7 = *x_ptr.2
            tmp.8 = tmp.7 != 1
            if tmp.8 jump or_true_4
            tmp.11 = add_ptr(x_ptr.2, index=4L, scale=1)
            tmp.12 = *tmp.11
            tmp.13 = tmp.12 != 2
            if tmp.13 jump or_true_4
            tmp.10 = 0
            jump or_end_5
        
          or_true_4:
            tmp.10 = 1
        
          or_end_5:
            if !tmp.10 jump end_if_6
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
            autom.2[0] = 1
            autom.2[4] = 2
            tmp.1 = autom.2[0]
            tmp.2 = tmp.1 + 1
            autom.2[0] = tmp.2
            tmp.3 = autom.2[0]
            tmp.4 = autom.2[4]
            tmp.5 = tmp.4 + 1
            autom.2[4] = tmp.5
            tmp.6 = autom.2[4]
            tmp.7 = stat.3[0]
            tmp.8 = tmp.7 + 1
            stat.3[0] = tmp.8
            tmp.9 = stat.3[0]
            tmp.10 = stat.3[4]
            tmp.11 = tmp.10 + 1
            stat.3[4] = tmp.11
            tmp.12 = stat.3[4]
            tmp.13 = i.1 == 9
            if !tmp.13 jump end_if_0
            tmp.14 = stat.3[0]
            tmp.15 = tmp.14 != 11
            if tmp.15 jump or_true_2
            tmp.18 = stat.3[4]
            tmp.19 = tmp.18 != 12
            if tmp.19 jump or_true_2
            tmp.17 = 0
            jump or_end_3
        
          or_true_2:
            tmp.17 = 1
        
          or_end_3:
            if !tmp.17 jump end_if_4
            return 1
        
          end_if_4:
            tmp.20 = autom.2[0]
            tmp.21 = tmp.20 != 2
            if tmp.21 jump or_true_6
            tmp.24 = autom.2[4]
            tmp.25 = tmp.24 != 3
            if tmp.25 jump or_true_6
            tmp.23 = 0
            jump or_end_7
        
          or_true_6:
            tmp.23 = 1
        
          or_end_7:
            if !tmp.23 jump end_if_8
            return 2
        
          end_if_8:
        
          end_if_0:
        
          continue_loop_0:
            tmp.26 = i.1 + 1
            i.1 = tmp.26
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
            x.6[0] = 'a'
            x.6[1] = 'b'
            x.6[2] = '\0'
            tmp.0 = - 1
            x.6[8] = tmp.0
            tmp.1 = sign_extend 2
            x.6[16] = tmp.1
            y.7[0] = 'x'
            y.7[1] = '\0'
            y.7[2] = '\0'
            y.7[8] = 1
            y.7[16] = 0L
            y.7 = x.6
            tmp.2 = &y.7
            tmp.3 = &string.0
            tmp.4 = strcmp(tmp.2, tmp.3)
            if tmp.4 jump or_true_0
            tmp.7 = y.7[8]
            tmp.9 = - 1
            tmp.8 = tmp.7 != tmp.9
            if tmp.8 jump or_true_0
            tmp.6 = 0
            jump or_end_1
        
          or_true_0:
            tmp.6 = 1
        
          or_end_1:
            if tmp.6 jump or_true_2
            tmp.12 = y.7[16]
            tmp.14 = sign_extend 2
            tmp.13 = tmp.12 != tmp.14
            if tmp.13 jump or_true_2
            tmp.11 = 0
            jump or_end_3
        
          or_true_2:
            tmp.11 = 1
        
          or_end_3:
            if !tmp.11 jump end_if_4
            return 0
        
          end_if_4:
            y.7[8] = 20
            tmp.15 = y.7[8]
            tmp.16 = y.7[8]
            tmp.17 = tmp.16 != 20
            if tmp.17 jump or_true_6
            tmp.20 = x.6[8]
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
            tmp.23 = &y.9
            tmp.24 = &string.0
            tmp.25 = strcmp(tmp.23, tmp.24)
            if tmp.25 jump or_true_10
            tmp.28 = y.9[8]
            tmp.29 = tmp.28 != 1
            if tmp.29 jump or_true_10
            tmp.27 = 0
            jump or_end_11
        
          or_true_10:
            tmp.27 = 1
        
          or_end_11:
            if tmp.27 jump or_true_12
            tmp.32 = y.9[16]
            tmp.34 = sign_extend 2
            tmp.33 = tmp.32 != tmp.34
            if tmp.33 jump or_true_12
            tmp.31 = 0
            jump or_end_13
        
          or_true_12:
            tmp.31 = 1
        
          or_end_13:
            if !tmp.31 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_wonky_size() { 
            x.11[0] = 'a'
            x.11[1] = 'b'
            x.11[2] = 'c'
            x.11[3] = 'd'
            x.11[4] = 'e'
            x.11[5] = 'f'
            x.11[6] = '\0'
            y.12 = x.11
            tmp.35 = &y.12
            tmp.36 = &string.1
            tmp.37 = strcmp(tmp.35, tmp.36)
            if !tmp.37 jump end_if_16
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
            y.14[0] = '!'
            y.14[1] = '\0'
            y.14[2] = '\0'
            tmp.38 = - 10
            y.14[8] = tmp.38
            y.14[16] = 0L
            tmp.39 = true_flag()
            if !tmp.39 jump else_19
            tmp.40 = x.13
            jump end_if_18
        
          else_19:
            tmp.40 = y.14
        
          end_if_18:
            z.15 = tmp.40
            tmp.41 = &z.15
            tmp.42 = &string.2
            tmp.43 = strcmp(tmp.41, tmp.42)
            if tmp.43 jump or_true_20
            tmp.46 = z.15[8]
            tmp.47 = tmp.46 != 1234
            if tmp.47 jump or_true_20
            tmp.45 = 0
            jump or_end_21
        
          or_true_20:
            tmp.45 = 1
        
          or_end_21:
            if tmp.45 jump or_true_22
            tmp.50 = z.15[16]
            tmp.52 = sign_extend 5678
            tmp.51 = tmp.50 != tmp.52
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
        global function main() { 
            tmp.53 = test_auto()
            tmp.54 = ! tmp.53
            if !tmp.54 jump end_if_26
            return 1
        
          end_if_26:
            tmp.55 = test_static()
            tmp.56 = ! tmp.55
            if !tmp.56 jump end_if_28
            return 2
        
          end_if_28:
            tmp.57 = test_wonky_size()
            tmp.58 = ! tmp.57
            if !tmp.58 jump end_if_30
            return 3
        
          end_if_30:
            tmp.59 = test_conditional()
            tmp.60 = ! tmp.59
            if !tmp.60 jump end_if_32
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
            y.6[0] = '!'
            y.6[1] = '?'
            y.6[2] = '\0'
            tmp.0 = - 20
            y.6[8] = tmp.0
            tmp.1 = - 30
            tmp.2 = sign_extend tmp.1
            y.6[16] = tmp.2
            tmp.3 = malloc(24UL)
            tmp.4 = tmp.3
            x.7 = tmp.4
            *x.7 = y.6
            tmp.5 = *x.7
            tmp.6 = &string.0
            tmp.7 = strcmp(x.7, tmp.6)
            if tmp.7 jump or_true_0
            tmp.10 = add_ptr(x.7, index=8L, scale=1)
            tmp.11 = *tmp.10
            tmp.13 = - 20
            tmp.12 = tmp.11 != tmp.13
            if tmp.12 jump or_true_0
            tmp.9 = 0
            jump or_end_1
        
          or_true_0:
            tmp.9 = 1
        
          or_end_1:
            if tmp.9 jump or_true_2
            tmp.16 = add_ptr(x.7, index=8L, scale=1)
            tmp.17 = add_ptr(tmp.16, index=8L, scale=1)
            tmp.18 = *tmp.17
            tmp.20 = - 30
            tmp.21 = sign_extend tmp.20
            tmp.19 = tmp.18 != tmp.21
            if tmp.19 jump or_true_2
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
            tmp.22 = &my_struct.8
            y.9 = tmp.22
            x.10[0] = '\0'
            x.10[1] = '\0'
            x.10[2] = '\0'
            x.10[8] = 0
            tmp.23 = sign_extend 0
            x.10[16] = tmp.23
            tmp.24 = *y.9
            x.10 = tmp.24
            tmp.25 = &x.10
            tmp.26 = &string.1
            tmp.27 = strcmp(tmp.25, tmp.26)
            if tmp.27 jump or_true_6
            tmp.30 = x.10[8]
            tmp.31 = tmp.30 != 77
            if tmp.31 jump or_true_6
            tmp.29 = 0
            jump or_end_7
        
          or_true_6:
            tmp.29 = 1
        
          or_end_7:
            if tmp.29 jump or_true_8
            tmp.34 = x.10[16]
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
            my_struct.11[0] = '+'
            my_struct.11[1] = '-'
            my_struct.11[2] = '\0'
            my_struct.11[8] = 1000
            tmp.37 = sign_extend 1001
            my_struct.11[16] = tmp.37
            tmp.38 = &my_struct.11
            y.12 = tmp.38
            tmp.39 = malloc(24UL)
            tmp.40 = tmp.39
            x.13 = tmp.40
            tmp.41 = *y.12
            *x.13 = tmp.41
            tmp.42 = *x.13
            tmp.43 = &string.2
            tmp.44 = strcmp(x.13, tmp.43)
            if tmp.44 jump or_true_12
            tmp.47 = add_ptr(x.13, index=8L, scale=1)
            tmp.48 = *tmp.47
            tmp.49 = tmp.48 != 1000
            if tmp.49 jump or_true_12
            tmp.46 = 0
            jump or_end_13
        
          or_true_12:
            tmp.46 = 1
        
          or_end_13:
            if tmp.46 jump or_true_14
            tmp.52 = add_ptr(x.13, index=8L, scale=1)
            tmp.53 = add_ptr(tmp.52, index=8L, scale=1)
            tmp.54 = *tmp.53
            tmp.56 = sign_extend 1001
            tmp.55 = tmp.54 != tmp.56
            if tmp.55 jump or_true_14
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
            y.14[0] = '\n'
            y.14[1] = '\t'
            y.14[2] = '\0'
            y.14[8] = 10000
            tmp.57 = sign_extend 20000
            y.14[16] = tmp.57
            tmp.58 = &arr.15
            tmp.59 = sign_extend 1
            tmp.60 = add_ptr(tmp.58, index=tmp.59, scale=24)
            *tmp.60 = y.14
            tmp.61 = *tmp.60
            tmp.62 = &arr.15
            tmp.63 = sign_extend 1
            tmp.64 = add_ptr(tmp.62, index=tmp.63, scale=24)
            tmp.65 = &string.3
            tmp.66 = strcmp(tmp.64, tmp.65)
            if tmp.66 jump or_true_18
            tmp.69 = &arr.15
            tmp.70 = sign_extend 1
            tmp.71 = add_ptr(tmp.69, index=tmp.70, scale=24)
            tmp.72 = add_ptr(tmp.71, index=8L, scale=1)
            tmp.73 = *tmp.72
            tmp.74 = tmp.73 != 10000
            if tmp.74 jump or_true_18
            tmp.68 = 0
            jump or_end_19
        
          or_true_18:
            tmp.68 = 1
        
          or_end_19:
            if tmp.68 jump or_true_20
            tmp.77 = &arr.15
            tmp.78 = sign_extend 1
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=24)
            tmp.80 = add_ptr(tmp.79, index=8L, scale=1)
            tmp.81 = add_ptr(tmp.80, index=8L, scale=1)
            tmp.82 = *tmp.81
            tmp.84 = sign_extend 20000
            tmp.83 = tmp.82 != tmp.84
            if tmp.83 jump or_true_20
            tmp.76 = 0
            jump or_end_21
        
          or_true_20:
            tmp.76 = 1
        
          or_end_21:
            if !tmp.76 jump end_if_22
            return 0
        
          end_if_22:
            tmp.85 = &arr.15
            tmp.86 = sign_extend 0
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=24)
            tmp.88 = add_ptr(tmp.87, index=8L, scale=1)
            tmp.89 = *tmp.88
            if tmp.89 jump or_true_24
            tmp.92 = &arr.15
            tmp.93 = sign_extend 0
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=24)
            tmp.95 = add_ptr(tmp.94, index=8L, scale=1)
            tmp.96 = add_ptr(tmp.95, index=8L, scale=1)
            tmp.97 = *tmp.96
            if tmp.97 jump or_true_24
            tmp.91 = 0
            jump or_end_25
        
          or_true_24:
            tmp.91 = 1
        
          or_end_25:
            if tmp.91 jump or_true_26
            tmp.100 = &arr.15
            tmp.101 = sign_extend 2
            tmp.102 = add_ptr(tmp.100, index=tmp.101, scale=24)
            tmp.103 = sign_extend 0
            tmp.104 = add_ptr(tmp.102, index=tmp.103, scale=1)
            tmp.105 = *tmp.104
            if tmp.105 jump or_true_26
            tmp.99 = 0
            jump or_end_27
        
          or_true_26:
            tmp.99 = 1
        
          or_end_27:
            if tmp.99 jump or_true_28
            tmp.108 = &arr.15
            tmp.109 = sign_extend 2
            tmp.110 = add_ptr(tmp.108, index=tmp.109, scale=24)
            tmp.111 = sign_extend 1
            tmp.112 = add_ptr(tmp.110, index=tmp.111, scale=1)
            tmp.113 = *tmp.112
            if tmp.113 jump or_true_28
            tmp.107 = 0
            jump or_end_29
        
          or_true_28:
            tmp.107 = 1
        
          or_end_29:
            if !tmp.107 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_copy_from_array_elem() { 
            arr.16[0] = 'a'
            arr.16[1] = 'b'
            arr.16[2] = '\0'
            tmp.114 = - 3000
            arr.16[8] = tmp.114
            tmp.115 = - 4000
            tmp.116 = sign_extend tmp.115
            arr.16[16] = tmp.116
            arr.16[24] = 'c'
            arr.16[25] = 'd'
            arr.16[26] = '\0'
            tmp.117 = - 5000
            arr.16[32] = tmp.117
            tmp.118 = - 6000
            tmp.119 = sign_extend tmp.118
            arr.16[40] = tmp.119
            arr.16[48] = 'e'
            arr.16[49] = 'f'
            arr.16[50] = '\0'
            tmp.120 = - 7000
            arr.16[56] = tmp.120
            tmp.121 = - 8000
            tmp.122 = sign_extend tmp.121
            arr.16[64] = tmp.122
            x.17[0] = '\0'
            x.17[1] = '\0'
            x.17[2] = '\0'
            x.17[8] = 0
            tmp.123 = sign_extend 0
            x.17[16] = tmp.123
            tmp.124 = &arr.16
            tmp.125 = sign_extend 1
            tmp.126 = add_ptr(tmp.124, index=tmp.125, scale=24)
            tmp.127 = *tmp.126
            x.17 = tmp.127
            tmp.128 = &x.17
            tmp.129 = &string.4
            tmp.130 = strcmp(tmp.128, tmp.129)
            if tmp.130 jump or_true_32
            tmp.133 = x.17[8]
            tmp.135 = - 5000
            tmp.134 = tmp.133 != tmp.135
            if tmp.134 jump or_true_32
            tmp.132 = 0
            jump or_end_33
        
          or_true_32:
            tmp.132 = 1
        
          or_end_33:
            if tmp.132 jump or_true_34
            tmp.138 = x.17[16]
            tmp.140 = - 6000
            tmp.141 = sign_extend tmp.140
            tmp.139 = tmp.138 != tmp.141
            if tmp.139 jump or_true_34
            tmp.137 = 0
            jump or_end_35
        
          or_true_34:
            tmp.137 = 1
        
          or_end_35:
            if !tmp.137 jump end_if_36
            return 0
        
          end_if_36:
            return 1
            return 0
        }
        global function test_copy_to_and_from_array_elem() { 
            arr.18[0] = 'a'
            arr.18[1] = 'b'
            arr.18[2] = '\0'
            tmp.142 = - 3000
            arr.18[8] = tmp.142
            tmp.143 = - 4000
            tmp.144 = sign_extend tmp.143
            arr.18[16] = tmp.144
            arr.18[24] = 'c'
            arr.18[25] = 'd'
            arr.18[26] = '\0'
            tmp.145 = - 5000
            arr.18[32] = tmp.145
            tmp.146 = - 6000
            tmp.147 = sign_extend tmp.146
            arr.18[40] = tmp.147
            arr.18[48] = 'e'
            arr.18[49] = 'f'
            arr.18[50] = '\0'
            tmp.148 = - 7000
            arr.18[56] = tmp.148
            tmp.149 = - 8000
            tmp.150 = sign_extend tmp.149
            arr.18[64] = tmp.150
            tmp.151 = &arr.18
            tmp.152 = sign_extend 0
            tmp.153 = add_ptr(tmp.151, index=tmp.152, scale=24)
            tmp.154 = &arr.18
            tmp.155 = sign_extend 2
            tmp.156 = add_ptr(tmp.154, index=tmp.155, scale=24)
            tmp.157 = *tmp.156
            *tmp.153 = tmp.157
            tmp.158 = *tmp.153
            tmp.159 = &arr.18
            tmp.160 = sign_extend 0
            tmp.161 = add_ptr(tmp.159, index=tmp.160, scale=24)
            tmp.162 = &string.5
            tmp.163 = strcmp(tmp.161, tmp.162)
            if tmp.163 jump or_true_38
            tmp.166 = &arr.18
            tmp.167 = sign_extend 0
            tmp.168 = add_ptr(tmp.166, index=tmp.167, scale=24)
            tmp.169 = add_ptr(tmp.168, index=8L, scale=1)
            tmp.170 = *tmp.169
            tmp.172 = - 7000
            tmp.171 = tmp.170 != tmp.172
            if tmp.171 jump or_true_38
            tmp.165 = 0
            jump or_end_39
        
          or_true_38:
            tmp.165 = 1
        
          or_end_39:
            if tmp.165 jump or_true_40
            tmp.175 = &arr.18
            tmp.176 = sign_extend 0
            tmp.177 = add_ptr(tmp.175, index=tmp.176, scale=24)
            tmp.178 = add_ptr(tmp.177, index=8L, scale=1)
            tmp.179 = add_ptr(tmp.178, index=8L, scale=1)
            tmp.180 = *tmp.179
            tmp.182 = - 8000
            tmp.183 = sign_extend tmp.182
            tmp.181 = tmp.180 != tmp.183
            if tmp.181 jump or_true_40
            tmp.174 = 0
            jump or_end_41
        
          or_true_40:
            tmp.174 = 1
        
          or_end_41:
            if !tmp.174 jump end_if_42
            return 0
        
          end_if_42:
            tmp.184 = &arr.18
            tmp.185 = sign_extend 1
            tmp.186 = add_ptr(tmp.184, index=tmp.185, scale=24)
            tmp.187 = &string.4
            tmp.188 = strcmp(tmp.186, tmp.187)
            if tmp.188 jump or_true_44
            tmp.191 = &arr.18
            tmp.192 = sign_extend 1
            tmp.193 = add_ptr(tmp.191, index=tmp.192, scale=24)
            tmp.194 = add_ptr(tmp.193, index=8L, scale=1)
            tmp.195 = *tmp.194
            tmp.197 = - 5000
            tmp.196 = tmp.195 != tmp.197
            if tmp.196 jump or_true_44
            tmp.190 = 0
            jump or_end_45
        
          or_true_44:
            tmp.190 = 1
        
          or_end_45:
            if tmp.190 jump or_true_46
            tmp.200 = &arr.18
            tmp.201 = sign_extend 1
            tmp.202 = add_ptr(tmp.200, index=tmp.201, scale=24)
            tmp.203 = add_ptr(tmp.202, index=8L, scale=1)
            tmp.204 = add_ptr(tmp.203, index=8L, scale=1)
            tmp.205 = *tmp.204
            tmp.207 = - 6000
            tmp.208 = sign_extend tmp.207
            tmp.206 = tmp.205 != tmp.208
            if tmp.206 jump or_true_46
            tmp.199 = 0
            jump or_end_47
        
          or_true_46:
            tmp.199 = 1
        
          or_end_47:
            if !tmp.199 jump end_if_48
            return 0
        
          end_if_48:
            tmp.209 = &arr.18
            tmp.210 = sign_extend 2
            tmp.211 = add_ptr(tmp.209, index=tmp.210, scale=24)
            tmp.212 = &string.5
            tmp.213 = strcmp(tmp.211, tmp.212)
            if tmp.213 jump or_true_50
            tmp.216 = &arr.18
            tmp.217 = sign_extend 2
            tmp.218 = add_ptr(tmp.216, index=tmp.217, scale=24)
            tmp.219 = add_ptr(tmp.218, index=8L, scale=1)
            tmp.220 = *tmp.219
            tmp.222 = - 7000
            tmp.221 = tmp.220 != tmp.222
            if tmp.221 jump or_true_50
            tmp.215 = 0
            jump or_end_51
        
          or_true_50:
            tmp.215 = 1
        
          or_end_51:
            if tmp.215 jump or_true_52
            tmp.225 = &arr.18
            tmp.226 = sign_extend 2
            tmp.227 = add_ptr(tmp.225, index=tmp.226, scale=24)
            tmp.228 = add_ptr(tmp.227, index=8L, scale=1)
            tmp.229 = add_ptr(tmp.228, index=8L, scale=1)
            tmp.230 = *tmp.229
            tmp.232 = - 8000
            tmp.233 = sign_extend tmp.232
            tmp.231 = tmp.230 != tmp.233
            if tmp.231 jump or_true_52
            tmp.224 = 0
            jump or_end_53
        
          or_true_52:
            tmp.224 = 1
        
          or_end_53:
            if !tmp.224 jump end_if_54
            return 0
        
          end_if_54:
            return 1
            return 0
        }
        global function test_copy_array_element_with_padding() { 
            arr.19[0] = 0
            arr.19[4] = 1
            tmp.234 = truncate 2
            arr.19[8] = tmp.234
            arr.19[12] = 3
            arr.19[16] = 4
            tmp.235 = truncate 5
            arr.19[20] = tmp.235
            arr.19[24] = 6
            arr.19[28] = 7
            tmp.236 = truncate 8
            arr.19[32] = tmp.236
            elem.20[0] = 9
            elem.20[4] = 9
            tmp.237 = truncate 9
            elem.20[8] = tmp.237
            tmp.238 = &arr.19
            tmp.239 = sign_extend 1
            tmp.240 = add_ptr(tmp.238, index=tmp.239, scale=12)
            *tmp.240 = elem.20
            tmp.241 = *tmp.240
            tmp.242 = &arr.19
            tmp.243 = sign_extend 0
            tmp.244 = add_ptr(tmp.242, index=tmp.243, scale=12)
            tmp.245 = *tmp.244
            tmp.246 = tmp.245 != 0
            if tmp.246 jump or_true_56
            tmp.249 = &arr.19
            tmp.250 = sign_extend 0
            tmp.251 = add_ptr(tmp.249, index=tmp.250, scale=12)
            tmp.252 = add_ptr(tmp.251, index=4L, scale=1)
            tmp.253 = *tmp.252
            tmp.254 = tmp.253 != 1
            if tmp.254 jump or_true_56
            tmp.248 = 0
            jump or_end_57
        
          or_true_56:
            tmp.248 = 1
        
          or_end_57:
            if tmp.248 jump or_true_58
            tmp.257 = &arr.19
            tmp.258 = sign_extend 0
            tmp.259 = add_ptr(tmp.257, index=tmp.258, scale=12)
            tmp.260 = add_ptr(tmp.259, index=8L, scale=1)
            tmp.261 = *tmp.260
            tmp.262 = sign_extend tmp.261
            tmp.263 = tmp.262 != 2
            if tmp.263 jump or_true_58
            tmp.256 = 0
            jump or_end_59
        
          or_true_58:
            tmp.256 = 1
        
          or_end_59:
            if tmp.256 jump or_true_60
            tmp.266 = &arr.19
            tmp.267 = sign_extend 1
            tmp.268 = add_ptr(tmp.266, index=tmp.267, scale=12)
            tmp.269 = *tmp.268
            tmp.270 = tmp.269 != 9
            if tmp.270 jump or_true_60
            tmp.265 = 0
            jump or_end_61
        
          or_true_60:
            tmp.265 = 1
        
          or_end_61:
            if tmp.265 jump or_true_62
            tmp.273 = &arr.19
            tmp.274 = sign_extend 1
            tmp.275 = add_ptr(tmp.273, index=tmp.274, scale=12)
            tmp.276 = add_ptr(tmp.275, index=4L, scale=1)
            tmp.277 = *tmp.276
            tmp.278 = tmp.277 != 9
            if tmp.278 jump or_true_62
            tmp.272 = 0
            jump or_end_63
        
          or_true_62:
            tmp.272 = 1
        
          or_end_63:
            if tmp.272 jump or_true_64
            tmp.281 = &arr.19
            tmp.282 = sign_extend 1
            tmp.283 = add_ptr(tmp.281, index=tmp.282, scale=12)
            tmp.284 = add_ptr(tmp.283, index=8L, scale=1)
            tmp.285 = *tmp.284
            tmp.286 = sign_extend tmp.285
            tmp.287 = tmp.286 != 9
            if tmp.287 jump or_true_64
            tmp.280 = 0
            jump or_end_65
        
          or_true_64:
            tmp.280 = 1
        
          or_end_65:
            if tmp.280 jump or_true_66
            tmp.290 = &arr.19
            tmp.291 = sign_extend 2
            tmp.292 = add_ptr(tmp.290, index=tmp.291, scale=12)
            tmp.293 = *tmp.292
            tmp.294 = tmp.293 != 6
            if tmp.294 jump or_true_66
            tmp.289 = 0
            jump or_end_67
        
          or_true_66:
            tmp.289 = 1
        
          or_end_67:
            if tmp.289 jump or_true_68
            tmp.297 = &arr.19
            tmp.298 = sign_extend 2
            tmp.299 = add_ptr(tmp.297, index=tmp.298, scale=12)
            tmp.300 = add_ptr(tmp.299, index=4L, scale=1)
            tmp.301 = *tmp.300
            tmp.302 = tmp.301 != 7
            if tmp.302 jump or_true_68
            tmp.296 = 0
            jump or_end_69
        
          or_true_68:
            tmp.296 = 1
        
          or_end_69:
            if tmp.296 jump or_true_70
            tmp.305 = &arr.19
            tmp.306 = sign_extend 2
            tmp.307 = add_ptr(tmp.305, index=tmp.306, scale=12)
            tmp.308 = add_ptr(tmp.307, index=8L, scale=1)
            tmp.309 = *tmp.308
            tmp.310 = sign_extend tmp.309
            tmp.311 = tmp.310 != 8
            if tmp.311 jump or_true_70
            tmp.304 = 0
            jump or_end_71
        
          or_true_70:
            tmp.304 = 1
        
          or_end_71:
            if !tmp.304 jump end_if_72
            return 0
        
          end_if_72:
            return 1
            return 0
        }
        global function main() { 
            tmp.312 = test_copy_to_pointer()
            tmp.313 = ! tmp.312
            if !tmp.313 jump end_if_74
            return 1
        
          end_if_74:
            tmp.314 = test_copy_from_pointer()
            tmp.315 = ! tmp.314
            if !tmp.315 jump end_if_76
            return 2
        
          end_if_76:
            tmp.316 = test_copy_to_and_from_pointer()
            tmp.317 = ! tmp.316
            if !tmp.317 jump end_if_78
            return 3
        
          end_if_78:
            tmp.318 = test_copy_to_array_elem()
            tmp.319 = ! tmp.318
            if !tmp.319 jump end_if_80
            return 4
        
          end_if_80:
            tmp.320 = test_copy_from_array_elem()
            tmp.321 = ! tmp.320
            if !tmp.321 jump end_if_82
            return 5
        
          end_if_82:
            tmp.322 = test_copy_to_and_from_array_elem()
            tmp.323 = ! tmp.322
            if !tmp.323 jump end_if_84
            return 6
        
          end_if_84:
            tmp.324 = test_copy_array_element_with_padding()
            tmp.325 = ! tmp.324
            if !tmp.325 jump end_if_86
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
            small.6[0] = 0D
            small.6[8] = 0
            tmp.0 = malloc(24UL)
            tmp.1 = tmp.0
            outer_ptr.7 = tmp.1
            tmp.2 = truncate 100
            *outer_ptr.7 = tmp.2
            tmp.3 = *outer_ptr.7
            tmp.4 = add_ptr(outer_ptr.7, index=8L, scale=1)
            *tmp.4 = 21.5D
            tmp.5 = *tmp.4
            tmp.6 = add_ptr(outer_ptr.7, index=8L, scale=1)
            tmp.7 = add_ptr(tmp.6, index=8L, scale=1)
            *tmp.7 = 100001
            tmp.8 = *tmp.7
            tmp.9 = add_ptr(outer_ptr.7, index=8L, scale=1)
            tmp.10 = *tmp.9
            small.6 = tmp.10
            tmp.11 = small.6[0]
            tmp.12 = tmp.11 != 21.5D
            if tmp.12 jump or_true_0
            tmp.15 = small.6[8]
            tmp.16 = tmp.15 != 100001
            if tmp.16 jump or_true_0
            tmp.14 = 0
            jump or_end_1
        
          or_true_0:
            tmp.14 = 1
        
          or_end_1:
            if !tmp.14 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
        global function test_copy_to_member_pointer() { 
            small.8[0] = 99.25D
            small.8[8] = 987654
            tmp.17 = sign_extend 1
            tmp.18 = calloc(tmp.17, 24UL)
            tmp.19 = tmp.18
            outer_ptr.9 = tmp.19
            tmp.20 = add_ptr(outer_ptr.9, index=8L, scale=1)
            *tmp.20 = small.8
            tmp.21 = *tmp.20
            tmp.22 = add_ptr(outer_ptr.9, index=8L, scale=1)
            tmp.23 = *tmp.22
            tmp.24 = tmp.23 != 99.25D
            if tmp.24 jump or_true_4
            tmp.27 = add_ptr(outer_ptr.9, index=8L, scale=1)
            tmp.28 = add_ptr(tmp.27, index=8L, scale=1)
            tmp.29 = *tmp.28
            tmp.30 = tmp.29 != 987654
            if tmp.30 jump or_true_4
            tmp.26 = 0
            jump or_end_5
        
          or_true_4:
            tmp.26 = 1
        
          or_end_5:
            if !tmp.26 jump end_if_6
            return 0
        
          end_if_6:
            tmp.31 = *outer_ptr.9
            if tmp.31 jump or_true_8
            tmp.34 = add_ptr(outer_ptr.9, index=1L, scale=1)
            tmp.35 = *tmp.34
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
        global function test_copy_from_nested_member_pointer() { 
            small.10[0] = 99.25D
            small.10[8] = 987654
            tmp.36 = sign_extend 1
            tmp.37 = calloc(tmp.36, 40UL)
            tmp.38 = tmp.37
            outer_ptr.11 = tmp.38
            tmp.39 = add_ptr(outer_ptr.11, index=8L, scale=1)
            tmp.40 = sign_extend 1
            tmp.41 = calloc(tmp.40, 24UL)
            tmp.42 = tmp.41
            *tmp.39 = tmp.42
            tmp.43 = *tmp.39
            tmp.44 = - 5
            *outer_ptr.11 = tmp.44
            tmp.45 = *outer_ptr.11
            tmp.46 = add_ptr(outer_ptr.11, index=8L, scale=1)
            tmp.47 = *tmp.46
            tmp.48 = truncate 101
            *tmp.47 = tmp.48
            tmp.49 = *tmp.47
            tmp.50 = add_ptr(outer_ptr.11, index=8L, scale=1)
            tmp.51 = *tmp.50
            tmp.52 = add_ptr(tmp.51, index=1L, scale=1)
            tmp.53 = truncate 102
            *tmp.52 = tmp.53
            tmp.54 = *tmp.52
            tmp.55 = add_ptr(outer_ptr.11, index=8L, scale=1)
            tmp.56 = *tmp.55
            tmp.57 = add_ptr(tmp.56, index=8L, scale=1)
            *tmp.57 = 77.5D
            tmp.58 = *tmp.57
            tmp.59 = add_ptr(outer_ptr.11, index=8L, scale=1)
            tmp.60 = *tmp.59
            tmp.61 = add_ptr(tmp.60, index=8L, scale=1)
            tmp.62 = add_ptr(tmp.61, index=8L, scale=1)
            *tmp.62 = 88
            tmp.63 = *tmp.62
            tmp.64 = add_ptr(outer_ptr.11, index=8L, scale=1)
            tmp.65 = *tmp.64
            tmp.66 = add_ptr(tmp.65, index=8L, scale=1)
            tmp.67 = *tmp.66
            small.10 = tmp.67
            tmp.68 = small.10[0]
            tmp.69 = tmp.68 != 77.5D
            if tmp.69 jump or_true_12
            tmp.72 = small.10[8]
            tmp.73 = tmp.72 != 88
            if tmp.73 jump or_true_12
            tmp.71 = 0
            jump or_end_13
        
          or_true_12:
            tmp.71 = 1
        
          or_end_13:
            if !tmp.71 jump end_if_14
            return 0
        
          end_if_14:
            tmp.74 = *outer_ptr.11
            tmp.76 = - 5
            tmp.75 = tmp.74 != tmp.76
            if tmp.75 jump or_true_16
            tmp.79 = add_ptr(outer_ptr.11, index=16L, scale=1)
            tmp.80 = *tmp.79
            if tmp.80 jump or_true_16
            tmp.78 = 0
            jump or_end_17
        
          or_true_16:
            tmp.78 = 1
        
          or_end_17:
            if !tmp.78 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function test_copy_to_nested_member_pointer() { 
            small.12[0] = 99.25D
            small.12[8] = 987654
            tmp.81 = sign_extend 1
            tmp.82 = calloc(tmp.81, 40UL)
            tmp.83 = tmp.82
            outer_ptr.13 = tmp.83
            tmp.84 = add_ptr(outer_ptr.13, index=8L, scale=1)
            tmp.85 = sign_extend 1
            tmp.86 = calloc(tmp.85, 24UL)
            tmp.87 = tmp.86
            *tmp.84 = tmp.87
            tmp.88 = *tmp.84
            tmp.89 = add_ptr(outer_ptr.13, index=8L, scale=1)
            tmp.90 = *tmp.89
            tmp.91 = add_ptr(tmp.90, index=8L, scale=1)
            *tmp.91 = small.12
            tmp.92 = *tmp.91
            tmp.93 = add_ptr(outer_ptr.13, index=8L, scale=1)
            tmp.94 = *tmp.93
            tmp.95 = add_ptr(tmp.94, index=8L, scale=1)
            tmp.96 = *tmp.95
            tmp.97 = tmp.96 != 99.25D
            if tmp.97 jump or_true_20
            tmp.100 = add_ptr(outer_ptr.13, index=8L, scale=1)
            tmp.101 = *tmp.100
            tmp.102 = add_ptr(tmp.101, index=8L, scale=1)
            tmp.103 = add_ptr(tmp.102, index=8L, scale=1)
            tmp.104 = *tmp.103
            tmp.105 = tmp.104 != 987654
            if tmp.105 jump or_true_20
            tmp.99 = 0
            jump or_end_21
        
          or_true_20:
            tmp.99 = 1
        
          or_end_21:
            if !tmp.99 jump end_if_22
            return 0
        
          end_if_22:
            tmp.106 = add_ptr(outer_ptr.13, index=8L, scale=1)
            tmp.107 = *tmp.106
            tmp.108 = *tmp.107
            if tmp.108 jump or_true_24
            tmp.111 = add_ptr(outer_ptr.13, index=8L, scale=1)
            tmp.112 = *tmp.111
            tmp.113 = add_ptr(tmp.112, index=1L, scale=1)
            tmp.114 = *tmp.113
            if tmp.114 jump or_true_24
            tmp.110 = 0
            jump or_end_25
        
          or_true_24:
            tmp.110 = 1
        
          or_end_25:
            if !tmp.110 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function test_mixed_nested_access() { 
            s1.14[0] = 100
            tmp.115 = sign_extend 0
            s1.14[8] = tmp.115
            tmp.116 = truncate 0
            s1.14[16] = tmp.116
            tmp.117 = truncate 0
            s1.14[17] = tmp.117
            tmp.118 = int_to_double 0
            s1.14[24] = tmp.118
            s1.14[32] = 0
            tmp.119 = sign_extend 1
            tmp.120 = calloc(tmp.119, 40UL)
            tmp.121 = tmp.120
            s2_ptr.15 = tmp.121
            s1.14[0] = 2147483647
            tmp.122 = s1.14[0]
            tmp.123 = sign_extend 1
            tmp.124 = calloc(tmp.123, 40UL)
            tmp.125 = tmp.124
            s1.14[8] = tmp.125
            tmp.126 = s1.14[8]
            tmp.127 = s1.14[8]
            tmp.128 = truncate 125
            *tmp.127 = tmp.128
            tmp.129 = *tmp.127
            tmp.130 = s1.14[8]
            tmp.131 = add_ptr(tmp.130, index=1L, scale=1)
            tmp.132 = truncate 126
            *tmp.131 = tmp.132
            tmp.133 = *tmp.131
            tmp.134 = s1.14[8]
            tmp.135 = add_ptr(tmp.134, index=8L, scale=1)
            tmp.136 = - 50D
            *tmp.135 = tmp.136
            tmp.137 = *tmp.135
            tmp.138 = s1.14[8]
            tmp.139 = add_ptr(tmp.138, index=8L, scale=1)
            tmp.140 = add_ptr(tmp.139, index=8L, scale=1)
            tmp.141 = - 70
            *tmp.140 = tmp.141
            tmp.142 = *tmp.140
            tmp.143 = truncate 101
            s1.14[16] = tmp.143
            tmp.144 = s1.14[16]
            tmp.145 = truncate 102
            s1.14[17] = tmp.145
            tmp.146 = s1.14[17]
            tmp.147 = - 2147483647
            *s2_ptr.15 = tmp.147
            tmp.148 = *s2_ptr.15
            tmp.149 = add_ptr(s2_ptr.15, index=8L, scale=1)
            tmp.150 = sign_extend 1
            tmp.151 = calloc(tmp.150, 40UL)
            tmp.152 = tmp.151
            *tmp.149 = tmp.152
            tmp.153 = *tmp.149
            tmp.154 = add_ptr(s2_ptr.15, index=8L, scale=1)
            tmp.155 = *tmp.154
            tmp.156 = truncate 5
            *tmp.155 = tmp.156
            tmp.157 = *tmp.155
            tmp.158 = add_ptr(s2_ptr.15, index=8L, scale=1)
            tmp.159 = *tmp.158
            tmp.160 = add_ptr(tmp.159, index=1L, scale=1)
            tmp.161 = truncate 6
            *tmp.160 = tmp.161
            tmp.162 = *tmp.160
            tmp.163 = add_ptr(s2_ptr.15, index=16L, scale=1)
            tmp.164 = add_ptr(tmp.163, index=8L, scale=1)
            *tmp.164 = 800000000D
            tmp.165 = *tmp.164
            tmp.166 = add_ptr(s2_ptr.15, index=16L, scale=1)
            tmp.167 = add_ptr(tmp.166, index=8L, scale=1)
            tmp.168 = add_ptr(tmp.167, index=8L, scale=1)
            tmp.169 = - 5
            *tmp.168 = tmp.169
            tmp.170 = *tmp.168
            tmp.171 = s1.14[8]
            tmp.172 = add_ptr(tmp.171, index=8L, scale=1)
            tmp.173 = add_ptr(s2_ptr.15, index=16L, scale=1)
            tmp.174 = add_ptr(tmp.173, index=8L, scale=1)
            tmp.175 = *tmp.174
            *tmp.172 = tmp.175
            tmp.176 = *tmp.172
            tmp.177 = s1.14[8]
            tmp.178 = add_ptr(tmp.177, index=8L, scale=1)
            tmp.179 = *tmp.178
            tmp.180 = tmp.179 != 800000000D
            if tmp.180 jump or_true_28
            tmp.183 = s1.14[8]
            tmp.184 = add_ptr(tmp.183, index=8L, scale=1)
            tmp.185 = add_ptr(tmp.184, index=8L, scale=1)
            tmp.186 = *tmp.185
            tmp.188 = - 5
            tmp.187 = tmp.186 != tmp.188
            if tmp.187 jump or_true_28
            tmp.182 = 0
            jump or_end_29
        
          or_true_28:
            tmp.182 = 1
        
          or_end_29:
            if !tmp.182 jump end_if_30
            return 0
        
          end_if_30:
            tmp.189 = s1.14[8]
            tmp.190 = *tmp.189
            tmp.191 = sign_extend tmp.190
            tmp.192 = tmp.191 != 125
            if tmp.192 jump or_true_32
            tmp.195 = s1.14[8]
            tmp.196 = add_ptr(tmp.195, index=1L, scale=1)
            tmp.197 = *tmp.196
            tmp.198 = sign_extend tmp.197
            tmp.199 = tmp.198 != 126
            if tmp.199 jump or_true_32
            tmp.194 = 0
            jump or_end_33
        
          or_true_32:
            tmp.194 = 1
        
          or_end_33:
            if !tmp.194 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function test_member_from_cast() { 
            small.16[0] = 20D
            small.16[8] = 10
            tmp.200 = sign_extend 1
            tmp.201 = calloc(tmp.200, 24UL)
            outer_ptr.17 = tmp.201
            tmp.202 = outer_ptr.17
            tmp.203 = add_ptr(tmp.202, index=8L, scale=1)
            *tmp.203 = small.16
            tmp.204 = *tmp.203
            tmp.205 = outer_ptr.17
            tmp.206 = add_ptr(tmp.205, index=8L, scale=1)
            tmp.207 = *tmp.206
            tmp.208 = tmp.207 != 20D
            if tmp.208 jump or_true_36
            tmp.211 = outer_ptr.17
            tmp.212 = add_ptr(tmp.211, index=8L, scale=1)
            tmp.213 = add_ptr(tmp.212, index=8L, scale=1)
            tmp.214 = *tmp.213
            tmp.215 = tmp.214 != 10
            if tmp.215 jump or_true_36
            tmp.210 = 0
            jump or_end_37
        
          or_true_36:
            tmp.210 = 1
        
          or_end_37:
            if !tmp.210 jump end_if_38
            return 0
        
          end_if_38:
            return 1
            return 0
        }
        global function main() { 
            tmp.216 = test_copy_from_member_pointer()
            tmp.217 = ! tmp.216
            if !tmp.217 jump end_if_40
            return 1
        
          end_if_40:
            tmp.218 = test_copy_to_member_pointer()
            tmp.219 = ! tmp.218
            if !tmp.219 jump end_if_42
            return 2
        
          end_if_42:
            tmp.220 = test_copy_from_nested_member_pointer()
            tmp.221 = ! tmp.220
            if !tmp.221 jump end_if_44
            return 3
        
          end_if_44:
            tmp.222 = test_copy_to_nested_member_pointer()
            tmp.223 = ! tmp.222
            if !tmp.223 jump end_if_46
            return 4
        
          end_if_46:
            tmp.224 = test_mixed_nested_access()
            tmp.225 = ! tmp.224
            if !tmp.225 jump end_if_48
            return 5
        
          end_if_48:
            tmp.226 = test_member_from_cast()
            tmp.227 = ! tmp.226
            if !tmp.227 jump end_if_50
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
            substruct.5[0] = tmp.4
            tmp.5 = - 1
            tmp.6 = truncate tmp.5
            substruct.5[1] = tmp.6
            tmp.7 = - 1
            tmp.8 = truncate tmp.7
            substruct.5[2] = tmp.8
            tmp.9 = - 1
            tmp.10 = truncate tmp.9
            substruct.5[3] = tmp.10
            tmp.11 = - 1
            tmp.12 = truncate tmp.11
            substruct.5[4] = tmp.12
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
            tmp.27 = &substruct.5
            tmp.27 = add_ptr(tmp.27, index=2L, scale=1)
            tmp.28 = sign_extend 0
            tmp.29 = add_ptr(tmp.27, index=tmp.28, scale=1)
            tmp.30 = *tmp.29
            tmp.31 = sign_extend tmp.30
            tmp.32 = tmp.31 != 8
            if tmp.32 jump or_true_2
            tmp.26 = 0
            jump or_end_3
        
          or_true_2:
            tmp.26 = 1
        
          or_end_3:
            if tmp.26 jump or_true_4
            tmp.35 = &substruct.5
            tmp.35 = add_ptr(tmp.35, index=2L, scale=1)
            tmp.36 = sign_extend 1
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=1)
            tmp.38 = *tmp.37
            tmp.39 = sign_extend tmp.38
            tmp.40 = tmp.39 != 7
            if tmp.40 jump or_true_4
            tmp.34 = 0
            jump or_end_5
        
          or_true_4:
            tmp.34 = 1
        
          or_end_5:
            if tmp.34 jump or_true_6
            tmp.43 = &substruct.5
            tmp.43 = add_ptr(tmp.43, index=2L, scale=1)
            tmp.44 = sign_extend 2
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=1)
            tmp.46 = *tmp.45
            tmp.47 = sign_extend tmp.46
            tmp.48 = tmp.47 != 6
            if tmp.48 jump or_true_6
            tmp.42 = 0
            jump or_end_7
        
          or_true_6:
            tmp.42 = 1
        
          or_end_7:
            if !tmp.42 jump end_if_8
            return 0
        
          end_if_8:
            tmp.49 = &arr.4
            tmp.50 = sign_extend 0
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=1)
            tmp.52 = *tmp.51
            tmp.53 = sign_extend tmp.52
            tmp.54 = tmp.53 != 97
            if tmp.54 jump or_true_10
            tmp.57 = &arr.4
            tmp.58 = sign_extend 1
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=1)
            tmp.60 = *tmp.59
            tmp.61 = sign_extend tmp.60
            tmp.62 = tmp.61 != 98
            if tmp.62 jump or_true_10
            tmp.56 = 0
            jump or_end_11
        
          or_true_10:
            tmp.56 = 1
        
          or_end_11:
            if tmp.56 jump or_true_12
            tmp.65 = &arr.4
            tmp.66 = sign_extend 2
            tmp.67 = add_ptr(tmp.65, index=tmp.66, scale=1)
            tmp.68 = *tmp.67
            tmp.69 = sign_extend tmp.68
            tmp.70 = tmp.69 != 99
            if tmp.70 jump or_true_12
            tmp.64 = 0
            jump or_end_13
        
          or_true_12:
            tmp.64 = 1
        
          or_end_13:
            if tmp.64 jump or_true_14
            tmp.73 = &arr2.6
            tmp.74 = sign_extend 0
            tmp.75 = add_ptr(tmp.73, index=tmp.74, scale=1)
            tmp.76 = *tmp.75
            tmp.77 = sign_extend tmp.76
            tmp.78 = tmp.77 != 100
            if tmp.78 jump or_true_14
            tmp.72 = 0
            jump or_end_15
        
          or_true_14:
            tmp.72 = 1
        
          or_end_15:
            if tmp.72 jump or_true_16
            tmp.81 = &arr2.6
            tmp.82 = sign_extend 1
            tmp.83 = add_ptr(tmp.81, index=tmp.82, scale=1)
            tmp.84 = *tmp.83
            tmp.85 = sign_extend tmp.84
            tmp.86 = tmp.85 != 101
            if tmp.86 jump or_true_16
            tmp.80 = 0
            jump or_end_17
        
          or_true_16:
            tmp.80 = 1
        
          or_end_17:
            if tmp.80 jump or_true_18
            tmp.89 = &arr2.6
            tmp.90 = sign_extend 2
            tmp.91 = add_ptr(tmp.89, index=tmp.90, scale=1)
            tmp.92 = *tmp.91
            tmp.93 = sign_extend tmp.92
            tmp.94 = tmp.93 != 102
            if tmp.94 jump or_true_18
            tmp.88 = 0
            jump or_end_19
        
          or_true_18:
            tmp.88 = 1
        
          or_end_19:
            if !tmp.88 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function test_copy_to_member() { 
            tmp.95 = - 1
            tmp.96 = truncate tmp.95
            small_struct.8[0] = tmp.96
            tmp.97 = - 2
            tmp.98 = truncate tmp.97
            small_struct.8[1] = tmp.98
            tmp.99 = - 3
            tmp.100 = truncate tmp.99
            small_struct.8[2] = tmp.100
            tmp.101 = - 4
            tmp.102 = truncate tmp.101
            small_struct.8[3] = tmp.102
            tmp.103 = - 5
            tmp.104 = truncate tmp.103
            small_struct.8[4] = tmp.104
            big_struct.7[0] = small_struct.8
            tmp.105 = big_struct.7[0]
            tmp.106 = big_struct.7[0]
            tmp.107 = sign_extend tmp.106
            tmp.109 = - 1
            tmp.108 = tmp.107 != tmp.109
            if tmp.108 jump or_true_22
            tmp.112 = big_struct.7[1]
            tmp.113 = sign_extend tmp.112
            tmp.115 = - 2
            tmp.114 = tmp.113 != tmp.115
            if tmp.114 jump or_true_22
            tmp.111 = 0
            jump or_end_23
        
          or_true_22:
            tmp.111 = 1
        
          or_end_23:
            if tmp.111 jump or_true_24
            tmp.118 = &big_struct.7
            tmp.118 = add_ptr(tmp.118, index=2L, scale=1)
            tmp.119 = sign_extend 0
            tmp.120 = add_ptr(tmp.118, index=tmp.119, scale=1)
            tmp.121 = *tmp.120
            tmp.122 = sign_extend tmp.121
            tmp.124 = - 3
            tmp.123 = tmp.122 != tmp.124
            if tmp.123 jump or_true_24
            tmp.117 = 0
            jump or_end_25
        
          or_true_24:
            tmp.117 = 1
        
          or_end_25:
            if tmp.117 jump or_true_26
            tmp.127 = &big_struct.7
            tmp.127 = add_ptr(tmp.127, index=2L, scale=1)
            tmp.128 = sign_extend 1
            tmp.129 = add_ptr(tmp.127, index=tmp.128, scale=1)
            tmp.130 = *tmp.129
            tmp.131 = sign_extend tmp.130
            tmp.133 = - 4
            tmp.132 = tmp.131 != tmp.133
            if tmp.132 jump or_true_26
            tmp.126 = 0
            jump or_end_27
        
          or_true_26:
            tmp.126 = 1
        
          or_end_27:
            if tmp.126 jump or_true_28
            tmp.136 = &big_struct.7
            tmp.136 = add_ptr(tmp.136, index=2L, scale=1)
            tmp.137 = sign_extend 2
            tmp.138 = add_ptr(tmp.136, index=tmp.137, scale=1)
            tmp.139 = *tmp.138
            tmp.140 = sign_extend tmp.139
            tmp.142 = - 5
            tmp.141 = tmp.140 != tmp.142
            if tmp.141 jump or_true_28
            tmp.135 = 0
            jump or_end_29
        
          or_true_28:
            tmp.135 = 1
        
          or_end_29:
            if !tmp.135 jump end_if_30
            return 0
        
          end_if_30:
            tmp.143 = big_struct.7[5]
            if tmp.143 jump or_true_32
            tmp.146 = big_struct.7[6]
            if tmp.146 jump or_true_32
            tmp.145 = 0
            jump or_end_33
        
          or_true_32:
            tmp.145 = 1
        
          or_end_33:
            if !tmp.145 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function test_copy_from_nested_member() { 
            tmp.147 = - 1
            tmp.148 = truncate tmp.147
            biggest_struct.9[0] = tmp.148
            tmp.149 = - 2
            tmp.150 = truncate tmp.149
            biggest_struct.9[1] = tmp.150
            tmp.151 = - 3
            tmp.152 = truncate tmp.151
            biggest_struct.9[2] = tmp.152
            tmp.153 = - 4
            tmp.154 = truncate tmp.153
            biggest_struct.9[3] = tmp.154
            tmp.155 = - 5
            tmp.156 = truncate tmp.155
            biggest_struct.9[4] = tmp.156
            tmp.157 = - 6
            tmp.158 = truncate tmp.157
            biggest_struct.9[5] = tmp.158
            tmp.159 = - 7
            tmp.160 = truncate tmp.159
            biggest_struct.9[6] = tmp.160
            biggest_struct.9[8] = 0
            tmp.161 = biggest_struct.9[0]
            small_struct.10 = tmp.161
            tmp.162 = small_struct.10[0]
            tmp.163 = sign_extend tmp.162
            tmp.165 = - 1
            tmp.164 = tmp.163 != tmp.165
            if tmp.164 jump or_true_36
            tmp.168 = small_struct.10[1]
            tmp.169 = sign_extend tmp.168
            tmp.171 = - 2
            tmp.170 = tmp.169 != tmp.171
            if tmp.170 jump or_true_36
            tmp.167 = 0
            jump or_end_37
        
          or_true_36:
            tmp.167 = 1
        
          or_end_37:
            if tmp.167 jump or_true_38
            tmp.174 = &small_struct.10
            tmp.174 = add_ptr(tmp.174, index=2L, scale=1)
            tmp.175 = sign_extend 0
            tmp.176 = add_ptr(tmp.174, index=tmp.175, scale=1)
            tmp.177 = *tmp.176
            tmp.178 = sign_extend tmp.177
            tmp.180 = - 3
            tmp.179 = tmp.178 != tmp.180
            if tmp.179 jump or_true_38
            tmp.173 = 0
            jump or_end_39
        
          or_true_38:
            tmp.173 = 1
        
          or_end_39:
            if tmp.173 jump or_true_40
            tmp.183 = &small_struct.10
            tmp.183 = add_ptr(tmp.183, index=2L, scale=1)
            tmp.184 = sign_extend 1
            tmp.185 = add_ptr(tmp.183, index=tmp.184, scale=1)
            tmp.186 = *tmp.185
            tmp.187 = sign_extend tmp.186
            tmp.189 = - 4
            tmp.188 = tmp.187 != tmp.189
            if tmp.188 jump or_true_40
            tmp.182 = 0
            jump or_end_41
        
          or_true_40:
            tmp.182 = 1
        
          or_end_41:
            if tmp.182 jump or_true_42
            tmp.192 = &small_struct.10
            tmp.192 = add_ptr(tmp.192, index=2L, scale=1)
            tmp.193 = sign_extend 2
            tmp.194 = add_ptr(tmp.192, index=tmp.193, scale=1)
            tmp.195 = *tmp.194
            tmp.196 = sign_extend tmp.195
            tmp.198 = - 5
            tmp.197 = tmp.196 != tmp.198
            if tmp.197 jump or_true_42
            tmp.191 = 0
            jump or_end_43
        
          or_true_42:
            tmp.191 = 1
        
          or_end_43:
            if !tmp.191 jump end_if_44
            return 0
        
          end_if_44:
            return 1
            return 0
        }
        global function test_copy_to_nested_member() { 
            tmp.199 = truncate 0
            biggest_struct.11[0] = tmp.199
            tmp.200 = truncate 0
            biggest_struct.11[1] = tmp.200
            tmp.201 = truncate 0
            biggest_struct.11[2] = tmp.201
            tmp.202 = truncate 0
            biggest_struct.11[3] = tmp.202
            tmp.203 = truncate 0
            biggest_struct.11[4] = tmp.203
            tmp.204 = truncate 0
            biggest_struct.11[5] = tmp.204
            tmp.205 = truncate 0
            biggest_struct.11[6] = tmp.205
            tmp.206 = - 1
            biggest_struct.11[8] = tmp.206
            biggest_struct.11[0] = small_struct.12
            tmp.207 = biggest_struct.11[0]
            tmp.208 = biggest_struct.11[0]
            tmp.209 = sign_extend tmp.208
            tmp.210 = tmp.209 != 50
            if tmp.210 jump or_true_46
            tmp.213 = biggest_struct.11[1]
            tmp.214 = sign_extend tmp.213
            tmp.215 = tmp.214 != 51
            if tmp.215 jump or_true_46
            tmp.212 = 0
            jump or_end_47
        
          or_true_46:
            tmp.212 = 1
        
          or_end_47:
            if tmp.212 jump or_true_48
            tmp.218 = &biggest_struct.11
            tmp.218 = add_ptr(tmp.218, index=2L, scale=1)
            tmp.219 = sign_extend 0
            tmp.220 = add_ptr(tmp.218, index=tmp.219, scale=1)
            tmp.221 = *tmp.220
            tmp.222 = sign_extend tmp.221
            tmp.223 = tmp.222 != 52
            if tmp.223 jump or_true_48
            tmp.217 = 0
            jump or_end_49
        
          or_true_48:
            tmp.217 = 1
        
          or_end_49:
            if tmp.217 jump or_true_50
            tmp.226 = &biggest_struct.11
            tmp.226 = add_ptr(tmp.226, index=2L, scale=1)
            tmp.227 = sign_extend 1
            tmp.228 = add_ptr(tmp.226, index=tmp.227, scale=1)
            tmp.229 = *tmp.228
            tmp.230 = sign_extend tmp.229
            tmp.231 = tmp.230 != 53
            if tmp.231 jump or_true_50
            tmp.225 = 0
            jump or_end_51
        
          or_true_50:
            tmp.225 = 1
        
          or_end_51:
            if tmp.225 jump or_true_52
            tmp.234 = &biggest_struct.11
            tmp.234 = add_ptr(tmp.234, index=2L, scale=1)
            tmp.235 = sign_extend 2
            tmp.236 = add_ptr(tmp.234, index=tmp.235, scale=1)
            tmp.237 = *tmp.236
            tmp.238 = sign_extend tmp.237
            tmp.239 = tmp.238 != 54
            if tmp.239 jump or_true_52
            tmp.233 = 0
            jump or_end_53
        
          or_true_52:
            tmp.233 = 1
        
          or_end_53:
            if !tmp.233 jump end_if_54
            return 0
        
          end_if_54:
            tmp.240 = biggest_struct.11[5]
            if tmp.240 jump or_true_56
            tmp.243 = biggest_struct.11[6]
            if tmp.243 jump or_true_56
            tmp.242 = 0
            jump or_end_57
        
          or_true_56:
            tmp.242 = 1
        
          or_end_57:
            if !tmp.242 jump end_if_58
            return 0
        
          end_if_58:
            tmp.244 = biggest_struct.11[8]
            tmp.246 = - 1
            tmp.245 = tmp.244 != tmp.246
            if !tmp.245 jump end_if_60
            return 0
        
          end_if_60:
            return 1
            return 0
        }
        global function test_copy_from_conditional() { 
            tmp.247 = truncate 127
            big_struct.13[0] = tmp.247
            tmp.248 = - 128
            tmp.249 = truncate tmp.248
            big_struct.13[1] = tmp.249
            tmp.250 = truncate 61
            big_struct.13[2] = tmp.250
            tmp.251 = truncate 62
            big_struct.13[3] = tmp.251
            tmp.252 = truncate 63
            big_struct.13[4] = tmp.252
            tmp.253 = - 10
            tmp.254 = truncate tmp.253
            big_struct.13[5] = tmp.254
            tmp.255 = - 11
            tmp.256 = truncate tmp.255
            big_struct.13[6] = tmp.256
            tmp.257 = truncate 0
            big_struct2.14[0] = tmp.257
            tmp.258 = truncate 1
            big_struct2.14[1] = tmp.258
            tmp.259 = truncate 2
            big_struct2.14[2] = tmp.259
            tmp.260 = truncate 3
            big_struct2.14[3] = tmp.260
            tmp.261 = truncate 4
            big_struct2.14[4] = tmp.261
            tmp.262 = truncate 5
            big_struct2.14[5] = tmp.262
            tmp.263 = truncate 6
            big_struct2.14[6] = tmp.263
            if !f.16 jump else_63
            tmp.264 = big_struct.13
            jump end_if_62
        
          else_63:
            tmp.264 = big_struct2.14
        
          end_if_62:
            tmp.265 = tmp.264[0]
            small_struct.17 = tmp.265
            tmp.266 = small_struct.17[0]
            tmp.267 = sign_extend tmp.266
            tmp.268 = tmp.267 != 0
            if tmp.268 jump or_true_64
            tmp.271 = small_struct.17[1]
            tmp.272 = sign_extend tmp.271
            tmp.273 = tmp.272 != 1
            if tmp.273 jump or_true_64
            tmp.270 = 0
            jump or_end_65
        
          or_true_64:
            tmp.270 = 1
        
          or_end_65:
            if tmp.270 jump or_true_66
            tmp.276 = &small_struct.17
            tmp.276 = add_ptr(tmp.276, index=2L, scale=1)
            tmp.277 = sign_extend 0
            tmp.278 = add_ptr(tmp.276, index=tmp.277, scale=1)
            tmp.279 = *tmp.278
            tmp.280 = sign_extend tmp.279
            tmp.281 = tmp.280 != 2
            if tmp.281 jump or_true_66
            tmp.275 = 0
            jump or_end_67
        
          or_true_66:
            tmp.275 = 1
        
          or_end_67:
            if tmp.275 jump or_true_68
            tmp.284 = &small_struct.17
            tmp.284 = add_ptr(tmp.284, index=2L, scale=1)
            tmp.285 = sign_extend 1
            tmp.286 = add_ptr(tmp.284, index=tmp.285, scale=1)
            tmp.287 = *tmp.286
            tmp.288 = sign_extend tmp.287
            tmp.289 = tmp.288 != 3
            if tmp.289 jump or_true_68
            tmp.283 = 0
            jump or_end_69
        
          or_true_68:
            tmp.283 = 1
        
          or_end_69:
            if tmp.283 jump or_true_70
            tmp.292 = &small_struct.17
            tmp.292 = add_ptr(tmp.292, index=2L, scale=1)
            tmp.293 = sign_extend 2
            tmp.294 = add_ptr(tmp.292, index=tmp.293, scale=1)
            tmp.295 = *tmp.294
            tmp.296 = sign_extend tmp.295
            tmp.297 = tmp.296 != 4
            if tmp.297 jump or_true_70
            tmp.291 = 0
            jump or_end_71
        
          or_true_70:
            tmp.291 = 1
        
          or_end_71:
            if !tmp.291 jump end_if_72
            return 0
        
          end_if_72:
            if !t.15 jump else_75
            tmp.298 = big_struct.13
            jump end_if_74
        
          else_75:
            tmp.298 = big_struct2.14
        
          end_if_74:
            tmp.299 = tmp.298[0]
            small_struct.17 = tmp.299
            tmp.300 = small_struct.17[0]
            tmp.301 = sign_extend tmp.300
            tmp.302 = tmp.301 != 127
            if tmp.302 jump or_true_76
            tmp.305 = small_struct.17[1]
            tmp.306 = sign_extend tmp.305
            tmp.308 = - 128
            tmp.307 = tmp.306 != tmp.308
            if tmp.307 jump or_true_76
            tmp.304 = 0
            jump or_end_77
        
          or_true_76:
            tmp.304 = 1
        
          or_end_77:
            if tmp.304 jump or_true_78
            tmp.311 = &small_struct.17
            tmp.311 = add_ptr(tmp.311, index=2L, scale=1)
            tmp.312 = sign_extend 0
            tmp.313 = add_ptr(tmp.311, index=tmp.312, scale=1)
            tmp.314 = *tmp.313
            tmp.315 = sign_extend tmp.314
            tmp.316 = tmp.315 != 61
            if tmp.316 jump or_true_78
            tmp.310 = 0
            jump or_end_79
        
          or_true_78:
            tmp.310 = 1
        
          or_end_79:
            if tmp.310 jump or_true_80
            tmp.319 = &small_struct.17
            tmp.319 = add_ptr(tmp.319, index=2L, scale=1)
            tmp.320 = sign_extend 1
            tmp.321 = add_ptr(tmp.319, index=tmp.320, scale=1)
            tmp.322 = *tmp.321
            tmp.323 = sign_extend tmp.322
            tmp.324 = tmp.323 != 62
            if tmp.324 jump or_true_80
            tmp.318 = 0
            jump or_end_81
        
          or_true_80:
            tmp.318 = 1
        
          or_end_81:
            if tmp.318 jump or_true_82
            tmp.327 = &small_struct.17
            tmp.327 = add_ptr(tmp.327, index=2L, scale=1)
            tmp.328 = sign_extend 2
            tmp.329 = add_ptr(tmp.327, index=tmp.328, scale=1)
            tmp.330 = *tmp.329
            tmp.331 = sign_extend tmp.330
            tmp.332 = tmp.331 != 63
            if tmp.332 jump or_true_82
            tmp.326 = 0
            jump or_end_83
        
          or_true_82:
            tmp.326 = 1
        
          or_end_83:
            if !tmp.326 jump end_if_84
            return 0
        
          end_if_84:
            return 1
            return 0
        }
        global function test_copy_from_assignment() { 
            tmp.333 = truncate 127
            big_struct.18[0] = tmp.333
            tmp.334 = - 128
            tmp.335 = truncate tmp.334
            big_struct.18[1] = tmp.335
            tmp.336 = truncate 61
            big_struct.18[2] = tmp.336
            tmp.337 = truncate 62
            big_struct.18[3] = tmp.337
            tmp.338 = truncate 63
            big_struct.18[4] = tmp.338
            tmp.339 = - 10
            tmp.340 = truncate tmp.339
            big_struct.18[5] = tmp.340
            tmp.341 = - 11
            tmp.342 = truncate tmp.341
            big_struct.18[6] = tmp.342
            big_struct2.19 = big_struct.18
            tmp.343 = big_struct2.19[0]
            small_struct.20 = tmp.343
            tmp.344 = small_struct.20[0]
            tmp.345 = sign_extend tmp.344
            tmp.346 = tmp.345 != 127
            if tmp.346 jump or_true_86
            tmp.349 = small_struct.20[1]
            tmp.350 = sign_extend tmp.349
            tmp.352 = - 128
            tmp.351 = tmp.350 != tmp.352
            if tmp.351 jump or_true_86
            tmp.348 = 0
            jump or_end_87
        
          or_true_86:
            tmp.348 = 1
        
          or_end_87:
            if tmp.348 jump or_true_88
            tmp.355 = &small_struct.20
            tmp.355 = add_ptr(tmp.355, index=2L, scale=1)
            tmp.356 = sign_extend 0
            tmp.357 = add_ptr(tmp.355, index=tmp.356, scale=1)
            tmp.358 = *tmp.357
            tmp.359 = sign_extend tmp.358
            tmp.360 = tmp.359 != 61
            if tmp.360 jump or_true_88
            tmp.354 = 0
            jump or_end_89
        
          or_true_88:
            tmp.354 = 1
        
          or_end_89:
            if tmp.354 jump or_true_90
            tmp.363 = &small_struct.20
            tmp.363 = add_ptr(tmp.363, index=2L, scale=1)
            tmp.364 = sign_extend 1
            tmp.365 = add_ptr(tmp.363, index=tmp.364, scale=1)
            tmp.366 = *tmp.365
            tmp.367 = sign_extend tmp.366
            tmp.368 = tmp.367 != 62
            if tmp.368 jump or_true_90
            tmp.362 = 0
            jump or_end_91
        
          or_true_90:
            tmp.362 = 1
        
          or_end_91:
            if tmp.362 jump or_true_92
            tmp.371 = &small_struct.20
            tmp.371 = add_ptr(tmp.371, index=2L, scale=1)
            tmp.372 = sign_extend 2
            tmp.373 = add_ptr(tmp.371, index=tmp.372, scale=1)
            tmp.374 = *tmp.373
            tmp.375 = sign_extend tmp.374
            tmp.376 = tmp.375 != 63
            if tmp.376 jump or_true_92
            tmp.370 = 0
            jump or_end_93
        
          or_true_92:
            tmp.370 = 1
        
          or_end_93:
            if !tmp.370 jump end_if_94
            return 0
        
          end_if_94:
            tmp.377 = big_struct2.19[0]
            tmp.378 = sign_extend tmp.377
            tmp.379 = tmp.378 != 127
            if tmp.379 jump or_true_96
            tmp.382 = big_struct2.19[1]
            tmp.383 = sign_extend tmp.382
            tmp.385 = - 128
            tmp.384 = tmp.383 != tmp.385
            if tmp.384 jump or_true_96
            tmp.381 = 0
            jump or_end_97
        
          or_true_96:
            tmp.381 = 1
        
          or_end_97:
            if tmp.381 jump or_true_98
            tmp.388 = &big_struct2.19
            tmp.388 = add_ptr(tmp.388, index=2L, scale=1)
            tmp.389 = sign_extend 0
            tmp.390 = add_ptr(tmp.388, index=tmp.389, scale=1)
            tmp.391 = *tmp.390
            tmp.392 = sign_extend tmp.391
            tmp.393 = tmp.392 != 61
            if tmp.393 jump or_true_98
            tmp.387 = 0
            jump or_end_99
        
          or_true_98:
            tmp.387 = 1
        
          or_end_99:
            if tmp.387 jump or_true_100
            tmp.396 = &big_struct2.19
            tmp.396 = add_ptr(tmp.396, index=2L, scale=1)
            tmp.397 = sign_extend 1
            tmp.398 = add_ptr(tmp.396, index=tmp.397, scale=1)
            tmp.399 = *tmp.398
            tmp.400 = sign_extend tmp.399
            tmp.401 = tmp.400 != 62
            if tmp.401 jump or_true_100
            tmp.395 = 0
            jump or_end_101
        
          or_true_100:
            tmp.395 = 1
        
          or_end_101:
            if tmp.395 jump or_true_102
            tmp.404 = &big_struct2.19
            tmp.404 = add_ptr(tmp.404, index=2L, scale=1)
            tmp.405 = sign_extend 2
            tmp.406 = add_ptr(tmp.404, index=tmp.405, scale=1)
            tmp.407 = *tmp.406
            tmp.408 = sign_extend tmp.407
            tmp.409 = tmp.408 != 63
            if tmp.409 jump or_true_102
            tmp.403 = 0
            jump or_end_103
        
          or_true_102:
            tmp.403 = 1
        
          or_end_103:
            if tmp.403 jump or_true_104
            tmp.412 = big_struct2.19[5]
            tmp.413 = sign_extend tmp.412
            tmp.415 = - 10
            tmp.414 = tmp.413 != tmp.415
            if tmp.414 jump or_true_104
            tmp.411 = 0
            jump or_end_105
        
          or_true_104:
            tmp.411 = 1
        
          or_end_105:
            if tmp.411 jump or_true_106
            tmp.418 = big_struct2.19[6]
            tmp.419 = sign_extend tmp.418
            tmp.421 = - 11
            tmp.420 = tmp.419 != tmp.421
            if tmp.420 jump or_true_106
            tmp.417 = 0
            jump or_end_107
        
          or_true_106:
            tmp.417 = 1
        
          or_end_107:
            if !tmp.417 jump end_if_108
            return 0
        
          end_if_108:
            return 1
            return 0
        }
        global function main() { 
            tmp.422 = test_copy_from_member()
            tmp.423 = ! tmp.422
            if !tmp.423 jump end_if_110
            return 1
        
          end_if_110:
            tmp.424 = test_copy_to_member()
            tmp.425 = ! tmp.424
            if !tmp.425 jump end_if_112
            return 2
        
          end_if_112:
            tmp.426 = test_copy_from_nested_member()
            tmp.427 = ! tmp.426
            if !tmp.427 jump end_if_114
            return 3
        
          end_if_114:
            tmp.428 = test_copy_to_nested_member()
            tmp.429 = ! tmp.428
            if !tmp.429 jump end_if_116
            return 4
        
          end_if_116:
            tmp.430 = test_copy_from_conditional()
            tmp.431 = ! tmp.430
            if !tmp.431 jump end_if_118
            return 6
        
          end_if_118:
            tmp.432 = test_copy_from_assignment()
            tmp.433 = ! tmp.432
            if !tmp.433 jump end_if_120
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
            tmp.8 = &y
            tmp.9 = sign_extend 0
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=1)
            tmp.11 = &y
            tmp.12 = sign_extend 0
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=1)
            tmp.14 = *tmp.13
            tmp.15 = sign_extend tmp.14
            tmp.16 = tmp.15 + 3
            tmp.17 = truncate tmp.16
            *tmp.10 = tmp.17
            tmp.18 = *tmp.10
            tmp.19 = &y
            tmp.20 = sign_extend 1
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=1)
            tmp.22 = &y
            tmp.23 = sign_extend 1
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=1)
            tmp.25 = *tmp.24
            tmp.26 = sign_extend tmp.25
            tmp.27 = tmp.26 + 3
            tmp.28 = truncate tmp.27
            *tmp.21 = tmp.28
            tmp.29 = *tmp.21
            tmp.30 = &y
            tmp.31 = sign_extend 2
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=1)
            tmp.33 = &y
            tmp.34 = sign_extend 2
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = sign_extend tmp.36
            tmp.38 = tmp.37 + 3
            tmp.39 = truncate tmp.38
            *tmp.32 = tmp.39
            tmp.40 = *tmp.32
            return 0
        }
        global function test_copy() { 
            a.6[0] = 'a'
            a.6[1] = 'b'
            a.6[2] = 'c'
            b.7[0] = 'x'
            b.7[1] = 'y'
            b.7[2] = 'z'
            c.8[0] = 'd'
            c.8[1] = 'e'
            c.8[2] = 'f'
            b.7 = y
            tmp.41 = &a.6
            validate_array(tmp.41, 97, 1)
            tmp.42 = &b.7
            validate_array(tmp.42, 0, 2)
            tmp.43 = &c.8
            validate_array(tmp.43, 100, 3)
            return 0
            return 0
        }
        global function validate_static(start.9, code.10) { 
            tmp.44 = &to_validate
            validate_array(tmp.44, start.9, code.10)
            return 0
        }
        global function test_load() { 
            a.12[0] = 'g'
            a.12[1] = 'h'
            a.12[2] = 'i'
            tmp.45 = *ptr
            b.11 = tmp.45
            to_validate = a.12
            validate_static(103, 4)
            to_validate = b.11
            validate_static(3, 5)
            return 0
            return 0
        }
        global function test_store() { 
            struct_array.13[0] = 'j'
            struct_array.13[1] = 'k'
            struct_array.13[2] = 'l'
            struct_array.13[3] = 'x'
            struct_array.13[4] = 'y'
            struct_array.13[5] = 'z'
            struct_array.13[6] = 'm'
            struct_array.13[7] = 'n'
            struct_array.13[8] = 'o'
            tmp.46 = &struct_array.13
            tmp.47 = sign_extend 1
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=3)
            ptr.14 = tmp.48
            *ptr.14 = y
            tmp.49 = *ptr.14
            tmp.50 = &struct_array.13
            tmp.51 = sign_extend 0
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=3)
            validate_array(tmp.52, 106, 6)
            tmp.53 = &struct_array.13
            tmp.54 = sign_extend 1
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=3)
            validate_array(tmp.55, 6, 7)
            tmp.56 = &struct_array.13
            tmp.57 = sign_extend 2
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=3)
            validate_array(tmp.58, 109, 8)
            return 0
            return 0
        }
        global function test_copy_from_offset() { 
            a.16[0] = 'p'
            a.16[1] = 'q'
            a.16[2] = 'r'
            tmp.59 = container.18[1]
            b.17 = tmp.59
            to_validate = a.16
            validate_static(112, 9)
            to_validate = b.17
            validate_static(9, 10)
            return 0
            return 0
        }
        global function test_copy_to_offset() { 
            tmp.60 = truncate 120
            container.19[0] = tmp.60
            tmp.61 = truncate 0
            container.19[1] = tmp.61
            tmp.62 = truncate 0
            container.19[2] = tmp.62
            tmp.63 = truncate 0
            container.19[3] = tmp.63
            container.19[4] = 's'
            container.19[5] = 't'
            container.19[6] = 'u'
            container.19[1] = y
            tmp.64 = container.19[1]
            tmp.65 = container.19[0]
            tmp.66 = sign_extend tmp.65
            tmp.67 = tmp.66 != 120
            if !tmp.67 jump end_if_2
            exit(11)
        
          end_if_2:
            tmp.68 = &container.19
            tmp.68 = add_ptr(tmp.68, index=1L, scale=1)
            validate_array(tmp.68, 12, 12)
            tmp.69 = &container.19
            tmp.69 = add_ptr(tmp.69, index=4L, scale=1)
            validate_array(tmp.69, 115, 13)
            return 0
            return 0
        }
        global function main() { 
            tmp.70 = &y
            ptr = tmp.70
            tmp.71 = test_copy()
            increment_y()
            tmp.72 = test_load()
            increment_y()
            tmp.73 = test_store()
            increment_y()
            tmp.74 = test_copy_from_offset()
            increment_y()
            tmp.75 = test_copy_to_offset()
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
            arg.3[0] = 1
            arg.3[4] = 2
            tmp.0 = foo(arg.3)
            return tmp.0
            return 0
        }
        global function foo(blah.4) { 
            tmp.1 = blah.4[0]
            tmp.3 = blah.4[4]
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
            tmp.4 = &s.19
            tmp.4 = add_ptr(tmp.4, index=4L, scale=1)
            tmp.5 = &string.0
            tmp.6 = strcmp(tmp.4, tmp.5)
            if tmp.6 jump or_true_0
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
            tmp.7 = s.20[0]
            tmp.8 = sign_extend tmp.7
            tmp.9 = tmp.8 != 127
            if tmp.9 jump or_true_4
            tmp.12 = s.20[4]
            tmp.13 = tmp.12 != 2147483647
            if tmp.13 jump or_true_4
            tmp.11 = 0
            jump or_end_5
        
          or_true_4:
            tmp.11 = 1
        
          or_end_5:
            if tmp.11 jump or_true_6
            tmp.16 = s.20[8]
            tmp.17 = sign_extend tmp.16
            tmp.19 = - 128
            tmp.18 = tmp.17 != tmp.19
            if tmp.18 jump or_true_6
            tmp.15 = 0
            jump or_end_7
        
          or_true_6:
            tmp.15 = 1
        
          or_end_7:
            if !tmp.15 jump end_if_8
            return 0
        
          end_if_8:
            return 1
            return 0
        }
        global function test_flattened_ints(s.21) { 
            tmp.20 = s.21[0]
            tmp.21 = sign_extend tmp.20
            tmp.22 = tmp.21 != 127
            if tmp.22 jump or_true_10
            tmp.25 = s.21[4]
            tmp.26 = tmp.25 != 2147483647
            if tmp.26 jump or_true_10
            tmp.24 = 0
            jump or_end_11
        
          or_true_10:
            tmp.24 = 1
        
          or_end_11:
            if tmp.24 jump or_true_12
            tmp.29 = s.21[8]
            tmp.30 = sign_extend tmp.29
            tmp.32 = - 128
            tmp.31 = tmp.30 != tmp.32
            if tmp.31 jump or_true_12
            tmp.28 = 0
            jump or_end_13
        
          or_true_12:
            tmp.28 = 1
        
          or_end_13:
            if !tmp.28 jump end_if_14
            return 0
        
          end_if_14:
            return 1
            return 0
        }
        global function test_large(s.22) { 
            tmp.33 = s.22[0]
            tmp.34 = tmp.33 != 200000
            if tmp.34 jump or_true_16
            tmp.37 = s.22[8]
            tmp.38 = tmp.37 != 23.25D
            if tmp.38 jump or_true_16
            tmp.36 = 0
            jump or_end_17
        
          or_true_16:
            tmp.36 = 1
        
          or_end_17:
            if tmp.36 jump or_true_18
            tmp.41 = &s.22
            tmp.41 = add_ptr(tmp.41, index=16L, scale=1)
            tmp.42 = &string.1
            tmp.43 = strcmp(tmp.41, tmp.42)
            if tmp.43 jump or_true_18
            tmp.40 = 0
            jump or_end_19
        
          or_true_18:
            tmp.40 = 1
        
          or_end_19:
            if !tmp.40 jump end_if_20
            return 0
        
          end_if_20:
            return 1
            return 0
        }
        global function test_two_ints(s.23) { 
            tmp.44 = s.23[0]
            tmp.45 = tmp.44 != 999
            if tmp.45 jump or_true_22
            tmp.48 = s.23[4]
            tmp.49 = tmp.48 != 888
            if tmp.49 jump or_true_22
            tmp.47 = 0
            jump or_end_23
        
          or_true_22:
            tmp.47 = 1
        
          or_end_23:
            if !tmp.47 jump end_if_24
            return 0
        
          end_if_24:
            return 1
            return 0
        }
        global function test_nested_double(s.24) { 
            tmp.50 = &s.24
            tmp.51 = sign_extend 0
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=8)
            tmp.53 = *tmp.52
            tmp.54 = tmp.53 != 25125D
            if !tmp.54 jump end_if_26
            return 0
        
          end_if_26:
            return 1
            return 0
        }
        global function test_two_eightbytes(s.25) { 
            tmp.55 = s.25[0]
            tmp.56 = tmp.55 != 1000D
            if tmp.56 jump or_true_28
            tmp.59 = s.25[8]
            tmp.60 = sign_extend tmp.59
            tmp.61 = tmp.60 != 120
            if tmp.61 jump or_true_28
            tmp.58 = 0
            jump or_end_29
        
          or_true_28:
            tmp.58 = 1
        
          or_end_29:
            if !tmp.58 jump end_if_30
            return 0
        
          end_if_30:
            return 1
            return 0
        }
        global function test_pass_in_memory(s.26) { 
            tmp.62 = s.26[0]
            tmp.63 = tmp.62 != 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            if tmp.63 jump or_true_32
            tmp.66 = s.26[8]
            tmp.68 = - 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            tmp.67 = tmp.66 != tmp.68
            if tmp.67 jump or_true_32
            tmp.65 = 0
            jump or_end_33
        
          or_true_32:
            tmp.65 = 1
        
          or_end_33:
            if tmp.65 jump or_true_34
            tmp.71 = s.26[16]
            tmp.73 = - 2147483647
            tmp.72 = tmp.71 != tmp.73
            if tmp.72 jump or_true_34
            tmp.70 = 0
            jump or_end_35
        
          or_true_34:
            tmp.70 = 1
        
          or_end_35:
            if tmp.70 jump or_true_36
            tmp.76 = s.26[24]
            tmp.78 = - 9223372036854775807L
            tmp.77 = tmp.76 != tmp.78
            if tmp.77 jump or_true_36
            tmp.75 = 0
            jump or_end_37
        
          or_true_36:
            tmp.75 = 1
        
          or_end_37:
            if !tmp.75 jump end_if_38
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
            s1.19[0] = 0
            s1.19[4] = 'l'
            s1.19[5] = 'm'
            s1.19[6] = 'n'
            s1.19[7] = 'o'
            s1.19[8] = 'p'
            s1.19[9] = 'q'
            s1.19[10] = 'r'
            s1.19[11] = '\0'
            tmp.0 = test_twelve_bytes(s1.19)
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = truncate 127
            s2.20[0] = tmp.2
            s2.20[4] = 2147483647
            tmp.3 = - 128
            tmp.4 = truncate tmp.3
            s2.20[8] = tmp.4
            tmp.5 = test_nested_ints(s2.20)
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = truncate 127
            s3.21[0] = tmp.7
            s3.21[4] = 2147483647
            tmp.8 = - 128
            tmp.9 = truncate tmp.8
            s3.21[8] = tmp.9
            tmp.10 = test_flattened_ints(s3.21)
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            s4.22[0] = 200000
            s4.22[8] = 23.25D
            s4.22[16] = 'a'
            s4.22[17] = 'b'
            s4.22[18] = 'c'
            s4.22[19] = 'd'
            s4.22[20] = 'e'
            s4.22[21] = 'f'
            s4.22[22] = 'g'
            s4.22[23] = 'h'
            s4.22[24] = 'i'
            s4.22[25] = '\0'
            tmp.12 = test_large(s4.22)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            s5.23[0] = 999
            s5.23[4] = 888
            tmp.14 = test_two_ints(s5.23)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_8
            return 5
        
          end_if_8:
            s6.24[0] = 25125D
            tmp.16 = test_nested_double(s6.24)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_10
            return 6
        
          end_if_10:
            s7.25[0] = 1000D
            tmp.18 = truncate 120
            s7.25[8] = tmp.18
            tmp.19 = test_two_eightbytes(s7.25)
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_12
            return 7
        
          end_if_12:
            s8.26[0] = 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            tmp.21 = - 170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
            s8.26[8] = tmp.21
            tmp.22 = - 2147483647
            s8.26[16] = tmp.22
            tmp.23 = - 9223372036854775807L
            s8.26[24] = tmp.23
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
            tmp.1 = s.4[0]
            if tmp.1 jump or_true_0
            tmp.4 = s.4[8]
            tmp.5 = tmp.4 != 3
            if tmp.5 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if !tmp.3 jump end_if_2
            return 0
        
          end_if_2:
            tmp.6 = copy.5[0]
            tmp.7 = tmp.6 != 2D
            if tmp.7 jump or_true_4
            tmp.10 = copy.5[8]
            tmp.11 = tmp.10 != 3
            if tmp.11 jump or_true_4
            tmp.9 = 0
            jump or_end_5
        
          or_true_4:
            tmp.9 = 1
        
          or_end_5:
            if !tmp.9 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        global function modify_nested_struct(s.6) { 
            copy.7 = s.6
            tmp.12 = sign_extend 10
            s.6[24] = tmp.12
            tmp.13 = s.6[24]
            s.6[8] = 200
            tmp.14 = s.6[8]
            tmp.15 = s.6[16]
            *tmp.15 = 10D
            tmp.16 = *tmp.15
            tmp.17 = s.6[16]
            tmp.18 = add_ptr(tmp.17, index=8L, scale=1)
            *tmp.18 = 11
            tmp.19 = *tmp.18
            tmp.20 = s.6[8]
            tmp.21 = tmp.20 != 200
            if tmp.21 jump or_true_8
            tmp.24 = s.6[0]
            tmp.25 = tmp.24 != 4D
            if tmp.25 jump or_true_8
            tmp.23 = 0
            jump or_end_9
        
          or_true_8:
            tmp.23 = 1
        
          or_end_9:
            if tmp.23 jump or_true_10
            tmp.28 = s.6[24]
            tmp.30 = sign_extend 10
            tmp.29 = tmp.28 != tmp.30
            if tmp.29 jump or_true_10
            tmp.27 = 0
            jump or_end_11
        
          or_true_10:
            tmp.27 = 1
        
          or_end_11:
            if tmp.27 jump or_true_12
            tmp.33 = s.6[16]
            tmp.34 = *tmp.33
            tmp.35 = tmp.34 != 10D
            if tmp.35 jump or_true_12
            tmp.32 = 0
            jump or_end_13
        
          or_true_12:
            tmp.32 = 1
        
          or_end_13:
            if tmp.32 jump or_true_14
            tmp.38 = s.6[16]
            tmp.39 = add_ptr(tmp.38, index=8L, scale=1)
            tmp.40 = *tmp.39
            tmp.41 = tmp.40 != 11
            if tmp.41 jump or_true_14
            tmp.37 = 0
            jump or_end_15
        
          or_true_14:
            tmp.37 = 1
        
          or_end_15:
            if !tmp.37 jump end_if_16
            return 0
        
          end_if_16:
            tmp.42 = copy.7[8]
            tmp.43 = tmp.42 != 5
            if tmp.43 jump or_true_18
            tmp.46 = copy.7[0]
            tmp.47 = tmp.46 != 4D
            if tmp.47 jump or_true_18
            tmp.45 = 0
            jump or_end_19
        
          or_true_18:
            tmp.45 = 1
        
          or_end_19:
            if tmp.45 jump or_true_20
            tmp.50 = copy.7[24]
            tmp.52 = sign_extend 1000
            tmp.51 = tmp.50 != tmp.52
            if tmp.51 jump or_true_20
            tmp.49 = 0
            jump or_end_21
        
          or_true_20:
            tmp.49 = 1
        
          or_end_21:
            if tmp.49 jump or_true_22
            tmp.55 = copy.7[16]
            tmp.56 = *tmp.55
            tmp.57 = tmp.56 != 10D
            if tmp.57 jump or_true_22
            tmp.54 = 0
            jump or_end_23
        
          or_true_22:
            tmp.54 = 1
        
          or_end_23:
            if tmp.54 jump or_true_24
            tmp.60 = copy.7[16]
            tmp.61 = add_ptr(tmp.60, index=8L, scale=1)
            tmp.62 = *tmp.61
            tmp.63 = tmp.62 != 11
            if tmp.63 jump or_true_24
            tmp.59 = 0
            jump or_end_25
        
          or_true_24:
            tmp.59 = 1
        
          or_end_25:
            if !tmp.59 jump end_if_26
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
            s_inner.4[0] = 2D
            s_inner.4[8] = 3
            tmp.0 = modify_simple_struct(s_inner.4)
            tmp.1 = ! tmp.0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = s_inner.4[0]
            tmp.3 = tmp.2 != 2D
            if tmp.3 jump or_true_2
            tmp.6 = s_inner.4[8]
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
            s_o.5[0] = 4D
            s_o.5[8] = 5
            tmp.8 = &s_inner.4
            s_o.5[16] = tmp.8
            s_o.5[24] = 1000L
            tmp.9 = modify_nested_struct(s_o.5)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_6
            return 3
        
          end_if_6:
            tmp.11 = s_o.5[0]
            tmp.12 = tmp.11 != 4D
            if tmp.12 jump or_true_8
            tmp.15 = s_o.5[8]
            tmp.16 = tmp.15 != 5
            if tmp.16 jump or_true_8
            tmp.14 = 0
            jump or_end_9
        
          or_true_8:
            tmp.14 = 1
        
          or_end_9:
            if tmp.14 jump or_true_10
            tmp.19 = s_o.5[24]
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
            tmp.21 = s_o.5[16]
            tmp.23 = &s_inner.4
            tmp.22 = tmp.21 != tmp.23
            if !tmp.22 jump end_if_14
            return 5
        
          end_if_14:
            tmp.24 = s_o.5[16]
            tmp.25 = *tmp.24
            tmp.26 = tmp.25 != 10D
            if tmp.26 jump or_true_16
            tmp.29 = s_o.5[16]
            tmp.30 = add_ptr(tmp.29, index=8L, scale=1)
            tmp.31 = *tmp.30
            tmp.32 = tmp.31 != 11
            if tmp.32 jump or_true_16
            tmp.28 = 0
            jump or_end_17
        
          or_true_16:
            tmp.28 = 1
        
          or_end_17:
            if !tmp.28 jump end_if_18
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
            tmp.0 = &two_xmm_struct.59
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=8)
            tmp.3 = *tmp.2
            tmp.4 = tmp.3 != 55.5D
            if tmp.4 jump or_true_0
            tmp.7 = &two_xmm_struct.59
            tmp.8 = sign_extend 1
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 != 44.4D
            if tmp.11 jump or_true_0
            tmp.6 = 0
            jump or_end_1
        
          or_true_0:
            tmp.6 = 1
        
          or_end_1:
            if !tmp.6 jump end_if_2
            return 0
        
          end_if_2:
            tmp.12 = int_struct.60[4]
            tmp.13 = sign_extend tmp.12
            tmp.14 = tmp.13 != 99
            if tmp.14 jump or_true_4
            tmp.17 = int_struct.60[0]
            tmp.18 = tmp.17 != 54320
            if tmp.18 jump or_true_4
            tmp.16 = 0
            jump or_end_5
        
          or_true_4:
            tmp.16 = 1
        
          or_end_5:
            if !tmp.16 jump end_if_6
            return 0
        
          end_if_6:
            tmp.19 = xmm_struct.61[0]
            tmp.20 = tmp.19 != 5.125D
            if !tmp.20 jump end_if_8
            return 0
        
          end_if_8:
            tmp.21 = &mixed_struct.62
            tmp.21 = add_ptr(tmp.21, index=8L, scale=1)
            tmp.22 = &string.0
            tmp.23 = strcmp(tmp.21, tmp.22)
            if tmp.23 jump or_true_10
            tmp.26 = mixed_struct.62[0]
            tmp.27 = tmp.26 != 1.234D
            if tmp.27 jump or_true_10
            tmp.25 = 0
            jump or_end_11
        
          or_true_10:
            tmp.25 = 1
        
          or_end_11:
            if !tmp.25 jump end_if_12
            return 0
        
          end_if_12:
            tmp.28 = &int_struct_2.63
            tmp.28 = add_ptr(tmp.28, index=4L, scale=1)
            tmp.29 = &string.1
            tmp.30 = strcmp(tmp.28, tmp.29)
            if tmp.30 jump or_true_14
            tmp.33 = int_struct_2.63[0]
            tmp.34 = tmp.33 != 123
            if tmp.34 jump or_true_14
            tmp.32 = 0
            jump or_end_15
        
          or_true_14:
            tmp.32 = 1
        
          or_end_15:
            if !tmp.32 jump end_if_16
            return 0
        
          end_if_16:
            tmp.35 = another_int_struct.64[0]
            tmp.37 = sign_extend 567890
            tmp.36 = tmp.35 != tmp.37
            if !tmp.36 jump end_if_18
            return 0
        
          end_if_18:
            return 1
            return 0
        }
        global function a_bunch_of_arguments(i0.65, i1.66, i2.67, i3.68, i4.69, param.70, i5.71) { 
            tmp.38 = i0.65 != 0
            if tmp.38 jump or_true_20
            tmp.41 = i1.66 != 1
            if tmp.41 jump or_true_20
            tmp.40 = 0
            jump or_end_21
        
          or_true_20:
            tmp.40 = 1
        
          or_end_21:
            if tmp.40 jump or_true_22
            tmp.44 = i2.67 != 2
            if tmp.44 jump or_true_22
            tmp.43 = 0
            jump or_end_23
        
          or_true_22:
            tmp.43 = 1
        
          or_end_23:
            if tmp.43 jump or_true_24
            tmp.47 = i3.68 != 3
            if tmp.47 jump or_true_24
            tmp.46 = 0
            jump or_end_25
        
          or_true_24:
            tmp.46 = 1
        
          or_end_25:
            if tmp.46 jump or_true_26
            tmp.50 = i4.69 != 4
            if tmp.50 jump or_true_26
            tmp.49 = 0
            jump or_end_27
        
          or_true_26:
            tmp.49 = 1
        
          or_end_27:
            if tmp.49 jump or_true_28
            tmp.53 = i5.71 != 5
            if tmp.53 jump or_true_28
            tmp.52 = 0
            jump or_end_29
        
          or_true_28:
            tmp.52 = 1
        
          or_end_29:
            if !tmp.52 jump end_if_30
            return 0
        
          end_if_30:
            tmp.54 = param.70[0]
            tmp.55 = tmp.54 != 1234567L
            if tmp.55 jump or_true_32
            tmp.58 = param.70[8]
            tmp.59 = tmp.58 != 89101112L
            if tmp.59 jump or_true_32
            tmp.57 = 0
            jump or_end_33
        
          or_true_32:
            tmp.57 = 1
        
          or_end_33:
            if !tmp.57 jump end_if_34
            return 0
        
          end_if_34:
            return 1
            return 0
        }
        global function structs_and_scalars(l.72, d.73, os.74, mem.75, xmm_struct.76) { 
            tmp.61 = sign_extend 10
            tmp.60 = l.72 != tmp.61
            if !tmp.60 jump end_if_36
            return 0
        
          end_if_36:
            tmp.62 = d.73 != 10D
            if !tmp.62 jump end_if_38
            return 0
        
          end_if_38:
            tmp.63 = &os.74
            tmp.64 = &string.2
            tmp.65 = strcmp(tmp.63, tmp.64)
            if !tmp.65 jump end_if_40
            return 0
        
          end_if_40:
            tmp.66 = &mem.75
            tmp.66 = add_ptr(tmp.66, index=8L, scale=1)
            tmp.67 = &string.3
            tmp.68 = strcmp(tmp.66, tmp.67)
            if tmp.68 jump or_true_42
            tmp.71 = mem.75[0]
            tmp.72 = tmp.71 != 15.75D
            if tmp.72 jump or_true_42
            tmp.70 = 0
            jump or_end_43
        
          or_true_42:
            tmp.70 = 1
        
          or_end_43:
            if tmp.70 jump or_true_44
            tmp.75 = mem.75[24]
            tmp.76 = tmp.75 != 3333
            if tmp.76 jump or_true_44
            tmp.74 = 0
            jump or_end_45
        
          or_true_44:
            tmp.74 = 1
        
          or_end_45:
            if tmp.74 jump or_true_46
            tmp.79 = mem.75[16]
            tmp.81 = sign_extend 4444
            tmp.80 = tmp.79 != tmp.81
            if tmp.80 jump or_true_46
            tmp.78 = 0
            jump or_end_47
        
          or_true_46:
            tmp.78 = 1
        
          or_end_47:
            if !tmp.78 jump end_if_48
            return 0
        
          end_if_48:
            tmp.82 = xmm_struct.76[0]
            tmp.83 = tmp.82 != 5.125D
            if !tmp.83 jump end_if_50
            return 0
        
          end_if_50:
            return 1
            return 0
        }
        global function struct_in_mem(a.77, b.78, c.79, first_struct.80, d.81, second_struct.82, l.83, third_struct.84, fourth_struct.85) { 
            tmp.84 = a.77 != 10D
            if tmp.84 jump or_true_52
            tmp.87 = b.78 != 11.125D
            if tmp.87 jump or_true_52
            tmp.86 = 0
            jump or_end_53
        
          or_true_52:
            tmp.86 = 1
        
          or_end_53:
            if tmp.86 jump or_true_54
            tmp.90 = c.79 != 12D
            if tmp.90 jump or_true_54
            tmp.89 = 0
            jump or_end_55
        
          or_true_54:
            tmp.89 = 1
        
          or_end_55:
            if !tmp.89 jump end_if_56
            return 0
        
          end_if_56:
            tmp.91 = &first_struct.80
            tmp.91 = add_ptr(tmp.91, index=8L, scale=1)
            tmp.92 = &string.0
            tmp.93 = strcmp(tmp.91, tmp.92)
            if tmp.93 jump or_true_58
            tmp.96 = first_struct.80[0]
            tmp.97 = tmp.96 != 1.234D
            if tmp.97 jump or_true_58
            tmp.95 = 0
            jump or_end_59
        
          or_true_58:
            tmp.95 = 1
        
          or_end_59:
            if !tmp.95 jump end_if_60
            return 0
        
          end_if_60:
            tmp.98 = d.81 != 13D
            if !tmp.98 jump end_if_62
            return 0
        
          end_if_62:
            tmp.99 = &second_struct.82
            tmp.100 = sign_extend 0
            tmp.101 = add_ptr(tmp.99, index=tmp.100, scale=8)
            tmp.102 = *tmp.101
            tmp.103 = tmp.102 != 55.5D
            if tmp.103 jump or_true_64
            tmp.106 = &second_struct.82
            tmp.107 = sign_extend 1
            tmp.108 = add_ptr(tmp.106, index=tmp.107, scale=8)
            tmp.109 = *tmp.108
            tmp.110 = tmp.109 != 44.4D
            if tmp.110 jump or_true_64
            tmp.105 = 0
            jump or_end_65
        
          or_true_64:
            tmp.105 = 1
        
          or_end_65:
            if !tmp.105 jump end_if_66
            return 0
        
          end_if_66:
            if !l.83 jump end_if_68
            return 0
        
          end_if_68:
            tmp.111 = third_struct.84[0]
            tmp.112 = sign_extend tmp.111
            tmp.113 = tmp.112 != 112
            if tmp.113 jump or_true_70
            tmp.116 = third_struct.84[8]
            tmp.117 = tmp.116 != 4.56D
            if tmp.117 jump or_true_70
            tmp.115 = 0
            jump or_end_71
        
          or_true_70:
            tmp.115 = 1
        
          or_end_71:
            if !tmp.115 jump end_if_72
            return 0
        
          end_if_72:
            tmp.118 = fourth_struct.85[0]
            tmp.119 = tmp.118 != 5.125D
            if !tmp.119 jump end_if_74
            return 0
        
          end_if_74:
            return 1
            return 0
        }
        global function pass_borderline_struct_in_memory(t_i.86, c.87, i_x.88, ptr.89, t_i_n.90, d.91) { 
            tmp.120 = t_i.86[0]
            tmp.121 = sign_extend tmp.120
            tmp.122 = tmp.121 != 95
            if tmp.122 jump or_true_76
            tmp.125 = &t_i.86
            tmp.125 = add_ptr(tmp.125, index=4L, scale=1)
            tmp.126 = sign_extend 0
            tmp.127 = add_ptr(tmp.125, index=tmp.126, scale=4)
            tmp.128 = *tmp.127
            tmp.129 = tmp.128 != 5
            if tmp.129 jump or_true_76
            tmp.124 = 0
            jump or_end_77
        
          or_true_76:
            tmp.124 = 1
        
          or_end_77:
            if tmp.124 jump or_true_78
            tmp.132 = &t_i.86
            tmp.132 = add_ptr(tmp.132, index=4L, scale=1)
            tmp.133 = sign_extend 1
            tmp.134 = add_ptr(tmp.132, index=tmp.133, scale=4)
            tmp.135 = *tmp.134
            tmp.136 = tmp.135 != 6
            if tmp.136 jump or_true_78
            tmp.131 = 0
            jump or_end_79
        
          or_true_78:
            tmp.131 = 1
        
          or_end_79:
            if tmp.131 jump or_true_80
            tmp.139 = &t_i.86
            tmp.139 = add_ptr(tmp.139, index=4L, scale=1)
            tmp.140 = sign_extend 2
            tmp.141 = add_ptr(tmp.139, index=tmp.140, scale=4)
            tmp.142 = *tmp.141
            tmp.143 = tmp.142 != 7
            if tmp.143 jump or_true_80
            tmp.138 = 0
            jump or_end_81
        
          or_true_80:
            tmp.138 = 1
        
          or_end_81:
            if !tmp.138 jump end_if_82
            return 0
        
          end_if_82:
            tmp.144 = c.87 != 33
            if !tmp.144 jump end_if_84
            return 0
        
          end_if_84:
            tmp.145 = i_x.88[0]
            tmp.146 = sign_extend tmp.145
            tmp.147 = tmp.146 != 112
            if tmp.147 jump or_true_86
            tmp.150 = i_x.88[8]
            tmp.151 = tmp.150 != 4.56D
            if tmp.151 jump or_true_86
            tmp.149 = 0
            jump or_end_87
        
          or_true_86:
            tmp.149 = 1
        
          or_end_87:
            if !tmp.149 jump end_if_88
            return 0
        
          end_if_88:
            if !ptr.89 jump end_if_90
            return 0
        
          end_if_90:
            tmp.152 = t_i_n.90[4]
            tmp.153 = sign_extend tmp.152
            tmp.154 = tmp.153 != 99
            if tmp.154 jump or_true_92
            tmp.157 = t_i_n.90[0]
            tmp.158 = tmp.157 != 54320
            if tmp.158 jump or_true_92
            tmp.156 = 0
            jump or_end_93
        
          or_true_92:
            tmp.156 = 1
        
          or_end_93:
            if !tmp.156 jump end_if_94
            return 0
        
          end_if_94:
            tmp.159 = t_i_n.90[12]
            tmp.160 = sign_extend tmp.159
            tmp.161 = tmp.160 != 99
            if tmp.161 jump or_true_96
            tmp.164 = t_i_n.90[8]
            tmp.165 = tmp.164 != 54320
            if tmp.165 jump or_true_96
            tmp.163 = 0
            jump or_end_97
        
          or_true_96:
            tmp.163 = 1
        
          or_end_97:
            if !tmp.163 jump end_if_98
            return 0
        
          end_if_98:
            tmp.166 = d.91 != 7.8D
            if !tmp.166 jump end_if_100
            return 0
        
          end_if_100:
            return 1
            return 0
        }
        global function pass_uneven_struct_in_mem(struct1.92, a.93, b.94, struct2.95, os.96, m.97) { 
            tmp.167 = struct1.92[0]
            tmp.169 = - 1
            tmp.168 = tmp.167 != tmp.169
            if !tmp.168 jump end_if_102
            return 0
        
          end_if_102:
            tmp.170 = &struct1.92
            tmp.170 = add_ptr(tmp.170, index=4L, scale=1)
            tmp.171 = sign_extend 0
            tmp.172 = add_ptr(tmp.170, index=tmp.171, scale=1)
            tmp.173 = *tmp.172
            tmp.174 = sign_extend tmp.173
            tmp.175 = tmp.174 != 127
            if tmp.175 jump or_true_104
            tmp.178 = &struct1.92
            tmp.178 = add_ptr(tmp.178, index=4L, scale=1)
            tmp.179 = sign_extend 1
            tmp.180 = add_ptr(tmp.178, index=tmp.179, scale=1)
            tmp.181 = *tmp.180
            tmp.182 = sign_extend tmp.181
            tmp.183 = tmp.182 != 126
            if tmp.183 jump or_true_104
            tmp.177 = 0
            jump or_end_105
        
          or_true_104:
            tmp.177 = 1
        
          or_end_105:
            if tmp.177 jump or_true_106
            tmp.186 = &struct1.92
            tmp.186 = add_ptr(tmp.186, index=4L, scale=1)
            tmp.187 = sign_extend 2
            tmp.188 = add_ptr(tmp.186, index=tmp.187, scale=1)
            tmp.189 = *tmp.188
            tmp.190 = sign_extend tmp.189
            tmp.191 = tmp.190 != 125
            if tmp.191 jump or_true_106
            tmp.185 = 0
            jump or_end_107
        
          or_true_106:
            tmp.185 = 1
        
          or_end_107:
            if !tmp.185 jump end_if_108
            return 0
        
          end_if_108:
            tmp.192 = a.93 != 9223372036854775805L
            if tmp.192 jump or_true_110
            tmp.195 = b.94 != 9223372036854775800L
            if tmp.195 jump or_true_110
            tmp.194 = 0
            jump or_end_111
        
          or_true_110:
            tmp.194 = 1
        
          or_end_111:
            if !tmp.194 jump end_if_112
            return 0
        
          end_if_112:
            tmp.196 = struct2.95[0]
            tmp.198 = - 5
            tmp.197 = tmp.196 != tmp.198
            if !tmp.197 jump end_if_114
            return 0
        
          end_if_114:
            tmp.199 = &struct2.95
            tmp.199 = add_ptr(tmp.199, index=4L, scale=1)
            tmp.200 = sign_extend 0
            tmp.201 = add_ptr(tmp.199, index=tmp.200, scale=1)
            tmp.202 = *tmp.201
            tmp.203 = sign_extend tmp.202
            tmp.204 = tmp.203 != 100
            if tmp.204 jump or_true_116
            tmp.207 = &struct2.95
            tmp.207 = add_ptr(tmp.207, index=4L, scale=1)
            tmp.208 = sign_extend 1
            tmp.209 = add_ptr(tmp.207, index=tmp.208, scale=1)
            tmp.210 = *tmp.209
            tmp.211 = sign_extend tmp.210
            tmp.212 = tmp.211 != 101
            if tmp.212 jump or_true_116
            tmp.206 = 0
            jump or_end_117
        
          or_true_116:
            tmp.206 = 1
        
          or_end_117:
            if tmp.206 jump or_true_118
            tmp.215 = &struct2.95
            tmp.215 = add_ptr(tmp.215, index=4L, scale=1)
            tmp.216 = sign_extend 2
            tmp.217 = add_ptr(tmp.215, index=tmp.216, scale=1)
            tmp.218 = *tmp.217
            tmp.219 = sign_extend tmp.218
            tmp.220 = tmp.219 != 102
            if tmp.220 jump or_true_118
            tmp.214 = 0
            jump or_end_119
        
          or_true_118:
            tmp.214 = 1
        
          or_end_119:
            if !tmp.214 jump end_if_120
            return 0
        
          end_if_120:
            i.98 = 0
        
          start_loop_0:
            tmp.221 = i.98 < 5
            if !tmp.221 jump break_loop_0
            tmp.222 = &os.96
            tmp.223 = sign_extend i.98
            tmp.224 = add_ptr(tmp.222, index=tmp.223, scale=1)
            tmp.225 = *tmp.224
            tmp.226 = sign_extend tmp.225
            tmp.228 = 100 - i.98
            tmp.227 = tmp.226 != tmp.228
            if !tmp.227 jump end_if_122
            return 0
        
          end_if_122:
        
          continue_loop_0:
            tmp.229 = i.98 + 1
            i.98 = tmp.229
            jump start_loop_0
        
          break_loop_0:
            tmp.230 = m.97[0]
            tmp.231 = tmp.230 != 5.345D
            if !tmp.231 jump end_if_124
            return 0
        
          end_if_124:
            tmp.232 = &m.97
            tmp.232 = add_ptr(tmp.232, index=8L, scale=1)
            tmp.233 = sign_extend 0
            tmp.234 = add_ptr(tmp.232, index=tmp.233, scale=1)
            tmp.235 = *tmp.234
            tmp.236 = sign_extend tmp.235
            tmp.238 = - 1
            tmp.237 = tmp.236 != tmp.238
            if tmp.237 jump or_true_126
            tmp.241 = &m.97
            tmp.241 = add_ptr(tmp.241, index=8L, scale=1)
            tmp.242 = sign_extend 1
            tmp.243 = add_ptr(tmp.241, index=tmp.242, scale=1)
            tmp.244 = *tmp.243
            tmp.245 = sign_extend tmp.244
            tmp.247 = - 2
            tmp.246 = tmp.245 != tmp.247
            if tmp.246 jump or_true_126
            tmp.240 = 0
            jump or_end_127
        
          or_true_126:
            tmp.240 = 1
        
          or_end_127:
            if tmp.240 jump or_true_128
            tmp.250 = &m.97
            tmp.250 = add_ptr(tmp.250, index=8L, scale=1)
            tmp.251 = sign_extend 2
            tmp.252 = add_ptr(tmp.250, index=tmp.251, scale=1)
            tmp.253 = *tmp.252
            tmp.254 = sign_extend tmp.253
            tmp.256 = - 3
            tmp.255 = tmp.254 != tmp.256
            if tmp.255 jump or_true_128
            tmp.249 = 0
            jump or_end_129
        
          or_true_128:
            tmp.249 = 1
        
          or_end_129:
            if !tmp.249 jump end_if_130
            return 0
        
          end_if_130:
            tmp.257 = m.97[16]
            tmp.258 = tmp.257 != 4294967300L
            if !tmp.258 jump end_if_132
            return 0
        
          end_if_132:
            tmp.259 = m.97[24]
            tmp.260 = tmp.259 != 10000
            if !tmp.260 jump end_if_134
            return 0
        
          end_if_134:
            return 1
            return 0
        }
        global function pass_later_structs_in_regs(m.99, struct1.100, struct2.101) { 
            tmp.261 = m.99[0]
            tmp.262 = tmp.261 != 5.345D
            if !tmp.262 jump end_if_136
            return 0
        
          end_if_136:
            tmp.263 = &m.99
            tmp.263 = add_ptr(tmp.263, index=8L, scale=1)
            tmp.264 = sign_extend 0
            tmp.265 = add_ptr(tmp.263, index=tmp.264, scale=1)
            tmp.266 = *tmp.265
            tmp.267 = sign_extend tmp.266
            tmp.269 = - 1
            tmp.268 = tmp.267 != tmp.269
            if tmp.268 jump or_true_138
            tmp.272 = &m.99
            tmp.272 = add_ptr(tmp.272, index=8L, scale=1)
            tmp.273 = sign_extend 1
            tmp.274 = add_ptr(tmp.272, index=tmp.273, scale=1)
            tmp.275 = *tmp.274
            tmp.276 = sign_extend tmp.275
            tmp.278 = - 2
            tmp.277 = tmp.276 != tmp.278
            if tmp.277 jump or_true_138
            tmp.271 = 0
            jump or_end_139
        
          or_true_138:
            tmp.271 = 1
        
          or_end_139:
            if tmp.271 jump or_true_140
            tmp.281 = &m.99
            tmp.281 = add_ptr(tmp.281, index=8L, scale=1)
            tmp.282 = sign_extend 2
            tmp.283 = add_ptr(tmp.281, index=tmp.282, scale=1)
            tmp.284 = *tmp.283
            tmp.285 = sign_extend tmp.284
            tmp.287 = - 3
            tmp.286 = tmp.285 != tmp.287
            if tmp.286 jump or_true_140
            tmp.280 = 0
            jump or_end_141
        
          or_true_140:
            tmp.280 = 1
        
          or_end_141:
            if !tmp.280 jump end_if_142
            return 0
        
          end_if_142:
            tmp.288 = m.99[16]
            tmp.289 = tmp.288 != 4294967300L
            if !tmp.289 jump end_if_144
            return 0
        
          end_if_144:
            tmp.290 = m.99[24]
            tmp.291 = tmp.290 != 10000
            if !tmp.291 jump end_if_146
            return 0
        
          end_if_146:
            tmp.292 = struct1.100[0]
            tmp.294 = - 1
            tmp.293 = tmp.292 != tmp.294
            if !tmp.293 jump end_if_148
            return 0
        
          end_if_148:
            tmp.295 = &struct1.100
            tmp.295 = add_ptr(tmp.295, index=4L, scale=1)
            tmp.296 = sign_extend 0
            tmp.297 = add_ptr(tmp.295, index=tmp.296, scale=1)
            tmp.298 = *tmp.297
            tmp.299 = sign_extend tmp.298
            tmp.300 = tmp.299 != 127
            if tmp.300 jump or_true_150
            tmp.303 = &struct1.100
            tmp.303 = add_ptr(tmp.303, index=4L, scale=1)
            tmp.304 = sign_extend 1
            tmp.305 = add_ptr(tmp.303, index=tmp.304, scale=1)
            tmp.306 = *tmp.305
            tmp.307 = sign_extend tmp.306
            tmp.308 = tmp.307 != 126
            if tmp.308 jump or_true_150
            tmp.302 = 0
            jump or_end_151
        
          or_true_150:
            tmp.302 = 1
        
          or_end_151:
            if tmp.302 jump or_true_152
            tmp.311 = &struct1.100
            tmp.311 = add_ptr(tmp.311, index=4L, scale=1)
            tmp.312 = sign_extend 2
            tmp.313 = add_ptr(tmp.311, index=tmp.312, scale=1)
            tmp.314 = *tmp.313
            tmp.315 = sign_extend tmp.314
            tmp.316 = tmp.315 != 125
            if tmp.316 jump or_true_152
            tmp.310 = 0
            jump or_end_153
        
          or_true_152:
            tmp.310 = 1
        
          or_end_153:
            if !tmp.310 jump end_if_154
            return 0
        
          end_if_154:
            tmp.317 = struct2.101[0]
            tmp.318 = tmp.317 != 5.125D
            if !tmp.318 jump end_if_156
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
            two_longs.59[0] = 1234567L
            two_longs.59[8] = 89101112L
            one_int.60[0] = 54320
            tmp.0 = truncate 99
            one_int.60[4] = tmp.0
            tmp.1 = 567890L
            one_long.61[0] = tmp.1
            tmp.2 = truncate 95
            two_ints.62[0] = tmp.2
            two_ints.62[4] = 5
            two_ints.62[8] = 6
            two_ints.62[12] = 7
            two_ints_nested.63[0] = one_int.60
            two_ints_nested.63[8] = one_int.60
            xii.64[0] = 123
            xii.64[4] = 's'
            xii.64[5] = 't'
            xii.64[6] = 'r'
            xii.64[7] = 'i'
            xii.64[8] = 'n'
            xii.64[9] = 'g'
            xii.64[10] = '!'
            xii.64[11] = '\0'
            one_xmm.65[0] = 5.125D
            two_xmm.66[0] = 55.5D
            two_xmm.66[8] = 44.4D
            tmp.3 = truncate 112
            int_and_xmm.67[0] = tmp.3
            int_and_xmm.67[8] = 4.56D
            xmm_and_int.68[0] = 1.234D
            xmm_and_int.68[8] = 'h'
            xmm_and_int.68[9] = 'i'
            xmm_and_int.68[10] = '\0'
            odd.69[0] = 'l'
            odd.69[1] = 'm'
            odd.69[2] = 'n'
            odd.69[3] = 'o'
            odd.69[4] = '\0'
            mem.70[0] = 15.75D
            mem.70[8] = 'r'
            mem.70[9] = 's'
            mem.70[10] = '\0'
            tmp.4 = sign_extend 4444
            mem.70[16] = tmp.4
            mem.70[24] = 3333
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
            struct1.71[0] = tmp.18
            tmp.19 = truncate 127
            struct1.71[4] = tmp.19
            tmp.20 = truncate 126
            struct1.71[5] = tmp.20
            tmp.21 = truncate 125
            struct1.71[6] = tmp.21
            struct1.71[7] = '\0'
            struct1.71[8] = '\0'
            struct1.71[9] = '\0'
            struct1.71[10] = '\0'
            struct1.71[11] = '\0'
            tmp.22 = - 5
            struct2.72[0] = tmp.22
            tmp.23 = truncate 100
            struct2.72[4] = tmp.23
            tmp.24 = truncate 101
            struct2.72[5] = tmp.24
            tmp.25 = truncate 102
            struct2.72[6] = tmp.25
            struct2.72[7] = '\0'
            struct2.72[8] = '\0'
            struct2.72[9] = '\0'
            struct2.72[10] = '\0'
            struct2.72[11] = '\0'
            tmp.26 = truncate 100
            os.73[0] = tmp.26
            tmp.27 = truncate 99
            os.73[1] = tmp.27
            tmp.28 = truncate 98
            os.73[2] = tmp.28
            tmp.29 = truncate 97
            os.73[3] = tmp.29
            tmp.30 = truncate 96
            os.73[4] = tmp.30
            m.74[0] = 5.345D
            tmp.31 = - 1
            tmp.32 = truncate tmp.31
            m.74[8] = tmp.32
            tmp.33 = - 2
            tmp.34 = truncate tmp.33
            m.74[9] = tmp.34
            tmp.35 = - 3
            tmp.36 = truncate tmp.35
            m.74[10] = tmp.36
            m.74[16] = 4294967300L
            m.74[24] = 10000
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
            tmp.4 = p.2[4]
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
            arg.2[0] = 1
            arg.2[4] = 2
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
            tmp.0 = &globvar_1
            tmp.1 = &globvar_2
            tmp.2 = &globvar_3
            tmp.3 = &globvar_4
            tmp.4 = &globvar_5
            tmp.5 = &globvar_6
            tmp.6 = &globvar_7
            tmp.7 = &globvar_8
            tmp.8 = &globvar_9
            tmp.9 = &globvar_10
            tmp.10 = &globvar_11
            tmp.11 = &globvar_12
            tmp.12 = &globvar_13
            tmp.13 = &globvar_14
            tmp.14 = &globvar_15
            tmp.15 = &globvar_16
            tmp.16 = &globvar_17
            tmp.17 = &globvar_18
            tmp.18 = &globvar_19
            tmp.19 = &globvar_20
            tmp.20 = &globvar_21
            tmp.21 = &globvar_22
            tmp.22 = &globvar_23
            tmp.23 = &globvar_24
            tmp.24 = fun0(globvar_1, globvar_2, globvar_3, globvar_4, globvar_5, globvar_6, globvar_7, globvar_8, globvar_9, globvar_10, globvar_11, globvar_12, globvar_13, globvar_14, globvar_15, globvar_16, globvar_17, globvar_18, globvar_19, globvar_20, globvar_21, globvar_22, globvar_23, globvar_24, tmp.0, tmp.1, tmp.2, tmp.3, tmp.4, tmp.5, tmp.6, tmp.7, tmp.8, tmp.9, tmp.10, tmp.11, tmp.12, tmp.13, tmp.14, tmp.15, tmp.16, tmp.17, tmp.18, tmp.19, tmp.20, tmp.21, tmp.22, tmp.23)
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_0
            return 1
        
          end_if_0:
            tmp.26 = &globvar_7
            tmp.27 = &globvar_8
            tmp.28 = &globvar_9
            tmp.29 = &globvar_10
            tmp.30 = &globvar_1
            tmp.31 = &globvar_2
            tmp.32 = &globvar_3
            tmp.33 = &globvar_4
            tmp.34 = &globvar_5
            tmp.35 = &globvar_6
            tmp.36 = fun1(globvar_7, globvar_8, globvar_9, globvar_10, globvar_1, globvar_2, globvar_3, globvar_4, globvar_5, globvar_6, tmp.26, tmp.27, tmp.28, tmp.29, tmp.30, tmp.31, tmp.32, tmp.33, tmp.34, tmp.35)
            tmp.37 = ! tmp.36
            if !tmp.37 jump end_if_2
            return 2
        
          end_if_2:
            tmp.38 = &globvar_11
            tmp.39 = &globvar_12
            tmp.40 = &globvar_13
            tmp.41 = &globvar_1
            tmp.42 = fun2(globvar_11, globvar_12, globvar_13, globvar_1, tmp.38, tmp.39, tmp.40, tmp.41)
            tmp.43 = ! tmp.42
            if !tmp.43 jump end_if_4
            return 3
        
          end_if_4:
            tmp.44 = &globvar_14
            tmp.45 = &globvar_15
            tmp.46 = &globvar_16
            tmp.47 = &globvar_2
            tmp.48 = fun3(globvar_14, globvar_15, globvar_16, globvar_2, tmp.44, tmp.45, tmp.46, tmp.47)
            tmp.49 = ! tmp.48
            if !tmp.49 jump end_if_6
            return 4
        
          end_if_6:
            tmp.50 = truncate 0
            locvar_1.108[0] = tmp.50
            tmp.51 = truncate 1
            locvar_2.109[0] = tmp.51
            tmp.52 = truncate 2
            locvar_2.109[1] = tmp.52
            tmp.53 = truncate 3
            locvar_3.110[0] = tmp.53
            tmp.54 = truncate 4
            locvar_3.110[1] = tmp.54
            tmp.55 = truncate 5
            locvar_3.110[2] = tmp.55
            tmp.56 = truncate 6
            locvar_4.111[0] = tmp.56
            tmp.57 = truncate 7
            locvar_4.111[1] = tmp.57
            tmp.58 = truncate 8
            locvar_4.111[2] = tmp.58
            tmp.59 = truncate 9
            locvar_4.111[3] = tmp.59
            tmp.60 = truncate 10
            locvar_5.112[0] = tmp.60
            tmp.61 = truncate 11
            locvar_5.112[1] = tmp.61
            tmp.62 = truncate 12
            locvar_5.112[2] = tmp.62
            tmp.63 = truncate 13
            locvar_5.112[3] = tmp.63
            tmp.64 = truncate 14
            locvar_5.112[4] = tmp.64
            tmp.65 = truncate 15
            locvar_6.113[0] = tmp.65
            tmp.66 = truncate 16
            locvar_6.113[1] = tmp.66
            tmp.67 = truncate 17
            locvar_6.113[2] = tmp.67
            tmp.68 = truncate 18
            locvar_6.113[3] = tmp.68
            tmp.69 = truncate 19
            locvar_6.113[4] = tmp.69
            tmp.70 = truncate 20
            locvar_6.113[5] = tmp.70
            tmp.71 = truncate 21
            locvar_7.114[0] = tmp.71
            tmp.72 = truncate 22
            locvar_7.114[1] = tmp.72
            tmp.73 = truncate 23
            locvar_7.114[2] = tmp.73
            tmp.74 = truncate 24
            locvar_7.114[3] = tmp.74
            tmp.75 = truncate 25
            locvar_7.114[4] = tmp.75
            tmp.76 = truncate 26
            locvar_7.114[5] = tmp.76
            tmp.77 = truncate 27
            locvar_7.114[6] = tmp.77
            tmp.78 = truncate 28
            locvar_8.115[0] = tmp.78
            tmp.79 = truncate 29
            locvar_8.115[1] = tmp.79
            tmp.80 = truncate 30
            locvar_8.115[2] = tmp.80
            tmp.81 = truncate 31
            locvar_8.115[3] = tmp.81
            tmp.82 = truncate 32
            locvar_8.115[4] = tmp.82
            tmp.83 = truncate 33
            locvar_8.115[5] = tmp.83
            tmp.84 = truncate 34
            locvar_8.115[6] = tmp.84
            tmp.85 = truncate 35
            locvar_8.115[7] = tmp.85
            tmp.86 = truncate 36
            locvar_9.116[0] = tmp.86
            tmp.87 = truncate 37
            locvar_9.116[1] = tmp.87
            tmp.88 = truncate 38
            locvar_9.116[2] = tmp.88
            tmp.89 = truncate 39
            locvar_9.116[3] = tmp.89
            tmp.90 = truncate 40
            locvar_9.116[4] = tmp.90
            tmp.91 = truncate 41
            locvar_9.116[5] = tmp.91
            tmp.92 = truncate 42
            locvar_9.116[6] = tmp.92
            tmp.93 = truncate 43
            locvar_9.116[7] = tmp.93
            tmp.94 = truncate 44
            locvar_9.116[8] = tmp.94
            tmp.95 = truncate 45
            locvar_10.117[0] = tmp.95
            tmp.96 = truncate 46
            locvar_10.117[1] = tmp.96
            tmp.97 = truncate 47
            locvar_10.117[2] = tmp.97
            tmp.98 = truncate 48
            locvar_10.117[3] = tmp.98
            tmp.99 = truncate 49
            locvar_10.117[4] = tmp.99
            tmp.100 = truncate 50
            locvar_10.117[5] = tmp.100
            tmp.101 = truncate 51
            locvar_10.117[6] = tmp.101
            tmp.102 = truncate 52
            locvar_10.117[7] = tmp.102
            tmp.103 = truncate 53
            locvar_10.117[8] = tmp.103
            tmp.104 = truncate 54
            locvar_10.117[9] = tmp.104
            tmp.105 = truncate 55
            locvar_11.118[0] = tmp.105
            tmp.106 = truncate 56
            locvar_11.118[1] = tmp.106
            tmp.107 = truncate 57
            locvar_11.118[2] = tmp.107
            tmp.108 = truncate 58
            locvar_11.118[3] = tmp.108
            tmp.109 = truncate 59
            locvar_11.118[4] = tmp.109
            tmp.110 = truncate 60
            locvar_11.118[5] = tmp.110
            tmp.111 = truncate 61
            locvar_11.118[6] = tmp.111
            tmp.112 = truncate 62
            locvar_11.118[7] = tmp.112
            tmp.113 = truncate 63
            locvar_11.118[8] = tmp.113
            tmp.114 = truncate 64
            locvar_11.118[9] = tmp.114
            tmp.115 = truncate 65
            locvar_11.118[10] = tmp.115
            tmp.116 = truncate 66
            locvar_12.119[0] = tmp.116
            tmp.117 = truncate 67
            locvar_12.119[1] = tmp.117
            tmp.118 = truncate 68
            locvar_12.119[2] = tmp.118
            tmp.119 = truncate 69
            locvar_12.119[3] = tmp.119
            tmp.120 = truncate 70
            locvar_12.119[4] = tmp.120
            tmp.121 = truncate 71
            locvar_12.119[5] = tmp.121
            tmp.122 = truncate 72
            locvar_12.119[6] = tmp.122
            tmp.123 = truncate 73
            locvar_12.119[7] = tmp.123
            tmp.124 = truncate 74
            locvar_12.119[8] = tmp.124
            tmp.125 = truncate 75
            locvar_12.119[9] = tmp.125
            tmp.126 = truncate 76
            locvar_12.119[10] = tmp.126
            tmp.127 = truncate 77
            locvar_12.119[11] = tmp.127
            tmp.128 = truncate 78
            locvar_13.120[0] = tmp.128
            tmp.129 = truncate 79
            locvar_13.120[1] = tmp.129
            tmp.130 = truncate 80
            locvar_13.120[2] = tmp.130
            tmp.131 = truncate 81
            locvar_13.120[3] = tmp.131
            tmp.132 = truncate 82
            locvar_13.120[4] = tmp.132
            tmp.133 = truncate 83
            locvar_13.120[5] = tmp.133
            tmp.134 = truncate 84
            locvar_13.120[6] = tmp.134
            tmp.135 = truncate 85
            locvar_13.120[7] = tmp.135
            tmp.136 = truncate 86
            locvar_13.120[8] = tmp.136
            tmp.137 = truncate 87
            locvar_13.120[9] = tmp.137
            tmp.138 = truncate 88
            locvar_13.120[10] = tmp.138
            tmp.139 = truncate 89
            locvar_13.120[11] = tmp.139
            tmp.140 = truncate 90
            locvar_13.120[12] = tmp.140
            tmp.141 = truncate 91
            locvar_14.121[0] = tmp.141
            tmp.142 = truncate 92
            locvar_14.121[1] = tmp.142
            tmp.143 = truncate 93
            locvar_14.121[2] = tmp.143
            tmp.144 = truncate 94
            locvar_14.121[3] = tmp.144
            tmp.145 = truncate 95
            locvar_14.121[4] = tmp.145
            tmp.146 = truncate 96
            locvar_14.121[5] = tmp.146
            tmp.147 = truncate 97
            locvar_14.121[6] = tmp.147
            tmp.148 = truncate 98
            locvar_14.121[7] = tmp.148
            tmp.149 = truncate 99
            locvar_14.121[8] = tmp.149
            tmp.150 = truncate 100
            locvar_14.121[9] = tmp.150
            tmp.151 = truncate 101
            locvar_14.121[10] = tmp.151
            tmp.152 = truncate 102
            locvar_14.121[11] = tmp.152
            tmp.153 = truncate 103
            locvar_14.121[12] = tmp.153
            tmp.154 = truncate 104
            locvar_14.121[13] = tmp.154
            tmp.155 = truncate 105
            locvar_15.122[0] = tmp.155
            tmp.156 = truncate 106
            locvar_15.122[1] = tmp.156
            tmp.157 = truncate 107
            locvar_15.122[2] = tmp.157
            tmp.158 = truncate 108
            locvar_15.122[3] = tmp.158
            tmp.159 = truncate 109
            locvar_15.122[4] = tmp.159
            tmp.160 = truncate 110
            locvar_15.122[5] = tmp.160
            tmp.161 = truncate 111
            locvar_15.122[6] = tmp.161
            tmp.162 = truncate 112
            locvar_15.122[7] = tmp.162
            tmp.163 = truncate 113
            locvar_15.122[8] = tmp.163
            tmp.164 = truncate 114
            locvar_15.122[9] = tmp.164
            tmp.165 = truncate 115
            locvar_15.122[10] = tmp.165
            tmp.166 = truncate 116
            locvar_15.122[11] = tmp.166
            tmp.167 = truncate 117
            locvar_15.122[12] = tmp.167
            tmp.168 = truncate 118
            locvar_15.122[13] = tmp.168
            tmp.169 = truncate 119
            locvar_15.122[14] = tmp.169
            tmp.170 = truncate 120
            locvar_16.123[0] = tmp.170
            tmp.171 = truncate 121
            locvar_16.123[1] = tmp.171
            tmp.172 = truncate 122
            locvar_16.123[2] = tmp.172
            tmp.173 = truncate 123
            locvar_16.123[3] = tmp.173
            tmp.174 = truncate 124
            locvar_16.123[4] = tmp.174
            tmp.175 = truncate 125
            locvar_16.123[5] = tmp.175
            tmp.176 = truncate 126
            locvar_16.123[6] = tmp.176
            tmp.177 = truncate 127
            locvar_16.123[7] = tmp.177
            tmp.178 = truncate 128
            locvar_16.123[8] = tmp.178
            tmp.179 = truncate 129
            locvar_16.123[9] = tmp.179
            tmp.180 = truncate 130
            locvar_16.123[10] = tmp.180
            tmp.181 = truncate 131
            locvar_16.123[11] = tmp.181
            tmp.182 = truncate 132
            locvar_16.123[12] = tmp.182
            tmp.183 = truncate 133
            locvar_16.123[13] = tmp.183
            tmp.184 = truncate 134
            locvar_16.123[14] = tmp.184
            tmp.185 = truncate 135
            locvar_16.123[15] = tmp.185
            tmp.186 = truncate 136
            locvar_17.124[0] = tmp.186
            tmp.187 = truncate 137
            locvar_17.124[1] = tmp.187
            tmp.188 = truncate 138
            locvar_17.124[2] = tmp.188
            tmp.189 = truncate 139
            locvar_17.124[3] = tmp.189
            tmp.190 = truncate 140
            locvar_17.124[4] = tmp.190
            tmp.191 = truncate 141
            locvar_17.124[5] = tmp.191
            tmp.192 = truncate 142
            locvar_17.124[6] = tmp.192
            tmp.193 = truncate 143
            locvar_17.124[7] = tmp.193
            tmp.194 = truncate 144
            locvar_17.124[8] = tmp.194
            tmp.195 = truncate 145
            locvar_17.124[9] = tmp.195
            tmp.196 = truncate 146
            locvar_17.124[10] = tmp.196
            tmp.197 = truncate 147
            locvar_17.124[11] = tmp.197
            tmp.198 = truncate 148
            locvar_17.124[12] = tmp.198
            tmp.199 = truncate 149
            locvar_17.124[13] = tmp.199
            tmp.200 = truncate 150
            locvar_17.124[14] = tmp.200
            tmp.201 = truncate 151
            locvar_17.124[15] = tmp.201
            tmp.202 = truncate 152
            locvar_17.124[16] = tmp.202
            tmp.203 = truncate 153
            locvar_18.125[0] = tmp.203
            tmp.204 = truncate 154
            locvar_18.125[1] = tmp.204
            tmp.205 = truncate 155
            locvar_18.125[2] = tmp.205
            tmp.206 = truncate 156
            locvar_18.125[3] = tmp.206
            tmp.207 = truncate 157
            locvar_18.125[4] = tmp.207
            tmp.208 = truncate 158
            locvar_18.125[5] = tmp.208
            tmp.209 = truncate 159
            locvar_18.125[6] = tmp.209
            tmp.210 = truncate 160
            locvar_18.125[7] = tmp.210
            tmp.211 = truncate 161
            locvar_18.125[8] = tmp.211
            tmp.212 = truncate 162
            locvar_18.125[9] = tmp.212
            tmp.213 = truncate 163
            locvar_18.125[10] = tmp.213
            tmp.214 = truncate 164
            locvar_18.125[11] = tmp.214
            tmp.215 = truncate 165
            locvar_18.125[12] = tmp.215
            tmp.216 = truncate 166
            locvar_18.125[13] = tmp.216
            tmp.217 = truncate 167
            locvar_18.125[14] = tmp.217
            tmp.218 = truncate 168
            locvar_18.125[15] = tmp.218
            tmp.219 = truncate 169
            locvar_18.125[16] = tmp.219
            tmp.220 = truncate 170
            locvar_18.125[17] = tmp.220
            tmp.221 = truncate 171
            locvar_19.126[0] = tmp.221
            tmp.222 = truncate 172
            locvar_19.126[1] = tmp.222
            tmp.223 = truncate 173
            locvar_19.126[2] = tmp.223
            tmp.224 = truncate 174
            locvar_19.126[3] = tmp.224
            tmp.225 = truncate 175
            locvar_19.126[4] = tmp.225
            tmp.226 = truncate 176
            locvar_19.126[5] = tmp.226
            tmp.227 = truncate 177
            locvar_19.126[6] = tmp.227
            tmp.228 = truncate 178
            locvar_19.126[7] = tmp.228
            tmp.229 = truncate 179
            locvar_19.126[8] = tmp.229
            tmp.230 = truncate 180
            locvar_19.126[9] = tmp.230
            tmp.231 = truncate 181
            locvar_19.126[10] = tmp.231
            tmp.232 = truncate 182
            locvar_19.126[11] = tmp.232
            tmp.233 = truncate 183
            locvar_19.126[12] = tmp.233
            tmp.234 = truncate 184
            locvar_19.126[13] = tmp.234
            tmp.235 = truncate 185
            locvar_19.126[14] = tmp.235
            tmp.236 = truncate 186
            locvar_19.126[15] = tmp.236
            tmp.237 = truncate 187
            locvar_19.126[16] = tmp.237
            tmp.238 = truncate 188
            locvar_19.126[17] = tmp.238
            tmp.239 = truncate 189
            locvar_19.126[18] = tmp.239
            tmp.240 = truncate 190
            locvar_20.127[0] = tmp.240
            tmp.241 = truncate 191
            locvar_20.127[1] = tmp.241
            tmp.242 = truncate 192
            locvar_20.127[2] = tmp.242
            tmp.243 = truncate 193
            locvar_20.127[3] = tmp.243
            tmp.244 = truncate 194
            locvar_20.127[4] = tmp.244
            tmp.245 = truncate 195
            locvar_20.127[5] = tmp.245
            tmp.246 = truncate 196
            locvar_20.127[6] = tmp.246
            tmp.247 = truncate 197
            locvar_20.127[7] = tmp.247
            tmp.248 = truncate 198
            locvar_20.127[8] = tmp.248
            tmp.249 = truncate 199
            locvar_20.127[9] = tmp.249
            tmp.250 = truncate 200
            locvar_20.127[10] = tmp.250
            tmp.251 = truncate 201
            locvar_20.127[11] = tmp.251
            tmp.252 = truncate 202
            locvar_20.127[12] = tmp.252
            tmp.253 = truncate 203
            locvar_20.127[13] = tmp.253
            tmp.254 = truncate 204
            locvar_20.127[14] = tmp.254
            tmp.255 = truncate 205
            locvar_20.127[15] = tmp.255
            tmp.256 = truncate 206
            locvar_20.127[16] = tmp.256
            tmp.257 = truncate 207
            locvar_20.127[17] = tmp.257
            tmp.258 = truncate 208
            locvar_20.127[18] = tmp.258
            tmp.259 = truncate 209
            locvar_20.127[19] = tmp.259
            tmp.260 = truncate 210
            locvar_21.128[0] = tmp.260
            tmp.261 = truncate 211
            locvar_21.128[1] = tmp.261
            tmp.262 = truncate 212
            locvar_21.128[2] = tmp.262
            tmp.263 = truncate 213
            locvar_21.128[3] = tmp.263
            tmp.264 = truncate 214
            locvar_21.128[4] = tmp.264
            tmp.265 = truncate 215
            locvar_21.128[5] = tmp.265
            tmp.266 = truncate 216
            locvar_21.128[6] = tmp.266
            tmp.267 = truncate 217
            locvar_21.128[7] = tmp.267
            tmp.268 = truncate 218
            locvar_21.128[8] = tmp.268
            tmp.269 = truncate 219
            locvar_21.128[9] = tmp.269
            tmp.270 = truncate 220
            locvar_21.128[10] = tmp.270
            tmp.271 = truncate 221
            locvar_21.128[11] = tmp.271
            tmp.272 = truncate 222
            locvar_21.128[12] = tmp.272
            tmp.273 = truncate 223
            locvar_21.128[13] = tmp.273
            tmp.274 = truncate 224
            locvar_21.128[14] = tmp.274
            tmp.275 = truncate 225
            locvar_21.128[15] = tmp.275
            tmp.276 = truncate 226
            locvar_21.128[16] = tmp.276
            tmp.277 = truncate 227
            locvar_21.128[17] = tmp.277
            tmp.278 = truncate 228
            locvar_21.128[18] = tmp.278
            tmp.279 = truncate 229
            locvar_21.128[19] = tmp.279
            tmp.280 = truncate 230
            locvar_21.128[20] = tmp.280
            tmp.281 = truncate 231
            locvar_22.129[0] = tmp.281
            tmp.282 = truncate 232
            locvar_22.129[1] = tmp.282
            tmp.283 = truncate 233
            locvar_22.129[2] = tmp.283
            tmp.284 = truncate 234
            locvar_22.129[3] = tmp.284
            tmp.285 = truncate 235
            locvar_22.129[4] = tmp.285
            tmp.286 = truncate 236
            locvar_22.129[5] = tmp.286
            tmp.287 = truncate 237
            locvar_22.129[6] = tmp.287
            tmp.288 = truncate 238
            locvar_22.129[7] = tmp.288
            tmp.289 = truncate 239
            locvar_22.129[8] = tmp.289
            tmp.290 = truncate 240
            locvar_22.129[9] = tmp.290
            tmp.291 = truncate 241
            locvar_22.129[10] = tmp.291
            tmp.292 = truncate 242
            locvar_22.129[11] = tmp.292
            tmp.293 = truncate 243
            locvar_22.129[12] = tmp.293
            tmp.294 = truncate 244
            locvar_22.129[13] = tmp.294
            tmp.295 = truncate 245
            locvar_22.129[14] = tmp.295
            tmp.296 = truncate 246
            locvar_22.129[15] = tmp.296
            tmp.297 = truncate 247
            locvar_22.129[16] = tmp.297
            tmp.298 = truncate 248
            locvar_22.129[17] = tmp.298
            tmp.299 = truncate 249
            locvar_22.129[18] = tmp.299
            tmp.300 = truncate 250
            locvar_22.129[19] = tmp.300
            tmp.301 = truncate 251
            locvar_22.129[20] = tmp.301
            tmp.302 = truncate 252
            locvar_22.129[21] = tmp.302
            tmp.303 = truncate 253
            locvar_23.130[0] = tmp.303
            tmp.304 = truncate 254
            locvar_23.130[1] = tmp.304
            tmp.305 = truncate 255
            locvar_23.130[2] = tmp.305
            tmp.306 = truncate 0
            locvar_23.130[3] = tmp.306
            tmp.307 = truncate 1
            locvar_23.130[4] = tmp.307
            tmp.308 = truncate 2
            locvar_23.130[5] = tmp.308
            tmp.309 = truncate 3
            locvar_23.130[6] = tmp.309
            tmp.310 = truncate 4
            locvar_23.130[7] = tmp.310
            tmp.311 = truncate 5
            locvar_23.130[8] = tmp.311
            tmp.312 = truncate 6
            locvar_23.130[9] = tmp.312
            tmp.313 = truncate 7
            locvar_23.130[10] = tmp.313
            tmp.314 = truncate 8
            locvar_23.130[11] = tmp.314
            tmp.315 = truncate 9
            locvar_23.130[12] = tmp.315
            tmp.316 = truncate 10
            locvar_23.130[13] = tmp.316
            tmp.317 = truncate 11
            locvar_23.130[14] = tmp.317
            tmp.318 = truncate 12
            locvar_23.130[15] = tmp.318
            tmp.319 = truncate 13
            locvar_23.130[16] = tmp.319
            tmp.320 = truncate 14
            locvar_23.130[17] = tmp.320
            tmp.321 = truncate 15
            locvar_23.130[18] = tmp.321
            tmp.322 = truncate 16
            locvar_23.130[19] = tmp.322
            tmp.323 = truncate 17
            locvar_23.130[20] = tmp.323
            tmp.324 = truncate 18
            locvar_23.130[21] = tmp.324
            tmp.325 = truncate 19
            locvar_23.130[22] = tmp.325
            tmp.326 = truncate 20
            locvar_24.131[0] = tmp.326
            tmp.327 = truncate 21
            locvar_24.131[1] = tmp.327
            tmp.328 = truncate 22
            locvar_24.131[2] = tmp.328
            tmp.329 = truncate 23
            locvar_24.131[3] = tmp.329
            tmp.330 = truncate 24
            locvar_24.131[4] = tmp.330
            tmp.331 = truncate 25
            locvar_24.131[5] = tmp.331
            tmp.332 = truncate 26
            locvar_24.131[6] = tmp.332
            tmp.333 = truncate 27
            locvar_24.131[7] = tmp.333
            tmp.334 = truncate 28
            locvar_24.131[8] = tmp.334
            tmp.335 = truncate 29
            locvar_24.131[9] = tmp.335
            tmp.336 = truncate 30
            locvar_24.131[10] = tmp.336
            tmp.337 = truncate 31
            locvar_24.131[11] = tmp.337
            tmp.338 = truncate 32
            locvar_24.131[12] = tmp.338
            tmp.339 = truncate 33
            locvar_24.131[13] = tmp.339
            tmp.340 = truncate 34
            locvar_24.131[14] = tmp.340
            tmp.341 = truncate 35
            locvar_24.131[15] = tmp.341
            tmp.342 = truncate 36
            locvar_24.131[16] = tmp.342
            tmp.343 = truncate 37
            locvar_24.131[17] = tmp.343
            tmp.344 = truncate 38
            locvar_24.131[18] = tmp.344
            tmp.345 = truncate 39
            locvar_24.131[19] = tmp.345
            tmp.346 = truncate 40
            locvar_24.131[20] = tmp.346
            tmp.347 = truncate 41
            locvar_24.131[21] = tmp.347
            tmp.348 = truncate 42
            locvar_24.131[22] = tmp.348
            tmp.349 = truncate 43
            locvar_24.131[23] = tmp.349
            tmp.350 = &locvar_1.108
            tmp.351 = &locvar_2.109
            tmp.352 = &locvar_3.110
            tmp.353 = &locvar_4.111
            tmp.354 = &locvar_5.112
            tmp.355 = &locvar_6.113
            tmp.356 = &locvar_7.114
            tmp.357 = &locvar_8.115
            tmp.358 = &locvar_9.116
            tmp.359 = &locvar_10.117
            tmp.360 = &locvar_11.118
            tmp.361 = &locvar_12.119
            tmp.362 = &locvar_13.120
            tmp.363 = &locvar_14.121
            tmp.364 = &locvar_15.122
            tmp.365 = &locvar_16.123
            tmp.366 = &locvar_17.124
            tmp.367 = &locvar_18.125
            tmp.368 = &locvar_19.126
            tmp.369 = &locvar_20.127
            tmp.370 = &locvar_21.128
            tmp.371 = &locvar_22.129
            tmp.372 = &locvar_23.130
            tmp.373 = &locvar_24.131
            tmp.374 = fun0(locvar_1.108, locvar_2.109, locvar_3.110, locvar_4.111, locvar_5.112, locvar_6.113, locvar_7.114, locvar_8.115, locvar_9.116, locvar_10.117, locvar_11.118, locvar_12.119, locvar_13.120, locvar_14.121, locvar_15.122, locvar_16.123, locvar_17.124, locvar_18.125, locvar_19.126, locvar_20.127, locvar_21.128, locvar_22.129, locvar_23.130, locvar_24.131, tmp.350, tmp.351, tmp.352, tmp.353, tmp.354, tmp.355, tmp.356, tmp.357, tmp.358, tmp.359, tmp.360, tmp.361, tmp.362, tmp.363, tmp.364, tmp.365, tmp.366, tmp.367, tmp.368, tmp.369, tmp.370, tmp.371, tmp.372, tmp.373)
            tmp.375 = ! tmp.374
            if !tmp.375 jump end_if_8
            return 5
        
          end_if_8:
            tmp.376 = &locvar_7.114
            tmp.377 = &locvar_8.115
            tmp.378 = &locvar_9.116
            tmp.379 = &locvar_10.117
            tmp.380 = &locvar_1.108
            tmp.381 = &locvar_2.109
            tmp.382 = &locvar_3.110
            tmp.383 = &locvar_4.111
            tmp.384 = &locvar_5.112
            tmp.385 = &locvar_6.113
            tmp.386 = fun1(locvar_7.114, locvar_8.115, locvar_9.116, locvar_10.117, locvar_1.108, locvar_2.109, locvar_3.110, locvar_4.111, locvar_5.112, locvar_6.113, tmp.376, tmp.377, tmp.378, tmp.379, tmp.380, tmp.381, tmp.382, tmp.383, tmp.384, tmp.385)
            tmp.387 = ! tmp.386
            if !tmp.387 jump end_if_10
            return 6
        
          end_if_10:
            tmp.388 = &locvar_11.118
            tmp.389 = &locvar_12.119
            tmp.390 = &locvar_13.120
            tmp.391 = &locvar_1.108
            tmp.392 = fun2(locvar_11.118, locvar_12.119, locvar_13.120, locvar_1.108, tmp.388, tmp.389, tmp.390, tmp.391)
            tmp.393 = ! tmp.392
            if !tmp.393 jump end_if_12
            return 7
        
          end_if_12:
            tmp.394 = &locvar_14.121
            tmp.395 = &locvar_15.122
            tmp.396 = &locvar_16.123
            tmp.397 = &locvar_2.109
            tmp.398 = fun3(locvar_14.121, locvar_15.122, locvar_16.123, locvar_2.109, tmp.394, tmp.395, tmp.396, tmp.397)
            tmp.399 = ! tmp.398
            if !tmp.399 jump end_if_14
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
            tmp.1 = &in_reg.1
            tmp.2 = sign_extend i.8
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=1)
            tmp.4 = *tmp.3
            in_reg_c.9 = tmp.4
            tmp.5 = &on_stack.7
            tmp.6 = sign_extend i.8
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=1)
            tmp.8 = *tmp.7
            on_stack_c.10 = tmp.8
            tmp.9 = i.8 == 2
            if !tmp.9 jump else_1
            tmp.10 = sign_extend in_reg_c.9
            tmp.11 = tmp.10 != 4
            if tmp.11 jump or_true_2
            tmp.14 = sign_extend on_stack_c.10
            tmp.15 = tmp.14 != 4
            if tmp.15 jump or_true_2
            tmp.13 = 0
            jump or_end_3
        
          or_true_2:
            tmp.13 = 1
        
          or_end_3:
            if !tmp.13 jump end_if_4
            return 1
        
          end_if_4:
            jump end_if_0
        
          else_1:
            tmp.16 = i.8 == 3
            if !tmp.16 jump else_7
            tmp.17 = sign_extend in_reg_c.9
            tmp.18 = tmp.17 != 5
            if tmp.18 jump or_true_8
            tmp.21 = sign_extend on_stack_c.10
            tmp.22 = tmp.21 != 5
            if tmp.22 jump or_true_8
            tmp.20 = 0
            jump or_end_9
        
          or_true_8:
            tmp.20 = 1
        
          or_end_9:
            if !tmp.20 jump end_if_10
            return 2
        
          end_if_10:
            jump end_if_6
        
          else_7:
            tmp.23 = i.8 == 8
            if !tmp.23 jump else_13
            tmp.24 = sign_extend in_reg_c.9
            tmp.25 = tmp.24 != 6
            if tmp.25 jump or_true_14
            tmp.28 = sign_extend on_stack_c.10
            tmp.29 = tmp.28 != 6
            if tmp.29 jump or_true_14
            tmp.27 = 0
            jump or_end_15
        
          or_true_14:
            tmp.27 = 1
        
          or_end_15:
            if !tmp.27 jump end_if_16
            return 3
        
          end_if_16:
            jump end_if_12
        
          else_13:
            if in_reg_c.9 jump or_true_18
            if on_stack_c.10 jump or_true_18
            tmp.31 = 0
            jump or_end_19
        
          or_true_18:
            tmp.31 = 1
        
          or_end_19:
            if !tmp.31 jump end_if_20
            return 4
        
          end_if_20:
        
          end_if_12:
        
          end_if_6:
        
          end_if_0:
        
          continue_loop_0:
            tmp.32 = i.8 + 1
            i.8 = tmp.32
            jump start_loop_0
        
          break_loop_0:
            tmp.33 = a.2 != 101
            if tmp.33 jump or_true_22
            tmp.36 = b.3 != 102
            if tmp.36 jump or_true_22
            tmp.35 = 0
            jump or_end_23
        
          or_true_22:
            tmp.35 = 1
        
          or_end_23:
            if tmp.35 jump or_true_24
            tmp.39 = c.4 != 103
            if tmp.39 jump or_true_24
            tmp.38 = 0
            jump or_end_25
        
          or_true_24:
            tmp.38 = 1
        
          or_end_25:
            if tmp.38 jump or_true_26
            tmp.42 = d.5 != 104
            if tmp.42 jump or_true_26
            tmp.41 = 0
            jump or_end_27
        
          or_true_26:
            tmp.41 = 1
        
          or_end_27:
            if tmp.41 jump or_true_28
            tmp.45 = e.6 != 105
            if tmp.45 jump or_true_28
            tmp.44 = 0
            jump or_end_29
        
          or_true_28:
            tmp.44 = 1
        
          or_end_29:
            if !tmp.44 jump end_if_30
            return 5
        
          end_if_30:
            return 0
            return 0
        }
        global function main() { 
            tmp.46 = &on_page_boundary
            tmp.47 = sign_extend 2
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=1)
            tmp.49 = truncate 4
            *tmp.48 = tmp.49
            tmp.50 = *tmp.48
            tmp.51 = &on_page_boundary
            tmp.52 = sign_extend 3
            tmp.53 = add_ptr(tmp.51, index=tmp.52, scale=1)
            tmp.54 = truncate 5
            *tmp.53 = tmp.54
            tmp.55 = *tmp.53
            tmp.56 = &on_page_boundary
            tmp.57 = sign_extend 8
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=1)
            tmp.59 = truncate 6
            *tmp.58 = tmp.59
            tmp.60 = *tmp.58
            tmp.61 = f(on_page_boundary, 101, 102, 103, 104, 105, on_page_boundary)
            return tmp.61
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
            tmp.4 = p.1[8]
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
            x.2[0] = 1
            x.2[8] = 2D
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
            tmp.0 = &to_validate
            tmp.1 = &string.0
            tmp.2 = strcmp(tmp.0, tmp.1)
            if !tmp.2 jump end_if_0
            exit(code.4)
        
          end_if_0:
            return 
        
            return 0
        }
        global function take_longword(s.6, code.7) { 
            tmp.3 = s.6[0]
            tmp.4 = tmp.3 != 10
            if !tmp.4 jump end_if_2
            exit(code.7)
        
          end_if_2:
            return 
        
            return 0
        }
        global function pass_longword() { 
            bytes.8[0] = 'e'
            bytes.8[1] = 'f'
            bytes.8[2] = 'g'
            bytes.8[3] = 'h'
            bytes.8[4] = 'i'
            bytes.8[5] = 'j'
            bytes.8[6] = 'k'
            bytes.8[7] = 'l'
            bytes.8[8] = 'm'
            bytes.8[9] = 'n'
            bytes.8[10] = 'o'
            bytes.8[11] = 'p'
            bytes.8[12] = 'q'
            bytes.8[13] = 'r'
            bytes.8[14] = 's'
            bytes.8[15] = '\0'
            take_longword(my_var.9, 1)
            to_validate = bytes.8
            validate_stack_bytes(2)
            return 0
            return 0
        }
        global function take_quadword(s.11, code.12) { 
            tmp.5 = s.11[0]
            tmp.7 = sign_extend 10
            tmp.6 = tmp.5 != tmp.7
            if !tmp.6 jump end_if_4
            exit(code.12)
        
          end_if_4:
            return 
        
            return 0
        }
        global function pass_quadword() { 
            bytes.13[0] = 'e'
            bytes.13[1] = 'f'
            bytes.13[2] = 'g'
            bytes.13[3] = 'h'
            bytes.13[4] = 'i'
            bytes.13[5] = 'j'
            bytes.13[6] = 'k'
            bytes.13[7] = 'l'
            bytes.13[8] = 'm'
            bytes.13[9] = 'n'
            bytes.13[10] = 'o'
            bytes.13[11] = 'p'
            bytes.13[12] = 'q'
            bytes.13[13] = 'r'
            bytes.13[14] = 's'
            bytes.13[15] = '\0'
            take_quadword(my_var.14, 3)
            to_validate = bytes.13
            validate_stack_bytes(4)
            return 0
            return 0
        }
        global function take_double(s.16, code.17) { 
            tmp.8 = s.16[0]
            tmp.10 = int_to_double 10
            tmp.9 = tmp.8 != tmp.10
            if !tmp.9 jump end_if_6
            exit(code.17)
        
          end_if_6:
            return 
        
            return 0
        }
        global function pass_double() { 
            bytes.18[0] = 'e'
            bytes.18[1] = 'f'
            bytes.18[2] = 'g'
            bytes.18[3] = 'h'
            bytes.18[4] = 'i'
            bytes.18[5] = 'j'
            bytes.18[6] = 'k'
            bytes.18[7] = 'l'
            bytes.18[8] = 'm'
            bytes.18[9] = 'n'
            bytes.18[10] = 'o'
            bytes.18[11] = 'p'
            bytes.18[12] = 'q'
            bytes.18[13] = 'r'
            bytes.18[14] = 's'
            bytes.18[15] = '\0'
            take_double(my_var.19, 5)
            to_validate = bytes.18
            validate_stack_bytes(6)
            return 0
            return 0
        }
        global function take_twelve_bytes(s.21, code.22) { 
            tmp.11 = &s.21
            tmp.12 = &string.1
            tmp.13 = strcmp(tmp.11, tmp.12)
            if !tmp.13 jump end_if_8
            exit(code.22)
        
          end_if_8:
            return 
        
            return 0
        }
        global function pass_twelve_bytes() { 
            bytes.23[0] = 'e'
            bytes.23[1] = 'f'
            bytes.23[2] = 'g'
            bytes.23[3] = 'h'
            bytes.23[4] = 'i'
            bytes.23[5] = 'j'
            bytes.23[6] = 'k'
            bytes.23[7] = 'l'
            bytes.23[8] = 'm'
            bytes.23[9] = 'n'
            bytes.23[10] = 'o'
            bytes.23[11] = 'p'
            bytes.23[12] = 'q'
            bytes.23[13] = 'r'
            bytes.23[14] = 's'
            bytes.23[15] = '\0'
            take_twelve_bytes(my_var.24, 7)
            to_validate = bytes.23
            validate_stack_bytes(8)
            return 0
            return 0
        }
        global function take_struct_in_mem(s.26, code.27) { 
            tmp.14 = &s.26
            tmp.15 = &string.2
            tmp.16 = strcmp(tmp.14, tmp.15)
            if !tmp.16 jump end_if_10
            exit(code.27)
        
          end_if_10:
            return 
        
            return 0
        }
        global function pass_struct_in_mem() { 
            bytes.28[0] = 'e'
            bytes.28[1] = 'f'
            bytes.28[2] = 'g'
            bytes.28[3] = 'h'
            bytes.28[4] = 'i'
            bytes.28[5] = 'j'
            bytes.28[6] = 'k'
            bytes.28[7] = 'l'
            bytes.28[8] = 'm'
            bytes.28[9] = 'n'
            bytes.28[10] = 'o'
            bytes.28[11] = 'p'
            bytes.28[12] = 'q'
            bytes.28[13] = 'r'
            bytes.28[14] = 's'
            bytes.28[15] = '\0'
            take_struct_in_mem(my_var.29, 9)
            to_validate = bytes.28
            validate_stack_bytes(10)
            return 0
            return 0
        }
        global function take_irregular_struct(s.31, code.32) { 
            tmp.17 = &s.31
            tmp.18 = &string.3
            tmp.19 = strcmp(tmp.17, tmp.18)
            if !tmp.19 jump end_if_12
            exit(code.32)
        
          end_if_12:
            return 
        
            return 0
        }
        global function pass_irregular_struct() { 
            bytes.33[0] = 'e'
            bytes.33[1] = 'f'
            bytes.33[2] = 'g'
            bytes.33[3] = 'h'
            bytes.33[4] = 'i'
            bytes.33[5] = 'j'
            bytes.33[6] = 'k'
            bytes.33[7] = 'l'
            bytes.33[8] = 'm'
            bytes.33[9] = 'n'
            bytes.33[10] = 'o'
            bytes.33[11] = 'p'
            bytes.33[12] = 'q'
            bytes.33[13] = 'r'
            bytes.33[14] = 's'
            bytes.33[15] = '\0'
            take_irregular_struct(my_var.34, 11)
            to_validate = bytes.33
            validate_stack_bytes(12)
            return 0
            return 0
        }
        global function take_irregular_memory_struct(s.36, code.37) { 
            tmp.20 = &s.36
            tmp.21 = &string.4
            tmp.22 = strcmp(tmp.20, tmp.21)
            if !tmp.22 jump end_if_14
            exit(code.37)
        
          end_if_14:
            return 
        
            return 0
        }
        global function pass_irregular_memory_struct() { 
            bytes.38[0] = 'e'
            bytes.38[1] = 'f'
            bytes.38[2] = 'g'
            bytes.38[3] = 'h'
            bytes.38[4] = 'i'
            bytes.38[5] = 'j'
            bytes.38[6] = 'k'
            bytes.38[7] = 'l'
            bytes.38[8] = 'm'
            bytes.38[9] = 'n'
            bytes.38[10] = 'o'
            bytes.38[11] = 'p'
            bytes.38[12] = 'q'
            bytes.38[13] = 'r'
            bytes.38[14] = 's'
            bytes.38[15] = '\0'
            take_irregular_memory_struct(my_var.39, 13)
            to_validate = bytes.38
            validate_stack_bytes(14)
            return 0
            return 0
        }
        global function main() { 
            tmp.23 = pass_longword()
            tmp.24 = pass_quadword()
            tmp.25 = pass_double()
            tmp.26 = pass_twelve_bytes()
            tmp.27 = pass_struct_in_mem()
            tmp.28 = pass_irregular_struct()
            tmp.29 = pass_irregular_memory_struct()
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
            tmp.2 = globl[0]
            return globl
            return 0
        }
        global function return_in_mem() { 
            tmp.3 = globl2[0]
            tmp.5 = int_to_double 2
            tmp.4 = tmp.3 * tmp.5
            globl2[0] = tmp.4
            tmp.6 = globl2[0]
            tmp.7 = globl2[8]
            tmp.8 = tmp.7 * 3
            globl2[8] = tmp.8
            tmp.9 = globl2[8]
            tmp.10 = globl2[16]
            tmp.12 = sign_extend 4
            tmp.11 = tmp.10 * tmp.12
            globl2[16] = tmp.11
            tmp.13 = globl2[16]
            return globl2
            return 0
        }
        global function main() { 
            tmp.14 = return_in_reg()
            tmp.15 = return_in_reg()
            tmp.16 = globl[0]
            tmp.17 = tmp.16 != 2
            if !tmp.17 jump end_if_0
            return 1
        
          end_if_0:
            tmp.18 = return_in_mem()
            tmp.19 = return_in_mem()
            tmp.20 = globl2[0]
            tmp.21 = tmp.20 != 5D
            if tmp.21 jump or_true_2
            tmp.24 = globl2[8]
            tmp.25 = tmp.24 != 18
            if tmp.25 jump or_true_2
            tmp.23 = 0
            jump or_end_3
        
          or_true_2:
            tmp.23 = 1
        
          or_end_3:
            if tmp.23 jump or_true_4
            tmp.28 = globl2[16]
            tmp.30 = sign_extend 4800
            tmp.29 = tmp.28 != tmp.30
            if tmp.29 jump or_true_4
            tmp.27 = 0
            jump or_end_5
        
          or_true_4:
            tmp.27 = 1
        
          or_end_5:
            if !tmp.27 jump end_if_6
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
            i.4[0] = tmp.0
            tmp.1 = sign_extend 102
            i.4[8] = tmp.1
            return i.4
            return 0
        }
        global function return_nested_struct() { 
            tmp.2 = ret.5[8]
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_0
            tmp.4 = sign_extend 1
            tmp.5 = calloc(tmp.4, 16UL)
            tmp.6 = tmp.5
            ret.5[8] = tmp.6
            tmp.7 = ret.5[8]
            tmp.8 = ret.5[8]
            tmp.9 = truncate 12
            *tmp.8 = tmp.9
            tmp.10 = *tmp.8
            tmp.11 = ret.5[8]
            tmp.12 = add_ptr(tmp.11, index=8L, scale=1)
            tmp.13 = sign_extend 13
            *tmp.12 = tmp.13
            tmp.14 = *tmp.12
        
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
            tmp.1 = tmp.0[8]
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
            tmp.10 = tmp.9[16]
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
            tmp.16 = tmp.15[24]
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
            tmp.20 = tmp.19[8]
            tmp.21 = *tmp.20
            tmp.22 = sign_extend tmp.21
            tmp.23 = tmp.22 != 12
            if tmp.23 jump or_true_8
            tmp.26 = return_nested_struct()
            tmp.27 = tmp.26[8]
            tmp.28 = add_ptr(tmp.27, index=8L, scale=1)
            tmp.29 = *tmp.28
            tmp.31 = sign_extend 13
            tmp.30 = tmp.29 != tmp.31
            if tmp.30 jump or_true_8
            tmp.25 = 0
            jump or_end_9
        
          or_true_8:
            tmp.25 = 1
        
          or_end_9:
            if !tmp.25 jump end_if_10
            return 4
        
          end_if_10:
            tmp.32 = return_nested_struct()
            tmp.33 = tmp.32[8]
            tmp.34 = truncate 70
            *tmp.33 = tmp.34
            tmp.35 = *tmp.33
            tmp.36 = return_nested_struct()
            tmp.37 = tmp.36[8]
            tmp.38 = add_ptr(tmp.37, index=8L, scale=1)
            tmp.39 = sign_extend 71
            *tmp.38 = tmp.39
            tmp.40 = *tmp.38
            tmp.41 = return_nested_struct()
            tmp.42 = tmp.41[8]
            tmp.43 = *tmp.42
            tmp.44 = sign_extend tmp.43
            tmp.45 = tmp.44 != 70
            if tmp.45 jump or_true_12
            tmp.48 = return_nested_struct()
            tmp.49 = tmp.48[8]
            tmp.50 = add_ptr(tmp.49, index=8L, scale=1)
            tmp.51 = *tmp.50
            tmp.53 = sign_extend 71
            tmp.52 = tmp.51 != tmp.53
            if tmp.52 jump or_true_12
            tmp.47 = 0
            jump or_end_13
        
          or_true_12:
            tmp.47 = 1
        
          or_end_13:
            if !tmp.47 jump end_if_14
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
            tmp.0 = *i.2
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
            retval.24[0] = 1
            tmp.0 = truncate 2
            retval.24[4] = tmp.0
            return retval.24
            return 0
        }
        global function return_two_int_struct() { 
            retval.25[0] = 10
            retval.25[4] = '1'
            retval.25[5] = '2'
            retval.25[6] = '3'
            retval.25[7] = '4'
            retval.25[8] = '5'
            retval.25[9] = '6'
            retval.25[10] = '7'
            retval.25[11] = '8'
            return retval.25
            return 0
        }
        global function return_double_struct() { 
            retval.26[0] = 100.625D
            return retval.26
            return 0
        }
        global function return_two_double_struct() { 
            retval.27[0] = 8.8D
            retval.27[8] = 7.8D
            return retval.27
            return 0
        }
        global function return_mixed() { 
            retval.28[0] = 10D
            retval.28[8] = 'a'
            retval.28[9] = 'b'
            retval.28[10] = '\0'
            return retval.28
            return 0
        }
        global function return_mixed2() { 
            tmp.1 = truncate 127
            retval.29[0] = tmp.1
            retval.29[8] = 340000000000000000000000000000000000000000000D
            return retval.29
            return 0
        }
        global function return_on_stack() { 
            retval.30[0] = 1.25D
            retval.30[8] = 'x'
            retval.30[9] = 'y'
            retval.30[10] = '\0'
            retval.30[16] = 100L
            retval.30[24] = 44
            return retval.30
            return 0
        }
        global function leaf_call(t_i.31, c.32, d.33) { 
            tmp.2 = t_i.31[0]
            tmp.3 = sign_extend tmp.2
            tmp.4 = tmp.3 != 95
            if tmp.4 jump or_true_0
            tmp.7 = &t_i.31
            tmp.7 = add_ptr(tmp.7, index=4L, scale=1)
            tmp.8 = sign_extend 0
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=4)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 != 5
            if tmp.11 jump or_true_0
            tmp.6 = 0
            jump or_end_1
        
          or_true_0:
            tmp.6 = 1
        
          or_end_1:
            if tmp.6 jump or_true_2
            tmp.14 = &t_i.31
            tmp.14 = add_ptr(tmp.14, index=4L, scale=1)
            tmp.15 = sign_extend 1
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=4)
            tmp.17 = *tmp.16
            tmp.18 = tmp.17 != 6
            if tmp.18 jump or_true_2
            tmp.13 = 0
            jump or_end_3
        
          or_true_2:
            tmp.13 = 1
        
          or_end_3:
            if tmp.13 jump or_true_4
            tmp.21 = &t_i.31
            tmp.21 = add_ptr(tmp.21, index=4L, scale=1)
            tmp.22 = sign_extend 2
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=4)
            tmp.24 = *tmp.23
            tmp.25 = tmp.24 != 7
            if tmp.25 jump or_true_4
            tmp.20 = 0
            jump or_end_5
        
          or_true_4:
            tmp.20 = 1
        
          or_end_5:
            if !tmp.20 jump end_if_6
            return 0
        
          end_if_6:
            tmp.26 = c.32 != 112
            if tmp.26 jump or_true_8
            tmp.29 = d.33 != 4.56D
            if tmp.29 jump or_true_8
            tmp.28 = 0
            jump or_end_9
        
          or_true_8:
            tmp.28 = 1
        
          or_end_9:
            if !tmp.28 jump end_if_10
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
            tmp.30 = int_to_double 0
            retval.43[0] = tmp.30
            tmp.31 = truncate 0
            retval.43[8] = tmp.31
            tmp.32 = truncate 0
            retval.43[9] = tmp.32
            tmp.33 = truncate 0
            retval.43[10] = tmp.33
            tmp.34 = sign_extend 0
            retval.43[16] = tmp.34
            retval.43[24] = 0
            tmp.35 = strct.36[0]
            tmp.36 = sign_extend tmp.35
            tmp.37 = strct.36[8]
            tmp.38 = leaf_call(t_i.38, tmp.36, tmp.37)
            tmp.39 = ! tmp.38
            if !tmp.39 jump end_if_12
            retval.43[24] = 1
            tmp.40 = retval.43[24]
            return retval.43
        
          end_if_12:
            tmp.41 = i.34 != 6
            if tmp.41 jump or_true_14
            tmp.44 = d.35 != 4D
            if tmp.44 jump or_true_14
            tmp.43 = 0
            jump or_end_15
        
          or_true_14:
            tmp.43 = 1
        
          or_end_15:
            if tmp.43 jump or_true_16
            tmp.47 = c.37 != 5
            if tmp.47 jump or_true_16
            tmp.46 = 0
            jump or_end_17
        
          or_true_16:
            tmp.46 = 1
        
          or_end_17:
            if tmp.46 jump or_true_18
            tmp.51 = sign_extend 77
            tmp.50 = l.39 != tmp.51
            if tmp.50 jump or_true_18
            tmp.49 = 0
            jump or_end_19
        
          or_true_18:
            tmp.49 = 1
        
          or_end_19:
            if tmp.49 jump or_true_20
            tmp.54 = c2.41 != 99
            if tmp.54 jump or_true_20
            tmp.53 = 0
            jump or_end_21
        
          or_true_20:
            tmp.53 = 1
        
          or_end_21:
            if !tmp.53 jump end_if_22
            retval.43[24] = 2
            tmp.55 = retval.43[24]
            return retval.43
        
          end_if_22:
            tmp.56 = o_i_e.40[0]
            tmp.58 = sign_extend 567890
            tmp.57 = tmp.56 != tmp.58
            if !tmp.57 jump end_if_24
            retval.43[24] = 3
            tmp.59 = retval.43[24]
            return retval.43
        
          end_if_24:
            tmp.60 = &stackbytes.42
            tmp.61 = &string.0
            tmp.62 = strcmp(tmp.60, tmp.61)
            if !tmp.62 jump end_if_26
            retval.43[24] = 4
            tmp.63 = retval.43[24]
            return retval.43
        
          end_if_26:
            tmp.64 = sign_extend 100
            retval.43[16] = tmp.64
            tmp.65 = retval.43[16]
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
            one_long.24[0] = tmp.0
            tmp.1 = truncate 95
            two_ints.25[0] = tmp.1
            two_ints.25[4] = 5
            two_ints.25[8] = 6
            two_ints.25[12] = 7
            tmp.2 = truncate 112
            int_and_xmm.26[0] = tmp.2
            int_and_xmm.26[8] = 4.56D
            tmp.3 = return_int_struct()
            s1.27 = tmp.3
            tmp.4 = s1.27[0]
            tmp.5 = tmp.4 != 1
            if tmp.5 jump or_true_0
            tmp.8 = s1.27[4]
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
            tmp.16 = &s2.28
            tmp.16 = add_ptr(tmp.16, index=4L, scale=1)
            tmp.17 = &string.0
            tmp.18 = strncmp(tmp.16, tmp.17, 8UL)
            if tmp.18 jump or_true_4
            tmp.15 = 0
            jump or_end_5
        
          or_true_4:
            tmp.15 = 1
        
          or_end_5:
            if !tmp.15 jump end_if_6
            return 2
        
          end_if_6:
            tmp.19 = return_double_struct()
            s3.29 = tmp.19
            tmp.20 = s3.29[0]
            tmp.21 = tmp.20 != 100.625D
            if !tmp.21 jump end_if_8
            return 3
        
          end_if_8:
            tmp.22 = return_two_double_struct()
            s4.30 = tmp.22
            tmp.23 = &s4.30
            tmp.24 = sign_extend 0
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=8)
            tmp.26 = *tmp.25
            tmp.27 = tmp.26 != 8.8D
            if tmp.27 jump or_true_10
            tmp.30 = &s4.30
            tmp.31 = sign_extend 1
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=8)
            tmp.33 = *tmp.32
            tmp.34 = tmp.33 != 7.8D
            if tmp.34 jump or_true_10
            tmp.29 = 0
            jump or_end_11
        
          or_true_10:
            tmp.29 = 1
        
          or_end_11:
            if !tmp.29 jump end_if_12
            return 4
        
          end_if_12:
            tmp.35 = return_mixed()
            s5.31 = tmp.35
            tmp.36 = s5.31[0]
            tmp.37 = tmp.36 != 10D
            if tmp.37 jump or_true_14
            tmp.40 = &s5.31
            tmp.40 = add_ptr(tmp.40, index=8L, scale=1)
            tmp.41 = &string.1
            tmp.42 = strcmp(tmp.40, tmp.41)
            if tmp.42 jump or_true_14
            tmp.39 = 0
            jump or_end_15
        
          or_true_14:
            tmp.39 = 1
        
          or_end_15:
            if !tmp.39 jump end_if_16
            return 5
        
          end_if_16:
            tmp.43 = return_mixed2()
            s6.32 = tmp.43
            tmp.44 = s6.32[0]
            tmp.45 = sign_extend tmp.44
            tmp.46 = tmp.45 != 127
            if tmp.46 jump or_true_18
            tmp.49 = s6.32[8]
            tmp.50 = tmp.49 != 340000000000000000000000000000000000000000000D
            if tmp.50 jump or_true_18
            tmp.48 = 0
            jump or_end_19
        
          or_true_18:
            tmp.48 = 1
        
          or_end_19:
            if !tmp.48 jump end_if_20
            return 6
        
          end_if_20:
            tmp.51 = return_on_stack()
            s7.33 = tmp.51
            tmp.52 = s7.33[0]
            tmp.53 = tmp.52 != 1.25D
            if tmp.53 jump or_true_22
            tmp.56 = &s7.33
            tmp.56 = add_ptr(tmp.56, index=8L, scale=1)
            tmp.57 = &string.2
            tmp.58 = strcmp(tmp.56, tmp.57)
            if tmp.58 jump or_true_22
            tmp.55 = 0
            jump or_end_23
        
          or_true_22:
            tmp.55 = 1
        
          or_end_23:
            if tmp.55 jump or_true_24
            tmp.61 = s7.33[16]
            tmp.62 = tmp.61 != 100L
            if tmp.62 jump or_true_24
            tmp.60 = 0
            jump or_end_25
        
          or_true_24:
            tmp.60 = 1
        
          or_end_25:
            if tmp.60 jump or_true_26
            tmp.65 = s7.33[24]
            tmp.66 = tmp.65 != 44
            if tmp.66 jump or_true_26
            tmp.64 = 0
            jump or_end_27
        
          or_true_26:
            tmp.64 = 1
        
          or_end_27:
            if !tmp.64 jump end_if_28
            return 7
        
          end_if_28:
            tmp.67 = sign_extend 77
            tmp.68 = pass_and_return_regs(6, 4D, int_and_xmm.26, 5, two_ints.25, tmp.67, one_long.24, 99)
            s7.33 = tmp.68
            tmp.69 = s7.33[0]
            if tmp.69 jump or_true_30
            tmp.72 = &s7.33
            tmp.72 = add_ptr(tmp.72, index=8L, scale=1)
            tmp.73 = sign_extend 0
            tmp.74 = add_ptr(tmp.72, index=tmp.73, scale=1)
            tmp.75 = *tmp.74
            if tmp.75 jump or_true_30
            tmp.71 = 0
            jump or_end_31
        
          or_true_30:
            tmp.71 = 1
        
          or_end_31:
            if tmp.71 jump or_true_32
            tmp.78 = &s7.33
            tmp.78 = add_ptr(tmp.78, index=8L, scale=1)
            tmp.79 = sign_extend 1
            tmp.80 = add_ptr(tmp.78, index=tmp.79, scale=1)
            tmp.81 = *tmp.80
            if tmp.81 jump or_true_32
            tmp.77 = 0
            jump or_end_33
        
          or_true_32:
            tmp.77 = 1
        
          or_end_33:
            if tmp.77 jump or_true_34
            tmp.84 = &s7.33
            tmp.84 = add_ptr(tmp.84, index=8L, scale=1)
            tmp.85 = sign_extend 2
            tmp.86 = add_ptr(tmp.84, index=tmp.85, scale=1)
            tmp.87 = *tmp.86
            if tmp.87 jump or_true_34
            tmp.83 = 0
            jump or_end_35
        
          or_true_34:
            tmp.83 = 1
        
          or_end_35:
            if !tmp.83 jump end_if_36
            return 8
        
          end_if_36:
            tmp.88 = s7.33[24]
            if !tmp.88 jump end_if_38
            return 9
        
          end_if_38:
            tmp.89 = s7.33[16]
            tmp.91 = sign_extend 100
            tmp.90 = tmp.89 != tmp.91
            if !tmp.90 jump end_if_40
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
            tmp.0 = &on_page_boundary
            tmp.1 = sign_extend 17
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = truncate 12
            *tmp.2 = tmp.3
            tmp.4 = *tmp.2
            tmp.5 = &on_page_boundary
            tmp.6 = sign_extend 9
            tmp.7 = add_ptr(tmp.5, index=tmp.6, scale=1)
            tmp.8 = - 1
            tmp.9 = truncate tmp.8
            *tmp.7 = tmp.9
            tmp.10 = *tmp.7
            tmp.11 = &on_page_boundary
            tmp.12 = sign_extend 8
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=1)
            tmp.14 = - 2
            tmp.15 = truncate tmp.14
            *tmp.13 = tmp.15
            tmp.16 = *tmp.13
            tmp.17 = &on_page_boundary
            tmp.18 = sign_extend 7
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=1)
            tmp.20 = - 3
            tmp.21 = truncate tmp.20
            *tmp.19 = tmp.21
            tmp.22 = *tmp.19
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
            tmp.25 = &x.1
            tmp.26 = sign_extend i.2
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=1)
            tmp.28 = *tmp.27
            val.3 = tmp.28
            tmp.29 = i.2 == 7
            if !tmp.29 jump else_1
            tmp.30 = sign_extend val.3
            tmp.32 = - 3
            tmp.31 = tmp.30 != tmp.32
            if !tmp.31 jump end_if_2
            return 1
        
          end_if_2:
            jump end_if_0
        
          else_1:
            tmp.33 = i.2 == 8
            if !tmp.33 jump else_5
            tmp.34 = sign_extend val.3
            tmp.36 = - 2
            tmp.35 = tmp.34 != tmp.36
            if !tmp.35 jump end_if_6
            return 2
        
          end_if_6:
            jump end_if_4
        
          else_5:
            tmp.37 = i.2 == 9
            if !tmp.37 jump else_9
            tmp.38 = sign_extend val.3
            tmp.40 = - 1
            tmp.39 = tmp.38 != tmp.40
            if !tmp.39 jump end_if_10
            return 3
        
          end_if_10:
            jump end_if_8
        
          else_9:
            tmp.41 = i.2 == 17
            if !tmp.41 jump else_13
            tmp.42 = sign_extend val.3
            tmp.43 = tmp.42 != 12
            if !tmp.43 jump end_if_14
            return 4
        
          end_if_14:
            jump end_if_12
        
          else_13:
            tmp.44 = &x.1
            tmp.45 = sign_extend i.2
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=1)
            tmp.47 = *tmp.46
            if !tmp.47 jump end_if_16
            return 5
        
          end_if_16:
        
          end_if_12:
        
          end_if_8:
        
          end_if_4:
        
          end_if_0:
        
          continue_loop_0:
            tmp.48 = i.2 + 1
            i.2 = tmp.48
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
            arg.3[0] = 1
            arg.3[4] = 2
            tmp.0 = increment_struct(arg.3)
            val.4 = tmp.0
            tmp.1 = val.4[0]
            tmp.2 = tmp.1 != 2
            if tmp.2 jump or_true_0
            tmp.5 = val.4[4]
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
            tmp.9 = param.5[0]
            tmp.10 = param.5[4]
            tmp.11 = tmp.10 + 1
            param.5[4] = tmp.11
            tmp.12 = param.5[4]
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
            result.1[0] = tmp.0
            tmp.1 = sign_extend 2
            result.1[8] = tmp.1
            tmp.2 = sign_extend 3
            result.1[16] = tmp.2
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
            tmp.5 = globvar[8]
            tmp.6 = tmp.5 != 500L
            if tmp.6 jump or_true_0
            tmp.4 = 0
            jump or_end_1
        
          or_true_0:
            tmp.4 = 1
        
          or_end_1:
            if tmp.4 jump or_true_2
            tmp.9 = globvar[16]
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
            my_struct.2[0] = 10L
            my_struct.2[8] = 9L
            my_struct.2[16] = 8L
            tmp.11 = &my_struct.2
            tmp.12 = overlap_with_pointer(tmp.11)
            my_struct.2 = tmp.12
            tmp.13 = my_struct.2[0]
            tmp.14 = tmp.13 != 20L
            if tmp.14 jump or_true_6
            tmp.17 = my_struct.2[8]
            tmp.18 = tmp.17 != 18L
            if tmp.18 jump or_true_6
            tmp.16 = 0
            jump or_end_7
        
          or_true_6:
            tmp.16 = 1
        
          or_end_7:
            if tmp.16 jump or_true_8
            tmp.21 = my_struct.2[16]
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
            tmp.0 = &on_page_boundary
            tmp.1 = sign_extend 9
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=1)
            tmp.3 = - 1
            tmp.4 = truncate tmp.3
            *tmp.2 = tmp.4
            tmp.5 = *tmp.2
            tmp.6 = &on_page_boundary
            tmp.7 = sign_extend 8
            tmp.8 = add_ptr(tmp.6, index=tmp.7, scale=1)
            tmp.9 = - 2
            tmp.10 = truncate tmp.9
            *tmp.8 = tmp.10
            tmp.11 = *tmp.8
            tmp.12 = &on_page_boundary
            tmp.13 = sign_extend 7
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = - 3
            tmp.16 = truncate tmp.15
            *tmp.14 = tmp.16
            tmp.17 = *tmp.14
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
            tmp.20 = &x.1
            tmp.21 = sign_extend i.2
            tmp.22 = add_ptr(tmp.20, index=tmp.21, scale=1)
            tmp.23 = *tmp.22
            if !tmp.23 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.24 = i.2 + 1
            i.2 = tmp.24
            jump start_loop_0
        
          break_loop_0:
            tmp.25 = &x.1
            tmp.26 = sign_extend 7
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=1)
            tmp.28 = *tmp.27
            tmp.29 = sign_extend tmp.28
            tmp.31 = - 3
            tmp.30 = tmp.29 != tmp.31
            if !tmp.30 jump end_if_2
            return 2
        
          end_if_2:
            tmp.32 = &x.1
            tmp.33 = sign_extend 8
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=1)
            tmp.35 = *tmp.34
            tmp.36 = sign_extend tmp.35
            tmp.38 = - 2
            tmp.37 = tmp.36 != tmp.38
            if !tmp.37 jump end_if_4
            return 2
        
          end_if_4:
            tmp.39 = &x.1
            tmp.40 = sign_extend 9
            tmp.41 = add_ptr(tmp.39, index=tmp.40, scale=1)
            tmp.42 = *tmp.41
            tmp.43 = sign_extend tmp.42
            tmp.45 = - 1
            tmp.44 = tmp.43 != tmp.45
            if !tmp.44 jump end_if_6
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
            retval.3[0] = tmp.2
            tmp.3 = p.2[4]
            tmp.4 = sign_extend tmp.3
            tmp.5 = tmp.4 * 2
            tmp.6 = sign_extend tmp.5
            retval.3[8] = tmp.6
            return retval.3
            return 0
        }
        global function main() { 
            arg.4[0] = 1
            tmp.7 = truncate 4
            arg.4[4] = tmp.7
            tmp.8 = double_members(arg.4)
            result.5 = tmp.8
            tmp.9 = result.5[0]
            tmp.10 = tmp.9 != 2D
            if tmp.10 jump or_true_0
            tmp.13 = result.5[8]
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
            tmp.0 = &to_validate
            tmp.1 = &string.0
            tmp.2 = strcmp(tmp.0, tmp.1)
            if !tmp.2 jump end_if_0
            exit(code.4)
        
          end_if_0:
            return 
        
            return 0
        }
        global function return_int_struct() { 
            tmp.3 = truncate 0
            retval.6[0] = tmp.3
            tmp.4 = truncate 0
            retval.6[1] = tmp.4
            tmp.5 = truncate 0
            retval.6[2] = tmp.5
            tmp.6 = truncate 0
            retval.6[3] = tmp.6
            tmp.7 = truncate 0
            retval.6[4] = tmp.7
            tmp.8 = truncate 0
            retval.6[5] = tmp.8
            tmp.9 = truncate 0
            retval.6[6] = tmp.9
            return retval.6
            return 0
        }
        global function validate_one_int_struct(code.7) { 
            i.8 = 0
        
          start_loop_0:
            tmp.10 = i.8 < 7
            if !tmp.10 jump break_loop_0
            tmp.11 = &one_int_struct
            tmp.12 = sign_extend i.8
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=1)
            tmp.14 = *tmp.13
            if !tmp.14 jump end_if_2
            exit(code.7)
        
          end_if_2:
        
          continue_loop_0:
            tmp.15 = i.8 + 1
            i.8 = tmp.15
            jump start_loop_0
        
          break_loop_0:
            return 0
        }
        global function test_int_struct() { 
            bytes.9[0] = 'e'
            bytes.9[1] = 'f'
            bytes.9[2] = 'g'
            bytes.9[3] = 'h'
            bytes.9[4] = 'i'
            bytes.9[5] = 'j'
            bytes.9[6] = 'k'
            bytes.9[7] = 'l'
            bytes.9[8] = 'm'
            bytes.9[9] = 'n'
            bytes.9[10] = 'o'
            bytes.9[11] = 'p'
            bytes.9[12] = 'q'
            bytes.9[13] = 'r'
            bytes.9[14] = 's'
            bytes.9[15] = '\0'
            tmp.16 = return_int_struct()
            one_int_struct = tmp.16
            to_validate = bytes.9
            validate_stack_bytes(1)
            validate_one_int_struct(2)
            return 0
            return 0
        }
        global function return_two_int_struct() { 
            tmp.17 = truncate 20
            retval.11[0] = tmp.17
            tmp.18 = truncate 21
            retval.11[1] = tmp.18
            tmp.19 = truncate 22
            retval.11[2] = tmp.19
            tmp.20 = truncate 23
            retval.11[3] = tmp.20
            tmp.21 = truncate 24
            retval.11[4] = tmp.21
            tmp.22 = truncate 25
            retval.11[5] = tmp.22
            tmp.23 = truncate 26
            retval.11[6] = tmp.23
            tmp.24 = truncate 27
            retval.11[7] = tmp.24
            tmp.25 = truncate 28
            retval.11[8] = tmp.25
            tmp.26 = truncate 29
            retval.11[9] = tmp.26
            tmp.27 = truncate 30
            retval.11[10] = tmp.27
            tmp.28 = truncate 31
            retval.11[11] = tmp.28
            tmp.29 = truncate 32
            retval.11[12] = tmp.29
            tmp.30 = truncate 33
            retval.11[13] = tmp.30
            tmp.31 = truncate 34
            retval.11[14] = tmp.31
            return retval.11
            return 0
        }
        global function validate_two_int_struct(code.12) { 
            i.13 = 0
        
          start_loop_1:
            tmp.32 = i.13 < 15
            if !tmp.32 jump break_loop_1
            tmp.33 = &two_int_struct
            tmp.34 = sign_extend i.13
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=1)
            tmp.36 = *tmp.35
            tmp.37 = sign_extend tmp.36
            tmp.39 = i.13 + 20
            tmp.38 = tmp.37 != tmp.39
            if !tmp.38 jump end_if_4
            exit(code.12)
        
          end_if_4:
        
          continue_loop_1:
            tmp.40 = i.13 + 1
            i.13 = tmp.40
            jump start_loop_1
        
          break_loop_1:
            return 0
        }
        global function test_two_int_struct() { 
            bytes.14[0] = 'e'
            bytes.14[1] = 'f'
            bytes.14[2] = 'g'
            bytes.14[3] = 'h'
            bytes.14[4] = 'i'
            bytes.14[5] = 'j'
            bytes.14[6] = 'k'
            bytes.14[7] = 'l'
            bytes.14[8] = 'm'
            bytes.14[9] = 'n'
            bytes.14[10] = 'o'
            bytes.14[11] = 'p'
            bytes.14[12] = 'q'
            bytes.14[13] = 'r'
            bytes.14[14] = 's'
            bytes.14[15] = '\0'
            tmp.41 = return_two_int_struct()
            two_int_struct = tmp.41
            to_validate = bytes.14
            validate_stack_bytes(3)
            validate_two_int_struct(4)
            return 0
            return 0
        }
        global function return_one_xmm_struct() { 
            retval.16[0] = 234.5D
            return retval.16
            return 0
        }
        global function validate_one_double_struct(code.17) { 
            tmp.42 = one_double_struct[0]
            tmp.43 = tmp.42 != 234.5D
            if !tmp.43 jump end_if_6
            exit(code.17)
        
          end_if_6:
            return 0
        }
        global function test_one_double_struct() { 
            bytes.18[0] = 'e'
            bytes.18[1] = 'f'
            bytes.18[2] = 'g'
            bytes.18[3] = 'h'
            bytes.18[4] = 'i'
            bytes.18[5] = 'j'
            bytes.18[6] = 'k'
            bytes.18[7] = 'l'
            bytes.18[8] = 'm'
            bytes.18[9] = 'n'
            bytes.18[10] = 'o'
            bytes.18[11] = 'p'
            bytes.18[12] = 'q'
            bytes.18[13] = 'r'
            bytes.18[14] = 's'
            bytes.18[15] = '\0'
            tmp.44 = return_one_xmm_struct()
            one_double_struct = tmp.44
            to_validate = bytes.18
            validate_stack_bytes(5)
            validate_one_double_struct(6)
            return 0
            return 0
        }
        global function return_two_xmm_struct() { 
            retval.20[0] = 234.5D
            retval.20[8] = 678.25D
            return retval.20
            return 0
        }
        global function validate_two_doubles_struct(code.21) { 
            tmp.45 = two_doubles_struct[0]
            tmp.46 = tmp.45 != 234.5D
            if tmp.46 jump or_true_8
            tmp.49 = two_doubles_struct[8]
            tmp.50 = tmp.49 != 678.25D
            if tmp.50 jump or_true_8
            tmp.48 = 0
            jump or_end_9
        
          or_true_8:
            tmp.48 = 1
        
          or_end_9:
            if !tmp.48 jump end_if_10
            exit(code.21)
        
          end_if_10:
            return 0
        }
        global function test_two_doubles_struct() { 
            bytes.22[0] = 'e'
            bytes.22[1] = 'f'
            bytes.22[2] = 'g'
            bytes.22[3] = 'h'
            bytes.22[4] = 'i'
            bytes.22[5] = 'j'
            bytes.22[6] = 'k'
            bytes.22[7] = 'l'
            bytes.22[8] = 'm'
            bytes.22[9] = 'n'
            bytes.22[10] = 'o'
            bytes.22[11] = 'p'
            bytes.22[12] = 'q'
            bytes.22[13] = 'r'
            bytes.22[14] = 's'
            bytes.22[15] = '\0'
            tmp.51 = return_two_xmm_struct()
            two_doubles_struct = tmp.51
            to_validate = bytes.22
            validate_stack_bytes(7)
            validate_two_doubles_struct(8)
            return 0
            return 0
        }
        global function return_mixed_struct() { 
            tmp.52 = truncate 125
            retval.24[0] = tmp.52
            retval.24[8] = 678.25D
            return retval.24
            return 0
        }
        global function validate_mixed_struct(code.25) { 
            tmp.53 = mixed_struct[0]
            tmp.54 = sign_extend tmp.53
            tmp.55 = tmp.54 != 125
            if tmp.55 jump or_true_12
            tmp.58 = mixed_struct[8]
            tmp.59 = tmp.58 != 678.25D
            if tmp.59 jump or_true_12
            tmp.57 = 0
            jump or_end_13
        
          or_true_12:
            tmp.57 = 1
        
          or_end_13:
            if !tmp.57 jump end_if_14
            exit(code.25)
        
          end_if_14:
            return 0
        }
        global function test_mixed_struct() { 
            bytes.26[0] = 'e'
            bytes.26[1] = 'f'
            bytes.26[2] = 'g'
            bytes.26[3] = 'h'
            bytes.26[4] = 'i'
            bytes.26[5] = 'j'
            bytes.26[6] = 'k'
            bytes.26[7] = 'l'
            bytes.26[8] = 'm'
            bytes.26[9] = 'n'
            bytes.26[10] = 'o'
            bytes.26[11] = 'p'
            bytes.26[12] = 'q'
            bytes.26[13] = 'r'
            bytes.26[14] = 's'
            bytes.26[15] = '\0'
            tmp.60 = return_mixed_struct()
            mixed_struct = tmp.60
            to_validate = bytes.26
            validate_stack_bytes(9)
            validate_mixed_struct(10)
            return 0
            return 0
        }
        global function return_stack_struct() { 
            tmp.61 = truncate 90
            retval.28[0] = tmp.61
            tmp.62 = truncate 91
            retval.28[1] = tmp.62
            tmp.63 = truncate 92
            retval.28[2] = tmp.63
            tmp.64 = truncate 93
            retval.28[3] = tmp.64
            tmp.65 = truncate 94
            retval.28[4] = tmp.65
            tmp.66 = truncate 95
            retval.28[5] = tmp.66
            tmp.67 = truncate 96
            retval.28[6] = tmp.67
            tmp.68 = truncate 97
            retval.28[7] = tmp.68
            tmp.69 = truncate 98
            retval.28[8] = tmp.69
            tmp.70 = truncate 99
            retval.28[9] = tmp.70
            tmp.71 = truncate 100
            retval.28[10] = tmp.71
            tmp.72 = truncate 101
            retval.28[11] = tmp.72
            tmp.73 = truncate 102
            retval.28[12] = tmp.73
            tmp.74 = truncate 103
            retval.28[13] = tmp.74
            tmp.75 = truncate 104
            retval.28[14] = tmp.75
            tmp.76 = truncate 105
            retval.28[15] = tmp.76
            tmp.77 = truncate 106
            retval.28[16] = tmp.77
            tmp.78 = truncate 107
            retval.28[17] = tmp.78
            tmp.79 = truncate 108
            retval.28[18] = tmp.79
            tmp.80 = truncate 109
            retval.28[19] = tmp.80
            tmp.81 = truncate 110
            retval.28[20] = tmp.81
            tmp.82 = truncate 111
            retval.28[21] = tmp.82
            tmp.83 = truncate 112
            retval.28[22] = tmp.83
            tmp.84 = truncate 113
            retval.28[23] = tmp.84
            tmp.85 = truncate 114
            retval.28[24] = tmp.85
            tmp.86 = truncate 115
            retval.28[25] = tmp.86
            tmp.87 = truncate 116
            retval.28[26] = tmp.87
            tmp.88 = truncate 117
            retval.28[27] = tmp.88
            return retval.28
            return 0
        }
        global function validate_stack_struct(code.29) { 
            i.30 = 0
        
          start_loop_2:
            tmp.89 = i.30 < 28
            if !tmp.89 jump break_loop_2
            tmp.90 = &stack_struct
            tmp.91 = sign_extend i.30
            tmp.92 = add_ptr(tmp.90, index=tmp.91, scale=1)
            tmp.93 = *tmp.92
            tmp.94 = sign_extend tmp.93
            tmp.96 = i.30 + 90
            tmp.95 = tmp.94 != tmp.96
            if !tmp.95 jump end_if_16
            exit(code.29)
        
          end_if_16:
        
          continue_loop_2:
            tmp.97 = i.30 + 1
            i.30 = tmp.97
            jump start_loop_2
        
          break_loop_2:
            return 0
        }
        global function test_stack_struct() { 
            bytes.31[0] = 'e'
            bytes.31[1] = 'f'
            bytes.31[2] = 'g'
            bytes.31[3] = 'h'
            bytes.31[4] = 'i'
            bytes.31[5] = 'j'
            bytes.31[6] = 'k'
            bytes.31[7] = 'l'
            bytes.31[8] = 'm'
            bytes.31[9] = 'n'
            bytes.31[10] = 'o'
            bytes.31[11] = 'p'
            bytes.31[12] = 'q'
            bytes.31[13] = 'r'
            bytes.31[14] = 's'
            bytes.31[15] = '\0'
            tmp.98 = return_stack_struct()
            stack_struct = tmp.98
            to_validate = bytes.31
            validate_stack_bytes(11)
            validate_stack_struct(12)
            return 0
            return 0
        }
        global function return_irregular_stack_struct() { 
            tmp.99 = truncate 70
            retval.33[0] = tmp.99
            tmp.100 = truncate 71
            retval.33[1] = tmp.100
            tmp.101 = truncate 72
            retval.33[2] = tmp.101
            tmp.102 = truncate 73
            retval.33[3] = tmp.102
            tmp.103 = truncate 74
            retval.33[4] = tmp.103
            tmp.104 = truncate 75
            retval.33[5] = tmp.104
            tmp.105 = truncate 76
            retval.33[6] = tmp.105
            tmp.106 = truncate 77
            retval.33[7] = tmp.106
            tmp.107 = truncate 78
            retval.33[8] = tmp.107
            tmp.108 = truncate 79
            retval.33[9] = tmp.108
            tmp.109 = truncate 80
            retval.33[10] = tmp.109
            tmp.110 = truncate 81
            retval.33[11] = tmp.110
            tmp.111 = truncate 82
            retval.33[12] = tmp.111
            tmp.112 = truncate 83
            retval.33[13] = tmp.112
            tmp.113 = truncate 84
            retval.33[14] = tmp.113
            tmp.114 = truncate 85
            retval.33[15] = tmp.114
            tmp.115 = truncate 86
            retval.33[16] = tmp.115
            tmp.116 = truncate 87
            retval.33[17] = tmp.116
            tmp.117 = truncate 88
            retval.33[18] = tmp.117
            return retval.33
            return 0
        }
        global function validate_irregular_stack_struct(code.34) { 
            i.35 = 0
        
          start_loop_3:
            tmp.118 = i.35 < 19
            if !tmp.118 jump break_loop_3
            tmp.119 = &irregular_stack_struct
            tmp.120 = sign_extend i.35
            tmp.121 = add_ptr(tmp.119, index=tmp.120, scale=1)
            tmp.122 = *tmp.121
            tmp.123 = sign_extend tmp.122
            tmp.125 = i.35 + 70
            tmp.124 = tmp.123 != tmp.125
            if !tmp.124 jump end_if_18
            exit(code.34)
        
          end_if_18:
        
          continue_loop_3:
            tmp.126 = i.35 + 1
            i.35 = tmp.126
            jump start_loop_3
        
          break_loop_3:
            return 0
        }
        global function test_irregular_stack_struct() { 
            bytes.36[0] = 'e'
            bytes.36[1] = 'f'
            bytes.36[2] = 'g'
            bytes.36[3] = 'h'
            bytes.36[4] = 'i'
            bytes.36[5] = 'j'
            bytes.36[6] = 'k'
            bytes.36[7] = 'l'
            bytes.36[8] = 'm'
            bytes.36[9] = 'n'
            bytes.36[10] = 'o'
            bytes.36[11] = 'p'
            bytes.36[12] = 'q'
            bytes.36[13] = 'r'
            bytes.36[14] = 's'
            bytes.36[15] = '\0'
            tmp.127 = return_irregular_stack_struct()
            irregular_stack_struct = tmp.127
            to_validate = bytes.36
            validate_stack_bytes(13)
            validate_irregular_stack_struct(14)
            return 0
            return 0
        }
        global function main() { 
            tmp.128 = test_int_struct()
            tmp.129 = test_two_int_struct()
            tmp.130 = test_one_double_struct()
            tmp.131 = test_two_doubles_struct()
            tmp.132 = test_mixed_struct()
            tmp.133 = test_stack_struct()
            tmp.134 = test_irregular_stack_struct()
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
            retval.1[0] = 1
            retval.1[4] = 2
            retval.1[8] = 3
            return retval.1
            return 0
        }
        global function main() { 
            tmp.0 = f()
            tmp.1 = &tmp.0
            tmp.2 = sign_extend 0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=4)
            tmp.4 = *tmp.3
            i.2 = tmp.4
            tmp.5 = f()
            tmp.6 = &tmp.5
            tmp.7 = sign_extend 1
            tmp.8 = add_ptr(tmp.6, index=tmp.7, scale=4)
            tmp.9 = *tmp.8
            j.3 = tmp.9
            tmp.10 = f()
            tmp.11 = &tmp.10
            tmp.12 = sign_extend 2
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=4)
            tmp.14 = *tmp.13
            k.4 = tmp.14
            tmp.15 = i.2 != 1
            if !tmp.15 jump end_if_0
            return 1
        
          end_if_0:
            tmp.16 = j.3 != 2
            if !tmp.16 jump end_if_2
            return 2
        
          end_if_2:
            tmp.17 = k.4 != 3
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
