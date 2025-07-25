use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_add() {
    let src = r#"
        int main(void) {
            return 1 + 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 + 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_associativity() {
    let src = r#"
        int main(void) {
            return 1 - 2 - 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 - 2
            tmp.1 = tmp.0 - 3
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_associativity_2() {
    let src = r#"
        int main(void) {
            return 6 / 3 / 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 6 / 3
            tmp.1 = tmp.0 / 2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_associativity_3() {
    let src = r#"
        int main(void) {
            return (3 / 2 * 4) + (5 - 4 + 3);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 3 / 2
            tmp.1 = tmp.0 * 4
            tmp.3 = 5 - 4
            tmp.4 = tmp.3 + 3
            tmp.2 = tmp.1 + tmp.4
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_associativity_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 * 4 / 2 -
                3 % (2 + 1);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 5 * 4
            tmp.1 = tmp.0 / 2
            tmp.4 = 2 + 1
            tmp.3 = 3 % tmp.4
            tmp.2 = tmp.1 - tmp.3
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_div() {
    let src = r#"
        int main(void) {
            return 4 / 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 4 / 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_div_neg() {
    let src = r#"
        int main(void) {
            return (-12) / 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 12
            tmp.1 = tmp.0 / 5
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_and() {
    let src = r#"
        int main(void) {
            return 3 & 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 3 & 5
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_or() {
    let src = r#"
        int main(void) {
            return 1 | 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 | 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_precedence() {
    let src = r#"
        int main(void) {
            return 80 >> 2 | 1 ^ 5 & 7 << 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 80 >> 2
            tmp.4 = 7 << 1
            tmp.3 = 5 & tmp.4
            tmp.2 = 1 ^ tmp.3
            tmp.1 = tmp.0 | tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shift_associativity() {
    let src = r#"
        int main(void) {
            return 33 << 4 >> 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 33 << 4
            tmp.1 = tmp.0 >> 2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shift_associativity_2() {
    let src = r#"
        int main(void) {
            return 33 >> 2 << 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 33 >> 2
            tmp.1 = tmp.0 << 1
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 40 << 4 + 12 >> 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 4 + 12
            tmp.0 = 40 << tmp.1
            tmp.2 = tmp.0 >> 1
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            return 35 << 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 35 << 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            return 1000 >> 4;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1000 >> 4
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr_negative() {
    let src = r#"
        int main(void) {
            return -5 >> 30;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 5
            tmp.1 = tmp.0 >> 30
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_variable_shift_count() {
    let src = r#"
        int main(void) {
            return (4 << (2 * 2)) + (100 >> (1 + 2));
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 2 * 2
            tmp.0 = 4 << tmp.1
            tmp.4 = 1 + 2
            tmp.3 = 100 >> tmp.4
            tmp.2 = tmp.0 + tmp.3
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_xor() {
    let src = r#"
        int main(void) {
            return 7 ^ 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 7 ^ 1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_mod() {
    let src = r#"
        int main(void) {
            return 4 % 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 4 % 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_mult() {
    let src = r#"
        int main(void) {
            return 2 * 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 2 * 3
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parens() {
    let src = r#"
        int main(void) {
            return 2 * (3 + 4);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 3 + 4
            tmp.0 = 2 * tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_precedence() {
    let src = r#"
        int main(void) {
            return 2 + 3 * 4;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 3 * 4
            tmp.0 = 2 + tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sub() {
    let src = r#"
        int main(void) {
            return 1 - 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 - 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_sub_neg() {
    let src = r#"
        int main(void) {
            return 2- -1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = - 1
            tmp.0 = 2 - tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_unop_add() {
    let src = r#"
        int main(void) {
            return ~2 + 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ~ 2
            tmp.1 = tmp.0 + 3
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_unop_parens() {
    let src = r#"
        int main(void) {
            return ~(1 + 1);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 + 1
            tmp.1 = ~ tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
