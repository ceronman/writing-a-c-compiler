use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_bitwise() {
    let src = r#"
        int main(void) {
            return ~12;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ~ 12
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_bitwise_int_min() {
    let src = r#"
        int main(void) {
            return ~-2147483647;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 2147483647
            tmp.1 = ~ tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_bitwise_zero() {
    let src = r#"
        int main(void) {
            return ~0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ~ 0
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_neg() {
    let src = r#"
        int main(void) {
            return -5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 5
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_neg_zero() {
    let src = r#"
        int main(void) {
            return -0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 0
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_negate_int_max() {
    let src = r#"
        int main(void) {
            return -2147483647;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 2147483647
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return ~-3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 3
            tmp.1 = ~ tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_ops_2() {
    let src = r#"
        int main(void) {
            return -~0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ~ 0
            tmp.1 = - tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parens() {
    let src = r#"
        int main(void) {
            return (-2);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parens_2() {
    let src = r#"
        int main(void) {
            return ~(2);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ~ 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_parens_3() {
    let src = r#"
        int main(void) {
            return -(-4);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 4
            tmp.1 = - tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_redundant_parens() {
    let src = r#"
        int main(void)
        {
            return -((((10))));
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 10
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
