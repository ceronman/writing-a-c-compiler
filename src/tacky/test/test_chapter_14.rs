use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_casts_cast_between_pointer_types() {
    let src = r#"
        int check_null_ptr_cast(void) {
            static long *long_ptr = 0;
            double *dbl_ptr = (double *)long_ptr;
            unsigned int *int_ptr = (unsigned int *)long_ptr;
            int **ptr_ptr = (int **)long_ptr;
            if (long_ptr) {
                return 1;
            }
            if (dbl_ptr) {
                return 2;
            }
            if (int_ptr) {
                return 3;
            }
            if (ptr_ptr) {
                return 4;
            }
            return 0;
        }
        int check_round_trip(void) {
            long l = -1;
            long *long_ptr = &l;
            double *dbl_ptr = (double *)long_ptr;
            long *other_long_ptr = (long *)dbl_ptr;
            if (*other_long_ptr != -1) {
                return 5;
            }
            return 0;
        }
        int main(void)
        {
            int result = check_null_ptr_cast();
            if (result) {
                return result;
            }
            result = check_round_trip();
            return result;
        }
    "#;
    let expected = r#"
        global function check_null_ptr_cast() { 
            tmp.0 = long_ptr.0
            dbl_ptr.1 = tmp.0
            tmp.1 = long_ptr.0
            int_ptr.2 = tmp.1
            tmp.2 = long_ptr.0
            ptr_ptr.3 = tmp.2
            if !long_ptr.0 jump end_if_0
            return 1
        
          end_if_0:
            if !dbl_ptr.1 jump end_if_2
            return 2
        
          end_if_2:
            if !int_ptr.2 jump end_if_4
            return 3
        
          end_if_4:
            if !ptr_ptr.3 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        global function check_round_trip() { 
            tmp.3 = - 1
            tmp.4 = sign_extend tmp.3
            l.4 = tmp.4
            tmp.5 = &l.4
            long_ptr.5 = tmp.5
            tmp.6 = long_ptr.5
            dbl_ptr.6 = tmp.6
            tmp.7 = dbl_ptr.6
            other_long_ptr.7 = tmp.7
            tmp.8 = *other_long_ptr.7
            tmp.10 = - 1
            tmp.11 = sign_extend tmp.10
            tmp.9 = tmp.8 != tmp.11
            if !tmp.9 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        global function main() { 
            tmp.12 = check_null_ptr_cast()
            result.8 = tmp.12
            if !result.8 jump end_if_10
            return result.8
        
          end_if_10:
            tmp.13 = check_round_trip()
            result.8 = tmp.13
            return result.8
            return 0
        }
        static long_ptr.0: Pointer(Long) = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_casts_null_pointer_conversion() {
    let src = r#"
        double *d = 0l;
        int *i = 0ul;
        int *i2 = 0u;
        int expect_null_param(int *val)
        {
            return (val == 0ul);
        }
        long *return_null_ptr(void)
        {
            return 0;
        }
        int main(void)
        {
            int x = 10;
            int *ptr = &x;
            if (d) {
                return 1;
            }
            if (i) {
                return 2;
            }
            if (i2) {
                return 3;
            }
            ptr = 0ul;
            if (ptr) {
                return 4;
            }
            int *y = 0;
            if (y != 0)
                return 5;
            if (!expect_null_param(0)) {
                return 6;
            }
            long *null_ptr = return_null_ptr();
            if (null_ptr != 0) {
                return 7;
            }
            ptr = &x;
            int *ternary_result = 10 ? 0 : ptr;
            if (ternary_result) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function expect_null_param(val.0) { 
            tmp.1 = 0UL
            tmp.0 = val.0 == tmp.1
            return tmp.0
            return 0
        }
        global function return_null_ptr() { 
            tmp.2 = sign_extend 0
            return tmp.2
            return 0
        }
        global function main() { 
            x.1 = 10
            tmp.3 = &x.1
            ptr.2 = tmp.3
            if !d jump end_if_0
            return 1
        
          end_if_0:
            if !i jump end_if_2
            return 2
        
          end_if_2:
            if !i2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.4 = 0UL
            ptr.2 = tmp.4
            if !ptr.2 jump end_if_6
            return 4
        
          end_if_6:
            tmp.5 = sign_extend 0
            y.3 = tmp.5
            tmp.7 = sign_extend 0
            tmp.6 = y.3 != tmp.7
            if !tmp.6 jump end_if_8
            return 5
        
          end_if_8:
            tmp.8 = sign_extend 0
            tmp.9 = expect_null_param(tmp.8)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_10
            return 6
        
          end_if_10:
            tmp.11 = return_null_ptr()
            null_ptr.4 = tmp.11
            tmp.13 = sign_extend 0
            tmp.12 = null_ptr.4 != tmp.13
            if !tmp.12 jump end_if_12
            return 7
        
          end_if_12:
            tmp.14 = &x.1
            ptr.2 = tmp.14
            if !10 jump else_15
            tmp.16 = sign_extend 0
            tmp.15 = tmp.16
            jump end_if_14
        
          else_15:
            tmp.15 = ptr.2
        
          end_if_14:
            ternary_result.5 = tmp.15
            if !ternary_result.5 jump end_if_16
            return 8
        
          end_if_16:
            return 0
            return 0
        }
        static global d: Pointer(Double) = 0UL
        static global i: Pointer(Int) = 0UL
        static global i2: Pointer(Int) = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_casts_pointer_int_casts() {
    let src = r#"
        int i = 128;
        long l = 128l;
        int int_to_pointer(void) {
            int *a = (int *) i;
            int *b = (int *) l;
            return a == b;
        }
        int pointer_to_int(void) {
            static long l;
            long *ptr = &l;
            unsigned long ptr_as_long = (unsigned long) ptr;
            return (ptr_as_long % 8 == 0);
        }
        int cast_long_round_trip(void) {
            int *ptr = (int *) l;
            long l2 = (long) ptr;
            return (l == l2);
        }
        int cast_ulong_round_trip(void) {
            long *ptr = &l;
            unsigned long ptr_as_ulong = (unsigned long) ptr;
            long *ptr2 = (long *) ptr_as_ulong;
            return (ptr == ptr2);
        }
        int cast_int_round_trip(void) {
            double *a = (double *)i;
            int i2 = (int) a;
            return (i2 == 128);
        }
        int main(void) {
            if (!int_to_pointer()) {
                return 1;
            }
            if (!pointer_to_int()) {
                return 2;
            }
            if (!cast_long_round_trip()) {
                return 3;
            }
            if (!cast_ulong_round_trip()) {
                return 4;
            }
            if (!cast_int_round_trip()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function int_to_pointer() { 
            tmp.0 = sign_extend i
            a.0 = tmp.0
            tmp.1 = l
            b.1 = tmp.1
            tmp.2 = a.0 == b.1
            return tmp.2
            return 0
        }
        global function pointer_to_int() { 
            tmp.3 = &l.2
            ptr.3 = tmp.3
            tmp.4 = ptr.3
            ptr_as_long.4 = tmp.4
            tmp.6 = sign_extend 8
            tmp.5 = ptr_as_long.4 % tmp.6
            tmp.8 = sign_extend 0
            tmp.7 = tmp.5 == tmp.8
            return tmp.7
            return 0
        }
        global function cast_long_round_trip() { 
            tmp.9 = l
            ptr.5 = tmp.9
            tmp.10 = ptr.5
            l2.6 = tmp.10
            tmp.11 = l == l2.6
            return tmp.11
            return 0
        }
        global function cast_ulong_round_trip() { 
            tmp.12 = &l
            ptr.7 = tmp.12
            tmp.13 = ptr.7
            ptr_as_ulong.8 = tmp.13
            tmp.14 = ptr_as_ulong.8
            ptr2.9 = tmp.14
            tmp.15 = ptr.7 == ptr2.9
            return tmp.15
            return 0
        }
        global function cast_int_round_trip() { 
            tmp.16 = sign_extend i
            a.10 = tmp.16
            tmp.17 = truncate a.10
            i2.11 = tmp.17
            tmp.18 = i2.11 == 128
            return tmp.18
            return 0
        }
        global function main() { 
            tmp.19 = int_to_pointer()
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_0
            return 1
        
          end_if_0:
            tmp.21 = pointer_to_int()
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_2
            return 2
        
          end_if_2:
            tmp.23 = cast_long_round_trip()
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_4
            return 3
        
          end_if_4:
            tmp.25 = cast_ulong_round_trip()
            tmp.26 = ! tmp.25
            if !tmp.26 jump end_if_6
            return 4
        
          end_if_6:
            tmp.27 = cast_int_round_trip()
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static global i: Int = 128
        static global l: Long = 128L
        static l.2: Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_comparisons_compare_pointers() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b;
            int *a_ptr = &a;
            int *a_ptr2 = &a;
            int *b_ptr = &b;
            if (a_ptr == b_ptr) {
                return 1;
            }
            if (a_ptr != a_ptr2) {
                return 2;
            }
            if (!(a_ptr == a_ptr2)) {
                return 3;
            }
            if (!(a_ptr != b_ptr)) {
                return 4;
            }
            *b_ptr = *a_ptr;
            if (a_ptr == b_ptr) {
                return 5;
            }
            b_ptr = a_ptr;
            if (b_ptr != a_ptr) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = &a.0
            a_ptr.2 = tmp.0
            tmp.1 = &a.0
            a_ptr2.3 = tmp.1
            tmp.2 = &b.1
            b_ptr.4 = tmp.2
            tmp.3 = a_ptr.2 == b_ptr.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = a_ptr.2 != a_ptr2.3
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = a_ptr.2 == a_ptr2.3
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = a_ptr.2 != b_ptr.4
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = *a_ptr.2
            *b_ptr.4 = tmp.9
            tmp.10 = a_ptr.2 == b_ptr.4
            if !tmp.10 jump end_if_8
            return 5
        
          end_if_8:
            b_ptr.4 = a_ptr.2
            tmp.11 = b_ptr.4 != a_ptr.2
            if !tmp.11 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_comparisons_compare_to_null() {
    let src = r#"
        double *get_null_pointer(void) {
            return 0;
        }
        int main(void)
        {
            double x;
            double *null = get_null_pointer();
            double *non_null = &x;
            if (non_null == 0) {
                return 1;
            }
            if (!(null == 0l)) {
                return 2;
            }
            if (!(non_null != 0u)) {
                return 3;
            }
            if (null != 0ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_null_pointer() { 
            tmp.0 = sign_extend 0
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = get_null_pointer()
            null.1 = tmp.1
            tmp.2 = &x.0
            non_null.2 = tmp.2
            tmp.4 = sign_extend 0
            tmp.3 = non_null.2 == tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = 0L
            tmp.5 = null.1 == tmp.6
            tmp.7 = ! tmp.5
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = zero_extend 0U
            tmp.8 = non_null.2 != tmp.9
            tmp.10 = ! tmp.8
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.12 = 0UL
            tmp.11 = null.1 != tmp.12
            if !tmp.11 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_comparisons_pointers_as_conditions() {
    let src = r#"
        long *get_null_pointer(void) {
            return 0;
        }
        int main(void)
        {
            long x;
            long *ptr = &x;
            long *null_ptr = get_null_pointer();
            if (5.0 && null_ptr) {
                return 1;
            }
            int a = 0;
            if (!(ptr || (a = 10))) {
                return 2;
            }
            if (a != 0) {
                return 3;
            }
            if (!ptr) {
                return 4;
            }
            int j = ptr ? 1 : 2;
            int k = null_ptr ? 3 : 4;
            if (j != 1) {
                return 5;
            }
            if (k != 4) {
                return 6;
            }
            int i = 0;
            while (ptr)
            {
                if (i >= 10) {
                    ptr = 0;
                    continue;
                }
                i = i + 1;
            }
            if (i != 10) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_null_pointer() { 
            tmp.0 = sign_extend 0
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = &x.0
            ptr.1 = tmp.1
            tmp.2 = get_null_pointer()
            null_ptr.2 = tmp.2
            if !5D jump and_false_0
            if !null_ptr.2 jump and_false_0
            tmp.4 = 1
            jump and_end_1
        
          and_false_0:
            tmp.4 = 0
        
          and_end_1:
            if !tmp.4 jump end_if_2
            return 1
        
          end_if_2:
            a.3 = 0
            if ptr.1 jump or_true_4
            a.3 = 10
            if 10 jump or_true_4
            tmp.6 = 0
            jump or_end_5
        
          or_true_4:
            tmp.6 = 1
        
          or_end_5:
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_6
            return 2
        
          end_if_6:
            tmp.8 = a.3 != 0
            if !tmp.8 jump end_if_8
            return 3
        
          end_if_8:
            tmp.9 = ! ptr.1
            if !tmp.9 jump end_if_10
            return 4
        
          end_if_10:
            if !ptr.1 jump else_13
            tmp.10 = 1
            jump end_if_12
        
          else_13:
            tmp.10 = 2
        
          end_if_12:
            j.4 = tmp.10
            if !null_ptr.2 jump else_15
            tmp.11 = 3
            jump end_if_14
        
          else_15:
            tmp.11 = 4
        
          end_if_14:
            k.5 = tmp.11
            tmp.12 = j.4 != 1
            if !tmp.12 jump end_if_16
            return 5
        
          end_if_16:
            tmp.13 = k.5 != 4
            if !tmp.13 jump end_if_18
            return 6
        
          end_if_18:
            i.6 = 0
        
          continue_loop_0:
            if !ptr.1 jump break_loop_0
            tmp.14 = i.6 >= 10
            if !tmp.14 jump end_if_20
            tmp.15 = sign_extend 0
            ptr.1 = tmp.15
            jump continue_loop_0
        
          end_if_20:
            tmp.16 = i.6 + 1
            i.6 = tmp.16
            jump continue_loop_0
        
          break_loop_0:
            tmp.17 = i.6 != 10
            if !tmp.17 jump end_if_22
            return 7
        
          end_if_22:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_abstract_declarators() {
    let src = r#"
        
        int main(void) {
            long int unsigned *x = 0;
            if (x != (unsigned long (*)) 0)
                return 1;
            if (x != (long unsigned int ((((*))))) 0)
                return 2;
            double ***y = 0;
            if (y != (double *(**)) 0)
                return 3;
            if (y != (double (***)) 0)
                return 4;
            if ((double (*(*(*)))) 0 != y)
                return 5;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 0
            x.0 = tmp.0
            tmp.2 = sign_extend 0
            tmp.1 = x.0 != tmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = sign_extend 0
            tmp.3 = x.0 != tmp.4
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 0
            y.1 = tmp.5
            tmp.7 = sign_extend 0
            tmp.6 = y.1 != tmp.7
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = sign_extend 0
            tmp.8 = y.1 != tmp.9
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.10 = sign_extend 0
            tmp.11 = tmp.10 != y.1
            if !tmp.11 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_declarators() {
    let src = r#"
        int return_3(void);
        int(return_3(void));
        int(return_3)(void);
        int((return_3))(void)
        {
            return 3;
        }
        long l = 100;
        long *two_pointers(double val, double *ptr)
        {
            *ptr = val;
            return &l;
        }
        long(*two_pointers(double val, double(*d)));
        long *(two_pointers)(double val, double *(d));
        long *(two_pointers)(double val, double(*(d)));
        unsigned **pointers_to_pointers(int **p)
        {
            static unsigned u;
            static unsigned *u_ptr;
            u_ptr = &u;
            u = **p;
            return &u_ptr;
        }
        unsigned(**(pointers_to_pointers(int *(*p))));
        unsigned *(*pointers_to_pointers(int(**p)));
        unsigned(*(*((pointers_to_pointers)(int(*(*(p)))))));
        int main(void)
        {
            int i = 0;
            int(*i_ptr) = &i;
            int(**ptr_to_iptr) = &i_ptr;
            double(d1) = 0.0;
            double d2 = 10.0;
            double *(d_ptr) = &d1;
            long(*(l_ptr));
            unsigned *(*(ptr_to_uptr));
            i = return_3();
            if (i != 3)
                return 1;
            if (*i_ptr != 3) {
                return 2;
            }
            l_ptr = two_pointers(d2, d_ptr);
            if (l_ptr != &l) {
                return 3;
            }
            if (*l_ptr != 100) {
                return 4;
            }
            if (*d_ptr != 10.0) {
                return 5;
            }
            if (d1 != 10.0) {
                return 6;
            }
            ptr_to_uptr = pointers_to_pointers(ptr_to_iptr);
            if (**ptr_to_uptr != 3) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_3() { 
            return 3
            return 0
        }
        global function two_pointers(val.0, ptr.1) { 
            *ptr.1 = val.0
            tmp.0 = &l
            return tmp.0
            return 0
        }
        global function pointers_to_pointers(p.8) { 
            tmp.1 = &u.9
            u_ptr.10 = tmp.1
            tmp.2 = *p.8
            tmp.3 = *tmp.2
            tmp.4 = tmp.3
            u.9 = tmp.4
            tmp.5 = &u_ptr.10
            return tmp.5
            return 0
        }
        global function main() { 
            i.14 = 0
            tmp.6 = &i.14
            i_ptr.15 = tmp.6
            tmp.7 = &i_ptr.15
            ptr_to_iptr.16 = tmp.7
            d1.17 = 0D
            d2.18 = 10D
            tmp.8 = &d1.17
            d_ptr.19 = tmp.8
            tmp.9 = return_3()
            i.14 = tmp.9
            tmp.10 = i.14 != 3
            if !tmp.10 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = *i_ptr.15
            tmp.12 = tmp.11 != 3
            if !tmp.12 jump end_if_2
            return 2
        
          end_if_2:
            tmp.13 = two_pointers(d2.18, d_ptr.19)
            l_ptr.20 = tmp.13
            tmp.15 = &l
            tmp.14 = l_ptr.20 != tmp.15
            if !tmp.14 jump end_if_4
            return 3
        
          end_if_4:
            tmp.16 = *l_ptr.20
            tmp.18 = sign_extend 100
            tmp.17 = tmp.16 != tmp.18
            if !tmp.17 jump end_if_6
            return 4
        
          end_if_6:
            tmp.19 = *d_ptr.19
            tmp.20 = tmp.19 != 10D
            if !tmp.20 jump end_if_8
            return 5
        
          end_if_8:
            tmp.21 = d1.17 != 10D
            if !tmp.21 jump end_if_10
            return 6
        
          end_if_10:
            tmp.22 = pointers_to_pointers(ptr_to_iptr.16)
            ptr_to_uptr.21 = tmp.22
            tmp.23 = *ptr_to_uptr.21
            tmp.24 = *tmp.23
            tmp.26 = 3
            tmp.25 = tmp.24 != tmp.26
            if !tmp.25 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
        static global l: Long = 100L
        static u.9: Unsigned Int = zero[4]
        static u_ptr.10: Pointer(Unsigned Int) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_declare_pointer_in_for_loop() {
    let src = r#"
        int main(void) {
            int x = 10;
            for (int *i = &x; i != 0; ) {
                *i = 5;
                i = 0;
            }
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = &x.0
            i.1 = tmp.0
        
          start_loop_0:
            tmp.2 = sign_extend 0
            tmp.1 = i.1 != tmp.2
            if !tmp.1 jump break_loop_0
            *i.1 = 5
            tmp.3 = sign_extend 0
            i.1 = tmp.3
        
          continue_loop_0:
            jump start_loop_0
        
          break_loop_0:
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_address_of_dereference() {
    let src = r#"
        int main(void) {
            int *null_ptr = 0;
            if (&*null_ptr != 0)
                return 1;
            int **ptr_to_null = &null_ptr;
            if (&**ptr_to_null)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 0
            null_ptr.0 = tmp.0
            tmp.2 = sign_extend 0
            tmp.1 = null_ptr.0 != tmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = &null_ptr.0
            ptr_to_null.1 = tmp.3
            tmp.4 = *ptr_to_null.1
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_dereference_expression_result() {
    let src = r#"
        int *return_pointer(void) {
            static int var = 10;
            return &var;
        }
        int one = 1;
        int main(void) {
            int val = 100;
            int *ptr_var = &val;
            if (*return_pointer() != 10) {
                return 1;
            }
            if (*(one ? return_pointer() : ptr_var) != 10)
                return 2;
            if (*(one - 1 ? return_pointer() : ptr_var) != 100) {
                return 3;
            }
            int *ptr_to_one = &one;
            if (*(ptr_var = ptr_to_one) != 1) {
                return 4;
            }
            *return_pointer() = 20;
            *(one ? ptr_var : return_pointer()) = 30;
            if (*return_pointer() != 20) {
                return 5;
            }
            if (*ptr_var != 30) {
                return 6;
            }
            if (one != 30) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_pointer() { 
            tmp.0 = &var.0
            return tmp.0
            return 0
        }
        global function main() { 
            val.1 = 100
            tmp.1 = &val.1
            ptr_var.2 = tmp.1
            tmp.2 = return_pointer()
            tmp.3 = *tmp.2
            tmp.4 = tmp.3 != 10
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            if !one jump else_3
            tmp.6 = return_pointer()
            tmp.5 = tmp.6
            jump end_if_2
        
          else_3:
            tmp.5 = ptr_var.2
        
          end_if_2:
            tmp.7 = *tmp.5
            tmp.8 = tmp.7 != 10
            if !tmp.8 jump end_if_4
            return 2
        
          end_if_4:
            tmp.9 = one - 1
            if !tmp.9 jump else_7
            tmp.11 = return_pointer()
            tmp.10 = tmp.11
            jump end_if_6
        
          else_7:
            tmp.10 = ptr_var.2
        
          end_if_6:
            tmp.12 = *tmp.10
            tmp.13 = tmp.12 != 100
            if !tmp.13 jump end_if_8
            return 3
        
          end_if_8:
            tmp.14 = &one
            ptr_to_one.3 = tmp.14
            ptr_var.2 = ptr_to_one.3
            tmp.15 = *ptr_to_one.3
            tmp.16 = tmp.15 != 1
            if !tmp.16 jump end_if_10
            return 4
        
          end_if_10:
            tmp.17 = return_pointer()
            *tmp.17 = 20
            if !one jump else_13
            tmp.18 = ptr_var.2
            jump end_if_12
        
          else_13:
            tmp.19 = return_pointer()
            tmp.18 = tmp.19
        
          end_if_12:
            *tmp.18 = 30
            tmp.20 = return_pointer()
            tmp.21 = *tmp.20
            tmp.22 = tmp.21 != 20
            if !tmp.22 jump end_if_14
            return 5
        
          end_if_14:
            tmp.23 = *ptr_var.2
            tmp.24 = tmp.23 != 30
            if !tmp.24 jump end_if_16
            return 6
        
          end_if_16:
            tmp.25 = one != 30
            if !tmp.25 jump end_if_18
            return 7
        
          end_if_18:
            return 0
            return 0
        }
        static global one: Int = 1
        static var.0: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_multilevel_indirection() {
    let src = r#"
        
        int main(void) {
            double d = 10.0;
            double *d_ptr = &d;
            double **d_ptr_ptr = &d_ptr;
            double ***d_ptr_ptr_ptr = &d_ptr_ptr;
            if (d != 10.0) {
                return 1;
            }
            if (*d_ptr != 10.0) {
                return 2;
            }
            if (**d_ptr_ptr != 10.0) {
                return 3;
            }
            if (***d_ptr_ptr_ptr != 10.0) {
                return 4;
            }
            if (&d != d_ptr) {
                return 5;
            }
            if (*d_ptr_ptr != d_ptr) {
                return 6;
            }
            if (**d_ptr_ptr_ptr != d_ptr) {
                return 7;
            }
            ***d_ptr_ptr_ptr = 5.0;
            if (d != 5.0) {
                return 8;
            }
            if (*d_ptr != 5.0) {
                return 9;
            }
            if (**d_ptr_ptr != 5.0) {
                return 10;
            }
            if (***d_ptr_ptr_ptr != 5.0) {
                return 11;
            }
            double d2 = 1.0;
            double *d2_ptr = &d2;
            double *d2_ptr2 = d2_ptr;
            double **d2_ptr_ptr = &d2_ptr;
            *d_ptr_ptr_ptr = d2_ptr_ptr;
            if (**d_ptr_ptr_ptr != d2_ptr) {
                return 12;
            }
            if (***d_ptr_ptr_ptr != 1.0) {
                return 13;
            }
            if (d2_ptr_ptr == &d2_ptr2)
                return 14;
            d2_ptr = d_ptr;
            if (**d_ptr_ptr_ptr != d_ptr) {
                return 15;
            }
            if (*d2_ptr_ptr != d_ptr) {
                return 16;
            }
            if (**d_ptr_ptr_ptr == d2_ptr2) {
                return 17;
            }
            if (***d_ptr_ptr_ptr != 5.0) {
                return 18;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            d.0 = 10D
            tmp.0 = &d.0
            d_ptr.1 = tmp.0
            tmp.1 = &d_ptr.1
            d_ptr_ptr.2 = tmp.1
            tmp.2 = &d_ptr_ptr.2
            d_ptr_ptr_ptr.3 = tmp.2
            tmp.3 = d.0 != 10D
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = *d_ptr.1
            tmp.5 = tmp.4 != 10D
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = *d_ptr_ptr.2
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 != 10D
            if !tmp.8 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = *d_ptr_ptr_ptr.3
            tmp.10 = *tmp.9
            tmp.11 = *tmp.10
            tmp.12 = tmp.11 != 10D
            if !tmp.12 jump end_if_6
            return 4
        
          end_if_6:
            tmp.13 = &d.0
            tmp.14 = tmp.13 != d_ptr.1
            if !tmp.14 jump end_if_8
            return 5
        
          end_if_8:
            tmp.15 = *d_ptr_ptr.2
            tmp.16 = tmp.15 != d_ptr.1
            if !tmp.16 jump end_if_10
            return 6
        
          end_if_10:
            tmp.17 = *d_ptr_ptr_ptr.3
            tmp.18 = *tmp.17
            tmp.19 = tmp.18 != d_ptr.1
            if !tmp.19 jump end_if_12
            return 7
        
          end_if_12:
            tmp.20 = *d_ptr_ptr_ptr.3
            tmp.21 = *tmp.20
            *tmp.21 = 5D
            tmp.22 = d.0 != 5D
            if !tmp.22 jump end_if_14
            return 8
        
          end_if_14:
            tmp.23 = *d_ptr.1
            tmp.24 = tmp.23 != 5D
            if !tmp.24 jump end_if_16
            return 9
        
          end_if_16:
            tmp.25 = *d_ptr_ptr.2
            tmp.26 = *tmp.25
            tmp.27 = tmp.26 != 5D
            if !tmp.27 jump end_if_18
            return 10
        
          end_if_18:
            tmp.28 = *d_ptr_ptr_ptr.3
            tmp.29 = *tmp.28
            tmp.30 = *tmp.29
            tmp.31 = tmp.30 != 5D
            if !tmp.31 jump end_if_20
            return 11
        
          end_if_20:
            d2.4 = 1D
            tmp.32 = &d2.4
            d2_ptr.5 = tmp.32
            d2_ptr2.6 = d2_ptr.5
            tmp.33 = &d2_ptr.5
            d2_ptr_ptr.7 = tmp.33
            *d_ptr_ptr_ptr.3 = d2_ptr_ptr.7
            tmp.34 = *d_ptr_ptr_ptr.3
            tmp.35 = *tmp.34
            tmp.36 = tmp.35 != d2_ptr.5
            if !tmp.36 jump end_if_22
            return 12
        
          end_if_22:
            tmp.37 = *d_ptr_ptr_ptr.3
            tmp.38 = *tmp.37
            tmp.39 = *tmp.38
            tmp.40 = tmp.39 != 1D
            if !tmp.40 jump end_if_24
            return 13
        
          end_if_24:
            tmp.42 = &d2_ptr2.6
            tmp.41 = d2_ptr_ptr.7 == tmp.42
            if !tmp.41 jump end_if_26
            return 14
        
          end_if_26:
            d2_ptr.5 = d_ptr.1
            tmp.43 = *d_ptr_ptr_ptr.3
            tmp.44 = *tmp.43
            tmp.45 = tmp.44 != d_ptr.1
            if !tmp.45 jump end_if_28
            return 15
        
          end_if_28:
            tmp.46 = *d2_ptr_ptr.7
            tmp.47 = tmp.46 != d_ptr.1
            if !tmp.47 jump end_if_30
            return 16
        
          end_if_30:
            tmp.48 = *d_ptr_ptr_ptr.3
            tmp.49 = *tmp.48
            tmp.50 = tmp.49 == d2_ptr2.6
            if !tmp.50 jump end_if_32
            return 17
        
          end_if_32:
            tmp.51 = *d_ptr_ptr_ptr.3
            tmp.52 = *tmp.51
            tmp.53 = *tmp.52
            tmp.54 = tmp.53 != 5D
            if !tmp.54 jump end_if_34
            return 18
        
          end_if_34:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_read_through_pointers() {
    let src = r#"
        
        int main(void) {
            int i = -100;
            unsigned long ul = 13835058055282163712ul;
            double d = 3.5;
            int *i_ptr = &i;
            unsigned long *ul_ptr = &ul;
            double *d_ptr = &d;
            if (*i_ptr != -100) {
                return 1;
            }
            if (*ul_ptr != 13835058055282163712ul) {
                return 2;
            }
            if (*d_ptr != 3.5) {
                return 3;
            }
            i = 12;
            ul = 1000;
            d = -000.001;
            if (*i_ptr != 12) {
                return 4;
            }
            if (*ul_ptr != 1000) {
                return 5;
            }
            if (*d_ptr != -000.001) {
                return 6;
            }
            int i2 = 1;
            unsigned long ul2 = 144115196665790464ul;
            double d2 = -33.3;
            i_ptr = &i2;
            ul_ptr = &ul2;
            d_ptr = &d2;
            if (*i_ptr != 1) {
                return 7;
            }
            if (*ul_ptr != 144115196665790464ul) {
                return 8;
            }
            if (*d_ptr != -33.3) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 100
            i.0 = tmp.0
            ul.1 = 13835058055282163712UL
            d.2 = 3.5D
            tmp.1 = &i.0
            i_ptr.3 = tmp.1
            tmp.2 = &ul.1
            ul_ptr.4 = tmp.2
            tmp.3 = &d.2
            d_ptr.5 = tmp.3
            tmp.4 = *i_ptr.3
            tmp.6 = - 100
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = *ul_ptr.4
            tmp.8 = tmp.7 != 13835058055282163712UL
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = *d_ptr.5
            tmp.10 = tmp.9 != 3.5D
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            i.0 = 12
            tmp.11 = sign_extend 1000
            ul.1 = tmp.11
            tmp.12 = - 0.001D
            d.2 = tmp.12
            tmp.13 = *i_ptr.3
            tmp.14 = tmp.13 != 12
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            tmp.15 = *ul_ptr.4
            tmp.17 = sign_extend 1000
            tmp.16 = tmp.15 != tmp.17
            if !tmp.16 jump end_if_8
            return 5
        
          end_if_8:
            tmp.18 = *d_ptr.5
            tmp.20 = - 0.001D
            tmp.19 = tmp.18 != tmp.20
            if !tmp.19 jump end_if_10
            return 6
        
          end_if_10:
            i2.6 = 1
            ul2.7 = 144115196665790464UL
            tmp.21 = - 33.3D
            d2.8 = tmp.21
            tmp.22 = &i2.6
            i_ptr.3 = tmp.22
            tmp.23 = &ul2.7
            ul_ptr.4 = tmp.23
            tmp.24 = &d2.8
            d_ptr.5 = tmp.24
            tmp.25 = *i_ptr.3
            tmp.26 = tmp.25 != 1
            if !tmp.26 jump end_if_12
            return 7
        
          end_if_12:
            tmp.27 = *ul_ptr.4
            tmp.28 = tmp.27 != 144115196665790464UL
            if !tmp.28 jump end_if_14
            return 8
        
          end_if_14:
            tmp.29 = *d_ptr.5
            tmp.31 = - 33.3D
            tmp.30 = tmp.29 != tmp.31
            if !tmp.30 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_simple() {
    let src = r#"
        int main(void) {
            int x = 3;
            int *ptr = &x;
            return *ptr;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 3
            tmp.0 = &x.0
            ptr.1 = tmp.0
            tmp.1 = *ptr.1
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_static_var_indirection() {
    let src = r#"
        unsigned int w = 4294967295U;
        int x = 10;
        unsigned int y = 4294967295U;
        double *dbl_ptr;
        long modify_ptr(long *new_ptr) {
            static long *p;
            if (new_ptr)
            {
                p = new_ptr;
            }
            return *p;
        }
        int increment_ptr(void)
        {
            *dbl_ptr = *dbl_ptr + 5.0;
            return 0;
        }
        int main(void) {
            int *pointer_to_static = &x;
            x = 20;
            if (*pointer_to_static != 20) {
                return 1;
            }
            *pointer_to_static = 100;
            if (x != 100) {
                return 2;
            }
            if (w != 4294967295U) {
                return 3;
            }
            if (y != 4294967295U) {
                return 4;
            }
            if (dbl_ptr) {
                return 5;
            }
            long l = 1000l;
            if (modify_ptr(&l) != 1000l) {
                return 6;
            }
            l = -1;
            if (modify_ptr(0) != l) {
                return 7;
            }
            double d = 10.0;
            dbl_ptr = &d;
            increment_ptr();
            if (*dbl_ptr != 15) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function modify_ptr(new_ptr.0) { 
            if !new_ptr.0 jump end_if_0
            p.1 = new_ptr.0
        
          end_if_0:
            tmp.0 = *p.1
            return tmp.0
            return 0
        }
        global function increment_ptr() { 
            tmp.1 = *dbl_ptr
            tmp.2 = tmp.1 + 5D
            *dbl_ptr = tmp.2
            return 0
            return 0
        }
        global function main() { 
            tmp.3 = &x
            pointer_to_static.2 = tmp.3
            x = 20
            tmp.4 = *pointer_to_static.2
            tmp.5 = tmp.4 != 20
            if !tmp.5 jump end_if_2
            return 1
        
          end_if_2:
            *pointer_to_static.2 = 100
            tmp.6 = x != 100
            if !tmp.6 jump end_if_4
            return 2
        
          end_if_4:
            tmp.7 = w != 4294967295U
            if !tmp.7 jump end_if_6
            return 3
        
          end_if_6:
            tmp.8 = y != 4294967295U
            if !tmp.8 jump end_if_8
            return 4
        
          end_if_8:
            if !dbl_ptr jump end_if_10
            return 5
        
          end_if_10:
            l.3 = 1000L
            tmp.9 = &l.3
            tmp.10 = modify_ptr(tmp.9)
            tmp.11 = tmp.10 != 1000L
            if !tmp.11 jump end_if_12
            return 6
        
          end_if_12:
            tmp.12 = - 1
            tmp.13 = sign_extend tmp.12
            l.3 = tmp.13
            tmp.14 = sign_extend 0
            tmp.15 = modify_ptr(tmp.14)
            tmp.16 = tmp.15 != l.3
            if !tmp.16 jump end_if_14
            return 7
        
          end_if_14:
            d.4 = 10D
            tmp.17 = &d.4
            dbl_ptr = tmp.17
            tmp.18 = increment_ptr()
            tmp.19 = *dbl_ptr
            tmp.21 = int_to_double 15
            tmp.20 = tmp.19 != tmp.21
            if !tmp.20 jump end_if_16
            return 8
        
          end_if_16:
            return 0
            return 0
        }
        static global dbl_ptr: Pointer(Double) = zero[8]
        static p.1: Pointer(Long) = zero[8]
        static global w: Unsigned Int = 4294967295U
        static global x: Int = 10
        static global y: Unsigned Int = 4294967295U
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_dereference_update_through_pointers() {
    let src = r#"
        int main(void) {
            unsigned int i = 2185232384u;
            signed long l = 144115196665790464l;
            double d = 1e50;
            unsigned *i_ptr = &i;
            long *l_ptr = &l;
            double *d_ptr = &d;
            *i_ptr = 10;
            *l_ptr = -20;
            *d_ptr = 30.1;
            if (i != 10) {
                return 1;
            }
            if (l != -20) {
                return 2;
            }
            if (d != 30.1) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 2185232384U
            l.1 = 144115196665790464L
            d.2 = 100000000000000000000000000000000000000000000000000D
            tmp.0 = &i.0
            i_ptr.3 = tmp.0
            tmp.1 = &l.1
            l_ptr.4 = tmp.1
            tmp.2 = &d.2
            d_ptr.5 = tmp.2
            tmp.3 = 10
            *i_ptr.3 = tmp.3
            tmp.4 = - 20
            tmp.5 = sign_extend tmp.4
            *l_ptr.4 = tmp.5
            *d_ptr.5 = 30.1D
            tmp.7 = 10
            tmp.6 = i.0 != tmp.7
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = - 20
            tmp.10 = sign_extend tmp.9
            tmp.8 = l.1 != tmp.10
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = d.2 != 30.1D
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
fn test_valid_extra_credit_bitshift_dereferenced_ptrs() {
    let src = r#"
        unsigned int ui = 4294967295;
        unsigned int *get_ui_ptr(void){
            return &ui;
        }
        int shiftcount = 5;
        int main(void) {
            if ((*get_ui_ptr() << 2l) != 4294967292) {
                return 1;
            }
            if ((*get_ui_ptr() >> 2) != 1073741823) {
                return 2;
            }
            int *shiftcount_ptr = &shiftcount;
            if ((1000000u >> *shiftcount_ptr) != 31250) {
                return 3;
            }
            if ((1000000u << *shiftcount_ptr) != 32000000) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_ui_ptr() { 
            tmp.0 = &ui
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = get_ui_ptr()
            tmp.2 = *tmp.1
            tmp.3 = tmp.2 << 2L
            tmp.4 = zero_extend tmp.3
            tmp.5 = tmp.4 != 4294967292L
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = get_ui_ptr()
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 >> 2
            tmp.10 = 1073741823
            tmp.9 = tmp.8 != tmp.10
            if !tmp.9 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = &shiftcount
            shiftcount_ptr.0 = tmp.11
            tmp.13 = *shiftcount_ptr.0
            tmp.12 = 1000000U >> tmp.13
            tmp.15 = 31250
            tmp.14 = tmp.12 != tmp.15
            if !tmp.14 jump end_if_4
            return 3
        
          end_if_4:
            tmp.17 = *shiftcount_ptr.0
            tmp.16 = 1000000U << tmp.17
            tmp.19 = 32000000
            tmp.18 = tmp.16 != tmp.19
            if !tmp.18 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static global shiftcount: Int = 5
        static global ui: Unsigned Int = 4294967295U
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_ops_with_dereferenced_ptrs() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            unsigned long ul = 9223372036854775808ul;
            unsigned int *ui_ptr = &ui;
            unsigned long *ul_ptr = &ul;
            if ((*ui_ptr & *ul_ptr) != 0) {
                return 1;
            }
            if ((*ui_ptr | *ul_ptr) != 9223372041149743103ul) {
                return 2;
            }
            int i = -1;
            signed int *i_ptr = &i;
            if ((*i_ptr & ul) != *ul_ptr) {
                return 3;
            }
            if ((*i_ptr | *ul_ptr) != i) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1U
            ui.0 = tmp.0
            ul.1 = 9223372036854775808UL
            tmp.1 = &ui.0
            ui_ptr.2 = tmp.1
            tmp.2 = &ul.1
            ul_ptr.3 = tmp.2
            tmp.3 = *ui_ptr.2
            tmp.4 = zero_extend tmp.3
            tmp.6 = *ul_ptr.3
            tmp.5 = tmp.4 & tmp.6
            tmp.8 = sign_extend 0
            tmp.7 = tmp.5 != tmp.8
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = *ui_ptr.2
            tmp.10 = zero_extend tmp.9
            tmp.12 = *ul_ptr.3
            tmp.11 = tmp.10 | tmp.12
            tmp.13 = tmp.11 != 9223372041149743103UL
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            tmp.14 = - 1
            i.4 = tmp.14
            tmp.15 = &i.4
            i_ptr.5 = tmp.15
            tmp.16 = *i_ptr.5
            tmp.17 = sign_extend tmp.16
            tmp.18 = tmp.17 & ul.1
            tmp.20 = *ul_ptr.3
            tmp.19 = tmp.18 != tmp.20
            if !tmp.19 jump end_if_4
            return 3
        
          end_if_4:
            tmp.21 = *i_ptr.5
            tmp.22 = sign_extend tmp.21
            tmp.24 = *ul_ptr.3
            tmp.23 = tmp.22 | tmp.24
            tmp.26 = sign_extend i.4
            tmp.25 = tmp.23 != tmp.26
            if !tmp.25 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_conversion() {
    let src = r#"
        int main(void) {
            double d = 5.0;
            double *d_ptr = &d;
            *d_ptr *= 1000u;
            if (d != 5000.0) {
                return 1;
            }
            int i = -50;
            int *i_ptr = &i;
            *i_ptr %= 4294967200U;
            if (*i_ptr != 46) {
                return 2;
            }
            unsigned int ui = 4294967295U;
            ui /= *d_ptr;
            if (ui != 858993u) {
                return 3;
            }
            i = -10;
            unsigned long ul = 9223372036854775807ul;
            unsigned long *ul_ptr = &ul;
            *i_ptr -= *ul_ptr;
            if (i != -9) {
                return 4;
            }
            if (ul != 9223372036854775807ul) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            d.0 = 5D
            tmp.0 = &d.0
            d_ptr.1 = tmp.0
            tmp.1 = *d_ptr.1
            tmp.3 = uint_to_double 1000U
            tmp.2 = tmp.1 * tmp.3
            *d_ptr.1 = tmp.2
            tmp.4 = d.0 != 5000D
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = - 50
            i.2 = tmp.5
            tmp.6 = &i.2
            i_ptr.3 = tmp.6
            tmp.7 = *i_ptr.3
            tmp.8 = tmp.7
            tmp.9 = tmp.8 % 4294967200U
            tmp.10 = tmp.9
            *i_ptr.3 = tmp.10
            tmp.11 = tmp.10
            tmp.12 = *i_ptr.3
            tmp.13 = tmp.12 != 46
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            ui.4 = 4294967295U
            tmp.14 = uint_to_double ui.4
            tmp.16 = *d_ptr.1
            tmp.15 = tmp.14 / tmp.16
            tmp.17 = double_to_uint tmp.15
            ui.4 = tmp.17
            tmp.18 = double_to_uint tmp.17
            tmp.19 = ui.4 != 858993U
            if !tmp.19 jump end_if_4
            return 3
        
          end_if_4:
            tmp.20 = - 10
            i.2 = tmp.20
            ul.5 = 9223372036854775807UL
            tmp.21 = &ul.5
            ul_ptr.6 = tmp.21
            tmp.22 = *i_ptr.3
            tmp.23 = sign_extend tmp.22
            tmp.25 = *ul_ptr.6
            tmp.24 = tmp.23 - tmp.25
            tmp.26 = truncate tmp.24
            *i_ptr.3 = tmp.26
            tmp.27 = truncate tmp.26
            tmp.29 = - 9
            tmp.28 = i.2 != tmp.29
            if !tmp.28 jump end_if_6
            return 4
        
          end_if_6:
            tmp.30 = ul.5 != 9223372036854775807UL
            if !tmp.30 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_through_pointer() {
    let src = r#"
        int main(void) {
            int x = 10;
            int *ptr = &x;
            *ptr += 5;
            if (x != 15) {
                return 1;
            }
            if ((*ptr -= 12) != 3) {
                return 2;
            }
            if (x != 3) {
                return 3;
            }
            *ptr *= 6;
            if (x != 18) {
                return 4;
            }
            *ptr /= 9;
            if (x != 2) {
                return 5;
            }
            *ptr %= 3;
            if (x != 2) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = &x.0
            ptr.1 = tmp.0
            tmp.1 = *ptr.1
            tmp.2 = tmp.1 + 5
            *ptr.1 = tmp.2
            tmp.3 = x.0 != 15
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = *ptr.1
            tmp.5 = tmp.4 - 12
            *ptr.1 = tmp.5
            tmp.6 = tmp.5 != 3
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = x.0 != 3
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = *ptr.1
            tmp.9 = tmp.8 * 6
            *ptr.1 = tmp.9
            tmp.10 = x.0 != 18
            if !tmp.10 jump end_if_6
            return 4
        
          end_if_6:
            tmp.11 = *ptr.1
            tmp.12 = tmp.11 / 9
            *ptr.1 = tmp.12
            tmp.13 = x.0 != 2
            if !tmp.13 jump end_if_8
            return 5
        
          end_if_8:
            tmp.14 = *ptr.1
            tmp.15 = tmp.14 % 3
            *ptr.1 = tmp.15
            tmp.16 = x.0 != 2
            if !tmp.16 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_bitwise_dereferenced_ptrs() {
    let src = r#"
        unsigned long ul = 18446460386757245432ul;
        int main(void) {
            unsigned long *ul_ptr = &ul;
            *ul_ptr &= -1000;
            if (ul != 18446460386757244952ul ) {
                return 1;
            }
            *ul_ptr |= 4294967040u;
            if (ul != 18446460386824683288ul ) {
                return 2;
            }
            int i = 123456;
            unsigned int ui = 4042322160u;
            long l = -252645136;
            unsigned int *ui_ptr = &ui;
            long *l_ptr = &l;
            if (*ui_ptr ^= *l_ptr) {
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
            tmp.0 = &ul
            ul_ptr.0 = tmp.0
            tmp.1 = *ul_ptr.0
            tmp.3 = - 1000
            tmp.4 = sign_extend tmp.3
            tmp.2 = tmp.1 & tmp.4
            *ul_ptr.0 = tmp.2
            tmp.5 = ul != 18446460386757244952UL
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = *ul_ptr.0
            tmp.8 = zero_extend 4294967040U
            tmp.7 = tmp.6 | tmp.8
            *ul_ptr.0 = tmp.7
            tmp.9 = ul != 18446460386824683288UL
            if !tmp.9 jump end_if_2
            return 2
        
          end_if_2:
            i.1 = 123456
            ui.2 = 4042322160U
            tmp.10 = - 252645136
            tmp.11 = sign_extend tmp.10
            l.3 = tmp.11
            tmp.12 = &ui.2
            ui_ptr.4 = tmp.12
            tmp.13 = &l.3
            l_ptr.5 = tmp.13
            tmp.14 = *ui_ptr.4
            tmp.15 = zero_extend tmp.14
            tmp.17 = *l_ptr.5
            tmp.16 = tmp.15 ^ tmp.17
            tmp.18 = truncate tmp.16
            *ui_ptr.4 = tmp.18
            tmp.19 = truncate tmp.18
            if !tmp.19 jump end_if_4
            return 3
        
          end_if_4:
            if !ui.2 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = i.1 != 123456
            if !tmp.20 jump end_if_8
            return 5
        
          end_if_8:
            tmp.22 = - 252645136
            tmp.23 = sign_extend tmp.22
            tmp.21 = l.3 != tmp.23
            if !tmp.21 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static global ul: Unsigned Long = 18446460386757245432UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_eval_compound_lhs_once() {
    let src = r#"
        int i = 0;
        int putchar(int c);
        int *print_A(void) {
            putchar(65);
            return &i;
        }
        int *print_B(void) {
            putchar(66);
            return &i;
        }
        int main(void) {
            *print_A() += 5;
            if (i != 5) {
                return 1;
            }
            *print_B() += 5l;
            if (i != 10) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function print_A() { 
            tmp.0 = putchar(65)
            tmp.1 = &i
            return tmp.1
            return 0
        }
        global function print_B() { 
            tmp.2 = putchar(66)
            tmp.3 = &i
            return tmp.3
            return 0
        }
        global function main() { 
            tmp.4 = print_A()
            tmp.5 = *tmp.4
            tmp.6 = tmp.5 + 5
            *tmp.4 = tmp.6
            tmp.7 = i != 5
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = print_B()
            tmp.9 = *tmp.8
            tmp.10 = sign_extend tmp.9
            tmp.11 = tmp.10 + 5L
            tmp.12 = truncate tmp.11
            *tmp.8 = tmp.12
            tmp.13 = truncate tmp.12
            tmp.14 = i != 10
            if !tmp.14 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global i: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_incr_and_decr_through_pointer() {
    let src = r#"
        
        int main(void) {
            int x = 10;
            int *y = &x;
            if (++*y != 11) {
                return 1;
            }
            if (x != 11) {
                return 2;
            }
            if (--*y != 10) {
                return 3;
            }
            if (x != 10) {
                return 4;
            }
            if ((*y)++ != 10) {
                return 5;
            }
            if (x != 11) {
                return 6;
            }
            if ((*y)-- != 11) {
                return 7;
            }
            if (x != 10) {
                return 8;
            }
            unsigned long ul = 0;
            unsigned long *ul_ptr = &ul;
            if ((*ul_ptr)--) {
                return 9;
            }
            if (ul != 18446744073709551615UL) {
                return 10;
            }
            double d = 0.0;
            double *d_ptr = &d;
            if (++(*d_ptr) != 1.0) {
                return 11;
            }
            if (d != 1.0) {
                return 12;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = &x.0
            y.1 = tmp.0
            tmp.1 = *y.1
            tmp.2 = inc tmp.1
            *y.1 = tmp.2
            tmp.3 = tmp.2 != 11
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = x.0 != 11
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = *y.1
            tmp.6 = dec tmp.5
            *y.1 = tmp.6
            tmp.7 = tmp.6 != 10
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = x.0 != 10
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = *y.1
            tmp.10 = tmp.9
            tmp.11 = inc tmp.9
            *y.1 = tmp.11
            tmp.12 = tmp.10 != 10
            if !tmp.12 jump end_if_8
            return 5
        
          end_if_8:
            tmp.13 = x.0 != 11
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            tmp.14 = *y.1
            tmp.15 = tmp.14
            tmp.16 = dec tmp.14
            *y.1 = tmp.16
            tmp.17 = tmp.15 != 11
            if !tmp.17 jump end_if_12
            return 7
        
          end_if_12:
            tmp.18 = x.0 != 10
            if !tmp.18 jump end_if_14
            return 8
        
          end_if_14:
            tmp.19 = sign_extend 0
            ul.2 = tmp.19
            tmp.20 = &ul.2
            ul_ptr.3 = tmp.20
            tmp.21 = *ul_ptr.3
            tmp.22 = tmp.21
            tmp.23 = dec tmp.21
            *ul_ptr.3 = tmp.23
            if !tmp.22 jump end_if_16
            return 9
        
          end_if_16:
            tmp.24 = ul.2 != 18446744073709551615UL
            if !tmp.24 jump end_if_18
            return 10
        
          end_if_18:
            d.4 = 0D
            tmp.25 = &d.4
            d_ptr.5 = tmp.25
            tmp.26 = *d_ptr.5
            tmp.27 = inc tmp.26
            *d_ptr.5 = tmp.27
            tmp.28 = tmp.27 != 1D
            if !tmp.28 jump end_if_20
            return 11
        
          end_if_20:
            tmp.29 = d.4 != 1D
            if !tmp.29 jump end_if_22
            return 12
        
          end_if_22:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_dereferenced_pointer() {
    let src = r#"
        long l = 4294967300l;
        long *get_ptr(void) {
            return &l;
        }
        int main(void) {
            switch (*get_ptr()) {
                case 1:
                    return 1;
                case 4:
                    return 2;
                case 4294967300l:
                    return 0;
                case 18446744073709551600UL:
                    return 3;
                default:
                    return 4;
            }
        }
    "#;
    let expected = r#"
        global function get_ptr() { 
            tmp.0 = &l
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = get_ptr()
            tmp.2 = *tmp.1
            tmp.3 = 1L == tmp.2
            if tmp.3 jump switch_0_case__1
            tmp.4 = 4L == tmp.2
            if tmp.4 jump switch_0_case__2
            tmp.5 = 4294967300L == tmp.2
            if tmp.5 jump switch_0_case__3
            tmp.6 = -16L == tmp.2
            if tmp.6 jump switch_0_case__4
            jump switch_0_default_5
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 2
        
          switch_0_case__3:
            return 0
        
          switch_0_case__4:
            return 3
        
          switch_0_default_5:
            return 4
        
          break_switch_0:
            return 0
        }
        static global l: Long = 4294967300L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_function_calls_address_of_argument() {
    let src = r#"
        int addr_of_arg(int a) {
            int *ptr = &a;
            *ptr = 10;
            return a;
        }
        int main(void) {
            int result = addr_of_arg(-20);
            if (result != 10) {
                return 1;
            }
            int var = 100;
            result = addr_of_arg(var);
            if (result != 10) {
                return 2;
            }
            if (var != 100) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function addr_of_arg(a.0) { 
            tmp.0 = &a.0
            ptr.1 = tmp.0
            *ptr.1 = 10
            return a.0
            return 0
        }
        global function main() { 
            tmp.1 = - 20
            tmp.2 = addr_of_arg(tmp.1)
            result.2 = tmp.2
            tmp.3 = result.2 != 10
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            var.3 = 100
            tmp.4 = addr_of_arg(var.3)
            result.2 = tmp.4
            tmp.5 = result.2 != 10
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = var.3 != 100
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_function_calls_return_pointer() {
    let src = r#"
        
        int *return_pointer(int *in) {
            return in;
        }
        int main(void) {
            int x = 10;
            int *x_ptr = return_pointer(&x);
            if (*x_ptr != 10)
                return 1;
            x = 100;
            if (*x_ptr != 100)
                return 2;
            if (x_ptr != &x)
                return 3;
            return 0;
        }
    "#;
    let expected = r#"
        global function return_pointer(in.0) { 
            return in.0
            return 0
        }
        global function main() { 
            x.1 = 10
            tmp.0 = &x.1
            tmp.1 = return_pointer(tmp.0)
            x_ptr.2 = tmp.1
            tmp.2 = *x_ptr.2
            tmp.3 = tmp.2 != 10
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            x.1 = 100
            tmp.4 = *x_ptr.2
            tmp.5 = tmp.4 != 100
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = &x.1
            tmp.6 = x_ptr.2 != tmp.7
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_function_calls_update_value_through_pointer_parameter() {
    let src = r#"
        
        int update_value(int *ptr) {
            int old_val = *ptr;
            *ptr = 10;
            return old_val;
        }
        int main(void) {
            int x = 20;
            int result = update_value(&x);
            if (result != 20) {
                return 1;
            }
            if (x != 10) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function update_value(ptr.0) { 
            tmp.0 = *ptr.0
            old_val.1 = tmp.0
            *ptr.0 = 10
            return old_val.1
            return 0
        }
        global function main() { 
            x.2 = 20
            tmp.1 = &x.2
            tmp.2 = update_value(tmp.1)
            result.3 = tmp.2
            tmp.3 = result.3 != 20
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = x.2 != 10
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_global_pointer() {
    let src = r#"
        double *d_ptr;
        int update_thru_ptr(double new_val) {
            *d_ptr = new_val;
            return 0;
        }
    "#;
    let expected = r#"
        global function update_thru_ptr(new_val.0) { 
            *d_ptr = new_val.0
            return 0
            return 0
        }
        static global d_ptr: Pointer(Double) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_global_pointer_client() {
    let src = r#"
        extern double *d_ptr;
        int update_thru_ptr(double new_val);
        int main(void) {
            double d = 0.0;
            d_ptr = &d;
            update_thru_ptr(10.0);
            return (d == 10.0);
        }
    "#;
    let expected = r#"
        global function main() { 
            d.1 = 0D
            tmp.0 = &d.1
            d_ptr = tmp.0
            tmp.1 = update_thru_ptr(10D)
            tmp.2 = d.1 == 10D
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_static_pointer() {
    let src = r#"
        static long *long_ptr;
        long *get_pointer(void) {
            return long_ptr;
        }
        int set_pointer(long *new_ptr) {
            long_ptr = new_ptr;
            return 0;
        }
    "#;
    let expected = r#"
        global function get_pointer() { 
            return long_ptr
            return 0
        }
        global function set_pointer(new_ptr.0) { 
            long_ptr = new_ptr.0
            return 0
            return 0
        }
        static long_ptr: Pointer(Long) = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_static_pointer_client() {
    let src = r#"
        long *get_pointer(void);
        int set_pointer(long *new_ptr);
        static long private_long = 100l;
        int main(void) {
            long *initial_ptr = get_pointer();
            if (initial_ptr) {
                return 1;
            }
            set_pointer(&private_long);
            long *new_ptr = get_pointer();
            if (initial_ptr == new_ptr) {
                return 2;
            }
            if (*new_ptr != 100l) {
                return 3;
            }
            if (new_ptr != &private_long) {
                return 4;
            }
            set_pointer(0);
            if (get_pointer()) {
                return 5;
            }
            if (new_ptr != &private_long) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = get_pointer()
            initial_ptr.1 = tmp.0
            if !initial_ptr.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = &private_long
            tmp.2 = set_pointer(tmp.1)
            tmp.3 = get_pointer()
            new_ptr.2 = tmp.3
            tmp.4 = initial_ptr.1 == new_ptr.2
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = *new_ptr.2
            tmp.6 = tmp.5 != 100L
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = &private_long
            tmp.7 = new_ptr.2 != tmp.8
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 0
            tmp.10 = set_pointer(tmp.9)
            tmp.11 = get_pointer()
            if !tmp.11 jump end_if_8
            return 5
        
          end_if_8:
            tmp.13 = &private_long
            tmp.12 = new_ptr.2 != tmp.13
            if !tmp.12 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static private_long: Long = 100L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
