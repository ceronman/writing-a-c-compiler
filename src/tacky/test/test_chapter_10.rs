use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_distinct_local_and_extern() {
    let src = r#"
        int a = 5;
        int return_a(void) {
            return a;
        }
        int main(void) {
            int a = 3;
            {
                extern int a;
                if (a != 5)
                    return 1;
                a = 4;
            }
            return a + return_a();
        }
    "#;
    let expected = r#"
        global function return_a() { 
            return a
            return 0
        }
        global function main() { 
            a.0 = 3
            tmp.0 = a != 5
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            a = 4
            tmp.2 = return_a()
            tmp.1 = a.0 + tmp.2
            return tmp.1
            return 0
        }
        static global a: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extern_block_scope_variable() {
    let src = r#"
        int main(void) {
            int outer = 1;
            int foo = 0;
            if (outer) {
                extern int foo;
                extern int foo;
                return foo;
            }
            return 0;
        }
        int foo = 3;
    "#;
    let expected = r#"
        global function main() { 
            outer.0 = 1
            foo.1 = 0
            if !outer.0 jump end_if_0
            return foo
        
          end_if_0:
            return 0
            return 0
        }
        static global foo: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_ops_file_scope_vars() {
    let src = r#"
        int x = 1;
        int y = 0;
        int main(void) {
            y = -1;
            x = (x << 1) | 1;
            if (x != 3) {
                return 1;
            }
            y = ((y & -5) ^ 12) >> 2;
            if (y != -3) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            y = tmp.0
            tmp.1 = x << 1
            tmp.2 = tmp.1 | 1
            x = tmp.2
            tmp.3 = x != 3
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = - 5
            tmp.4 = y & tmp.5
            tmp.6 = tmp.4 ^ 12
            tmp.7 = tmp.6 >> 2
            y = tmp.7
            tmp.9 = - 3
            tmp.8 = y != tmp.9
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global x: Int = 1
        static global y: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assignment_static_var() {
    let src = r#"
        int f(void) {
            static int i = 0;
            static int j = 0;
            static int k = 1;
            static int l = 48;
            i += 1;
            j -= i;
            k *= j;
            l /= 2;
            if (i != 3) {
                return 1;
            }
            if (j != -6) {
                return 2;
            }
            if (k != -18) {
                return 3;
            }
            if (l != 6) {
                return 4;
            }
            return 0;
        }
        int main(void) {
            f();
            f();
            return f();
        }
    "#;
    let expected = r#"
        global function f() { 
            tmp.0 = i.0 + 1
            i.0 = tmp.0
            tmp.1 = j.1 - i.0
            j.1 = tmp.1
            tmp.2 = k.2 * j.1
            k.2 = tmp.2
            tmp.3 = l.3 / 2
            l.3 = tmp.3
            tmp.4 = i.0 != 3
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = - 6
            tmp.5 = j.1 != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = - 18
            tmp.7 = k.2 != tmp.8
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = l.3 != 6
            if !tmp.9 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        global function main() { 
            tmp.10 = f()
            tmp.11 = f()
            tmp.12 = f()
            return tmp.12
            return 0
        }
        static i.0: Int = 0
        static j.1: Int = 0
        static k.2: Int = 1
        static l.3: Int = 48
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_skip_static_initializer() {
    let src = r#"
        int main(void) {
            goto end;
            static int x = 10;
            end:
                return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump end_0
        
          end_0:
            return x.0
            return 0
        }
        static x.0: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_increment_global_vars() {
    let src = r#"
        
        int i = 0;
        int j = 0;
        int incr_i(void){
            if (i == 1) {
                i++;
                ++i;
            }
            return 0;
        }
        int decr_j(void) {
            if (j == -1) {
                j--;
            }
            return 0;
        }
        int main(void) {
            i++ ? 0 : incr_i();
            if (i != 3) {
                return 1;
            }
            --j? decr_j(): 0;
            if (j != -2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function incr_i() { 
            tmp.0 = i == 1
            if !tmp.0 jump end_if_0
            tmp.1 = i
            tmp.2 = inc i
            i = tmp.2
            tmp.3 = inc i
            i = tmp.3
        
          end_if_0:
            return 0
            return 0
        }
        global function decr_j() { 
            tmp.5 = - 1
            tmp.4 = j == tmp.5
            if !tmp.4 jump end_if_2
            tmp.6 = j
            tmp.7 = dec j
            j = tmp.7
        
          end_if_2:
            return 0
            return 0
        }
        global function main() { 
            tmp.8 = i
            tmp.9 = inc i
            i = tmp.9
            if !tmp.8 jump else_5
            tmp.10 = 0
            jump end_if_4
        
          else_5:
            tmp.11 = incr_i()
            tmp.10 = tmp.11
        
          end_if_4:
            tmp.12 = i != 3
            if !tmp.12 jump end_if_6
            return 1
        
          end_if_6:
            tmp.13 = dec j
            j = tmp.13
            if !tmp.13 jump else_9
            tmp.15 = decr_j()
            tmp.14 = tmp.15
            jump end_if_8
        
          else_9:
            tmp.14 = 0
        
          end_if_8:
            tmp.17 = - 2
            tmp.16 = j != tmp.17
            if !tmp.16 jump end_if_10
            return 2
        
          end_if_10:
            return 0
            return 0
        }
        static global i: Int = 0
        static global j: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_file_scope_var_same_name() {
    let src = r#"
        int x;
        int main(void) {
            int x = 10;
            goto x;
            return x;
            {
                extern int x;
            x:
                return x;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            jump x_0
            return x.0
        
          x_0:
            return x
            return 0
        }
        static global x: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_static_var_same_name() {
    let src = r#"
        int main(void) {
            static int x = 5;
            goto x;
            x = 0;
        x:
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump x_0
            x.0 = 0
        
          x_0:
            return x.0
            return 0
        }
        static x.0: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_same_label_same_fun() {
    let src = r#"
        static int f(void) {
            goto x;
            return 0;
            x:
            return 2;
        }
        int f_caller(void) {
            return f();
        }
    "#;
    let expected = r#"
        function f() { 
            jump x_0
            return 0
        
          x_0:
            return 2
            return 0
        }
        global function f_caller() { 
            tmp.0 = f()
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_libraries_same_label_same_fun_client() {
    let src = r#"
        int f(void) {
            goto x;
            return 0;
        x:
            return 1;
        }
        int f_caller(void);
        int main(void) {
            if (f() != 1) {
                return 1;
            }
            if (f_caller() !=
                2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function f() { 
            jump x_0
            return 0
        
          x_0:
            return 1
            return 0
        }
        global function main() { 
            tmp.0 = f()
            tmp.1 = tmp.0 != 1
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = f_caller()
            tmp.3 = tmp.2 != 2
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
fn test_valid_extra_credit_switch_on_extern() {
    let src = r#"
        int update_x(void);
        int main(void) {
            update_x();
            extern int x;
            switch(x) {
                case 0: return 1;
                case 1: return 2;
                case 4: return 0;
                default: return 4;
            }
        }
        int x;
        int update_x(void) {
            x = 4;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = update_x()
            tmp.1 = 0 == x
            if tmp.1 jump switch_0_case__1
            tmp.2 = 1 == x
            if tmp.2 jump switch_0_case__2
            tmp.3 = 4 == x
            if tmp.3 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 2
        
          switch_0_case__3:
            return 0
        
          switch_0_default_4:
            return 4
        
          break_switch_0:
            return 0
        }
        global function update_x() { 
            x = 4
            return 0
            return 0
        }
        static global x: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_skip_extern_decl() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch(a) {
                case 1: return 1;
                extern int x;
                case 2: return 2;
                case 10:
                if (x * 2 == 30) {
                    return 0;
                }
                default: return 5;
            }
            return 6;
        }
        int x = 15;
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            tmp.0 = 1 == a.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 2 == a.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = 10 == a.0
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 2
        
          switch_0_case__3:
            tmp.3 = x * 2
            tmp.4 = tmp.3 == 30
            if !tmp.4 jump end_if_0
            return 0
        
          end_if_0:
        
          switch_0_default_4:
            return 5
        
          break_switch_0:
            return 6
            return 0
        }
        static global x: Int = 15
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_skip_static_initializer() {
    let src = r#"
        int a = 3;
        int main(void) {
            switch (a) {
                case 1:;
                    static int x = 10;
                    x = 0;
                case 3:
                    return x;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 == a
            if tmp.0 jump switch_0_case__1
            tmp.1 = 3 == a
            if tmp.1 jump switch_0_case__2
            jump break_switch_0
        
          switch_0_case__1:
            x.0 = 0
        
          switch_0_case__2:
            return x.0
        
          break_switch_0:
            return 0
            return 0
        }
        static global a: Int = 3
        static x.0: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_linkage_function() {
    let src = r#"
        extern int sum(int a, int b);
        int sum(int i, int j) {
            return i + j;
        }
        int sum(int x, int y);
    "#;
    let expected = r#"
        global function sum(i.2, j.3) { 
            tmp.0 = i.2 + j.3
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_linkage_function_client() {
    let src = r#"
        int add_one_and_two(void) {
            extern int sum(int a, int b);
            int sum(int a, int b);
            return sum(1, 2);
        }
        extern int sum(int x, int y);
        int sum(int x, int y);
        int add_three_and_four(void) {
            int sum = 3;
            if (sum > 2) {
                extern int sum(int one, int two);
                return sum(3, 4);
            }
            return 1;
        }
        int main(void) {
            if (add_three_and_four() != 7)
                return 1;
            if (add_one_and_two() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function add_one_and_two() { 
            tmp.0 = sum(1, 2)
            return tmp.0
            return 0
        }
        global function add_three_and_four() { 
            sum.8 = 3
            tmp.1 = sum.8 > 2
            if !tmp.1 jump end_if_0
            tmp.2 = sum(3, 4)
            return tmp.2
        
          end_if_0:
            return 1
            return 0
        }
        global function main() { 
            tmp.3 = add_three_and_four()
            tmp.4 = tmp.3 != 7
            if !tmp.4 jump end_if_2
            return 1
        
          end_if_2:
            tmp.5 = add_one_and_two()
            tmp.6 = tmp.5 != 3
            if !tmp.6 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_tentative_var() {
    let src = r#"
        
        int x;
        int read_x(void) {
            return x;
        }
    "#;
    let expected = r#"
        global function read_x() { 
            return x
            return 0
        }
        static global x: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_tentative_var_client() {
    let src = r#"
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 0)
                return 1;
            x = 3;
            if (read_x() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 0
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            x = 3
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 3
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_var_scoping() {
    let src = r#"
        int read_x(void) {
            int x = 4;
            if (x == 4) {
                extern int x;
                return x;
            } else {
                return -1;
            }
        }
    "#;
    let expected = r#"
        global function read_x() { 
            x.0 = 4
            tmp.0 = x.0 == 4
            if !tmp.0 jump else_1
            return x
            jump end_if_0
        
          else_1:
            tmp.1 = - 1
            return tmp.1
        
          end_if_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_var_scoping_client() {
    let src = r#"
        int x = 10;
        int read_x(void);
        int main(void) {
            int x = 0;
            if (x == 0) {
                if (read_x() != 10)
                    return 1;
                extern int x;
                if (x != 10)
                    return 1;
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 0
            tmp.0 = x.0 == 0
            if !tmp.0 jump end_if_0
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 10
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            tmp.3 = x != 10
            if !tmp.3 jump end_if_4
            return 1
        
          end_if_4:
            return 0
        
          end_if_0:
            return 1
            return 0
        }
        static global x: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_variable() {
    let src = r#"
        int x;
        extern int x;
        int x;
        int update_x(int new_val) {
            x = new_val;
            return 0;
        }
        int read_x(void) {
            return x;
        }
        int x = 3;
    "#;
    let expected = r#"
        global function update_x(new_val.0) { 
            x = new_val.0
            return 0
            return 0
        }
        global function read_x() { 
            return x
            return 0
        }
        static global x: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_external_variable_client() {
    let src = r#"
        int update_x(int new_val);
        int read_x(void);
        extern int x;
        int main(void) {
            if (x != 3)
                return 1;
            if (read_x() != 3)
                return 1;
            x = 4;
            if (x != 4)
                return 1;
            if (read_x() != 4)
                return 1;
            update_x(5);
            if (x != 5)
                return 1;
            if (read_x() != 5)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 3
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 3
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            x = 4
            tmp.3 = x != 4
            if !tmp.3 jump end_if_4
            return 1
        
          end_if_4:
            tmp.4 = read_x()
            tmp.5 = tmp.4 != 4
            if !tmp.5 jump end_if_6
            return 1
        
          end_if_6:
            tmp.6 = update_x(5)
            tmp.7 = x != 5
            if !tmp.7 jump end_if_8
            return 1
        
          end_if_8:
            tmp.8 = read_x()
            tmp.9 = tmp.8 != 5
            if !tmp.9 jump end_if_10
            return 1
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_internal_hides_external_linkage() {
    let src = r#"
        int x = 10;
        int read_x(void){
            return x;
        }
    "#;
    let expected = r#"
        global function read_x() { 
            return x
            return 0
        }
        static global x: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_internal_hides_external_linkage_client() {
    let src = r#"
        static int x = 1;
        int read_internal_x(void);
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 1)
                return 1;
            x = 2;
            if (read_internal_x() != 2)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        extern int x;
        int read_internal_x(void) {
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            x = 2
            tmp.1 = read_internal_x()
            tmp.2 = tmp.1 != 2
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            tmp.3 = read_x()
            tmp.4 = tmp.3 != 10
            if !tmp.4 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
        global function read_internal_x() { 
            return x
            return 0
        }
        static x: Int = 1
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_internal_linkage_function() {
    let src = r#"
        
        static int my_fun(void);
        int call_static_my_fun(void) {
            return my_fun();
        }
        int call_static_my_fun_2(void) {
            int my_fun(void);
            return my_fun();
        }
        extern int my_fun(void);
        static int my_fun(void);
        int my_fun(void) {
            static int i = 0;
            i = i + 1;
            return i;
        }
    "#;
    let expected = r#"
        global function call_static_my_fun() { 
            tmp.0 = my_fun()
            return tmp.0
            return 0
        }
        global function call_static_my_fun_2() { 
            tmp.1 = my_fun()
            return tmp.1
            return 0
        }
        function my_fun() { 
            tmp.2 = i.0 + 1
            i.0 = tmp.2
            return i.0
            return 0
        }
        static i.0: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_internal_linkage_function_client() {
    let src = r#"
        extern int my_fun(void);
        int call_static_my_fun(void);
        int call_static_my_fun_2(void);
        int main(void) {
            if (call_static_my_fun() != 1)
                return 1;
            if (my_fun() != 100)
                return 1;
            if (call_static_my_fun_2() != 2)
                return 1;
            return 0;
        }
        int my_fun(void) {
            return 100;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = call_static_my_fun()
            tmp.1 = tmp.0 != 1
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = my_fun()
            tmp.3 = tmp.2 != 100
            if !tmp.3 jump end_if_2
            return 1
        
          end_if_2:
            tmp.4 = call_static_my_fun_2()
            tmp.5 = tmp.4 != 2
            if !tmp.5 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
        global function my_fun() { 
            return 100
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_internal_linkage_var() {
    let src = r#"
        static int x;
        int read_x(void) {
            return x;
        }
        int update_x(int new_val) {
            extern int x;
            x = new_val;
            return 0;
        }
        extern int x;
        static int x = 5;
        static int x;
    "#;
    let expected = r#"
        global function read_x() { 
            return x
            return 0
        }
        global function update_x(new_val.0) { 
            x = new_val.0
            return 0
            return 0
        }
        static x: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_internal_linkage_var_client() {
    let src = r#"
        static int x;
        static int x;
        int read_x(void);
        int update_x(int x);
        int main(void) {
            if (x != 0)
                return 1;
            if (read_x() != 5)
                return 1;
            extern int x;
            update_x(10);
            if (read_x() != 10)
                return 1;
            if (x != 0)
                return 1;
            x = 20;
            if (x != 20)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        static int x;
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 0
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 5
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            tmp.3 = update_x(10)
            tmp.4 = read_x()
            tmp.5 = tmp.4 != 10
            if !tmp.5 jump end_if_4
            return 1
        
          end_if_4:
            tmp.6 = x != 0
            if !tmp.6 jump end_if_6
            return 1
        
          end_if_6:
            x = 20
            tmp.7 = x != 20
            if !tmp.7 jump end_if_8
            return 1
        
          end_if_8:
            tmp.8 = read_x()
            tmp.9 = tmp.8 != 10
            if !tmp.9 jump end_if_10
            return 1
        
          end_if_10:
            return 0
            return 0
        }
        static x: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multiple_static_file_scope_vars() {
    let src = r#"
        static int foo;
        int main(void) {
            return foo;
        }
        extern int foo;
        static int foo = 4;
    "#;
    let expected = r#"
        global function main() { 
            return foo
            return 0
        }
        static foo: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multiple_static_local() {
    let src = r#"
        int foo(void) {
            static int a = 3;
            a = a * 2;
            return a;
        }
        int bar(void) {
            static int a = 4;
            a = a + 1;
            return a;
        }
        int main(void) {
            return foo() + bar() + foo() + bar();
        }
    "#;
    let expected = r#"
        global function foo() { 
            tmp.0 = a.0 * 2
            a.0 = tmp.0
            return a.0
            return 0
        }
        global function bar() { 
            tmp.1 = a.1 + 1
            a.1 = tmp.1
            return a.1
            return 0
        }
        global function main() { 
            tmp.2 = foo()
            tmp.4 = bar()
            tmp.3 = tmp.2 + tmp.4
            tmp.6 = foo()
            tmp.5 = tmp.3 + tmp.6
            tmp.8 = bar()
            tmp.7 = tmp.5 + tmp.8
            return tmp.7
            return 0
        }
        static a.0: Int = 3
        static a.1: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_push_arg_on_page_boundary() {
    let src = r#"
        extern int zed;
        int foo(int a, int b, int c, int d, int e, int f, int g) {
            return g + 1;
        }
        int main(void) {
            return foo(0, 0, 0, 0, 0, 0, zed);
        }
    "#;
    let expected = r#"
        global function foo(a.0, b.1, c.2, d.3, e.4, f.5, g.6) { 
            tmp.0 = g.6 + 1
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = foo(0, 0, 0, 0, 0, 0, zed)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_shadow_static_local_var() {
    let src = r#"
        int i;
        int update_static_or_global(int update_global, int new_val)
        {
            static int i;
            if (update_global)
            {
                extern int i;
                i = new_val;
            }
            else
                i = new_val;
            return i;
        }
        int main(void)
        {
            if (i != 0)
                return 1;
            int result = update_static_or_global(1, 10);
            if (result != 0)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(0, 9);
            if (result != 9)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(1, 11);
            if (result != 9)
                return 1;
            if (i != 11)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function update_static_or_global(update_global.0, new_val.1) { 
            if !update_global.0 jump else_1
            i = new_val.1
            jump end_if_0
        
          else_1:
            i.2 = new_val.1
        
          end_if_0:
            return i.2
            return 0
        }
        global function main() { 
            tmp.0 = i != 0
            if !tmp.0 jump end_if_2
            return 1
        
          end_if_2:
            tmp.1 = update_static_or_global(1, 10)
            result.4 = tmp.1
            tmp.2 = result.4 != 0
            if !tmp.2 jump end_if_4
            return 1
        
          end_if_4:
            tmp.3 = i != 10
            if !tmp.3 jump end_if_6
            return 1
        
          end_if_6:
            tmp.4 = update_static_or_global(0, 9)
            result.4 = tmp.4
            tmp.5 = result.4 != 9
            if !tmp.5 jump end_if_8
            return 1
        
          end_if_8:
            tmp.6 = i != 10
            if !tmp.6 jump end_if_10
            return 1
        
          end_if_10:
            tmp.7 = update_static_or_global(1, 11)
            result.4 = tmp.7
            tmp.8 = result.4 != 9
            if !tmp.8 jump end_if_12
            return 1
        
          end_if_12:
            tmp.9 = i != 11
            if !tmp.9 jump end_if_14
            return 1
        
          end_if_14:
            return 0
            return 0
        }
        static global i: Int = zero[4]
        static i.2: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_static_local_multiple_scopes() {
    let src = r#"
        int putchar (int ch);
        int print_letters(void) {
            static int i = 65;
            putchar(i);
            {
                i = i + 1;
                static int i = 97;
                putchar(i);
                i = i + 1;
            }
            putchar(10);
            return 0;
        }
        int main(void) {
            for (int i = 0; i < 26; i = i + 1)
                print_letters();
        }
    "#;
    let expected = r#"
        global function print_letters() { 
            tmp.0 = putchar(i.1)
            tmp.1 = i.1 + 1
            i.1 = tmp.1
            tmp.2 = putchar(i.2)
            tmp.3 = i.2 + 1
            i.2 = tmp.3
            tmp.4 = putchar(10)
            return 0
            return 0
        }
        global function main() { 
            i.3 = 0
        
          start_loop_0:
            tmp.5 = i.3 < 26
            if !tmp.5 jump break_loop_0
            tmp.6 = print_letters()
        
          continue_loop_0:
            tmp.7 = i.3 + 1
            i.3 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            return 0
        }
        static i.1: Int = 65
        static i.2: Int = 97
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_static_local_uninitialized() {
    let src = r#"
        int foo(void) {
            static int x;
            x = x + 1;
            return x;
        }
        int main(void) {
            int ret;
            for (int i = 0; i < 4; i = i + 1)
                ret = foo();
            return ret;
        }
    "#;
    let expected = r#"
        global function foo() { 
            tmp.0 = x.0 + 1
            x.0 = tmp.0
            return x.0
            return 0
        }
        global function main() { 
            i.2 = 0
        
          start_loop_0:
            tmp.1 = i.2 < 4
            if !tmp.1 jump break_loop_0
            tmp.2 = foo()
            ret.1 = tmp.2
        
          continue_loop_0:
            tmp.3 = i.2 + 1
            i.2 = tmp.3
            jump start_loop_0
        
          break_loop_0:
            return ret.1
            return 0
        }
        static x.0: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_static_recursive_call() {
    let src = r#"
        int putchar (int ch);
        int print_alphabet(void) {
            static int count = 0;
            putchar(count + 65);
            count = count + 1;
            if (count < 26) {
                print_alphabet();
            }
            return count;
        }
        int main(void) {
            print_alphabet();
        }
    "#;
    let expected = r#"
        global function print_alphabet() { 
            tmp.0 = count.1 + 65
            tmp.1 = putchar(tmp.0)
            tmp.2 = count.1 + 1
            count.1 = tmp.2
            tmp.3 = count.1 < 26
            if !tmp.3 jump end_if_0
            tmp.4 = print_alphabet()
        
          end_if_0:
            return count.1
            return 0
        }
        global function main() { 
            tmp.5 = print_alphabet()
            return 0
        }
        static count.1: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_static_then_extern() {
    let src = r#"
        static int foo = 3;
        int main(void) {
            return foo;
        }
        extern int foo;
    "#;
    let expected = r#"
        global function main() { 
            return foo
            return 0
        }
        static foo: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_static_variables_in_expressions() {
    let src = r#"
        int main(void) {
            static int i = 2;
            static int j = 3;
            int cmp = i < j;
            if (!cmp)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = i.0 < j.1
            cmp.2 = tmp.0
            tmp.1 = ! cmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
        static i.0: Int = 2
        static j.1: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_tentative_definition() {
    let src = r#"
        extern int foo;
        int foo;
        int foo;
        int main(void) {
            for (int i = 0; i < 5; i = i + 1)
                foo = foo + 1;
            return foo;
        }
        int foo;
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
        
          start_loop_0:
            tmp.0 = i.0 < 5
            if !tmp.0 jump break_loop_0
            tmp.1 = foo + 1
            foo = tmp.1
        
          continue_loop_0:
            tmp.2 = i.0 + 1
            i.0 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            return foo
            return 0
        }
        static global foo: Int = zero[4]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_type_before_storage_class() {
    let src = r#"
        int static foo(void) {
            return 3;
        }
        int static bar = 4;
        int main(void) {
            int extern foo(void);
            int extern bar;
            return foo() + bar;
        }
    "#;
    let expected = r#"
        function foo() { 
            return 3
            return 0
        }
        global function main() { 
            tmp.0 = foo()
            tmp.1 = tmp.0 + bar
            return tmp.1
            return 0
        }
        static bar: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
