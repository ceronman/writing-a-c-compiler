use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_extra_credit_sizeof_bitwise() {
    let src = r#"
        
        int main(void) {
            static long l = 0;
            int i = 0;
            static char c = 0;
            if (sizeof (c & i) != 4) {
                return 1;
            }
            if (sizeof (i | l) != 8) {
                return 2;
            }
            if (sizeof (c ^ c) != 4) {
                return 3;
            }
            if (sizeof (i << l) != 4) {
                return 4;
            }
            if (sizeof (c << i) != 4) {
                return 5;
            }
            if (sizeof (l >> c) != 8) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.1 = 0
            tmp.1 = sign_extend 4
            tmp.0 = 4UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 8
            tmp.2 = 8UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 4
            tmp.4 = 4UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend 4
            tmp.6 = 4UL != tmp.7
            if !tmp.6 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 4
            tmp.8 = 4UL != tmp.9
            if !tmp.8 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = sign_extend 8
            tmp.10 = 8UL != tmp.11
            if !tmp.10 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static c.2: Char = '\0'
        static l.0: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_sizeof_compound() {
    let src = r#"
        int main(void) {
            long long_arr[2] = {1, 2};
            static int i = 3;
            static unsigned char uc = 4;
            double d = 5.0;
            long *ptr = long_arr;
            if (sizeof(long_arr[1] *= 10) != 8) {
                return 1;
            }
            if (sizeof(i /= 10ul) != 4) {
                return 2;
            }
            if (sizeof(uc %= 2) != 1) {
                return 3;
            }
            if (sizeof(d -= 11) != 8) {
                return 4;
            }
            if (sizeof(ptr += 1) != 8) {
                return 5;
            }
            if (long_arr[0] != 1) {
                return 6;
            }
            if (long_arr[1] != 2) {
                return 7;
            }
            if (i != 3) {
                return 8;
            }
            if (uc != 4) {
                return 9;
            }
            if (d != 5.0) {
                return 10;
            }
            if (ptr != long_arr) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 1
            long_arr.0[0] = tmp.0
            tmp.1 = sign_extend 2
            long_arr.0[8] = tmp.1
            d.3 = 5D
            tmp.2 = &long_arr.0
            ptr.4 = tmp.2
            tmp.4 = sign_extend 8
            tmp.3 = 8UL != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = sign_extend 4
            tmp.5 = 4UL != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = sign_extend 1
            tmp.7 = 1UL != tmp.8
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.10 = sign_extend 8
            tmp.9 = 8UL != tmp.10
            if !tmp.9 jump end_if_6
            return 4
        
          end_if_6:
            tmp.12 = sign_extend 8
            tmp.11 = 8UL != tmp.12
            if !tmp.11 jump end_if_8
            return 5
        
          end_if_8:
            tmp.13 = &long_arr.0
            tmp.14 = sign_extend 0
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=8)
            tmp.16 = *tmp.15
            tmp.18 = sign_extend 1
            tmp.17 = tmp.16 != tmp.18
            if !tmp.17 jump end_if_10
            return 6
        
          end_if_10:
            tmp.19 = &long_arr.0
            tmp.20 = sign_extend 1
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=8)
            tmp.22 = *tmp.21
            tmp.24 = sign_extend 2
            tmp.23 = tmp.22 != tmp.24
            if !tmp.23 jump end_if_12
            return 7
        
          end_if_12:
            tmp.25 = i.1 != 3
            if !tmp.25 jump end_if_14
            return 8
        
          end_if_14:
            tmp.26 = zero_extend uc.2
            tmp.27 = tmp.26 != 4
            if !tmp.27 jump end_if_16
            return 9
        
          end_if_16:
            tmp.28 = d.3 != 5D
            if !tmp.28 jump end_if_18
            return 10
        
          end_if_18:
            tmp.30 = &long_arr.0
            tmp.29 = ptr.4 != tmp.30
            if !tmp.29 jump end_if_20
            return 11
        
          end_if_20:
            return 0
            return 0
        }
        static i.1: Int = 3
        static uc.2: Unsigned Char = 4UC
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_sizeof_compound_bitwise() {
    let src = r#"
        int main(void) {
            static signed char sc = 10;
            unsigned int u = 10000u;
            long l = -99999;
            if (sizeof(sc &= l) != 1) {
                return 1;
            }
            if (sizeof(l |= u) != 8) {
                return 2;
            }
            if (sizeof(u ^= l) != 4) {
                return 3;
            }
            if (sizeof(l >>= sc) != 8) {
                return 4;
            }
            if (sizeof(sc <<= sc) != 1) {
                return 5;
            }
            if (sc != 10) {
                return 6;
            }
            if (u != 10000u) {
                return 7;
            }
            if (l != -99999) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            u.1 = 10000U
            tmp.0 = - 99999
            tmp.1 = sign_extend tmp.0
            l.2 = tmp.1
            tmp.3 = sign_extend 1
            tmp.2 = 1UL != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = sign_extend 8
            tmp.4 = 8UL != tmp.5
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = sign_extend 4
            tmp.6 = 4UL != tmp.7
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = sign_extend 8
            tmp.8 = 8UL != tmp.9
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.11 = sign_extend 1
            tmp.10 = 1UL != tmp.11
            if !tmp.10 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = sign_extend sc.0
            tmp.13 = tmp.12 != 10
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            tmp.14 = u.1 != 10000U
            if !tmp.14 jump end_if_12
            return 7
        
          end_if_12:
            tmp.16 = - 99999
            tmp.17 = sign_extend tmp.16
            tmp.15 = l.2 != tmp.17
            if !tmp.15 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        static sc.0: Signed Char = '\n'
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_sizeof_incr() {
    let src = r#"
        int main(void) {
            int i = 0;
            long l = 0;
            static char arr[3] = {0, 0, 0};
            char *ptr = arr;
            if (sizeof (i++) != 4) {
                return 1;
            }
            if (sizeof (arr[0]--) != 1) {
                return 2;
            }
            if (sizeof (++l) != 8) {
                return 3;
            }
            if (sizeof (--arr[1]) != 1) {
                return 4;
            }
            if (sizeof (ptr--) != 8) {
                return 5;
            }
            if (i) {
                return 6;
            }
            if (l) {
                return 7;
            }
            if (arr[0] || arr[1] || arr[2]) {
                return 8;
            }
            if (ptr != arr) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
            tmp.0 = sign_extend 0
            l.1 = tmp.0
            tmp.1 = &arr.2
            ptr.3 = tmp.1
            tmp.3 = sign_extend 4
            tmp.2 = 4UL != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = sign_extend 1
            tmp.4 = 1UL != tmp.5
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = sign_extend 8
            tmp.6 = 8UL != tmp.7
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = sign_extend 1
            tmp.8 = 1UL != tmp.9
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.11 = sign_extend 8
            tmp.10 = 8UL != tmp.11
            if !tmp.10 jump end_if_8
            return 5
        
          end_if_8:
            if !i.0 jump end_if_10
            return 6
        
          end_if_10:
            if !l.1 jump end_if_12
            return 7
        
          end_if_12:
            tmp.12 = &arr.2
            tmp.13 = sign_extend 0
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            if tmp.15 jump or_true_14
            tmp.18 = &arr.2
            tmp.19 = sign_extend 1
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=1)
            tmp.21 = *tmp.20
            if tmp.21 jump or_true_14
            tmp.17 = 0
            jump or_end_15
        
          or_true_14:
            tmp.17 = 1
        
          or_end_15:
            if tmp.17 jump or_true_16
            tmp.24 = &arr.2
            tmp.25 = sign_extend 2
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=1)
            tmp.27 = *tmp.26
            if tmp.27 jump or_true_16
            tmp.23 = 0
            jump or_end_17
        
          or_true_16:
            tmp.23 = 1
        
          or_end_17:
            if !tmp.23 jump end_if_18
            return 8
        
          end_if_18:
            tmp.29 = &arr.2
            tmp.28 = ptr.3 != tmp.29
            if !tmp.28 jump end_if_20
            return 9
        
          end_if_20:
            return 0
            return 0
        }
        static arr.2: Array(3,Char) = [ '\0', '\0', '\0']
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_pass_alloced_memory() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *memset(void *s, int c, unsigned long n);
        void free(void *ptr);
        void *get_100_zeroed_bytes(void) {
            return calloc(100, 1);
        }
        void fill_100_bytes(void *pointer, int byte) {
            memset(pointer, byte, 100);
        }
        void free_bytes(void *ptr) {
            free(ptr);
        }
    "#;
    let expected = r#"
        global function get_100_zeroed_bytes() { 
            tmp.0 = sign_extend 100
            tmp.1 = sign_extend 1
            tmp.2 = calloc(tmp.0, tmp.1)
            return tmp.2
            return 0
        }
        global function fill_100_bytes(pointer.6, byte.7) { 
            tmp.3 = sign_extend 100
            tmp.4 = memset(pointer.6, byte.7, tmp.3)
            return 0
        }
        global function free_bytes(ptr.8) { 
            free(ptr.8)
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_pass_alloced_memory_client() {
    let src = r#"
        void *get_100_zeroed_bytes(void);
        void fill_100_bytes(void *pointer, int byte);
        void free_bytes(void *ptr);
        int main(void) {
            void *mem = get_100_zeroed_bytes();
            for (int i = 0; i < 100; i = i + 1) {
                if (((char *) mem + i)[0]) {
                    return 1;
                }
            }
            fill_100_bytes(mem, 99);
            for (int i = 0; i < 100; i = i + 1) {
                if (((char *) mem + i)[0] != 99) {
                    return 2;
                }
            }
            free_bytes(mem);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = get_100_zeroed_bytes()
            mem.3 = tmp.0
            i.4 = 0
        
          start_loop_0:
            tmp.1 = i.4 < 100
            if !tmp.1 jump break_loop_0
            tmp.2 = mem.3
            tmp.4 = sign_extend i.4
            tmp.3 = add_ptr(tmp.2, index=tmp.4, scale=1)
            tmp.5 = sign_extend 0
            tmp.6 = add_ptr(tmp.3, index=tmp.5, scale=1)
            tmp.7 = *tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.8 = i.4 + 1
            i.4 = tmp.8
            jump start_loop_0
        
          break_loop_0:
            fill_100_bytes(mem.3, 99)
            i.5 = 0
        
          start_loop_1:
            tmp.9 = i.5 < 100
            if !tmp.9 jump break_loop_1
            tmp.10 = mem.3
            tmp.12 = sign_extend i.5
            tmp.11 = add_ptr(tmp.10, index=tmp.12, scale=1)
            tmp.13 = sign_extend 0
            tmp.14 = add_ptr(tmp.11, index=tmp.13, scale=1)
            tmp.15 = *tmp.14
            tmp.16 = sign_extend tmp.15
            tmp.17 = tmp.16 != 99
            if !tmp.17 jump end_if_2
            return 2
        
          end_if_2:
        
          continue_loop_1:
            tmp.18 = i.5 + 1
            i.5 = tmp.18
            jump start_loop_1
        
          break_loop_1:
            free_bytes(mem.3)
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_sizeof_extern() {
    let src = r#"
        double large_array[1000][2000];
    "#;
    let expected = r#"
        static global large_array: Array(1000,Array(2000,Double)) = zero[16000000]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_sizeof_extern_client() {
    let src = r#"
        
        extern double large_array[1000][2000];
        int main(void) {
            return sizeof large_array == 16000000;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 16000000
            tmp.0 = 16000000UL == tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_test_for_memory_leaks() {
    let src = r#"
        void exit(int status);
        long sum = 0;
        void lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int m, int n, int o) {
            if (a != 1) {
                exit(1);
            }
            if (b != 2) {
                exit(2);
            }
            if (c != 3) {
                exit(3);
            }
            if (d != 4) {
                exit(4);
            }
            if (e != 5) {
                exit(5);
            }
            if (f != 6) {
                exit(6);
            }
            if (g != 7) {
                exit(7);
            }
            if (h != 8) {
                exit(8);
            }
            if (i != 9) {
                exit(9);
            }
            if (j != 10) {
                exit(10);
            }
            if (k != 11) {
                exit(11);
            }
            if (l != 12) {
                exit(12);
            }
            if (m != 13) {
                exit(13);
            }
            if (n != 14) {
             exit(14);
            }
            sum = sum + o;
            return;
        }
    "#;
    let expected = r#"
        global function lots_of_args(a.1, b.2, c.3, d.4, e.5, f.6, g.7, h.8, i.9, j.10, k.11, l.12, m.13, n.14, o.15) { 
            tmp.0 = a.1 != 1
            if !tmp.0 jump end_if_0
            exit(1)
        
          end_if_0:
            tmp.1 = b.2 != 2
            if !tmp.1 jump end_if_2
            exit(2)
        
          end_if_2:
            tmp.2 = c.3 != 3
            if !tmp.2 jump end_if_4
            exit(3)
        
          end_if_4:
            tmp.3 = d.4 != 4
            if !tmp.3 jump end_if_6
            exit(4)
        
          end_if_6:
            tmp.4 = e.5 != 5
            if !tmp.4 jump end_if_8
            exit(5)
        
          end_if_8:
            tmp.5 = f.6 != 6
            if !tmp.5 jump end_if_10
            exit(6)
        
          end_if_10:
            tmp.6 = g.7 != 7
            if !tmp.6 jump end_if_12
            exit(7)
        
          end_if_12:
            tmp.7 = h.8 != 8
            if !tmp.7 jump end_if_14
            exit(8)
        
          end_if_14:
            tmp.8 = i.9 != 9
            if !tmp.8 jump end_if_16
            exit(9)
        
          end_if_16:
            tmp.9 = j.10 != 10
            if !tmp.9 jump end_if_18
            exit(10)
        
          end_if_18:
            tmp.10 = k.11 != 11
            if !tmp.10 jump end_if_20
            exit(11)
        
          end_if_20:
            tmp.11 = l.12 != 12
            if !tmp.11 jump end_if_22
            exit(12)
        
          end_if_22:
            tmp.12 = m.13 != 13
            if !tmp.12 jump end_if_24
            exit(13)
        
          end_if_24:
            tmp.13 = n.14 != 14
            if !tmp.13 jump end_if_26
            exit(14)
        
          end_if_26:
            tmp.15 = sign_extend o.15
            tmp.14 = sum + tmp.15
            sum = tmp.14
            return 
        
            return 0
        }
        static global sum: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_test_for_memory_leaks_client() {
    let src = r#"
        extern long sum;
        void lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j, int k, int l, int m, int n, int o);
        int main(void) {
            for (int i = 0; i < 10000000; i = i + 1) {
                lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, i);
            }
            if (sum != 49999995000000) {
                return 15;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.15 = 0
        
          start_loop_0:
            tmp.0 = i.15 < 10000000
            if !tmp.0 jump break_loop_0
            lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, i.15)
        
          continue_loop_0:
            tmp.1 = i.15 + 1
            i.15 = tmp.1
            jump start_loop_0
        
          break_loop_0:
            tmp.2 = sum != 49999995000000L
            if !tmp.2 jump end_if_0
            return 15
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_simple() {
    let src = r#"
        int main(void) {
            if (sizeof (int) != 4) {
                return 1;
            }
            if (sizeof 3.0 != 8) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 4
            tmp.0 = 4UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 8
            tmp.2 = 8UL != tmp.3
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
fn test_valid_sizeof_sizeof_array() {
    let src = r#"
        unsigned long sizeof_adjusted_param(int arr[3]) {
            return sizeof arr;
        }
        int main(void) {
            int arr[3];
            if (sizeof arr != 12) {
                return 1;
            }
            static long nested_arr[4][5];
            if (sizeof nested_arr[2] != 40) {
                return 2;
            }
            if (sizeof "Hello, World!" != 14) {
                return 3;
            }
            if (sizeof_adjusted_param(arr) != 8) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function sizeof_adjusted_param(arr.0) { 
            return 8UL
            return 0
        }
        global function main() { 
            tmp.1 = sign_extend 12
            tmp.0 = 12UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 40
            tmp.2 = 40UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 14
            tmp.4 = 14UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.6 = &arr.1
            tmp.7 = sizeof_adjusted_param(tmp.6)
            tmp.9 = sign_extend 8
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static nested_arr.2: Array(4,Array(5,Long)) = zero[160]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_sizeof_basic_types() {
    let src = r#"
        int main(void) {
            if (sizeof(char) != 1) {
                return 1;
            }
            if (sizeof(signed char) != 1) {
                return 2;
            }
            if (sizeof(unsigned char) != 1) {
                return 3;
            }
            if (sizeof(int) != 4) {
                return 4;
            }
            if (sizeof(unsigned int) != 4) {
                return 5;
            }
            if (sizeof(long) != 8) {
                return 6;
            }
            if (sizeof(unsigned long) != 8) {
                return 7;
            }
            if (sizeof(double) != 8) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 1
            tmp.0 = 1UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 1
            tmp.2 = 1UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 1
            tmp.4 = 1UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend 4
            tmp.6 = 4UL != tmp.7
            if !tmp.6 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 4
            tmp.8 = 4UL != tmp.9
            if !tmp.8 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = sign_extend 8
            tmp.10 = 8UL != tmp.11
            if !tmp.10 jump end_if_10
            return 6
        
          end_if_10:
            tmp.13 = sign_extend 8
            tmp.12 = 8UL != tmp.13
            if !tmp.12 jump end_if_12
            return 7
        
          end_if_12:
            tmp.15 = sign_extend 8
            tmp.14 = 8UL != tmp.15
            if !tmp.14 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_sizeof_consts() {
    let src = r#"
        int main(void) {
            if (sizeof 'a' != 4) {
                return 1;
            }
            if (sizeof 2147483647 != 4) {
                return 2;
            }
            if (sizeof 4294967295U != 4) {
                return 3;
            }
            if (sizeof 2l != 8) {
                return 4;
            }
            if (sizeof 0ul != 8) {
                return 5;
            }
            if (sizeof 1.0 != 8) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 4
            tmp.0 = 4UL != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = sign_extend 4
            tmp.2 = 4UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = sign_extend 4
            tmp.4 = 4UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend 8
            tmp.6 = 8UL != tmp.7
            if !tmp.6 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 8
            tmp.8 = 8UL != tmp.9
            if !tmp.8 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = sign_extend 8
            tmp.10 = 8UL != tmp.11
            if !tmp.10 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_sizeof_derived_types() {
    let src = r#"
        void *malloc(unsigned long size);
        int main(void) {
            if (sizeof(int[2]) != 8) {
                return 1;
            }
            if (sizeof(char[3][6][17][9]) != 2754) {
                return 2;
            }
            if (sizeof(int[4294967297L][100000000]) != 1717986918800000000l) {
                return 3;
            }
            if (sizeof(int *) != 8) {
                return 4;
            }
            if (sizeof(int(*)[2][4][6]) !=
                8) {
                return 5;
            }
            if (sizeof(char *) != 8) {
                return 6;
            }
            if (sizeof(double(*([3][4]))[2]) != 96) {
                return 7;
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
            tmp.3 = sign_extend 2754
            tmp.2 = 2754UL != tmp.3
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = 1717986918800000000L
            tmp.4 = 1717986918800000000UL != tmp.5
            if !tmp.4 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = sign_extend 8
            tmp.6 = 8UL != tmp.7
            if !tmp.6 jump end_if_6
            return 4
        
          end_if_6:
            tmp.9 = sign_extend 8
            tmp.8 = 8UL != tmp.9
            if !tmp.8 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = sign_extend 8
            tmp.10 = 8UL != tmp.11
            if !tmp.10 jump end_if_10
            return 6
        
          end_if_10:
            tmp.13 = sign_extend 96
            tmp.12 = 96UL != tmp.13
            if !tmp.12 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_sizeof_expressions() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        int main(void) {
            double d;
            if (sizeof d != 8) {
                return 2;
            }
            unsigned char c;
            if (sizeof c != 1) {
                return 3;
            }
            void *buffer = malloc(100);
            if (sizeof(buffer) != 8) {
                return 4;
            }
            free(buffer);
            if (sizeof ((int)d) != 4) {
                return 5;
            }
            if (sizeof (d ? c : 10l) != 8) {
                return 6;
            }
            if (sizeof (c = 10.0) != 1) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = sign_extend 8
            tmp.0 = 8UL != tmp.1
            if !tmp.0 jump end_if_0
            return 2
        
          end_if_0:
            tmp.3 = sign_extend 1
            tmp.2 = 1UL != tmp.3
            if !tmp.2 jump end_if_2
            return 3
        
          end_if_2:
            tmp.4 = sign_extend 100
            tmp.5 = malloc(tmp.4)
            buffer.4 = tmp.5
            tmp.7 = sign_extend 8
            tmp.6 = 8UL != tmp.7
            if !tmp.6 jump end_if_4
            return 4
        
          end_if_4:
            free(buffer.4)
            tmp.9 = sign_extend 4
            tmp.8 = 4UL != tmp.9
            if !tmp.8 jump end_if_6
            return 5
        
          end_if_6:
            tmp.11 = sign_extend 8
            tmp.10 = 8UL != tmp.11
            if !tmp.10 jump end_if_8
            return 6
        
          end_if_8:
            tmp.13 = sign_extend 1
            tmp.12 = 1UL != tmp.13
            if !tmp.12 jump end_if_10
            return 7
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_sizeof_not_evaluated() {
    let src = r#"
        void exit(int status);
        int foo(void) { exit(10); }
        int main(void) {
          return sizeof(foo());
        }
    "#;
    let expected = r#"
        global function foo() { 
            exit(10)
            return 0
        }
        global function main() { 
            tmp.0 = truncate 4UL
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sizeof_sizeof_result_is_ulong() {
    let src = r#"
        int main(void) {
            if (sizeof sizeof (char) != 8) {
                return 1;
            }
            if (sizeof 4 - sizeof 4 - 1 < 0) {
                return 2;
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
            tmp.2 = 4UL - 4UL
            tmp.4 = sign_extend 1
            tmp.3 = tmp.2 - tmp.4
            tmp.6 = sign_extend 0
            tmp.5 = tmp.3 < tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_cast_to_void() {
    let src = r#"
        int x;
        int set_x(int i) {
            x = i;
            return 0;
        }
        void do_nothing(void) {
            ;
        }
        int main(void) {
            (void) x;
            (void) set_x(12);
            (void) do_nothing();
            return x;
        }
    "#;
    let expected = r#"
        global function set_x(i.0) { 
            x = i.0
            return 0
            return 0
        }
        global function do_nothing() { 
            return 0
        }
        global function main() { 
            tmp.0 = set_x(12)
            do_nothing()
            return x
            return 0
        }
        static global x: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_ternary() {
    let src = r#"
        int i = 4;
        int j = 5;
        int flag_1 = 1;
        int flag_0 = 0;
        void incr_i(void) {
            i = i + 1;
        }
        void incr_j(void) {
            j = j + 1;
        }
        int main(void) {
            flag_1 ? incr_i() : incr_j();
            flag_0 ? incr_i() : incr_j();
            if (i != 5) {
                return 1;
            }
            if (j != 6) {
                return 2;
            }
            flag_0 ? incr_j() : flag_1 ? incr_i() : incr_j();
            if (i != 6) {
                return 3;
            }
            if (j != 6) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function incr_i() { 
            tmp.0 = i + 1
            i = tmp.0
            return 0
        }
        global function incr_j() { 
            tmp.1 = j + 1
            j = tmp.1
            return 0
        }
        global function main() { 
            if !flag_1 jump else_1
            incr_i()
            jump end_if_0
        
          else_1:
            incr_j()
        
          end_if_0:
            if !flag_0 jump else_3
            incr_i()
            jump end_if_2
        
          else_3:
            incr_j()
        
          end_if_2:
            tmp.2 = i != 5
            if !tmp.2 jump end_if_4
            return 1
        
          end_if_4:
            tmp.3 = j != 6
            if !tmp.3 jump end_if_6
            return 2
        
          end_if_6:
            if !flag_0 jump else_9
            incr_j()
            jump end_if_8
        
          else_9:
            if !flag_1 jump else_11
            incr_i()
            jump end_if_10
        
          else_11:
            incr_j()
        
          end_if_10:
        
          end_if_8:
            tmp.4 = i != 6
            if !tmp.4 jump end_if_12
            return 3
        
          end_if_12:
            tmp.5 = j != 6
            if !tmp.5 jump end_if_14
            return 4
        
          end_if_14:
            return 0
            return 0
        }
        static global flag_0: Int = 0
        static global flag_1: Int = 1
        static global i: Int = 4
        static global j: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_void_for_loop() {
    let src = r#"
        int putchar(int c);
        int letter;
        void initialize_letter(void) {
            letter = 'Z';
        }
        void decrement_letter(void) {
            letter = letter - 1;
        }
        int main(void) {
            for (initialize_letter(); letter >= 'A';
                 letter = letter - 1) {
                putchar(letter);
            }
            for (letter = 'A'; letter <= 90; (void)(letter = letter + 1)) {
                putchar(letter);
            }
            for (initialize_letter(); letter >= 65; decrement_letter()) {
                putchar(letter);
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function initialize_letter() { 
            letter = 90
            return 0
        }
        global function decrement_letter() { 
            tmp.0 = letter - 1
            letter = tmp.0
            return 0
        }
        global function main() { 
            initialize_letter()
        
          start_loop_0:
            tmp.1 = letter >= 65
            if !tmp.1 jump break_loop_0
            tmp.2 = putchar(letter)
        
          continue_loop_0:
            tmp.3 = letter - 1
            letter = tmp.3
            jump start_loop_0
        
          break_loop_0:
            letter = 65
        
          start_loop_1:
            tmp.4 = letter <= 90
            if !tmp.4 jump break_loop_1
            tmp.5 = putchar(letter)
        
          continue_loop_1:
            tmp.6 = letter + 1
            letter = tmp.6
            jump start_loop_1
        
          break_loop_1:
            initialize_letter()
        
          start_loop_2:
            tmp.7 = letter >= 65
            if !tmp.7 jump break_loop_2
            tmp.8 = putchar(letter)
        
          continue_loop_2:
            decrement_letter()
            jump start_loop_2
        
          break_loop_2:
            return 0
            return 0
        }
        static global letter: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_void_function() {
    let src = r#"
        
        int foo = 0;
        void set_foo_to_positive_num(int a) {
            if (a < 0) {
                return;
            }
            foo = a;
            return;
        }
        void do_nothing(void) {
        }
        int main(void) {
            set_foo_to_positive_num(-2);
            if (foo) {
                return 1;
            }
            set_foo_to_positive_num(12);
            if (foo != 12) {
                return 2;
            }
            do_nothing();
            return 0;
        }
    "#;
    let expected = r#"
        global function set_foo_to_positive_num(a.0) { 
            tmp.0 = a.0 < 0
            if !tmp.0 jump end_if_0
            return 
        
        
          end_if_0:
            foo = a.0
            return 
        
            return 0
        }
        global function do_nothing() { 
            return 0
        }
        global function main() { 
            tmp.1 = - 2
            set_foo_to_positive_num(tmp.1)
            if !foo jump end_if_2
            return 1
        
          end_if_2:
            set_foo_to_positive_num(12)
            tmp.2 = foo != 12
            if !tmp.2 jump end_if_4
            return 2
        
          end_if_4:
            do_nothing()
            return 0
            return 0
        }
        static global foo: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_pointer_array_of_pointers_to_void() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void free(void *ptr);
        int main(void) {
            int i = 10;
            void *arr[4] = {
                calloc(2, sizeof(int)),
                &i,
                0,
                arr
            };
            long *l = arr[0];
            if (*l)
                return 1;
            int elem_1_val = *(int *)arr[1];
            if (elem_1_val != 10)
                return 2;
            if (arr[2])
                return 3;
            if (arr[3] != arr)
                return 4;
            free(arr[0]);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.3 = 10
            tmp.0 = sign_extend 2
            tmp.1 = calloc(tmp.0, 4UL)
            arr.4[0] = tmp.1
            tmp.2 = &i.3
            tmp.3 = tmp.2
            arr.4[8] = tmp.3
            tmp.4 = sign_extend 0
            arr.4[16] = tmp.4
            tmp.5 = &arr.4
            tmp.6 = tmp.5
            arr.4[24] = tmp.6
            tmp.7 = &arr.4
            tmp.8 = sign_extend 0
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10
            l.5 = tmp.11
            tmp.12 = *l.5
            if !tmp.12 jump end_if_0
            return 1
        
          end_if_0:
            tmp.13 = &arr.4
            tmp.14 = sign_extend 1
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=8)
            tmp.16 = *tmp.15
            tmp.17 = tmp.16
            tmp.18 = *tmp.17
            elem_1_val.6 = tmp.18
            tmp.19 = elem_1_val.6 != 10
            if !tmp.19 jump end_if_2
            return 2
        
          end_if_2:
            tmp.20 = &arr.4
            tmp.21 = sign_extend 2
            tmp.22 = add_ptr(tmp.20, index=tmp.21, scale=8)
            tmp.23 = *tmp.22
            if !tmp.23 jump end_if_4
            return 3
        
          end_if_4:
            tmp.24 = &arr.4
            tmp.25 = sign_extend 3
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=8)
            tmp.27 = *tmp.26
            tmp.29 = &arr.4
            tmp.30 = tmp.29
            tmp.28 = tmp.27 != tmp.30
            if !tmp.28 jump end_if_6
            return 4
        
          end_if_6:
            tmp.31 = &arr.4
            tmp.32 = sign_extend 0
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=8)
            tmp.34 = *tmp.33
            free(tmp.34)
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_pointer_common_pointer_type() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void free(void *ptr);
        int main(void) {
            void *void_ptr = calloc(3, sizeof(unsigned int));
            unsigned int array[3] = {1, 2, 3};
            if (void_ptr == 0)
                return 1;
            if (void_ptr == array)
                return 2;
            if (!(void_ptr != array))
                return 3;
            static void *null_ptr = 0;
            int *my_array = null_ptr ? void_ptr : array;
            int array_element = my_array[1];
            if (array_element != 2) {
                return 4;
            }
            free(void_ptr);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 3
            tmp.1 = calloc(tmp.0, 4UL)
            void_ptr.3 = tmp.1
            tmp.2 = 1
            array.4[0] = tmp.2
            tmp.3 = 2
            array.4[4] = tmp.3
            tmp.4 = 3
            array.4[8] = tmp.4
            tmp.6 = sign_extend 0
            tmp.5 = void_ptr.3 == tmp.6
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = &array.4
            tmp.9 = tmp.8
            tmp.7 = void_ptr.3 == tmp.9
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = &array.4
            tmp.12 = tmp.11
            tmp.10 = void_ptr.3 != tmp.12
            tmp.13 = ! tmp.10
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            if !null_ptr.5 jump else_7
            tmp.14 = void_ptr.3
            jump end_if_6
        
          else_7:
            tmp.15 = &array.4
            tmp.16 = tmp.15
            tmp.14 = tmp.16
        
          end_if_6:
            tmp.17 = tmp.14
            my_array.6 = tmp.17
            tmp.18 = sign_extend 1
            tmp.19 = add_ptr(my_array.6, index=tmp.18, scale=4)
            tmp.20 = *tmp.19
            array_element.7 = tmp.20
            tmp.21 = array_element.7 != 2
            if !tmp.21 jump end_if_8
            return 4
        
          end_if_8:
            free(void_ptr.3)
            return 0
            return 0
        }
        static null_ptr.5: Pointer(Void) = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_pointer_conversion_by_assignment() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        int memcmp(void *s1, void *s2, unsigned long n);
        void *return_ptr(char *i) {
            return i + 3;
        }
        int check_char_ptr_argument(char *pointer, char expected_val) {
            return *pointer == expected_val;
        }
        int *return_void_ptr_as_int_ptr(void *pointer) {
            return pointer;
        }
        double *get_dbl_array(unsigned long n) {
            return (double *) malloc(n * sizeof (double));
        }
        void set_doubles(double *array, unsigned long n, double d) {
            for (unsigned long i = 0; i < n; i = i + 1) {
                array[i] = d;
            }
            return;
        }
        void *return_dbl_ptr_as_void_ptr(double *ptr) {
            return ptr;
        }
        int main(void) {
            void *four_bytes = malloc(4);
            int *int_ptr = four_bytes;
            *int_ptr = -1;
            if (!check_char_ptr_argument(four_bytes, -1)) {
                return 1;
            }
            if (return_void_ptr_as_int_ptr(four_bytes) != int_ptr) {
                return 2;
            }
            double *dbl_ptr = four_bytes;
            int (*complicated_ptr)[3][2][5] = four_bytes;
            long *long_ptr = four_bytes;
            if (dbl_ptr != four_bytes || complicated_ptr != four_bytes || long_ptr != four_bytes) {
                return 3;
            }
            free(four_bytes);
            double *dbl_array = get_dbl_array(5);
            void *void_array = dbl_array;
            set_doubles(void_array, 5, 4.0);
            if (dbl_array[3] != 4.0) {
                return 4;
            }
            if (return_dbl_ptr_as_void_ptr(dbl_array) != void_array) {
                return 5;
            }
            void *some_other_ptr = 0;
            some_other_ptr = dbl_array;
            if (some_other_ptr != void_array) {
                return 6;
            }
            some_other_ptr = &some_other_ptr;
            if (some_other_ptr == void_array) {
                return 7;
            }
            complicated_ptr = 0;
            some_other_ptr = complicated_ptr;
            if (some_other_ptr) {
                return 8;
            }
            free(dbl_array);
            long *long_ptr_array[3] = {
                malloc(sizeof(long)), malloc(sizeof(long)), malloc(sizeof(long))
            };
            *long_ptr_array[0] = 100l;
            *long_ptr_array[1] = 200l;
            *long_ptr_array[2] = 300l;
            long sum = (*long_ptr_array[0] + *long_ptr_array[1] + *long_ptr_array[2]);
            if (sum != 600l) {
                return 9;
            }
            free(long_ptr_array[0]);
            free(long_ptr_array[1]);
            free(long_ptr_array[2]);
            long arr1[3] = {1, 2, 3};
            long arr2[3] = {1, 2, 3};
            long arr3[3] = {1, 2, 4};
            if (memcmp(arr1, arr2, sizeof arr1) != 0) {
                return 10;
            }
            if (memcmp(arr1, arr3, sizeof arr2) != -1) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_ptr(i.5) { 
            tmp.1 = sign_extend 3
            tmp.0 = add_ptr(i.5, index=tmp.1, scale=1)
            tmp.2 = tmp.0
            return tmp.2
            return 0
        }
        global function check_char_ptr_argument(pointer.6, expected_val.7) { 
            tmp.3 = *pointer.6
            tmp.4 = sign_extend tmp.3
            tmp.6 = sign_extend expected_val.7
            tmp.5 = tmp.4 == tmp.6
            return tmp.5
            return 0
        }
        global function return_void_ptr_as_int_ptr(pointer.8) { 
            tmp.7 = pointer.8
            return tmp.7
            return 0
        }
        global function get_dbl_array(n.9) { 
            tmp.8 = n.9 * 8UL
            tmp.9 = malloc(tmp.8)
            tmp.10 = tmp.9
            return tmp.10
            return 0
        }
        global function set_doubles(array.10, n.11, d.12) { 
            tmp.11 = sign_extend 0
            i.13 = tmp.11
        
          start_loop_0:
            tmp.12 = i.13 < n.11
            if !tmp.12 jump break_loop_0
            tmp.13 = i.13
            tmp.14 = add_ptr(array.10, index=tmp.13, scale=8)
            *tmp.14 = d.12
        
          continue_loop_0:
            tmp.16 = sign_extend 1
            tmp.15 = i.13 + tmp.16
            i.13 = tmp.15
            jump start_loop_0
        
          break_loop_0:
            return 
        
            return 0
        }
        global function return_dbl_ptr_as_void_ptr(ptr.14) { 
            tmp.17 = ptr.14
            return tmp.17
            return 0
        }
        global function main() { 
            tmp.18 = sign_extend 4
            tmp.19 = malloc(tmp.18)
            four_bytes.15 = tmp.19
            tmp.20 = four_bytes.15
            int_ptr.16 = tmp.20
            tmp.21 = - 1
            *int_ptr.16 = tmp.21
            tmp.22 = four_bytes.15
            tmp.23 = - 1
            tmp.24 = truncate tmp.23
            tmp.25 = check_char_ptr_argument(tmp.22, tmp.24)
            tmp.26 = ! tmp.25
            if !tmp.26 jump end_if_0
            return 1
        
          end_if_0:
            tmp.27 = return_void_ptr_as_int_ptr(four_bytes.15)
            tmp.28 = tmp.27 != int_ptr.16
            if !tmp.28 jump end_if_2
            return 2
        
          end_if_2:
            tmp.29 = four_bytes.15
            dbl_ptr.17 = tmp.29
            tmp.30 = four_bytes.15
            complicated_ptr.18 = tmp.30
            tmp.31 = four_bytes.15
            long_ptr.19 = tmp.31
            tmp.32 = dbl_ptr.17
            tmp.33 = tmp.32 != four_bytes.15
            if tmp.33 jump or_true_4
            tmp.36 = complicated_ptr.18
            tmp.37 = tmp.36 != four_bytes.15
            if tmp.37 jump or_true_4
            tmp.35 = 0
            jump or_end_5
        
          or_true_4:
            tmp.35 = 1
        
          or_end_5:
            if tmp.35 jump or_true_6
            tmp.40 = long_ptr.19
            tmp.41 = tmp.40 != four_bytes.15
            if tmp.41 jump or_true_6
            tmp.39 = 0
            jump or_end_7
        
          or_true_6:
            tmp.39 = 1
        
          or_end_7:
            if !tmp.39 jump end_if_8
            return 3
        
          end_if_8:
            free(four_bytes.15)
            tmp.42 = sign_extend 5
            tmp.43 = get_dbl_array(tmp.42)
            dbl_array.20 = tmp.43
            tmp.44 = dbl_array.20
            void_array.21 = tmp.44
            tmp.45 = void_array.21
            tmp.46 = sign_extend 5
            set_doubles(tmp.45, tmp.46, 4D)
            tmp.47 = sign_extend 3
            tmp.48 = add_ptr(dbl_array.20, index=tmp.47, scale=8)
            tmp.49 = *tmp.48
            tmp.50 = tmp.49 != 4D
            if !tmp.50 jump end_if_10
            return 4
        
          end_if_10:
            tmp.51 = return_dbl_ptr_as_void_ptr(dbl_array.20)
            tmp.52 = tmp.51 != void_array.21
            if !tmp.52 jump end_if_12
            return 5
        
          end_if_12:
            tmp.53 = sign_extend 0
            some_other_ptr.22 = tmp.53
            tmp.54 = dbl_array.20
            some_other_ptr.22 = tmp.54
            tmp.55 = some_other_ptr.22 != void_array.21
            if !tmp.55 jump end_if_14
            return 6
        
          end_if_14:
            tmp.56 = &some_other_ptr.22
            tmp.57 = tmp.56
            some_other_ptr.22 = tmp.57
            tmp.58 = some_other_ptr.22 == void_array.21
            if !tmp.58 jump end_if_16
            return 7
        
          end_if_16:
            tmp.59 = sign_extend 0
            complicated_ptr.18 = tmp.59
            tmp.60 = complicated_ptr.18
            some_other_ptr.22 = tmp.60
            if !some_other_ptr.22 jump end_if_18
            return 8
        
          end_if_18:
            tmp.61 = dbl_array.20
            free(tmp.61)
            tmp.62 = malloc(8UL)
            tmp.63 = tmp.62
            long_ptr_array.23[0] = tmp.63
            tmp.64 = malloc(8UL)
            tmp.65 = tmp.64
            long_ptr_array.23[8] = tmp.65
            tmp.66 = malloc(8UL)
            tmp.67 = tmp.66
            long_ptr_array.23[16] = tmp.67
            tmp.68 = &long_ptr_array.23
            tmp.69 = sign_extend 0
            tmp.70 = add_ptr(tmp.68, index=tmp.69, scale=8)
            tmp.71 = *tmp.70
            *tmp.71 = 100L
            tmp.72 = &long_ptr_array.23
            tmp.73 = sign_extend 1
            tmp.74 = add_ptr(tmp.72, index=tmp.73, scale=8)
            tmp.75 = *tmp.74
            *tmp.75 = 200L
            tmp.76 = &long_ptr_array.23
            tmp.77 = sign_extend 2
            tmp.78 = add_ptr(tmp.76, index=tmp.77, scale=8)
            tmp.79 = *tmp.78
            *tmp.79 = 300L
            tmp.80 = &long_ptr_array.23
            tmp.81 = sign_extend 0
            tmp.82 = add_ptr(tmp.80, index=tmp.81, scale=8)
            tmp.83 = *tmp.82
            tmp.84 = *tmp.83
            tmp.86 = &long_ptr_array.23
            tmp.87 = sign_extend 1
            tmp.88 = add_ptr(tmp.86, index=tmp.87, scale=8)
            tmp.89 = *tmp.88
            tmp.90 = *tmp.89
            tmp.85 = tmp.84 + tmp.90
            tmp.92 = &long_ptr_array.23
            tmp.93 = sign_extend 2
            tmp.94 = add_ptr(tmp.92, index=tmp.93, scale=8)
            tmp.95 = *tmp.94
            tmp.96 = *tmp.95
            tmp.91 = tmp.85 + tmp.96
            sum.24 = tmp.91
            tmp.97 = sum.24 != 600L
            if !tmp.97 jump end_if_20
            return 9
        
          end_if_20:
            tmp.98 = &long_ptr_array.23
            tmp.99 = sign_extend 0
            tmp.100 = add_ptr(tmp.98, index=tmp.99, scale=8)
            tmp.101 = *tmp.100
            tmp.102 = tmp.101
            free(tmp.102)
            tmp.103 = &long_ptr_array.23
            tmp.104 = sign_extend 1
            tmp.105 = add_ptr(tmp.103, index=tmp.104, scale=8)
            tmp.106 = *tmp.105
            tmp.107 = tmp.106
            free(tmp.107)
            tmp.108 = &long_ptr_array.23
            tmp.109 = sign_extend 2
            tmp.110 = add_ptr(tmp.108, index=tmp.109, scale=8)
            tmp.111 = *tmp.110
            tmp.112 = tmp.111
            free(tmp.112)
            tmp.113 = sign_extend 1
            arr1.25[0] = tmp.113
            tmp.114 = sign_extend 2
            arr1.25[8] = tmp.114
            tmp.115 = sign_extend 3
            arr1.25[16] = tmp.115
            tmp.116 = sign_extend 1
            arr2.26[0] = tmp.116
            tmp.117 = sign_extend 2
            arr2.26[8] = tmp.117
            tmp.118 = sign_extend 3
            arr2.26[16] = tmp.118
            tmp.119 = sign_extend 1
            arr3.27[0] = tmp.119
            tmp.120 = sign_extend 2
            arr3.27[8] = tmp.120
            tmp.121 = sign_extend 4
            arr3.27[16] = tmp.121
            tmp.122 = &arr1.25
            tmp.123 = tmp.122
            tmp.124 = &arr2.26
            tmp.125 = tmp.124
            tmp.126 = memcmp(tmp.123, tmp.125, 24UL)
            tmp.127 = tmp.126 != 0
            if !tmp.127 jump end_if_22
            return 10
        
          end_if_22:
            tmp.128 = &arr1.25
            tmp.129 = tmp.128
            tmp.130 = &arr3.27
            tmp.131 = tmp.130
            tmp.132 = memcmp(tmp.129, tmp.131, 24UL)
            tmp.134 = - 1
            tmp.133 = tmp.132 != tmp.134
            if !tmp.133 jump end_if_24
            return 11
        
          end_if_24:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_pointer_explicit_cast() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        void *memcpy(void *s1, void *s2, unsigned long n);
        int main(void) {
            void *ptr = malloc(4 * sizeof(double));
            double *double_ptr = (double *)ptr;
            double_ptr[2] = 10.0;
            if ((void *)double_ptr != ptr) {
                return 1;
            }
            double result = double_ptr[2];
            if (result != 10.0) {
                return 2;
            }
            unsigned long ul = (unsigned long)ptr;
            if (ul % 8) {
                return 3;
            }
            free(ptr);
            long zero = 0;
            ptr = (void *) zero;
            if (ptr) {
                return 4;
            }
            zero = (long) ptr;
            if (zero) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 4
            tmp.1 = tmp.0 * 8UL
            tmp.2 = malloc(tmp.1)
            ptr.5 = tmp.2
            tmp.3 = ptr.5
            double_ptr.6 = tmp.3
            tmp.4 = sign_extend 2
            tmp.5 = add_ptr(double_ptr.6, index=tmp.4, scale=8)
            *tmp.5 = 10D
            tmp.6 = double_ptr.6
            tmp.7 = tmp.6 != ptr.5
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = sign_extend 2
            tmp.9 = add_ptr(double_ptr.6, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            result.7 = tmp.10
            tmp.11 = result.7 != 10D
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = ptr.5
            ul.8 = tmp.12
            tmp.14 = sign_extend 8
            tmp.13 = ul.8 % tmp.14
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            free(ptr.5)
            tmp.15 = sign_extend 0
            zero.9 = tmp.15
            tmp.16 = zero.9
            ptr.5 = tmp.16
            if !ptr.5 jump end_if_6
            return 4
        
          end_if_6:
            tmp.17 = ptr.5
            zero.9 = tmp.17
            if !zero.9 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_pointer_memory_management_functions() {
    let src = r#"
        void *malloc(unsigned long size);
        void *realloc(void *ptr, unsigned long size);
        void *calloc(unsigned long nmemb, unsigned long size);
        void *aligned_alloc(unsigned long alignment, unsigned long size);
        void free(void *ptr);
        int main(void) {
            char *char_buffer = malloc(50);
            for (int i = 0; i < 50; i = i + 1) {
                char_buffer[i] = i;
            }
            char *char_buffer2 = realloc(char_buffer, 100);
            char_buffer2[75] = 11;
            for (int i = 0; i < 50; i = i + 1) {
                if ( char_buffer2[i] != i) {
                    return 1;
                }
            }
            if (char_buffer2[75] != 11) {
                return 2;
            }
            free(char_buffer2);
            double *double_buffer = calloc(10, sizeof(double));
            for (int i = 0; i < 10; i = i + 1) {
                if (double_buffer[i]) {
                    return 3;
                }
            }
            free(double_buffer);
            char_buffer = aligned_alloc(256, 256);
            if ((unsigned long) char_buffer % 256) {
                return 4;
            }
            free(char_buffer);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 50
            tmp.1 = malloc(tmp.0)
            tmp.2 = tmp.1
            char_buffer.8 = tmp.2
            i.9 = 0
        
          start_loop_0:
            tmp.3 = i.9 < 50
            if !tmp.3 jump break_loop_0
            tmp.4 = sign_extend i.9
            tmp.5 = add_ptr(char_buffer.8, index=tmp.4, scale=1)
            tmp.6 = truncate i.9
            *tmp.5 = tmp.6
        
          continue_loop_0:
            tmp.7 = i.9 + 1
            i.9 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            tmp.8 = char_buffer.8
            tmp.9 = sign_extend 100
            tmp.10 = realloc(tmp.8, tmp.9)
            tmp.11 = tmp.10
            char_buffer2.10 = tmp.11
            tmp.12 = sign_extend 75
            tmp.13 = add_ptr(char_buffer2.10, index=tmp.12, scale=1)
            tmp.14 = truncate 11
            *tmp.13 = tmp.14
            i.11 = 0
        
          start_loop_1:
            tmp.15 = i.11 < 50
            if !tmp.15 jump break_loop_1
            tmp.16 = sign_extend i.11
            tmp.17 = add_ptr(char_buffer2.10, index=tmp.16, scale=1)
            tmp.18 = *tmp.17
            tmp.19 = sign_extend tmp.18
            tmp.20 = tmp.19 != i.11
            if !tmp.20 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_1:
            tmp.21 = i.11 + 1
            i.11 = tmp.21
            jump start_loop_1
        
          break_loop_1:
            tmp.22 = sign_extend 75
            tmp.23 = add_ptr(char_buffer2.10, index=tmp.22, scale=1)
            tmp.24 = *tmp.23
            tmp.25 = sign_extend tmp.24
            tmp.26 = tmp.25 != 11
            if !tmp.26 jump end_if_2
            return 2
        
          end_if_2:
            tmp.27 = char_buffer2.10
            free(tmp.27)
            tmp.28 = sign_extend 10
            tmp.29 = calloc(tmp.28, 8UL)
            tmp.30 = tmp.29
            double_buffer.12 = tmp.30
            i.13 = 0
        
          start_loop_2:
            tmp.31 = i.13 < 10
            if !tmp.31 jump break_loop_2
            tmp.32 = sign_extend i.13
            tmp.33 = add_ptr(double_buffer.12, index=tmp.32, scale=8)
            tmp.34 = *tmp.33
            tmp.35 = tmp.34 != 0D
            if !tmp.35 jump end_if_4
            return 3
        
          end_if_4:
        
          continue_loop_2:
            tmp.36 = i.13 + 1
            i.13 = tmp.36
            jump start_loop_2
        
          break_loop_2:
            tmp.37 = double_buffer.12
            free(tmp.37)
            tmp.38 = sign_extend 256
            tmp.39 = sign_extend 256
            tmp.40 = aligned_alloc(tmp.38, tmp.39)
            tmp.41 = tmp.40
            char_buffer.8 = tmp.41
            tmp.42 = char_buffer.8
            tmp.44 = sign_extend 256
            tmp.43 = tmp.42 % tmp.44
            if !tmp.43 jump end_if_6
            return 4
        
          end_if_6:
            tmp.45 = char_buffer.8
            free(tmp.45)
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_void_pointer_simple() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        int main(void) {
            int *array = malloc(10 * sizeof (int));
            array[2] = 100;
            int result = array[2];
            free(array);
            return result;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 10
            tmp.1 = tmp.0 * 4UL
            tmp.2 = malloc(tmp.1)
            tmp.3 = tmp.2
            array.2 = tmp.3
            tmp.4 = sign_extend 2
            tmp.5 = add_ptr(array.2, index=tmp.4, scale=4)
            *tmp.5 = 100
            tmp.6 = sign_extend 2
            tmp.7 = add_ptr(array.2, index=tmp.6, scale=4)
            tmp.8 = *tmp.7
            result.3 = tmp.8
            tmp.9 = array.2
            free(tmp.9)
            return result.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
