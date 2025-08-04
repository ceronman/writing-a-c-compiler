use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_end_before_expr() {
    assert_error(
        r#"
        int main(void) {
            return
    
// Expected expression, but found ''"#,
    );
}

#[test]
fn test_invalid_parse_extra_junk() {
    assert_error(
        r#"
        int main(void)
        {
            return 2;
        }
        foo
      //^^^ Expected type specifier
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_function_name() {
    assert_error(
        r#"
        
        int 3 (void) {
          //^ Expected identifier, but found '3'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_keyword_wrong_case() {
    assert_error(
        r#"
        int main(void) {
            RETURN 0;
                 //^ Expected ';', but found '0'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_type() {
    assert_error(
        r#"
        main(void) {
      //^^^^ Expected type specifier
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_misspelled_keyword() {
    assert_error(
        r#"
        int main(void) {
            returns 0;
                  //^ Expected ';', but found '0'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_no_semicolon() {
    assert_error(
        r#"
        int main (void) {
            return 0
        }
      //^ Expected ';', but found '}'
    "#,
    );
}

#[test]
fn test_invalid_parse_not_expression() {
    assert_error(
        r#"
        int main(void) {
            return int;
                 //^^^ Expected expression, but found 'int'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_space_in_keyword() {
    assert_error(
        r#"
        int main(void){
            retur n 0;
                //^ Expected ';', but found 'n'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_switched_parens() {
    assert_error(
        r#"
        int main )( {
               //^ Expected ';', but found ')'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_unclosed_brace() {
    assert_error(
        r#"
        int main(void) {
            return 0;
    
// Expected statement, but found ''"#,
    );
}

#[test]
fn test_invalid_parse_unclosed_paren() {
    assert_error(
        r#"
        int main( {
                //^ Expected type specifier
            return 0;
        }
    "#,
    );
}

#[test]
fn test_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [100]
    "#;
    assert_parse(src, expected);
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
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_newlines() {
    let src = r#"
        int main(void){return 0;}
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_return_0() {
    let src = r#"
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_return_2() {
    let src = r#"
        int main(void) {
            return 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_spaces() {
    let src = r#"
           int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_tabs() {
    let src = r#"
        int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}
