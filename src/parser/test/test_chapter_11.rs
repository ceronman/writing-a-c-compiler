use super::{assert_error, assert_parse};

#[test]
fn test_invalid_labels_extra_credit_bitshift_duplicate_cases() {
    let src = r#"
        int main(void) {
            int x = 100;
            switch (x << 2l) {
                case 34359738768l:
                    return 1;
                case 400:
                    return 0;
            }
            return 10;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <15>  [<<]
                    │   │       ├── <12> Var [x]
                    │   │       ╰── Constant Long [2]
                    │   ╰── Block
                    │       ├── Case [34359738768]
                    │       │   ╰── Return
                    │       │       ╰── Constant Int [1]
                    │       ╰── Case [400]
                    │           ╰── Return
                    │               ╰── Constant Int [0]
                    ╰── Return
                        ╰── Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_labels_extra_credit_switch_duplicate_cases() {
    let src = r#"
        int switch_statement(int i) {
            switch(i) {
                case 0: return 0;
                case 17179869184: return 0;
                default: return 1;
            }
        }
        int main(void) {
            return switch_statement(0);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [switch_statement]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <9> Var [i]
            │           ╰── Block
            │               ├── Case [0]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ├── Case [17179869184]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <32> FunctionCall [switch_statement]
                            ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_labels_extra_credit_switch_duplicate_cases_2() {
    let src = r#"
        int switch_statement(int i) {
            switch((long) i) {
                case 100l: return 0;
                case 100: return 0;
                default: return 1;
            }
        }
        int main(void) {
            return switch_statement(100);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [switch_statement]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <12> Cast
            │           │       ├── Target
            │           │       │   ╰── Long
            │           │       ╰── Expression
            │           │           ╰── <11> Var [i]
            │           ╰── Block
            │               ├── Case [100]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ├── Case [100]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <35> FunctionCall [switch_statement]
                            ╰── Constant Int [100]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_parse_bad_specifiers() {
    assert_error(
        r#"
        int main(void) {
            int long int i = 0;
          //^^^^^^^^^^^^ Invalid type specifier
            return i;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_empty_cast() {
    assert_error(
        r#"
        int main(void) {
            return () 0;
                  //^ Expected expression, but found ')'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_fun_name_long() {
    assert_error(
        r#"
        
        int long(void) {
               //^^^^ Expected identifier, but found 'void'
            return 4;
        }
        int main(void){
            return long();
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_cast() {
    assert_error(
        r#"
        int main(void) {
            return (static int) 10;
                  //^^^^^^ Expected expression, but found 'static'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_suffix() {
    assert_error(
        r#"
        int main(void) {
            return 0 l;
                   //^ Expected ';', but found 'l'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_long_constant_as_var() {
    assert_error(
        r#"
        int main(void) {
            int 10l;
              //^^^ Expected identifier, but found '10l'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_cast_parentheses() {
    assert_error(
        r#"
        int main(void) {
            return long 0;
                 //^^^^ Expected expression, but found 'long'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_var_name_long() {
    assert_error(
        r#"
        int main(void) {
            int long = 5;
                   //^ Expected identifier, but found '='
            return long;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_call_long_as_function() {
    let src = r#"
        long x(void);
        int main(void) {
            long x = 0;
            return x();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ╰── Return
                        ╰── <17> FunctionCall [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_cast_lvalue() {
    let src = r#"
        int main(void) {
            int i = 0;
            i = (long) i = 10;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <22> Assign [=]
                    │   ├── <12> Var [i]
                    │   ╰── <21> Assign [=]
                    │       ├── <18> Cast
                    │       │   ├── Target
                    │       │   │   ╰── Long
                    │       │   ╰── Expression
                    │       │       ╰── <17> Var [i]
                    │       ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_function_types() {
    let src = r#"
        int foo(int a);
        int main(void) {
            return 0;
        }
        int foo(long a);
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [foo]
                ╰── Parameters
                    ╰── Param
                        ├── Name
                        │   ╰── a
                        ╰── Type
                            ╰── Long
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_global_types() {
    let src = r#"
        int foo = 3;
        long foo;
        int main(void) {
            return foo;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ╰── Type
            │       ╰── Long
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <16> Var [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_variable_types() {
    let src = r#"
        long a;
        int main(void) {
            extern int a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ╰── Type
            │       ╰── Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_sign_extend() {
    let src = r#"
        long sign_extend(int i, long expected) {
            long extended = (long) i;
            return (extended == expected);
        }
        int main(void) {
            if (!sign_extend(10, 10l)) {
                return 1;
            }
            if (!sign_extend(-10, -10l)) {
                return 2;
            }
            long l = (long) 100;
            if (l != 100l) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [sign_extend]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── extended
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <18> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <17> Var [i]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <22> Var [extended]
            │               ╰── <25> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40> Unary [!]
                    │   │       ╰── <39> FunctionCall [sign_extend]
                    │   │           ├── Constant Int [10]
                    │   │           ╰── Constant Long [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55> Unary [!]
                    │   │       ╰── <54> FunctionCall [sign_extend]
                    │   │           ├── <50> Unary [-]
                    │   │           │   ╰── Constant Int [10]
                    │   │           ╰── <53> Unary [-]
                    │   │               ╰── Constant Long [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <67> Cast
                    │           ├── Target
                    │           │   ╰── Long
                    │           ╰── Expression
                    │               ╰── Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> Var [l]
                    │   │       ╰── Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_truncate() {
    let src = r#"
        int truncate(long l, int expected) {
            int result = (int) l;
            return (result == expected);
        }
        int main(void)
        {
            if (!truncate(10l, 10)) {
                return 1;
            }
            if (!truncate(-10l, -10)) {
                return 2;
            }
            if (!truncate(17179869189l,
                          5)) {
                return 3;
            }
            if (!truncate(-17179869179l,
                          5l)) {
                return 4;
            }
            int i = (int)17179869189l;
            if (i != 5)
                return 5;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [truncate]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <18> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <17> Var [l]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <22> Var [result]
            │               ╰── <25> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40> Unary [!]
                    │   │       ╰── <39> FunctionCall [truncate]
                    │   │           ├── Constant Long [10]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55> Unary [!]
                    │   │       ╰── <54> FunctionCall [truncate]
                    │   │           ├── <50> Unary [-]
                    │   │           │   ╰── Constant Long [10]
                    │   │           ╰── <53> Unary [-]
                    │   │               ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66> Unary [!]
                    │   │       ╰── <65> FunctionCall [truncate]
                    │   │           ├── Constant Long [17179869189]
                    │   │           ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79> Unary [!]
                    │   │       ╰── <78> FunctionCall [truncate]
                    │   │           ├── <76> Unary [-]
                    │   │           │   ╰── Constant Long [17179869179]
                    │   │           ╰── Constant Long [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <91> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── Constant Long [17179869189]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <95> Var [i]
                    │   │       ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitshift() {
    let src = r#"
        int main(void) {
            long l = 137438953472l;
            int shiftcount = 2;
            if (l >> shiftcount != 34359738368l ) {
                return 1;
            }
            if (l << shiftcount != 549755813888 ) {
                return 2;
            }
            if (l << 2 != 549755813888 ) {
                return 3;
            }
            if ((40l << 40) != 43980465111040l) {
                return 4;
            }
            long long_shiftcount = 3l;
            int i_neighbor1 = 0;
            int i = -2147483645;
            int i_neighbor2 = 0;
            if (i >> long_shiftcount != -268435456) {
                return 5;
            }
            i = -1;
            if (i >> 10l != -1) {
                return 6;
            }
            if (i_neighbor1) {
                return 7;
            }
            if (i_neighbor2) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [137438953472]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shiftcount
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <25>  [!=]
                    │   │       ├── <22>  [>>]
                    │   │       │   ├── <18> Var [l]
                    │   │       │   ╰── <21> Var [shiftcount]
                    │   │       ╰── Constant Long [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36>  [<<]
                    │   │       │   ├── <32> Var [l]
                    │   │       │   ╰── <35> Var [shiftcount]
                    │   │       ╰── Constant Long [549755813888]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49>  [<<]
                    │   │       │   ├── <46> Var [l]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Long [549755813888]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62>  [<<]
                    │   │       │   ├── Constant Long [40]
                    │   │       │   ╰── Constant Int [40]
                    │   │       ╰── Constant Long [43980465111040]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_shiftcount
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_neighbor1
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <88> Unary [-]
                    │           ╰── Constant Int [2147483645]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_neighbor2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <102>  [>>]
                    │   │       │   ├── <98> Var [i]
                    │   │       │   ╰── <101> Var [long_shiftcount]
                    │   │       ╰── <106> Unary [-]
                    │   │           ╰── Constant Int [268435456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <119> Assign [=]
                    │   ├── <114> Var [i]
                    │   ╰── <118> Unary [-]
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130>  [!=]
                    │   │       ├── <125>  [>>]
                    │   │       │   ├── <122> Var [i]
                    │   │       │   ╰── Constant Long [10]
                    │   │       ╰── <129> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137> Var [i_neighbor1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144> Var [i_neighbor2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_long_op() {
    let src = r#"
        int main(void) {
            long l1 = 71777214294589695l;
            long l2 = -4294967296;
            if ((l1 & l2) != 71777214277877760l ) {
                return 1;
            }
            if ((l1 | l2) != -4278255361 ) {
                return 2;
            }
            if ((l1 ^ l2) != -71777218556133121 ) {
                return 3;
            }
            if ((-1l & 34359738368l) != 34359738368l) {
                return 4;
            }
            if ((0l | 34359738368l) != 34359738368l) {
                return 5;
            }
            if ((34359738368l ^ 137438953472l) != 171798691840l) {
                return 6;
            }
            long l = 4611686018427387903l;
            int i = -1073741824;
            int i2 = -1;
            if ((i & l) != 4611686017353646080l) {
                return 7;
            }
            if ((l | i) != -1) {
                return 8;
            }
            if ((l ^ i) != -4611686017353646081) {
                return 9;
            }
            if ((i2 ^ 4611686018427387903l) != ~4611686018427387903l) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l1
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [71777214294589695]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l2
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <16> Unary [-]
                    │           ╰── Constant Long [4294967296]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25>  [&]
                    │   │       │   ├── <20> Var [l1]
                    │   │       │   ╰── <23> Var [l2]
                    │   │       ╰── Constant Long [71777214277877760]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <40>  [|]
                    │   │       │   ├── <35> Var [l1]
                    │   │       │   ╰── <38> Var [l2]
                    │   │       ╰── <44> Unary [-]
                    │   │           ╰── Constant Long [4278255361]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <57>  [^]
                    │   │       │   ├── <52> Var [l1]
                    │   │       │   ╰── <55> Var [l2]
                    │   │       ╰── <61> Unary [-]
                    │   │           ╰── Constant Long [71777218556133121]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74>  [&]
                    │   │       │   ├── <70> Unary [-]
                    │   │       │   │   ╰── Constant Long [1]
                    │   │       │   ╰── Constant Long [34359738368]
                    │   │       ╰── Constant Long [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87>  [|]
                    │   │       │   ├── Constant Long [0]
                    │   │       │   ╰── Constant Long [34359738368]
                    │   │       ╰── Constant Long [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <100>  [^]
                    │   │       │   ├── Constant Long [34359738368]
                    │   │       │   ╰── Constant Long [137438953472]
                    │   │       ╰── Constant Long [171798691840]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [4611686018427387903]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <120> Unary [-]
                    │           ╰── Constant Int [1073741824]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <128> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140>  [!=]
                    │   │       ├── <137>  [&]
                    │   │       │   ├── <132> Var [i]
                    │   │       │   ╰── <135> Var [l]
                    │   │       ╰── Constant Long [4611686017353646080]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157>  [!=]
                    │   │       ├── <152>  [|]
                    │   │       │   ├── <147> Var [l]
                    │   │       │   ╰── <150> Var [i]
                    │   │       ╰── <156> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <174>  [!=]
                    │   │       ├── <169>  [^]
                    │   │       │   ├── <164> Var [l]
                    │   │       │   ╰── <167> Var [i]
                    │   │       ╰── <173> Unary [-]
                    │   │           ╰── Constant Long [4611686017353646081]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190>  [!=]
                    │   │       ├── <185>  [^]
                    │   │       │   ├── <181> Var [i2]
                    │   │       │   ╰── Constant Long [4611686018427387903]
                    │   │       ╰── <189> Unary [~]
                    │   │           ╰── Constant Long [4611686018427387903]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_to_int() {
    let src = r#"
        int main(void) {
            int i = -20;
            int b = 2147483647;
            int c = -5000000;
            i += 2147483648l;
            if (i != 2147483628) {
                return 1;
            }
            if (b != 2147483647) {
                return 2;
            }
            b /= -34359738367l;
            if (b) {
                return 3;
            }
            if (i != 2147483628) {
                return 4;
            }
            if (c != -5000000) {
                return 5;
            }
            c *= 10000l;
            if (c != 1539607552) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [2147483647]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <24> Unary [-]
                    │           ╰── Constant Int [5000000]
                    ├── <31> Assign [+=]
                    │   ├── <28> Var [i]
                    │   ╰── Constant Long [2147483648]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [i]
                    │   │       ╰── Constant Int [2147483628]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Var [b]
                    │   │       ╰── Constant Int [2147483647]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <59> Assign [/=]
                    │   ├── <54> Var [b]
                    │   ╰── <58> Unary [-]
                    │       ╰── Constant Long [34359738367]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62> Var [b]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Var [i]
                    │   │       ╰── Constant Int [2147483628]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <79> Var [c]
                    │   │       ╰── <83> Unary [-]
                    │   │           ╰── Constant Int [5000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <94> Assign [*=]
                    │   ├── <91> Var [c]
                    │   ╰── Constant Long [10000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [c]
                    │   │       ╰── Constant Int [1539607552]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_to_long() {
    let src = r#"
        int main(void) {
            long l = -34359738368l;
            int i = -10;
            l -= i;
            if (l != -34359738358l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant Long [34359738368]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <18> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── <26> Assign [-=]
                    │   ├── <22> Var [l]
                    │   ╰── <25> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <34>  [!=]
                    │   │       ├── <29> Var [l]
                    │   │       ╰── <33> Unary [-]
                    │   │           ╰── Constant Long [34359738358]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitshift() {
    let src = r#"
        int main(void) {
            int x = 100;
            x <<= 22l;
            if (x != 419430400) {
                return 1;
            }
            if ((x >>= 4l) != 26214400) {
                return 2;
            }
            if (x != 26214400) {
                return 3;
            }
            long l = 12345l;
            if ((l <<= 33) != 106042742538240l) {
                return 4;
            }
            l = -l;
            if ((l >>= 10) != -103557365760l) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── <15> Assign [<<=]
                    │   ├── <12> Var [x]
                    │   ╰── Constant Long [22]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Var [x]
                    │   │       ╰── Constant Int [419430400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> Assign [>>=]
                    │   │       │   ├── <28> Var [x]
                    │   │       │   ╰── Constant Long [4]
                    │   │       ╰── Constant Int [26214400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Var [x]
                    │   │       ╰── Constant Int [26214400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [12345]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> Assign [<<=]
                    │   │       │   ├── <58> Var [l]
                    │   │       │   ╰── Constant Int [33]
                    │   │       ╰── Constant Long [106042742538240]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <78> Assign [=]
                    │   ├── <72> Var [l]
                    │   ╰── <77> Unary [-]
                    │       ╰── <76> Var [l]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <85> Assign [>>=]
                    │   │       │   ├── <81> Var [l]
                    │   │       │   ╰── Constant Int [10]
                    │   │       ╰── <89> Unary [-]
                    │   │           ╰── Constant Long [103557365760]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise() {
    let src = r#"
        int main(void) {
            long l1 = 71777214294589695l;
            long l2 = -4294967296;
            l1 &= l2;
            if (l1 != 71777214277877760l) {
                return 1;
            }
            l2 |= 100l;
            if (l2 != -4294967196) {
                return 2;
            }
            l1 ^= -9223372036854775807l;
            if (l1 != -9151594822576898047l ) {
                return 3;
            }
            l1 = 4611686018427387903l;
            int i = -1073741824;
            l1 &= i;
            if (l1 != 4611686017353646080l) {
                return 4;
            }
            i = -2147483648l;
            if ((i |= 71777214294589695l) != -2130771713) {
                return 5;
            }
            if (i != -2130771713) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l1
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [71777214294589695]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l2
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <16> Unary [-]
                    │           ╰── Constant Long [4294967296]
                    ├── <24> Assign [&=]
                    │   ├── <20> Var [l1]
                    │   ╰── <23> Var [l2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30>  [!=]
                    │   │       ├── <27> Var [l1]
                    │   │       ╰── Constant Long [71777214277877760]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <40> Assign [|=]
                    │   ├── <37> Var [l2]
                    │   ╰── Constant Long [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <43> Var [l2]
                    │   │       ╰── <47> Unary [-]
                    │   │           ╰── Constant Long [4294967196]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <60> Assign [^=]
                    │   ├── <55> Var [l1]
                    │   ╰── <59> Unary [-]
                    │       ╰── Constant Long [9223372036854775807]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <63> Var [l1]
                    │   │       ╰── <67> Unary [-]
                    │   │           ╰── Constant Long [9151594822576898047]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <78> Assign [=]
                    │   ├── <75> Var [l1]
                    │   ╰── Constant Long [4611686018427387903]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <85> Unary [-]
                    │           ╰── Constant Int [1073741824]
                    ├── <93> Assign [&=]
                    │   ├── <89> Var [l1]
                    │   ╰── <92> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> Var [l1]
                    │   │       ╰── Constant Long [4611686017353646080]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <111> Assign [=]
                    │   ├── <106> Var [i]
                    │   ╰── <110> Unary [-]
                    │       ╰── Constant Long [2147483648]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123>  [!=]
                    │   │       ├── <118> Assign [|=]
                    │   │       │   ├── <114> Var [i]
                    │   │       │   ╰── Constant Long [71777214294589695]
                    │   │       ╰── <122> Unary [-]
                    │   │           ╰── Constant Int [2130771713]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135>  [!=]
                    │   │       ├── <130> Var [i]
                    │   │       ╰── <134> Unary [-]
                    │   │           ╰── Constant Int [2130771713]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_increment_long() {
    let src = r#"
        
        int main(void) {
            long x = -9223372036854775807l;
            if (x++ != -9223372036854775807l) {
                return 1;
            }
            if (x != -9223372036854775806l) {
                return 2;
            }
            if (--x != -9223372036854775807l) {
                return 3;
            }
            if (x != -9223372036854775807l) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant Long [9223372036854775807]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <16> Postfix [++]
                    │   │       │   ╰── <14> Var [x]
                    │   │       ╰── <20> Unary [-]
                    │   │           ╰── Constant Long [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <28> Var [x]
                    │   │       ╰── <32> Unary [-]
                    │   │           ╰── Constant Long [9223372036854775806]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <42> Unary [--]
                    │   │       │   ╰── <41> Var [x]
                    │   │       ╰── <46> Unary [-]
                    │   │           ╰── Constant Long [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <54> Var [x]
                    │   │       ╰── <58> Unary [-]
                    │   │           ╰── Constant Long [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_int() {
    let src = r#"
        int switch_on_int(int i) {
            switch(i) {
                case 5:
                    return 0;
                case 8589934592l:
                    return 1;
                case 34359738367:
                    return 2;
                default:
                    return 3;
            }
        }
        int main(void) {
            if (switch_on_int(5) != 0)
                return 1;
            if (switch_on_int(0) != 1)
                return 2;
            if (switch_on_int(-1) != 2)
                return 3;
            if (switch_on_int(17179869184) != 1)
                return 4;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [switch_on_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <9> Var [i]
            │           ╰── Block
            │               ├── Case [5]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ├── Case [8589934592]
            │               │   ╰── Return
            │               │       ╰── Constant Int [1]
            │               ├── Case [34359738367]
            │               │   ╰── Return
            │               │       ╰── Constant Int [2]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> FunctionCall [switch_on_int]
                    │   │       │   ╰── Constant Int [5]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45> FunctionCall [switch_on_int]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> FunctionCall [switch_on_int]
                    │   │       │   ╰── <55> Unary [-]
                    │   │       │       ╰── Constant Int [1]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> FunctionCall [switch_on_int]
                    │   │       │   ╰── Constant Long [17179869184]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_long() {
    let src = r#"
        int switch_on_long(long l) {
            switch (l) {
                case 0: return 0;
                case 100: return 1;
                case 8589934592l:
                    return 2;
                default:
                    return -1;
            }
        }
        int main(void) {
            if (switch_on_long(8589934592) != 2)
                return 1;
            if (switch_on_long(100) != 1)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [switch_on_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <9> Var [l]
            │           ╰── Block
            │               ├── Case [0]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ├── Case [100]
            │               │   ╰── Return
            │               │       ╰── Constant Int [1]
            │               ├── Case [8589934592]
            │               │   ╰── Return
            │               │       ╰── Constant Int [2]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <24> Unary [-]
            │                           ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> FunctionCall [switch_on_long]
                    │   │       │   ╰── Constant Long [8589934592]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> FunctionCall [switch_on_long]
                    │   │       │   ╰── Constant Int [100]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_common_type() {
    let src = r#"
        long l;
        int i;
        int addition(void) {
            long result = i + l;
            return (result == 2147483663l);
        }
        int division(void) {
            int int_result = l / i;
            return (int_result == 214748364);
        }
        int comparison(void) {
            return (i <= l);
        }
        int conditional(void) {
            long result = 1 ? l : i;
            return (result == 8589934592l);
        }
        int main(void) {
            l = 2147483653;
            i = 10;
            if (!addition()) {
                return 1;
            }
            l = 2147483649l;
            if (!division()) {
                return 2;
            }
            i = -100;
            l = 2147483648;
            if (!comparison()) {
                return 3;
            }
            l = 8589934592l;
            i = 10;
            if (!conditional()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ╰── Type
            │       ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ╰── Type
            │       ╰── Int
            ├── Function [addition]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <21>  [+]
            │       │           ├── <17> Var [i]
            │       │           ╰── <20> Var [l]
            │       ╰── Return
            │           ╰── <29>  [==]
            │               ├── <25> Var [result]
            │               ╰── Constant Long [2147483663]
            ├── Function [division]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── int_result
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <45>  [/]
            │       │           ├── <41> Var [l]
            │       │           ╰── <44> Var [i]
            │       ╰── Return
            │           ╰── <53>  [==]
            │               ├── <49> Var [int_result]
            │               ╰── Constant Int [214748364]
            ├── Function [comparison]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <67>  [<=]
            │               ├── <62> Var [i]
            │               ╰── <65> Var [l]
            ├── Function [conditional]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <{node_id}> Conditional [?]
            │       │           ├── Constant Int [1]
            │       │           ├── Then
            │       │           │   ╰── <80> Var [l]
            │       │           ╰── Else
            │       │               ╰── <82> Var [i]
            │       ╰── Return
            │           ╰── <91>  [==]
            │               ├── <87> Var [result]
            │               ╰── Constant Long [8589934592]
            ╰── Function [main]
                ╰── Body
                    ├── <103> Assign [=]
                    │   ├── <100> Var [l]
                    │   ╰── Constant Long [2147483653]
                    ├── <109> Assign [=]
                    │   ├── <106> Var [i]
                    │   ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114> Unary [!]
                    │   │       ╰── <113> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <124> Assign [=]
                    │   ├── <121> Var [l]
                    │   ╰── Constant Long [2147483649]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <129> Unary [!]
                    │   │       ╰── <128> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <141> Assign [=]
                    │   ├── <136> Var [i]
                    │   ╰── <140> Unary [-]
                    │       ╰── Constant Int [100]
                    ├── <147> Assign [=]
                    │   ├── <144> Var [l]
                    │   ╰── Constant Long [2147483648]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152> Unary [!]
                    │   │       ╰── <151> FunctionCall [comparison]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <162> Assign [=]
                    │   ├── <159> Var [l]
                    │   ╰── Constant Long [8589934592]
                    ├── <168> Assign [=]
                    │   ├── <165> Var [i]
                    │   ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <173> Unary [!]
                    │   │       ╰── <172> FunctionCall [conditional]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_convert_by_assignment() {
    let src = r#"
        int return_truncated_long(long l) {
            return l;
        }
        long return_extended_int(int i) {
            return i;
        }
        int truncate_on_assignment(long l, int expected) {
            int result = l;
            return result == expected;
        }
        int main(void) {
            long result = return_truncated_long(4294967298l);
            if (result != 2l) {
                return 1;
            }
            result = return_extended_int(-10);
            if (result != -10) {
                return 2;
            }
            int i = 4294967298l;
            if (i != 2) {
                return 3;
            }
            if (!truncate_on_assignment(17179869184l, 0)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_truncated_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <9> Var [l]
            ├── Function [return_extended_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <21> Var [i]
            ├── Function [truncate_on_assignment]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <39> Var [l]
            │       ╰── Return
            │           ╰── <47>  [==]
            │               ├── <43> Var [result]
            │               ╰── <46> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <60> FunctionCall [return_truncated_long]
                    │           ╰── Constant Long [4294967298]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> Var [result]
                    │   │       ╰── Constant Long [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <81> Assign [=]
                    │   ├── <74> Var [result]
                    │   ╰── <80> FunctionCall [return_extended_int]
                    │       ╰── <79> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <84> Var [result]
                    │   │       ╰── <88> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Long [4294967298]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <102> Var [i]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116> Unary [!]
                    │   │       ╰── <115> FunctionCall [truncate_on_assignment]
                    │   │           ├── Constant Long [17179869184]
                    │   │           ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_convert_function_arguments() {
    let src = r#"
        int foo(long a, int b, int c, int d, long e, int f, long g, int h) {
            if (a != -1l)
                return 1;
            if (b != 2)
                return 2;
            if (c != 0)
                return 3;
            if (d != -5)
                return 4;
            if (e != -101l)
                return 5;
            if (f != -123)
                return 6;
            if (g != -10l)
                return 7;
            if (h != 1234)
                return 8;
            return 0;
        }
        int main(void) {
            int a = -1;
            long int b = 4294967298;
            long c = -4294967296;
            long d =
                21474836475;
            int e = -101;
            long f = -123;
            int g = -10;
            long h = -9223372036854774574;
            return foo(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── h
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <35>  [!=]
            │       │   │       ├── <30> Var [a]
            │       │   │       ╰── <34> Unary [-]
            │       │   │           ╰── Constant Long [1]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <43>  [!=]
            │       │   │       ├── <40> Var [b]
            │       │   │       ╰── Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <51>  [!=]
            │       │   │       ├── <48> Var [c]
            │       │   │       ╰── Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <61>  [!=]
            │       │   │       ├── <56> Var [d]
            │       │   │       ╰── <60> Unary [-]
            │       │   │           ╰── Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <71>  [!=]
            │       │   │       ├── <66> Var [e]
            │       │   │       ╰── <70> Unary [-]
            │       │   │           ╰── Constant Long [101]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <81>  [!=]
            │       │   │       ├── <76> Var [f]
            │       │   │       ╰── <80> Unary [-]
            │       │   │           ╰── Constant Int [123]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <91>  [!=]
            │       │   │       ├── <86> Var [g]
            │       │   │       ╰── <90> Unary [-]
            │       │   │           ╰── Constant Long [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <99>  [!=]
            │       │   │       ├── <96> Var [h]
            │       │   │       ╰── Constant Int [1234]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [8]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <116> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [4294967298]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <130> Unary [-]
                    │           ╰── Constant Long [4294967296]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [21474836475]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <144> Unary [-]
                    │           ╰── Constant Int [101]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <152> Unary [-]
                    │           ╰── Constant Int [123]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <160> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <168> Unary [-]
                    │           ╰── Constant Long [9223372036854774574]
                    ╰── Return
                        ╰── <188> FunctionCall [foo]
                            ├── <173> Var [a]
                            ├── <175> Var [b]
                            ├── <177> Var [c]
                            ├── <179> Var [d]
                            ├── <181> Var [e]
                            ├── <183> Var [f]
                            ├── <185> Var [g]
                            ╰── <187> Var [h]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_convert_static_initializer() {
    let src = r#"
        int i = 8589934592l;
        long j = 123456;
        int main(void) {
            if (i != 0) {
                return 1;
            }
            if (j != 123456l) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant Long [8589934592]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── j
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Int [123456]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Var [i]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [j]
                    │   │       ╰── Constant Long [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_long_constants() {
    let src = r#"
        int main(void) {
            if (2147483647l + 2147483647l < 0l) {
                return 1;
            }
            if (19327352832 < 100l) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <11>  [<]
                    │   │       ├── <8>  [+]
                    │   │       │   ├── Constant Long [2147483647]
                    │   │       │   ╰── Constant Long [2147483647]
                    │   │       ╰── Constant Long [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [<]
                    │   │       ├── Constant Long [19327352832]
                    │   │       ╰── Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_long_args() {
    let src = r#"
        int test_sum(int a, int b, int c, long d, int e, long f, int g, int h, long i) {
            if (d + f < 100l) {
                return 1;
            }
            if (i < 100l)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [test_sum]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Long
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Long
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── h
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── i
                │       ╰── Type
                │           ╰── Long
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [<]
                    │   │       ├── <37>  [+]
                    │   │       │   ├── <33> Var [d]
                    │   │       │   ╰── <36> Var [f]
                    │   │       ╰── Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [<]
                    │   │       ├── <47> Var [i]
                    │   │       ╰── Constant Long [100]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_long_args_client() {
    let src = r#"
        int test_sum(int a, int b, int c, long d, int e, long f, int g, int h, long i);
        int main(void) {
            return test_sum(0, 0, 0, 34359738368l, 0, 34359738368l, 0, 0, 34359738368l);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [test_sum]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── h
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── i
            │           ╰── Type
            │               ╰── Long
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <47> FunctionCall [test_sum]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Long [34359738368]
                            ├── Constant Int [0]
                            ├── Constant Long [34359738368]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ╰── Constant Long [34359738368]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_long_global_var() {
    let src = r#"
        long int l = 8589934592l;
        long return_l(void) {
            return l;
        }
        int return_l_as_int(void) {
            return l;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Long [8589934592]
            ├── Function [return_l]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Var [l]
            ╰── Function [return_l_as_int]
                ╰── Body
                    ╰── Return
                        ╰── <21> Var [l]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_long_global_var_client() {
    let src = r#"
        extern long int l;
        long return_l(void);
        int return_l_as_int(void);
        int main(void) {
            if (return_l() != 8589934592l)
                return 1;
            if (return_l_as_int() != 0)
                return 2;
            l = l - 10l;
            if (return_l() != 8589934582l)
                return 3;
            if (return_l_as_int() != -10)
                return 4;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Extern
            ├── Function [return_l]
            ├── Function [return_l_as_int]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <24>  [!=]
                    │   │       ├── <21> FunctionCall [return_l]
                    │   │       ╰── Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <29> FunctionCall [return_l_as_int]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── <44> Assign [=]
                    │   ├── <37> Var [l]
                    │   ╰── <43>  [-]
                    │       ├── <40> Var [l]
                    │       ╰── Constant Long [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> FunctionCall [return_l]
                    │   │       ╰── Constant Long [8589934582]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <55> FunctionCall [return_l_as_int]
                    │   │       ╰── <59> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_maintain_stack_alignment() {
    let src = r#"
        long add_variables(long x, long y, int z){
            return x + y + z;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [add_variables]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── x
                │   │   ╰── Type
                │   │       ╰── Long
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── y
                │   │   ╰── Type
                │   │       ╰── Long
                │   ╰── Param
                │       ├── Name
                │       │   ╰── z
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <23>  [+]
                            ├── <19>  [+]
                            │   ├── <15> Var [x]
                            │   ╰── <18> Var [y]
                            ╰── <22> Var [z]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_maintain_stack_alignment_client() {
    let src = r#"
        long add_variables(long x, long y, int z);
        int main(void) {
            long x = 3;
            long y = 4;
            int z = 5;
            return add_variables(x, y, z);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add_variables]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ╰── Type
            │       │       ╰── Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── y
            │       │   ╰── Type
            │       │       ╰── Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── z
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [5]
                    ╰── Return
                        ╰── <44> FunctionCall [add_variables]
                            ├── <39> Var [x]
                            ├── <41> Var [y]
                            ╰── <43> Var [z]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_return_long() {
    let src = r#"
        long add(int a, int b) {
            return (long) a + (long) b;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [add]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── b
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <22>  [+]
                            ├── <15> Cast
                            │   ├── Target
                            │   │   ╰── Long
                            │   ╰── Expression
                            │       ╰── <14> Var [a]
                            ╰── <21> Cast
                                ├── Target
                                │   ╰── Long
                                ╰── Expression
                                    ╰── <20> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_return_long_client() {
    let src = r#"
        long add(int a, int b);
        int main(void) {
            long a = add(2147483645, 2147483645);
            if (a != 4294967290l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── b
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <22> FunctionCall [add]
                    │           ├── Constant Int [2147483645]
                    │           ╰── Constant Int [2147483645]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Var [a]
                    │   │       ╰── Constant Long [4294967290]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_arithmetic_ops() {
    let src = r#"
        long a;
        long b;
        int addition(void) {
            return (a + b == 4294967295l);
        }
        int subtraction(void) {
            return (a - b == -4294967380l);
        }
        int multiplication(void) {
            return (a * 4l == 17179869160l);
        }
        int division(void) {
            b = a / 128l;
            return (b == 33554431l);
        }
        int remaind(void) {
            b = -a % 4294967290l;
            return (b == -5l);
        }
        int complement(void) {
            return (~a == -9223372036854775807l);
        }
        int main(void) {
            a = 4294967290l;
            b = 5l;
            if (!addition()) {
                return 1;
            }
            a = -4294967290l;
            b = 90l;
            if (!subtraction()) {
                return 2;
            }
            a = 4294967290l;
            if (!multiplication()) {
                return 3;
            }
            a = 4294967290l;
            if (!division()) {
                return 4;
            }
            a = 8589934585l;
            if (!remaind()) {
                return 5;
            }
            a = 9223372036854775806l;
            if (!complement()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ╰── Type
            │       ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── b
            │   ╰── Type
            │       ╰── Long
            ├── Function [addition]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <22>  [==]
            │               ├── <18>  [+]
            │               │   ├── <14> Var [a]
            │               │   ╰── <17> Var [b]
            │               ╰── Constant Long [4294967295]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <41>  [==]
            │               ├── <35>  [-]
            │               │   ├── <31> Var [a]
            │               │   ╰── <34> Var [b]
            │               ╰── <39> Unary [-]
            │                   ╰── Constant Long [4294967380]
            ├── Function [multiplication]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <57>  [==]
            │               ├── <53>  [*]
            │               │   ├── <50> Var [a]
            │               │   ╰── Constant Long [4]
            │               ╰── Constant Long [17179869160]
            ├── Function [division]
            │   ╰── Body
            │       ├── <73> Assign [=]
            │       │   ├── <66> Var [b]
            │       │   ╰── <72>  [/]
            │       │       ├── <69> Var [a]
            │       │       ╰── Constant Long [128]
            │       ╰── Return
            │           ╰── <80>  [==]
            │               ├── <76> Var [b]
            │               ╰── Constant Long [33554431]
            ├── Function [remaind]
            │   ╰── Body
            │       ├── <98> Assign [=]
            │       │   ├── <89> Var [b]
            │       │   ╰── <97>  [%]
            │       │       ├── <94> Unary [-]
            │       │       │   ╰── <93> Var [a]
            │       │       ╰── Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <107>  [==]
            │               ├── <101> Var [b]
            │               ╰── <105> Unary [-]
            │                   ╰── Constant Long [5]
            ├── Function [complement]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <124>  [==]
            │               ├── <118> Unary [~]
            │               │   ╰── <117> Var [a]
            │               ╰── <122> Unary [-]
            │                   ╰── Constant Long [9223372036854775807]
            ╰── Function [main]
                ╰── Body
                    ├── <136> Assign [=]
                    │   ├── <133> Var [a]
                    │   ╰── Constant Long [4294967290]
                    ├── <142> Assign [=]
                    │   ├── <139> Var [b]
                    │   ╰── Constant Long [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <147> Unary [!]
                    │   │       ╰── <146> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <159> Assign [=]
                    │   ├── <154> Var [a]
                    │   ╰── <158> Unary [-]
                    │       ╰── Constant Long [4294967290]
                    ├── <165> Assign [=]
                    │   ├── <162> Var [b]
                    │   ╰── Constant Long [90]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <170> Unary [!]
                    │   │       ╰── <169> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <180> Assign [=]
                    │   ├── <177> Var [a]
                    │   ╰── Constant Long [4294967290]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <185> Unary [!]
                    │   │       ╰── <184> FunctionCall [multiplication]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <195> Assign [=]
                    │   ├── <192> Var [a]
                    │   ╰── Constant Long [4294967290]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <200> Unary [!]
                    │   │       ╰── <199> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <210> Assign [=]
                    │   ├── <207> Var [a]
                    │   ╰── Constant Long [8589934585]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215> Unary [!]
                    │   │       ╰── <214> FunctionCall [remaind]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <225> Assign [=]
                    │   ├── <222> Var [a]
                    │   ╰── Constant Long [9223372036854775806]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <230> Unary [!]
                    │   │       ╰── <229> FunctionCall [complement]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_assign() {
    let src = r#"
        int main(void) {
            long a = 4294967290l;
            long b = 0l;
            b = a;
            return (b == 4294967290l);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [4294967290]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [0]
                    ├── <22> Assign [=]
                    │   ├── <18> Var [b]
                    │   ╰── <21> Var [a]
                    ╰── Return
                        ╰── <29>  [==]
                            ├── <25> Var [b]
                            ╰── Constant Long [4294967290]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_comparisons() {
    let src = r#"
        long l;
        long l2;
        int compare_constants(void) {
            return 8589934593l > 255l;
        }
        int compare_constants_2(void) {
            return 255l < 8589934593l;
        }
        int l_geq_2_60(void) {
            return (l >= 1152921504606846976l);
        }
        int uint_max_leq_l(void) {
            return (4294967295l <= l);
        }
        int l_eq_l2(void) {
            return (l == l2);
        }
        int main(void) {
            if (!compare_constants()) {
                return 1;
            }
            if (!compare_constants_2()) {
                return 2;
            }
            l = -9223372036854775807l;
            if (l_geq_2_60()) {
                return 3;
            }
            if (uint_max_leq_l()) {
                return 4;
            }
            l = 1152921504606846976l;
            if (!l_geq_2_60()) {
                return 5;
            }
            if (!uint_max_leq_l()) {
                return 6;
            }
            l2 = l;
            if (!l_eq_l2()) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ╰── Type
            │       ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l2
            │   ╰── Type
            │       ╰── Long
            ├── Function [compare_constants]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16>  [>]
            │               ├── Constant Long [8589934593]
            │               ╰── Constant Long [255]
            ├── Function [compare_constants_2]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <27>  [<]
            │               ├── Constant Long [255]
            │               ╰── Constant Long [8589934593]
            ├── Function [l_geq_2_60]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <40>  [>=]
            │               ├── <36> Var [l]
            │               ╰── Constant Long [1152921504606846976]
            ├── Function [uint_max_leq_l]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <53>  [<=]
            │               ├── Constant Long [4294967295]
            │               ╰── <51> Var [l]
            ├── Function [l_eq_l2]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <67>  [==]
            │               ├── <62> Var [l]
            │               ╰── <65> Var [l2]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78> Unary [!]
                    │   │       ╰── <77> FunctionCall [compare_constants]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87> Unary [!]
                    │   │       ╰── <86> FunctionCall [compare_constants_2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <99> Assign [=]
                    │   ├── <94> Var [l]
                    │   ╰── <98> Unary [-]
                    │       ╰── Constant Long [9223372036854775807]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102> FunctionCall [l_geq_2_60]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109> FunctionCall [uint_max_leq_l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <119> Assign [=]
                    │   ├── <116> Var [l]
                    │   ╰── Constant Long [1152921504606846976]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124> Unary [!]
                    │   │       ╰── <123> FunctionCall [l_geq_2_60]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133> Unary [!]
                    │   │       ╰── <132> FunctionCall [uint_max_leq_l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── <144> Assign [=]
                    │   ├── <140> Var [l2]
                    │   ╰── <143> Var [l]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149> Unary [!]
                    │   │       ╰── <148> FunctionCall [l_eq_l2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_large_constants() {
    let src = r#"
        long x = 5l;
        int add_large(void) {
            x = x + 4294967290l;
            return (x == 4294967295l);
        }
        int subtract_large(void) {
            x = x - 4294967290l;
            return (x == 5l);
        }
        int multiply_by_large(void) {
            x = x * 4294967290l;
            return (x == 21474836450l);
        }
        int main(void) {
            if (!add_large()) {
                return 1;
            }
            if (!subtract_large()) {
                return 2;
            }
            if (!multiply_by_large()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Long [5]
            ├── Function [add_large]
            │   ╰── Body
            │       ├── <19> Assign [=]
            │       │   ├── <12> Var [x]
            │       │   ╰── <18>  [+]
            │       │       ├── <15> Var [x]
            │       │       ╰── Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <26>  [==]
            │               ├── <22> Var [x]
            │               ╰── Constant Long [4294967295]
            ├── Function [subtract_large]
            │   ╰── Body
            │       ├── <42> Assign [=]
            │       │   ├── <35> Var [x]
            │       │   ╰── <41>  [-]
            │       │       ├── <38> Var [x]
            │       │       ╰── Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <49>  [==]
            │               ├── <45> Var [x]
            │               ╰── Constant Long [5]
            ├── Function [multiply_by_large]
            │   ╰── Body
            │       ├── <65> Assign [=]
            │       │   ├── <58> Var [x]
            │       │   ╰── <64>  [*]
            │       │       ├── <61> Var [x]
            │       │       ╰── Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <72>  [==]
            │               ├── <68> Var [x]
            │               ╰── Constant Long [21474836450]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83> Unary [!]
                    │   │       ╰── <82> FunctionCall [add_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92> Unary [!]
                    │   │       ╰── <91> FunctionCall [subtract_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101> Unary [!]
                    │   │       ╰── <100> FunctionCall [multiply_by_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_logical() {
    let src = r#"
        int not(long l) {
            return !l;
        }
        int if_cond(long l) {
            if (l) {
                return 1;
            }
            return 0;
        }
        int and(long l1, int l2) {
            return l1 && l2;
        }
        int or(int l1, long l2) {
            return l1 || l2;
        }
        int main(void) {
            long l = 1152921504606846976l;
            long zero = 0l;
            if (not(l)) {
                return 1;
            }
            if (!not(zero)) {
                return 2;
            }
            if(!if_cond(l)) {
                return 3;
            }
            if(if_cond(zero)) {
                return 4;
            }
            if (and(zero, 1)) {
                return 5;
            }
            if (!or(1, l)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [not]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <11> Unary [!]
            │               ╰── <10> Var [l]
            ├── Function [if_cond]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <23> Var [l]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ├── Function [and]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l1
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l2
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <48>  [&&]
            │               ├── <44> Var [l1]
            │               ╰── <47> Var [l2]
            ├── Function [or]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l1
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l2
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <67>  [||]
            │               ├── <63> Var [l1]
            │               ╰── <66> Var [l2]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [1152921504606846976]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90> FunctionCall [not]
                    │   │       ╰── <89> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101> Unary [!]
                    │   │       ╰── <100> FunctionCall [not]
                    │   │           ╰── <99> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112> Unary [!]
                    │   │       ╰── <111> FunctionCall [if_cond]
                    │   │           ╰── <110> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121> FunctionCall [if_cond]
                    │   │       ╰── <120> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131> FunctionCall [and]
                    │   │       ├── <129> Var [zero]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143> Unary [!]
                    │   │       ╰── <142> FunctionCall [or]
                    │   │           ├── Constant Int [1]
                    │   │           ╰── <141> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_long_and_int_locals() {
    let src = r#"
        int main(void) {
            long a = 8589934592l;
            int b = -1;
            long c = -8589934592l;
            int d = 10;
            if (a != 8589934592l) {
                return 1;
            }
            if (b != -1){
                return 2;
            }
            if (c != -8589934592l) {
                return 3;
            }
            if (d != 10) {
                return 4;
            }
            a = -a;
            b = b - 1;
            c = c + 8589934594l;
            d = d + 10;
            if (a != -8589934592l) {
                return 5;
            }
            if (b != -2) {
                return 6;
            }
            if (c != 2) {
                return 7;
            }
            if (d != 20) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [8589934592]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <24> Unary [-]
                    │           ╰── Constant Long [8589934592]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [a]
                    │   │       ╰── Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <44> Var [b]
                    │   │       ╰── <48> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <56> Var [c]
                    │   │       ╰── <60> Unary [-]
                    │   │           ╰── Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <68> Var [d]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <84> Assign [=]
                    │   ├── <78> Var [a]
                    │   ╰── <83> Unary [-]
                    │       ╰── <82> Var [a]
                    ├── <94> Assign [=]
                    │   ├── <87> Var [b]
                    │   ╰── <93>  [-]
                    │       ├── <90> Var [b]
                    │       ╰── Constant Int [1]
                    ├── <104> Assign [=]
                    │   ├── <97> Var [c]
                    │   ╰── <103>  [+]
                    │       ├── <100> Var [c]
                    │       ╰── Constant Long [8589934594]
                    ├── <114> Assign [=]
                    │   ├── <107> Var [d]
                    │   ╰── <113>  [+]
                    │       ├── <110> Var [d]
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <117> Var [a]
                    │   │       ╰── <121> Unary [-]
                    │   │           ╰── Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134>  [!=]
                    │   │       ├── <129> Var [b]
                    │   │       ╰── <133> Unary [-]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <141> Var [c]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154>  [!=]
                    │   │       ├── <151> Var [d]
                    │   │       ╰── Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_long_args() {
    let src = r#"
        int test_sum(long a, long b, int c, int d, int e, int f, int g, int h, long i) {
            if (a + b < 100l) {
                return 1;
            }
            if (i < 100l)
                return 2;
            return 0;
        }
        int main(void) {
            return test_sum(34359738368l, 34359738368l, 0, 0, 0, 0, 0, 0, 34359738368l);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [test_sum]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── h
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <40>  [<]
            │       │   │       ├── <37>  [+]
            │       │   │       │   ├── <33> Var [a]
            │       │   │       │   ╰── <36> Var [b]
            │       │   │       ╰── Constant Long [100]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <50>  [<]
            │       │   │       ├── <47> Var [i]
            │       │   │       ╰── Constant Long [100]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [2]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <72> FunctionCall [test_sum]
                            ├── Constant Long [34359738368]
                            ├── Constant Long [34359738368]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ╰── Constant Long [34359738368]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_multi_op() {
    let src = r#"
        int target(long a) {
            long b = a * 5l - 10l;
            if (b == 21474836440l) {
                return 1;
            }
            return 0;
        }
        int main(void) {
            return target(4294967290l);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [target]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <18>  [-]
            │       │           ├── <15>  [*]
            │       │           │   ├── <12> Var [a]
            │       │           │   ╰── Constant Long [5]
            │       │           ╰── Constant Long [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <25>  [==]
            │       │   │       ├── <22> Var [b]
            │       │   │       ╰── Constant Long [21474836440]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <41> FunctionCall [target]
                            ╰── Constant Long [4294967290]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_return_long() {
    let src = r#"
        long add(int a, int b) {
            return (long) a + (long) b;
        }
        int main(void) {
            long a = add(2147483645, 2147483645);
            if (a == 4294967290l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <22>  [+]
            │               ├── <15> Cast
            │               │   ├── Target
            │               │   │   ╰── Long
            │               │   ╰── Expression
            │               │       ╰── <14> Var [a]
            │               ╰── <21> Cast
            │                   ├── Target
            │                   │   ╰── Long
            │                   ╰── Expression
            │                       ╰── <20> Var [b]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <36> FunctionCall [add]
                    │           ├── Constant Int [2147483645]
                    │           ╰── Constant Int [2147483645]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [==]
                    │   │       ├── <40> Var [a]
                    │   │       ╰── Constant Long [4294967290]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_rewrite_large_multiply_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        long glob = 5l;
        int main(void) {
            long should_spill = glob * 4294967307l;
            int one = glob - 4;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = glob + 8;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 21474836535l) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_12_ints]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── start
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── h
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── j
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── k
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── l
            │           ╰── Type
            │               ╰── Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── glob
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Long [5]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── should_spill
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <62>  [*]
            │       │           ├── <59> Var [glob]
            │       │           ╰── Constant Long [4294967307]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <72>  [-]
            │       │           ├── <69> Var [glob]
            │       │           ╰── Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <83>  [+]
            │       │           ├── <79> Var [one]
            │       │           ╰── <82> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <93>  [+]
            │       │           ├── Constant Int [2]
            │       │           ╰── <92> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <104>  [*]
            │       │           ├── <100> Var [two]
            │       │           ╰── <103> Var [two]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── five
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <114>  [-]
            │       │           ├── Constant Int [6]
            │       │           ╰── <113> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── six
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <125>  [*]
            │       │           ├── <121> Var [two]
            │       │           ╰── <124> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <135>  [+]
            │       │           ├── <132> Var [one]
            │       │           ╰── Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eight
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <145>  [*]
            │       │           ├── <142> Var [two]
            │       │           ╰── Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nine
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <156>  [*]
            │       │           ├── <152> Var [three]
            │       │           ╰── <155> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ten
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <167>  [+]
            │       │           ├── <163> Var [four]
            │       │           ╰── <166> Var [six]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eleven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <177>  [-]
            │       │           ├── Constant Int [16]
            │       │           ╰── <176> Var [five]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twelve
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <188>  [+]
            │       │           ├── <184> Var [six]
            │       │           ╰── <187> Var [six]
            │       ├── <217> FunctionCall [check_12_ints]
            │       │   ├── <193> Var [one]
            │       │   ├── <195> Var [two]
            │       │   ├── <197> Var [three]
            │       │   ├── <199> Var [four]
            │       │   ├── <201> Var [five]
            │       │   ├── <203> Var [six]
            │       │   ├── <205> Var [seven]
            │       │   ├── <207> Var [eight]
            │       │   ├── <209> Var [nine]
            │       │   ├── <211> Var [ten]
            │       │   ├── <213> Var [eleven]
            │       │   ├── <215> Var [twelve]
            │       │   ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── thirteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <226>  [+]
            │       │           ├── <223> Var [glob]
            │       │           ╰── Constant Int [8]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fourteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <236>  [+]
            │       │           ├── <233> Var [thirteen]
            │       │           ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fifteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <246>  [-]
            │       │           ├── Constant Int [28]
            │       │           ╰── <245> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── sixteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <256>  [+]
            │       │           ├── <253> Var [fourteen]
            │       │           ╰── Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seventeen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <266>  [+]
            │       │           ├── Constant Int [4]
            │       │           ╰── <265> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eighteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <276>  [-]
            │       │           ├── Constant Int [32]
            │       │           ╰── <275> Var [fourteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nineteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <286>  [-]
            │       │           ├── Constant Int [35]
            │       │           ╰── <285> Var [sixteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <296>  [+]
            │       │           ├── <293> Var [fifteen]
            │       │           ╰── Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <309>  [-]
            │       │           ├── <306>  [*]
            │       │           │   ├── <303> Var [thirteen]
            │       │           │   ╰── Constant Int [2]
            │       │           ╰── Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <319>  [+]
            │       │           ├── <316> Var [fifteen]
            │       │           ╰── Constant Int [7]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <329>  [+]
            │       │           ├── Constant Int [6]
            │       │           ╰── <328> Var [seventeen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <339>  [+]
            │       │           ├── <336> Var [thirteen]
            │       │           ╰── Constant Int [11]
            │       ├── <368> FunctionCall [check_12_ints]
            │       │   ├── <344> Var [thirteen]
            │       │   ├── <346> Var [fourteen]
            │       │   ├── <348> Var [fifteen]
            │       │   ├── <350> Var [sixteen]
            │       │   ├── <352> Var [seventeen]
            │       │   ├── <354> Var [eighteen]
            │       │   ├── <356> Var [nineteen]
            │       │   ├── <358> Var [twenty]
            │       │   ├── <360> Var [twenty_one]
            │       │   ├── <362> Var [twenty_two]
            │       │   ├── <364> Var [twenty_three]
            │       │   ├── <366> Var [twenty_four]
            │       │   ╰── Constant Int [13]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <374>  [!=]
            │       │   │       ├── <371> Var [should_spill]
            │       │   │       ╰── Constant Long [21474836535]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <377> Unary [-]
            │       │                   ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [check_12_ints]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── h
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── j
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── k
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── l
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── start
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <443> Assign [=]
                    │   ├── <436> Var [expected]
                    │   ╰── <442>  [+]
                    │       ├── <439> Var [start]
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <450>  [!=]
                    │   │       ├── <446> Var [a]
                    │   │       ╰── <449> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <452> Var [expected]
                    ├── <465> Assign [=]
                    │   ├── <458> Var [expected]
                    │   ╰── <464>  [+]
                    │       ├── <461> Var [start]
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <472>  [!=]
                    │   │       ├── <468> Var [b]
                    │   │       ╰── <471> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <474> Var [expected]
                    ├── <487> Assign [=]
                    │   ├── <480> Var [expected]
                    │   ╰── <486>  [+]
                    │       ├── <483> Var [start]
                    │       ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <494>  [!=]
                    │   │       ├── <490> Var [c]
                    │   │       ╰── <493> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <496> Var [expected]
                    ├── <509> Assign [=]
                    │   ├── <502> Var [expected]
                    │   ╰── <508>  [+]
                    │       ├── <505> Var [start]
                    │       ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <516>  [!=]
                    │   │       ├── <512> Var [d]
                    │   │       ╰── <515> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <518> Var [expected]
                    ├── <531> Assign [=]
                    │   ├── <524> Var [expected]
                    │   ╰── <530>  [+]
                    │       ├── <527> Var [start]
                    │       ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <538>  [!=]
                    │   │       ├── <534> Var [e]
                    │   │       ╰── <537> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <540> Var [expected]
                    ├── <553> Assign [=]
                    │   ├── <546> Var [expected]
                    │   ╰── <552>  [+]
                    │       ├── <549> Var [start]
                    │       ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <560>  [!=]
                    │   │       ├── <556> Var [f]
                    │   │       ╰── <559> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <562> Var [expected]
                    ├── <575> Assign [=]
                    │   ├── <568> Var [expected]
                    │   ╰── <574>  [+]
                    │       ├── <571> Var [start]
                    │       ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <582>  [!=]
                    │   │       ├── <578> Var [g]
                    │   │       ╰── <581> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <584> Var [expected]
                    ├── <597> Assign [=]
                    │   ├── <590> Var [expected]
                    │   ╰── <596>  [+]
                    │       ├── <593> Var [start]
                    │       ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <604>  [!=]
                    │   │       ├── <600> Var [h]
                    │   │       ╰── <603> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <606> Var [expected]
                    ├── <619> Assign [=]
                    │   ├── <612> Var [expected]
                    │   ╰── <618>  [+]
                    │       ├── <615> Var [start]
                    │       ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <626>  [!=]
                    │   │       ├── <622> Var [i]
                    │   │       ╰── <625> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <628> Var [expected]
                    ├── <641> Assign [=]
                    │   ├── <634> Var [expected]
                    │   ╰── <640>  [+]
                    │       ├── <637> Var [start]
                    │       ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <648>  [!=]
                    │   │       ├── <644> Var [j]
                    │   │       ╰── <647> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <650> Var [expected]
                    ├── <663> Assign [=]
                    │   ├── <656> Var [expected]
                    │   ╰── <662>  [+]
                    │       ├── <659> Var [start]
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <670>  [!=]
                    │   │       ├── <666> Var [k]
                    │   │       ╰── <669> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <672> Var [expected]
                    ├── <685> Assign [=]
                    │   ├── <678> Var [expected]
                    │   ╰── <684>  [+]
                    │       ├── <681> Var [start]
                    │       ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <692>  [!=]
                    │   │       ├── <688> Var [l]
                    │   │       ╰── <691> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <694> Var [expected]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_simple() {
    let src = r#"
        int main(void) {
            long l = 9223372036854775807l;
            return (l - 2l == 9223372036854775805l);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [9223372036854775807]
                    ╰── Return
                        ╰── <19>  [==]
                            ├── <15>  [-]
                            │   ├── <12> Var [l]
                            │   ╰── Constant Long [2]
                            ╰── Constant Long [9223372036854775805]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_static_long() {
    let src = r#"
        
        static long foo = 4294967290l;
        int main(void)
        {
            if (foo + 5l == 4294967295l)
            {
                foo = 1152921504606846988l;
                if (foo == 1152921504606846988l)
                    return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Long
            │   ├── Initializer
            │   │   ╰── Constant Long [4294967290]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19>  [==]
                    │   │       ├── <16>  [+]
                    │   │       │   ├── <13> Var [foo]
                    │   │       │   ╰── Constant Long [5]
                    │   │       ╰── Constant Long [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── <24> Assign [=]
                    │           │   ├── <21> Var [foo]
                    │           │   ╰── Constant Long [1152921504606846988]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <30>  [==]
                    │               │       ├── <27> Var [foo]
                    │               │       ╰── Constant Long [1152921504606846988]
                    │               ╰── Then
                    │                   ╰── Return
                    │                       ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_long_expressions_type_specifiers() {
    let src = r#"
        static int long a;
        int static long a;
        long static a;
        int my_function(long a, long int b, int long c);
        int my_function(long int x, int long y, long z) {
            return x + y + z;
        }
        int main(void) {
            long x = 1l;
            long int y = 2l;
            int long z = 3l;
            extern long a;
            a = 4;
           int sum = 0;
            for (long i = 1099511627776l; i > 0; i = i / 2) {
                sum = sum + 1;
            }
            if (x != 1) {
                return 1;
            }
            if (y != 2) {
                return 2;
            }
            if (a != 4) {
                return 3;
            }
            if (my_function(x, y, z) != 6) {
                return 4;
            }
            if (sum != 41) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Static
            ├── Function [my_function]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Long
            ├── Function [my_function]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── x
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── y
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── z
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <52>  [+]
            │               ├── <48>  [+]
            │               │   ├── <44> Var [x]
            │               │   ╰── <47> Var [y]
            │               ╰── <51> Var [z]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Extern
                    ├── <87> Assign [=]
                    │   ├── <84> Var [a]
                    │   ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Long
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Long [1099511627776]
                    │   ├── Condition
                    │   │   ╰── <106>  [>]
                    │   │       ├── <103> Var [i]
                    │   │       ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <115> Assign [=]
                    │   │       ├── <108> Var [i]
                    │   │       ╰── <114>  [/]
                    │   │           ├── <111> Var [i]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Block
                    │       ╰── <124> Assign [=]
                    │           ├── <117> Var [sum]
                    │           ╰── <123>  [+]
                    │               ├── <120> Var [sum]
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <130> Var [x]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143>  [!=]
                    │   │       ├── <140> Var [y]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <150> Var [a]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <169>  [!=]
                    │   │       ├── <166> FunctionCall [my_function]
                    │   │       │   ├── <161> Var [x]
                    │   │       │   ├── <163> Var [y]
                    │   │       │   ╰── <165> Var [z]
                    │   │       ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <179>  [!=]
                    │   │       ├── <176> Var [sum]
                    │   │       ╰── Constant Int [41]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}
