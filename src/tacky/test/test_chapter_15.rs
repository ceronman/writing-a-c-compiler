use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_allocation_test_alignment() {
    let src = r#"
        int check_alignment(int *ptr) {
            unsigned long addr = (unsigned long) ptr;
            return (addr % 16 == 0);
        }
        int main(void)
        {
            int arr[5] = {0};
            int arr2[7] = {0};
            int arr3[2][2] = {{0}};
            if (!check_alignment(arr)) {
                return 1;
            }
            for (int i = 0; i < 5; i = i + 1)
                arr[i] = i;
            if (!check_alignment(arr2)) {
                return 2;
            }
            for (int i = 0; i < 7; i = i + 1)
                if (arr2[i])
                    return 3;
            for (int i = 0; i < 7; i = i + 1){
                arr2[i] = -i;
            }
            if (!check_alignment((int *)arr3)) {
                return 4;
            }
            for (int i = 0; i < 5; i = i + 1) {
                if (arr[i] != i) {
                    return 5;
                }
            }
            for (int i = 0; i < 2; i = i + 1)
                for (int j = 0; j < 2; j = j + 1)
                    if (arr3[i][j] != 0)
                        return 6;
            return 0;
        }
    "#;
    let expected = r#"
        global function check_alignment(ptr.0) { 
            tmp.0 = ptr.0
            addr.1 = tmp.0
            tmp.2 = sign_extend 16
            tmp.1 = addr.1 % tmp.2
            tmp.4 = sign_extend 0
            tmp.3 = tmp.1 == tmp.4
            return tmp.3
            return 0
        }
        global function main() { 
            arr.2[0] = 0
            arr.2[4] = 0
            arr.2[8] = 0
            arr.2[12] = 0
            arr.2[16] = 0
            arr2.3[0] = 0
            arr2.3[4] = 0
            arr2.3[8] = 0
            arr2.3[12] = 0
            arr2.3[16] = 0
            arr2.3[20] = 0
            arr2.3[24] = 0
            arr3.4[0] = 0
            arr3.4[4] = 0
            arr3.4[8] = 0
            arr3.4[12] = 0
            tmp.5 = &arr.2
            tmp.6 = check_alignment(tmp.5)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            i.5 = 0
        
          start_loop_0:
            tmp.8 = i.5 < 5
            if !tmp.8 jump break_loop_0
            tmp.9 = &arr.2
            tmp.10 = sign_extend i.5
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=4)
            *tmp.11 = i.5
        
          continue_loop_0:
            tmp.12 = i.5 + 1
            i.5 = tmp.12
            jump start_loop_0
        
          break_loop_0:
            tmp.13 = &arr2.3
            tmp.14 = check_alignment(tmp.13)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_2
            return 2
        
          end_if_2:
            i.6 = 0
        
          start_loop_1:
            tmp.16 = i.6 < 7
            if !tmp.16 jump break_loop_1
            tmp.17 = &arr2.3
            tmp.18 = sign_extend i.6
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=4)
            tmp.20 = *tmp.19
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
        
          continue_loop_1:
            tmp.21 = i.6 + 1
            i.6 = tmp.21
            jump start_loop_1
        
          break_loop_1:
            i.7 = 0
        
          start_loop_2:
            tmp.22 = i.7 < 7
            if !tmp.22 jump break_loop_2
            tmp.23 = &arr2.3
            tmp.24 = sign_extend i.7
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=4)
            tmp.26 = - i.7
            *tmp.25 = tmp.26
        
          continue_loop_2:
            tmp.27 = i.7 + 1
            i.7 = tmp.27
            jump start_loop_2
        
          break_loop_2:
            tmp.28 = &arr3.4
            tmp.29 = tmp.28
            tmp.30 = check_alignment(tmp.29)
            tmp.31 = ! tmp.30
            if !tmp.31 jump end_if_6
            return 4
        
          end_if_6:
            i.8 = 0
        
          start_loop_3:
            tmp.32 = i.8 < 5
            if !tmp.32 jump break_loop_3
            tmp.33 = &arr.2
            tmp.34 = sign_extend i.8
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=4)
            tmp.36 = *tmp.35
            tmp.37 = tmp.36 != i.8
            if !tmp.37 jump end_if_8
            return 5
        
          end_if_8:
        
          continue_loop_3:
            tmp.38 = i.8 + 1
            i.8 = tmp.38
            jump start_loop_3
        
          break_loop_3:
            i.9 = 0
        
          start_loop_4:
            tmp.39 = i.9 < 2
            if !tmp.39 jump break_loop_4
            j.10 = 0
        
          start_loop_5:
            tmp.40 = j.10 < 2
            if !tmp.40 jump break_loop_5
            tmp.41 = &arr3.4
            tmp.42 = sign_extend i.9
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=8)
            tmp.44 = sign_extend j.10
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=4)
            tmp.46 = *tmp.45
            tmp.47 = tmp.46 != 0
            if !tmp.47 jump end_if_10
            return 6
        
          end_if_10:
        
          continue_loop_5:
            tmp.48 = j.10 + 1
            j.10 = tmp.48
            jump start_loop_5
        
          break_loop_5:
        
          continue_loop_4:
            tmp.49 = i.9 + 1
            i.9 = tmp.49
            jump start_loop_4
        
          break_loop_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_casts_cast_array_of_pointers() {
    let src = r#"
        int main(void) {
            int simple_array[2] = {1, 2};
            int(*ptr_arr[3])[2] = {&simple_array, 0, &simple_array};
            long *other_ptr = (long *)ptr_arr;
            return (int(**)[2])other_ptr == ptr_arr;
        }
    "#;
    let expected = r#"
        global function main() { 
            simple_array.0[0] = 1
            simple_array.0[4] = 2
            tmp.0 = &simple_array.0
            ptr_arr.1[0] = tmp.0
            tmp.1 = sign_extend 0
            ptr_arr.1[8] = tmp.1
            tmp.2 = &simple_array.0
            ptr_arr.1[16] = tmp.2
            tmp.3 = &ptr_arr.1
            tmp.4 = tmp.3
            other_ptr.2 = tmp.4
            tmp.5 = other_ptr.2
            tmp.7 = &ptr_arr.1
            tmp.6 = tmp.5 == tmp.7
            return tmp.6
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_casts_implicit_and_explicit_conversions() {
    let src = r#"
        int main(void) {
            long arr[4] = {-1,-2,-3,-4};
            if (arr != (long *) arr) {
                return 1;
            }
            if ((long (*)[4]) arr != &arr) {
                return 2;
            }
            unsigned long *unsigned_arr = (unsigned long *)arr;
            if (unsigned_arr[0] != 18446744073709551615UL) {
                return 3;
            }
            if (unsigned_arr[3] != 18446744073709551612UL) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            tmp.1 = sign_extend tmp.0
            arr.0[0] = tmp.1
            tmp.2 = - 2
            tmp.3 = sign_extend tmp.2
            arr.0[8] = tmp.3
            tmp.4 = - 3
            tmp.5 = sign_extend tmp.4
            arr.0[16] = tmp.5
            tmp.6 = - 4
            tmp.7 = sign_extend tmp.6
            arr.0[24] = tmp.7
            tmp.8 = &arr.0
            tmp.10 = &arr.0
            tmp.9 = tmp.8 != tmp.10
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = &arr.0
            tmp.12 = tmp.11
            tmp.14 = &arr.0
            tmp.13 = tmp.12 != tmp.14
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            tmp.15 = &arr.0
            tmp.16 = tmp.15
            unsigned_arr.1 = tmp.16
            tmp.17 = sign_extend 0
            tmp.18 = add_ptr(unsigned_arr.1, index=tmp.17, scale=8)
            tmp.19 = *tmp.18
            tmp.20 = tmp.19 != 18446744073709551615UL
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
            tmp.21 = sign_extend 3
            tmp.22 = add_ptr(unsigned_arr.1, index=tmp.21, scale=8)
            tmp.23 = *tmp.22
            tmp.24 = tmp.23 != 18446744073709551612UL
            if !tmp.24 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_casts_multi_dim_casts() {
    let src = r#"
        int main(void) {
            int multi_dim[2][3] = {{0, 1, 2}, {3, 4, 5}};
            int (*array_pointer)[2][3] = &multi_dim;
            int (*row_pointer)[3] = (int (*)[3]) array_pointer;
            if (row_pointer != multi_dim) {
                return 1;
            }
            row_pointer = row_pointer + 1;
            if (row_pointer[0][1] != 4) {
                return 2;
            }
            int *elem_ptr = (int *) row_pointer;
            if (*elem_ptr != 3 ){
                return 3;
            }
            elem_ptr = elem_ptr + 2;
            if (*elem_ptr != 5) {
                return 4;
            }
            row_pointer = row_pointer - 1;
            if ((int (*)[2][3]) row_pointer != array_pointer) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            multi_dim.0[0] = 0
            multi_dim.0[4] = 1
            multi_dim.0[8] = 2
            multi_dim.0[12] = 3
            multi_dim.0[16] = 4
            multi_dim.0[20] = 5
            tmp.0 = &multi_dim.0
            array_pointer.1 = tmp.0
            tmp.1 = array_pointer.1
            row_pointer.2 = tmp.1
            tmp.3 = &multi_dim.0
            tmp.2 = row_pointer.2 != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = sign_extend 1
            tmp.4 = add_ptr(row_pointer.2, index=tmp.5, scale=12)
            row_pointer.2 = tmp.4
            tmp.6 = sign_extend 0
            tmp.7 = add_ptr(row_pointer.2, index=tmp.6, scale=12)
            tmp.8 = sign_extend 1
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=4)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 != 4
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = row_pointer.2
            elem_ptr.3 = tmp.12
            tmp.13 = *elem_ptr.3
            tmp.14 = tmp.13 != 3
            if !tmp.14 jump end_if_4
            return 3
        
          end_if_4:
            tmp.16 = sign_extend 2
            tmp.15 = add_ptr(elem_ptr.3, index=tmp.16, scale=4)
            elem_ptr.3 = tmp.15
            tmp.17 = *elem_ptr.3
            tmp.18 = tmp.17 != 5
            if !tmp.18 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = sign_extend 1
            tmp.21 = - tmp.20
            tmp.19 = add_ptr(row_pointer.2, index=tmp.21, scale=12)
            row_pointer.2 = tmp.19
            tmp.22 = row_pointer.2
            tmp.23 = tmp.22 != array_pointer.1
            if !tmp.23 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_array_as_argument() {
    let src = r#"
        int array_param(int a[5]) {
            a[4] = 0;
            return 0;
        }
        int nested_array_param(int a[2][3]) {
            a[1][1] = 1;
            return 0;
        }
        int array_param(int a[2]);
        int nested_array_param(int (*a)[3]);
        int main(void) {
            int array_param(int a[6]);
            int nested_array_param(int a[5][3]);
            int arr[8] = {8, 7, 6, 5, 4, 3, 2, 1};
            array_param(arr);
            if (arr[4]) {
                return 1;
            }
            for (int i = 0; i < 8; i = i + 1) {
                if (i != 4 && arr[i] != 8 - i)
                    return 2;
            }
            int nested_arr[4][3] = { {-1, -1, -1}, {-2, -2, -2}, {-3, -3, -3}, {-4, -4, -4}};
            nested_array_param(nested_arr);
            if (nested_arr[1][1] != 1) {
                return 3;
            }
            for (int i = 0; i < 4; i = i + 1) {
                int expected = -1 - i;
                for (int j = 0; j < 3; j = j + 1) {
                    if ((i != 1 || j != 1) &&
                        (nested_arr[i][j] != expected)) {
                            return 4;
                    }
                }
            }
            return 0;
        }
        int array_param(int *a);
    "#;
    let expected = r#"
        global function array_param(a.0) { 
            tmp.0 = sign_extend 4
            tmp.1 = add_ptr(a.0, index=tmp.0, scale=4)
            *tmp.1 = 0
            return 0
            return 0
        }
        global function nested_array_param(a.1) { 
            tmp.2 = sign_extend 1
            tmp.3 = add_ptr(a.1, index=tmp.2, scale=12)
            tmp.4 = sign_extend 1
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=4)
            *tmp.5 = 1
            return 0
            return 0
        }
        global function main() { 
            arr.6[0] = 8
            arr.6[4] = 7
            arr.6[8] = 6
            arr.6[12] = 5
            arr.6[16] = 4
            arr.6[20] = 3
            arr.6[24] = 2
            arr.6[28] = 1
            tmp.6 = &arr.6
            tmp.7 = array_param(tmp.6)
            tmp.8 = &arr.6
            tmp.9 = sign_extend 4
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=4)
            tmp.11 = *tmp.10
            if !tmp.11 jump end_if_0
            return 1
        
          end_if_0:
            i.7 = 0
        
          start_loop_0:
            tmp.12 = i.7 < 8
            if !tmp.12 jump break_loop_0
            tmp.13 = i.7 != 4
            if !tmp.13 jump and_false_2
            tmp.16 = &arr.6
            tmp.17 = sign_extend i.7
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=4)
            tmp.19 = *tmp.18
            tmp.21 = 8 - i.7
            tmp.20 = tmp.19 != tmp.21
            if !tmp.20 jump and_false_2
            tmp.15 = 1
            jump and_end_3
        
          and_false_2:
            tmp.15 = 0
        
          and_end_3:
            if !tmp.15 jump end_if_4
            return 2
        
          end_if_4:
        
          continue_loop_0:
            tmp.22 = i.7 + 1
            i.7 = tmp.22
            jump start_loop_0
        
          break_loop_0:
            tmp.23 = - 1
            nested_arr.8[0] = tmp.23
            tmp.24 = - 1
            nested_arr.8[4] = tmp.24
            tmp.25 = - 1
            nested_arr.8[8] = tmp.25
            tmp.26 = - 2
            nested_arr.8[12] = tmp.26
            tmp.27 = - 2
            nested_arr.8[16] = tmp.27
            tmp.28 = - 2
            nested_arr.8[20] = tmp.28
            tmp.29 = - 3
            nested_arr.8[24] = tmp.29
            tmp.30 = - 3
            nested_arr.8[28] = tmp.30
            tmp.31 = - 3
            nested_arr.8[32] = tmp.31
            tmp.32 = - 4
            nested_arr.8[36] = tmp.32
            tmp.33 = - 4
            nested_arr.8[40] = tmp.33
            tmp.34 = - 4
            nested_arr.8[44] = tmp.34
            tmp.35 = &nested_arr.8
            tmp.36 = nested_array_param(tmp.35)
            tmp.37 = &nested_arr.8
            tmp.38 = sign_extend 1
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=12)
            tmp.40 = sign_extend 1
            tmp.41 = add_ptr(tmp.39, index=tmp.40, scale=4)
            tmp.42 = *tmp.41
            tmp.43 = tmp.42 != 1
            if !tmp.43 jump end_if_6
            return 3
        
          end_if_6:
            i.9 = 0
        
          start_loop_1:
            tmp.44 = i.9 < 4
            if !tmp.44 jump break_loop_1
            tmp.45 = - 1
            tmp.46 = tmp.45 - i.9
            expected.10 = tmp.46
            j.11 = 0
        
          start_loop_2:
            tmp.47 = j.11 < 3
            if !tmp.47 jump break_loop_2
            tmp.48 = i.9 != 1
            if tmp.48 jump or_true_8
            tmp.51 = j.11 != 1
            if tmp.51 jump or_true_8
            tmp.50 = 0
            jump or_end_9
        
          or_true_8:
            tmp.50 = 1
        
          or_end_9:
            if !tmp.50 jump and_false_10
            tmp.54 = &nested_arr.8
            tmp.55 = sign_extend i.9
            tmp.56 = add_ptr(tmp.54, index=tmp.55, scale=12)
            tmp.57 = sign_extend j.11
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=4)
            tmp.59 = *tmp.58
            tmp.60 = tmp.59 != expected.10
            if !tmp.60 jump and_false_10
            tmp.53 = 1
            jump and_end_11
        
          and_false_10:
            tmp.53 = 0
        
          and_end_11:
            if !tmp.53 jump end_if_12
            return 4
        
          end_if_12:
        
          continue_loop_2:
            tmp.61 = j.11 + 1
            j.11 = tmp.61
            jump start_loop_2
        
          break_loop_2:
        
          continue_loop_1:
            tmp.62 = i.9 + 1
            i.9 = tmp.62
            jump start_loop_1
        
          break_loop_1:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_big_array() {
    let src = r#"
        extern int x[4294967297L][100000000];
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
fn test_valid_declarators_equivalent_declarators() {
    let src = r#"
        long int(arr)[4] = {1, 2, 3, 4};
        int long arr[4ul];
        int (*ptr_to_arr)[3][6l];
        int((*(ptr_to_arr))[3l])[6u] = 0;
        int *array_of_pointers[3] = {0, 0, 0};
        int test_arr(void) {
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != i + 1) {
                    return 1;
                }
            }
            return 0;
        }
        int test_ptr_to_arr(void) {
            if (ptr_to_arr) {
                return 2;
            }
            static int nested_arr[3][6];
            ptr_to_arr = &nested_arr;
            ptr_to_arr[0][2][4] = 100;
            if (nested_arr[2][4] != 100) {
                return 3;
            }
            return 0;
        }
        int test_array_of_pointers(int *ptr) {
            extern int *((array_of_pointers)[3]);
            for (int i = 0; i < 3; i = i + 1) {
                if (array_of_pointers[i])
                    return 4;
                array_of_pointers[i] = ptr;
            }
            array_of_pointers[2][0] = 11;
            if (*ptr != 11) {
                return 5;
            }
            for (int i = 0; i < 3; i = i + 1) {
                if (array_of_pointers[i][0] != 11) {
                    return 6;
                }
            }
            return 0;
        }
        int main(void)
        {
            int check = test_arr();
            if (check) {
                return check;
            }
            check = test_ptr_to_arr();
            if (check) {
                return check;
            }
            int x = 0;
            check = test_array_of_pointers(&x);
            if (check) {
                return check;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_arr() { 
            i.0 = 0
        
          start_loop_0:
            tmp.0 = i.0 < 4
            if !tmp.0 jump break_loop_0
            tmp.1 = &arr
            tmp.2 = sign_extend i.0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=8)
            tmp.4 = *tmp.3
            tmp.6 = i.0 + 1
            tmp.7 = sign_extend tmp.6
            tmp.5 = tmp.4 != tmp.7
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.8 = i.0 + 1
            i.0 = tmp.8
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
        global function test_ptr_to_arr() { 
            if !ptr_to_arr jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = &nested_arr.1
            ptr_to_arr = tmp.9
            tmp.10 = sign_extend 0
            tmp.11 = add_ptr(ptr_to_arr, index=tmp.10, scale=72)
            tmp.12 = sign_extend 2
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=24)
            tmp.14 = sign_extend 4
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=4)
            *tmp.15 = 100
            tmp.16 = &nested_arr.1
            tmp.17 = sign_extend 2
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=24)
            tmp.19 = sign_extend 4
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=4)
            tmp.21 = *tmp.20
            tmp.22 = tmp.21 != 100
            if !tmp.22 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        global function test_array_of_pointers(ptr.2) { 
            i.4 = 0
        
          start_loop_1:
            tmp.23 = i.4 < 3
            if !tmp.23 jump break_loop_1
            tmp.24 = &array_of_pointers
            tmp.25 = sign_extend i.4
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=8)
            tmp.27 = *tmp.26
            if !tmp.27 jump end_if_6
            return 4
        
          end_if_6:
            tmp.28 = &array_of_pointers
            tmp.29 = sign_extend i.4
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=8)
            *tmp.30 = ptr.2
        
          continue_loop_1:
            tmp.31 = i.4 + 1
            i.4 = tmp.31
            jump start_loop_1
        
          break_loop_1:
            tmp.32 = &array_of_pointers
            tmp.33 = sign_extend 2
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=8)
            tmp.35 = *tmp.34
            tmp.36 = sign_extend 0
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=4)
            *tmp.37 = 11
            tmp.38 = *ptr.2
            tmp.39 = tmp.38 != 11
            if !tmp.39 jump end_if_8
            return 5
        
          end_if_8:
            i.5 = 0
        
          start_loop_2:
            tmp.40 = i.5 < 3
            if !tmp.40 jump break_loop_2
            tmp.41 = &array_of_pointers
            tmp.42 = sign_extend i.5
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=8)
            tmp.44 = *tmp.43
            tmp.45 = sign_extend 0
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=4)
            tmp.47 = *tmp.46
            tmp.48 = tmp.47 != 11
            if !tmp.48 jump end_if_10
            return 6
        
          end_if_10:
        
          continue_loop_2:
            tmp.49 = i.5 + 1
            i.5 = tmp.49
            jump start_loop_2
        
          break_loop_2:
            return 0
            return 0
        }
        global function main() { 
            tmp.50 = test_arr()
            check.6 = tmp.50
            if !check.6 jump end_if_12
            return check.6
        
          end_if_12:
            tmp.51 = test_ptr_to_arr()
            check.6 = tmp.51
            if !check.6 jump end_if_14
            return check.6
        
          end_if_14:
            x.7 = 0
            tmp.52 = &x.7
            tmp.53 = test_array_of_pointers(tmp.52)
            check.6 = tmp.53
            if !check.6 jump end_if_16
            return check.6
        
          end_if_16:
            return 0
            return 0
        }
        static global arr: Array(4,Long) = [ 1L, 2L, 3L, 4L]
        static global array_of_pointers: Array(3,Pointer(Int)) = [ 0UL, 0UL, 0UL]
        static nested_arr.1: Array(3,Array(6,Int)) = zero[72]
        static global ptr_to_arr: Pointer(Array(3,Array(6,Int))) = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_for_loop_array() {
    let src = r#"
        
        int main(void) {
            int counter = 0;
            for (int i[3] = {1, 2, 3}; counter < 3; counter = counter + 1){
                if (i[counter] != counter + 1) {
                    return 1;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            counter.0 = 0
            i.1[0] = 1
            i.1[4] = 2
            i.1[8] = 3
        
          start_loop_0:
            tmp.0 = counter.0 < 3
            if !tmp.0 jump break_loop_0
            tmp.1 = &i.1
            tmp.2 = sign_extend counter.0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=4)
            tmp.4 = *tmp.3
            tmp.6 = counter.0 + 1
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.7 = counter.0 + 1
            counter.0 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declarators_return_nested_array() {
    let src = r#"
        int arr[3] = {1, 1, 1};
        int (*foo(int x, int y))[3] {
            arr[1] = x;
            arr[2] = y;
            return &arr;
        }
        int main(void) {
            int (*arr)[3] = foo(2, 3);
            if (arr[0][0] != 1) {
                return 1;
            }
            if (arr[0][1] != 2) {
                return 2;
            }
            if (arr[0][2] != 3) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function foo(x.0, y.1) { 
            tmp.0 = &arr
            tmp.1 = sign_extend 1
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=4)
            *tmp.2 = x.0
            tmp.3 = &arr
            tmp.4 = sign_extend 2
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=4)
            *tmp.5 = y.1
            tmp.6 = &arr
            return tmp.6
            return 0
        }
        global function main() { 
            tmp.7 = foo(2, 3)
            arr.2 = tmp.7
            tmp.8 = sign_extend 0
            tmp.9 = add_ptr(arr.2, index=tmp.8, scale=12)
            tmp.10 = sign_extend 0
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=4)
            tmp.12 = *tmp.11
            tmp.13 = tmp.12 != 1
            if !tmp.13 jump end_if_0
            return 1
        
          end_if_0:
            tmp.14 = sign_extend 0
            tmp.15 = add_ptr(arr.2, index=tmp.14, scale=12)
            tmp.16 = sign_extend 1
            tmp.17 = add_ptr(tmp.15, index=tmp.16, scale=4)
            tmp.18 = *tmp.17
            tmp.19 = tmp.18 != 2
            if !tmp.19 jump end_if_2
            return 2
        
          end_if_2:
            tmp.20 = sign_extend 0
            tmp.21 = add_ptr(arr.2, index=tmp.20, scale=12)
            tmp.22 = sign_extend 2
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=4)
            tmp.24 = *tmp.23
            tmp.25 = tmp.24 != 3
            if !tmp.25 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static global arr: Array(3,Int) = [ 1, 1, 1]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_subscript() {
    let src = r#"
        
        int main(void) {
            int arr[6] = {-10, 10, -11, 11, -12, 12};
            if ((arr[0] & arr[5]) != 4) {
                return 1;
            }
            if ((arr[1] | arr[4]) != -2) {
                return 2;
            }
            if ((arr[2] ^ arr[3]) != -2) {
                return 3;
            }
            arr[0] = 2041302511;
            if ((arr[0] >> arr[1]) != 1993459) {
                return 4;
            }
            if ((arr[5] << 3 ) != 96) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 10
            arr.0[0] = tmp.0
            arr.0[4] = 10
            tmp.1 = - 11
            arr.0[8] = tmp.1
            arr.0[12] = 11
            tmp.2 = - 12
            arr.0[16] = tmp.2
            arr.0[20] = 12
            tmp.3 = &arr.0
            tmp.4 = sign_extend 0
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=4)
            tmp.6 = *tmp.5
            tmp.8 = &arr.0
            tmp.9 = sign_extend 5
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=4)
            tmp.11 = *tmp.10
            tmp.7 = tmp.6 & tmp.11
            tmp.12 = tmp.7 != 4
            if !tmp.12 jump end_if_0
            return 1
        
          end_if_0:
            tmp.13 = &arr.0
            tmp.14 = sign_extend 1
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=4)
            tmp.16 = *tmp.15
            tmp.18 = &arr.0
            tmp.19 = sign_extend 4
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=4)
            tmp.21 = *tmp.20
            tmp.17 = tmp.16 | tmp.21
            tmp.23 = - 2
            tmp.22 = tmp.17 != tmp.23
            if !tmp.22 jump end_if_2
            return 2
        
          end_if_2:
            tmp.24 = &arr.0
            tmp.25 = sign_extend 2
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=4)
            tmp.27 = *tmp.26
            tmp.29 = &arr.0
            tmp.30 = sign_extend 3
            tmp.31 = add_ptr(tmp.29, index=tmp.30, scale=4)
            tmp.32 = *tmp.31
            tmp.28 = tmp.27 ^ tmp.32
            tmp.34 = - 2
            tmp.33 = tmp.28 != tmp.34
            if !tmp.33 jump end_if_4
            return 3
        
          end_if_4:
            tmp.35 = &arr.0
            tmp.36 = sign_extend 0
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=4)
            *tmp.37 = 2041302511
            tmp.38 = &arr.0
            tmp.39 = sign_extend 0
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=4)
            tmp.41 = *tmp.40
            tmp.43 = &arr.0
            tmp.44 = sign_extend 1
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=4)
            tmp.46 = *tmp.45
            tmp.42 = tmp.41 >> tmp.46
            tmp.47 = tmp.42 != 1993459
            if !tmp.47 jump end_if_6
            return 4
        
          end_if_6:
            tmp.48 = &arr.0
            tmp.49 = sign_extend 5
            tmp.50 = add_ptr(tmp.48, index=tmp.49, scale=4)
            tmp.51 = *tmp.50
            tmp.52 = tmp.51 << 3
            tmp.53 = tmp.52 != 96
            if !tmp.53 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_and_increment() {
    let src = r#"
        
        int main(void) {
            int arr[4] = {-1, -2, -3, -4};
            int *ptr = arr;
            int idx = 2;
            if ((ptr++[idx++] *= 3) != -9) {
                return 1;
            }
            if (*ptr != -2) {
                return 2;
            }
            if (idx != 3) {
                return 3;
            }
            idx--;
            if ((--ptr)[3] += 4) {
                return 4;
            }
            if (arr[0] != -1 || arr[1] != -2 || arr[2] != -9 || arr[3] != 0) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            arr.0[0] = tmp.0
            tmp.1 = - 2
            arr.0[4] = tmp.1
            tmp.2 = - 3
            arr.0[8] = tmp.2
            tmp.3 = - 4
            arr.0[12] = tmp.3
            tmp.4 = &arr.0
            ptr.1 = tmp.4
            idx.2 = 2
            tmp.5 = ptr.1
            tmp.6 = add_ptr(ptr.1, index=1L, scale=4)
            ptr.1 = tmp.6
            tmp.7 = idx.2
            tmp.8 = inc idx.2
            idx.2 = tmp.8
            tmp.9 = sign_extend tmp.7
            tmp.10 = add_ptr(tmp.5, index=tmp.9, scale=4)
            tmp.11 = *tmp.10
            tmp.12 = tmp.11 * 3
            *tmp.10 = tmp.12
            tmp.14 = - 9
            tmp.13 = tmp.12 != tmp.14
            if !tmp.13 jump end_if_0
            return 1
        
          end_if_0:
            tmp.15 = *ptr.1
            tmp.17 = - 2
            tmp.16 = tmp.15 != tmp.17
            if !tmp.16 jump end_if_2
            return 2
        
          end_if_2:
            tmp.18 = idx.2 != 3
            if !tmp.18 jump end_if_4
            return 3
        
          end_if_4:
            tmp.19 = idx.2
            tmp.20 = dec idx.2
            idx.2 = tmp.20
            tmp.21 = add_ptr(ptr.1, index=-1L, scale=4)
            ptr.1 = tmp.21
            tmp.22 = sign_extend 3
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=4)
            tmp.24 = *tmp.23
            tmp.25 = tmp.24 + 4
            *tmp.23 = tmp.25
            if !tmp.25 jump end_if_6
            return 4
        
          end_if_6:
            tmp.26 = &arr.0
            tmp.27 = sign_extend 0
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=4)
            tmp.29 = *tmp.28
            tmp.31 = - 1
            tmp.30 = tmp.29 != tmp.31
            if tmp.30 jump or_true_8
            tmp.34 = &arr.0
            tmp.35 = sign_extend 1
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=4)
            tmp.37 = *tmp.36
            tmp.39 = - 2
            tmp.38 = tmp.37 != tmp.39
            if tmp.38 jump or_true_8
            tmp.33 = 0
            jump or_end_9
        
          or_true_8:
            tmp.33 = 1
        
          or_end_9:
            if tmp.33 jump or_true_10
            tmp.42 = &arr.0
            tmp.43 = sign_extend 2
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=4)
            tmp.45 = *tmp.44
            tmp.47 = - 9
            tmp.46 = tmp.45 != tmp.47
            if tmp.46 jump or_true_10
            tmp.41 = 0
            jump or_end_11
        
          or_true_10:
            tmp.41 = 1
        
          or_end_11:
            if tmp.41 jump or_true_12
            tmp.50 = &arr.0
            tmp.51 = sign_extend 3
            tmp.52 = add_ptr(tmp.50, index=tmp.51, scale=4)
            tmp.53 = *tmp.52
            tmp.54 = tmp.53 != 0
            if tmp.54 jump or_true_12
            tmp.49 = 0
            jump or_end_13
        
          or_true_12:
            tmp.49 = 1
        
          or_end_13:
            if !tmp.49 jump end_if_14
            return 5
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_array_of_pointers() {
    let src = r#"
        
        int main(void) {
            static int (*array_of_pointers[3])[4] = {0, 0, 0};
            int array1[4] = {100, 101, 102, 103};
            int nested_array[2][4] = {
                {200, 201, 202, 203},
                {300, 301, 302, 303}
            };
            array_of_pointers[0] = &array1;
            array_of_pointers[1] = &nested_array[0];
            array_of_pointers[2] = &nested_array[1];
            array_of_pointers[0] += 1;
            if (array_of_pointers[0][-1][3] != 103) {
                return 1;
            }
            array_of_pointers[1] += 1;
            array_of_pointers[2] -= 1;
            if (array_of_pointers[1][0][3] != 303) {
                return 2;
            }
            if (array_of_pointers[2][0][3] != 203) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            array1.1[0] = 100
            array1.1[4] = 101
            array1.1[8] = 102
            array1.1[12] = 103
            nested_array.2[0] = 200
            nested_array.2[4] = 201
            nested_array.2[8] = 202
            nested_array.2[12] = 203
            nested_array.2[16] = 300
            nested_array.2[20] = 301
            nested_array.2[24] = 302
            nested_array.2[28] = 303
            tmp.0 = &array_of_pointers.0
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=8)
            tmp.3 = &array1.1
            *tmp.2 = tmp.3
            tmp.4 = &array_of_pointers.0
            tmp.5 = sign_extend 1
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=8)
            tmp.7 = &nested_array.2
            tmp.8 = sign_extend 0
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=16)
            *tmp.6 = tmp.9
            tmp.10 = &array_of_pointers.0
            tmp.11 = sign_extend 2
            tmp.12 = add_ptr(tmp.10, index=tmp.11, scale=8)
            tmp.13 = &nested_array.2
            tmp.14 = sign_extend 1
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=16)
            *tmp.12 = tmp.15
            tmp.16 = &array_of_pointers.0
            tmp.17 = sign_extend 0
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=8)
            tmp.19 = *tmp.18
            tmp.21 = sign_extend 1
            tmp.20 = add_ptr(tmp.19, index=tmp.21, scale=16)
            *tmp.18 = tmp.20
            tmp.22 = &array_of_pointers.0
            tmp.23 = sign_extend 0
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=8)
            tmp.25 = *tmp.24
            tmp.26 = - 1
            tmp.27 = sign_extend tmp.26
            tmp.28 = add_ptr(tmp.25, index=tmp.27, scale=16)
            tmp.29 = sign_extend 3
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=4)
            tmp.31 = *tmp.30
            tmp.32 = tmp.31 != 103
            if !tmp.32 jump end_if_0
            return 1
        
          end_if_0:
            tmp.33 = &array_of_pointers.0
            tmp.34 = sign_extend 1
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=8)
            tmp.36 = *tmp.35
            tmp.38 = sign_extend 1
            tmp.37 = add_ptr(tmp.36, index=tmp.38, scale=16)
            *tmp.35 = tmp.37
            tmp.39 = &array_of_pointers.0
            tmp.40 = sign_extend 2
            tmp.41 = add_ptr(tmp.39, index=tmp.40, scale=8)
            tmp.42 = *tmp.41
            tmp.44 = sign_extend 1
            tmp.45 = - tmp.44
            tmp.43 = add_ptr(tmp.42, index=tmp.45, scale=16)
            *tmp.41 = tmp.43
            tmp.46 = &array_of_pointers.0
            tmp.47 = sign_extend 1
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=8)
            tmp.49 = *tmp.48
            tmp.50 = sign_extend 0
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=16)
            tmp.52 = sign_extend 3
            tmp.53 = add_ptr(tmp.51, index=tmp.52, scale=4)
            tmp.54 = *tmp.53
            tmp.55 = tmp.54 != 303
            if !tmp.55 jump end_if_2
            return 2
        
          end_if_2:
            tmp.56 = &array_of_pointers.0
            tmp.57 = sign_extend 2
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=8)
            tmp.59 = *tmp.58
            tmp.60 = sign_extend 0
            tmp.61 = add_ptr(tmp.59, index=tmp.60, scale=16)
            tmp.62 = sign_extend 3
            tmp.63 = add_ptr(tmp.61, index=tmp.62, scale=4)
            tmp.64 = *tmp.63
            tmp.65 = tmp.64 != 203
            if !tmp.65 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static array_of_pointers.0: Array(3,Pointer(Array(4,Int))) = [ 0UL, 0UL, 0UL]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_to_nested_subscript() {
    let src = r#"
        long long_nested_arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
        double dbl_nested_arr[3][2] = {{100.0, 101.0}, {102.0, 103.0}, {104.0, 105.0}};
        unsigned unsigned_index = 10;
        int main(void) {
            if ((long_nested_arr[1][unsigned_index - 8] *= -1) != -6) {
                return 1;
            }
            if (long_nested_arr[1][2] != -6) {
                return 2;
            }
            for (int i = 0; i < 2; i += 1) {
                for (int j = 0; j < 3; j += 1) {
                    if (i == 1 && j == 2) {
                        break;
                    }
                    long expected = i * 3 + j + 1;
                    if (long_nested_arr[i][j] != expected) {
                        return 3;
                    }
                }
            }
            if ((dbl_nested_arr[1][1] += 100.0) != 203.0) {
                return 4;
            }
            for (int i = 0; i < 3; i += 1) {
                for (int j = 0; j < 2; j += 1) {
                    if (i == 1 && j == 1) {
                        continue;
                    }
                    int expected = 100 + i * 2 + j;
                    if (dbl_nested_arr[i][j] != expected) {
                        return 5;
                    }
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &long_nested_arr
            tmp.1 = sign_extend 1
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=24)
            tmp.4 = 8
            tmp.3 = unsigned_index - tmp.4
            tmp.5 = zero_extend tmp.3
            tmp.6 = add_ptr(tmp.2, index=tmp.5, scale=8)
            tmp.7 = *tmp.6
            tmp.9 = - 1
            tmp.10 = sign_extend tmp.9
            tmp.8 = tmp.7 * tmp.10
            *tmp.6 = tmp.8
            tmp.12 = - 6
            tmp.13 = sign_extend tmp.12
            tmp.11 = tmp.8 != tmp.13
            if !tmp.11 jump end_if_0
            return 1
        
          end_if_0:
            tmp.14 = &long_nested_arr
            tmp.15 = sign_extend 1
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=24)
            tmp.17 = sign_extend 2
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=8)
            tmp.19 = *tmp.18
            tmp.21 = - 6
            tmp.22 = sign_extend tmp.21
            tmp.20 = tmp.19 != tmp.22
            if !tmp.20 jump end_if_2
            return 2
        
          end_if_2:
            i.0 = 0
        
          start_loop_0:
            tmp.23 = i.0 < 2
            if !tmp.23 jump break_loop_0
            j.1 = 0
        
          start_loop_1:
            tmp.24 = j.1 < 3
            if !tmp.24 jump break_loop_1
            tmp.25 = i.0 == 1
            if !tmp.25 jump and_false_4
            tmp.28 = j.1 == 2
            if !tmp.28 jump and_false_4
            tmp.27 = 1
            jump and_end_5
        
          and_false_4:
            tmp.27 = 0
        
          and_end_5:
            if !tmp.27 jump end_if_6
            jump break_loop_1
        
          end_if_6:
            tmp.29 = i.0 * 3
            tmp.30 = tmp.29 + j.1
            tmp.31 = tmp.30 + 1
            tmp.32 = sign_extend tmp.31
            expected.2 = tmp.32
            tmp.33 = &long_nested_arr
            tmp.34 = sign_extend i.0
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=24)
            tmp.36 = sign_extend j.1
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=8)
            tmp.38 = *tmp.37
            tmp.39 = tmp.38 != expected.2
            if !tmp.39 jump end_if_8
            return 3
        
          end_if_8:
        
          continue_loop_1:
            tmp.40 = j.1 + 1
            j.1 = tmp.40
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.41 = i.0 + 1
            i.0 = tmp.41
            jump start_loop_0
        
          break_loop_0:
            tmp.42 = &dbl_nested_arr
            tmp.43 = sign_extend 1
            tmp.44 = add_ptr(tmp.42, index=tmp.43, scale=16)
            tmp.45 = sign_extend 1
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=8)
            tmp.47 = *tmp.46
            tmp.48 = tmp.47 + 100D
            *tmp.46 = tmp.48
            tmp.49 = tmp.48 != 203D
            if !tmp.49 jump end_if_10
            return 4
        
          end_if_10:
            i.3 = 0
        
          start_loop_2:
            tmp.50 = i.3 < 3
            if !tmp.50 jump break_loop_2
            j.4 = 0
        
          start_loop_3:
            tmp.51 = j.4 < 2
            if !tmp.51 jump break_loop_3
            tmp.52 = i.3 == 1
            if !tmp.52 jump and_false_12
            tmp.55 = j.4 == 1
            if !tmp.55 jump and_false_12
            tmp.54 = 1
            jump and_end_13
        
          and_false_12:
            tmp.54 = 0
        
          and_end_13:
            if !tmp.54 jump end_if_14
            jump continue_loop_3
        
          end_if_14:
            tmp.57 = i.3 * 2
            tmp.56 = 100 + tmp.57
            tmp.58 = tmp.56 + j.4
            expected.5 = tmp.58
            tmp.59 = &dbl_nested_arr
            tmp.60 = sign_extend i.3
            tmp.61 = add_ptr(tmp.59, index=tmp.60, scale=16)
            tmp.62 = sign_extend j.4
            tmp.63 = add_ptr(tmp.61, index=tmp.62, scale=8)
            tmp.64 = *tmp.63
            tmp.66 = int_to_double expected.5
            tmp.65 = tmp.64 != tmp.66
            if !tmp.65 jump end_if_16
            return 5
        
          end_if_16:
        
          continue_loop_3:
            tmp.67 = j.4 + 1
            j.4 = tmp.67
            jump start_loop_3
        
          break_loop_3:
        
          continue_loop_2:
            tmp.68 = i.3 + 1
            i.3 = tmp.68
            jump start_loop_2
        
          break_loop_2:
            return 0
            return 0
        }
        static global dbl_nested_arr: Array(3,Array(2,Double)) = [ 100D, 101D, 102D, 103D, 104D, 105D]
        static global long_nested_arr: Array(2,Array(3,Long)) = [ 1L, 2L, 3L, 4L, 5L, 6L]
        static global unsigned_index: Unsigned Int = 10U
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_to_subscripted_val() {
    let src = r#"
        unsigned unsigned_arr[4] = {4294967295U, 4294967294U, 4294967293U, 4294967292U};
        int idx = 2;
        long long_idx = 1;
        int main(void) {
            long_idx = -long_idx;
            unsigned_arr[1] += 2;
            if (unsigned_arr[1]) {
                return 1;
            }
            unsigned_arr[idx] -= 10.0;
            if (unsigned_arr[idx] != 4294967283U) {
                return 2;
            }
            unsigned *unsigned_ptr = unsigned_arr + 4;
            unsigned_ptr[long_idx] /= 10;
            if (unsigned_arr[3] != 429496729U) {
                return 3;
            }
            unsigned_ptr[long_idx *= 2] *= unsigned_arr[0];
            if (unsigned_arr[2] != 13) {
                return 4;
            }
            if ((unsigned_arr[idx + long_idx] %= 10) != 5) {
                return 5;
            }
            if (unsigned_arr[0] != 5u) {
                return 6;
            }
            if (unsigned_arr[1]) {
                return 7;
            }
            if (unsigned_arr[2] != 13) {
                return 8;
            }
            if (unsigned_arr[3] != 429496729U) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - long_idx
            long_idx = tmp.0
            tmp.1 = &unsigned_arr
            tmp.2 = sign_extend 1
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=4)
            tmp.4 = *tmp.3
            tmp.6 = 2
            tmp.5 = tmp.4 + tmp.6
            *tmp.3 = tmp.5
            tmp.7 = &unsigned_arr
            tmp.8 = sign_extend 1
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=4)
            tmp.10 = *tmp.9
            if !tmp.10 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = &unsigned_arr
            tmp.12 = sign_extend idx
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=4)
            tmp.14 = *tmp.13
            tmp.15 = uint_to_double tmp.14
            tmp.16 = tmp.15 - 10D
            tmp.17 = double_to_uint tmp.16
            *tmp.13 = tmp.17
            tmp.18 = double_to_uint tmp.17
            tmp.19 = &unsigned_arr
            tmp.20 = sign_extend idx
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=4)
            tmp.22 = *tmp.21
            tmp.23 = tmp.22 != 4294967283U
            if !tmp.23 jump end_if_2
            return 2
        
          end_if_2:
            tmp.24 = &unsigned_arr
            tmp.26 = sign_extend 4
            tmp.25 = add_ptr(tmp.24, index=tmp.26, scale=4)
            unsigned_ptr.0 = tmp.25
            tmp.27 = add_ptr(unsigned_ptr.0, index=long_idx, scale=4)
            tmp.28 = *tmp.27
            tmp.30 = 10
            tmp.29 = tmp.28 / tmp.30
            *tmp.27 = tmp.29
            tmp.31 = &unsigned_arr
            tmp.32 = sign_extend 3
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=4)
            tmp.34 = *tmp.33
            tmp.35 = tmp.34 != 429496729U
            if !tmp.35 jump end_if_4
            return 3
        
          end_if_4:
            tmp.37 = sign_extend 2
            tmp.36 = long_idx * tmp.37
            long_idx = tmp.36
            tmp.38 = add_ptr(unsigned_ptr.0, index=tmp.36, scale=4)
            tmp.39 = *tmp.38
            tmp.41 = &unsigned_arr
            tmp.42 = sign_extend 0
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=4)
            tmp.44 = *tmp.43
            tmp.40 = tmp.39 * tmp.44
            *tmp.38 = tmp.40
            tmp.45 = &unsigned_arr
            tmp.46 = sign_extend 2
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=4)
            tmp.48 = *tmp.47
            tmp.50 = 13
            tmp.49 = tmp.48 != tmp.50
            if !tmp.49 jump end_if_6
            return 4
        
          end_if_6:
            tmp.51 = &unsigned_arr
            tmp.52 = sign_extend idx
            tmp.53 = tmp.52 + long_idx
            tmp.54 = add_ptr(tmp.51, index=tmp.53, scale=4)
            tmp.55 = *tmp.54
            tmp.57 = 10
            tmp.56 = tmp.55 % tmp.57
            *tmp.54 = tmp.56
            tmp.59 = 5
            tmp.58 = tmp.56 != tmp.59
            if !tmp.58 jump end_if_8
            return 5
        
          end_if_8:
            tmp.60 = &unsigned_arr
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=4)
            tmp.63 = *tmp.62
            tmp.64 = tmp.63 != 5U
            if !tmp.64 jump end_if_10
            return 6
        
          end_if_10:
            tmp.65 = &unsigned_arr
            tmp.66 = sign_extend 1
            tmp.67 = add_ptr(tmp.65, index=tmp.66, scale=4)
            tmp.68 = *tmp.67
            if !tmp.68 jump end_if_12
            return 7
        
          end_if_12:
            tmp.69 = &unsigned_arr
            tmp.70 = sign_extend 2
            tmp.71 = add_ptr(tmp.69, index=tmp.70, scale=4)
            tmp.72 = *tmp.71
            tmp.74 = 13
            tmp.73 = tmp.72 != tmp.74
            if !tmp.73 jump end_if_14
            return 8
        
          end_if_14:
            tmp.75 = &unsigned_arr
            tmp.76 = sign_extend 3
            tmp.77 = add_ptr(tmp.75, index=tmp.76, scale=4)
            tmp.78 = *tmp.77
            tmp.79 = tmp.78 != 429496729U
            if !tmp.79 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
        static global idx: Int = 2
        static global long_idx: Long = 1L
        static global unsigned_arr: Array(4,Unsigned Int) = [ 4294967295U, 4294967294U, 4294967293U, 4294967292U]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_bitwise_subscript() {
    let src = r#"
        
        int main(void) {
            unsigned long arr[4] = {
                2147483648l,
                18446744069414584320ul,
                9223372036854775808ul,
                1085102592571150095l
            };
            arr[1] &= arr[3];
            if (arr[1] != 1085102592318504960 ) {
                return 1;
            }
            arr[0] |= arr[1];
            if (arr[0] != 1085102594465988608ul) {
                return 2;
            }
            arr[2] ^= arr[3];
            if (arr[2] != 10308474629425925903ul) {
                return 3;
            }
            arr[3] >>= 25;
            if (arr[3] != 32338577287l) {
                return 4;
            }
            arr[1] <<= 12;
            if (arr[1] != 17361640446303928320ul) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 2147483648L
            arr.0[0] = tmp.0
            arr.0[8] = 18446744069414584320UL
            arr.0[16] = 9223372036854775808UL
            tmp.1 = 1085102592571150095L
            arr.0[24] = tmp.1
            tmp.2 = &arr.0
            tmp.3 = sign_extend 1
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=8)
            tmp.5 = *tmp.4
            tmp.7 = &arr.0
            tmp.8 = sign_extend 3
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            tmp.6 = tmp.5 & tmp.10
            *tmp.4 = tmp.6
            tmp.11 = &arr.0
            tmp.12 = sign_extend 1
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=8)
            tmp.14 = *tmp.13
            tmp.16 = 1085102592318504960L
            tmp.15 = tmp.14 != tmp.16
            if !tmp.15 jump end_if_0
            return 1
        
          end_if_0:
            tmp.17 = &arr.0
            tmp.18 = sign_extend 0
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=8)
            tmp.20 = *tmp.19
            tmp.22 = &arr.0
            tmp.23 = sign_extend 1
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=8)
            tmp.25 = *tmp.24
            tmp.21 = tmp.20 | tmp.25
            *tmp.19 = tmp.21
            tmp.26 = &arr.0
            tmp.27 = sign_extend 0
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=8)
            tmp.29 = *tmp.28
            tmp.30 = tmp.29 != 1085102594465988608UL
            if !tmp.30 jump end_if_2
            return 2
        
          end_if_2:
            tmp.31 = &arr.0
            tmp.32 = sign_extend 2
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=8)
            tmp.34 = *tmp.33
            tmp.36 = &arr.0
            tmp.37 = sign_extend 3
            tmp.38 = add_ptr(tmp.36, index=tmp.37, scale=8)
            tmp.39 = *tmp.38
            tmp.35 = tmp.34 ^ tmp.39
            *tmp.33 = tmp.35
            tmp.40 = &arr.0
            tmp.41 = sign_extend 2
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=8)
            tmp.43 = *tmp.42
            tmp.44 = tmp.43 != 10308474629425925903UL
            if !tmp.44 jump end_if_4
            return 3
        
          end_if_4:
            tmp.45 = &arr.0
            tmp.46 = sign_extend 3
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=8)
            tmp.48 = *tmp.47
            tmp.50 = sign_extend 25
            tmp.49 = tmp.48 >> tmp.50
            *tmp.47 = tmp.49
            tmp.51 = &arr.0
            tmp.52 = sign_extend 3
            tmp.53 = add_ptr(tmp.51, index=tmp.52, scale=8)
            tmp.54 = *tmp.53
            tmp.56 = 32338577287L
            tmp.55 = tmp.54 != tmp.56
            if !tmp.55 jump end_if_6
            return 4
        
          end_if_6:
            tmp.57 = &arr.0
            tmp.58 = sign_extend 1
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=8)
            tmp.60 = *tmp.59
            tmp.62 = sign_extend 12
            tmp.61 = tmp.60 << tmp.62
            *tmp.59 = tmp.61
            tmp.63 = &arr.0
            tmp.64 = sign_extend 1
            tmp.65 = add_ptr(tmp.63, index=tmp.64, scale=8)
            tmp.66 = *tmp.65
            tmp.67 = tmp.66 != 17361640446303928320UL
            if !tmp.67 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_lval_evaluated_once() {
    let src = r#"
        int get_call_count(void) {
            static int count = 0;
            count += 1;
            return count;
        }
        int main(void) {
            int arr[4] = {10, 11, 12, 13};
            if (arr[get_call_count()] != 11) {
                return 1;
            }
            int *end_ptr = arr + 4;
            if ((end_ptr - 1)[-get_call_count()] != 11) {
                return 2;
            }
            if (get_call_count() != 3) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_call_count() { 
            tmp.0 = count.0 + 1
            count.0 = tmp.0
            return count.0
            return 0
        }
        global function main() { 
            arr.1[0] = 10
            arr.1[4] = 11
            arr.1[8] = 12
            arr.1[12] = 13
            tmp.1 = &arr.1
            tmp.2 = get_call_count()
            tmp.3 = sign_extend tmp.2
            tmp.4 = add_ptr(tmp.1, index=tmp.3, scale=4)
            tmp.5 = *tmp.4
            tmp.6 = tmp.5 != 11
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = &arr.1
            tmp.9 = sign_extend 4
            tmp.8 = add_ptr(tmp.7, index=tmp.9, scale=4)
            end_ptr.2 = tmp.8
            tmp.11 = sign_extend 1
            tmp.12 = - tmp.11
            tmp.10 = add_ptr(end_ptr.2, index=tmp.12, scale=4)
            tmp.13 = get_call_count()
            tmp.14 = - tmp.13
            tmp.15 = sign_extend tmp.14
            tmp.16 = add_ptr(tmp.10, index=tmp.15, scale=4)
            tmp.17 = *tmp.16
            tmp.18 = tmp.17 != 11
            if !tmp.18 jump end_if_2
            return 2
        
          end_if_2:
            tmp.19 = get_call_count()
            tmp.20 = tmp.19 != 3
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static count.0: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_nested_pointer_assignment() {
    let src = r#"
        static long nested_arr[3][4][5] = {{{10, 9, 8}, {1, 2}}, {{100, 99, 98}}};
        int main(void) {
            long(*outer_ptr)[4][5] = nested_arr;
            outer_ptr += 1;
            if (outer_ptr != nested_arr + 1) {
                return 1;
            }
            if (outer_ptr[0][0][0] != 100) {
                return 2;
            }
            long(*inner_ptr)[5] =
                nested_arr[0] + 4;
            inner_ptr -= 3;
            if (inner_ptr[0][1] != 2) {
                return 3;
            }
            unsigned long idx = nested_arr[0][0][0] - 9;
            if ((inner_ptr += idx) != &nested_arr[0][2]) {
                return 4;
            }
            if ((inner_ptr[-2][1] != 9)) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = &nested_arr
            outer_ptr.0 = tmp.0
            tmp.2 = sign_extend 1
            tmp.1 = add_ptr(outer_ptr.0, index=tmp.2, scale=160)
            outer_ptr.0 = tmp.1
            tmp.4 = &nested_arr
            tmp.6 = sign_extend 1
            tmp.5 = add_ptr(tmp.4, index=tmp.6, scale=160)
            tmp.3 = outer_ptr.0 != tmp.5
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = sign_extend 0
            tmp.8 = add_ptr(outer_ptr.0, index=tmp.7, scale=160)
            tmp.9 = sign_extend 0
            tmp.10 = add_ptr(tmp.8, index=tmp.9, scale=40)
            tmp.11 = sign_extend 0
            tmp.12 = add_ptr(tmp.10, index=tmp.11, scale=8)
            tmp.13 = *tmp.12
            tmp.15 = sign_extend 100
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_2
            return 2
        
          end_if_2:
            tmp.16 = &nested_arr
            tmp.17 = sign_extend 0
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=160)
            tmp.20 = sign_extend 4
            tmp.19 = add_ptr(tmp.18, index=tmp.20, scale=40)
            inner_ptr.1 = tmp.19
            tmp.22 = sign_extend 3
            tmp.23 = - tmp.22
            tmp.21 = add_ptr(inner_ptr.1, index=tmp.23, scale=40)
            inner_ptr.1 = tmp.21
            tmp.24 = sign_extend 0
            tmp.25 = add_ptr(inner_ptr.1, index=tmp.24, scale=40)
            tmp.26 = sign_extend 1
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=8)
            tmp.28 = *tmp.27
            tmp.30 = sign_extend 2
            tmp.29 = tmp.28 != tmp.30
            if !tmp.29 jump end_if_4
            return 3
        
          end_if_4:
            tmp.31 = &nested_arr
            tmp.32 = sign_extend 0
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=160)
            tmp.34 = sign_extend 0
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=40)
            tmp.36 = sign_extend 0
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=8)
            tmp.38 = *tmp.37
            tmp.40 = sign_extend 9
            tmp.39 = tmp.38 - tmp.40
            tmp.41 = tmp.39
            idx.2 = tmp.41
            tmp.43 = idx.2
            tmp.42 = add_ptr(inner_ptr.1, index=tmp.43, scale=40)
            inner_ptr.1 = tmp.42
            tmp.45 = &nested_arr
            tmp.46 = sign_extend 0
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=160)
            tmp.48 = sign_extend 2
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=40)
            tmp.44 = tmp.42 != tmp.49
            if !tmp.44 jump end_if_6
            return 4
        
          end_if_6:
            tmp.50 = - 2
            tmp.51 = sign_extend tmp.50
            tmp.52 = add_ptr(inner_ptr.1, index=tmp.51, scale=40)
            tmp.53 = sign_extend 1
            tmp.54 = add_ptr(tmp.52, index=tmp.53, scale=8)
            tmp.55 = *tmp.54
            tmp.57 = sign_extend 9
            tmp.56 = tmp.55 != tmp.57
            if !tmp.56 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static nested_arr: Array(3,Array(4,Array(5,Long))) = [ 10L, 9L, 8L, zero[16], 1L, 2L, zero[24], zero[80], 100L, 99L, 98L, zero[16], zero[120], zero[160]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_pointer_assignment() {
    let src = r#"
        int i = 4;
        int int_array(void) {
            int arr[6] = {1, 2, 3, 4, 5, 6};
            int *ptr = arr;
            if (*(ptr += 5) != 6) {
                return 1;
            }
            if (ptr[0] != 6) {
                 return 2;
            }
            if (ptr != arr + 5) {
                return 3;
            }
            if (*(ptr -=3) != 3) {
                return 4;
            }
            if (ptr[0] != 3) {
                return 5;
            }
            if (ptr != arr + 2) {
                return 6;
            }
            if ((ptr += i - 1) != arr + 5) {
                return 7;
            }
            if (*ptr != 6) {
                return 8;
            }
            if ((ptr -= (4294967295U + i)) != arr + 2) {
                return 9;
            }
            if (*ptr != 3) {
                return 10;
            }
            long l = 9223372036854775807l;
            if ((ptr += l - 9223372036854775806l) != arr + 3) {
                return 11;
            }
            if (*ptr != 4) {
                return 12;
            }
            return 0;
        }
        int double_array(void) {
            static double arr[6] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0};
            double *ptr = arr;
            if (*(ptr += 5) != 6) {
                return 1;
            }
            if (ptr[0] != 6) {
                 return 2;
            }
            if (ptr != arr + 5) {
                return 3;
            }
            if (*(ptr -=3) != 3) {
                return 4;
            }
            if (ptr[0] != 3) {
                return 5;
            }
            if (ptr != arr + 2) {
                return 6;
            }
            if ((ptr += i - 1) != arr + 5) {
                return 7;
            }
            if (*ptr != 6) {
                return 8;
            }
            if ((ptr -= (4294967295U + i)) != arr + 2) {
                return 9;
            }
            if (*ptr != 3) {
                return 10;
            }
            long l = 9223372036854775807l;
            if ((ptr += l - 9223372036854775806l) != arr + 3) {
                return 11;
            }
            if (*ptr != 4) {
                return 12;
            }
            return 0;
        }
        int main(void) {
            int result;
            if ((result = int_array())) {
                return result;
            }
            if ((result = double_array())) {
                return result + 12;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function int_array() { 
            arr.0[0] = 1
            arr.0[4] = 2
            arr.0[8] = 3
            arr.0[12] = 4
            arr.0[16] = 5
            arr.0[20] = 6
            tmp.0 = &arr.0
            ptr.1 = tmp.0
            tmp.2 = sign_extend 5
            tmp.1 = add_ptr(ptr.1, index=tmp.2, scale=4)
            ptr.1 = tmp.1
            tmp.3 = *tmp.1
            tmp.4 = tmp.3 != 6
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = sign_extend 0
            tmp.6 = add_ptr(ptr.1, index=tmp.5, scale=4)
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 != 6
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.10 = &arr.0
            tmp.12 = sign_extend 5
            tmp.11 = add_ptr(tmp.10, index=tmp.12, scale=4)
            tmp.9 = ptr.1 != tmp.11
            if !tmp.9 jump end_if_4
            return 3
        
          end_if_4:
            tmp.14 = sign_extend 3
            tmp.15 = - tmp.14
            tmp.13 = add_ptr(ptr.1, index=tmp.15, scale=4)
            ptr.1 = tmp.13
            tmp.16 = *tmp.13
            tmp.17 = tmp.16 != 3
            if !tmp.17 jump end_if_6
            return 4
        
          end_if_6:
            tmp.18 = sign_extend 0
            tmp.19 = add_ptr(ptr.1, index=tmp.18, scale=4)
            tmp.20 = *tmp.19
            tmp.21 = tmp.20 != 3
            if !tmp.21 jump end_if_8
            return 5
        
          end_if_8:
            tmp.23 = &arr.0
            tmp.25 = sign_extend 2
            tmp.24 = add_ptr(tmp.23, index=tmp.25, scale=4)
            tmp.22 = ptr.1 != tmp.24
            if !tmp.22 jump end_if_10
            return 6
        
          end_if_10:
            tmp.27 = i - 1
            tmp.28 = sign_extend tmp.27
            tmp.26 = add_ptr(ptr.1, index=tmp.28, scale=4)
            ptr.1 = tmp.26
            tmp.30 = &arr.0
            tmp.32 = sign_extend 5
            tmp.31 = add_ptr(tmp.30, index=tmp.32, scale=4)
            tmp.29 = tmp.26 != tmp.31
            if !tmp.29 jump end_if_12
            return 7
        
          end_if_12:
            tmp.33 = *ptr.1
            tmp.34 = tmp.33 != 6
            if !tmp.34 jump end_if_14
            return 8
        
          end_if_14:
            tmp.37 = i
            tmp.36 = 4294967295U + tmp.37
            tmp.38 = zero_extend tmp.36
            tmp.39 = - tmp.38
            tmp.35 = add_ptr(ptr.1, index=tmp.39, scale=4)
            ptr.1 = tmp.35
            tmp.41 = &arr.0
            tmp.43 = sign_extend 2
            tmp.42 = add_ptr(tmp.41, index=tmp.43, scale=4)
            tmp.40 = tmp.35 != tmp.42
            if !tmp.40 jump end_if_16
            return 9
        
          end_if_16:
            tmp.44 = *ptr.1
            tmp.45 = tmp.44 != 3
            if !tmp.45 jump end_if_18
            return 10
        
          end_if_18:
            l.2 = 9223372036854775807L
            tmp.47 = l.2 - 9223372036854775806L
            tmp.48 = tmp.47
            tmp.46 = add_ptr(ptr.1, index=tmp.48, scale=4)
            ptr.1 = tmp.46
            tmp.50 = &arr.0
            tmp.52 = sign_extend 3
            tmp.51 = add_ptr(tmp.50, index=tmp.52, scale=4)
            tmp.49 = tmp.46 != tmp.51
            if !tmp.49 jump end_if_20
            return 11
        
          end_if_20:
            tmp.53 = *ptr.1
            tmp.54 = tmp.53 != 4
            if !tmp.54 jump end_if_22
            return 12
        
          end_if_22:
            return 0
            return 0
        }
        global function double_array() { 
            tmp.55 = &arr.3
            ptr.4 = tmp.55
            tmp.57 = sign_extend 5
            tmp.56 = add_ptr(ptr.4, index=tmp.57, scale=8)
            ptr.4 = tmp.56
            tmp.58 = *tmp.56
            tmp.60 = int_to_double 6
            tmp.59 = tmp.58 != tmp.60
            if !tmp.59 jump end_if_24
            return 1
        
          end_if_24:
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(ptr.4, index=tmp.61, scale=8)
            tmp.63 = *tmp.62
            tmp.65 = int_to_double 6
            tmp.64 = tmp.63 != tmp.65
            if !tmp.64 jump end_if_26
            return 2
        
          end_if_26:
            tmp.67 = &arr.3
            tmp.69 = sign_extend 5
            tmp.68 = add_ptr(tmp.67, index=tmp.69, scale=8)
            tmp.66 = ptr.4 != tmp.68
            if !tmp.66 jump end_if_28
            return 3
        
          end_if_28:
            tmp.71 = sign_extend 3
            tmp.72 = - tmp.71
            tmp.70 = add_ptr(ptr.4, index=tmp.72, scale=8)
            ptr.4 = tmp.70
            tmp.73 = *tmp.70
            tmp.75 = int_to_double 3
            tmp.74 = tmp.73 != tmp.75
            if !tmp.74 jump end_if_30
            return 4
        
          end_if_30:
            tmp.76 = sign_extend 0
            tmp.77 = add_ptr(ptr.4, index=tmp.76, scale=8)
            tmp.78 = *tmp.77
            tmp.80 = int_to_double 3
            tmp.79 = tmp.78 != tmp.80
            if !tmp.79 jump end_if_32
            return 5
        
          end_if_32:
            tmp.82 = &arr.3
            tmp.84 = sign_extend 2
            tmp.83 = add_ptr(tmp.82, index=tmp.84, scale=8)
            tmp.81 = ptr.4 != tmp.83
            if !tmp.81 jump end_if_34
            return 6
        
          end_if_34:
            tmp.86 = i - 1
            tmp.87 = sign_extend tmp.86
            tmp.85 = add_ptr(ptr.4, index=tmp.87, scale=8)
            ptr.4 = tmp.85
            tmp.89 = &arr.3
            tmp.91 = sign_extend 5
            tmp.90 = add_ptr(tmp.89, index=tmp.91, scale=8)
            tmp.88 = tmp.85 != tmp.90
            if !tmp.88 jump end_if_36
            return 7
        
          end_if_36:
            tmp.92 = *ptr.4
            tmp.94 = int_to_double 6
            tmp.93 = tmp.92 != tmp.94
            if !tmp.93 jump end_if_38
            return 8
        
          end_if_38:
            tmp.97 = i
            tmp.96 = 4294967295U + tmp.97
            tmp.98 = zero_extend tmp.96
            tmp.99 = - tmp.98
            tmp.95 = add_ptr(ptr.4, index=tmp.99, scale=8)
            ptr.4 = tmp.95
            tmp.101 = &arr.3
            tmp.103 = sign_extend 2
            tmp.102 = add_ptr(tmp.101, index=tmp.103, scale=8)
            tmp.100 = tmp.95 != tmp.102
            if !tmp.100 jump end_if_40
            return 9
        
          end_if_40:
            tmp.104 = *ptr.4
            tmp.106 = int_to_double 3
            tmp.105 = tmp.104 != tmp.106
            if !tmp.105 jump end_if_42
            return 10
        
          end_if_42:
            l.5 = 9223372036854775807L
            tmp.108 = l.5 - 9223372036854775806L
            tmp.109 = tmp.108
            tmp.107 = add_ptr(ptr.4, index=tmp.109, scale=8)
            ptr.4 = tmp.107
            tmp.111 = &arr.3
            tmp.113 = sign_extend 3
            tmp.112 = add_ptr(tmp.111, index=tmp.113, scale=8)
            tmp.110 = tmp.107 != tmp.112
            if !tmp.110 jump end_if_44
            return 11
        
          end_if_44:
            tmp.114 = *ptr.4
            tmp.116 = int_to_double 4
            tmp.115 = tmp.114 != tmp.116
            if !tmp.115 jump end_if_46
            return 12
        
          end_if_46:
            return 0
            return 0
        }
        global function main() { 
            tmp.117 = int_array()
            result.6 = tmp.117
            if !tmp.117 jump end_if_48
            return result.6
        
          end_if_48:
            tmp.118 = double_array()
            result.6 = tmp.118
            if !tmp.118 jump end_if_50
            tmp.119 = result.6 + 12
            return tmp.119
        
          end_if_50:
            return 0
            return 0
        }
        static arr.3: Array(6,Double) = [ 1D, 2D, 3D, 4D, 5D, 6D]
        static global i: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_incr_and_decr_nested_pointers() {
    let src = r#"
        
        int main(void) {
            long arr[2][3][4] = {
                {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}},
                {{13, 14, 15, 16}, {17, 18, 19, 20}, {21, 22, 23, 24}}};
            long (*outer_ptr)[3][4] = arr + 1;
            if (outer_ptr-- != &arr[1]) {
                return 1;
            }
            if (outer_ptr[0][1] != arr[0][1]) {
                return 2;
            }
            if ((++outer_ptr)[0][2][3] != 24) {
                return 3;
            }
            if (outer_ptr[0][2][3] != 24) {
                return 4;
            }
            long (*inner_ptr)[4] = arr[0] + 1;
            if (inner_ptr++[0][2] != 7) {
                return 5;
            }
            if (inner_ptr[0][2] != 11) {
                return 6;
            }
            if ((--inner_ptr)[0][1] != 6) {
                return 7;
            }
            long *scalar_ptr = arr[1][2];
            if (scalar_ptr--[2] != 23) {
                return 8;
            }
            if (scalar_ptr[2] != 22) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 1
            arr.0[0] = tmp.0
            tmp.1 = sign_extend 2
            arr.0[8] = tmp.1
            tmp.2 = sign_extend 3
            arr.0[16] = tmp.2
            tmp.3 = sign_extend 4
            arr.0[24] = tmp.3
            tmp.4 = sign_extend 5
            arr.0[32] = tmp.4
            tmp.5 = sign_extend 6
            arr.0[40] = tmp.5
            tmp.6 = sign_extend 7
            arr.0[48] = tmp.6
            tmp.7 = sign_extend 8
            arr.0[56] = tmp.7
            tmp.8 = sign_extend 9
            arr.0[64] = tmp.8
            tmp.9 = sign_extend 10
            arr.0[72] = tmp.9
            tmp.10 = sign_extend 11
            arr.0[80] = tmp.10
            tmp.11 = sign_extend 12
            arr.0[88] = tmp.11
            tmp.12 = sign_extend 13
            arr.0[96] = tmp.12
            tmp.13 = sign_extend 14
            arr.0[104] = tmp.13
            tmp.14 = sign_extend 15
            arr.0[112] = tmp.14
            tmp.15 = sign_extend 16
            arr.0[120] = tmp.15
            tmp.16 = sign_extend 17
            arr.0[128] = tmp.16
            tmp.17 = sign_extend 18
            arr.0[136] = tmp.17
            tmp.18 = sign_extend 19
            arr.0[144] = tmp.18
            tmp.19 = sign_extend 20
            arr.0[152] = tmp.19
            tmp.20 = sign_extend 21
            arr.0[160] = tmp.20
            tmp.21 = sign_extend 22
            arr.0[168] = tmp.21
            tmp.22 = sign_extend 23
            arr.0[176] = tmp.22
            tmp.23 = sign_extend 24
            arr.0[184] = tmp.23
            tmp.24 = &arr.0
            tmp.26 = sign_extend 1
            tmp.25 = add_ptr(tmp.24, index=tmp.26, scale=96)
            outer_ptr.1 = tmp.25
            tmp.27 = outer_ptr.1
            tmp.28 = add_ptr(outer_ptr.1, index=-1L, scale=96)
            outer_ptr.1 = tmp.28
            tmp.30 = &arr.0
            tmp.31 = sign_extend 1
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=96)
            tmp.29 = tmp.27 != tmp.32
            if !tmp.29 jump end_if_0
            return 1
        
          end_if_0:
            tmp.33 = sign_extend 0
            tmp.34 = add_ptr(outer_ptr.1, index=tmp.33, scale=96)
            tmp.35 = sign_extend 1
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=32)
            tmp.38 = &arr.0
            tmp.39 = sign_extend 0
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=96)
            tmp.41 = sign_extend 1
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=32)
            tmp.37 = tmp.36 != tmp.42
            if !tmp.37 jump end_if_2
            return 2
        
          end_if_2:
            tmp.43 = add_ptr(outer_ptr.1, index=1L, scale=96)
            outer_ptr.1 = tmp.43
            tmp.44 = sign_extend 0
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=96)
            tmp.46 = sign_extend 2
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=32)
            tmp.48 = sign_extend 3
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=8)
            tmp.50 = *tmp.49
            tmp.52 = sign_extend 24
            tmp.51 = tmp.50 != tmp.52
            if !tmp.51 jump end_if_4
            return 3
        
          end_if_4:
            tmp.53 = sign_extend 0
            tmp.54 = add_ptr(outer_ptr.1, index=tmp.53, scale=96)
            tmp.55 = sign_extend 2
            tmp.56 = add_ptr(tmp.54, index=tmp.55, scale=32)
            tmp.57 = sign_extend 3
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=8)
            tmp.59 = *tmp.58
            tmp.61 = sign_extend 24
            tmp.60 = tmp.59 != tmp.61
            if !tmp.60 jump end_if_6
            return 4
        
          end_if_6:
            tmp.62 = &arr.0
            tmp.63 = sign_extend 0
            tmp.64 = add_ptr(tmp.62, index=tmp.63, scale=96)
            tmp.66 = sign_extend 1
            tmp.65 = add_ptr(tmp.64, index=tmp.66, scale=32)
            inner_ptr.2 = tmp.65
            tmp.67 = inner_ptr.2
            tmp.68 = add_ptr(inner_ptr.2, index=1L, scale=32)
            inner_ptr.2 = tmp.68
            tmp.69 = sign_extend 0
            tmp.70 = add_ptr(tmp.67, index=tmp.69, scale=32)
            tmp.71 = sign_extend 2
            tmp.72 = add_ptr(tmp.70, index=tmp.71, scale=8)
            tmp.73 = *tmp.72
            tmp.75 = sign_extend 7
            tmp.74 = tmp.73 != tmp.75
            if !tmp.74 jump end_if_8
            return 5
        
          end_if_8:
            tmp.76 = sign_extend 0
            tmp.77 = add_ptr(inner_ptr.2, index=tmp.76, scale=32)
            tmp.78 = sign_extend 2
            tmp.79 = add_ptr(tmp.77, index=tmp.78, scale=8)
            tmp.80 = *tmp.79
            tmp.82 = sign_extend 11
            tmp.81 = tmp.80 != tmp.82
            if !tmp.81 jump end_if_10
            return 6
        
          end_if_10:
            tmp.83 = add_ptr(inner_ptr.2, index=-1L, scale=32)
            inner_ptr.2 = tmp.83
            tmp.84 = sign_extend 0
            tmp.85 = add_ptr(tmp.83, index=tmp.84, scale=32)
            tmp.86 = sign_extend 1
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=8)
            tmp.88 = *tmp.87
            tmp.90 = sign_extend 6
            tmp.89 = tmp.88 != tmp.90
            if !tmp.89 jump end_if_12
            return 7
        
          end_if_12:
            tmp.91 = &arr.0
            tmp.92 = sign_extend 1
            tmp.93 = add_ptr(tmp.91, index=tmp.92, scale=96)
            tmp.94 = sign_extend 2
            tmp.95 = add_ptr(tmp.93, index=tmp.94, scale=32)
            scalar_ptr.3 = tmp.95
            tmp.96 = scalar_ptr.3
            tmp.97 = add_ptr(scalar_ptr.3, index=-1L, scale=8)
            scalar_ptr.3 = tmp.97
            tmp.98 = sign_extend 2
            tmp.99 = add_ptr(tmp.96, index=tmp.98, scale=8)
            tmp.100 = *tmp.99
            tmp.102 = sign_extend 23
            tmp.101 = tmp.100 != tmp.102
            if !tmp.101 jump end_if_14
            return 8
        
          end_if_14:
            tmp.103 = sign_extend 2
            tmp.104 = add_ptr(scalar_ptr.3, index=tmp.103, scale=8)
            tmp.105 = *tmp.104
            tmp.107 = sign_extend 22
            tmp.106 = tmp.105 != tmp.107
            if !tmp.106 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_incr_and_decr_pointers() {
    let src = r#"
        
        int main(void) {
            double x[3] = {0.0, 1.0, 2.0};
            double *ptr = x;
            if (++ptr != x + 1) {
                return 1;
            }
            if (*ptr != 1.0) {
                return 2;
            }
            if (ptr++ != x + 1) {
                return 3;
            }
            if (ptr != x + 2) {
                return 4;
            }
            if (*ptr != 2.0) {
                return 5;
            }
            if (--ptr != x + 1) {
                return 6;
            }
            if (*ptr != 1.0) {
                return 7;
            }
            if (ptr-- != x + 1) {
                return 8;
            }
            if (*ptr != 0.0) {
                return 9;
            }
            if (ptr != x) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0[0] = 0D
            x.0[8] = 1D
            x.0[16] = 2D
            tmp.0 = &x.0
            ptr.1 = tmp.0
            tmp.1 = add_ptr(ptr.1, index=1L, scale=8)
            ptr.1 = tmp.1
            tmp.3 = &x.0
            tmp.5 = sign_extend 1
            tmp.4 = add_ptr(tmp.3, index=tmp.5, scale=8)
            tmp.2 = tmp.1 != tmp.4
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = *ptr.1
            tmp.7 = tmp.6 != 1D
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = ptr.1
            tmp.9 = add_ptr(ptr.1, index=1L, scale=8)
            ptr.1 = tmp.9
            tmp.11 = &x.0
            tmp.13 = sign_extend 1
            tmp.12 = add_ptr(tmp.11, index=tmp.13, scale=8)
            tmp.10 = tmp.8 != tmp.12
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.15 = &x.0
            tmp.17 = sign_extend 2
            tmp.16 = add_ptr(tmp.15, index=tmp.17, scale=8)
            tmp.14 = ptr.1 != tmp.16
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            tmp.18 = *ptr.1
            tmp.19 = tmp.18 != 2D
            if !tmp.19 jump end_if_8
            return 5
        
          end_if_8:
            tmp.20 = add_ptr(ptr.1, index=-1L, scale=8)
            ptr.1 = tmp.20
            tmp.22 = &x.0
            tmp.24 = sign_extend 1
            tmp.23 = add_ptr(tmp.22, index=tmp.24, scale=8)
            tmp.21 = tmp.20 != tmp.23
            if !tmp.21 jump end_if_10
            return 6
        
          end_if_10:
            tmp.25 = *ptr.1
            tmp.26 = tmp.25 != 1D
            if !tmp.26 jump end_if_12
            return 7
        
          end_if_12:
            tmp.27 = ptr.1
            tmp.28 = add_ptr(ptr.1, index=-1L, scale=8)
            ptr.1 = tmp.28
            tmp.30 = &x.0
            tmp.32 = sign_extend 1
            tmp.31 = add_ptr(tmp.30, index=tmp.32, scale=8)
            tmp.29 = tmp.27 != tmp.31
            if !tmp.29 jump end_if_14
            return 8
        
          end_if_14:
            tmp.33 = *ptr.1
            tmp.34 = tmp.33 != 0D
            if !tmp.34 jump end_if_16
            return 9
        
          end_if_16:
            tmp.36 = &x.0
            tmp.35 = ptr.1 != tmp.36
            if !tmp.35 jump end_if_18
            return 10
        
          end_if_18:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_incr_decr_subscripted_vals() {
    let src = r#"
        int i = 2;
        int j = 1;
        int k = 0;
        int main(void) {
            int arr[3][2][2] = {
                {{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}, {{9, 10}, {11, 12}}};
            if (arr[i][j][k]++ != 11) {
                return 1;
            }
            if (arr[i][j][k] != 12) {
                return 2;
            }
            if (++arr[--i][j--][++k] != 9) {
                return 3;
            }
            if (arr[i][j][k] != 6) {
                return 4;
            }
            if (--arr[i][j][k] != 5) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.0[0] = 1
            arr.0[4] = 2
            arr.0[8] = 3
            arr.0[12] = 4
            arr.0[16] = 5
            arr.0[20] = 6
            arr.0[24] = 7
            arr.0[28] = 8
            arr.0[32] = 9
            arr.0[36] = 10
            arr.0[40] = 11
            arr.0[44] = 12
            tmp.0 = &arr.0
            tmp.1 = sign_extend i
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=16)
            tmp.3 = sign_extend j
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=8)
            tmp.5 = sign_extend k
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=4)
            tmp.7 = *tmp.6
            tmp.8 = tmp.7
            tmp.9 = inc tmp.7
            *tmp.6 = tmp.9
            tmp.10 = tmp.8 != 11
            if !tmp.10 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = &arr.0
            tmp.12 = sign_extend i
            tmp.13 = add_ptr(tmp.11, index=tmp.12, scale=16)
            tmp.14 = sign_extend j
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=8)
            tmp.16 = sign_extend k
            tmp.17 = add_ptr(tmp.15, index=tmp.16, scale=4)
            tmp.18 = *tmp.17
            tmp.19 = tmp.18 != 12
            if !tmp.19 jump end_if_2
            return 2
        
          end_if_2:
            tmp.20 = &arr.0
            tmp.21 = dec i
            i = tmp.21
            tmp.22 = sign_extend tmp.21
            tmp.23 = add_ptr(tmp.20, index=tmp.22, scale=16)
            tmp.24 = j
            tmp.25 = dec j
            j = tmp.25
            tmp.26 = sign_extend tmp.24
            tmp.27 = add_ptr(tmp.23, index=tmp.26, scale=8)
            tmp.28 = inc k
            k = tmp.28
            tmp.29 = sign_extend tmp.28
            tmp.30 = add_ptr(tmp.27, index=tmp.29, scale=4)
            tmp.31 = *tmp.30
            tmp.32 = inc tmp.31
            *tmp.30 = tmp.32
            tmp.33 = tmp.32 != 9
            if !tmp.33 jump end_if_4
            return 3
        
          end_if_4:
            tmp.34 = &arr.0
            tmp.35 = sign_extend i
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=16)
            tmp.37 = sign_extend j
            tmp.38 = add_ptr(tmp.36, index=tmp.37, scale=8)
            tmp.39 = sign_extend k
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=4)
            tmp.41 = *tmp.40
            tmp.42 = tmp.41 != 6
            if !tmp.42 jump end_if_6
            return 4
        
          end_if_6:
            tmp.43 = &arr.0
            tmp.44 = sign_extend i
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=16)
            tmp.46 = sign_extend j
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=8)
            tmp.48 = sign_extend k
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=4)
            tmp.50 = *tmp.49
            tmp.51 = dec tmp.50
            *tmp.49 = tmp.51
            tmp.52 = tmp.51 != 5
            if !tmp.52 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static global i: Int = 2
        static global j: Int = 1
        static global k: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_postfix_prefix_precedence() {
    let src = r#"
        
        int idx = 3;
        int main(void) {
            int arr[5] = {1, 2, 3, 4, 5};
            int *ptr = arr + 1;
            int result = ++ptr--[idx];
            if (result != 6) {
                return 1;
            }
            if (*ptr != 1) {
                return 2;
            }
            if (ptr != arr) {
                return 3;
            }
            if (*ptr++ != 1) {
                return 4;
            }
            if (*ptr != 2) {
                return 5;
            }
            for (int i = 0; i < 4; i++) {
                if (arr[i] != i + 1) {
                    return 6;
                }
            }
            if (arr[4] != 6) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.0[0] = 1
            arr.0[4] = 2
            arr.0[8] = 3
            arr.0[12] = 4
            arr.0[16] = 5
            tmp.0 = &arr.0
            tmp.2 = sign_extend 1
            tmp.1 = add_ptr(tmp.0, index=tmp.2, scale=4)
            ptr.1 = tmp.1
            tmp.3 = ptr.1
            tmp.4 = add_ptr(ptr.1, index=-1L, scale=4)
            ptr.1 = tmp.4
            tmp.5 = sign_extend idx
            tmp.6 = add_ptr(tmp.3, index=tmp.5, scale=4)
            tmp.7 = *tmp.6
            tmp.8 = inc tmp.7
            *tmp.6 = tmp.8
            result.2 = tmp.8
            tmp.9 = result.2 != 6
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.10 = *ptr.1
            tmp.11 = tmp.10 != 1
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.13 = &arr.0
            tmp.12 = ptr.1 != tmp.13
            if !tmp.12 jump end_if_4
            return 3
        
          end_if_4:
            tmp.14 = ptr.1
            tmp.15 = add_ptr(ptr.1, index=1L, scale=4)
            ptr.1 = tmp.15
            tmp.16 = *tmp.14
            tmp.17 = tmp.16 != 1
            if !tmp.17 jump end_if_6
            return 4
        
          end_if_6:
            tmp.18 = *ptr.1
            tmp.19 = tmp.18 != 2
            if !tmp.19 jump end_if_8
            return 5
        
          end_if_8:
            i.3 = 0
        
          start_loop_0:
            tmp.20 = i.3 < 4
            if !tmp.20 jump break_loop_0
            tmp.21 = &arr.0
            tmp.22 = sign_extend i.3
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=4)
            tmp.24 = *tmp.23
            tmp.26 = i.3 + 1
            tmp.25 = tmp.24 != tmp.26
            if !tmp.25 jump end_if_10
            return 6
        
          end_if_10:
        
          continue_loop_0:
            tmp.27 = i.3
            tmp.28 = inc i.3
            i.3 = tmp.28
            jump start_loop_0
        
          break_loop_0:
            tmp.29 = &arr.0
            tmp.30 = sign_extend 4
            tmp.31 = add_ptr(tmp.29, index=tmp.30, scale=4)
            tmp.32 = *tmp.31
            tmp.33 = tmp.32 != 6
            if !tmp.33 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
        static global idx: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_initialization_automatic() {
    let src = r#"
        int test_simple(void) {
            unsigned long arr[3] = {18446744073709551615UL, 9223372036854775807UL,
                                    100ul};
            return (arr[0] == 18446744073709551615UL &&
                    arr[1] == 9223372036854775807UL && arr[2] == 100ul);
        }
        int test_partial(void) {
            double arr[5] = {1.0, 123e4};
            return (arr[0] == 1.0 && arr[1] == 123e4 && !arr[2] && !arr[3] && !arr[4]);
        }
        int test_non_constant(long negative_7billion, int *ptr) {
            *ptr = 1;
            extern int three(void);
            long var = negative_7billion * three();
            long arr[5] = {
                negative_7billion,
                three() * 7l,
                -(long)*ptr,
                var + (negative_7billion ? 2 : 3)
            };
            return (arr[0] == -7000000000 && arr[1] == 21l && arr[2] == -1l &&
                    arr[3] == -20999999998l && arr[4] == 0l);
        }
        int three(void) {
            return 3;
        }
        long global_one = 1l;
        int test_type_conversion(int *ptr) {
            *ptr = -100;
            unsigned long arr[4] = {
                3458764513821589504.0,
                *ptr,
                (unsigned int)18446744073709551615UL,
                -global_one
            };
            return (arr[0] == 3458764513821589504ul &&
                    arr[1] == 18446744073709551516ul && arr[2] == 4294967295U &&
                    arr[3] == 18446744073709551615UL);
        }
        int test_preserve_stack(void) {
            int i = -1;
            int arr[3] = {global_one * 2l, global_one + three()};
            unsigned int u = 2684366905;
            if (i != -1) {
                return 0;
            }
            if (u != 2684366905) {
                return 0;
            }
            return (arr[0] == 2 && arr[1] == 4 && !arr[2]);
        }
        int main(void) {
            if (!test_simple()) {
                return 1;
            }
            if (!test_partial()) {
                return 2;
            }
            long negative_seven_billion = -7000000000l;
            int i = 0;
            if (!test_non_constant(negative_seven_billion, &i)) {
                return 3;
            }
            if (!test_type_conversion(&i)) {
                return 4;
            }
            if (!test_preserve_stack()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_simple() { 
            arr.0[0] = 18446744073709551615UL
            arr.0[8] = 9223372036854775807UL
            arr.0[16] = 100UL
            tmp.0 = &arr.0
            tmp.1 = sign_extend 0
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=8)
            tmp.3 = *tmp.2
            tmp.4 = tmp.3 == 18446744073709551615UL
            if !tmp.4 jump and_false_0
            tmp.7 = &arr.0
            tmp.8 = sign_extend 1
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 == 9223372036854775807UL
            if !tmp.11 jump and_false_0
            tmp.6 = 1
            jump and_end_1
        
          and_false_0:
            tmp.6 = 0
        
          and_end_1:
            if !tmp.6 jump and_false_2
            tmp.14 = &arr.0
            tmp.15 = sign_extend 2
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=8)
            tmp.17 = *tmp.16
            tmp.18 = tmp.17 == 100UL
            if !tmp.18 jump and_false_2
            tmp.13 = 1
            jump and_end_3
        
          and_false_2:
            tmp.13 = 0
        
          and_end_3:
            return tmp.13
            return 0
        }
        global function test_partial() { 
            arr.1[0] = 1D
            arr.1[8] = 1230000D
            arr.1[16] = 0D
            arr.1[24] = 0D
            arr.1[32] = 0D
            tmp.19 = &arr.1
            tmp.20 = sign_extend 0
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=8)
            tmp.22 = *tmp.21
            tmp.23 = tmp.22 == 1D
            if !tmp.23 jump and_false_4
            tmp.26 = &arr.1
            tmp.27 = sign_extend 1
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=8)
            tmp.29 = *tmp.28
            tmp.30 = tmp.29 == 1230000D
            if !tmp.30 jump and_false_4
            tmp.25 = 1
            jump and_end_5
        
          and_false_4:
            tmp.25 = 0
        
          and_end_5:
            if !tmp.25 jump and_false_6
            tmp.33 = &arr.1
            tmp.34 = sign_extend 2
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=8)
            tmp.36 = *tmp.35
            tmp.37 = ! tmp.36
            if !tmp.37 jump and_false_6
            tmp.32 = 1
            jump and_end_7
        
          and_false_6:
            tmp.32 = 0
        
          and_end_7:
            if !tmp.32 jump and_false_8
            tmp.40 = &arr.1
            tmp.41 = sign_extend 3
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=8)
            tmp.43 = *tmp.42
            tmp.44 = ! tmp.43
            if !tmp.44 jump and_false_8
            tmp.39 = 1
            jump and_end_9
        
          and_false_8:
            tmp.39 = 0
        
          and_end_9:
            if !tmp.39 jump and_false_10
            tmp.47 = &arr.1
            tmp.48 = sign_extend 4
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=8)
            tmp.50 = *tmp.49
            tmp.51 = ! tmp.50
            if !tmp.51 jump and_false_10
            tmp.46 = 1
            jump and_end_11
        
          and_false_10:
            tmp.46 = 0
        
          and_end_11:
            return tmp.46
            return 0
        }
        global function test_non_constant(negative_7billion.2, ptr.3) { 
            *ptr.3 = 1
            tmp.53 = three()
            tmp.54 = sign_extend tmp.53
            tmp.52 = negative_7billion.2 * tmp.54
            var.4 = tmp.52
            arr.5[0] = negative_7billion.2
            tmp.55 = three()
            tmp.56 = sign_extend tmp.55
            tmp.57 = tmp.56 * 7L
            arr.5[8] = tmp.57
            tmp.58 = *ptr.3
            tmp.59 = sign_extend tmp.58
            tmp.60 = - tmp.59
            arr.5[16] = tmp.60
            if !negative_7billion.2 jump else_13
            tmp.62 = 2
            jump end_if_12
        
          else_13:
            tmp.62 = 3
        
          end_if_12:
            tmp.63 = sign_extend tmp.62
            tmp.61 = var.4 + tmp.63
            arr.5[24] = tmp.61
            arr.5[32] = 0L
            tmp.64 = &arr.5
            tmp.65 = sign_extend 0
            tmp.66 = add_ptr(tmp.64, index=tmp.65, scale=8)
            tmp.67 = *tmp.66
            tmp.69 = - 7000000000L
            tmp.68 = tmp.67 == tmp.69
            if !tmp.68 jump and_false_14
            tmp.72 = &arr.5
            tmp.73 = sign_extend 1
            tmp.74 = add_ptr(tmp.72, index=tmp.73, scale=8)
            tmp.75 = *tmp.74
            tmp.76 = tmp.75 == 21L
            if !tmp.76 jump and_false_14
            tmp.71 = 1
            jump and_end_15
        
          and_false_14:
            tmp.71 = 0
        
          and_end_15:
            if !tmp.71 jump and_false_16
            tmp.79 = &arr.5
            tmp.80 = sign_extend 2
            tmp.81 = add_ptr(tmp.79, index=tmp.80, scale=8)
            tmp.82 = *tmp.81
            tmp.84 = - 1L
            tmp.83 = tmp.82 == tmp.84
            if !tmp.83 jump and_false_16
            tmp.78 = 1
            jump and_end_17
        
          and_false_16:
            tmp.78 = 0
        
          and_end_17:
            if !tmp.78 jump and_false_18
            tmp.87 = &arr.5
            tmp.88 = sign_extend 3
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=8)
            tmp.90 = *tmp.89
            tmp.92 = - 20999999998L
            tmp.91 = tmp.90 == tmp.92
            if !tmp.91 jump and_false_18
            tmp.86 = 1
            jump and_end_19
        
          and_false_18:
            tmp.86 = 0
        
          and_end_19:
            if !tmp.86 jump and_false_20
            tmp.95 = &arr.5
            tmp.96 = sign_extend 4
            tmp.97 = add_ptr(tmp.95, index=tmp.96, scale=8)
            tmp.98 = *tmp.97
            tmp.99 = tmp.98 == 0L
            if !tmp.99 jump and_false_20
            tmp.94 = 1
            jump and_end_21
        
          and_false_20:
            tmp.94 = 0
        
          and_end_21:
            return tmp.94
            return 0
        }
        global function three() { 
            return 3
            return 0
        }
        global function test_type_conversion(ptr.6) { 
            tmp.100 = - 100
            *ptr.6 = tmp.100
            tmp.101 = double_to_uint 3458764513821589500D
            arr.7[0] = tmp.101
            tmp.102 = *ptr.6
            tmp.103 = sign_extend tmp.102
            arr.7[8] = tmp.103
            tmp.104 = truncate 18446744073709551615UL
            tmp.105 = zero_extend tmp.104
            arr.7[16] = tmp.105
            tmp.106 = - global_one
            tmp.107 = tmp.106
            arr.7[24] = tmp.107
            tmp.108 = &arr.7
            tmp.109 = sign_extend 0
            tmp.110 = add_ptr(tmp.108, index=tmp.109, scale=8)
            tmp.111 = *tmp.110
            tmp.112 = tmp.111 == 3458764513821589504UL
            if !tmp.112 jump and_false_22
            tmp.115 = &arr.7
            tmp.116 = sign_extend 1
            tmp.117 = add_ptr(tmp.115, index=tmp.116, scale=8)
            tmp.118 = *tmp.117
            tmp.119 = tmp.118 == 18446744073709551516UL
            if !tmp.119 jump and_false_22
            tmp.114 = 1
            jump and_end_23
        
          and_false_22:
            tmp.114 = 0
        
          and_end_23:
            if !tmp.114 jump and_false_24
            tmp.122 = &arr.7
            tmp.123 = sign_extend 2
            tmp.124 = add_ptr(tmp.122, index=tmp.123, scale=8)
            tmp.125 = *tmp.124
            tmp.127 = zero_extend 4294967295U
            tmp.126 = tmp.125 == tmp.127
            if !tmp.126 jump and_false_24
            tmp.121 = 1
            jump and_end_25
        
          and_false_24:
            tmp.121 = 0
        
          and_end_25:
            if !tmp.121 jump and_false_26
            tmp.130 = &arr.7
            tmp.131 = sign_extend 3
            tmp.132 = add_ptr(tmp.130, index=tmp.131, scale=8)
            tmp.133 = *tmp.132
            tmp.134 = tmp.133 == 18446744073709551615UL
            if !tmp.134 jump and_false_26
            tmp.129 = 1
            jump and_end_27
        
          and_false_26:
            tmp.129 = 0
        
          and_end_27:
            return tmp.129
            return 0
        }
        global function test_preserve_stack() { 
            tmp.135 = - 1
            i.8 = tmp.135
            tmp.136 = global_one * 2L
            tmp.137 = truncate tmp.136
            arr.9[0] = tmp.137
            tmp.139 = three()
            tmp.140 = sign_extend tmp.139
            tmp.138 = global_one + tmp.140
            tmp.141 = truncate tmp.138
            arr.9[4] = tmp.141
            arr.9[8] = 0
            tmp.142 = truncate 2684366905L
            u.10 = tmp.142
            tmp.144 = - 1
            tmp.143 = i.8 != tmp.144
            if !tmp.143 jump end_if_28
            return 0
        
          end_if_28:
            tmp.145 = zero_extend u.10
            tmp.146 = tmp.145 != 2684366905L
            if !tmp.146 jump end_if_30
            return 0
        
          end_if_30:
            tmp.147 = &arr.9
            tmp.148 = sign_extend 0
            tmp.149 = add_ptr(tmp.147, index=tmp.148, scale=4)
            tmp.150 = *tmp.149
            tmp.151 = tmp.150 == 2
            if !tmp.151 jump and_false_32
            tmp.154 = &arr.9
            tmp.155 = sign_extend 1
            tmp.156 = add_ptr(tmp.154, index=tmp.155, scale=4)
            tmp.157 = *tmp.156
            tmp.158 = tmp.157 == 4
            if !tmp.158 jump and_false_32
            tmp.153 = 1
            jump and_end_33
        
          and_false_32:
            tmp.153 = 0
        
          and_end_33:
            if !tmp.153 jump and_false_34
            tmp.161 = &arr.9
            tmp.162 = sign_extend 2
            tmp.163 = add_ptr(tmp.161, index=tmp.162, scale=4)
            tmp.164 = *tmp.163
            tmp.165 = ! tmp.164
            if !tmp.165 jump and_false_34
            tmp.160 = 1
            jump and_end_35
        
          and_false_34:
            tmp.160 = 0
        
          and_end_35:
            return tmp.160
            return 0
        }
        global function main() { 
            tmp.166 = test_simple()
            tmp.167 = ! tmp.166
            if !tmp.167 jump end_if_36
            return 1
        
          end_if_36:
            tmp.168 = test_partial()
            tmp.169 = ! tmp.168
            if !tmp.169 jump end_if_38
            return 2
        
          end_if_38:
            tmp.170 = - 7000000000L
            negative_seven_billion.11 = tmp.170
            i.12 = 0
            tmp.171 = &i.12
            tmp.172 = test_non_constant(negative_seven_billion.11, tmp.171)
            tmp.173 = ! tmp.172
            if !tmp.173 jump end_if_40
            return 3
        
          end_if_40:
            tmp.174 = &i.12
            tmp.175 = test_type_conversion(tmp.174)
            tmp.176 = ! tmp.175
            if !tmp.176 jump end_if_42
            return 4
        
          end_if_42:
            tmp.177 = test_preserve_stack()
            tmp.178 = ! tmp.177
            if !tmp.178 jump end_if_44
            return 5
        
          end_if_44:
            return 0
            return 0
        }
        static global global_one: Long = 1L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_initialization_automatic_nested() {
    let src = r#"
        int test_simple(void) {
            int arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            for (int i = 0; i < 3; i = i + 1) {
                for (int j = 0; j < 3; j = j + 1) {
                    if (arr[i][j] != i * 3 + j + 1) {
                        return 0;
                    }
                }
            }
            return 1;
        }
        int test_partial(void) {
            int first_half_only[4][2][6] = {
                {{1, 2, 3}},
                {{4, 5, 6}}
            };
            int expected = 1;
            for (int i = 0; i < 4; i = i + 1) {
                for (int j = 0; j < 2; j = j + 1) {
                    for (int k = 0; k < 6; k = k + 1) {
                        int val = first_half_only[i][j][k];
                        if (i > 1 || j > 0 || k > 2) {
                            if (val) {
                                return 0;
                            }
                        } else {
                            if (val != expected) {
                                return 0;
                            }
                            expected = expected + 1;
                        }
                    }
                }
            }
            return 1;
        }
        int test_non_constant_and_type_conversion(void) {
            extern unsigned int three(void);
            static int x = 2000;
            int negative_four = -4;
            int *ptr = &negative_four;
            double arr[3][2] = {
                {x, x / *ptr},
                {three()},
            };
            if (arr[0][0] != 2000.0 || arr[0][1] != -500.0 || arr[1][0] != 3.0) {
                return 0;
            }
            if (arr[1][1] || arr[2][0] || arr[2][1]) {
                return 0;
            }
            return 1;
        }
        unsigned int three(void) {
            return 3u;
        }
        long one = 1l;
        int test_preserve_stack(void) {
            int i = -1;
            int arr[3][1] = {{one * 2l}, {one + three()}};
            unsigned int u = 2684366905;
            if (i != -1) {
                return 0;
            }
            if (u != 2684366905) {
                return 0;
            }
            if (arr[0][0] != 2 || arr[1][0] != 4 || arr[2][0] != 0) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_simple()) {
                return 1;
            }
            if (!test_partial()) {
                return 2;
            }
            if (!test_non_constant_and_type_conversion()) {
                return 3;
            }
            if (!test_preserve_stack()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_simple() { 
            arr.0[0] = 1
            arr.0[4] = 2
            arr.0[8] = 3
            arr.0[12] = 4
            arr.0[16] = 5
            arr.0[20] = 6
            arr.0[24] = 7
            arr.0[28] = 8
            arr.0[32] = 9
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 3
            if !tmp.0 jump break_loop_0
            j.2 = 0
        
          start_loop_1:
            tmp.1 = j.2 < 3
            if !tmp.1 jump break_loop_1
            tmp.2 = &arr.0
            tmp.3 = sign_extend i.1
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=12)
            tmp.5 = sign_extend j.2
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=4)
            tmp.7 = *tmp.6
            tmp.9 = i.1 * 3
            tmp.10 = tmp.9 + j.2
            tmp.11 = tmp.10 + 1
            tmp.8 = tmp.7 != tmp.11
            if !tmp.8 jump end_if_0
            return 0
        
          end_if_0:
        
          continue_loop_1:
            tmp.12 = j.2 + 1
            j.2 = tmp.12
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.13 = i.1 + 1
            i.1 = tmp.13
            jump start_loop_0
        
          break_loop_0:
            return 1
            return 0
        }
        global function test_partial() { 
            first_half_only.3[0] = 1
            first_half_only.3[4] = 2
            first_half_only.3[8] = 3
            first_half_only.3[12] = 0
            first_half_only.3[16] = 0
            first_half_only.3[20] = 0
            first_half_only.3[24] = 0
            first_half_only.3[28] = 0
            first_half_only.3[32] = 0
            first_half_only.3[36] = 0
            first_half_only.3[40] = 0
            first_half_only.3[44] = 0
            first_half_only.3[48] = 4
            first_half_only.3[52] = 5
            first_half_only.3[56] = 6
            first_half_only.3[60] = 0
            first_half_only.3[64] = 0
            first_half_only.3[68] = 0
            first_half_only.3[72] = 0
            first_half_only.3[76] = 0
            first_half_only.3[80] = 0
            first_half_only.3[84] = 0
            first_half_only.3[88] = 0
            first_half_only.3[92] = 0
            first_half_only.3[96] = 0
            first_half_only.3[100] = 0
            first_half_only.3[104] = 0
            first_half_only.3[108] = 0
            first_half_only.3[112] = 0
            first_half_only.3[116] = 0
            first_half_only.3[120] = 0
            first_half_only.3[124] = 0
            first_half_only.3[128] = 0
            first_half_only.3[132] = 0
            first_half_only.3[136] = 0
            first_half_only.3[140] = 0
            first_half_only.3[144] = 0
            first_half_only.3[148] = 0
            first_half_only.3[152] = 0
            first_half_only.3[156] = 0
            first_half_only.3[160] = 0
            first_half_only.3[164] = 0
            first_half_only.3[168] = 0
            first_half_only.3[172] = 0
            first_half_only.3[176] = 0
            first_half_only.3[180] = 0
            first_half_only.3[184] = 0
            first_half_only.3[188] = 0
            expected.4 = 1
            i.5 = 0
        
          start_loop_2:
            tmp.14 = i.5 < 4
            if !tmp.14 jump break_loop_2
            j.6 = 0
        
          start_loop_3:
            tmp.15 = j.6 < 2
            if !tmp.15 jump break_loop_3
            k.7 = 0
        
          start_loop_4:
            tmp.16 = k.7 < 6
            if !tmp.16 jump break_loop_4
            tmp.17 = &first_half_only.3
            tmp.18 = sign_extend i.5
            tmp.19 = add_ptr(tmp.17, index=tmp.18, scale=48)
            tmp.20 = sign_extend j.6
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=24)
            tmp.22 = sign_extend k.7
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=4)
            tmp.24 = *tmp.23
            val.8 = tmp.24
            tmp.25 = i.5 > 1
            if tmp.25 jump or_true_2
            tmp.28 = j.6 > 0
            if tmp.28 jump or_true_2
            tmp.27 = 0
            jump or_end_3
        
          or_true_2:
            tmp.27 = 1
        
          or_end_3:
            if tmp.27 jump or_true_4
            tmp.31 = k.7 > 2
            if tmp.31 jump or_true_4
            tmp.30 = 0
            jump or_end_5
        
          or_true_4:
            tmp.30 = 1
        
          or_end_5:
            if !tmp.30 jump else_7
            if !val.8 jump end_if_8
            return 0
        
          end_if_8:
            jump end_if_6
        
          else_7:
            tmp.32 = val.8 != expected.4
            if !tmp.32 jump end_if_10
            return 0
        
          end_if_10:
            tmp.33 = expected.4 + 1
            expected.4 = tmp.33
        
          end_if_6:
        
          continue_loop_4:
            tmp.34 = k.7 + 1
            k.7 = tmp.34
            jump start_loop_4
        
          break_loop_4:
        
          continue_loop_3:
            tmp.35 = j.6 + 1
            j.6 = tmp.35
            jump start_loop_3
        
          break_loop_3:
        
          continue_loop_2:
            tmp.36 = i.5 + 1
            i.5 = tmp.36
            jump start_loop_2
        
          break_loop_2:
            return 1
            return 0
        }
        global function test_non_constant_and_type_conversion() { 
            tmp.37 = - 4
            negative_four.10 = tmp.37
            tmp.38 = &negative_four.10
            ptr.11 = tmp.38
            tmp.39 = int_to_double x.9
            arr.12[0] = tmp.39
            tmp.41 = *ptr.11
            tmp.40 = x.9 / tmp.41
            tmp.42 = int_to_double tmp.40
            arr.12[8] = tmp.42
            tmp.43 = three()
            tmp.44 = uint_to_double tmp.43
            arr.12[16] = tmp.44
            arr.12[24] = 0D
            arr.12[32] = 0D
            arr.12[40] = 0D
            tmp.45 = &arr.12
            tmp.46 = sign_extend 0
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=16)
            tmp.48 = sign_extend 0
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=8)
            tmp.50 = *tmp.49
            tmp.51 = tmp.50 != 2000D
            if tmp.51 jump or_true_12
            tmp.54 = &arr.12
            tmp.55 = sign_extend 0
            tmp.56 = add_ptr(tmp.54, index=tmp.55, scale=16)
            tmp.57 = sign_extend 1
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=8)
            tmp.59 = *tmp.58
            tmp.61 = - 500D
            tmp.60 = tmp.59 != tmp.61
            if tmp.60 jump or_true_12
            tmp.53 = 0
            jump or_end_13
        
          or_true_12:
            tmp.53 = 1
        
          or_end_13:
            if tmp.53 jump or_true_14
            tmp.64 = &arr.12
            tmp.65 = sign_extend 1
            tmp.66 = add_ptr(tmp.64, index=tmp.65, scale=16)
            tmp.67 = sign_extend 0
            tmp.68 = add_ptr(tmp.66, index=tmp.67, scale=8)
            tmp.69 = *tmp.68
            tmp.70 = tmp.69 != 3D
            if tmp.70 jump or_true_14
            tmp.63 = 0
            jump or_end_15
        
          or_true_14:
            tmp.63 = 1
        
          or_end_15:
            if !tmp.63 jump end_if_16
            return 0
        
          end_if_16:
            tmp.71 = &arr.12
            tmp.72 = sign_extend 1
            tmp.73 = add_ptr(tmp.71, index=tmp.72, scale=16)
            tmp.74 = sign_extend 1
            tmp.75 = add_ptr(tmp.73, index=tmp.74, scale=8)
            tmp.76 = *tmp.75
            if tmp.76 jump or_true_18
            tmp.79 = &arr.12
            tmp.80 = sign_extend 2
            tmp.81 = add_ptr(tmp.79, index=tmp.80, scale=16)
            tmp.82 = sign_extend 0
            tmp.83 = add_ptr(tmp.81, index=tmp.82, scale=8)
            tmp.84 = *tmp.83
            if tmp.84 jump or_true_18
            tmp.78 = 0
            jump or_end_19
        
          or_true_18:
            tmp.78 = 1
        
          or_end_19:
            if tmp.78 jump or_true_20
            tmp.87 = &arr.12
            tmp.88 = sign_extend 2
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=16)
            tmp.90 = sign_extend 1
            tmp.91 = add_ptr(tmp.89, index=tmp.90, scale=8)
            tmp.92 = *tmp.91
            if tmp.92 jump or_true_20
            tmp.86 = 0
            jump or_end_21
        
          or_true_20:
            tmp.86 = 1
        
          or_end_21:
            if !tmp.86 jump end_if_22
            return 0
        
          end_if_22:
            return 1
            return 0
        }
        global function three() { 
            return 3U
            return 0
        }
        global function test_preserve_stack() { 
            tmp.93 = - 1
            i.13 = tmp.93
            tmp.94 = one * 2L
            tmp.95 = truncate tmp.94
            arr.14[0] = tmp.95
            tmp.97 = three()
            tmp.98 = zero_extend tmp.97
            tmp.96 = one + tmp.98
            tmp.99 = truncate tmp.96
            arr.14[4] = tmp.99
            arr.14[8] = 0
            tmp.100 = truncate 2684366905L
            u.15 = tmp.100
            tmp.102 = - 1
            tmp.101 = i.13 != tmp.102
            if !tmp.101 jump end_if_24
            return 0
        
          end_if_24:
            tmp.103 = zero_extend u.15
            tmp.104 = tmp.103 != 2684366905L
            if !tmp.104 jump end_if_26
            return 0
        
          end_if_26:
            tmp.105 = &arr.14
            tmp.106 = sign_extend 0
            tmp.107 = add_ptr(tmp.105, index=tmp.106, scale=4)
            tmp.108 = sign_extend 0
            tmp.109 = add_ptr(tmp.107, index=tmp.108, scale=4)
            tmp.110 = *tmp.109
            tmp.111 = tmp.110 != 2
            if tmp.111 jump or_true_28
            tmp.114 = &arr.14
            tmp.115 = sign_extend 1
            tmp.116 = add_ptr(tmp.114, index=tmp.115, scale=4)
            tmp.117 = sign_extend 0
            tmp.118 = add_ptr(tmp.116, index=tmp.117, scale=4)
            tmp.119 = *tmp.118
            tmp.120 = tmp.119 != 4
            if tmp.120 jump or_true_28
            tmp.113 = 0
            jump or_end_29
        
          or_true_28:
            tmp.113 = 1
        
          or_end_29:
            if tmp.113 jump or_true_30
            tmp.123 = &arr.14
            tmp.124 = sign_extend 2
            tmp.125 = add_ptr(tmp.123, index=tmp.124, scale=4)
            tmp.126 = sign_extend 0
            tmp.127 = add_ptr(tmp.125, index=tmp.126, scale=4)
            tmp.128 = *tmp.127
            tmp.129 = tmp.128 != 0
            if tmp.129 jump or_true_30
            tmp.122 = 0
            jump or_end_31
        
          or_true_30:
            tmp.122 = 1
        
          or_end_31:
            if !tmp.122 jump end_if_32
            return 0
        
          end_if_32:
            return 1
            return 0
        }
        global function main() { 
            tmp.130 = test_simple()
            tmp.131 = ! tmp.130
            if !tmp.131 jump end_if_34
            return 1
        
          end_if_34:
            tmp.132 = test_partial()
            tmp.133 = ! tmp.132
            if !tmp.133 jump end_if_36
            return 2
        
          end_if_36:
            tmp.134 = test_non_constant_and_type_conversion()
            tmp.135 = ! tmp.134
            if !tmp.135 jump end_if_38
            return 3
        
          end_if_38:
            tmp.136 = test_preserve_stack()
            tmp.137 = ! tmp.136
            if !tmp.137 jump end_if_40
            return 4
        
          end_if_40:
            return 0
            return 0
        }
        static global one: Long = 1L
        static x.9: Int = 2000
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_initialization_static() {
    let src = r#"
        double double_arr[3] = {1.0, 2.0, 3.0};
        int check_double_arr(double *arr) {
            if (arr[0] != 1.0) {
                return 1;
            }
            if (arr[1] != 2.0) {
                return 2;
            }
            if (arr[2] != 3.0) {
                return 3;
            }
            return 0;
        }
        unsigned uint_arr[5] = {
            1u,
            0u,
            2147497230u,
        };
        int check_uint_arr(unsigned *arr) {
            if (arr[0] != 1u) {
                return 4;
            }
            if (arr[1]) {
                return 5;
            }
            if (arr[2] != 2147497230u) {
                return 6;
            }
            if (arr[3] || arr[4]) {
                return 7;
            }
            return 0;
        }
        long long_arr[1000];
        int check_long_arr(long *arr) {
            for (int i = 0; i < 1000; i = i + 1) {
                if (arr[i]) {
                    return 8;
                }
            }
            return 0;
        }
        unsigned long ulong_arr[4] = {
            100.0, 11, 12345l, 4294967295U
        };
        int check_ulong_arr(unsigned long *arr) {
            if (arr[0] != 100ul) {
                return 9;
            }
            if (arr[1] != 11ul) {
                return 10;
            }
            if (arr[2] != 12345ul) {
                return 11;
            }
            if (arr[3] != 4294967295Ul) {
                return 12;
            }
            return 0;
        }
        int test_global(void) {
            int check = check_double_arr(double_arr);
            if (check) {
                return check;
            }
            check = check_uint_arr(uint_arr);
            if (check) {
                return check;
            }
            check = check_long_arr(long_arr);
            if (check) {
                return check;
            }
            check = check_ulong_arr(ulong_arr);
            if (check) {
                return check;
            }
            return 0;
        }
        int test_local(void) {
            double local_double_arr[3] = {1.0, 2.0, 3.0};
            static unsigned local_uint_arr[5] = {
                1u,
                0u,
                2147497230u,
            };
            static long local_long_arr[1000];
            static unsigned long local_ulong_arr[4] = {
                100.0, 11, 12345l, 4294967295U
            };
            int check = check_double_arr(local_double_arr);
            if (check) {
                return 100 + check;
            }
            check = check_uint_arr(local_uint_arr);
            if (check) {
                return 100 + check;
            }
            check = check_long_arr(local_long_arr);
            if (check) {
                return 100 + check;
            }
            check = check_ulong_arr(local_ulong_arr);
            if (check) {
                return 100 + check;
            }
            return 0;
        }
        int main(void) {
            int check = test_global();
            if (check) {
                return check;
            }
            return test_local();
        }
    "#;
    let expected = r#"
        global function check_double_arr(arr.0) { 
            tmp.0 = sign_extend 0
            tmp.1 = add_ptr(arr.0, index=tmp.0, scale=8)
            tmp.2 = *tmp.1
            tmp.3 = tmp.2 != 1D
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = sign_extend 1
            tmp.5 = add_ptr(arr.0, index=tmp.4, scale=8)
            tmp.6 = *tmp.5
            tmp.7 = tmp.6 != 2D
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = sign_extend 2
            tmp.9 = add_ptr(arr.0, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 != 3D
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        global function check_uint_arr(arr.1) { 
            tmp.12 = sign_extend 0
            tmp.13 = add_ptr(arr.1, index=tmp.12, scale=4)
            tmp.14 = *tmp.13
            tmp.15 = tmp.14 != 1U
            if !tmp.15 jump end_if_6
            return 4
        
          end_if_6:
            tmp.16 = sign_extend 1
            tmp.17 = add_ptr(arr.1, index=tmp.16, scale=4)
            tmp.18 = *tmp.17
            if !tmp.18 jump end_if_8
            return 5
        
          end_if_8:
            tmp.19 = sign_extend 2
            tmp.20 = add_ptr(arr.1, index=tmp.19, scale=4)
            tmp.21 = *tmp.20
            tmp.22 = tmp.21 != 2147497230U
            if !tmp.22 jump end_if_10
            return 6
        
          end_if_10:
            tmp.23 = sign_extend 3
            tmp.24 = add_ptr(arr.1, index=tmp.23, scale=4)
            tmp.25 = *tmp.24
            if tmp.25 jump or_true_12
            tmp.28 = sign_extend 4
            tmp.29 = add_ptr(arr.1, index=tmp.28, scale=4)
            tmp.30 = *tmp.29
            if tmp.30 jump or_true_12
            tmp.27 = 0
            jump or_end_13
        
          or_true_12:
            tmp.27 = 1
        
          or_end_13:
            if !tmp.27 jump end_if_14
            return 7
        
          end_if_14:
            return 0
            return 0
        }
        global function check_long_arr(arr.2) { 
            i.3 = 0
        
          start_loop_0:
            tmp.31 = i.3 < 1000
            if !tmp.31 jump break_loop_0
            tmp.32 = sign_extend i.3
            tmp.33 = add_ptr(arr.2, index=tmp.32, scale=8)
            tmp.34 = *tmp.33
            if !tmp.34 jump end_if_16
            return 8
        
          end_if_16:
        
          continue_loop_0:
            tmp.35 = i.3 + 1
            i.3 = tmp.35
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
        global function check_ulong_arr(arr.4) { 
            tmp.36 = sign_extend 0
            tmp.37 = add_ptr(arr.4, index=tmp.36, scale=8)
            tmp.38 = *tmp.37
            tmp.39 = tmp.38 != 100UL
            if !tmp.39 jump end_if_18
            return 9
        
          end_if_18:
            tmp.40 = sign_extend 1
            tmp.41 = add_ptr(arr.4, index=tmp.40, scale=8)
            tmp.42 = *tmp.41
            tmp.43 = tmp.42 != 11UL
            if !tmp.43 jump end_if_20
            return 10
        
          end_if_20:
            tmp.44 = sign_extend 2
            tmp.45 = add_ptr(arr.4, index=tmp.44, scale=8)
            tmp.46 = *tmp.45
            tmp.47 = tmp.46 != 12345UL
            if !tmp.47 jump end_if_22
            return 11
        
          end_if_22:
            tmp.48 = sign_extend 3
            tmp.49 = add_ptr(arr.4, index=tmp.48, scale=8)
            tmp.50 = *tmp.49
            tmp.51 = tmp.50 != 4294967295UL
            if !tmp.51 jump end_if_24
            return 12
        
          end_if_24:
            return 0
            return 0
        }
        global function test_global() { 
            tmp.52 = &double_arr
            tmp.53 = check_double_arr(tmp.52)
            check.5 = tmp.53
            if !check.5 jump end_if_26
            return check.5
        
          end_if_26:
            tmp.54 = &uint_arr
            tmp.55 = check_uint_arr(tmp.54)
            check.5 = tmp.55
            if !check.5 jump end_if_28
            return check.5
        
          end_if_28:
            tmp.56 = &long_arr
            tmp.57 = check_long_arr(tmp.56)
            check.5 = tmp.57
            if !check.5 jump end_if_30
            return check.5
        
          end_if_30:
            tmp.58 = &ulong_arr
            tmp.59 = check_ulong_arr(tmp.58)
            check.5 = tmp.59
            if !check.5 jump end_if_32
            return check.5
        
          end_if_32:
            return 0
            return 0
        }
        global function test_local() { 
            local_double_arr.6[0] = 1D
            local_double_arr.6[8] = 2D
            local_double_arr.6[16] = 3D
            tmp.60 = &local_double_arr.6
            tmp.61 = check_double_arr(tmp.60)
            check.10 = tmp.61
            if !check.10 jump end_if_34
            tmp.62 = 100 + check.10
            return tmp.62
        
          end_if_34:
            tmp.63 = &local_uint_arr.7
            tmp.64 = check_uint_arr(tmp.63)
            check.10 = tmp.64
            if !check.10 jump end_if_36
            tmp.65 = 100 + check.10
            return tmp.65
        
          end_if_36:
            tmp.66 = &local_long_arr.8
            tmp.67 = check_long_arr(tmp.66)
            check.10 = tmp.67
            if !check.10 jump end_if_38
            tmp.68 = 100 + check.10
            return tmp.68
        
          end_if_38:
            tmp.69 = &local_ulong_arr.9
            tmp.70 = check_ulong_arr(tmp.69)
            check.10 = tmp.70
            if !check.10 jump end_if_40
            tmp.71 = 100 + check.10
            return tmp.71
        
          end_if_40:
            return 0
            return 0
        }
        global function main() { 
            tmp.72 = test_global()
            check.11 = tmp.72
            if !check.11 jump end_if_42
            return check.11
        
          end_if_42:
            tmp.73 = test_local()
            return tmp.73
            return 0
        }
        static global double_arr: Array(3,Double) = [ 1D, 2D, 3D]
        static local_long_arr.8: Array(1000,Long) = zero[8000]
        static local_uint_arr.7: Array(5,Unsigned Int) = [ 1U, 0U, 2147497230U, zero[8]]
        static local_ulong_arr.9: Array(4,Unsigned Long) = [ 100UL, 11UL, 12345UL, 4294967295UL]
        static global long_arr: Array(1000,Long) = zero[8000]
        static global uint_arr: Array(5,Unsigned Int) = [ 1U, 0U, 2147497230U, zero[8]]
        static global ulong_arr: Array(4,Unsigned Long) = [ 100UL, 11UL, 12345UL, 4294967295UL]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_initialization_static_nested() {
    let src = r#"
        double double_arr[2][2] = {{1.1, 2.2}, {3.3, 4.4}};
        int check_double_arr(double (*arr)[2]) {
            if (arr[0][0] != 1.1) {
                return 1;
            }
            if (arr[0][1] != 2.2) {
                return 2;
            }
            if (arr[1][0] != 3.3) {
                return 3;
            }
            if (arr[1][1] != 4.4) {
                return 4;
            }
            return 0;
        }
        long long_arr[30][50][40];
        int check_long_arr(long (*arr)[50][40]) {
            for (int i = 0; i < 30; i = i + 1) {
                for (int j = 0; j < 50; j = j + 1) {
                    for (int k = 0; k < 40; k = k + 1) {
                        if (arr[i][j][k]) {
                            return 5;
                        }
                    }
                }
            }
            return 0;
        }
        unsigned long ulong_arr[4][6][2] = {
            {{
                 1000.3,
             },
             {12u}},
            {{2}}};
        int check_ulong_arr(unsigned long (*arr)[6][2]) {
            for (int i = 0; i < 4; i = i + 1) {
                for (int j = 0; j < 6; j = j + 1) {
                    for (int k = 0; k < 2; k = k + 1) {
                        int val = arr[i][j][k];
                        if (i == 0 && j == 0 && k == 0) {
                            if (val != 1000ul) {
                                return 6;
                            }
                        } else if (i == 0 && j == 1 && k == 0) {
                            if (val != 12ul) {
                                return 7;
                            }
                        } else if (i == 1 && j == 0 && k == 0) {
                            if (val != 2ul) {
                                return 8;
                            }
                        } else {
                            if (val) {
                                return 9;
                            }
                        }
                    }
                }
            }
            return 0;
        }
        int test_global(void) {
            int check = check_double_arr(double_arr);
            if (check) {
                return check;
            }
            check = check_long_arr(long_arr);
            if (check) {
                return check;
            }
            check = check_ulong_arr(ulong_arr);
            if (check) {
                return check;
            }
            return 0;
        }
        int test_local(void) {
            static double local_double_arr[2][2] = {{1.1, 2.2}, {3.3, 4.4}};
            int check = check_double_arr(local_double_arr);
            if (check) {
                return 100 + check;
            }
            static long local_long_arr[30][50][40];
            check = check_long_arr(local_long_arr);
            if (check) {
                return 100 + check;
            }
            static unsigned long local_ulong_arr[4][6][2] = {
                {{
                    1000.3,
                },
                {12u}},
                {{2}}};
            check = check_ulong_arr(local_ulong_arr);
            if (check) {
                return 100 + check;
            }
            return 0;
        }
        int main(void) {
            int check = test_global();
            if (check) {
                return check;
            }
            return test_local();
        }
    "#;
    let expected = r#"
        global function check_double_arr(arr.0) { 
            tmp.0 = sign_extend 0
            tmp.1 = add_ptr(arr.0, index=tmp.0, scale=16)
            tmp.2 = sign_extend 0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=8)
            tmp.4 = *tmp.3
            tmp.5 = tmp.4 != 1.1D
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = sign_extend 0
            tmp.7 = add_ptr(arr.0, index=tmp.6, scale=16)
            tmp.8 = sign_extend 1
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=8)
            tmp.10 = *tmp.9
            tmp.11 = tmp.10 != 2.2D
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = sign_extend 1
            tmp.13 = add_ptr(arr.0, index=tmp.12, scale=16)
            tmp.14 = sign_extend 0
            tmp.15 = add_ptr(tmp.13, index=tmp.14, scale=8)
            tmp.16 = *tmp.15
            tmp.17 = tmp.16 != 3.3D
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            tmp.18 = sign_extend 1
            tmp.19 = add_ptr(arr.0, index=tmp.18, scale=16)
            tmp.20 = sign_extend 1
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=8)
            tmp.22 = *tmp.21
            tmp.23 = tmp.22 != 4.4D
            if !tmp.23 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        global function check_long_arr(arr.1) { 
            i.2 = 0
        
          start_loop_0:
            tmp.24 = i.2 < 30
            if !tmp.24 jump break_loop_0
            j.3 = 0
        
          start_loop_1:
            tmp.25 = j.3 < 50
            if !tmp.25 jump break_loop_1
            k.4 = 0
        
          start_loop_2:
            tmp.26 = k.4 < 40
            if !tmp.26 jump break_loop_2
            tmp.27 = sign_extend i.2
            tmp.28 = add_ptr(arr.1, index=tmp.27, scale=16000)
            tmp.29 = sign_extend j.3
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=320)
            tmp.31 = sign_extend k.4
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=8)
            tmp.33 = *tmp.32
            if !tmp.33 jump end_if_8
            return 5
        
          end_if_8:
        
          continue_loop_2:
            tmp.34 = k.4 + 1
            k.4 = tmp.34
            jump start_loop_2
        
          break_loop_2:
        
          continue_loop_1:
            tmp.35 = j.3 + 1
            j.3 = tmp.35
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.36 = i.2 + 1
            i.2 = tmp.36
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
        global function check_ulong_arr(arr.5) { 
            i.6 = 0
        
          start_loop_3:
            tmp.37 = i.6 < 4
            if !tmp.37 jump break_loop_3
            j.7 = 0
        
          start_loop_4:
            tmp.38 = j.7 < 6
            if !tmp.38 jump break_loop_4
            k.8 = 0
        
          start_loop_5:
            tmp.39 = k.8 < 2
            if !tmp.39 jump break_loop_5
            tmp.40 = sign_extend i.6
            tmp.41 = add_ptr(arr.5, index=tmp.40, scale=96)
            tmp.42 = sign_extend j.7
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=16)
            tmp.44 = sign_extend k.8
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=8)
            tmp.46 = *tmp.45
            tmp.47 = truncate tmp.46
            val.9 = tmp.47
            tmp.48 = i.6 == 0
            if !tmp.48 jump and_false_10
            tmp.51 = j.7 == 0
            if !tmp.51 jump and_false_10
            tmp.50 = 1
            jump and_end_11
        
          and_false_10:
            tmp.50 = 0
        
          and_end_11:
            if !tmp.50 jump and_false_12
            tmp.54 = k.8 == 0
            if !tmp.54 jump and_false_12
            tmp.53 = 1
            jump and_end_13
        
          and_false_12:
            tmp.53 = 0
        
          and_end_13:
            if !tmp.53 jump else_15
            tmp.55 = sign_extend val.9
            tmp.56 = tmp.55 != 1000UL
            if !tmp.56 jump end_if_16
            return 6
        
          end_if_16:
            jump end_if_14
        
          else_15:
            tmp.57 = i.6 == 0
            if !tmp.57 jump and_false_18
            tmp.60 = j.7 == 1
            if !tmp.60 jump and_false_18
            tmp.59 = 1
            jump and_end_19
        
          and_false_18:
            tmp.59 = 0
        
          and_end_19:
            if !tmp.59 jump and_false_20
            tmp.63 = k.8 == 0
            if !tmp.63 jump and_false_20
            tmp.62 = 1
            jump and_end_21
        
          and_false_20:
            tmp.62 = 0
        
          and_end_21:
            if !tmp.62 jump else_23
            tmp.64 = sign_extend val.9
            tmp.65 = tmp.64 != 12UL
            if !tmp.65 jump end_if_24
            return 7
        
          end_if_24:
            jump end_if_22
        
          else_23:
            tmp.66 = i.6 == 1
            if !tmp.66 jump and_false_26
            tmp.69 = j.7 == 0
            if !tmp.69 jump and_false_26
            tmp.68 = 1
            jump and_end_27
        
          and_false_26:
            tmp.68 = 0
        
          and_end_27:
            if !tmp.68 jump and_false_28
            tmp.72 = k.8 == 0
            if !tmp.72 jump and_false_28
            tmp.71 = 1
            jump and_end_29
        
          and_false_28:
            tmp.71 = 0
        
          and_end_29:
            if !tmp.71 jump else_31
            tmp.73 = sign_extend val.9
            tmp.74 = tmp.73 != 2UL
            if !tmp.74 jump end_if_32
            return 8
        
          end_if_32:
            jump end_if_30
        
          else_31:
            if !val.9 jump end_if_34
            return 9
        
          end_if_34:
        
          end_if_30:
        
          end_if_22:
        
          end_if_14:
        
          continue_loop_5:
            tmp.75 = k.8 + 1
            k.8 = tmp.75
            jump start_loop_5
        
          break_loop_5:
        
          continue_loop_4:
            tmp.76 = j.7 + 1
            j.7 = tmp.76
            jump start_loop_4
        
          break_loop_4:
        
          continue_loop_3:
            tmp.77 = i.6 + 1
            i.6 = tmp.77
            jump start_loop_3
        
          break_loop_3:
            return 0
            return 0
        }
        global function test_global() { 
            tmp.78 = &double_arr
            tmp.79 = check_double_arr(tmp.78)
            check.10 = tmp.79
            if !check.10 jump end_if_36
            return check.10
        
          end_if_36:
            tmp.80 = &long_arr
            tmp.81 = check_long_arr(tmp.80)
            check.10 = tmp.81
            if !check.10 jump end_if_38
            return check.10
        
          end_if_38:
            tmp.82 = &ulong_arr
            tmp.83 = check_ulong_arr(tmp.82)
            check.10 = tmp.83
            if !check.10 jump end_if_40
            return check.10
        
          end_if_40:
            return 0
            return 0
        }
        global function test_local() { 
            tmp.84 = &local_double_arr.11
            tmp.85 = check_double_arr(tmp.84)
            check.12 = tmp.85
            if !check.12 jump end_if_42
            tmp.86 = 100 + check.12
            return tmp.86
        
          end_if_42:
            tmp.87 = &local_long_arr.13
            tmp.88 = check_long_arr(tmp.87)
            check.12 = tmp.88
            if !check.12 jump end_if_44
            tmp.89 = 100 + check.12
            return tmp.89
        
          end_if_44:
            tmp.90 = &local_ulong_arr.14
            tmp.91 = check_ulong_arr(tmp.90)
            check.12 = tmp.91
            if !check.12 jump end_if_46
            tmp.92 = 100 + check.12
            return tmp.92
        
          end_if_46:
            return 0
            return 0
        }
        global function main() { 
            tmp.93 = test_global()
            check.15 = tmp.93
            if !check.15 jump end_if_48
            return check.15
        
          end_if_48:
            tmp.94 = test_local()
            return tmp.94
            return 0
        }
        static global double_arr: Array(2,Array(2,Double)) = [ 1.1D, 2.2D, 3.3D, 4.4D]
        static local_double_arr.11: Array(2,Array(2,Double)) = [ 1.1D, 2.2D, 3.3D, 4.4D]
        static local_long_arr.13: Array(30,Array(50,Array(40,Long))) = zero[480000]
        static local_ulong_arr.14: Array(4,Array(6,Array(2,Unsigned Long))) = [ 1000UL, zero[8], 12UL, zero[8], zero[64], 2UL, zero[8], zero[80], zero[192]]
        static global long_arr: Array(30,Array(50,Array(40,Long))) = zero[480000]
        static global ulong_arr: Array(4,Array(6,Array(2,Unsigned Long))) = [ 1000UL, zero[8], 12UL, zero[8], zero[64], 2UL, zero[8], zero[80], zero[192]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_initialization_trailing_comma_initializer() {
    let src = r#"
        int foo(int a, int b, int c);
        int main(void) {
            int arr[3] = {
                1,
                2,
                3,
            };
            return arr[2];
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.3[0] = 1
            arr.3[4] = 2
            arr.3[8] = 3
            tmp.0 = &arr.3
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=4)
            tmp.3 = *tmp.2
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_global_array() {
    let src = r#"
        long arr[4] = {1, 2, 3, 4};
        int double_each_element(void) {
            for (int i = 0; i < 4; i = i + 1) {
                arr[i] = arr[i] * 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function double_each_element() { 
            i.0 = 0
        
          start_loop_0:
            tmp.0 = i.0 < 4
            if !tmp.0 jump break_loop_0
            tmp.1 = &arr
            tmp.2 = sign_extend i.0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=8)
            tmp.4 = &arr
            tmp.5 = sign_extend i.0
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=8)
            tmp.7 = *tmp.6
            tmp.9 = sign_extend 2
            tmp.8 = tmp.7 * tmp.9
            *tmp.3 = tmp.8
        
          continue_loop_0:
            tmp.10 = i.0 + 1
            i.0 = tmp.10
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
        static global arr: Array(4,Long) = [ 1L, 2L, 3L, 4L]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_global_array_client() {
    let src = r#"
        
        extern long arr[4];
        int double_each_element(void);
        int main(void) {
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != i + 1) {
                    return i + 1;
                }
            }
            double_each_element();
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != (i + 1) * 2) {
                    return i + 5;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
        
          start_loop_0:
            tmp.0 = i.0 < 4
            if !tmp.0 jump break_loop_0
            tmp.1 = &arr
            tmp.2 = sign_extend i.0
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=8)
            tmp.4 = *tmp.3
            tmp.6 = i.0 + 1
            tmp.7 = sign_extend tmp.6
            tmp.5 = tmp.4 != tmp.7
            if !tmp.5 jump end_if_0
            tmp.8 = i.0 + 1
            return tmp.8
        
          end_if_0:
        
          continue_loop_0:
            tmp.9 = i.0 + 1
            i.0 = tmp.9
            jump start_loop_0
        
          break_loop_0:
            tmp.10 = double_each_element()
            i.1 = 0
        
          start_loop_1:
            tmp.11 = i.1 < 4
            if !tmp.11 jump break_loop_1
            tmp.12 = &arr
            tmp.13 = sign_extend i.1
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=8)
            tmp.15 = *tmp.14
            tmp.17 = i.1 + 1
            tmp.18 = tmp.17 * 2
            tmp.19 = sign_extend tmp.18
            tmp.16 = tmp.15 != tmp.19
            if !tmp.16 jump end_if_2
            tmp.20 = i.1 + 5
            return tmp.20
        
          end_if_2:
        
          continue_loop_1:
            tmp.21 = i.1 + 1
            i.1 = tmp.21
            jump start_loop_1
        
          break_loop_1:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_return_pointer_to_array() {
    let src = r#"
        
        long (*return_row(long (*arr)[3][4], int idx))[4] {
            return arr[idx];
        }
    "#;
    let expected = r#"
        global function return_row(arr.0, idx.1) { 
            tmp.0 = sign_extend idx.1
            tmp.1 = add_ptr(arr.0, index=tmp.0, scale=96)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_return_pointer_to_array_client() {
    let src = r#"
        
        long (*return_row(long (*arr)[3][4], int idx))[4];
        int main(void) {
            long nested_array[2][3][4] = {
                {{0}},
                {{-12, -13, -14, -15}, {-16}}
            };
            long (*row_pointer)[4] = return_row(nested_array, 1);
            for (int i = 0; i < 3; i = i + 1) {
                for (int j = 0; j < 4; j = j + 1) {
                    if (row_pointer[i][j] != nested_array[1][i][j]) {
                        return 1;
                    }
                }
            }
            row_pointer[2][1] = 100;
            if (nested_array[1][2][1] != 100) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 0
            nested_array.2[0] = tmp.0
            nested_array.2[8] = 0L
            nested_array.2[16] = 0L
            nested_array.2[24] = 0L
            nested_array.2[32] = 0L
            nested_array.2[40] = 0L
            nested_array.2[48] = 0L
            nested_array.2[56] = 0L
            nested_array.2[64] = 0L
            nested_array.2[72] = 0L
            nested_array.2[80] = 0L
            nested_array.2[88] = 0L
            tmp.1 = - 12
            tmp.2 = sign_extend tmp.1
            nested_array.2[96] = tmp.2
            tmp.3 = - 13
            tmp.4 = sign_extend tmp.3
            nested_array.2[104] = tmp.4
            tmp.5 = - 14
            tmp.6 = sign_extend tmp.5
            nested_array.2[112] = tmp.6
            tmp.7 = - 15
            tmp.8 = sign_extend tmp.7
            nested_array.2[120] = tmp.8
            tmp.9 = - 16
            tmp.10 = sign_extend tmp.9
            nested_array.2[128] = tmp.10
            nested_array.2[136] = 0L
            nested_array.2[144] = 0L
            nested_array.2[152] = 0L
            nested_array.2[160] = 0L
            nested_array.2[168] = 0L
            nested_array.2[176] = 0L
            nested_array.2[184] = 0L
            tmp.11 = &nested_array.2
            tmp.12 = return_row(tmp.11, 1)
            row_pointer.3 = tmp.12
            i.4 = 0
        
          start_loop_0:
            tmp.13 = i.4 < 3
            if !tmp.13 jump break_loop_0
            j.5 = 0
        
          start_loop_1:
            tmp.14 = j.5 < 4
            if !tmp.14 jump break_loop_1
            tmp.15 = sign_extend i.4
            tmp.16 = add_ptr(row_pointer.3, index=tmp.15, scale=32)
            tmp.17 = sign_extend j.5
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=8)
            tmp.19 = *tmp.18
            tmp.21 = &nested_array.2
            tmp.22 = sign_extend 1
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=96)
            tmp.24 = sign_extend i.4
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=32)
            tmp.26 = sign_extend j.5
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=8)
            tmp.28 = *tmp.27
            tmp.20 = tmp.19 != tmp.28
            if !tmp.20 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_1:
            tmp.29 = j.5 + 1
            j.5 = tmp.29
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.30 = i.4 + 1
            i.4 = tmp.30
            jump start_loop_0
        
          break_loop_0:
            tmp.31 = sign_extend 2
            tmp.32 = add_ptr(row_pointer.3, index=tmp.31, scale=32)
            tmp.33 = sign_extend 1
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=8)
            tmp.35 = sign_extend 100
            *tmp.34 = tmp.35
            tmp.36 = &nested_array.2
            tmp.37 = sign_extend 1
            tmp.38 = add_ptr(tmp.36, index=tmp.37, scale=96)
            tmp.39 = sign_extend 2
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=32)
            tmp.41 = sign_extend 1
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=8)
            tmp.43 = *tmp.42
            tmp.45 = sign_extend 100
            tmp.44 = tmp.43 != tmp.45
            if !tmp.44 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_set_array_val() {
    let src = r#"
        int set_nth_element(double *arr, int idx) {
            for (int i = 0; i < 5; i = i + 1) {
                if (arr[i]) {
                    return 1;
                }
            }
            arr[idx] = 8;
            return 0;
        }
        int set_nested_element(int (*arr)[2], int i, int j) {
            for (int x = 0; x < 3; x = x + 1) {
                for (int y = 0; y < 2; y = y + 1) {
                    int expected = -10 + 2*x + y;
                    if (arr[x][y] != expected) {
                        return 4;
                    }
                }
            }
            arr[i][j] = 10;
            return 0;
        }
    "#;
    let expected = r#"
        global function set_nth_element(arr.0, idx.1) { 
            i.2 = 0
        
          start_loop_0:
            tmp.0 = i.2 < 5
            if !tmp.0 jump break_loop_0
            tmp.1 = sign_extend i.2
            tmp.2 = add_ptr(arr.0, index=tmp.1, scale=8)
            tmp.3 = *tmp.2
            tmp.4 = tmp.3 != 0D
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.5 = i.2 + 1
            i.2 = tmp.5
            jump start_loop_0
        
          break_loop_0:
            tmp.6 = sign_extend idx.1
            tmp.7 = add_ptr(arr.0, index=tmp.6, scale=8)
            tmp.8 = int_to_double 8
            *tmp.7 = tmp.8
            return 0
            return 0
        }
        global function set_nested_element(arr.3, i.4, j.5) { 
            x.6 = 0
        
          start_loop_1:
            tmp.9 = x.6 < 3
            if !tmp.9 jump break_loop_1
            y.7 = 0
        
          start_loop_2:
            tmp.10 = y.7 < 2
            if !tmp.10 jump break_loop_2
            tmp.11 = - 10
            tmp.13 = 2 * x.6
            tmp.12 = tmp.11 + tmp.13
            tmp.14 = tmp.12 + y.7
            expected.8 = tmp.14
            tmp.15 = sign_extend x.6
            tmp.16 = add_ptr(arr.3, index=tmp.15, scale=8)
            tmp.17 = sign_extend y.7
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=4)
            tmp.19 = *tmp.18
            tmp.20 = tmp.19 != expected.8
            if !tmp.20 jump end_if_2
            return 4
        
          end_if_2:
        
          continue_loop_2:
            tmp.21 = y.7 + 1
            y.7 = tmp.21
            jump start_loop_2
        
          break_loop_2:
        
          continue_loop_1:
            tmp.22 = x.6 + 1
            x.6 = tmp.22
            jump start_loop_1
        
          break_loop_1:
            tmp.23 = sign_extend i.4
            tmp.24 = add_ptr(arr.3, index=tmp.23, scale=8)
            tmp.25 = sign_extend j.5
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=4)
            *tmp.26 = 10
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_set_array_val_client() {
    let src = r#"
        int set_nth_element(double *arr, int idx);
        int set_nested_element(int (*arr)[2], int i, int j);
        int main(void) {
            double arr[5] = {0.0, 0.0, 0.0, 0.0, 0.0};
            int check = set_nth_element(arr, 4);
            if (check) {
                return check;
            }
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != 0) {
                    return 2;
                }
            }
            if (arr[4] != 8)
                return 3;
            int nested_arr[3][2] = {{-10, -9}, {-8, -7}, {-6, -5}};
            check = set_nested_element(nested_arr, 2, 1);
            if (check) {
                return check;
            }
            for (int i = 0; i < 3; i = i + 1) {
                for (int j = 0; j < 2; j = j + 1) {
                    if (i == 2 && j == 1) {
                        if (nested_arr[i][j] != 10) {
                            return 5;
                        }
                    } else {
                        int expected = -10 + 2 * i + j;
                        if (nested_arr[i][j] != expected) {
                            return 6;
                        }
                    }
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.5[0] = 0D
            arr.5[8] = 0D
            arr.5[16] = 0D
            arr.5[24] = 0D
            arr.5[32] = 0D
            tmp.0 = &arr.5
            tmp.1 = set_nth_element(tmp.0, 4)
            check.6 = tmp.1
            if !check.6 jump end_if_0
            return check.6
        
          end_if_0:
            i.7 = 0
        
          start_loop_0:
            tmp.2 = i.7 < 4
            if !tmp.2 jump break_loop_0
            tmp.3 = &arr.5
            tmp.4 = sign_extend i.7
            tmp.5 = add_ptr(tmp.3, index=tmp.4, scale=8)
            tmp.6 = *tmp.5
            tmp.8 = int_to_double 0
            tmp.7 = tmp.6 != tmp.8
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
        
          continue_loop_0:
            tmp.9 = i.7 + 1
            i.7 = tmp.9
            jump start_loop_0
        
          break_loop_0:
            tmp.10 = &arr.5
            tmp.11 = sign_extend 4
            tmp.12 = add_ptr(tmp.10, index=tmp.11, scale=8)
            tmp.13 = *tmp.12
            tmp.15 = int_to_double 8
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_4
            return 3
        
          end_if_4:
            tmp.16 = - 10
            nested_arr.8[0] = tmp.16
            tmp.17 = - 9
            nested_arr.8[4] = tmp.17
            tmp.18 = - 8
            nested_arr.8[8] = tmp.18
            tmp.19 = - 7
            nested_arr.8[12] = tmp.19
            tmp.20 = - 6
            nested_arr.8[16] = tmp.20
            tmp.21 = - 5
            nested_arr.8[20] = tmp.21
            tmp.22 = &nested_arr.8
            tmp.23 = set_nested_element(tmp.22, 2, 1)
            check.6 = tmp.23
            if !check.6 jump end_if_6
            return check.6
        
          end_if_6:
            i.9 = 0
        
          start_loop_1:
            tmp.24 = i.9 < 3
            if !tmp.24 jump break_loop_1
            j.10 = 0
        
          start_loop_2:
            tmp.25 = j.10 < 2
            if !tmp.25 jump break_loop_2
            tmp.26 = i.9 == 2
            if !tmp.26 jump and_false_8
            tmp.29 = j.10 == 1
            if !tmp.29 jump and_false_8
            tmp.28 = 1
            jump and_end_9
        
          and_false_8:
            tmp.28 = 0
        
          and_end_9:
            if !tmp.28 jump else_11
            tmp.30 = &nested_arr.8
            tmp.31 = sign_extend i.9
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=8)
            tmp.33 = sign_extend j.10
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=4)
            tmp.35 = *tmp.34
            tmp.36 = tmp.35 != 10
            if !tmp.36 jump end_if_12
            return 5
        
          end_if_12:
            jump end_if_10
        
          else_11:
            tmp.37 = - 10
            tmp.39 = 2 * i.9
            tmp.38 = tmp.37 + tmp.39
            tmp.40 = tmp.38 + j.10
            expected.11 = tmp.40
            tmp.41 = &nested_arr.8
            tmp.42 = sign_extend i.9
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=8)
            tmp.44 = sign_extend j.10
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=4)
            tmp.46 = *tmp.45
            tmp.47 = tmp.46 != expected.11
            if !tmp.47 jump end_if_14
            return 6
        
          end_if_14:
        
          end_if_10:
        
          continue_loop_2:
            tmp.48 = j.10 + 1
            j.10 = tmp.48
            jump start_loop_2
        
          break_loop_2:
        
          continue_loop_1:
            tmp.49 = i.9 + 1
            i.9 = tmp.49
            jump start_loop_1
        
          break_loop_1:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_pointer_arithmetic_add_dereference_and_assign() {
    let src = r#"
        int main(void) {
            int arr[2] = {1, 2};
            *arr = 3;
            *(arr + 1) = 4;
            if (arr[0] != 3) {
                return 1;
            }
            if (arr[1] != 4) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.0[0] = 1
            arr.0[4] = 2
            tmp.0 = &arr.0
            *tmp.0 = 3
            tmp.1 = &arr.0
            tmp.3 = sign_extend 1
            tmp.2 = add_ptr(tmp.1, index=tmp.3, scale=4)
            *tmp.2 = 4
            tmp.4 = &arr.0
            tmp.5 = sign_extend 0
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=4)
            tmp.7 = *tmp.6
            tmp.8 = tmp.7 != 3
            if !tmp.8 jump end_if_0
            return 1
        
          end_if_0:
            tmp.9 = &arr.0
            tmp.10 = sign_extend 1
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=4)
            tmp.12 = *tmp.11
            tmp.13 = tmp.12 != 4
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_pointer_arithmetic_compare() {
    let src = r#"
        unsigned long gt(unsigned long *a, unsigned long *b) {
            return a > b;
        }
        unsigned long lt(unsigned long *a, unsigned long *b) {
            return a < b;
        }
        unsigned long ge(unsigned long *a, unsigned long *b) {
            return a >= b;
        }
        unsigned long le(unsigned long *a, unsigned long *b) {
            return a <= b;
        }
        unsigned long gt_nested(unsigned long (*a)[5], unsigned long (*b)[5]) {
            return a > b;
        }
        unsigned long ge_nested(unsigned long (*a)[5], unsigned long (*b)[5]) {
            return a >= b;
        }
        int main(void)
        {
            unsigned long arr[5];
            unsigned long *elem_1 = arr + 1;
            unsigned long *elem_4 = arr + 4;
            if (gt(elem_1, elem_4)) {
                return 1;
            }
            if (!(lt(elem_1, elem_4))) {
                return 2;
            }
            if (!(ge(elem_1, elem_1))) {
                return 3;
            }
            if (le(elem_4, elem_1)) {
                return 4;
            }
            unsigned long *one_past_the_end = arr + 5;
            if (!(gt(one_past_the_end, elem_4))) {
                return 5;
            }
            if (one_past_the_end != elem_4 + 1) {
                return 6;
            }
            unsigned long nested_arr[4][5];
            unsigned long *elem_3_2 = *(nested_arr + 3) + 2;
            unsigned long *elem_3_3 = *(nested_arr + 3) + 3;
            if (lt(elem_3_3, elem_3_2)) {
                return 7;
            }
            if (!ge(elem_3_3, elem_3_2)) {
                return 8;
            }
            unsigned long (*subarray_0)[5] = nested_arr;
            unsigned long (*subarray_3)[5] = nested_arr + 3;
            unsigned long (*subarray_one_past_the_end)[5] = nested_arr + 4;
            if (ge_nested(subarray_0, subarray_3)){
                return 9;
            }
            if (!(gt_nested(subarray_one_past_the_end, subarray_3))) {
                return 10;
            }
            if (subarray_3 != subarray_one_past_the_end - 1) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function gt(a.0, b.1) { 
            tmp.0 = a.0 > b.1
            tmp.1 = sign_extend tmp.0
            return tmp.1
            return 0
        }
        global function lt(a.2, b.3) { 
            tmp.2 = a.2 < b.3
            tmp.3 = sign_extend tmp.2
            return tmp.3
            return 0
        }
        global function ge(a.4, b.5) { 
            tmp.4 = a.4 >= b.5
            tmp.5 = sign_extend tmp.4
            return tmp.5
            return 0
        }
        global function le(a.6, b.7) { 
            tmp.6 = a.6 <= b.7
            tmp.7 = sign_extend tmp.6
            return tmp.7
            return 0
        }
        global function gt_nested(a.8, b.9) { 
            tmp.8 = a.8 > b.9
            tmp.9 = sign_extend tmp.8
            return tmp.9
            return 0
        }
        global function ge_nested(a.10, b.11) { 
            tmp.10 = a.10 >= b.11
            tmp.11 = sign_extend tmp.10
            return tmp.11
            return 0
        }
        global function main() { 
            tmp.12 = &arr.12
            tmp.14 = sign_extend 1
            tmp.13 = add_ptr(tmp.12, index=tmp.14, scale=8)
            elem_1.13 = tmp.13
            tmp.15 = &arr.12
            tmp.17 = sign_extend 4
            tmp.16 = add_ptr(tmp.15, index=tmp.17, scale=8)
            elem_4.14 = tmp.16
            tmp.18 = gt(elem_1.13, elem_4.14)
            if !tmp.18 jump end_if_0
            return 1
        
          end_if_0:
            tmp.19 = lt(elem_1.13, elem_4.14)
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_2
            return 2
        
          end_if_2:
            tmp.21 = ge(elem_1.13, elem_1.13)
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_4
            return 3
        
          end_if_4:
            tmp.23 = le(elem_4.14, elem_1.13)
            if !tmp.23 jump end_if_6
            return 4
        
          end_if_6:
            tmp.24 = &arr.12
            tmp.26 = sign_extend 5
            tmp.25 = add_ptr(tmp.24, index=tmp.26, scale=8)
            one_past_the_end.15 = tmp.25
            tmp.27 = gt(one_past_the_end.15, elem_4.14)
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_8
            return 5
        
          end_if_8:
            tmp.31 = sign_extend 1
            tmp.30 = add_ptr(elem_4.14, index=tmp.31, scale=8)
            tmp.29 = one_past_the_end.15 != tmp.30
            if !tmp.29 jump end_if_10
            return 6
        
          end_if_10:
            tmp.32 = &nested_arr.16
            tmp.34 = sign_extend 3
            tmp.33 = add_ptr(tmp.32, index=tmp.34, scale=40)
            tmp.36 = sign_extend 2
            tmp.35 = add_ptr(tmp.33, index=tmp.36, scale=8)
            elem_3_2.17 = tmp.35
            tmp.37 = &nested_arr.16
            tmp.39 = sign_extend 3
            tmp.38 = add_ptr(tmp.37, index=tmp.39, scale=40)
            tmp.41 = sign_extend 3
            tmp.40 = add_ptr(tmp.38, index=tmp.41, scale=8)
            elem_3_3.18 = tmp.40
            tmp.42 = lt(elem_3_3.18, elem_3_2.17)
            if !tmp.42 jump end_if_12
            return 7
        
          end_if_12:
            tmp.43 = ge(elem_3_3.18, elem_3_2.17)
            tmp.44 = ! tmp.43
            if !tmp.44 jump end_if_14
            return 8
        
          end_if_14:
            tmp.45 = &nested_arr.16
            subarray_0.19 = tmp.45
            tmp.46 = &nested_arr.16
            tmp.48 = sign_extend 3
            tmp.47 = add_ptr(tmp.46, index=tmp.48, scale=40)
            subarray_3.20 = tmp.47
            tmp.49 = &nested_arr.16
            tmp.51 = sign_extend 4
            tmp.50 = add_ptr(tmp.49, index=tmp.51, scale=40)
            subarray_one_past_the_end.21 = tmp.50
            tmp.52 = ge_nested(subarray_0.19, subarray_3.20)
            if !tmp.52 jump end_if_16
            return 9
        
          end_if_16:
            tmp.53 = gt_nested(subarray_one_past_the_end.21, subarray_3.20)
            tmp.54 = ! tmp.53
            if !tmp.54 jump end_if_18
            return 10
        
          end_if_18:
            tmp.57 = sign_extend 1
            tmp.58 = - tmp.57
            tmp.56 = add_ptr(subarray_one_past_the_end.21, index=tmp.58, scale=40)
            tmp.55 = subarray_3.20 != tmp.56
            if !tmp.55 jump end_if_20
            return 11
        
          end_if_20:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_pointer_arithmetic_pointer_add() {
    let src = r#"
        int test_add_constant_to_pointer(void) {
            long long_arr[12] = {0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 13};
            long *ptr = long_arr + 10;
            return *ptr == 13;
        }
        int test_add_negative_index(void) {
            unsigned unsigned_arr[12] = {0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 42};
            unsigned *end_ptr = unsigned_arr + 12;
            unsigned *ptr = end_ptr + -10;
            return *ptr == 2;
        }
        int test_add_pointer_to_int(void) {
            int int_arr[5] = {0, 98, 99};
            int *ptr1 = int_arr + 2;
            int *ptr2 = 2 + int_arr;
            return (ptr1 == ptr2 && *ptr2 == 99);
        }
        int test_add_different_index_types(void) {
            double double_arr[11] = {0, 0, 0, 0, 0, 6.0};
            double *ptr1 = double_arr + 5;
            double *ptr2 = double_arr + 5l;
            double *ptr3 = double_arr + 5u;
            double *ptr4 = double_arr + 5ul;
            return (ptr1 == ptr2 && ptr1 == ptr3 && ptr1 == ptr4 && *ptr4 == 6.0);
        }
        int test_add_complex_expressions(void) {
            static int flag;
            int i = -2;
            int *small_int_ptr = &i;
            extern int return_one(void);
            extern int *get_elem1_ptr(int *arr);
            extern int *get_elem2_ptr(int *arr);
            static int arr[4] = {1, 2, 3, 4};
            int *ptr = return_one() + (*small_int_ptr) +
                       (flag ? get_elem1_ptr(arr) : get_elem2_ptr(arr));
            return (ptr == arr + 1 && *ptr == 2);
        }
        int return_one(void) {
            return 1;
        }
        int *get_elem1_ptr(int *arr) {
            return arr + 1;
        }
        int *get_elem2_ptr(int *arr) {
            return arr + 2;
        }
        int test_add_multi_dimensional(void) {
            static int index = 2;
            int nested_arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            int(*row_pointer)[3] = nested_arr + index;
            return **row_pointer == 7;
        }
        int test_add_to_subarray_pointer(void) {
            static int index = 2;
            int nested_arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            int *row1 = *(nested_arr + 1);
            int *elem_ptr = row1 + index;
            return *elem_ptr == 6;
        }
        int test_subtract_from_pointer(void) {
            long long_arr[5] = {10, 9, 8, 7, 6};
            long *one_past_the_end = long_arr + 5;
            static int index = 3;
            long *subtraction_result = one_past_the_end - index;
            return *subtraction_result == 8;
        }
        int test_subtract_negative_index(void) {
            unsigned arr[5] = {100, 101, 102, 103, 104};
            unsigned *ptr = arr - (-3);
            return *ptr == 103;
        }
        int test_subtract_different_index_types(void) {
            double double_arr[11] = {0, 0, 0, 0, 0, 0, 6.0};
            double *end_ptr = double_arr + 11;
            double *ptr1 = end_ptr - 5;
            double *ptr2 = end_ptr - 5l;
            double *ptr3 = end_ptr - 5u;
            double *ptr4 = end_ptr - 5ul;
            return (ptr1 == ptr2 && ptr1 == ptr3 && ptr1 == ptr4 && *ptr4 == 6.0);
        }
        int test_subtract_complex_expressions(void) {
            static int flag = 1;
            static int four = 4;
            static int arr[4] = {1, 2, 3, 4};
            int *ptr = (flag ? get_elem1_ptr(arr) : get_elem2_ptr(arr)) - (four / -2);
            return (*ptr == 4);
        }
        int test_subtract_multi_dimensional(void) {
            static int index = 1;
            int nested_arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            int(*last_row_pointer)[3] = nested_arr + 2;
            int(*row_pointer)[3] = last_row_pointer - index;
            return (**row_pointer == 4);
        }
        int main(void) {
            if (!test_add_constant_to_pointer()) {
                return 1;
            }
            if (!test_add_negative_index()) {
                return 2;
            }
            if (!test_add_pointer_to_int()) {
                return 3;
            }
            if (!test_add_different_index_types()) {
                return 4;
            }
            if (!test_add_complex_expressions()) {
                return 5;
            }
            if (!test_add_multi_dimensional()) {
                return 6;
            }
            if (!test_add_to_subarray_pointer()) {
                return 7;
            }
            if (!test_subtract_from_pointer()) {
                return 8;
            }
            if (!test_subtract_negative_index()) {
                return 9;
            }
            if (!test_subtract_different_index_types()) {
                return 10;
            }
            if (!test_subtract_complex_expressions()) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function test_add_constant_to_pointer() { 
            tmp.0 = sign_extend 0
            long_arr.0[0] = tmp.0
            tmp.1 = sign_extend 0
            long_arr.0[8] = tmp.1
            tmp.2 = sign_extend 3
            long_arr.0[16] = tmp.2
            tmp.3 = sign_extend 0
            long_arr.0[24] = tmp.3
            tmp.4 = sign_extend 0
            long_arr.0[32] = tmp.4
            tmp.5 = sign_extend 0
            long_arr.0[40] = tmp.5
            tmp.6 = sign_extend 0
            long_arr.0[48] = tmp.6
            tmp.7 = sign_extend 0
            long_arr.0[56] = tmp.7
            tmp.8 = sign_extend 0
            long_arr.0[64] = tmp.8
            tmp.9 = sign_extend 0
            long_arr.0[72] = tmp.9
            tmp.10 = sign_extend 13
            long_arr.0[80] = tmp.10
            long_arr.0[88] = 0L
            tmp.11 = &long_arr.0
            tmp.13 = sign_extend 10
            tmp.12 = add_ptr(tmp.11, index=tmp.13, scale=8)
            ptr.1 = tmp.12
            tmp.14 = *ptr.1
            tmp.16 = sign_extend 13
            tmp.15 = tmp.14 == tmp.16
            return tmp.15
            return 0
        }
        global function test_add_negative_index() { 
            tmp.17 = 0
            unsigned_arr.2[0] = tmp.17
            tmp.18 = 0
            unsigned_arr.2[4] = tmp.18
            tmp.19 = 2
            unsigned_arr.2[8] = tmp.19
            tmp.20 = 0
            unsigned_arr.2[12] = tmp.20
            tmp.21 = 0
            unsigned_arr.2[16] = tmp.21
            tmp.22 = 0
            unsigned_arr.2[20] = tmp.22
            tmp.23 = 0
            unsigned_arr.2[24] = tmp.23
            tmp.24 = 0
            unsigned_arr.2[28] = tmp.24
            tmp.25 = 0
            unsigned_arr.2[32] = tmp.25
            tmp.26 = 0
            unsigned_arr.2[36] = tmp.26
            tmp.27 = 42
            unsigned_arr.2[40] = tmp.27
            unsigned_arr.2[44] = 0U
            tmp.28 = &unsigned_arr.2
            tmp.30 = sign_extend 12
            tmp.29 = add_ptr(tmp.28, index=tmp.30, scale=4)
            end_ptr.3 = tmp.29
            tmp.32 = - 10
            tmp.33 = sign_extend tmp.32
            tmp.31 = add_ptr(end_ptr.3, index=tmp.33, scale=4)
            ptr.4 = tmp.31
            tmp.34 = *ptr.4
            tmp.36 = 2
            tmp.35 = tmp.34 == tmp.36
            return tmp.35
            return 0
        }
        global function test_add_pointer_to_int() { 
            int_arr.5[0] = 0
            int_arr.5[4] = 98
            int_arr.5[8] = 99
            int_arr.5[12] = 0
            int_arr.5[16] = 0
            tmp.37 = &int_arr.5
            tmp.39 = sign_extend 2
            tmp.38 = add_ptr(tmp.37, index=tmp.39, scale=4)
            ptr1.6 = tmp.38
            tmp.40 = sign_extend 2
            tmp.42 = &int_arr.5
            tmp.41 = add_ptr(tmp.42, index=tmp.40, scale=4)
            ptr2.7 = tmp.41
            tmp.43 = ptr1.6 == ptr2.7
            if !tmp.43 jump and_false_0
            tmp.46 = *ptr2.7
            tmp.47 = tmp.46 == 99
            if !tmp.47 jump and_false_0
            tmp.45 = 1
            jump and_end_1
        
          and_false_0:
            tmp.45 = 0
        
          and_end_1:
            return tmp.45
            return 0
        }
        global function test_add_different_index_types() { 
            tmp.48 = int_to_double 0
            double_arr.8[0] = tmp.48
            tmp.49 = int_to_double 0
            double_arr.8[8] = tmp.49
            tmp.50 = int_to_double 0
            double_arr.8[16] = tmp.50
            tmp.51 = int_to_double 0
            double_arr.8[24] = tmp.51
            tmp.52 = int_to_double 0
            double_arr.8[32] = tmp.52
            double_arr.8[40] = 6D
            double_arr.8[48] = 0D
            double_arr.8[56] = 0D
            double_arr.8[64] = 0D
            double_arr.8[72] = 0D
            double_arr.8[80] = 0D
            tmp.53 = &double_arr.8
            tmp.55 = sign_extend 5
            tmp.54 = add_ptr(tmp.53, index=tmp.55, scale=8)
            ptr1.9 = tmp.54
            tmp.56 = &double_arr.8
            tmp.57 = add_ptr(tmp.56, index=5L, scale=8)
            ptr2.10 = tmp.57
            tmp.58 = &double_arr.8
            tmp.60 = zero_extend 5U
            tmp.59 = add_ptr(tmp.58, index=tmp.60, scale=8)
            ptr3.11 = tmp.59
            tmp.61 = &double_arr.8
            tmp.63 = 5UL
            tmp.62 = add_ptr(tmp.61, index=tmp.63, scale=8)
            ptr4.12 = tmp.62
            tmp.64 = ptr1.9 == ptr2.10
            if !tmp.64 jump and_false_2
            tmp.67 = ptr1.9 == ptr3.11
            if !tmp.67 jump and_false_2
            tmp.66 = 1
            jump and_end_3
        
          and_false_2:
            tmp.66 = 0
        
          and_end_3:
            if !tmp.66 jump and_false_4
            tmp.70 = ptr1.9 == ptr4.12
            if !tmp.70 jump and_false_4
            tmp.69 = 1
            jump and_end_5
        
          and_false_4:
            tmp.69 = 0
        
          and_end_5:
            if !tmp.69 jump and_false_6
            tmp.73 = *ptr4.12
            tmp.74 = tmp.73 == 6D
            if !tmp.74 jump and_false_6
            tmp.72 = 1
            jump and_end_7
        
          and_false_6:
            tmp.72 = 0
        
          and_end_7:
            return tmp.72
            return 0
        }
        global function test_add_complex_expressions() { 
            tmp.75 = - 2
            i.14 = tmp.75
            tmp.76 = &i.14
            small_int_ptr.15 = tmp.76
            tmp.77 = return_one()
            tmp.79 = *small_int_ptr.15
            tmp.78 = tmp.77 + tmp.79
            tmp.80 = sign_extend tmp.78
            if !flag.13 jump else_9
            tmp.83 = &arr.18
            tmp.84 = get_elem1_ptr(tmp.83)
            tmp.82 = tmp.84
            jump end_if_8
        
          else_9:
            tmp.85 = &arr.18
            tmp.86 = get_elem2_ptr(tmp.85)
            tmp.82 = tmp.86
        
          end_if_8:
            tmp.81 = add_ptr(tmp.82, index=tmp.80, scale=4)
            ptr.19 = tmp.81
            tmp.88 = &arr.18
            tmp.90 = sign_extend 1
            tmp.89 = add_ptr(tmp.88, index=tmp.90, scale=4)
            tmp.87 = ptr.19 == tmp.89
            if !tmp.87 jump and_false_10
            tmp.93 = *ptr.19
            tmp.94 = tmp.93 == 2
            if !tmp.94 jump and_false_10
            tmp.92 = 1
            jump and_end_11
        
          and_false_10:
            tmp.92 = 0
        
          and_end_11:
            return tmp.92
            return 0
        }
        global function return_one() { 
            return 1
            return 0
        }
        global function get_elem1_ptr(arr.20) { 
            tmp.96 = sign_extend 1
            tmp.95 = add_ptr(arr.20, index=tmp.96, scale=4)
            return tmp.95
            return 0
        }
        global function get_elem2_ptr(arr.21) { 
            tmp.98 = sign_extend 2
            tmp.97 = add_ptr(arr.21, index=tmp.98, scale=4)
            return tmp.97
            return 0
        }
        global function test_add_multi_dimensional() { 
            nested_arr.23[0] = 1
            nested_arr.23[4] = 2
            nested_arr.23[8] = 3
            nested_arr.23[12] = 4
            nested_arr.23[16] = 5
            nested_arr.23[20] = 6
            nested_arr.23[24] = 7
            nested_arr.23[28] = 8
            nested_arr.23[32] = 9
            tmp.99 = &nested_arr.23
            tmp.101 = sign_extend index.22
            tmp.100 = add_ptr(tmp.99, index=tmp.101, scale=12)
            row_pointer.24 = tmp.100
            tmp.102 = *row_pointer.24
            tmp.103 = tmp.102 == 7
            return tmp.103
            return 0
        }
        global function test_add_to_subarray_pointer() { 
            nested_arr.26[0] = 1
            nested_arr.26[4] = 2
            nested_arr.26[8] = 3
            nested_arr.26[12] = 4
            nested_arr.26[16] = 5
            nested_arr.26[20] = 6
            nested_arr.26[24] = 7
            nested_arr.26[28] = 8
            nested_arr.26[32] = 9
            tmp.104 = &nested_arr.26
            tmp.106 = sign_extend 1
            tmp.105 = add_ptr(tmp.104, index=tmp.106, scale=12)
            row1.27 = tmp.105
            tmp.108 = sign_extend index.25
            tmp.107 = add_ptr(row1.27, index=tmp.108, scale=4)
            elem_ptr.28 = tmp.107
            tmp.109 = *elem_ptr.28
            tmp.110 = tmp.109 == 6
            return tmp.110
            return 0
        }
        global function test_subtract_from_pointer() { 
            tmp.111 = sign_extend 10
            long_arr.29[0] = tmp.111
            tmp.112 = sign_extend 9
            long_arr.29[8] = tmp.112
            tmp.113 = sign_extend 8
            long_arr.29[16] = tmp.113
            tmp.114 = sign_extend 7
            long_arr.29[24] = tmp.114
            tmp.115 = sign_extend 6
            long_arr.29[32] = tmp.115
            tmp.116 = &long_arr.29
            tmp.118 = sign_extend 5
            tmp.117 = add_ptr(tmp.116, index=tmp.118, scale=8)
            one_past_the_end.30 = tmp.117
            tmp.120 = sign_extend index.31
            tmp.121 = - tmp.120
            tmp.119 = add_ptr(one_past_the_end.30, index=tmp.121, scale=8)
            subtraction_result.32 = tmp.119
            tmp.122 = *subtraction_result.32
            tmp.124 = sign_extend 8
            tmp.123 = tmp.122 == tmp.124
            return tmp.123
            return 0
        }
        global function test_subtract_negative_index() { 
            tmp.125 = 100
            arr.33[0] = tmp.125
            tmp.126 = 101
            arr.33[4] = tmp.126
            tmp.127 = 102
            arr.33[8] = tmp.127
            tmp.128 = 103
            arr.33[12] = tmp.128
            tmp.129 = 104
            arr.33[16] = tmp.129
            tmp.130 = &arr.33
            tmp.132 = - 3
            tmp.133 = sign_extend tmp.132
            tmp.134 = - tmp.133
            tmp.131 = add_ptr(tmp.130, index=tmp.134, scale=4)
            ptr.34 = tmp.131
            tmp.135 = *ptr.34
            tmp.137 = 103
            tmp.136 = tmp.135 == tmp.137
            return tmp.136
            return 0
        }
        global function test_subtract_different_index_types() { 
            tmp.138 = int_to_double 0
            double_arr.35[0] = tmp.138
            tmp.139 = int_to_double 0
            double_arr.35[8] = tmp.139
            tmp.140 = int_to_double 0
            double_arr.35[16] = tmp.140
            tmp.141 = int_to_double 0
            double_arr.35[24] = tmp.141
            tmp.142 = int_to_double 0
            double_arr.35[32] = tmp.142
            tmp.143 = int_to_double 0
            double_arr.35[40] = tmp.143
            double_arr.35[48] = 6D
            double_arr.35[56] = 0D
            double_arr.35[64] = 0D
            double_arr.35[72] = 0D
            double_arr.35[80] = 0D
            tmp.144 = &double_arr.35
            tmp.146 = sign_extend 11
            tmp.145 = add_ptr(tmp.144, index=tmp.146, scale=8)
            end_ptr.36 = tmp.145
            tmp.148 = sign_extend 5
            tmp.149 = - tmp.148
            tmp.147 = add_ptr(end_ptr.36, index=tmp.149, scale=8)
            ptr1.37 = tmp.147
            tmp.151 = - 5L
            tmp.150 = add_ptr(end_ptr.36, index=tmp.151, scale=8)
            ptr2.38 = tmp.150
            tmp.153 = zero_extend 5U
            tmp.154 = - tmp.153
            tmp.152 = add_ptr(end_ptr.36, index=tmp.154, scale=8)
            ptr3.39 = tmp.152
            tmp.156 = 5UL
            tmp.157 = - tmp.156
            tmp.155 = add_ptr(end_ptr.36, index=tmp.157, scale=8)
            ptr4.40 = tmp.155
            tmp.158 = ptr1.37 == ptr2.38
            if !tmp.158 jump and_false_12
            tmp.161 = ptr1.37 == ptr3.39
            if !tmp.161 jump and_false_12
            tmp.160 = 1
            jump and_end_13
        
          and_false_12:
            tmp.160 = 0
        
          and_end_13:
            if !tmp.160 jump and_false_14
            tmp.164 = ptr1.37 == ptr4.40
            if !tmp.164 jump and_false_14
            tmp.163 = 1
            jump and_end_15
        
          and_false_14:
            tmp.163 = 0
        
          and_end_15:
            if !tmp.163 jump and_false_16
            tmp.167 = *ptr4.40
            tmp.168 = tmp.167 == 6D
            if !tmp.168 jump and_false_16
            tmp.166 = 1
            jump and_end_17
        
          and_false_16:
            tmp.166 = 0
        
          and_end_17:
            return tmp.166
            return 0
        }
        global function test_subtract_complex_expressions() { 
            if !flag.41 jump else_19
            tmp.170 = &arr.43
            tmp.171 = get_elem1_ptr(tmp.170)
            tmp.169 = tmp.171
            jump end_if_18
        
          else_19:
            tmp.172 = &arr.43
            tmp.173 = get_elem2_ptr(tmp.172)
            tmp.169 = tmp.173
        
          end_if_18:
            tmp.176 = - 2
            tmp.175 = four.42 / tmp.176
            tmp.177 = sign_extend tmp.175
            tmp.178 = - tmp.177
            tmp.174 = add_ptr(tmp.169, index=tmp.178, scale=4)
            ptr.44 = tmp.174
            tmp.179 = *ptr.44
            tmp.180 = tmp.179 == 4
            return tmp.180
            return 0
        }
        global function test_subtract_multi_dimensional() { 
            nested_arr.46[0] = 1
            nested_arr.46[4] = 2
            nested_arr.46[8] = 3
            nested_arr.46[12] = 4
            nested_arr.46[16] = 5
            nested_arr.46[20] = 6
            nested_arr.46[24] = 7
            nested_arr.46[28] = 8
            nested_arr.46[32] = 9
            tmp.181 = &nested_arr.46
            tmp.183 = sign_extend 2
            tmp.182 = add_ptr(tmp.181, index=tmp.183, scale=12)
            last_row_pointer.47 = tmp.182
            tmp.185 = sign_extend index.45
            tmp.186 = - tmp.185
            tmp.184 = add_ptr(last_row_pointer.47, index=tmp.186, scale=12)
            row_pointer.48 = tmp.184
            tmp.187 = *row_pointer.48
            tmp.188 = tmp.187 == 4
            return tmp.188
            return 0
        }
        global function main() { 
            tmp.189 = test_add_constant_to_pointer()
            tmp.190 = ! tmp.189
            if !tmp.190 jump end_if_20
            return 1
        
          end_if_20:
            tmp.191 = test_add_negative_index()
            tmp.192 = ! tmp.191
            if !tmp.192 jump end_if_22
            return 2
        
          end_if_22:
            tmp.193 = test_add_pointer_to_int()
            tmp.194 = ! tmp.193
            if !tmp.194 jump end_if_24
            return 3
        
          end_if_24:
            tmp.195 = test_add_different_index_types()
            tmp.196 = ! tmp.195
            if !tmp.196 jump end_if_26
            return 4
        
          end_if_26:
            tmp.197 = test_add_complex_expressions()
            tmp.198 = ! tmp.197
            if !tmp.198 jump end_if_28
            return 5
        
          end_if_28:
            tmp.199 = test_add_multi_dimensional()
            tmp.200 = ! tmp.199
            if !tmp.200 jump end_if_30
            return 6
        
          end_if_30:
            tmp.201 = test_add_to_subarray_pointer()
            tmp.202 = ! tmp.201
            if !tmp.202 jump end_if_32
            return 7
        
          end_if_32:
            tmp.203 = test_subtract_from_pointer()
            tmp.204 = ! tmp.203
            if !tmp.204 jump end_if_34
            return 8
        
          end_if_34:
            tmp.205 = test_subtract_negative_index()
            tmp.206 = ! tmp.205
            if !tmp.206 jump end_if_36
            return 9
        
          end_if_36:
            tmp.207 = test_subtract_different_index_types()
            tmp.208 = ! tmp.207
            if !tmp.208 jump end_if_38
            return 10
        
          end_if_38:
            tmp.209 = test_subtract_complex_expressions()
            tmp.210 = ! tmp.209
            if !tmp.210 jump end_if_40
            return 11
        
          end_if_40:
            return 0
            return 0
        }
        static arr.18: Array(4,Int) = [ 1, 2, 3, 4]
        static arr.43: Array(4,Int) = [ 1, 2, 3, 4]
        static flag.13: Int = zero[4]
        static flag.41: Int = 1
        static four.42: Int = 4
        static index.22: Int = 2
        static index.25: Int = 2
        static index.31: Int = 3
        static index.45: Int = 1
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_pointer_arithmetic_pointer_diff() {
    let src = r#"
        int get_ptr_diff(int *ptr1, int *ptr2) {
            return (ptr2 - ptr1);
        }
        int get_long_ptr_diff(long *ptr1, long *ptr2) {
            return (ptr2 - ptr1);
        }
        int get_multidim_ptr_diff(double (*ptr1)[3][5], double (*ptr2)[3][5]) {
            return (ptr2 - ptr1);
        }
        int get_multidim_ptr_diff_2(double (*ptr1)[5], double (*ptr2)[5]) {
            return (ptr2 - ptr1);
        }
        int main(void) {
            int arr[5] = {5, 4, 3, 2, 1};
            int *end_of_array = arr + 5;
            if (get_ptr_diff(arr, end_of_array) != 5) {
                return 1;
            }
            long long_arr[8];
            if (get_long_ptr_diff(long_arr + 3, long_arr) != -3) {
                return 2;
            }
            static double multidim[6][7][3][5];
            if (get_multidim_ptr_diff(multidim[2] + 1, multidim[2] + 4) != 3) {
                return 3;
            }
            if (get_multidim_ptr_diff_2(multidim[2][2] + 2, multidim[2][2]) != -2) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function get_ptr_diff(ptr1.0, ptr2.1) { 
            tmp.1 = ptr2.1 - ptr1.0
            tmp.0 = tmp.1 / 4L
            tmp.2 = truncate tmp.0
            return tmp.2
            return 0
        }
        global function get_long_ptr_diff(ptr1.2, ptr2.3) { 
            tmp.4 = ptr2.3 - ptr1.2
            tmp.3 = tmp.4 / 8L
            tmp.5 = truncate tmp.3
            return tmp.5
            return 0
        }
        global function get_multidim_ptr_diff(ptr1.4, ptr2.5) { 
            tmp.7 = ptr2.5 - ptr1.4
            tmp.6 = tmp.7 / 120L
            tmp.8 = truncate tmp.6
            return tmp.8
            return 0
        }
        global function get_multidim_ptr_diff_2(ptr1.6, ptr2.7) { 
            tmp.10 = ptr2.7 - ptr1.6
            tmp.9 = tmp.10 / 40L
            tmp.11 = truncate tmp.9
            return tmp.11
            return 0
        }
        global function main() { 
            arr.8[0] = 5
            arr.8[4] = 4
            arr.8[8] = 3
            arr.8[12] = 2
            arr.8[16] = 1
            tmp.12 = &arr.8
            tmp.14 = sign_extend 5
            tmp.13 = add_ptr(tmp.12, index=tmp.14, scale=4)
            end_of_array.9 = tmp.13
            tmp.15 = &arr.8
            tmp.16 = get_ptr_diff(tmp.15, end_of_array.9)
            tmp.17 = tmp.16 != 5
            if !tmp.17 jump end_if_0
            return 1
        
          end_if_0:
            tmp.18 = &long_arr.10
            tmp.20 = sign_extend 3
            tmp.19 = add_ptr(tmp.18, index=tmp.20, scale=8)
            tmp.21 = &long_arr.10
            tmp.22 = get_long_ptr_diff(tmp.19, tmp.21)
            tmp.24 = - 3
            tmp.23 = tmp.22 != tmp.24
            if !tmp.23 jump end_if_2
            return 2
        
          end_if_2:
            tmp.25 = &multidim.11
            tmp.26 = sign_extend 2
            tmp.27 = add_ptr(tmp.25, index=tmp.26, scale=840)
            tmp.29 = sign_extend 1
            tmp.28 = add_ptr(tmp.27, index=tmp.29, scale=120)
            tmp.30 = &multidim.11
            tmp.31 = sign_extend 2
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=840)
            tmp.34 = sign_extend 4
            tmp.33 = add_ptr(tmp.32, index=tmp.34, scale=120)
            tmp.35 = get_multidim_ptr_diff(tmp.28, tmp.33)
            tmp.36 = tmp.35 != 3
            if !tmp.36 jump end_if_4
            return 3
        
          end_if_4:
            tmp.37 = &multidim.11
            tmp.38 = sign_extend 2
            tmp.39 = add_ptr(tmp.37, index=tmp.38, scale=840)
            tmp.40 = sign_extend 2
            tmp.41 = add_ptr(tmp.39, index=tmp.40, scale=120)
            tmp.43 = sign_extend 2
            tmp.42 = add_ptr(tmp.41, index=tmp.43, scale=40)
            tmp.44 = &multidim.11
            tmp.45 = sign_extend 2
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=840)
            tmp.47 = sign_extend 2
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=120)
            tmp.49 = get_multidim_ptr_diff_2(tmp.42, tmp.48)
            tmp.51 = - 2
            tmp.50 = tmp.49 != tmp.51
            if !tmp.50 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static multidim.11: Array(6,Array(7,Array(3,Array(5,Double)))) = zero[5040]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_addition_subscript_equivalence() {
    let src = r#"
        int main(void)
        {
            unsigned long x[300][5];
            for (int i = 0; i < 300; i = i + 1) {
                for (int j = 0; j < 5; j = j + 1) {
                    x[i][j] = i * 5 + j;
                }
            }
            if (*(*(x + 20) + 3) != x[20][3]) {
                return 1;
            }
            if (&(*(*(x + 290) + 3)) != &x[290][3]) {
                return 2;
            }
            for (int i = 0; i < 300; i = i + 1) {
                for (int j = 0; j < 5; j = j + 1) {
                    if (*(*(x + i) + j) != x[i][j]) {
                        return 3;
                    }
                }
            }
            *(*(x + 275) + 4) = 22000ul;
            if (x[275][4] != 22000ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 300
            if !tmp.0 jump break_loop_0
            j.2 = 0
        
          start_loop_1:
            tmp.1 = j.2 < 5
            if !tmp.1 jump break_loop_1
            tmp.2 = &x.0
            tmp.3 = sign_extend i.1
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=40)
            tmp.5 = sign_extend j.2
            tmp.6 = add_ptr(tmp.4, index=tmp.5, scale=8)
            tmp.7 = i.1 * 5
            tmp.8 = tmp.7 + j.2
            tmp.9 = sign_extend tmp.8
            *tmp.6 = tmp.9
        
          continue_loop_1:
            tmp.10 = j.2 + 1
            j.2 = tmp.10
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.11 = i.1 + 1
            i.1 = tmp.11
            jump start_loop_0
        
          break_loop_0:
            tmp.12 = &x.0
            tmp.14 = sign_extend 20
            tmp.13 = add_ptr(tmp.12, index=tmp.14, scale=40)
            tmp.16 = sign_extend 3
            tmp.15 = add_ptr(tmp.13, index=tmp.16, scale=8)
            tmp.17 = *tmp.15
            tmp.19 = &x.0
            tmp.20 = sign_extend 20
            tmp.21 = add_ptr(tmp.19, index=tmp.20, scale=40)
            tmp.22 = sign_extend 3
            tmp.23 = add_ptr(tmp.21, index=tmp.22, scale=8)
            tmp.24 = *tmp.23
            tmp.18 = tmp.17 != tmp.24
            if !tmp.18 jump end_if_0
            return 1
        
          end_if_0:
            tmp.25 = &x.0
            tmp.27 = sign_extend 290
            tmp.26 = add_ptr(tmp.25, index=tmp.27, scale=40)
            tmp.29 = sign_extend 3
            tmp.28 = add_ptr(tmp.26, index=tmp.29, scale=8)
            tmp.31 = &x.0
            tmp.32 = sign_extend 290
            tmp.33 = add_ptr(tmp.31, index=tmp.32, scale=40)
            tmp.34 = sign_extend 3
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=8)
            tmp.30 = tmp.28 != tmp.35
            if !tmp.30 jump end_if_2
            return 2
        
          end_if_2:
            i.3 = 0
        
          start_loop_2:
            tmp.36 = i.3 < 300
            if !tmp.36 jump break_loop_2
            j.4 = 0
        
          start_loop_3:
            tmp.37 = j.4 < 5
            if !tmp.37 jump break_loop_3
            tmp.38 = &x.0
            tmp.40 = sign_extend i.3
            tmp.39 = add_ptr(tmp.38, index=tmp.40, scale=40)
            tmp.42 = sign_extend j.4
            tmp.41 = add_ptr(tmp.39, index=tmp.42, scale=8)
            tmp.43 = *tmp.41
            tmp.45 = &x.0
            tmp.46 = sign_extend i.3
            tmp.47 = add_ptr(tmp.45, index=tmp.46, scale=40)
            tmp.48 = sign_extend j.4
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=8)
            tmp.50 = *tmp.49
            tmp.44 = tmp.43 != tmp.50
            if !tmp.44 jump end_if_4
            return 3
        
          end_if_4:
        
          continue_loop_3:
            tmp.51 = j.4 + 1
            j.4 = tmp.51
            jump start_loop_3
        
          break_loop_3:
        
          continue_loop_2:
            tmp.52 = i.3 + 1
            i.3 = tmp.52
            jump start_loop_2
        
          break_loop_2:
            tmp.53 = &x.0
            tmp.55 = sign_extend 275
            tmp.54 = add_ptr(tmp.53, index=tmp.55, scale=40)
            tmp.57 = sign_extend 4
            tmp.56 = add_ptr(tmp.54, index=tmp.57, scale=8)
            *tmp.56 = 22000UL
            tmp.58 = &x.0
            tmp.59 = sign_extend 275
            tmp.60 = add_ptr(tmp.58, index=tmp.59, scale=40)
            tmp.61 = sign_extend 4
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=8)
            tmp.63 = *tmp.62
            tmp.64 = tmp.63 != 22000UL
            if !tmp.64 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_array_of_pointers_to_arrays() {
    let src = r#"
        int main(void) {
            int x = 0;
            int y = 1;
            int z = 2;
            int *arr[3] = { &x, &y, &z };
            int *arr2[3] = {&z, &y, &x};
            int *(*array_of_pointers[3])[3] = {&arr, &arr2, &arr};
            if (array_of_pointers[0] != (int *(*)[3]) arr) {
                return 1;
            }
            if (array_of_pointers[1] != (int *(*)[3]) arr2) {
                return 2;
            }
            if (array_of_pointers[2] != (int *(*)[3]) arr) {
                return 3;
            }
            if (array_of_pointers[1][0][0] != &z) {
                return 4;
            }
            if (array_of_pointers[1][0][1] != &y) {
                return 5;
            }
            if (array_of_pointers[2][0][2][0] != 2) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 0
            y.1 = 1
            z.2 = 2
            tmp.0 = &x.0
            arr.3[0] = tmp.0
            tmp.1 = &y.1
            arr.3[8] = tmp.1
            tmp.2 = &z.2
            arr.3[16] = tmp.2
            tmp.3 = &z.2
            arr2.4[0] = tmp.3
            tmp.4 = &y.1
            arr2.4[8] = tmp.4
            tmp.5 = &x.0
            arr2.4[16] = tmp.5
            tmp.6 = &arr.3
            array_of_pointers.5[0] = tmp.6
            tmp.7 = &arr2.4
            array_of_pointers.5[8] = tmp.7
            tmp.8 = &arr.3
            array_of_pointers.5[16] = tmp.8
            tmp.9 = &array_of_pointers.5
            tmp.10 = sign_extend 0
            tmp.11 = add_ptr(tmp.9, index=tmp.10, scale=8)
            tmp.12 = *tmp.11
            tmp.14 = &arr.3
            tmp.15 = tmp.14
            tmp.13 = tmp.12 != tmp.15
            if !tmp.13 jump end_if_0
            return 1
        
          end_if_0:
            tmp.16 = &array_of_pointers.5
            tmp.17 = sign_extend 1
            tmp.18 = add_ptr(tmp.16, index=tmp.17, scale=8)
            tmp.19 = *tmp.18
            tmp.21 = &arr2.4
            tmp.22 = tmp.21
            tmp.20 = tmp.19 != tmp.22
            if !tmp.20 jump end_if_2
            return 2
        
          end_if_2:
            tmp.23 = &array_of_pointers.5
            tmp.24 = sign_extend 2
            tmp.25 = add_ptr(tmp.23, index=tmp.24, scale=8)
            tmp.26 = *tmp.25
            tmp.28 = &arr.3
            tmp.29 = tmp.28
            tmp.27 = tmp.26 != tmp.29
            if !tmp.27 jump end_if_4
            return 3
        
          end_if_4:
            tmp.30 = &array_of_pointers.5
            tmp.31 = sign_extend 1
            tmp.32 = add_ptr(tmp.30, index=tmp.31, scale=8)
            tmp.33 = *tmp.32
            tmp.34 = sign_extend 0
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=24)
            tmp.36 = sign_extend 0
            tmp.37 = add_ptr(tmp.35, index=tmp.36, scale=8)
            tmp.38 = *tmp.37
            tmp.40 = &z.2
            tmp.39 = tmp.38 != tmp.40
            if !tmp.39 jump end_if_6
            return 4
        
          end_if_6:
            tmp.41 = &array_of_pointers.5
            tmp.42 = sign_extend 1
            tmp.43 = add_ptr(tmp.41, index=tmp.42, scale=8)
            tmp.44 = *tmp.43
            tmp.45 = sign_extend 0
            tmp.46 = add_ptr(tmp.44, index=tmp.45, scale=24)
            tmp.47 = sign_extend 1
            tmp.48 = add_ptr(tmp.46, index=tmp.47, scale=8)
            tmp.49 = *tmp.48
            tmp.51 = &y.1
            tmp.50 = tmp.49 != tmp.51
            if !tmp.50 jump end_if_8
            return 5
        
          end_if_8:
            tmp.52 = &array_of_pointers.5
            tmp.53 = sign_extend 2
            tmp.54 = add_ptr(tmp.52, index=tmp.53, scale=8)
            tmp.55 = *tmp.54
            tmp.56 = sign_extend 0
            tmp.57 = add_ptr(tmp.55, index=tmp.56, scale=24)
            tmp.58 = sign_extend 2
            tmp.59 = add_ptr(tmp.57, index=tmp.58, scale=8)
            tmp.60 = *tmp.59
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=4)
            tmp.63 = *tmp.62
            tmp.64 = tmp.63 != 2
            if !tmp.64 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_complex_operands() {
    let src = r#"
        int assign_in_index(int idx) {
            int arr[3] = {1, 2, 3};
            int val = arr[idx = idx + 2];
            if (idx != 1) {
                return 1;
            }
            if (val != 2) {
                return 2;
            }
            return 0;
        }
        int static_index(void) {
            static int index = 0;
            int retval = index;
            index = index + 1;
            return retval;
        }
        int funcall_in_index(void) {
            int arr[3] = {1, 2, 3};
            int v1 = arr[static_index()];
            int v2 = arr[static_index()];
            if (v1 != 1) {
                return 3;
            }
            if (v2 != 2) {
                return 4;
            }
            return 0;
        }
        int subscript_inception(long *arr, int *a, int b){
            return arr[a[b]];
        }
        int check_subscript_inception(void) {
            long arr[4] = {4, 3, 2, 1};
            int indices[2] = {1, 2};
            if (subscript_inception(arr, indices, 1) != 2) {
                return 5;
            }
            if (subscript_inception(arr, indices, 0) != 3) {
                return 6;
            }
            return 0;
        }
        int *get_array(void) {
            static int arr[3];
            return arr;
        }
        int subscript_function_result(void){
            get_array()[2] = 1;
            if (get_array()[2] != 1) {
                return 7;
            }
            return 0;
        }
        int negate_subscript(int *arr, int idx, int expected) {
            if (arr[-idx] != expected) {
                return 8;
            }
            return 0;
        }
        int main(void) {
            int check = assign_in_index(-1);
            if (check) {
                return check;
            }
            check = funcall_in_index();
            if (check) {
                return check;
            }
            check = check_subscript_inception();
            if (check) {
                return check;
            }
            check = subscript_function_result();
            if (check) {
                return check;
            }
            int arr[3] = {0, 1, 2};
            check = negate_subscript(arr + 2, 2, 0);
            if (check) {
                return check;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function assign_in_index(idx.0) { 
            arr.1[0] = 1
            arr.1[4] = 2
            arr.1[8] = 3
            tmp.0 = &arr.1
            tmp.1 = idx.0 + 2
            idx.0 = tmp.1
            tmp.2 = sign_extend tmp.1
            tmp.3 = add_ptr(tmp.0, index=tmp.2, scale=4)
            tmp.4 = *tmp.3
            val.2 = tmp.4
            tmp.5 = idx.0 != 1
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = val.2 != 2
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        global function static_index() { 
            retval.4 = index.3
            tmp.7 = index.3 + 1
            index.3 = tmp.7
            return retval.4
            return 0
        }
        global function funcall_in_index() { 
            arr.5[0] = 1
            arr.5[4] = 2
            arr.5[8] = 3
            tmp.8 = &arr.5
            tmp.9 = static_index()
            tmp.10 = sign_extend tmp.9
            tmp.11 = add_ptr(tmp.8, index=tmp.10, scale=4)
            tmp.12 = *tmp.11
            v1.6 = tmp.12
            tmp.13 = &arr.5
            tmp.14 = static_index()
            tmp.15 = sign_extend tmp.14
            tmp.16 = add_ptr(tmp.13, index=tmp.15, scale=4)
            tmp.17 = *tmp.16
            v2.7 = tmp.17
            tmp.18 = v1.6 != 1
            if !tmp.18 jump end_if_4
            return 3
        
          end_if_4:
            tmp.19 = v2.7 != 2
            if !tmp.19 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        global function subscript_inception(arr.8, a.9, b.10) { 
            tmp.20 = sign_extend b.10
            tmp.21 = add_ptr(a.9, index=tmp.20, scale=4)
            tmp.22 = *tmp.21
            tmp.23 = sign_extend tmp.22
            tmp.24 = add_ptr(arr.8, index=tmp.23, scale=8)
            tmp.25 = *tmp.24
            tmp.26 = truncate tmp.25
            return tmp.26
            return 0
        }
        global function check_subscript_inception() { 
            tmp.27 = sign_extend 4
            arr.11[0] = tmp.27
            tmp.28 = sign_extend 3
            arr.11[8] = tmp.28
            tmp.29 = sign_extend 2
            arr.11[16] = tmp.29
            tmp.30 = sign_extend 1
            arr.11[24] = tmp.30
            indices.12[0] = 1
            indices.12[4] = 2
            tmp.31 = &arr.11
            tmp.32 = &indices.12
            tmp.33 = subscript_inception(tmp.31, tmp.32, 1)
            tmp.34 = tmp.33 != 2
            if !tmp.34 jump end_if_8
            return 5
        
          end_if_8:
            tmp.35 = &arr.11
            tmp.36 = &indices.12
            tmp.37 = subscript_inception(tmp.35, tmp.36, 0)
            tmp.38 = tmp.37 != 3
            if !tmp.38 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        global function get_array() { 
            tmp.39 = &arr.13
            return tmp.39
            return 0
        }
        global function subscript_function_result() { 
            tmp.40 = get_array()
            tmp.41 = sign_extend 2
            tmp.42 = add_ptr(tmp.40, index=tmp.41, scale=4)
            *tmp.42 = 1
            tmp.43 = get_array()
            tmp.44 = sign_extend 2
            tmp.45 = add_ptr(tmp.43, index=tmp.44, scale=4)
            tmp.46 = *tmp.45
            tmp.47 = tmp.46 != 1
            if !tmp.47 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
        global function negate_subscript(arr.14, idx.15, expected.16) { 
            tmp.48 = - idx.15
            tmp.49 = sign_extend tmp.48
            tmp.50 = add_ptr(arr.14, index=tmp.49, scale=4)
            tmp.51 = *tmp.50
            tmp.52 = tmp.51 != expected.16
            if !tmp.52 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        global function main() { 
            tmp.53 = - 1
            tmp.54 = assign_in_index(tmp.53)
            check.17 = tmp.54
            if !check.17 jump end_if_16
            return check.17
        
          end_if_16:
            tmp.55 = funcall_in_index()
            check.17 = tmp.55
            if !check.17 jump end_if_18
            return check.17
        
          end_if_18:
            tmp.56 = check_subscript_inception()
            check.17 = tmp.56
            if !check.17 jump end_if_20
            return check.17
        
          end_if_20:
            tmp.57 = subscript_function_result()
            check.17 = tmp.57
            if !check.17 jump end_if_22
            return check.17
        
          end_if_22:
            arr.18[0] = 0
            arr.18[4] = 1
            arr.18[8] = 2
            tmp.58 = &arr.18
            tmp.60 = sign_extend 2
            tmp.59 = add_ptr(tmp.58, index=tmp.60, scale=4)
            tmp.61 = negate_subscript(tmp.59, 2, 0)
            check.17 = tmp.61
            if !check.17 jump end_if_24
            return check.17
        
          end_if_24:
            return 0
            return 0
        }
        static arr.13: Array(3,Int) = zero[12]
        static index.3: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_simple() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            return arr[2];
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.0[0] = 1
            arr.0[4] = 2
            arr.0[8] = 3
            tmp.0 = &arr.0
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=4)
            tmp.3 = *tmp.2
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_simple_subscripts() {
    let src = r#"
        int integer_types(unsigned *arr, unsigned expected) {
            unsigned val1 = arr[5];
            unsigned val2 = arr[5u];
            unsigned val3 = arr[5l];
            unsigned val4 = arr[5ul];
            if (val1 != expected) {
                return 1;
            }
            if (val2 != expected) {
                return 2;
            }
            if (val3 != expected) {
                return 3;
            }
            if (val4 != expected) {
                return 4;
            }
            return 0;
        }
        int reverse_subscript(long *arr, long expected) {
            if (arr[3] != expected) {
                return 5;
            }
            if (3[arr] != expected) {
                return 6;
            }
            if (&3[arr] != &arr[3]) {
                return 7;
            }
            return 0;
        }
        static double static_array[3] = {0.1, 0.2, 0.3};
        int subscript_static(void) {
            if (static_array[0] != 0.1) {
                return 8;
            }
            if (static_array[1] != 0.2) {
                return 9;
            }
            if (static_array[2] != 0.3) {
                return 10;
            }
            return 0;
        }
        int update_element(int *arr, int expected) {
            arr[10] = arr[10] * 2;
            if (arr[10] != expected) {
                return 11;
            }
            return 0;
        }
        int *increment_static_element(void) {
            static int arr[4];
            arr[3] = arr[3] + 1;
            return arr;
        }
        int check_increment_static_element(void) {
            int *arr1 = increment_static_element();
            if (arr1[3] != 1) {
                return 12;
            }
            if (arr1[0] || arr1[1] || arr1[2]) {
                return 13;
            }
            int *arr2 = increment_static_element();
            if (arr1 != arr2) {
                return 14;
            }
            if (arr1[3] != 2) {
                return 15;
            }
            return 0;
        }
        int main(void) {
            unsigned int unsigned_arr[6] = {0, 0, 0, 0, 0, 7u};
            int check = integer_types(unsigned_arr, 7u);
            if (check) {
                return check;
            }
            long int long_arr[4] = {100, 102, 104, 106};
            check = reverse_subscript(long_arr, 106);
            if (check) {
                return check;
            }
            check = subscript_static();
            if (check) {
                return check;
            }
            int int_arr[11] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15};
            check = update_element(int_arr, 30);
            if (check) {
                return check;
            }
            check = check_increment_static_element();
            if (check) {
                return check;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function integer_types(arr.0, expected.1) { 
            tmp.0 = sign_extend 5
            tmp.1 = add_ptr(arr.0, index=tmp.0, scale=4)
            tmp.2 = *tmp.1
            val1.2 = tmp.2
            tmp.3 = zero_extend 5U
            tmp.4 = add_ptr(arr.0, index=tmp.3, scale=4)
            tmp.5 = *tmp.4
            val2.3 = tmp.5
            tmp.6 = add_ptr(arr.0, index=5L, scale=4)
            tmp.7 = *tmp.6
            val3.4 = tmp.7
            tmp.8 = 5UL
            tmp.9 = add_ptr(arr.0, index=tmp.8, scale=4)
            tmp.10 = *tmp.9
            val4.5 = tmp.10
            tmp.11 = val1.2 != expected.1
            if !tmp.11 jump end_if_0
            return 1
        
          end_if_0:
            tmp.12 = val2.3 != expected.1
            if !tmp.12 jump end_if_2
            return 2
        
          end_if_2:
            tmp.13 = val3.4 != expected.1
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            tmp.14 = val4.5 != expected.1
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        global function reverse_subscript(arr.6, expected.7) { 
            tmp.15 = sign_extend 3
            tmp.16 = add_ptr(arr.6, index=tmp.15, scale=8)
            tmp.17 = *tmp.16
            tmp.18 = tmp.17 != expected.7
            if !tmp.18 jump end_if_8
            return 5
        
          end_if_8:
            tmp.19 = sign_extend 3
            tmp.20 = add_ptr(arr.6, index=tmp.19, scale=8)
            tmp.21 = *tmp.20
            tmp.22 = tmp.21 != expected.7
            if !tmp.22 jump end_if_10
            return 6
        
          end_if_10:
            tmp.23 = sign_extend 3
            tmp.24 = add_ptr(arr.6, index=tmp.23, scale=8)
            tmp.26 = sign_extend 3
            tmp.27 = add_ptr(arr.6, index=tmp.26, scale=8)
            tmp.25 = tmp.24 != tmp.27
            if !tmp.25 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
        global function subscript_static() { 
            tmp.28 = &static_array
            tmp.29 = sign_extend 0
            tmp.30 = add_ptr(tmp.28, index=tmp.29, scale=8)
            tmp.31 = *tmp.30
            tmp.32 = tmp.31 != 0.1D
            if !tmp.32 jump end_if_14
            return 8
        
          end_if_14:
            tmp.33 = &static_array
            tmp.34 = sign_extend 1
            tmp.35 = add_ptr(tmp.33, index=tmp.34, scale=8)
            tmp.36 = *tmp.35
            tmp.37 = tmp.36 != 0.2D
            if !tmp.37 jump end_if_16
            return 9
        
          end_if_16:
            tmp.38 = &static_array
            tmp.39 = sign_extend 2
            tmp.40 = add_ptr(tmp.38, index=tmp.39, scale=8)
            tmp.41 = *tmp.40
            tmp.42 = tmp.41 != 0.3D
            if !tmp.42 jump end_if_18
            return 10
        
          end_if_18:
            return 0
            return 0
        }
        global function update_element(arr.8, expected.9) { 
            tmp.43 = sign_extend 10
            tmp.44 = add_ptr(arr.8, index=tmp.43, scale=4)
            tmp.45 = sign_extend 10
            tmp.46 = add_ptr(arr.8, index=tmp.45, scale=4)
            tmp.47 = *tmp.46
            tmp.48 = tmp.47 * 2
            *tmp.44 = tmp.48
            tmp.49 = sign_extend 10
            tmp.50 = add_ptr(arr.8, index=tmp.49, scale=4)
            tmp.51 = *tmp.50
            tmp.52 = tmp.51 != expected.9
            if !tmp.52 jump end_if_20
            return 11
        
          end_if_20:
            return 0
            return 0
        }
        global function increment_static_element() { 
            tmp.53 = &arr.10
            tmp.54 = sign_extend 3
            tmp.55 = add_ptr(tmp.53, index=tmp.54, scale=4)
            tmp.56 = &arr.10
            tmp.57 = sign_extend 3
            tmp.58 = add_ptr(tmp.56, index=tmp.57, scale=4)
            tmp.59 = *tmp.58
            tmp.60 = tmp.59 + 1
            *tmp.55 = tmp.60
            tmp.61 = &arr.10
            return tmp.61
            return 0
        }
        global function check_increment_static_element() { 
            tmp.62 = increment_static_element()
            arr1.11 = tmp.62
            tmp.63 = sign_extend 3
            tmp.64 = add_ptr(arr1.11, index=tmp.63, scale=4)
            tmp.65 = *tmp.64
            tmp.66 = tmp.65 != 1
            if !tmp.66 jump end_if_22
            return 12
        
          end_if_22:
            tmp.67 = sign_extend 0
            tmp.68 = add_ptr(arr1.11, index=tmp.67, scale=4)
            tmp.69 = *tmp.68
            if tmp.69 jump or_true_24
            tmp.72 = sign_extend 1
            tmp.73 = add_ptr(arr1.11, index=tmp.72, scale=4)
            tmp.74 = *tmp.73
            if tmp.74 jump or_true_24
            tmp.71 = 0
            jump or_end_25
        
          or_true_24:
            tmp.71 = 1
        
          or_end_25:
            if tmp.71 jump or_true_26
            tmp.77 = sign_extend 2
            tmp.78 = add_ptr(arr1.11, index=tmp.77, scale=4)
            tmp.79 = *tmp.78
            if tmp.79 jump or_true_26
            tmp.76 = 0
            jump or_end_27
        
          or_true_26:
            tmp.76 = 1
        
          or_end_27:
            if !tmp.76 jump end_if_28
            return 13
        
          end_if_28:
            tmp.80 = increment_static_element()
            arr2.12 = tmp.80
            tmp.81 = arr1.11 != arr2.12
            if !tmp.81 jump end_if_30
            return 14
        
          end_if_30:
            tmp.82 = sign_extend 3
            tmp.83 = add_ptr(arr1.11, index=tmp.82, scale=4)
            tmp.84 = *tmp.83
            tmp.85 = tmp.84 != 2
            if !tmp.85 jump end_if_32
            return 15
        
          end_if_32:
            return 0
            return 0
        }
        global function main() { 
            tmp.86 = 0
            unsigned_arr.13[0] = tmp.86
            tmp.87 = 0
            unsigned_arr.13[4] = tmp.87
            tmp.88 = 0
            unsigned_arr.13[8] = tmp.88
            tmp.89 = 0
            unsigned_arr.13[12] = tmp.89
            tmp.90 = 0
            unsigned_arr.13[16] = tmp.90
            unsigned_arr.13[20] = 7U
            tmp.91 = &unsigned_arr.13
            tmp.92 = integer_types(tmp.91, 7U)
            check.14 = tmp.92
            if !check.14 jump end_if_34
            return check.14
        
          end_if_34:
            tmp.93 = sign_extend 100
            long_arr.15[0] = tmp.93
            tmp.94 = sign_extend 102
            long_arr.15[8] = tmp.94
            tmp.95 = sign_extend 104
            long_arr.15[16] = tmp.95
            tmp.96 = sign_extend 106
            long_arr.15[24] = tmp.96
            tmp.97 = &long_arr.15
            tmp.98 = sign_extend 106
            tmp.99 = reverse_subscript(tmp.97, tmp.98)
            check.14 = tmp.99
            if !check.14 jump end_if_36
            return check.14
        
          end_if_36:
            tmp.100 = subscript_static()
            check.14 = tmp.100
            if !check.14 jump end_if_38
            return check.14
        
          end_if_38:
            int_arr.16[0] = 0
            int_arr.16[4] = 0
            int_arr.16[8] = 0
            int_arr.16[12] = 0
            int_arr.16[16] = 0
            int_arr.16[20] = 0
            int_arr.16[24] = 0
            int_arr.16[28] = 0
            int_arr.16[32] = 0
            int_arr.16[36] = 0
            int_arr.16[40] = 15
            tmp.101 = &int_arr.16
            tmp.102 = update_element(tmp.101, 30)
            check.14 = tmp.102
            if !check.14 jump end_if_40
            return check.14
        
          end_if_40:
            tmp.103 = check_increment_static_element()
            check.14 = tmp.103
            if !check.14 jump end_if_42
            return check.14
        
          end_if_42:
            return 0
            return 0
        }
        static arr.10: Array(4,Int) = zero[16]
        static static_array: Array(3,Double) = [ 0.1D, 0.2D, 0.3D]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_subscript_nested() {
    let src = r#"
        int read_nested(int nested_arr[2][3], int i, int j, int expected) {
            return (nested_arr[i][j] == expected);
        }
        int write_nested(int nested_arr[2][3], int i, int j, int new_val) {
            nested_arr[i][j] = new_val;
            return 0;
        }
        int read_nested_negated(int (*nested_arr)[3], int i, int j, int expected) {
            return (nested_arr[-i][j] == expected);
        }
        int get_nested_addr(int nested_arr[2][3], int i, int j, int *expected) {
            return &nested_arr[i][j] == expected;
        }
        static int nested_arr[4][3][5] = {
            {{1, 2}, {3}},
            {{4}, {5}}
        };
        int read_static_nested(int i, int j, int k, int expected) {
            return nested_arr[i][j][k] == expected;
        }
        int (*get_array(void))[3][5] {
            return nested_arr;
        }
        int write_nested_complex(int i, int j, int k, int val) {
            get_array()[i][j][k] = val;
            return 0;
        }
        int *get_subarray(int nested[2][3], int i) {
            return nested[i];
        }
        int main(void) {
            int nested_arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            if (!read_nested(nested_arr, 1, 2, 6)) {
                return 1;
            }
            write_nested(nested_arr, 1, 2, -1);
            if (nested_arr[1][2] != -1) {
                return 2;
            }
            if (!read_nested_negated(nested_arr + 2, 2, 0, 1)) {
                return 3;
            }
            int *ptr = (nested_arr[0]) + 1;
            if (!get_nested_addr(nested_arr, 0, 1, ptr)) {
                return 4;
            }
            if (!read_static_nested(1, 1, 0, 5)) {
                return 5;
            }
            write_nested_complex(0, 2, 3, 111);
            if (get_array()[0][2][3] != 111) {
                return 6;
            }
            int *row_1 = get_subarray(nested_arr, 1);
            if (row_1 + 1 != &nested_arr[1][1]) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function read_nested(nested_arr.0, i.1, j.2, expected.3) { 
            tmp.0 = sign_extend i.1
            tmp.1 = add_ptr(nested_arr.0, index=tmp.0, scale=12)
            tmp.2 = sign_extend j.2
            tmp.3 = add_ptr(tmp.1, index=tmp.2, scale=4)
            tmp.4 = *tmp.3
            tmp.5 = tmp.4 == expected.3
            return tmp.5
            return 0
        }
        global function write_nested(nested_arr.4, i.5, j.6, new_val.7) { 
            tmp.6 = sign_extend i.5
            tmp.7 = add_ptr(nested_arr.4, index=tmp.6, scale=12)
            tmp.8 = sign_extend j.6
            tmp.9 = add_ptr(tmp.7, index=tmp.8, scale=4)
            *tmp.9 = new_val.7
            return 0
            return 0
        }
        global function read_nested_negated(nested_arr.8, i.9, j.10, expected.11) { 
            tmp.10 = - i.9
            tmp.11 = sign_extend tmp.10
            tmp.12 = add_ptr(nested_arr.8, index=tmp.11, scale=12)
            tmp.13 = sign_extend j.10
            tmp.14 = add_ptr(tmp.12, index=tmp.13, scale=4)
            tmp.15 = *tmp.14
            tmp.16 = tmp.15 == expected.11
            return tmp.16
            return 0
        }
        global function get_nested_addr(nested_arr.12, i.13, j.14, expected.15) { 
            tmp.17 = sign_extend i.13
            tmp.18 = add_ptr(nested_arr.12, index=tmp.17, scale=12)
            tmp.19 = sign_extend j.14
            tmp.20 = add_ptr(tmp.18, index=tmp.19, scale=4)
            tmp.21 = tmp.20 == expected.15
            return tmp.21
            return 0
        }
        global function read_static_nested(i.16, j.17, k.18, expected.19) { 
            tmp.22 = &nested_arr
            tmp.23 = sign_extend i.16
            tmp.24 = add_ptr(tmp.22, index=tmp.23, scale=60)
            tmp.25 = sign_extend j.17
            tmp.26 = add_ptr(tmp.24, index=tmp.25, scale=20)
            tmp.27 = sign_extend k.18
            tmp.28 = add_ptr(tmp.26, index=tmp.27, scale=4)
            tmp.29 = *tmp.28
            tmp.30 = tmp.29 == expected.19
            return tmp.30
            return 0
        }
        global function get_array() { 
            tmp.31 = &nested_arr
            return tmp.31
            return 0
        }
        global function write_nested_complex(i.20, j.21, k.22, val.23) { 
            tmp.32 = get_array()
            tmp.33 = sign_extend i.20
            tmp.34 = add_ptr(tmp.32, index=tmp.33, scale=60)
            tmp.35 = sign_extend j.21
            tmp.36 = add_ptr(tmp.34, index=tmp.35, scale=20)
            tmp.37 = sign_extend k.22
            tmp.38 = add_ptr(tmp.36, index=tmp.37, scale=4)
            *tmp.38 = val.23
            return 0
            return 0
        }
        global function get_subarray(nested.24, i.25) { 
            tmp.39 = sign_extend i.25
            tmp.40 = add_ptr(nested.24, index=tmp.39, scale=12)
            return tmp.40
            return 0
        }
        global function main() { 
            nested_arr.26[0] = 1
            nested_arr.26[4] = 2
            nested_arr.26[8] = 3
            nested_arr.26[12] = 4
            nested_arr.26[16] = 5
            nested_arr.26[20] = 6
            tmp.41 = &nested_arr.26
            tmp.42 = read_nested(tmp.41, 1, 2, 6)
            tmp.43 = ! tmp.42
            if !tmp.43 jump end_if_0
            return 1
        
          end_if_0:
            tmp.44 = &nested_arr.26
            tmp.45 = - 1
            tmp.46 = write_nested(tmp.44, 1, 2, tmp.45)
            tmp.47 = &nested_arr.26
            tmp.48 = sign_extend 1
            tmp.49 = add_ptr(tmp.47, index=tmp.48, scale=12)
            tmp.50 = sign_extend 2
            tmp.51 = add_ptr(tmp.49, index=tmp.50, scale=4)
            tmp.52 = *tmp.51
            tmp.54 = - 1
            tmp.53 = tmp.52 != tmp.54
            if !tmp.53 jump end_if_2
            return 2
        
          end_if_2:
            tmp.55 = &nested_arr.26
            tmp.57 = sign_extend 2
            tmp.56 = add_ptr(tmp.55, index=tmp.57, scale=12)
            tmp.58 = read_nested_negated(tmp.56, 2, 0, 1)
            tmp.59 = ! tmp.58
            if !tmp.59 jump end_if_4
            return 3
        
          end_if_4:
            tmp.60 = &nested_arr.26
            tmp.61 = sign_extend 0
            tmp.62 = add_ptr(tmp.60, index=tmp.61, scale=12)
            tmp.64 = sign_extend 1
            tmp.63 = add_ptr(tmp.62, index=tmp.64, scale=4)
            ptr.27 = tmp.63
            tmp.65 = &nested_arr.26
            tmp.66 = get_nested_addr(tmp.65, 0, 1, ptr.27)
            tmp.67 = ! tmp.66
            if !tmp.67 jump end_if_6
            return 4
        
          end_if_6:
            tmp.68 = read_static_nested(1, 1, 0, 5)
            tmp.69 = ! tmp.68
            if !tmp.69 jump end_if_8
            return 5
        
          end_if_8:
            tmp.70 = write_nested_complex(0, 2, 3, 111)
            tmp.71 = get_array()
            tmp.72 = sign_extend 0
            tmp.73 = add_ptr(tmp.71, index=tmp.72, scale=60)
            tmp.74 = sign_extend 2
            tmp.75 = add_ptr(tmp.73, index=tmp.74, scale=20)
            tmp.76 = sign_extend 3
            tmp.77 = add_ptr(tmp.75, index=tmp.76, scale=4)
            tmp.78 = *tmp.77
            tmp.79 = tmp.78 != 111
            if !tmp.79 jump end_if_10
            return 6
        
          end_if_10:
            tmp.80 = &nested_arr.26
            tmp.81 = get_subarray(tmp.80, 1)
            row_1.28 = tmp.81
            tmp.83 = sign_extend 1
            tmp.82 = add_ptr(row_1.28, index=tmp.83, scale=4)
            tmp.85 = &nested_arr.26
            tmp.86 = sign_extend 1
            tmp.87 = add_ptr(tmp.85, index=tmp.86, scale=12)
            tmp.88 = sign_extend 1
            tmp.89 = add_ptr(tmp.87, index=tmp.88, scale=4)
            tmp.84 = tmp.82 != tmp.89
            if !tmp.84 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
        static nested_arr: Array(4,Array(3,Array(5,Int))) = [ 1, 2, zero[12], 3, zero[16], zero[20], 4, zero[16], 5, zero[16], zero[20], zero[120]]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_subscript_pointer() {
    let src = r#"
        int subscript_pointer_to_pointer(int **x) {
            return x[0][0];
        }
        int main(void) {
            int a = 3;
            int *ptr = &a;
            if (ptr[0] != 3) {
                return 1;
            }
            int **ptr_ptr = &ptr;
            if (ptr_ptr[0][0] != 3) {
                return 2;
            }
            int dereferenced = subscript_pointer_to_pointer(ptr_ptr);
            if (dereferenced != 3) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function subscript_pointer_to_pointer(x.0) { 
            tmp.0 = sign_extend 0
            tmp.1 = add_ptr(x.0, index=tmp.0, scale=8)
            tmp.2 = *tmp.1
            tmp.3 = sign_extend 0
            tmp.4 = add_ptr(tmp.2, index=tmp.3, scale=4)
            tmp.5 = *tmp.4
            return tmp.5
            return 0
        }
        global function main() { 
            a.1 = 3
            tmp.6 = &a.1
            ptr.2 = tmp.6
            tmp.7 = sign_extend 0
            tmp.8 = add_ptr(ptr.2, index=tmp.7, scale=4)
            tmp.9 = *tmp.8
            tmp.10 = tmp.9 != 3
            if !tmp.10 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = &ptr.2
            ptr_ptr.3 = tmp.11
            tmp.12 = sign_extend 0
            tmp.13 = add_ptr(ptr_ptr.3, index=tmp.12, scale=8)
            tmp.14 = *tmp.13
            tmp.15 = sign_extend 0
            tmp.16 = add_ptr(tmp.14, index=tmp.15, scale=4)
            tmp.17 = *tmp.16
            tmp.18 = tmp.17 != 3
            if !tmp.18 jump end_if_2
            return 2
        
          end_if_2:
            tmp.19 = subscript_pointer_to_pointer(ptr_ptr.3)
            dereferenced.4 = tmp.19
            tmp.20 = dereferenced.4 != 3
            if !tmp.20 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_subscripting_subscript_precedence() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            return (-arr[2] == -3);
        }
    "#;
    let expected = r#"
        global function main() { 
            arr.0[0] = 1
            arr.0[4] = 2
            arr.0[8] = 3
            tmp.0 = &arr.0
            tmp.1 = sign_extend 2
            tmp.2 = add_ptr(tmp.0, index=tmp.1, scale=4)
            tmp.3 = *tmp.2
            tmp.4 = - tmp.3
            tmp.6 = - 3
            tmp.5 = tmp.4 == tmp.6
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
