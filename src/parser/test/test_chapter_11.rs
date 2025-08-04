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
                    │       ╰── <9> Constant Int [100]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <16>  [<<]
                    │   │       ├── <13> Var [x]
                    │   │       ╰── <15> Constant Long [2]
                    │   ╰── Block
                    │       ├── Case [34359738768]
                    │       │   ╰── Return
                    │       │       ╰── <18> Constant Int [1]
                    │       ╰── Case [400]
                    │           ╰── Return
                    │               ╰── <22> Constant Int [0]
                    ╰── Return
                        ╰── <28> Constant Int [10]
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
            │           │   ╰── <10> Var [i]
            │           ╰── Block
            │               ├── Case [0]
            │               │   ╰── Return
            │               │       ╰── <12> Constant Int [0]
            │               ├── Case [17179869184]
            │               │   ╰── Return
            │               │       ╰── <16> Constant Int [0]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <19> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <34> FunctionCall [switch_statement]
                            ╰── <33> Constant Int [0]
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
            │           │   ╰── <13> Cast
            │           │       ├── Target
            │           │       │   ╰── Long
            │           │       ╰── Expression
            │           │           ╰── <12> Var [i]
            │           ╰── Block
            │               ├── Case [100]
            │               │   ╰── Return
            │               │       ╰── <15> Constant Int [0]
            │               ├── Case [100]
            │               │   ╰── Return
            │               │       ╰── <19> Constant Int [0]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <22> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <37> FunctionCall [switch_statement]
                            ╰── <36> Constant Int [100]
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
                    │       ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <19> FunctionCall [x]
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
                    │       ╰── <9> Constant Int [0]
                    ├── <23> Assign [=]
                    │   ├── <13> Var [i]
                    │   ╰── <22> Assign [=]
                    │       ├── <19> Cast
                    │       │   ├── Target
                    │       │   │   ╰── Long
                    │       │   ╰── Expression
                    │       │       ╰── <18> Var [i]
                    │       ╰── <21> Constant Int [10]
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
            │           ╰── <15> Constant Int [0]
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
            │       ╰── <4> Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ╰── Type
            │       ╰── Long
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <17> Var [foo]
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
                        ╰── <15> Constant Int [0]
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
            │       │       ╰── <19> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <18> Var [i]
            │       ╰── Return
            │           ╰── <28>  [==]
            │               ├── <23> Var [extended]
            │               ╰── <26> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42> Unary [!]
                    │   │       ╰── <41> FunctionCall [sign_extend]
                    │   │           ├── <39> Constant Int [10]
                    │   │           ╰── <40> Constant Long [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <43> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57> Unary [!]
                    │   │       ╰── <56> FunctionCall [sign_extend]
                    │   │           ├── <52> Unary [-]
                    │   │           │   ╰── <51> Constant Int [10]
                    │   │           ╰── <55> Unary [-]
                    │   │               ╰── <54> Constant Long [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <69> Cast
                    │           ├── Target
                    │           │   ╰── Long
                    │           ╰── Expression
                    │               ╰── <68> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> Var [l]
                    │   │       ╰── <75> Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [3]
                    ╰── Return
                        ╰── <82> Constant Int [0]
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
            │       │       ╰── <19> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <18> Var [l]
            │       ╰── Return
            │           ╰── <28>  [==]
            │               ├── <23> Var [result]
            │               ╰── <26> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42> Unary [!]
                    │   │       ╰── <41> FunctionCall [truncate]
                    │   │           ├── <39> Constant Long [10]
                    │   │           ╰── <40> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <43> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57> Unary [!]
                    │   │       ╰── <56> FunctionCall [truncate]
                    │   │           ├── <52> Unary [-]
                    │   │           │   ╰── <51> Constant Long [10]
                    │   │           ╰── <55> Unary [-]
                    │   │               ╰── <54> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68> Unary [!]
                    │   │       ╰── <67> FunctionCall [truncate]
                    │   │           ├── <65> Constant Long [17179869189]
                    │   │           ╰── <66> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81> Unary [!]
                    │   │       ╰── <80> FunctionCall [truncate]
                    │   │           ├── <78> Unary [-]
                    │   │           │   ╰── <77> Constant Long [17179869179]
                    │   │           ╰── <79> Constant Long [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <93> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <92> Constant Long [17179869189]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [i]
                    │   │       ╰── <99> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <101> Constant Int [5]
                    ╰── Return
                        ╰── <104> Constant Int [0]
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
                    │       ╰── <9> Constant Long [137438953472]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shiftcount
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <26>  [!=]
                    │   │       ├── <23>  [>>]
                    │   │       │   ├── <19> Var [l]
                    │   │       │   ╰── <22> Var [shiftcount]
                    │   │       ╰── <25> Constant Long [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <27> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37>  [<<]
                    │   │       │   ├── <33> Var [l]
                    │   │       │   ╰── <36> Var [shiftcount]
                    │   │       ╰── <39> Constant Long [549755813888]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50>  [<<]
                    │   │       │   ├── <47> Var [l]
                    │   │       │   ╰── <49> Constant Int [2]
                    │   │       ╰── <52> Constant Long [549755813888]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63>  [<<]
                    │   │       │   ├── <59> Constant Long [40]
                    │   │       │   ╰── <61> Constant Int [40]
                    │   │       ╰── <65> Constant Long [43980465111040]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_shiftcount
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <75> Constant Long [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_neighbor1
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <81> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <89> Unary [-]
                    │           ╰── <88> Constant Int [2147483645]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_neighbor2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <95> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108>  [!=]
                    │   │       ├── <103>  [>>]
                    │   │       │   ├── <99> Var [i]
                    │   │       │   ╰── <102> Var [long_shiftcount]
                    │   │       ╰── <107> Unary [-]
                    │   │           ╰── <106> Constant Int [268435456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <109> Constant Int [5]
                    ├── <120> Assign [=]
                    │   ├── <115> Var [i]
                    │   ╰── <119> Unary [-]
                    │       ╰── <118> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131>  [!=]
                    │   │       ├── <126>  [>>]
                    │   │       │   ├── <123> Var [i]
                    │   │       │   ╰── <125> Constant Long [10]
                    │   │       ╰── <130> Unary [-]
                    │   │           ╰── <129> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <132> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <138> Var [i_neighbor1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <139> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145> Var [i_neighbor2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <146> Constant Int [8]
                    ╰── Return
                        ╰── <151> Constant Int [0]
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
                    │       ╰── <9> Constant Long [71777214294589695]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l2
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <17> Unary [-]
                    │           ╰── <16> Constant Long [4294967296]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26>  [&]
                    │   │       │   ├── <21> Var [l1]
                    │   │       │   ╰── <24> Var [l2]
                    │   │       ╰── <28> Constant Long [71777214277877760]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <41>  [|]
                    │   │       │   ├── <36> Var [l1]
                    │   │       │   ╰── <39> Var [l2]
                    │   │       ╰── <45> Unary [-]
                    │   │           ╰── <44> Constant Long [4278255361]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <47> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <58>  [^]
                    │   │       │   ├── <53> Var [l1]
                    │   │       │   ╰── <56> Var [l2]
                    │   │       ╰── <62> Unary [-]
                    │   │           ╰── <61> Constant Long [71777218556133121]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <75>  [&]
                    │   │       │   ├── <71> Unary [-]
                    │   │       │   │   ╰── <70> Constant Long [1]
                    │   │       │   ╰── <73> Constant Long [34359738368]
                    │   │       ╰── <77> Constant Long [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88>  [|]
                    │   │       │   ├── <84> Constant Long [0]
                    │   │       │   ╰── <86> Constant Long [34359738368]
                    │   │       ╰── <90> Constant Long [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <101>  [^]
                    │   │       │   ├── <97> Constant Long [34359738368]
                    │   │       │   ╰── <99> Constant Long [137438953472]
                    │   │       ╰── <103> Constant Long [171798691840]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <113> Constant Long [4611686018427387903]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <121> Unary [-]
                    │           ╰── <120> Constant Int [1073741824]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <129> Unary [-]
                    │           ╰── <128> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141>  [!=]
                    │   │       ├── <138>  [&]
                    │   │       │   ├── <133> Var [i]
                    │   │       │   ╰── <136> Var [l]
                    │   │       ╰── <140> Constant Long [4611686017353646080]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <142> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <158>  [!=]
                    │   │       ├── <153>  [|]
                    │   │       │   ├── <148> Var [l]
                    │   │       │   ╰── <151> Var [i]
                    │   │       ╰── <157> Unary [-]
                    │   │           ╰── <156> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <159> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <175>  [!=]
                    │   │       ├── <170>  [^]
                    │   │       │   ├── <165> Var [l]
                    │   │       │   ╰── <168> Var [i]
                    │   │       ╰── <174> Unary [-]
                    │   │           ╰── <173> Constant Long [4611686017353646081]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <191>  [!=]
                    │   │       ├── <186>  [^]
                    │   │       │   ├── <182> Var [i2]
                    │   │       │   ╰── <184> Constant Long [4611686018427387903]
                    │   │       ╰── <190> Unary [~]
                    │   │           ╰── <189> Constant Long [4611686018427387903]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <192> Constant Int [10]
                    ╰── Return
                        ╰── <197> Constant Int [0]
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Constant Int [2147483647]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <25> Unary [-]
                    │           ╰── <24> Constant Int [5000000]
                    ├── <32> Assign [+=]
                    │   ├── <29> Var [i]
                    │   ╰── <31> Constant Long [2147483648]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> Var [i]
                    │   │       ╰── <37> Constant Int [2147483628]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <39> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45> Var [b]
                    │   │       ╰── <47> Constant Int [2147483647]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <49> Constant Int [2]
                    ├── <60> Assign [/=]
                    │   ├── <55> Var [b]
                    │   ╰── <59> Unary [-]
                    │       ╰── <58> Constant Long [34359738367]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63> Var [b]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> Var [i]
                    │   │       ╰── <72> Constant Int [2147483628]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <74> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <80> Var [c]
                    │   │       ╰── <84> Unary [-]
                    │   │           ╰── <83> Constant Int [5000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [5]
                    ├── <95> Assign [*=]
                    │   ├── <92> Var [c]
                    │   ╰── <94> Constant Long [10000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> Var [c]
                    │   │       ╰── <100> Constant Int [1539607552]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [6]
                    ╰── Return
                        ╰── <107> Constant Int [0]
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Long [34359738368]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Unary [-]
                    │           ╰── <18> Constant Int [10]
                    ├── <27> Assign [-=]
                    │   ├── <23> Var [l]
                    │   ╰── <26> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <30> Var [l]
                    │   │       ╰── <34> Unary [-]
                    │   │           ╰── <33> Constant Long [34359738358]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <36> Constant Int [1]
                    ╰── Return
                        ╰── <41> Constant Int [0]
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
                    │       ╰── <9> Constant Int [100]
                    ├── <16> Assign [<<=]
                    │   ├── <13> Var [x]
                    │   ╰── <15> Constant Long [22]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> Var [x]
                    │   │       ╰── <21> Constant Int [419430400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <33> Assign [>>=]
                    │   │       │   ├── <29> Var [x]
                    │   │       │   ╰── <31> Constant Long [4]
                    │   │       ╰── <35> Constant Int [26214400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <37> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <43> Var [x]
                    │   │       ╰── <45> Constant Int [26214400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <47> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <55> Constant Long [12345]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Assign [<<=]
                    │   │       │   ├── <59> Var [l]
                    │   │       │   ╰── <61> Constant Int [33]
                    │   │       ╰── <65> Constant Long [106042742538240]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [4]
                    ├── <79> Assign [=]
                    │   ├── <73> Var [l]
                    │   ╰── <78> Unary [-]
                    │       ╰── <77> Var [l]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <86> Assign [>>=]
                    │   │       │   ├── <82> Var [l]
                    │   │       │   ╰── <84> Constant Int [10]
                    │   │       ╰── <90> Unary [-]
                    │   │           ╰── <89> Constant Long [103557365760]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [5]
                    ╰── Return
                        ╰── <97> Constant Int [0]
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
                    │       ╰── <9> Constant Long [71777214294589695]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l2
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <17> Unary [-]
                    │           ╰── <16> Constant Long [4294967296]
                    ├── <25> Assign [&=]
                    │   ├── <21> Var [l1]
                    │   ╰── <24> Var [l2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [l1]
                    │   │       ╰── <30> Constant Long [71777214277877760]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ├── <41> Assign [|=]
                    │   ├── <38> Var [l2]
                    │   ╰── <40> Constant Long [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <44> Var [l2]
                    │   │       ╰── <48> Unary [-]
                    │   │           ╰── <47> Constant Long [4294967196]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <50> Constant Int [2]
                    ├── <61> Assign [^=]
                    │   ├── <56> Var [l1]
                    │   ╰── <60> Unary [-]
                    │       ╰── <59> Constant Long [9223372036854775807]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <64> Var [l1]
                    │   │       ╰── <68> Unary [-]
                    │   │           ╰── <67> Constant Long [9151594822576898047]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <70> Constant Int [3]
                    ├── <79> Assign [=]
                    │   ├── <76> Var [l1]
                    │   ╰── <78> Constant Long [4611686018427387903]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <86> Unary [-]
                    │           ╰── <85> Constant Int [1073741824]
                    ├── <94> Assign [&=]
                    │   ├── <90> Var [l1]
                    │   ╰── <93> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [l1]
                    │   │       ╰── <99> Constant Long [4611686017353646080]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [4]
                    ├── <112> Assign [=]
                    │   ├── <107> Var [i]
                    │   ╰── <111> Unary [-]
                    │       ╰── <110> Constant Long [2147483648]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124>  [!=]
                    │   │       ├── <119> Assign [|=]
                    │   │       │   ├── <115> Var [i]
                    │   │       │   ╰── <117> Constant Long [71777214294589695]
                    │   │       ╰── <123> Unary [-]
                    │   │           ╰── <122> Constant Int [2130771713]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <125> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <131> Var [i]
                    │   │       ╰── <135> Unary [-]
                    │   │           ╰── <134> Constant Int [2130771713]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <137> Constant Int [6]
                    ╰── Return
                        ╰── <142> Constant Int [0]
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Long [9223372036854775807]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <17> Postfix [++]
                    │   │       │   ╰── <15> Var [x]
                    │   │       ╰── <21> Unary [-]
                    │   │           ╰── <20> Constant Long [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <34>  [!=]
                    │   │       ├── <29> Var [x]
                    │   │       ╰── <33> Unary [-]
                    │   │           ╰── <32> Constant Long [9223372036854775806]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <35> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <43> Unary [--]
                    │   │       │   ╰── <42> Var [x]
                    │   │       ╰── <47> Unary [-]
                    │   │           ╰── <46> Constant Long [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <49> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <55> Var [x]
                    │   │       ╰── <59> Unary [-]
                    │   │           ╰── <58> Constant Long [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <61> Constant Int [4]
                    ╰── Return
                        ╰── <66> Constant Int [0]
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
            │           │   ╰── <10> Var [i]
            │           ╰── Block
            │               ├── Case [5]
            │               │   ╰── Return
            │               │       ╰── <12> Constant Int [0]
            │               ├── Case [8589934592]
            │               │   ╰── Return
            │               │       ╰── <16> Constant Int [1]
            │               ├── Case [34359738367]
            │               │   ╰── Return
            │               │       ╰── <20> Constant Int [2]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <23> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> FunctionCall [switch_on_int]
                    │   │       │   ╰── <37> Constant Int [5]
                    │   │       ╰── <40> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <42> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> FunctionCall [switch_on_int]
                    │   │       │   ╰── <46> Constant Int [0]
                    │   │       ╰── <49> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <51> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <58> FunctionCall [switch_on_int]
                    │   │       │   ╰── <57> Unary [-]
                    │   │       │       ╰── <56> Constant Int [1]
                    │   │       ╰── <60> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <62> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> FunctionCall [switch_on_int]
                    │   │       │   ╰── <66> Constant Long [17179869184]
                    │   │       ╰── <69> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <71> Constant Int [4]
                    ╰── Return
                        ╰── <74> Constant Int [0]
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
            │           │   ╰── <10> Var [l]
            │           ╰── Block
            │               ├── Case [0]
            │               │   ╰── Return
            │               │       ╰── <12> Constant Int [0]
            │               ├── Case [100]
            │               │   ╰── Return
            │               │       ╰── <16> Constant Int [1]
            │               ├── Case [8589934592]
            │               │   ╰── Return
            │               │       ╰── <20> Constant Int [2]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <25> Unary [-]
            │                           ╰── <24> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> FunctionCall [switch_on_long]
                    │   │       │   ╰── <39> Constant Long [8589934592]
                    │   │       ╰── <42> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <44> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> FunctionCall [switch_on_long]
                    │   │       │   ╰── <48> Constant Int [100]
                    │   │       ╰── <51> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <53> Constant Int [2]
                    ╰── Return
                        ╰── <56> Constant Int [0]
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
            │       │       ╰── <22>  [+]
            │       │           ├── <18> Var [i]
            │       │           ╰── <21> Var [l]
            │       ╰── Return
            │           ╰── <30>  [==]
            │               ├── <26> Var [result]
            │               ╰── <28> Constant Long [2147483663]
            ├── Function [division]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── int_result
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <47>  [/]
            │       │           ├── <43> Var [l]
            │       │           ╰── <46> Var [i]
            │       ╰── Return
            │           ╰── <55>  [==]
            │               ├── <51> Var [int_result]
            │               ╰── <53> Constant Int [214748364]
            ├── Function [comparison]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <70>  [<=]
            │               ├── <65> Var [i]
            │               ╰── <68> Var [l]
            ├── Function [conditional]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <87> Conditional [?]
            │       │           ├── <82> Constant Int [1]
            │       │           ├── Then
            │       │           │   ╰── <84> Var [l]
            │       │           ╰── Else
            │       │               ╰── <86> Var [i]
            │       ╰── Return
            │           ╰── <95>  [==]
            │               ├── <91> Var [result]
            │               ╰── <93> Constant Long [8589934592]
            ╰── Function [main]
                ╰── Body
                    ├── <108> Assign [=]
                    │   ├── <105> Var [l]
                    │   ╰── <107> Constant Long [2147483653]
                    ├── <114> Assign [=]
                    │   ├── <111> Var [i]
                    │   ╰── <113> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119> Unary [!]
                    │   │       ╰── <118> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <120> Constant Int [1]
                    ├── <129> Assign [=]
                    │   ├── <126> Var [l]
                    │   ╰── <128> Constant Long [2147483649]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134> Unary [!]
                    │   │       ╰── <133> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <135> Constant Int [2]
                    ├── <146> Assign [=]
                    │   ├── <141> Var [i]
                    │   ╰── <145> Unary [-]
                    │       ╰── <144> Constant Int [100]
                    ├── <152> Assign [=]
                    │   ├── <149> Var [l]
                    │   ╰── <151> Constant Long [2147483648]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157> Unary [!]
                    │   │       ╰── <156> FunctionCall [comparison]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <158> Constant Int [3]
                    ├── <167> Assign [=]
                    │   ├── <164> Var [l]
                    │   ╰── <166> Constant Long [8589934592]
                    ├── <173> Assign [=]
                    │   ├── <170> Var [i]
                    │   ╰── <172> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <178> Unary [!]
                    │   │       ╰── <177> FunctionCall [conditional]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <179> Constant Int [4]
                    ╰── Return
                        ╰── <184> Constant Int [0]
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
            │           ╰── <10> Var [l]
            ├── Function [return_extended_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <23> Var [i]
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
            │       │       ╰── <42> Var [l]
            │       ╰── Return
            │           ╰── <50>  [==]
            │               ├── <46> Var [result]
            │               ╰── <49> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <64> FunctionCall [return_truncated_long]
                    │           ╰── <63> Constant Long [4294967298]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <68> Var [result]
                    │   │       ╰── <70> Constant Long [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <72> Constant Int [1]
                    ├── <85> Assign [=]
                    │   ├── <78> Var [result]
                    │   ╰── <84> FunctionCall [return_extended_int]
                    │       ╰── <83> Unary [-]
                    │           ╰── <82> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <88> Var [result]
                    │   │       ╰── <92> Unary [-]
                    │   │           ╰── <91> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <102> Constant Long [4294967298]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> Var [i]
                    │   │       ╰── <108> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120> Unary [!]
                    │   │       ╰── <119> FunctionCall [truncate_on_assignment]
                    │   │           ├── <117> Constant Long [17179869184]
                    │   │           ╰── <118> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [4]
                    ╰── Return
                        ╰── <126> Constant Int [0]
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
            │       │   │   ╰── <36>  [!=]
            │       │   │       ├── <31> Var [a]
            │       │   │       ╰── <35> Unary [-]
            │       │   │           ╰── <34> Constant Long [1]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <37> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <44>  [!=]
            │       │   │       ├── <41> Var [b]
            │       │   │       ╰── <43> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <45> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <52>  [!=]
            │       │   │       ├── <49> Var [c]
            │       │   │       ╰── <51> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <53> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <62>  [!=]
            │       │   │       ├── <57> Var [d]
            │       │   │       ╰── <61> Unary [-]
            │       │   │           ╰── <60> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <63> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <72>  [!=]
            │       │   │       ├── <67> Var [e]
            │       │   │       ╰── <71> Unary [-]
            │       │   │           ╰── <70> Constant Long [101]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <73> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <82>  [!=]
            │       │   │       ├── <77> Var [f]
            │       │   │       ╰── <81> Unary [-]
            │       │   │           ╰── <80> Constant Int [123]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <83> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <92>  [!=]
            │       │   │       ├── <87> Var [g]
            │       │   │       ╰── <91> Unary [-]
            │       │   │           ╰── <90> Constant Long [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <93> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <100>  [!=]
            │       │   │       ├── <97> Var [h]
            │       │   │       ╰── <99> Constant Int [1234]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <101> Constant Int [8]
            │       ╰── Return
            │           ╰── <104> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <118> Unary [-]
                    │           ╰── <117> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <124> Constant Long [4294967298]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <132> Unary [-]
                    │           ╰── <131> Constant Long [4294967296]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <138> Constant Long [21474836475]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <146> Unary [-]
                    │           ╰── <145> Constant Int [101]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <154> Unary [-]
                    │           ╰── <153> Constant Int [123]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <162> Unary [-]
                    │           ╰── <161> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <170> Unary [-]
                    │           ╰── <169> Constant Long [9223372036854774574]
                    ╰── Return
                        ╰── <190> FunctionCall [foo]
                            ├── <175> Var [a]
                            ├── <177> Var [b]
                            ├── <179> Var [c]
                            ├── <181> Var [d]
                            ├── <183> Var [e]
                            ├── <185> Var [f]
                            ├── <187> Var [g]
                            ╰── <189> Var [h]
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
            │       ╰── <4> Constant Long [8589934592]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── j
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <10> Constant Int [123456]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <29> Var [j]
                    │   │       ╰── <31> Constant Long [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <33> Constant Int [2]
                    ╰── Return
                        ╰── <38> Constant Int [0]
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
                    │   │   ╰── <12>  [<]
                    │   │       ├── <9>  [+]
                    │   │       │   ├── <6> Constant Long [2147483647]
                    │   │       │   ╰── <8> Constant Long [2147483647]
                    │   │       ╰── <11> Constant Long [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <13> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [<]
                    │   │       ├── <18> Constant Long [19327352832]
                    │   │       ╰── <20> Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <22> Constant Int [2]
                    ╰── Return
                        ╰── <27> Constant Int [0]
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
                    │   │   ╰── <41>  [<]
                    │   │       ├── <38>  [+]
                    │   │       │   ├── <34> Var [d]
                    │   │       │   ╰── <37> Var [f]
                    │   │       ╰── <40> Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <42> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [<]
                    │   │       ├── <48> Var [i]
                    │   │       ╰── <50> Constant Long [100]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <52> Constant Int [2]
                    ╰── Return
                        ╰── <55> Constant Int [0]
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
                        ╰── <49> FunctionCall [test_sum]
                            ├── <40> Constant Int [0]
                            ├── <41> Constant Int [0]
                            ├── <42> Constant Int [0]
                            ├── <43> Constant Long [34359738368]
                            ├── <44> Constant Int [0]
                            ├── <45> Constant Long [34359738368]
                            ├── <46> Constant Int [0]
                            ├── <47> Constant Int [0]
                            ╰── <48> Constant Long [34359738368]
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
            │       ╰── <4> Constant Long [8589934592]
            ├── Function [return_l]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Var [l]
            ╰── Function [return_l_as_int]
                ╰── Body
                    ╰── Return
                        ╰── <23> Var [l]
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
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> FunctionCall [return_l]
                    │   │       ╰── <26> Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <28> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> FunctionCall [return_l_as_int]
                    │   │       ╰── <34> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <36> Constant Int [2]
                    ├── <47> Assign [=]
                    │   ├── <40> Var [l]
                    │   ╰── <46>  [-]
                    │       ├── <43> Var [l]
                    │       ╰── <45> Constant Long [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> FunctionCall [return_l]
                    │   │       ╰── <52> Constant Long [8589934582]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <54> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <58> FunctionCall [return_l_as_int]
                    │   │       ╰── <62> Unary [-]
                    │   │           ╰── <61> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <64> Constant Int [4]
                    ╰── Return
                        ╰── <67> Constant Int [0]
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
                        ╰── <24>  [+]
                            ├── <20>  [+]
                            │   ├── <16> Var [x]
                            │   ╰── <19> Var [y]
                            ╰── <23> Var [z]
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
                    │       ╰── <24> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <30> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <36> Constant Int [5]
                    ╰── Return
                        ╰── <46> FunctionCall [add_variables]
                            ├── <41> Var [x]
                            ├── <43> Var [y]
                            ╰── <45> Var [z]
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
                        ╰── <23>  [+]
                            ├── <16> Cast
                            │   ├── Target
                            │   │   ╰── Long
                            │   ╰── Expression
                            │       ╰── <15> Var [a]
                            ╰── <22> Cast
                                ├── Target
                                │   ╰── Long
                                ╰── Expression
                                    ╰── <21> Var [b]
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
                    │       ╰── <24> FunctionCall [add]
                    │           ├── <22> Constant Int [2147483645]
                    │           ╰── <23> Constant Int [2147483645]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [a]
                    │   │       ╰── <30> Constant Long [4294967290]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
            │           ╰── <23>  [==]
            │               ├── <19>  [+]
            │               │   ├── <15> Var [a]
            │               │   ╰── <18> Var [b]
            │               ╰── <21> Constant Long [4294967295]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <43>  [==]
            │               ├── <37>  [-]
            │               │   ├── <33> Var [a]
            │               │   ╰── <36> Var [b]
            │               ╰── <41> Unary [-]
            │                   ╰── <40> Constant Long [4294967380]
            ├── Function [multiplication]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <60>  [==]
            │               ├── <56>  [*]
            │               │   ├── <53> Var [a]
            │               │   ╰── <55> Constant Long [4]
            │               ╰── <58> Constant Long [17179869160]
            ├── Function [division]
            │   ╰── Body
            │       ├── <77> Assign [=]
            │       │   ├── <70> Var [b]
            │       │   ╰── <76>  [/]
            │       │       ├── <73> Var [a]
            │       │       ╰── <75> Constant Long [128]
            │       ╰── Return
            │           ╰── <84>  [==]
            │               ├── <80> Var [b]
            │               ╰── <82> Constant Long [33554431]
            ├── Function [remaind]
            │   ╰── Body
            │       ├── <103> Assign [=]
            │       │   ├── <94> Var [b]
            │       │   ╰── <102>  [%]
            │       │       ├── <99> Unary [-]
            │       │       │   ╰── <98> Var [a]
            │       │       ╰── <101> Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <112>  [==]
            │               ├── <106> Var [b]
            │               ╰── <110> Unary [-]
            │                   ╰── <109> Constant Long [5]
            ├── Function [complement]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <130>  [==]
            │               ├── <124> Unary [~]
            │               │   ╰── <123> Var [a]
            │               ╰── <128> Unary [-]
            │                   ╰── <127> Constant Long [9223372036854775807]
            ╰── Function [main]
                ╰── Body
                    ├── <143> Assign [=]
                    │   ├── <140> Var [a]
                    │   ╰── <142> Constant Long [4294967290]
                    ├── <149> Assign [=]
                    │   ├── <146> Var [b]
                    │   ╰── <148> Constant Long [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154> Unary [!]
                    │   │       ╰── <153> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <155> Constant Int [1]
                    ├── <166> Assign [=]
                    │   ├── <161> Var [a]
                    │   ╰── <165> Unary [-]
                    │       ╰── <164> Constant Long [4294967290]
                    ├── <172> Assign [=]
                    │   ├── <169> Var [b]
                    │   ╰── <171> Constant Long [90]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <177> Unary [!]
                    │   │       ╰── <176> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <178> Constant Int [2]
                    ├── <187> Assign [=]
                    │   ├── <184> Var [a]
                    │   ╰── <186> Constant Long [4294967290]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192> Unary [!]
                    │   │       ╰── <191> FunctionCall [multiplication]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <193> Constant Int [3]
                    ├── <202> Assign [=]
                    │   ├── <199> Var [a]
                    │   ╰── <201> Constant Long [4294967290]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <207> Unary [!]
                    │   │       ╰── <206> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <208> Constant Int [4]
                    ├── <217> Assign [=]
                    │   ├── <214> Var [a]
                    │   ╰── <216> Constant Long [8589934585]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <222> Unary [!]
                    │   │       ╰── <221> FunctionCall [remaind]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <223> Constant Int [5]
                    ├── <232> Assign [=]
                    │   ├── <229> Var [a]
                    │   ╰── <231> Constant Long [9223372036854775806]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <237> Unary [!]
                    │   │       ╰── <236> FunctionCall [complement]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <238> Constant Int [6]
                    ╰── Return
                        ╰── <243> Constant Int [0]
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
                    │       ╰── <9> Constant Long [4294967290]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <15> Constant Long [0]
                    ├── <23> Assign [=]
                    │   ├── <19> Var [b]
                    │   ╰── <22> Var [a]
                    ╰── Return
                        ╰── <30>  [==]
                            ├── <26> Var [b]
                            ╰── <28> Constant Long [4294967290]
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
            │           ╰── <17>  [>]
            │               ├── <14> Constant Long [8589934593]
            │               ╰── <16> Constant Long [255]
            ├── Function [compare_constants_2]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <29>  [<]
            │               ├── <26> Constant Long [255]
            │               ╰── <28> Constant Long [8589934593]
            ├── Function [l_geq_2_60]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <43>  [>=]
            │               ├── <39> Var [l]
            │               ╰── <41> Constant Long [1152921504606846976]
            ├── Function [uint_max_leq_l]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <57>  [<=]
            │               ├── <52> Constant Long [4294967295]
            │               ╰── <55> Var [l]
            ├── Function [l_eq_l2]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <72>  [==]
            │               ├── <67> Var [l]
            │               ╰── <70> Var [l2]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84> Unary [!]
                    │   │       ╰── <83> FunctionCall [compare_constants]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <85> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93> Unary [!]
                    │   │       ╰── <92> FunctionCall [compare_constants_2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [2]
                    ├── <105> Assign [=]
                    │   ├── <100> Var [l]
                    │   ╰── <104> Unary [-]
                    │       ╰── <103> Constant Long [9223372036854775807]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108> FunctionCall [l_geq_2_60]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <109> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115> FunctionCall [uint_max_leq_l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [4]
                    ├── <125> Assign [=]
                    │   ├── <122> Var [l]
                    │   ╰── <124> Constant Long [1152921504606846976]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130> Unary [!]
                    │   │       ╰── <129> FunctionCall [l_geq_2_60]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <131> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <139> Unary [!]
                    │   │       ╰── <138> FunctionCall [uint_max_leq_l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <140> Constant Int [6]
                    ├── <150> Assign [=]
                    │   ├── <146> Var [l2]
                    │   ╰── <149> Var [l]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <155> Unary [!]
                    │   │       ╰── <154> FunctionCall [l_eq_l2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <156> Constant Int [7]
                    ╰── Return
                        ╰── <161> Constant Int [0]
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
            │       ╰── <4> Constant Long [5]
            ├── Function [add_large]
            │   ╰── Body
            │       ├── <20> Assign [=]
            │       │   ├── <13> Var [x]
            │       │   ╰── <19>  [+]
            │       │       ├── <16> Var [x]
            │       │       ╰── <18> Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <23> Var [x]
            │               ╰── <25> Constant Long [4294967295]
            ├── Function [subtract_large]
            │   ╰── Body
            │       ├── <44> Assign [=]
            │       │   ├── <37> Var [x]
            │       │   ╰── <43>  [-]
            │       │       ├── <40> Var [x]
            │       │       ╰── <42> Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <51>  [==]
            │               ├── <47> Var [x]
            │               ╰── <49> Constant Long [5]
            ├── Function [multiply_by_large]
            │   ╰── Body
            │       ├── <68> Assign [=]
            │       │   ├── <61> Var [x]
            │       │   ╰── <67>  [*]
            │       │       ├── <64> Var [x]
            │       │       ╰── <66> Constant Long [4294967290]
            │       ╰── Return
            │           ╰── <75>  [==]
            │               ├── <71> Var [x]
            │               ╰── <73> Constant Long [21474836450]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87> Unary [!]
                    │   │       ╰── <86> FunctionCall [add_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <96> Unary [!]
                    │   │       ╰── <95> FunctionCall [subtract_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <97> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105> Unary [!]
                    │   │       ╰── <104> FunctionCall [multiply_by_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [3]
                    ╰── Return
                        ╰── <111> Constant Int [0]
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
            │           ╰── <12> Unary [!]
            │               ╰── <11> Var [l]
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
            │       │   │   ╰── <25> Var [l]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <26> Constant Int [1]
            │       ╰── Return
            │           ╰── <31> Constant Int [0]
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
            │           ╰── <51>  [&&]
            │               ├── <47> Var [l1]
            │               ╰── <50> Var [l2]
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
            │           ╰── <71>  [||]
            │               ├── <67> Var [l1]
            │               ╰── <70> Var [l2]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <83> Constant Long [1152921504606846976]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <89> Constant Long [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95> FunctionCall [not]
                    │   │       ╰── <94> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <96> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106> Unary [!]
                    │   │       ╰── <105> FunctionCall [not]
                    │   │           ╰── <104> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <107> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117> Unary [!]
                    │   │       ╰── <116> FunctionCall [if_cond]
                    │   │           ╰── <115> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <126> FunctionCall [if_cond]
                    │   │       ╰── <125> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <127> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136> FunctionCall [and]
                    │   │       ├── <134> Var [zero]
                    │   │       ╰── <135> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <137> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148> Unary [!]
                    │   │       ╰── <147> FunctionCall [or]
                    │   │           ├── <144> Constant Int [1]
                    │   │           ╰── <146> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <149> Constant Int [6]
                    ╰── Return
                        ╰── <154> Constant Int [0]
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
                    │       ╰── <9> Constant Long [8589934592]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Unary [-]
                    │           ╰── <16> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <25> Unary [-]
                    │           ╰── <24> Constant Long [8589934592]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <31> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> Var [a]
                    │   │       ╰── <37> Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <39> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <45> Var [b]
                    │   │       ╰── <49> Unary [-]
                    │   │           ╰── <48> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <57> Var [c]
                    │   │       ╰── <61> Unary [-]
                    │   │           ╰── <60> Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Var [d]
                    │   │       ╰── <71> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [4]
                    ├── <85> Assign [=]
                    │   ├── <79> Var [a]
                    │   ╰── <84> Unary [-]
                    │       ╰── <83> Var [a]
                    ├── <95> Assign [=]
                    │   ├── <88> Var [b]
                    │   ╰── <94>  [-]
                    │       ├── <91> Var [b]
                    │       ╰── <93> Constant Int [1]
                    ├── <105> Assign [=]
                    │   ├── <98> Var [c]
                    │   ╰── <104>  [+]
                    │       ├── <101> Var [c]
                    │       ╰── <103> Constant Long [8589934594]
                    ├── <115> Assign [=]
                    │   ├── <108> Var [d]
                    │   ╰── <114>  [+]
                    │       ├── <111> Var [d]
                    │       ╰── <113> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123>  [!=]
                    │   │       ├── <118> Var [a]
                    │   │       ╰── <122> Unary [-]
                    │   │           ╰── <121> Constant Long [8589934592]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <124> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135>  [!=]
                    │   │       ├── <130> Var [b]
                    │   │       ╰── <134> Unary [-]
                    │   │           ╰── <133> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <136> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [!=]
                    │   │       ├── <142> Var [c]
                    │   │       ╰── <144> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <146> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <155>  [!=]
                    │   │       ├── <152> Var [d]
                    │   │       ╰── <154> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <156> Constant Int [8]
                    ╰── Return
                        ╰── <161> Constant Int [0]
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
            │       │   │   ╰── <41>  [<]
            │       │   │       ├── <38>  [+]
            │       │   │       │   ├── <34> Var [a]
            │       │   │       │   ╰── <37> Var [b]
            │       │   │       ╰── <40> Constant Long [100]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <42> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <51>  [<]
            │       │   │       ├── <48> Var [i]
            │       │   │       ╰── <50> Constant Long [100]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <52> Constant Int [2]
            │       ╰── Return
            │           ╰── <55> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <74> FunctionCall [test_sum]
                            ├── <65> Constant Long [34359738368]
                            ├── <66> Constant Long [34359738368]
                            ├── <67> Constant Int [0]
                            ├── <68> Constant Int [0]
                            ├── <69> Constant Int [0]
                            ├── <70> Constant Int [0]
                            ├── <71> Constant Int [0]
                            ├── <72> Constant Int [0]
                            ╰── <73> Constant Long [34359738368]
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
            │       │       ╰── <19>  [-]
            │       │           ├── <16>  [*]
            │       │           │   ├── <13> Var [a]
            │       │           │   ╰── <15> Constant Long [5]
            │       │           ╰── <18> Constant Long [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <26>  [==]
            │       │   │       ├── <23> Var [b]
            │       │   │       ╰── <25> Constant Long [21474836440]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <27> Constant Int [1]
            │       ╰── Return
            │           ╰── <32> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <43> FunctionCall [target]
                            ╰── <42> Constant Long [4294967290]
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
            │           ╰── <23>  [+]
            │               ├── <16> Cast
            │               │   ├── Target
            │               │   │   ╰── Long
            │               │   ╰── Expression
            │               │       ╰── <15> Var [a]
            │               ╰── <22> Cast
            │                   ├── Target
            │                   │   ╰── Long
            │                   ╰── Expression
            │                       ╰── <21> Var [b]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <38> FunctionCall [add]
                    │           ├── <36> Constant Int [2147483645]
                    │           ╰── <37> Constant Int [2147483645]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [==]
                    │   │       ├── <42> Var [a]
                    │   │       ╰── <44> Constant Long [4294967290]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [1]
                    ╰── Return
                        ╰── <51> Constant Int [0]
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
            │       ╰── <49> Constant Long [5]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── should_spill
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <64>  [*]
            │       │           ├── <61> Var [glob]
            │       │           ╰── <63> Constant Long [4294967307]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <74>  [-]
            │       │           ├── <71> Var [glob]
            │       │           ╰── <73> Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <85>  [+]
            │       │           ├── <81> Var [one]
            │       │           ╰── <84> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <95>  [+]
            │       │           ├── <91> Constant Int [2]
            │       │           ╰── <94> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <106>  [*]
            │       │           ├── <102> Var [two]
            │       │           ╰── <105> Var [two]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── five
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <116>  [-]
            │       │           ├── <112> Constant Int [6]
            │       │           ╰── <115> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── six
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <127>  [*]
            │       │           ├── <123> Var [two]
            │       │           ╰── <126> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <137>  [+]
            │       │           ├── <134> Var [one]
            │       │           ╰── <136> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eight
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <147>  [*]
            │       │           ├── <144> Var [two]
            │       │           ╰── <146> Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nine
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <158>  [*]
            │       │           ├── <154> Var [three]
            │       │           ╰── <157> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ten
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <169>  [+]
            │       │           ├── <165> Var [four]
            │       │           ╰── <168> Var [six]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eleven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <179>  [-]
            │       │           ├── <175> Constant Int [16]
            │       │           ╰── <178> Var [five]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twelve
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <190>  [+]
            │       │           ├── <186> Var [six]
            │       │           ╰── <189> Var [six]
            │       ├── <219> FunctionCall [check_12_ints]
            │       │   ├── <195> Var [one]
            │       │   ├── <197> Var [two]
            │       │   ├── <199> Var [three]
            │       │   ├── <201> Var [four]
            │       │   ├── <203> Var [five]
            │       │   ├── <205> Var [six]
            │       │   ├── <207> Var [seven]
            │       │   ├── <209> Var [eight]
            │       │   ├── <211> Var [nine]
            │       │   ├── <213> Var [ten]
            │       │   ├── <215> Var [eleven]
            │       │   ├── <217> Var [twelve]
            │       │   ╰── <218> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── thirteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <228>  [+]
            │       │           ├── <225> Var [glob]
            │       │           ╰── <227> Constant Int [8]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fourteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <238>  [+]
            │       │           ├── <235> Var [thirteen]
            │       │           ╰── <237> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fifteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <248>  [-]
            │       │           ├── <244> Constant Int [28]
            │       │           ╰── <247> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── sixteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <258>  [+]
            │       │           ├── <255> Var [fourteen]
            │       │           ╰── <257> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seventeen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <268>  [+]
            │       │           ├── <264> Constant Int [4]
            │       │           ╰── <267> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eighteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <278>  [-]
            │       │           ├── <274> Constant Int [32]
            │       │           ╰── <277> Var [fourteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nineteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <288>  [-]
            │       │           ├── <284> Constant Int [35]
            │       │           ╰── <287> Var [sixteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <298>  [+]
            │       │           ├── <295> Var [fifteen]
            │       │           ╰── <297> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <311>  [-]
            │       │           ├── <308>  [*]
            │       │           │   ├── <305> Var [thirteen]
            │       │           │   ╰── <307> Constant Int [2]
            │       │           ╰── <310> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <321>  [+]
            │       │           ├── <318> Var [fifteen]
            │       │           ╰── <320> Constant Int [7]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <331>  [+]
            │       │           ├── <327> Constant Int [6]
            │       │           ╰── <330> Var [seventeen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <341>  [+]
            │       │           ├── <338> Var [thirteen]
            │       │           ╰── <340> Constant Int [11]
            │       ├── <370> FunctionCall [check_12_ints]
            │       │   ├── <346> Var [thirteen]
            │       │   ├── <348> Var [fourteen]
            │       │   ├── <350> Var [fifteen]
            │       │   ├── <352> Var [sixteen]
            │       │   ├── <354> Var [seventeen]
            │       │   ├── <356> Var [eighteen]
            │       │   ├── <358> Var [nineteen]
            │       │   ├── <360> Var [twenty]
            │       │   ├── <362> Var [twenty_one]
            │       │   ├── <364> Var [twenty_two]
            │       │   ├── <366> Var [twenty_three]
            │       │   ├── <368> Var [twenty_four]
            │       │   ╰── <369> Constant Int [13]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <376>  [!=]
            │       │   │       ├── <373> Var [should_spill]
            │       │   │       ╰── <375> Constant Long [21474836535]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <379> Unary [-]
            │       │                   ╰── <378> Constant Int [1]
            │       ╰── Return
            │           ╰── <384> Constant Int [0]
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
                    │       ╰── <435> Constant Int [0]
                    ├── <446> Assign [=]
                    │   ├── <439> Var [expected]
                    │   ╰── <445>  [+]
                    │       ├── <442> Var [start]
                    │       ╰── <444> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <453>  [!=]
                    │   │       ├── <449> Var [a]
                    │   │       ╰── <452> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <455> Var [expected]
                    ├── <468> Assign [=]
                    │   ├── <461> Var [expected]
                    │   ╰── <467>  [+]
                    │       ├── <464> Var [start]
                    │       ╰── <466> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <475>  [!=]
                    │   │       ├── <471> Var [b]
                    │   │       ╰── <474> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <477> Var [expected]
                    ├── <490> Assign [=]
                    │   ├── <483> Var [expected]
                    │   ╰── <489>  [+]
                    │       ├── <486> Var [start]
                    │       ╰── <488> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <497>  [!=]
                    │   │       ├── <493> Var [c]
                    │   │       ╰── <496> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <499> Var [expected]
                    ├── <512> Assign [=]
                    │   ├── <505> Var [expected]
                    │   ╰── <511>  [+]
                    │       ├── <508> Var [start]
                    │       ╰── <510> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <519>  [!=]
                    │   │       ├── <515> Var [d]
                    │   │       ╰── <518> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <521> Var [expected]
                    ├── <534> Assign [=]
                    │   ├── <527> Var [expected]
                    │   ╰── <533>  [+]
                    │       ├── <530> Var [start]
                    │       ╰── <532> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <541>  [!=]
                    │   │       ├── <537> Var [e]
                    │   │       ╰── <540> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <543> Var [expected]
                    ├── <556> Assign [=]
                    │   ├── <549> Var [expected]
                    │   ╰── <555>  [+]
                    │       ├── <552> Var [start]
                    │       ╰── <554> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <563>  [!=]
                    │   │       ├── <559> Var [f]
                    │   │       ╰── <562> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <565> Var [expected]
                    ├── <578> Assign [=]
                    │   ├── <571> Var [expected]
                    │   ╰── <577>  [+]
                    │       ├── <574> Var [start]
                    │       ╰── <576> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <585>  [!=]
                    │   │       ├── <581> Var [g]
                    │   │       ╰── <584> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <587> Var [expected]
                    ├── <600> Assign [=]
                    │   ├── <593> Var [expected]
                    │   ╰── <599>  [+]
                    │       ├── <596> Var [start]
                    │       ╰── <598> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <607>  [!=]
                    │   │       ├── <603> Var [h]
                    │   │       ╰── <606> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <609> Var [expected]
                    ├── <622> Assign [=]
                    │   ├── <615> Var [expected]
                    │   ╰── <621>  [+]
                    │       ├── <618> Var [start]
                    │       ╰── <620> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <629>  [!=]
                    │   │       ├── <625> Var [i]
                    │   │       ╰── <628> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <631> Var [expected]
                    ├── <644> Assign [=]
                    │   ├── <637> Var [expected]
                    │   ╰── <643>  [+]
                    │       ├── <640> Var [start]
                    │       ╰── <642> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <651>  [!=]
                    │   │       ├── <647> Var [j]
                    │   │       ╰── <650> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <653> Var [expected]
                    ├── <666> Assign [=]
                    │   ├── <659> Var [expected]
                    │   ╰── <665>  [+]
                    │       ├── <662> Var [start]
                    │       ╰── <664> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <673>  [!=]
                    │   │       ├── <669> Var [k]
                    │   │       ╰── <672> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <675> Var [expected]
                    ├── <688> Assign [=]
                    │   ├── <681> Var [expected]
                    │   ╰── <687>  [+]
                    │       ├── <684> Var [start]
                    │       ╰── <686> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <695>  [!=]
                    │   │       ├── <691> Var [l]
                    │   │       ╰── <694> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <697> Var [expected]
                    ╰── Return
                        ╰── <702> Constant Int [0]
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
                    │       ╰── <9> Constant Long [9223372036854775807]
                    ╰── Return
                        ╰── <20>  [==]
                            ├── <16>  [-]
                            │   ├── <13> Var [l]
                            │   ╰── <15> Constant Long [2]
                            ╰── <18> Constant Long [9223372036854775805]
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
            │   │   ╰── <5> Constant Long [4294967290]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [==]
                    │   │       ├── <17>  [+]
                    │   │       │   ├── <14> Var [foo]
                    │   │       │   ╰── <16> Constant Long [5]
                    │   │       ╰── <19> Constant Long [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── <25> Assign [=]
                    │           │   ├── <22> Var [foo]
                    │           │   ╰── <24> Constant Long [1152921504606846988]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <31>  [==]
                    │               │       ├── <28> Var [foo]
                    │               │       ╰── <30> Constant Long [1152921504606846988]
                    │               ╰── Then
                    │                   ╰── Return
                    │                       ╰── <32> Constant Int [1]
                    ╰── Return
                        ╰── <38> Constant Int [0]
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
            │           ╰── <54>  [+]
            │               ├── <50>  [+]
            │               │   ├── <46> Var [x]
            │               │   ╰── <49> Var [y]
            │               ╰── <53> Var [z]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <66> Constant Long [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <72> Constant Long [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <78> Constant Long [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Extern
                    ├── <90> Assign [=]
                    │   ├── <87> Var [a]
                    │   ╰── <89> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <95> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Long
                    │   │       ╰── Initializer
                    │   │           ╰── <101> Constant Long [1099511627776]
                    │   ├── Condition
                    │   │   ╰── <109>  [>]
                    │   │       ├── <106> Var [i]
                    │   │       ╰── <108> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <118> Assign [=]
                    │   │       ├── <111> Var [i]
                    │   │       ╰── <117>  [/]
                    │   │           ├── <114> Var [i]
                    │   │           ╰── <116> Constant Int [2]
                    │   ╰── Block
                    │       ╰── <127> Assign [=]
                    │           ├── <120> Var [sum]
                    │           ╰── <126>  [+]
                    │               ├── <123> Var [sum]
                    │               ╰── <125> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <133> Var [x]
                    │   │       ╰── <135> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <137> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <146>  [!=]
                    │   │       ├── <143> Var [y]
                    │   │       ╰── <145> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <147> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <153> Var [a]
                    │   │       ╰── <155> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <157> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172>  [!=]
                    │   │       ├── <169> FunctionCall [my_function]
                    │   │       │   ├── <164> Var [x]
                    │   │       │   ├── <166> Var [y]
                    │   │       │   ╰── <168> Var [z]
                    │   │       ╰── <171> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <173> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <182>  [!=]
                    │   │       ├── <179> Var [sum]
                    │   │       ╰── <181> Constant Int [41]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <183> Constant Int [5]
                    ╰── Return
                        ╰── <188> Constant Int [0]
    "#;
    assert_parse(src, expected);
}
