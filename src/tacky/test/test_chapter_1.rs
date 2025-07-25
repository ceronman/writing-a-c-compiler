use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
        }
    "#;
    let expected = r#"
        global function main() { 
            return 100
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_newlines() {
    let src = r#"
        int
        main
        (
        void
        )
        {
        return
        0
        ;
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
fn test_valid_no_newlines() {
    let src = r#"
        int main(void){return 0;}
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
fn test_valid_return_0() {
    let src = r#"
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
fn test_valid_return_2() {
    let src = r#"
        int main(void) {
            return 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            return 2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_spaces() {
    let src = r#"
           int main ( void) { return 0 ; }
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
fn test_valid_tabs() {
    let src = r#"
        int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        global function main() { 
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
