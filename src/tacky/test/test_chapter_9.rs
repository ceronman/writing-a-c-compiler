use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_arguments_in_registers_dont_clobber_edx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 / a);
        }
    "#;
    let expected = r#"
        global function x(a.0, b.1, c.2, d.3, e.4, f.5) { 
            tmp.0 = a.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = b.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = c.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = d.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = e.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = f.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            return tmp.14
            return 0
        }
        global function main() { 
            a.6 = 4
            tmp.16 = 24 / a.6
            tmp.17 = x(1, 2, 3, 4, 5, tmp.16)
            return tmp.17
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_expression_args() {
    let src = r#"
        int sub(int a, int b) {
            return a - b;
        }
        int main(void) {
            int sum = sub(1 + 2, 1);
            return sum;
        }
    "#;
    let expected = r#"
        global function sub(a.0, b.1) { 
            tmp.0 = a.0 - b.1
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = 1 + 2
            tmp.2 = sub(tmp.1, 1)
            sum.2 = tmp.2
            return sum.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_fibonacci() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int main(void) {
            int n = 6;
            return fib(n);
        }
    "#;
    let expected = r#"
        global function fib(n.0) { 
            tmp.0 = n.0 == 0
            if tmp.0 jump or_true_0
            tmp.3 = n.0 == 1
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if !tmp.2 jump else_3
            return n.0
            jump end_if_2
        
          else_3:
            tmp.4 = n.0 - 1
            tmp.5 = fib(tmp.4)
            tmp.7 = n.0 - 2
            tmp.8 = fib(tmp.7)
            tmp.6 = tmp.5 + tmp.8
            return tmp.6
        
          end_if_2:
            return 0
        }
        global function main() { 
            n.1 = 6
            tmp.9 = fib(n.1)
            return tmp.9
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_forward_decl_multi_arg() {
    let src = r#"
        int foo(int a, int b);
        int main(void) {
            return foo(2, 1);
        }
        int foo(int x, int y){
            return x - y;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo(2, 1)
            return tmp.0
            return 0
        }
        global function foo(x.2, y.3) { 
            tmp.1 = x.2 - y.3
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_hello_world() {
    let src = r#"
        int putchar(int c);
        int main(void) {
            putchar(72);
            putchar(101);
            putchar(108);
            putchar(108);
            putchar(111);
            putchar(44);
            putchar(32);
            putchar(87);
            putchar(111);
            putchar(114);
            putchar(108);
            putchar(100);
            putchar(33);
            putchar(10);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = putchar(72)
            tmp.1 = putchar(101)
            tmp.2 = putchar(108)
            tmp.3 = putchar(108)
            tmp.4 = putchar(111)
            tmp.5 = putchar(44)
            tmp.6 = putchar(32)
            tmp.7 = putchar(87)
            tmp.8 = putchar(111)
            tmp.9 = putchar(114)
            tmp.10 = putchar(108)
            tmp.11 = putchar(100)
            tmp.12 = putchar(33)
            tmp.13 = putchar(10)
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_param_shadows_local_var() {
    let src = r#"
        int main(void) {
            int a = 10;
            int f(int a);
            return f(a);
        }
        int f(int a) {
            return a * 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            tmp.0 = f(a.0)
            return tmp.0
            return 0
        }
        global function f(a.2) { 
            tmp.1 = a.2 * 2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_parameter_shadows_function() {
    let src = r#"
        int a(void) {
            return 1;
        }
        int b(int a) {
            return a;
        }
        int main(void) {
            return a() + b(2);
        }
    "#;
    let expected = r#"
        global function a() { 
            return 1
            return 0
        }
        global function b(a.0) { 
            return a.0
            return 0
        }
        global function main() { 
            tmp.0 = a()
            tmp.2 = b(2)
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_parameter_shadows_own_function() {
    let src = r#"
        int a(int a) {
            return a * 2;
        }
        int main(void) {
            return a(1);
        }
    "#;
    let expected = r#"
        global function a(a.0) { 
            tmp.0 = a.0 * 2
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = a(1)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_parameters_are_preserved() {
    let src = r#"
        int g(int w, int x, int y, int z) {
            if (w == 2 && x == 4 && y == 6 && z == 8)
                return 1;
            return 0;
        }
        int f(int a, int b, int c, int d) {
            int result = g(a * 2, b * 2, c * 2, d * 2);
            return (result == 1 && a == 1 && b == 2 && c == 3 && d == 4);
        }
        int main(void) {
            return f(1, 2, 3, 4);
        }
    "#;
    let expected = r#"
        global function g(w.0, x.1, y.2, z.3) { 
            tmp.0 = w.0 == 2
            if !tmp.0 jump and_false_0
            tmp.3 = x.1 == 4
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = y.2 == 6
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = z.3 == 8
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump end_if_6
            return 1
        
          end_if_6:
            return 0
            return 0
        }
        global function f(a.4, b.5, c.6, d.7) { 
            tmp.10 = a.4 * 2
            tmp.11 = b.5 * 2
            tmp.12 = c.6 * 2
            tmp.13 = d.7 * 2
            tmp.14 = g(tmp.10, tmp.11, tmp.12, tmp.13)
            result.8 = tmp.14
            tmp.15 = result.8 == 1
            if !tmp.15 jump and_false_8
            tmp.18 = a.4 == 1
            if !tmp.18 jump and_false_8
            tmp.17 = 1
            jump and_end_9
        
          and_false_8:
            tmp.17 = 0
        
          and_end_9:
            if !tmp.17 jump and_false_10
            tmp.21 = b.5 == 2
            if !tmp.21 jump and_false_10
            tmp.20 = 1
            jump and_end_11
        
          and_false_10:
            tmp.20 = 0
        
          and_end_11:
            if !tmp.20 jump and_false_12
            tmp.24 = c.6 == 3
            if !tmp.24 jump and_false_12
            tmp.23 = 1
            jump and_end_13
        
          and_false_12:
            tmp.23 = 0
        
          and_end_13:
            if !tmp.23 jump and_false_14
            tmp.27 = d.7 == 4
            if !tmp.27 jump and_false_14
            tmp.26 = 1
            jump and_end_15
        
          and_false_14:
            tmp.26 = 0
        
          and_end_15:
            return tmp.26
            return 0
        }
        global function main() { 
            tmp.28 = f(1, 2, 3, 4)
            return tmp.28
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_arguments_in_registers_single_arg() {
    let src = r#"
        int twice(int x){
            return 2 * x;
        }
        int main(void) {
            return twice(3);
        }
    "#;
    let expected = r#"
        global function twice(x.0) { 
            tmp.0 = 2 * x.0
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = twice(3)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_function_result() {
    let src = r#"
        int foo(void) {
            return 2;
        }
        int main(void) {
            int x = 3;
            x -= foo();
            return x;
        }
    "#;
    let expected = r#"
        global function foo() { 
            return 2
            return 0
        }
        global function main() { 
            x.0 = 3
            tmp.1 = foo()
            tmp.0 = x.0 - tmp.1
            x.0 = tmp.0
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_dont_clobber_ecx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 >> (a / 2));
        }
    "#;
    let expected = r#"
        global function x(a.0, b.1, c.2, d.3, e.4, f.5) { 
            tmp.0 = a.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = b.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = c.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = d.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = e.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = f.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            return tmp.14
            return 0
        }
        global function main() { 
            a.6 = 4
            tmp.17 = a.6 / 2
            tmp.16 = 24 >> tmp.17
            tmp.18 = x(1, 2, 3, 4, 5, tmp.16)
            return tmp.18
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_label_multiple_functions() {
    let src = r#"
        
        int foo(void) {
            goto label;
            return 0;
            label:
                return 5;
        }
        int main(void) {
            goto label;
            return 0;
            label:
                return foo();
        }
    "#;
    let expected = r#"
        global function foo() { 
            jump label_0
            return 0
        
          label_0:
            return 5
            return 0
        }
        global function main() { 
            jump label_1
            return 0
        
          label_1:
            tmp.0 = foo()
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_shared_name() {
    let src = r#"
        int foo(void) {
            goto foo;
            return 0;
            foo:
                return 1;
        }
        int main(void) {
            return foo();
        }
    "#;
    let expected = r#"
        global function foo() { 
            jump foo_0
            return 0
        
          foo_0:
            return 1
            return 0
        }
        global function main() { 
            tmp.0 = foo()
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_naming_scheme() {
    let src = r#"
        int main(void) {
            _label:
            label_:
            return 0;
        }
        int main_(void) {
            label:
            return 0;
        }
        int _main(void) {
            label: return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
        
          _label_0:
        
          label__1:
            return 0
            return 0
        }
        global function main_() { 
        
          label_2:
            return 0
            return 0
        }
        global function _main() { 
        
          label_3:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_addition() {
    let src = r#"
        int add(int x, int y) {
            return x + y;
        }
    "#;
    let expected = r#"
        global function add(x.0, y.1) { 
            tmp.0 = x.0 + y.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_addition_client() {
    let src = r#"
        int add(int x, int y);
        int main(void) {
            return add(1, 2);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = add(1, 2)
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_many_args() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h) {
            return a * b * c * d * e * f * fib(g) * fib(h);
        }
    "#;
    let expected = r#"
        global function fib(n.0) { 
            tmp.0 = n.0 == 0
            if tmp.0 jump or_true_0
            tmp.3 = n.0 == 1
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if !tmp.2 jump else_3
            return n.0
            jump end_if_2
        
          else_3:
            tmp.4 = n.0 - 1
            tmp.5 = fib(tmp.4)
            tmp.7 = n.0 - 2
            tmp.8 = fib(tmp.7)
            tmp.6 = tmp.5 + tmp.8
            return tmp.6
        
          end_if_2:
            return 0
        }
        global function multiply_many_args(a.1, b.2, c.3, d.4, e.5, f.6, g.7, h.8) { 
            tmp.9 = a.1 * b.2
            tmp.10 = tmp.9 * c.3
            tmp.11 = tmp.10 * d.4
            tmp.12 = tmp.11 * e.5
            tmp.13 = tmp.12 * f.6
            tmp.15 = fib(g.7)
            tmp.14 = tmp.13 * tmp.15
            tmp.17 = fib(h.8)
            tmp.16 = tmp.14 * tmp.17
            return tmp.16
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_many_args_client() {
    let src = r#"
        int fib(int a);
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h);
        int main(void) {
            int x = fib(4);
            int seven = 7;
            int eight = fib(6);
            int y = multiply_many_args(x, 2, 3, 4, 5, 6, seven, eight);
            if (x != 3) {
                return 1;
            }
            if (y != 589680) {
                return 2;
            }
            return x + (y % 256);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fib(4)
            x.9 = tmp.0
            seven.10 = 7
            tmp.1 = fib(6)
            eight.11 = tmp.1
            tmp.2 = multiply_many_args(x.9, 2, 3, 4, 5, 6, seven.10, eight.11)
            y.12 = tmp.2
            tmp.3 = x.9 != 3
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = y.12 != 589680
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = y.12 % 256
            tmp.5 = x.9 + tmp.6
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_no_function_calls_division() {
    let src = r#"
        int f(int a, int b, int c, int d) {
            int x = a / b;
            if (a == 10 && b == 2 && c == 100 && d == 4 && x == 5)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function f(a.0, b.1, c.2, d.3) { 
            tmp.0 = a.0 / b.1
            x.4 = tmp.0
            tmp.1 = a.0 == 10
            if !tmp.1 jump and_false_0
            tmp.4 = b.1 == 2
            if !tmp.4 jump and_false_0
            tmp.3 = 1
            jump and_end_1
        
          and_false_0:
            tmp.3 = 0
        
          and_end_1:
            if !tmp.3 jump and_false_2
            tmp.7 = c.2 == 100
            if !tmp.7 jump and_false_2
            tmp.6 = 1
            jump and_end_3
        
          and_false_2:
            tmp.6 = 0
        
          and_end_3:
            if !tmp.6 jump and_false_4
            tmp.10 = d.3 == 4
            if !tmp.10 jump and_false_4
            tmp.9 = 1
            jump and_end_5
        
          and_false_4:
            tmp.9 = 0
        
          and_end_5:
            if !tmp.9 jump and_false_6
            tmp.13 = x.4 == 5
            if !tmp.13 jump and_false_6
            tmp.12 = 1
            jump and_end_7
        
          and_false_6:
            tmp.12 = 0
        
          and_end_7:
            if !tmp.12 jump end_if_8
            return 1
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_no_function_calls_division_client() {
    let src = r#"
        int f(int a, int b, int c, int d);
        int main(void) {
            return f(10, 2, 100, 4);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = f(10, 2, 100, 4)
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_no_function_calls_local_stack_variables() {
    let src = r#"
        
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3) {
            int x = 10;
            if (reg1 == 1 && reg2 == 2 && reg3 == 3 && reg4 == 4 && reg5 == 5
                && reg6 == 6 && stack1 == -1 && stack2 == -2 && stack3 == -3
                && x == 10) {
                stack2 = 100;
                return stack2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function f(reg1.0, reg2.1, reg3.2, reg4.3, reg5.4, reg6.5, stack1.6, stack2.7, stack3.8) { 
            x.9 = 10
            tmp.0 = reg1.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = reg2.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = reg3.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = reg4.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = reg5.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = reg6.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            if !tmp.14 jump and_false_10
            tmp.19 = - 1
            tmp.18 = stack1.6 == tmp.19
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            if !tmp.17 jump and_false_12
            tmp.23 = - 2
            tmp.22 = stack2.7 == tmp.23
            if !tmp.22 jump and_false_12
            tmp.21 = 1
            jump and_end_13
        
          and_false_12:
            tmp.21 = 0
        
          and_end_13:
            if !tmp.21 jump and_false_14
            tmp.27 = - 3
            tmp.26 = stack3.8 == tmp.27
            if !tmp.26 jump and_false_14
            tmp.25 = 1
            jump and_end_15
        
          and_false_14:
            tmp.25 = 0
        
          and_end_15:
            if !tmp.25 jump and_false_16
            tmp.30 = x.9 == 10
            if !tmp.30 jump and_false_16
            tmp.29 = 1
            jump and_end_17
        
          and_false_16:
            tmp.29 = 0
        
          and_end_17:
            if !tmp.29 jump end_if_18
            stack2.7 = 100
            return stack2.7
        
          end_if_18:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_no_function_calls_local_stack_variables_client() {
    let src = r#"
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3);
        int main(void) {
            return f(1, 2, 3, 4, 5, 6, -1, -2, -3);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            tmp.1 = - 2
            tmp.2 = - 3
            tmp.3 = f(1, 2, 3, 4, 5, 6, tmp.0, tmp.1, tmp.2)
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_system_call() {
    let src = r#"
        int putchar(int c);
        int incr_and_print(int b) {
            return putchar(b + 2);
        }
    "#;
    let expected = r#"
        global function incr_and_print(b.1) { 
            tmp.0 = b.1 + 2
            tmp.1 = putchar(tmp.0)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_system_call_client() {
    let src = r#"
        int incr_and_print(int c);
        int main(void) {
            incr_and_print(70);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = incr_and_print(70)
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_forward_decl() {
    let src = r#"
        int foo(void);
        int main(void) {
            return foo();
        }
        int foo(void) {
            return 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo()
            return tmp.0
            return 0
        }
        global function foo() { 
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_function_shadows_variable() {
    let src = r#"
        int main(void) {
            int foo = 3;
            int bar = 4;
            if (foo + bar > 0) {
                int foo(void);
                bar = foo();
            }
            return foo + bar;
        }
        int foo(void) {
            return 8;
        }
    "#;
    let expected = r#"
        global function main() { 
            foo.0 = 3
            bar.1 = 4
            tmp.0 = foo.0 + bar.1
            tmp.1 = tmp.0 > 0
            if !tmp.1 jump end_if_0
            tmp.2 = foo()
            bar.1 = tmp.2
        
          end_if_0:
            tmp.3 = foo.0 + bar.1
            return tmp.3
            return 0
        }
        global function foo() { 
            return 8
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_multiple_declarations() {
    let src = r#"
        int main(void) {
            int f(void);
            int f(void);
            return f();
        }
        int f(void) {
            return 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = f()
            return tmp.0
            return 0
        }
        global function f() { 
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_no_return_value() {
    let src = r#"
        int foo(void) {
            int x = 1;
        }
        int main(void) {
            foo();
            return 3;
        }
    "#;
    let expected = r#"
        global function foo() { 
            x.0 = 1
            return 0
        }
        global function main() { 
            tmp.0 = foo()
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_precedence() {
    let src = r#"
        int three(void) {
            return 3;
        }
        int main(void) {
            return !three();
        }
    "#;
    let expected = r#"
        global function three() { 
            return 3
            return 0
        }
        global function main() { 
            tmp.0 = three()
            tmp.1 = ! tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_use_function_in_expression() {
    let src = r#"
        int bar(void) {
            return 9;
        }
        int foo(void) {
            return 2 * bar();
        }
        int main(void) {
            return foo() + bar() / 3;
        }
    "#;
    let expected = r#"
        global function bar() { 
            return 9
            return 0
        }
        global function foo() { 
            tmp.1 = bar()
            tmp.0 = 2 * tmp.1
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.2 = foo()
            tmp.4 = bar()
            tmp.5 = tmp.4 / 3
            tmp.3 = tmp.2 + tmp.5
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_no_arguments_variable_shadows_function() {
    let src = r#"
        int main(void) {
            int foo(void);
            int x = foo();
            if (x > 0) {
                int foo = 3;
                x = x + foo;
            }
            return x;
        }
        int foo(void) {
            return 4;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo()
            x.0 = tmp.0
            tmp.1 = x.0 > 0
            if !tmp.1 jump end_if_0
            foo.1 = 3
            tmp.2 = x.0 + foo.1
            x.0 = tmp.2
        
          end_if_0:
            return x.0
            return 0
        }
        global function foo() { 
            return 4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_stack_arguments_call_putchar() {
    let src = r#"
        int putchar(int c);
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            putchar(h);
            return a + g;
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 65);
        }
    "#;
    let expected = r#"
        global function foo(a.1, b.2, c.3, d.4, e.5, f.6, g.7, h.8) { 
            tmp.0 = putchar(h.8)
            tmp.1 = a.1 + g.7
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = foo(1, 2, 3, 4, 5, 6, 7, 65)
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_stack_arguments_lots_of_arguments() {
    let src = r#"
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            return (a == 1 && b == 2 && c == 3 && d == 4 && e == 5
                    && f == 6 && g == 7 && h == 8);
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 8);
        }
    "#;
    let expected = r#"
        global function foo(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7) { 
            tmp.0 = a.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = b.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = c.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = d.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = e.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = f.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            if !tmp.14 jump and_false_10
            tmp.18 = g.6 == 7
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            if !tmp.17 jump and_false_12
            tmp.21 = h.7 == 8
            if !tmp.21 jump and_false_12
            tmp.20 = 1
            jump and_end_13
        
          and_false_12:
            tmp.20 = 0
        
          and_end_13:
            return tmp.20
            return 0
        }
        global function main() { 
            tmp.22 = foo(1, 2, 3, 4, 5, 6, 7, 8)
            return tmp.22
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_stack_arguments_stack_alignment() {
    let src = r#"
        int even_arguments(int a, int b, int c, int d, int e, int f, int g, int h);
        int odd_arguments(int a, int b, int c, int d, int e, int f, int g, int h, int i);
        int main(void) {
            int x = 3;
            even_arguments(1, 2, 3, 4, 5, 6, 7, 8);
            odd_arguments(1, 2, 3, 4, 5, 6, 7, 8, 9);
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.17 = 3
            tmp.0 = even_arguments(1, 2, 3, 4, 5, 6, 7, 8)
            tmp.1 = odd_arguments(1, 2, 3, 4, 5, 6, 7, 8, 9)
            return x.17
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_stack_arguments_test_for_memory_leaks() {
    let src = r#"
        int lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j, int k, int l, int m, int n, int o) {
            return l + o;
        }
        int main(void) {
            int ret = 0;
            for (int i = 0; i < 10000000; i = i + 1) {
                ret = lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ret, 13, 14, 15);
            }
            return ret == 150000000;
        }
    "#;
    let expected = r#"
        global function lots_of_args(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7, i.8, j.9, k.10, l.11, m.12, n.13, o.14) { 
            tmp.0 = l.11 + o.14
            return tmp.0
            return 0
        }
        global function main() { 
            ret.15 = 0
            i.16 = 0
        
          start_loop_0:
            tmp.1 = i.16 < 10000000
            if !tmp.1 jump break_loop_0
            tmp.2 = lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ret.15, 13, 14, 15)
            ret.15 = tmp.2
        
          continue_loop_0:
            tmp.3 = i.16 + 1
            i.16 = tmp.3
            jump start_loop_0
        
          break_loop_0:
            tmp.4 = ret.15 == 150000000
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
