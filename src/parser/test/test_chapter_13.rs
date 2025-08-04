use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_invalid_type_specifier() {
    assert_error(
        r#"
        int main(void) {
            unsigned double d = 10.0;
          //^^^^^^^^^^^^^^^ Invalid type specifier
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_type_specifier_2() {
    assert_error(
        r#"
        int main(void) {
            double double d = 10.0;
          //^^^^^^^^^^^^^ Invalid type specifier
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_complement_double() {
    let src = r#"
        int main(void) {
            double d = ~10.0;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <11> Unary [~]
                    │           ╰── <10> Constant Double [+1e1]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_and() {
    let src = r#"
        int main(void) {
            double d = 10.0 & -1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <14>  [&]
                    │           ├── <9> Constant Double [+1e1]
                    │           ╰── <13> Unary [-]
                    │               ╰── <12> Constant Int [1]
                    ╰── Return
                        ╰── <17> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_or() {
    let src = r#"
        int main(void) {
            double d = 0.0 | -0.0;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <14>  [|]
                    │           ├── <9> Constant Double [+0e0]
                    │           ╰── <13> Unary [-]
                    │               ╰── <12> Constant Double [+0e0]
                    ╰── Return
                        ╰── <17> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_shift_double() {
    let src = r#"
        int main(void) {
            double d = 5.0 << 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <12>  [<<]
                    │           ├── <9> Constant Double [+5e0]
                    │           ╰── <11> Constant Int [3]
                    ╰── Return
                        ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_shift_double_2() {
    let src = r#"
        int main(void) {
            return 1 << 2.0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [<<]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Double [+2e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_xor() {
    let src = r#"
        int main(void) {
            return 1e10 ^ -1e10;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [^]
                            ├── <6> Constant Double [+1e10]
                            ╰── <10> Unary [-]
                                ╰── <9> Constant Double [+1e10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_bitwise_and() {
    let src = r#"
        int main(void) {
            double d = 1.0;
            d &= 0;
            return (int) d;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1e0]
                    ├── <16> Assign [&=]
                    │   ├── <13> Var [d]
                    │   ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <22> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <21> Var [d]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_bitwise_xor() {
    let src = r#"
        int main(void) {
            int i = 0;
            i |= 2.0;
            return (int) i;
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
                    ├── <16> Assign [|=]
                    │   ├── <13> Var [i]
                    │   ╰── <15> Constant Double [+2e0]
                    ╰── Return
                        ╰── <22> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <21> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_left_bitshift() {
    let src = r#"
        int main(void) {
            double d = 1.0;
            d <<= 1;
            return d;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1e0]
                    ├── <16> Assign [<<=]
                    │   ├── <13> Var [d]
                    │   ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <19> Var [d]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_mod() {
    let src = r#"
        
        int main(void) {
            double d = 5.0;
            d %= 2;
            return (int) d;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+5e0]
                    ├── <16> Assign [&=]
                    │   ├── <13> Var [d]
                    │   ╰── <15> Constant Int [2]
                    ╰── Return
                        ╰── <22> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <21> Var [d]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_mod_2() {
    let src = r#"
        
        int main(void) {
            int i = 5;
            i %= 1.0;
            return i;
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
                    │       ╰── <9> Constant Int [5]
                    ├── <16> Assign [&=]
                    │   ├── <13> Var [i]
                    │   ╰── <15> Constant Double [+1e0]
                    ╰── Return
                        ╰── <19> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_right_bitshift() {
    let src = r#"
        int main(void) {
            int i = 1000;
            i >>= 2.0;
            return i;
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
                    │       ╰── <9> Constant Int [1000]
                    ├── <16> Assign [>>=]
                    │   ├── <13> Var [i]
                    │   ╰── <15> Constant Double [+2e0]
                    ╰── Return
                        ╰── <19> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_double_case() {
    let src = r#"
        int main(void) {
            int x = 10;
            switch (x) {
                case 1.0: return 0;
                default: return 4;
            }
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
                    │       ╰── <9> Constant Int [10]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [x]
                        ╰── Block
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── <15> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <18> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_on_double() {
    let src = r#"
        int main(void) {
            double d = 10;
            switch (d) {
                case 10: return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [d]
                    │   ╰── Block
                    │       ╰── Case [10]
                    │           ╰── Return
                    │               ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <21> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_mod_double() {
    let src = r#"
        int main(void) {
            double d = 10.0;
            d = d % 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1e1]
                    ├── <20> Assign [=]
                    │   ├── <13> Var [d]
                    │   ╰── <19>  [%]
                    │       ├── <16> Var [d]
                    │       ╰── <18> Constant Int [3]
                    ╰── Return
                        ╰── <22> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_mod_double_2() {
    let src = r#"
        int main(void) {
            double e = 3.0 % 5;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <12>  [%]
                    │           ├── <9> Constant Double [+3e0]
                    │           ╰── <11> Constant Int [5]
                    ╰── Return
                        ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_constants_constant_doubles() {
    let src = r#"
        int main(void) {
            double a = 1.0;
            double b = 1.;
            double c = 1E0;
            double d = .01e+2;
            if (! (a == b && a == c && a == d) )
                return 1;
            if (a + b + c + d != 4.0)
                return 2;
            double e = .125;
            double f = 12.5e-2;
            double g = 125.E-3;
            double h = 1250000000e-10;
            if (! (e == f && e == g && e == h) )
                return 3;
            if (e + f + g + h != 0.5)
                return 4;
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
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <15> Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <21> Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <27> Constant Double [+1e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54> Unary [!]
                    │   │       ╰── <53>  [&&]
                    │   │           ├── <44>  [&&]
                    │   │           │   ├── <36>  [==]
                    │   │           │   │   ├── <32> Var [a]
                    │   │           │   │   ╰── <35> Var [b]
                    │   │           │   ╰── <43>  [==]
                    │   │           │       ├── <39> Var [a]
                    │   │           │       ╰── <42> Var [c]
                    │   │           ╰── <51>  [==]
                    │   │               ├── <47> Var [a]
                    │   │               ╰── <50> Var [d]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <55> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71>  [+]
                    │   │       │   ├── <67>  [+]
                    │   │       │   │   ├── <63>  [+]
                    │   │       │   │   │   ├── <59> Var [a]
                    │   │       │   │   │   ╰── <62> Var [b]
                    │   │       │   │   ╰── <66> Var [c]
                    │   │       │   ╰── <70> Var [d]
                    │   │       ╰── <73> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <75> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <81> Constant Double [+1.25e-1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <87> Constant Double [+1.25e-1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <93> Constant Double [+1.25e-1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <99> Constant Double [+1.25e-1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <126> Unary [!]
                    │   │       ╰── <125>  [&&]
                    │   │           ├── <116>  [&&]
                    │   │           │   ├── <108>  [==]
                    │   │           │   │   ├── <104> Var [e]
                    │   │           │   │   ╰── <107> Var [f]
                    │   │           │   ╰── <115>  [==]
                    │   │           │       ├── <111> Var [e]
                    │   │           │       ╰── <114> Var [g]
                    │   │           ╰── <123>  [==]
                    │   │               ├── <119> Var [e]
                    │   │               ╰── <122> Var [h]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <127> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <146>  [!=]
                    │   │       ├── <143>  [+]
                    │   │       │   ├── <139>  [+]
                    │   │       │   │   ├── <135>  [+]
                    │   │       │   │   │   ├── <131> Var [e]
                    │   │       │   │   │   ╰── <134> Var [f]
                    │   │       │   │   ╰── <138> Var [g]
                    │   │       │   ╰── <142> Var [h]
                    │   │       ╰── <145> Constant Double [+5e-1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <147> Constant Int [4]
                    ╰── Return
                        ╰── <150> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_constants_round_constants() {
    let src = r#"
        int main(void) {
            if (1.00000000000000033306690738754696212708950042724609375 != 1.0000000000000004) {
                return 1;
            }
            if (9223372036854776832.5 != 9223372036854777856.0) {
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
                    │   │   ╰── <9>  [!=]
                    │   │       ├── <6> Constant Double [+1.0000000000000004e0]
                    │   │       ╰── <8> Constant Double [+1.0000000000000004e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <10> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <18>  [!=]
                    │   │       ├── <15> Constant Double [+9.223372036854778e18]
                    │   │       ╰── <17> Constant Double [+9.223372036854778e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <19> Constant Int [2]
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_cvttsd2si_rewrite() {
    let src = r#"
        double glob = 3.0;
        int main(void) {
            long l = -1l;
            int i = -1;
            int j = (int) glob;
            int k = 20;
            if (l != -1l) {
                return 1;
            }
            if (i != -1) {
                return 2;
            }
            if (j != 3) {
                return 3;
            }
            if (k != 20) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── glob
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <4> Constant Double [+3e0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <17> Unary [-]
                    │           ╰── <16> Constant Long [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <25> Unary [-]
                    │           ╰── <24> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <34> Var [glob]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── k
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <41> Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <45> Var [l]
                    │   │       ╰── <49> Unary [-]
                    │   │           ╰── <48> Constant Long [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <57> Var [i]
                    │   │       ╰── <61> Unary [-]
                    │   │           ╰── <60> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Var [j]
                    │   │       ╰── <71> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Var [k]
                    │   │       ╰── <81> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [4]
                    ╰── Return
                        ╰── <88> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_double_to_signed() {
    let src = r#"
        int double_to_int(double d) {
            return (int) d;
        }
        long double_to_long(double d) {
            return (long) d;
        }
        int main(void) {
            long l = double_to_long(2148429099.3);
            if (l != 2148429099l) {
                return 1;
            }
            int i = double_to_int(-200000.9999);
            if (i != -200000) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [double_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <12> Var [d]
            ├── Function [double_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <29> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <28> Var [d]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <43> FunctionCall [double_to_long]
                    │           ╰── <42> Constant Double [+2.1484290993e9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Var [l]
                    │   │       ╰── <49> Constant Long [2148429099]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <63> FunctionCall [double_to_int]
                    │           ╰── <62> Unary [-]
                    │               ╰── <61> Constant Double [+2.000009999e5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <67> Var [i]
                    │   │       ╰── <71> Unary [-]
                    │   │           ╰── <70> Constant Int [200000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [2]
                    ╰── Return
                        ╰── <78> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_double_to_unsigned() {
    let src = r#"
        unsigned int double_to_uint(double d) {
            return (unsigned int) d;
        }
        unsigned long double_to_ulong(double d) {
            return (unsigned long) d;
        }
        int main(void) {
            if (double_to_uint(10.9) != 10u) {
                return 1;
            }
            if (double_to_uint(2147483750.5) != 2147483750) {
                return 2;
            }
            if (double_to_ulong(34359738368.5) != 34359738368ul) {
                return 3;
            }
            if (double_to_ulong(3458764513821589504.0) != 3458764513821589504ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [double_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <12> Var [d]
            ├── Function [double_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <29> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <28> Var [d]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> FunctionCall [double_to_uint]
                    │   │       │   ╰── <39> Constant Double [+1.09e1]
                    │   │       ╰── <42> Constant UInt [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> FunctionCall [double_to_uint]
                    │   │       │   ╰── <50> Constant Double [+2.1474837505e9]
                    │   │       ╰── <53> Constant Long [2147483750]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> FunctionCall [double_to_ulong]
                    │   │       │   ╰── <61> Constant Double [+3.43597383685e10]
                    │   │       ╰── <64> Constant ULong [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> FunctionCall [double_to_ulong]
                    │   │       │   ╰── <72> Constant Double [+3.4587645138215895e18]
                    │   │       ╰── <75> Constant ULong [3458764513821589504]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [4]
                    ╰── Return
                        ╰── <82> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_rewrite_cvttsd2si_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        double glob = 5000.;
        int main(void) {
            long should_spill = (long)glob;
            int one = glob - 4999;
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
            int thirteen = glob - 4987;
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
            if (should_spill != 5000) {
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
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <49> Constant Double [+5e3]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── should_spill
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <64> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <63> Var [glob]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <74>  [-]
            │       │           ├── <71> Var [glob]
            │       │           ╰── <73> Constant Int [4999]
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
            │       │       ╰── <228>  [-]
            │       │           ├── <225> Var [glob]
            │       │           ╰── <227> Constant Int [4987]
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
            │       │   │       ╰── <375> Constant Int [5000]
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
fn test_valid_explicit_casts_signed_to_double() {
    let src = r#"
        
        double int_to_double(int i) {
            return (double) i;
        }
        double long_to_double(long l) {
            return (double) l;
        }
        int main(void) {
            if (int_to_double(-100000) != -100000.0) {
                return 1;
            }
            if (long_to_double(-9007199254751227l) != -9007199254751228.0) {
                return 2;
            }
            double d = (double) 1152921504606846977l;
            if (d != 1152921504606846976.0) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [int_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <12> Var [i]
            ├── Function [long_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <29> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <28> Var [l]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <42> FunctionCall [int_to_double]
                    │   │       │   ╰── <41> Unary [-]
                    │   │       │       ╰── <40> Constant Int [100000]
                    │   │       ╰── <46> Unary [-]
                    │   │           ╰── <45> Constant Double [+1e5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <57> FunctionCall [long_to_double]
                    │   │       │   ╰── <56> Unary [-]
                    │   │       │       ╰── <55> Constant Long [9007199254751227]
                    │   │       ╰── <61> Unary [-]
                    │   │           ╰── <60> Constant Double [+9.007199254751228e15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <74> Cast
                    │           ├── Target
                    │           │   ╰── Double
                    │           ╰── Expression
                    │               ╰── <73> Constant Long [1152921504606846977]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Var [d]
                    │   │       ╰── <80> Constant Double [+1.152921504606847e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [3]
                    ╰── Return
                        ╰── <87> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_unsigned_to_double() {
    let src = r#"
        
        double uint_to_double(unsigned int ui) {
            return (double) ui;
        }
        double ulong_to_double(unsigned long ul) {
            return (double) ul;
        }
        int main(void) {
            if (uint_to_double(1000u) != 1000.0) {
                return 1;
            }
            if (uint_to_double(4294967200u) != 4294967200.0) {
                return 2;
            }
            if (ulong_to_double(138512825844ul) != 138512825844.0) {
                return 3;
            }
            if (ulong_to_double(10223372036854775816ul) != 10223372036854775808.0) {
                return 4;
            }
            if (ulong_to_double(9223372036854776832ul) != 9223372036854775808.0) {
                return 5;
            }
            if (ulong_to_double(9223372036854776833ul) != 9223372036854777856.0) {
                return 6;
            }
            if (ulong_to_double(9223372036854776831ul) != 9223372036854775808.0) {
                return 7;
            }
            if (ulong_to_double(9223372036854776830ul) != 9223372036854775808.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [uint_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ui
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <12> Var [ui]
            ├── Function [ulong_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <29> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <28> Var [ul]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> FunctionCall [uint_to_double]
                    │   │       │   ╰── <39> Constant UInt [1000]
                    │   │       ╰── <42> Constant Double [+1e3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> FunctionCall [uint_to_double]
                    │   │       │   ╰── <50> Constant UInt [4294967200]
                    │   │       ╰── <53> Constant Double [+4.2949672e9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> FunctionCall [ulong_to_double]
                    │   │       │   ╰── <61> Constant ULong [138512825844]
                    │   │       ╰── <64> Constant Double [+1.38512825844e11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> FunctionCall [ulong_to_double]
                    │   │       │   ╰── <72> Constant ULong [10223372036854775816]
                    │   │       ╰── <75> Constant Double [+1.0223372036854776e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> FunctionCall [ulong_to_double]
                    │   │       │   ╰── <83> Constant ULong [9223372036854776832]
                    │   │       ╰── <86> Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <95> FunctionCall [ulong_to_double]
                    │   │       │   ╰── <94> Constant ULong [9223372036854776833]
                    │   │       ╰── <97> Constant Double [+9.223372036854778e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <99> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> FunctionCall [ulong_to_double]
                    │   │       │   ╰── <105> Constant ULong [9223372036854776831]
                    │   │       ╰── <108> Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> FunctionCall [ulong_to_double]
                    │   │       │   ╰── <116> Constant ULong [9223372036854776830]
                    │   │       ╰── <119> Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [8]
                    ╰── Return
                        ╰── <126> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign() {
    let src = r#"
        
        int main(void) {
            double d = 10.0;
            d /= 4.0;
            if (d != 2.5) {
                return 1;
            }
            d *= 10000.0;
            if (d != 25000.0) {
                return 2;
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
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1e1]
                    ├── <16> Assign [/=]
                    │   ├── <13> Var [d]
                    │   ╰── <15> Constant Double [+4e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> Var [d]
                    │   │       ╰── <21> Constant Double [+2.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── <32> Assign [*=]
                    │   ├── <29> Var [d]
                    │   ╰── <31> Constant Double [+1e4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> Var [d]
                    │   │       ╰── <37> Constant Double [+2.5e4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <39> Constant Int [2]
                    ╰── Return
                        ╰── <44> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_implicit_cast() {
    let src = r#"
        int main(void) {
            double d = 1000.5;
            d += 1000;
            if (d != 2000.5) {
                return 1;
            }
            unsigned long ul = 18446744073709551586ul;
            ul -= 1.5E19;
            if (ul != 3446744073709551616ul) {
                return 2;
            }
            int i = 10;
            i += 0.99999;
            if (i != 10) {
                return 3;
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
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1.0005e3]
                    ├── <16> Assign [+=]
                    │   ├── <13> Var [d]
                    │   ╰── <15> Constant Int [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> Var [d]
                    │   │       ╰── <21> Constant Double [+2.0005e3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <31> Constant ULong [18446744073709551586]
                    ├── <38> Assign [-=]
                    │   ├── <35> Var [ul]
                    │   ╰── <37> Constant Double [+1.5e19]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <41> Var [ul]
                    │   │       ╰── <43> Constant ULong [3446744073709551616]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <53> Constant Int [10]
                    ├── <60> Assign [+=]
                    │   ├── <57> Var [i]
                    │   ╰── <59> Constant Double [+9.9999e-1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Var [i]
                    │   │       ╰── <65> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [3]
                    ╰── Return
                        ╰── <72> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_and_decr() {
    let src = r#"
        
        int main(void) {
            static double d = 0.75;
            if (d++ != 0.75) {
                return 1;
            }
            if (d != 1.75) {
                return 2;
            }
            d = -100.2;
            if (++d != -99.2) {
                return 3;
            }
            if (d != -99.2) {
                return 4;
            }
            if (d-- != -99.2) {
                return 5;
            }
            if (d != -100.2) {
                return 6;
            }
            if (--d != -101.2) {
                return 7;
            }
            if (d != -101.2) {
                return 8;
            }
            d = 0.000000000000000000001;
            d++;
            if (d != 1.0) {
                return 9;
            }
            d = 10e20;
            d--;
            if (d != 10e20) {
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
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ├── Initializer
                    │   │   ╰── <10> Constant Double [+7.5e-1]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19>  [!=]
                    │   │       ├── <16> Postfix [++]
                    │   │       │   ╰── <14> Var [d]
                    │   │       ╰── <18> Constant Double [+7.5e-1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <20> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Var [d]
                    │   │       ╰── <28> Constant Double [+1.75e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [2]
                    ├── <41> Assign [=]
                    │   ├── <36> Var [d]
                    │   ╰── <40> Unary [-]
                    │       ╰── <39> Constant Double [+1.002e2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <46> Unary [++]
                    │   │       │   ╰── <45> Var [d]
                    │   │       ╰── <50> Unary [-]
                    │   │           ╰── <49> Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <58> Var [d]
                    │   │       ╰── <62> Unary [-]
                    │   │           ╰── <61> Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <72> Postfix [--]
                    │   │       │   ╰── <70> Var [d]
                    │   │       ╰── <76> Unary [-]
                    │   │           ╰── <75> Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <84> Var [d]
                    │   │       ╰── <88> Unary [-]
                    │   │           ╰── <87> Constant Double [+1.002e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <98> Unary [--]
                    │   │       │   ╰── <97> Var [d]
                    │   │       ╰── <102> Unary [-]
                    │   │           ╰── <101> Constant Double [+1.012e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <110> Var [d]
                    │   │       ╰── <114> Unary [-]
                    │   │           ╰── <113> Constant Double [+1.012e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [8]
                    ├── <125> Assign [=]
                    │   ├── <122> Var [d]
                    │   ╰── <124> Constant Double [+1e-21]
                    ├── <130> Postfix [++]
                    │   ╰── <128> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <133> Var [d]
                    │   │       ╰── <135> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <137> Constant Int [9]
                    ├── <146> Assign [=]
                    │   ├── <143> Var [d]
                    │   ╰── <145> Constant Double [+1e21]
                    ├── <151> Postfix [--]
                    │   ╰── <149> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157>  [!=]
                    │   │       ├── <154> Var [d]
                    │   │       ╰── <156> Constant Double [+1e21]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <158> Constant Int [10]
                    ╰── Return
                        ╰── <163> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_nan() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (nan < 0.0 || nan == 0.0 || nan > 0.0 || nan <= 0.0 || nan >= 0.0)
                return 1;
            if (1 < nan || 1 == nan || 1 > nan || 1 <= nan || 1 >= nan)
                return 2;
            if (nan == nan)
                return 3;
            if (!(nan != nan)) {
                return 4;
            }
            if (!double_isnan(nan)) {
                return 5;
            }
            if (!double_isnan(4 * nan)) {
                return 6;
            }
            if (!double_isnan(22e2 / nan)) {
                return 7;
            }
            if (!double_isnan(-nan)) {
                return 8;
            }
            if (!nan) {
                return 9;
            }
            if (nan) {
            } else {
                return 10;
            }
            int nan_is_nonzero;
            for (nan_is_nonzero = 0; nan;) {
                nan_is_nonzero = 1;
                break;
            }
            if (!nan_is_nonzero) {
                return 11;
            }
            nan_is_nonzero = 0;
            while (nan) {
                nan_is_nonzero = 1;
                break;
            }
            if (!nan_is_nonzero) {
                return 12;
            }
            nan_is_nonzero = -1;
            do {
                nan_is_nonzero = nan_is_nonzero + 1;
                if (nan_is_nonzero) {
                    break;
                }
            } while (nan);
            if (!nan_is_nonzero) {
                return 13;
            }
            nan_is_nonzero = nan ? 1 : 0;
            if (!nan_is_nonzero) {
                return 14;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [double_isnan]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Double
                    │   ├── Initializer
                    │   │   ╰── <19> Constant Double [+0e0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <29>  [/]
                    │           ├── <25> Constant Double [+0e0]
                    │           ╰── <28> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [||]
                    │   │       ├── <57>  [||]
                    │   │       │   ├── <50>  [||]
                    │   │       │   │   ├── <43>  [||]
                    │   │       │   │   │   ├── <36>  [<]
                    │   │       │   │   │   │   ├── <33> Var [nan]
                    │   │       │   │   │   │   ╰── <35> Constant Double [+0e0]
                    │   │       │   │   │   ╰── <42>  [==]
                    │   │       │   │   │       ├── <39> Var [nan]
                    │   │       │   │   │       ╰── <41> Constant Double [+0e0]
                    │   │       │   │   ╰── <49>  [>]
                    │   │       │   │       ├── <46> Var [nan]
                    │   │       │   │       ╰── <48> Constant Double [+0e0]
                    │   │       │   ╰── <56>  [<=]
                    │   │       │       ├── <53> Var [nan]
                    │   │       │       ╰── <55> Constant Double [+0e0]
                    │   │       ╰── <63>  [>=]
                    │   │           ├── <60> Var [nan]
                    │   │           ╰── <62> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <65> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [||]
                    │   │       ├── <93>  [||]
                    │   │       │   ├── <86>  [||]
                    │   │       │   │   ├── <79>  [||]
                    │   │       │   │   │   ├── <72>  [<]
                    │   │       │   │   │   │   ├── <68> Constant Int [1]
                    │   │       │   │   │   │   ╰── <71> Var [nan]
                    │   │       │   │   │   ╰── <78>  [==]
                    │   │       │   │   │       ├── <74> Constant Int [1]
                    │   │       │   │   │       ╰── <77> Var [nan]
                    │   │       │   │   ╰── <85>  [>]
                    │   │       │   │       ├── <81> Constant Int [1]
                    │   │       │   │       ╰── <84> Var [nan]
                    │   │       │   ╰── <92>  [<=]
                    │   │       │       ├── <88> Constant Int [1]
                    │   │       │       ╰── <91> Var [nan]
                    │   │       ╰── <99>  [>=]
                    │   │           ├── <95> Constant Int [1]
                    │   │           ╰── <98> Var [nan]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <101> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [==]
                    │   │       ├── <105> Var [nan]
                    │   │       ╰── <108> Var [nan]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <110> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121> Unary [!]
                    │   │       ╰── <120>  [!=]
                    │   │           ├── <115> Var [nan]
                    │   │           ╰── <118> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <122> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132> Unary [!]
                    │   │       ╰── <131> FunctionCall [double_isnan]
                    │   │           ╰── <130> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <146> Unary [!]
                    │   │       ╰── <145> FunctionCall [double_isnan]
                    │   │           ╰── <144>  [*]
                    │   │               ├── <140> Constant Int [4]
                    │   │               ╰── <143> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <147> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <160> Unary [!]
                    │   │       ╰── <159> FunctionCall [double_isnan]
                    │   │           ╰── <158>  [/]
                    │   │               ├── <154> Constant Double [+2.2e3]
                    │   │               ╰── <157> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <161> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <173> Unary [!]
                    │   │       ╰── <172> FunctionCall [double_isnan]
                    │   │           ╰── <171> Unary [-]
                    │   │               ╰── <170> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <174> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <182> Unary [!]
                    │   │       ╰── <181> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <183> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <189> Var [nan]
                    │   ├── Then
                    │   │   ╰── Block
                    │   ╰── Else
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <192> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan_is_nonzero
                    │   ╰── Type
                    │       ╰── Int
                    ├── For
                    │   ├── Init
                    │   │   ╰── <205> Assign [=]
                    │   │       ├── <202> Var [nan_is_nonzero]
                    │   │       ╰── <204> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <207> Var [nan]
                    │   ╰── Block
                    │       ├── <212> Assign [=]
                    │       │   ├── <209> Var [nan_is_nonzero]
                    │       │   ╰── <211> Constant Int [1]
                    │       ╰── Break
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <221> Unary [!]
                    │   │       ╰── <220> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <222> Constant Int [11]
                    ├── <231> Assign [=]
                    │   ├── <228> Var [nan_is_nonzero]
                    │   ╰── <230> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <234> Var [nan]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── <239> Assign [=]
                    │           │   ├── <236> Var [nan_is_nonzero]
                    │           │   ╰── <238> Constant Int [1]
                    │           ╰── Break
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <248> Unary [!]
                    │   │       ╰── <247> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <249> Constant Int [12]
                    ├── <260> Assign [=]
                    │   ├── <255> Var [nan_is_nonzero]
                    │   ╰── <259> Unary [-]
                    │       ╰── <258> Constant Int [1]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── <270> Assign [=]
                    │   │       │   ├── <263> Var [nan_is_nonzero]
                    │   │       │   ╰── <269>  [+]
                    │   │       │       ├── <266> Var [nan_is_nonzero]
                    │   │       │       ╰── <268> Constant Int [1]
                    │   │       ╰── If
                    │   │           ├── Condition
                    │   │           │   ╰── <273> Var [nan_is_nonzero]
                    │   │           ╰── Then
                    │   │               ╰── Block
                    │   │                   ╰── Break
                    │   ╰── Condition
                    │       ╰── <281> Var [nan]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <286> Unary [!]
                    │   │       ╰── <285> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <287> Constant Int [13]
                    ├── <300> Assign [=]
                    │   ├── <293> Var [nan_is_nonzero]
                    │   ╰── <299> Conditional [?]
                    │       ├── <296> Var [nan]
                    │       ├── Then
                    │       │   ╰── <297> Constant Int [1]
                    │       ╰── Else
                    │           ╰── <298> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <305> Unary [!]
                    │   │       ╰── <304> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <306> Constant Int [14]
                    ╰── Return
                        ╰── <311> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_nan_compound_assign() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (!double_isnan(nan += 99.2)) {
                return 1;
            }
            if (!double_isnan(nan -= nan)) {
                return 2;
            }
            if (!double_isnan(nan *= 4.0)) {
                return 3;
            }
            if (!double_isnan(nan /= 0.0)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [double_isnan]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Double
                    │   ├── Initializer
                    │   │   ╰── <19> Constant Double [+0e0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <29>  [/]
                    │           ├── <25> Constant Double [+0e0]
                    │           ╰── <28> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40> Unary [!]
                    │   │       ╰── <39> FunctionCall [double_isnan]
                    │   │           ╰── <38> Assign [+=]
                    │   │               ├── <35> Var [nan]
                    │   │               ╰── <37> Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55> Unary [!]
                    │   │       ╰── <54> FunctionCall [double_isnan]
                    │   │           ╰── <53> Assign [-=]
                    │   │               ├── <49> Var [nan]
                    │   │               ╰── <52> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <56> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69> Unary [!]
                    │   │       ╰── <68> FunctionCall [double_isnan]
                    │   │           ╰── <67> Assign [*=]
                    │   │               ├── <64> Var [nan]
                    │   │               ╰── <66> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <70> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83> Unary [!]
                    │   │       ╰── <82> FunctionCall [double_isnan]
                    │   │           ╰── <81> Assign [/=]
                    │   │               ├── <78> Var [nan]
                    │   │               ╰── <80> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [4]
                    ╰── Return
                        ╰── <89> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_nan_incr_and_decr() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (!double_isnan(++nan)) {
                return 1;
            }
            if (!double_isnan(--nan)) {
                return 2;
            }
            if (!double_isnan(nan++)) {
                return 3;
            }
            if (!double_isnan(nan--)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [double_isnan]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Double
                    │   ├── Initializer
                    │   │   ╰── <19> Constant Double [+0e0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <29>  [/]
                    │           ├── <25> Constant Double [+0e0]
                    │           ╰── <28> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39> Unary [!]
                    │   │       ╰── <38> FunctionCall [double_isnan]
                    │   │           ╰── <37> Unary [++]
                    │   │               ╰── <36> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52> Unary [!]
                    │   │       ╰── <51> FunctionCall [double_isnan]
                    │   │           ╰── <50> Unary [--]
                    │   │               ╰── <49> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65> Unary [!]
                    │   │       ╰── <64> FunctionCall [double_isnan]
                    │   │           ╰── <63> Postfix [++]
                    │   │               ╰── <61> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78> Unary [!]
                    │   │       ╰── <77> FunctionCall [double_isnan]
                    │   │           ╰── <76> Postfix [--]
                    │   │               ╰── <74> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [4]
                    ╰── Return
                        ╰── <84> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_floating_expressions_arithmetic_ops() {
    let src = r#"
        double point_one = 0.1;
        double point_two = 0.2;
        double point_three = 0.3;
        double two = 2.0;
        double three = 3.0;
        double four = 4.0;
        double twelveE30 = 12e30;
        int addition(void) {
            return (point_one + point_two == 0.30000000000000004);
        }
        int subtraction(void) {
            return (four - 1.0 == 3.0);
        }
        int multiplication(void) {
            return (0.01 * point_three == 0.003);
        }
        int division(void) {
            return (7.0 / two == 3.5);
        }
        int negation(void) {
            double neg = -twelveE30;
            return !(12e30 + neg);
        }
        int complex_expression(void) {
            double complex_expression = (two + three) - 127.5 * four;
            return complex_expression == -505.0;
        }
        int main(void) {
            if (!addition()) {
                return 1;
            }
            if (!subtraction()){
                return 2;
            }
            if (!multiplication()) {
                return 3;
            }
            if (!division()) {
                return 4;
            }
            if (!negation()) {
                return 5;
            }
            if (!complex_expression()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_one
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <4> Constant Double [+1e-1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_two
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <10> Constant Double [+2e-1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_three
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <16> Constant Double [+3e-1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── two
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <22> Constant Double [+2e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── three
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <28> Constant Double [+3e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── four
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <34> Constant Double [+4e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── twelveE30
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <40> Constant Double [+1.2e31]
            ├── Function [addition]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <57>  [==]
            │               ├── <53>  [+]
            │               │   ├── <49> Var [point_one]
            │               │   ╰── <52> Var [point_two]
            │               ╰── <55> Constant Double [+3.0000000000000004e-1]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <74>  [==]
            │               ├── <70>  [-]
            │               │   ├── <67> Var [four]
            │               │   ╰── <69> Constant Double [+1e0]
            │               ╰── <72> Constant Double [+3e0]
            ├── Function [multiplication]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <91>  [==]
            │               ├── <87>  [*]
            │               │   ├── <83> Constant Double [+1e-2]
            │               │   ╰── <86> Var [point_three]
            │               ╰── <89> Constant Double [+3e-3]
            ├── Function [division]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <108>  [==]
            │               ├── <104>  [/]
            │               │   ├── <100> Constant Double [+7e0]
            │               │   ╰── <103> Var [two]
            │               ╰── <106> Constant Double [+3.5e0]
            ├── Function [negation]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── neg
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <123> Unary [-]
            │       │           ╰── <122> Var [twelveE30]
            │       ╰── Return
            │           ╰── <133> Unary [!]
            │               ╰── <132>  [+]
            │                   ├── <127> Constant Double [+1.2e31]
            │                   ╰── <130> Var [neg]
            ├── Function [complex_expression]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── complex_expression
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <158>  [-]
            │       │           ├── <151>  [+]
            │       │           │   ├── <146> Var [two]
            │       │           │   ╰── <149> Var [three]
            │       │           ╰── <157>  [*]
            │       │               ├── <153> Constant Double [+1.275e2]
            │       │               ╰── <156> Var [four]
            │       ╰── Return
            │           ╰── <167>  [==]
            │               ├── <162> Var [complex_expression]
            │               ╰── <166> Unary [-]
            │                   ╰── <165> Constant Double [+5.05e2]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <179> Unary [!]
                    │   │       ╰── <178> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <180> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <188> Unary [!]
                    │   │       ╰── <187> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <189> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <197> Unary [!]
                    │   │       ╰── <196> FunctionCall [multiplication]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <198> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <206> Unary [!]
                    │   │       ╰── <205> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <207> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215> Unary [!]
                    │   │       ╰── <214> FunctionCall [negation]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <216> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <224> Unary [!]
                    │   │       ╰── <223> FunctionCall [complex_expression]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <225> Constant Int [5]
                    ╰── Return
                        ╰── <230> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_floating_expressions_comparisons() {
    let src = r#"
        double fifty_fiveE5 = 55e5;
        double fifty_fourE4 = 54e4;
        double tiny = .00004;
        double four = 4.;
        double point_one = 0.1;
        int main(void) {
            if (fifty_fiveE5 < fifty_fourE4) {
                return 1;
            }
            if (four > 4.0) {
                return 2;
            }
            if (tiny <= 0.0) {
                return 3;
            }
            if (fifty_fourE4 >= fifty_fiveE5) {
                return 4;
            }
            if (tiny == 0.0) {
                return 5;
            }
            if (point_one != point_one) {
                return 6;
            }
            if (!(tiny > 00.000005)) {
                return 7;
            }
            if (!(-.00004 < four)) {
                return 8;
            }
            if (!(tiny <= tiny)) {
                return 9;
            }
            if (!(fifty_fiveE5 >= fifty_fiveE5)) {
                return 10;
            }
            if (!(0.1 == point_one)) {
                return 11;
            }
            if (!(tiny != .00003)) {
                return 12;
            }
            if (0.00003 < 0.000000000003) {
                return 13;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── fifty_fiveE5
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <4> Constant Double [+5.5e6]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── fifty_fourE4
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <10> Constant Double [+5.4e5]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── tiny
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <16> Constant Double [+4e-5]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── four
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <22> Constant Double [+4e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_one
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <28> Constant Double [+1e-1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [<]
                    │   │       ├── <37> Var [fifty_fiveE5]
                    │   │       ╰── <40> Var [fifty_fourE4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <42> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [>]
                    │   │       ├── <48> Var [four]
                    │   │       ╰── <50> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [<=]
                    │   │       ├── <58> Var [tiny]
                    │   │       ╰── <60> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <62> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [>=]
                    │   │       ├── <68> Var [fifty_fourE4]
                    │   │       ╰── <71> Var [fifty_fiveE5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [==]
                    │   │       ├── <79> Var [tiny]
                    │   │       ╰── <81> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <89> Var [point_one]
                    │   │       ╰── <92> Var [point_one]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106> Unary [!]
                    │   │       ╰── <105>  [>]
                    │   │           ├── <101> Var [tiny]
                    │   │           ╰── <103> Constant Double [+5e-6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <107> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121> Unary [!]
                    │   │       ╰── <120>  [<]
                    │   │           ├── <115> Unary [-]
                    │   │           │   ╰── <114> Constant Double [+4e-5]
                    │   │           ╰── <118> Var [four]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <122> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135> Unary [!]
                    │   │       ╰── <134>  [<=]
                    │   │           ├── <129> Var [tiny]
                    │   │           ╰── <132> Var [tiny]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <136> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149> Unary [!]
                    │   │       ╰── <148>  [>=]
                    │   │           ├── <143> Var [fifty_fiveE5]
                    │   │           ╰── <146> Var [fifty_fiveE5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <150> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162> Unary [!]
                    │   │       ╰── <161>  [==]
                    │   │           ├── <156> Constant Double [+1e-1]
                    │   │           ╰── <159> Var [point_one]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <163> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <175> Unary [!]
                    │   │       ╰── <174>  [!=]
                    │   │           ├── <170> Var [tiny]
                    │   │           ╰── <172> Constant Double [+3e-5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <184>  [<]
                    │   │       ├── <181> Constant Double [+3e-5]
                    │   │       ╰── <183> Constant Double [+3e-12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <185> Constant Int [13]
                    ╰── Return
                        ╰── <190> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_floating_expressions_logical() {
    let src = r#"
        double zero = 0.0;
        double non_zero = 1E-20;
        double one = 1.0;
        double rounded_to_zero = 1e-330;
        int main(void) {
            if (zero) {
                return 1;
            }
            if (rounded_to_zero) {
                return 2;
            }
            if (non_zero) {
            } else {
                return 3;
            }
            if (0.e10) {
                return 4;
            }
            if (!non_zero) {
                return 4;
            }
            if (!(!zero)) {
                return 5;
            }
            if (!(!rounded_to_zero)) {
                return 6;
            }
            if (!(non_zero && 1.0)) {
                return 8;
            }
            if (3.0 && zero) {
                return 8;
            }
            if (rounded_to_zero && 1000e10) {
                return 9;
            }
            if (18446744073709551615UL && zero) {
                return 10;
            }
            if (!(non_zero && 5l)) {
                return 11;
            }
            if (!(5.0 || zero)) {
                return 12;
            }
            if (zero || rounded_to_zero) {
                return 13;
            }
            if (!(rounded_to_zero || 0.0001)) {
                return 14;
            }
            if (!(non_zero || 0u)) {
                return 15;
            }
            if (!(0 || 0.0000005)) {
                return 16;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <4> Constant Double [+0e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── non_zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <10> Constant Double [+1e-20]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <16> Constant Double [+1e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── rounded_to_zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <22> Constant Double [+0e0]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38> Var [rounded_to_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <39> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45> Var [non_zero]
                    │   ├── Then
                    │   │   ╰── Block
                    │   ╰── Else
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62> Unary [!]
                    │   │       ╰── <61> Var [non_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74> Unary [!]
                    │   │       ╰── <73> Unary [!]
                    │   │           ╰── <71> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <75> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86> Unary [!]
                    │   │       ╰── <85> Unary [!]
                    │   │           ╰── <83> Var [rounded_to_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <87> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99> Unary [!]
                    │   │       ╰── <98>  [&&]
                    │   │           ├── <94> Var [non_zero]
                    │   │           ╰── <96> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <100> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [&&]
                    │   │       ├── <105> Constant Double [+3e0]
                    │   │       ╰── <108> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [&&]
                    │   │       ├── <116> Var [rounded_to_zero]
                    │   │       ╰── <118> Constant Double [+1e13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <120> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <129>  [&&]
                    │   │       ├── <125> Constant ULong [18446744073709551615]
                    │   │       ╰── <128> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <130> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142> Unary [!]
                    │   │       ╰── <141>  [&&]
                    │   │           ├── <137> Var [non_zero]
                    │   │           ╰── <139> Constant Long [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <155> Unary [!]
                    │   │       ╰── <154>  [||]
                    │   │           ├── <149> Constant Double [+5e0]
                    │   │           ╰── <152> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <156> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166>  [||]
                    │   │       ├── <162> Var [zero]
                    │   │       ╰── <165> Var [rounded_to_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <167> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <179> Unary [!]
                    │   │       ╰── <178>  [||]
                    │   │           ├── <174> Var [rounded_to_zero]
                    │   │           ╰── <176> Constant Double [+1e-4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <180> Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192> Unary [!]
                    │   │       ╰── <191>  [||]
                    │   │           ├── <187> Var [non_zero]
                    │   │           ╰── <189> Constant UInt [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <193> Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <204> Unary [!]
                    │   │       ╰── <203>  [||]
                    │   │           ├── <199> Constant Int [0]
                    │   │           ╰── <201> Constant Double [+5e-7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <205> Constant Int [16]
                    ╰── Return
                        ╰── <210> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_floating_expressions_loop_controlling_expression() {
    let src = r#"
        int main(void) {
            int a = 0;
            for(double d = 100.0; d > 0.0; d = d - 1.0) {
                a = a + 1;
            }
            return a;
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
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── d
                    │   │       ├── Type
                    │   │       │   ╰── Double
                    │   │       ╰── Initializer
                    │   │           ╰── <15> Constant Double [+1e2]
                    │   ├── Condition
                    │   │   ╰── <23>  [>]
                    │   │       ├── <20> Var [d]
                    │   │       ╰── <22> Constant Double [+0e0]
                    │   ├── Condition
                    │   │   ╰── <32> Assign [=]
                    │   │       ├── <25> Var [d]
                    │   │       ╰── <31>  [-]
                    │   │           ├── <28> Var [d]
                    │   │           ╰── <30> Constant Double [+1e0]
                    │   ╰── Block
                    │       ╰── <41> Assign [=]
                    │           ├── <34> Var [a]
                    │           ╰── <40>  [+]
                    │               ├── <37> Var [a]
                    │               ╰── <39> Constant Int [1]
                    ╰── Return
                        ╰── <47> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_floating_expressions_simple() {
    let src = r#"
        
        int main(void) {
            double x = 2.0;
            return (x * 2.0 == 4.0);
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
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+2e0]
                    ╰── Return
                        ╰── <20>  [==]
                            ├── <16>  [*]
                            │   ├── <13> Var [x]
                            │   ╰── <15> Constant Double [+2e0]
                            ╰── <18> Constant Double [+4e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_floating_expressions_static_initialized_double() {
    let src = r#"
        double return_static_variable(void) {
            static double d = 0.5;
            double ret = d;
            d = d + 1.0;
            return ret;
        }
        int main(void) {
            double d1 = return_static_variable();
            double d2 = return_static_variable();
            double d3 = return_static_variable();
            if (d1 != 0.5) {
                return 1;
            }
            if (d2 != 1.5) {
                return 2;
            }
            if (d3 != 2.5) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_static_variable]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ├── Initializer
            │       │   │   ╰── <10> Constant Double [+5e-1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ret
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <17> Var [d]
            │       ├── <28> Assign [=]
            │       │   ├── <21> Var [d]
            │       │   ╰── <27>  [+]
            │       │       ├── <24> Var [d]
            │       │       ╰── <26> Constant Double [+1e0]
            │       ╰── Return
            │           ╰── <31> Var [ret]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d1
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <44> FunctionCall [return_static_variable]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <51> FunctionCall [return_static_variable]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d3
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <58> FunctionCall [return_static_variable]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> Var [d1]
                    │   │       ╰── <64> Constant Double [+5e-1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Var [d2]
                    │   │       ╰── <74> Constant Double [+1.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> Var [d3]
                    │   │       ╰── <84> Constant Double [+2.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ╰── Return
                        ╰── <91> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_double_and_int_parameters() {
    let src = r#"
        int check_arguments(double d1, double d2, int i1, double d3, double d4, int i2, int i3,
                            int i4, double d5, double d6, double d7, int i5, double d8) {
            if (d1 != 1.0) {
                return 1;
            }
            if (d2 != 2.0) {
                return 2;
            }
            if (d3 != 3.0) {
                return 3;
            }
            if (d4 != 4.0 ){
                return 4;
            }
            if (d5 != 5.0){
                return 5;
            }
            if (d6 != 6.0 ){
                return 6;
            }
            if (d7 != 7.0 ){
                return 7;
            }
            if (d8 != 8.0 ){
                return 8;
            }
            if (i1 != 101 ){
                return 9;
            }
            if (i2 != 102 ){
                return 10;
            }
            if (i3 != 103){
                return 11;
            }
            if (i4 != 104) {
                return 12;
            }
            if (i5 != 105) {
                return 13;
            }
            return 0;
        }
        int main(void) {
            return check_arguments(1.0, 2.0, 101, 3.0, 4.0, 102, 103, 104, 5.0, 6.0, 7.0, 105, 8.0);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_arguments]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d1
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d2
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i1
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d3
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d4
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i2
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i3
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i4
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d5
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d6
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d7
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i5
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d8
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <49>  [!=]
            │       │   │       ├── <46> Var [d1]
            │       │   │       ╰── <48> Constant Double [+1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <50> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <59>  [!=]
            │       │   │       ├── <56> Var [d2]
            │       │   │       ╰── <58> Constant Double [+2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <60> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <69>  [!=]
            │       │   │       ├── <66> Var [d3]
            │       │   │       ╰── <68> Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <70> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <79>  [!=]
            │       │   │       ├── <76> Var [d4]
            │       │   │       ╰── <78> Constant Double [+4e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <80> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <89>  [!=]
            │       │   │       ├── <86> Var [d5]
            │       │   │       ╰── <88> Constant Double [+5e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <90> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <99>  [!=]
            │       │   │       ├── <96> Var [d6]
            │       │   │       ╰── <98> Constant Double [+6e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <100> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <109>  [!=]
            │       │   │       ├── <106> Var [d7]
            │       │   │       ╰── <108> Constant Double [+7e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <110> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <119>  [!=]
            │       │   │       ├── <116> Var [d8]
            │       │   │       ╰── <118> Constant Double [+8e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <120> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <129>  [!=]
            │       │   │       ├── <126> Var [i1]
            │       │   │       ╰── <128> Constant Int [101]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <130> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <139>  [!=]
            │       │   │       ├── <136> Var [i2]
            │       │   │       ╰── <138> Constant Int [102]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <140> Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <149>  [!=]
            │       │   │       ├── <146> Var [i3]
            │       │   │       ╰── <148> Constant Int [103]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <150> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <159>  [!=]
            │       │   │       ├── <156> Var [i4]
            │       │   │       ╰── <158> Constant Int [104]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <160> Constant Int [12]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <169>  [!=]
            │       │   │       ├── <166> Var [i5]
            │       │   │       ╰── <168> Constant Int [105]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <170> Constant Int [13]
            │       ╰── Return
            │           ╰── <175> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <198> FunctionCall [check_arguments]
                            ├── <185> Constant Double [+1e0]
                            ├── <186> Constant Double [+2e0]
                            ├── <187> Constant Int [101]
                            ├── <188> Constant Double [+3e0]
                            ├── <189> Constant Double [+4e0]
                            ├── <190> Constant Int [102]
                            ├── <191> Constant Int [103]
                            ├── <192> Constant Int [104]
                            ├── <193> Constant Double [+5e0]
                            ├── <194> Constant Double [+6e0]
                            ├── <195> Constant Double [+7e0]
                            ├── <196> Constant Int [105]
                            ╰── <197> Constant Double [+8e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_double_and_int_params_recursive() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9) {
            if (i1 != d9) {
                int call1 = fun(i1 + 1, d1, i2 + 1, d2, i3 + 1, d3, i4 + 1, d4, i5 + 1, d5, i6 + 1, d6, i7 + 1, d7, i8 + 1, d8, i9 + 1, d9);
                int call2 = fun(i1, d1 - 1, i2, d2 - 1, i3, d3 - 1, i4, d4 - 1, i5, d5 - 1, i6, d6 - 1, i7, d7 - 1, i8, d8 - 1, i9, d9 - 1);
                if (call1) {
                    return call1;
                }
                if (call2) {
                    return call2;
                }
            }
            if (i2 != i1 + 2) {
                return 2;
            }
            if (i3 != i1 + 4) {
                return 3;
            }
            if (i4 != i1 + 6) {
                return 4;
            }
            if (i5 != i1 + 8) {
                return 5;
            }
            if (i6 != i1 + 10) {
                return 6;
            }
            if (i7 != i1 + 12) {
                return 7;
            }
            if (i8 != i1 + 14) {
                return 8;
            }
            if (i9 != i1 + 16) {
                return 9;
            }
            if (d1 != d9 - 16) {
                return 11;
            }
            if (d2 != d9 - 14) {
                return 12;
            }
            if (d3 != d9 - 12) {
                return 13;
            }
            if (d4 != d9 - 10) {
                return 14;
            }
            if (d5 != d9 - 8) {
                return 15;
            }
            if (d6 != d9 - 6) {
                return 16;
            }
            if (d7 != d9 - 4) {
                return 17;
            }
            if (d8 != d9 - 2) {
                return 18;
            }
            return 0;
        }
        int main(void) {
            return fun(1, 2.0, 3, 4.0, 5, 6.0, 7, 8.0, 9, 10.0, 11, 12.0, 13, 14.0, 15, 16.0, 17, 18.0);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fun]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i1
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d1
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i2
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d2
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i3
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d3
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i4
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d4
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i5
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d5
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i6
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d6
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i7
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d7
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i8
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d8
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i9
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d9
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <65>  [!=]
            │       │   │       ├── <61> Var [i1]
            │       │   │       ╰── <64> Var [d9]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── call1
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Initializer
            │       │           │       ╰── <133> FunctionCall [fun]
            │       │           │           ├── <74>  [+]
            │       │           │           │   ├── <71> Var [i1]
            │       │           │           │   ╰── <73> Constant Int [1]
            │       │           │           ├── <76> Var [d1]
            │       │           │           ├── <81>  [+]
            │       │           │           │   ├── <78> Var [i2]
            │       │           │           │   ╰── <80> Constant Int [1]
            │       │           │           ├── <83> Var [d2]
            │       │           │           ├── <88>  [+]
            │       │           │           │   ├── <85> Var [i3]
            │       │           │           │   ╰── <87> Constant Int [1]
            │       │           │           ├── <90> Var [d3]
            │       │           │           ├── <95>  [+]
            │       │           │           │   ├── <92> Var [i4]
            │       │           │           │   ╰── <94> Constant Int [1]
            │       │           │           ├── <97> Var [d4]
            │       │           │           ├── <102>  [+]
            │       │           │           │   ├── <99> Var [i5]
            │       │           │           │   ╰── <101> Constant Int [1]
            │       │           │           ├── <104> Var [d5]
            │       │           │           ├── <109>  [+]
            │       │           │           │   ├── <106> Var [i6]
            │       │           │           │   ╰── <108> Constant Int [1]
            │       │           │           ├── <111> Var [d6]
            │       │           │           ├── <116>  [+]
            │       │           │           │   ├── <113> Var [i7]
            │       │           │           │   ╰── <115> Constant Int [1]
            │       │           │           ├── <118> Var [d7]
            │       │           │           ├── <123>  [+]
            │       │           │           │   ├── <120> Var [i8]
            │       │           │           │   ╰── <122> Constant Int [1]
            │       │           │           ├── <125> Var [d8]
            │       │           │           ├── <130>  [+]
            │       │           │           │   ├── <127> Var [i9]
            │       │           │           │   ╰── <129> Constant Int [1]
            │       │           │           ╰── <132> Var [d9]
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── call2
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Initializer
            │       │           │       ╰── <203> FunctionCall [fun]
            │       │           │           ├── <141> Var [i1]
            │       │           │           ├── <146>  [-]
            │       │           │           │   ├── <143> Var [d1]
            │       │           │           │   ╰── <145> Constant Int [1]
            │       │           │           ├── <148> Var [i2]
            │       │           │           ├── <153>  [-]
            │       │           │           │   ├── <150> Var [d2]
            │       │           │           │   ╰── <152> Constant Int [1]
            │       │           │           ├── <155> Var [i3]
            │       │           │           ├── <160>  [-]
            │       │           │           │   ├── <157> Var [d3]
            │       │           │           │   ╰── <159> Constant Int [1]
            │       │           │           ├── <162> Var [i4]
            │       │           │           ├── <167>  [-]
            │       │           │           │   ├── <164> Var [d4]
            │       │           │           │   ╰── <166> Constant Int [1]
            │       │           │           ├── <169> Var [i5]
            │       │           │           ├── <174>  [-]
            │       │           │           │   ├── <171> Var [d5]
            │       │           │           │   ╰── <173> Constant Int [1]
            │       │           │           ├── <176> Var [i6]
            │       │           │           ├── <181>  [-]
            │       │           │           │   ├── <178> Var [d6]
            │       │           │           │   ╰── <180> Constant Int [1]
            │       │           │           ├── <183> Var [i7]
            │       │           │           ├── <188>  [-]
            │       │           │           │   ├── <185> Var [d7]
            │       │           │           │   ╰── <187> Constant Int [1]
            │       │           │           ├── <190> Var [i8]
            │       │           │           ├── <195>  [-]
            │       │           │           │   ├── <192> Var [d8]
            │       │           │           │   ╰── <194> Constant Int [1]
            │       │           │           ├── <197> Var [i9]
            │       │           │           ╰── <202>  [-]
            │       │           │               ├── <199> Var [d9]
            │       │           │               ╰── <201> Constant Int [1]
            │       │           ├── If
            │       │           │   ├── Condition
            │       │           │   │   ╰── <207> Var [call1]
            │       │           │   ╰── Then
            │       │           │       ╰── Block
            │       │           │           ╰── Return
            │       │           │               ╰── <209> Var [call1]
            │       │           ╰── If
            │       │               ├── Condition
            │       │               │   ╰── <215> Var [call2]
            │       │               ╰── Then
            │       │                   ╰── Block
            │       │                       ╰── Return
            │       │                           ╰── <217> Var [call2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <233>  [!=]
            │       │   │       ├── <226> Var [i2]
            │       │   │       ╰── <232>  [+]
            │       │   │           ├── <229> Var [i1]
            │       │   │           ╰── <231> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <234> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <247>  [!=]
            │       │   │       ├── <240> Var [i3]
            │       │   │       ╰── <246>  [+]
            │       │   │           ├── <243> Var [i1]
            │       │   │           ╰── <245> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <248> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <261>  [!=]
            │       │   │       ├── <254> Var [i4]
            │       │   │       ╰── <260>  [+]
            │       │   │           ├── <257> Var [i1]
            │       │   │           ╰── <259> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <262> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <275>  [!=]
            │       │   │       ├── <268> Var [i5]
            │       │   │       ╰── <274>  [+]
            │       │   │           ├── <271> Var [i1]
            │       │   │           ╰── <273> Constant Int [8]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <276> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <289>  [!=]
            │       │   │       ├── <282> Var [i6]
            │       │   │       ╰── <288>  [+]
            │       │   │           ├── <285> Var [i1]
            │       │   │           ╰── <287> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <290> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <303>  [!=]
            │       │   │       ├── <296> Var [i7]
            │       │   │       ╰── <302>  [+]
            │       │   │           ├── <299> Var [i1]
            │       │   │           ╰── <301> Constant Int [12]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <304> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <317>  [!=]
            │       │   │       ├── <310> Var [i8]
            │       │   │       ╰── <316>  [+]
            │       │   │           ├── <313> Var [i1]
            │       │   │           ╰── <315> Constant Int [14]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <318> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <331>  [!=]
            │       │   │       ├── <324> Var [i9]
            │       │   │       ╰── <330>  [+]
            │       │   │           ├── <327> Var [i1]
            │       │   │           ╰── <329> Constant Int [16]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <332> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <345>  [!=]
            │       │   │       ├── <338> Var [d1]
            │       │   │       ╰── <344>  [-]
            │       │   │           ├── <341> Var [d9]
            │       │   │           ╰── <343> Constant Int [16]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <346> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <359>  [!=]
            │       │   │       ├── <352> Var [d2]
            │       │   │       ╰── <358>  [-]
            │       │   │           ├── <355> Var [d9]
            │       │   │           ╰── <357> Constant Int [14]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <360> Constant Int [12]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <373>  [!=]
            │       │   │       ├── <366> Var [d3]
            │       │   │       ╰── <372>  [-]
            │       │   │           ├── <369> Var [d9]
            │       │   │           ╰── <371> Constant Int [12]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <374> Constant Int [13]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <387>  [!=]
            │       │   │       ├── <380> Var [d4]
            │       │   │       ╰── <386>  [-]
            │       │   │           ├── <383> Var [d9]
            │       │   │           ╰── <385> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <388> Constant Int [14]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <401>  [!=]
            │       │   │       ├── <394> Var [d5]
            │       │   │       ╰── <400>  [-]
            │       │   │           ├── <397> Var [d9]
            │       │   │           ╰── <399> Constant Int [8]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <402> Constant Int [15]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <415>  [!=]
            │       │   │       ├── <408> Var [d6]
            │       │   │       ╰── <414>  [-]
            │       │   │           ├── <411> Var [d9]
            │       │   │           ╰── <413> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <416> Constant Int [16]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <429>  [!=]
            │       │   │       ├── <422> Var [d7]
            │       │   │       ╰── <428>  [-]
            │       │   │           ├── <425> Var [d9]
            │       │   │           ╰── <427> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <430> Constant Int [17]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <443>  [!=]
            │       │   │       ├── <436> Var [d8]
            │       │   │       ╰── <442>  [-]
            │       │   │           ├── <439> Var [d9]
            │       │   │           ╰── <441> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <444> Constant Int [18]
            │       ╰── Return
            │           ╰── <449> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <477> FunctionCall [fun]
                            ├── <459> Constant Int [1]
                            ├── <460> Constant Double [+2e0]
                            ├── <461> Constant Int [3]
                            ├── <462> Constant Double [+4e0]
                            ├── <463> Constant Int [5]
                            ├── <464> Constant Double [+6e0]
                            ├── <465> Constant Int [7]
                            ├── <466> Constant Double [+8e0]
                            ├── <467> Constant Int [9]
                            ├── <468> Constant Double [+1e1]
                            ├── <469> Constant Int [11]
                            ├── <470> Constant Double [+1.2e1]
                            ├── <471> Constant Int [13]
                            ├── <472> Constant Double [+1.4e1]
                            ├── <473> Constant Int [15]
                            ├── <474> Constant Double [+1.6e1]
                            ├── <475> Constant Int [17]
                            ╰── <476> Constant Double [+1.8e1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_double_parameters() {
    let src = r#"
        
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h);
        int main(void) {
            return check_arguments(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        }
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h) {
            if (a != 1.0) {
                return 1;
            }
            if (b != 2.0) {
                return 2;
            }
            if (c != 3.0) {
                return 3;
            }
            if (d != 4.0) {
                return 4;
            }
            if (e != -1.0) {
                return 5;
            }
            if (f != -2.0) {
                return 6;
            }
            if (g != -3.0) {
                return 7;
            }
            if (h != -4.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_arguments]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── h
            │           ╰── Type
            │               ╰── Double
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <53> FunctionCall [check_arguments]
            │               ├── <37> Constant Double [+1e0]
            │               ├── <38> Constant Double [+2e0]
            │               ├── <39> Constant Double [+3e0]
            │               ├── <40> Constant Double [+4e0]
            │               ├── <43> Unary [-]
            │               │   ╰── <42> Constant Double [+1e0]
            │               ├── <46> Unary [-]
            │               │   ╰── <45> Constant Double [+2e0]
            │               ├── <49> Unary [-]
            │               │   ╰── <48> Constant Double [+3e0]
            │               ╰── <52> Unary [-]
            │                   ╰── <51> Constant Double [+4e0]
            ╰── Function [check_arguments]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Double
                │   ╰── Param
                │       ├── Name
                │       │   ╰── h
                │       ╰── Type
                │           ╰── Double
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [a]
                    │   │       ╰── <89> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <91> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [b]
                    │   │       ╰── <99> Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [!=]
                    │   │       ├── <107> Var [c]
                    │   │       ╰── <109> Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Var [d]
                    │   │       ╰── <119> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <127> Var [e]
                    │   │       ╰── <131> Unary [-]
                    │   │           ╰── <130> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <139> Var [f]
                    │   │       ╰── <143> Unary [-]
                    │   │           ╰── <142> Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <145> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <151> Var [g]
                    │   │       ╰── <155> Unary [-]
                    │   │           ╰── <154> Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <157> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168>  [!=]
                    │   │       ├── <163> Var [h]
                    │   │       ╰── <167> Unary [-]
                    │   │           ╰── <166> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <169> Constant Int [8]
                    ╰── Return
                        ╰── <174> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_push_xmm() {
    let src = r#"
        int callee(double a, double b, double c, double d, double e, double f, double g,
                   double h, double i, double j, double k) {
            if (a != 0.) {
                return 1;
            }
            if (b != 1.) {
                return 2;
            }
            if (c != 2.) {
                return 3;
            }
            if (d != 3.) {
                return 4;
            }
            if (e != 4.) {
                return 5;
            }
            if (f != 5.) {
                return 6;
            }
            if (g != 6.) {
                return 7;
            }
            if (h != 7.) {
                return 8;
            }
            if (i != 8.) {
                return 9;
            }
            if (j != 9.) {
                return 10;
            }
            if (k != 10.) {
                return 11;
            }
            return 0;
        }
        int target(int a, int b, int c, int d, int e) {
            return callee(0., 1., 2., 3., 4., 5., e + 1., d + 3., c + 5., b + 7.,
                          a + 9.);
        }
        int main(void) {
            return target(1, 2, 3, 4, 5);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [callee]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── h
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── k
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <43>  [!=]
            │       │   │       ├── <40> Var [a]
            │       │   │       ╰── <42> Constant Double [+0e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <44> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <53>  [!=]
            │       │   │       ├── <50> Var [b]
            │       │   │       ╰── <52> Constant Double [+1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <54> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <63>  [!=]
            │       │   │       ├── <60> Var [c]
            │       │   │       ╰── <62> Constant Double [+2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <64> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <73>  [!=]
            │       │   │       ├── <70> Var [d]
            │       │   │       ╰── <72> Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <74> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <83>  [!=]
            │       │   │       ├── <80> Var [e]
            │       │   │       ╰── <82> Constant Double [+4e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <84> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <93>  [!=]
            │       │   │       ├── <90> Var [f]
            │       │   │       ╰── <92> Constant Double [+5e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <94> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <103>  [!=]
            │       │   │       ├── <100> Var [g]
            │       │   │       ╰── <102> Constant Double [+6e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <104> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <113>  [!=]
            │       │   │       ├── <110> Var [h]
            │       │   │       ╰── <112> Constant Double [+7e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <114> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <123>  [!=]
            │       │   │       ├── <120> Var [i]
            │       │   │       ╰── <122> Constant Double [+8e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <124> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <133>  [!=]
            │       │   │       ├── <130> Var [j]
            │       │   │       ╰── <132> Constant Double [+9e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <134> Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <143>  [!=]
            │       │   │       ├── <140> Var [k]
            │       │   │       ╰── <142> Constant Double [+1e1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <144> Constant Int [11]
            │       ╰── Return
            │           ╰── <149> Constant Int [0]
            ├── Function [target]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
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
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── e
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <205> FunctionCall [callee]
            │               ├── <174> Constant Double [+0e0]
            │               ├── <175> Constant Double [+1e0]
            │               ├── <176> Constant Double [+2e0]
            │               ├── <177> Constant Double [+3e0]
            │               ├── <178> Constant Double [+4e0]
            │               ├── <179> Constant Double [+5e0]
            │               ├── <184>  [+]
            │               │   ├── <181> Var [e]
            │               │   ╰── <183> Constant Double [+1e0]
            │               ├── <189>  [+]
            │               │   ├── <186> Var [d]
            │               │   ╰── <188> Constant Double [+3e0]
            │               ├── <194>  [+]
            │               │   ├── <191> Var [c]
            │               │   ╰── <193> Constant Double [+5e0]
            │               ├── <199>  [+]
            │               │   ├── <196> Var [b]
            │               │   ╰── <198> Constant Double [+7e0]
            │               ╰── <204>  [+]
            │                   ├── <201> Var [a]
            │                   ╰── <203> Constant Double [+9e0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <220> FunctionCall [target]
                            ├── <215> Constant Int [1]
                            ├── <216> Constant Int [2]
                            ├── <217> Constant Int [3]
                            ├── <218> Constant Int [4]
                            ╰── <219> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_return_double() {
    let src = r#"
        
        double d(void) {
            return 1234.e75;
        }
        int main(void) {
            double retval = d();
            return retval == 1234.e75;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [d]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <6> Constant Double [+1.234e78]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <19> FunctionCall [d]
                    ╰── Return
                        ╰── <26>  [==]
                            ├── <23> Var [retval]
                            ╰── <25> Constant Double [+1.234e78]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_standard_library_call() {
    let src = r#"
        double fma(double x, double y, double z);
        double ldexp(double x, int exp);
        int main(void) {
            double fma_result = fma(5.0, 1E22, 4000000.0);
            double ldexp_result = ldexp(92E73, 5);
            if (fma_result != 50000000000000004194304.0) {
                return 1;
            }
            if (ldexp_result != 2.944E76) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fma]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── y
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── z
            │           ╰── Type
            │               ╰── Double
            ├── Function [ldexp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── exp
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── fma_result
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <40> FunctionCall [fma]
                    │           ├── <37> Constant Double [+5e0]
                    │           ├── <38> Constant Double [+1e22]
                    │           ╰── <39> Constant Double [+4e6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ldexp_result
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <49> FunctionCall [ldexp]
                    │           ├── <47> Constant Double [+9.2e74]
                    │           ╰── <48> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Var [fma_result]
                    │   │       ╰── <55> Constant Double [+5.0000000000000004e22]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <57> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Var [ldexp_result]
                    │   │       ╰── <65> Constant Double [+2.944e76]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [2]
                    ╰── Return
                        ╰── <72> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_use_arg_after_fun_call() {
    let src = r#"
        double fun(double x) {
            if (x > 2)
                return x;
            else {
                double ret = fun(x + 2);
                return ret + x;
            }
        }
        int main(void) {
            return fun(1.0);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fun]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── x
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── If
            │           ├── Condition
            │           │   ╰── <13>  [>]
            │           │       ├── <10> Var [x]
            │           │       ╰── <12> Constant Int [2]
            │           ├── Then
            │           │   ╰── Return
            │           │       ╰── <15> Var [x]
            │           ╰── Else
            │               ╰── Block
            │                   ├── VarDeclaration
            │                   │   ├── Name
            │                   │   │   ╰── ret
            │                   │   ├── Type
            │                   │   │   ╰── Double
            │                   │   ╰── Initializer
            │                   │       ╰── <26> FunctionCall [fun]
            │                   │           ╰── <25>  [+]
            │                   │               ├── <22> Var [x]
            │                   │               ╰── <24> Constant Int [2]
            │                   ╰── Return
            │                       ╰── <34>  [+]
            │                           ├── <30> Var [ret]
            │                           ╰── <33> Var [x]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <48> FunctionCall [fun]
                            ╰── <47> Constant Double [+1e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_common_type() {
    let src = r#"
        int lt(double d, long l) {
            return d < l;
        }
        double tern_double_flag(double flag) {
            return (double) (flag ? -30 : 10ul);
        }
        double tern_double_result(int flag) {
            return flag ? 5.0 : 9223372036854777850ul;
        }
        int ten = 10;
        int multiply(void) {
            int i = 10.75 * ten;
            return i == 107;
        }
        int main(void) {
            if (lt(-9007199254751228.0, -9007199254751227l)) {
                return 1;
            }
            if (tern_double_flag(20.0) != 18446744073709551586.0) {
                return 2;
            }
            if (tern_double_flag(0.0) != 10.0) {
                return 3;
            }
            if (tern_double_result(1) != 5.0) {
                return 4;
            }
            if (tern_double_result(0) != 9223372036854777856.0) {
                return 5;
            }
            if (!multiply()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [lt]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <17>  [<]
            │               ├── <13> Var [d]
            │               ╰── <16> Var [l]
            ├── Function [tern_double_flag]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── flag
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <39> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <38> Conditional [?]
            │                       ├── <32> Var [flag]
            │                       ├── Then
            │                       │   ╰── <35> Unary [-]
            │                       │       ╰── <34> Constant Int [30]
            │                       ╰── Else
            │                           ╰── <36> Constant ULong [10]
            ├── Function [tern_double_result]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── flag
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <55> Conditional [?]
            │               ├── <52> Var [flag]
            │               ├── Then
            │               │   ╰── <53> Constant Double [+5e0]
            │               ╰── Else
            │                   ╰── <54> Constant ULong [9223372036854777850]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ten
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <62> Constant Int [10]
            ├── Function [multiply]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <77>  [*]
            │       │           ├── <73> Constant Double [+1.075e1]
            │       │           ╰── <76> Var [ten]
            │       ╰── Return
            │           ╰── <84>  [==]
            │               ├── <81> Var [i]
            │               ╰── <83> Constant Int [107]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100> FunctionCall [lt]
                    │   │       ├── <96> Unary [-]
                    │   │       │   ╰── <95> Constant Double [+9.007199254751228e15]
                    │   │       ╰── <99> Unary [-]
                    │   │           ╰── <98> Constant Long [9007199254751227]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <108> FunctionCall [tern_double_flag]
                    │   │       │   ╰── <107> Constant Double [+2e1]
                    │   │       ╰── <110> Constant Double [+1.8446744073709552e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <112> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <119> FunctionCall [tern_double_flag]
                    │   │       │   ╰── <118> Constant Double [+0e0]
                    │   │       ╰── <121> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <123> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <130> FunctionCall [tern_double_result]
                    │   │       │   ╰── <129> Constant Int [1]
                    │   │       ╰── <132> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <141> FunctionCall [tern_double_result]
                    │   │       │   ╰── <140> Constant Int [0]
                    │   │       ╰── <143> Constant Double [+9.223372036854778e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <145> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153> Unary [!]
                    │   │       ╰── <152> FunctionCall [multiply]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [6]
                    ╰── Return
                        ╰── <159> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_complex_arithmetic_common_type() {
    let src = r#"
        unsigned long ul = 10000ul;
        int main(void) {
            int i = -50;
            double d = (ul + i) * 3.125;
            return d == 31093.75;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <4> Constant ULong [10000]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Unary [-]
                    │           ╰── <16> Constant Int [50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <32>  [*]
                    │           ├── <29>  [+]
                    │           │   ├── <24> Var [ul]
                    │           │   ╰── <27> Var [i]
                    │           ╰── <31> Constant Double [+3.125e0]
                    ╰── Return
                        ╰── <39>  [==]
                            ├── <36> Var [d]
                            ╰── <38> Constant Double [+3.109375e4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_convert_for_assignment() {
    let src = r#"
        int check_args(long l, double d) {
            return l == 2 && d == -6.0;
        }
        double return_double(void) {
            return 18446744073709551586ul;
        }
        int check_assignment(double arg) {
            int i = 0;
            i = arg;
            return i == 4;
        }
        int main(void) {
            if (!check_args(2.4, -6)) {
                return 1;
            }
            if (return_double() != 18446744073709551616.0) {
                return 2;
            }
            if (!check_assignment(4.9)) {
                return 3;
            }
            double d = 18446744073709551586ul;
            if (d != 18446744073709551616.) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_args]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <25>  [&&]
            │               ├── <16>  [==]
            │               │   ├── <13> Var [l]
            │               │   ╰── <15> Constant Int [2]
            │               ╰── <24>  [==]
            │                   ├── <19> Var [d]
            │                   ╰── <23> Unary [-]
            │                       ╰── <22> Constant Double [+6e0]
            ├── Function [return_double]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <34> Constant ULong [18446744073709551586]
            ├── Function [check_assignment]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arg
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <49> Constant Int [0]
            │       ├── <57> Assign [=]
            │       │   ├── <53> Var [i]
            │       │   ╰── <56> Var [arg]
            │       ╰── Return
            │           ╰── <63>  [==]
            │               ├── <60> Var [i]
            │               ╰── <62> Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79> Unary [!]
                    │   │       ╰── <78> FunctionCall [check_args]
                    │   │           ├── <74> Constant Double [+2.4e0]
                    │   │           ╰── <77> Unary [-]
                    │   │               ╰── <76> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <80> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <86> FunctionCall [return_double]
                    │   │       ╰── <88> Constant Double [+1.8446744073709552e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99> Unary [!]
                    │   │       ╰── <98> FunctionCall [check_assignment]
                    │   │           ╰── <97> Constant Double [+4.9e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <100> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <108> Constant ULong [18446744073709551586]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <112> Var [d]
                    │   │       ╰── <114> Constant Double [+1.8446744073709552e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [4]
                    ╰── Return
                        ╰── <121> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_static_initializers() {
    let src = r#"
        double d1 = 2147483647;
        double d2 = 4294967295u;
        double d3 = 4611686018427389440l;
        double d4 = 4611686018427389955l;
        double d5 = 9223372036854775810ul;
        double d6 = 4611686018427389955ul;
        double d7 = 9223372036854776832ul;
        double uninitialized;
        static int i = 4.9;
        int unsigned u = 42949.672923E5;
        long l = 4611686018427389440.;
        unsigned long ul = 18446744073709549568.;
        int main(void) {
            if (d1 != 2147483647.) {
                return 1;
            }
            if (d2 != 4294967295.) {
                return 2;
            }
            if (d3 != 4611686018427389952.) {
                return 3;
            }
            if (d4 != d3) {
                return 4;
            }
            if (d5 != 9223372036854775808.) {
                return 5;
            }
            if (d6 != d3) {
                return 6;
            }
            if (d7 != d5) {
                return 7;
            }
            if (uninitialized) {
                return 8;
            }
            if (i != 4) {
                return 9;
            }
            if (u != 4294967292u) {
                return 10;
            }
            if (l != 4611686018427389952l) {
                return 11;
            }
            if (ul != 18446744073709549568ul) {
                return 12;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d1
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <4> Constant Int [2147483647]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d2
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <10> Constant UInt [4294967295]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d3
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <16> Constant Long [4611686018427389440]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d4
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <22> Constant Long [4611686018427389955]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d5
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <28> Constant ULong [9223372036854775810]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d6
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <34> Constant ULong [4611686018427389955]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d7
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <40> Constant ULong [9223372036854776832]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uninitialized
            │   ╰── Type
            │       ╰── Double
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <51> Constant Double [+4.9e0]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── u
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <57> Constant Double [+4.2949672923e9]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <63> Constant Double [+4.61168601842739e18]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <69> Constant Double [+1.844674407370955e19]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Var [d1]
                    │   │       ╰── <80> Constant Double [+2.147483647e9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88> Var [d2]
                    │   │       ╰── <90> Constant Double [+4.294967295e9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> Var [d3]
                    │   │       ╰── <100> Constant Double [+4.61168601842739e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <108> Var [d4]
                    │   │       ╰── <111> Var [d3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <119> Var [d5]
                    │   │       ╰── <121> Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <123> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <129> Var [d6]
                    │   │       ╰── <132> Var [d3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <140> Var [d7]
                    │   │       ╰── <143> Var [d5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <145> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151> Var [uninitialized]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <161>  [!=]
                    │   │       ├── <158> Var [i]
                    │   │       ╰── <160> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <162> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171>  [!=]
                    │   │       ├── <168> Var [u]
                    │   │       ╰── <170> Constant UInt [4294967292]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <172> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <181>  [!=]
                    │   │       ├── <178> Var [l]
                    │   │       ╰── <180> Constant Long [4611686018427389952]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <182> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <191>  [!=]
                    │   │       ├── <188> Var [ul]
                    │   │       ╰── <190> Constant ULong [18446744073709549568]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <192> Constant Int [12]
                    ╰── Return
                        ╰── <197> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_double_and_int_params_recursive() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9) {
            if (i1 != d9) {
                int call1 = fun(i1 + 1, d1, i2 + 1, d2, i3 + 1, d3, i4 + 1, d4, i5 + 1, d5, i6 + 1, d6, i7 + 1, d7, i8 + 1, d8, i9 + 1, d9);
                int call2 = fun(i1, d1 - 1, i2, d2 - 1, i3, d3 - 1, i4, d4 - 1, i5, d5 - 1, i6, d6 - 1, i7, d7 - 1, i8, d8 - 1, i9, d9 - 1);
                if (call1) {
                    return call1;
                }
                if (call2) {
                    return call2;
                }
            }
            if (i2 != i1 + 2) {
                return 2;
            }
            if (i3 != i1 + 4) {
                return 3;
            }
            if (i4 != i1 + 6) {
                return 4;
            }
            if (i5 != i1 + 8) {
                return 5;
            }
            if (i6 != i1 + 10) {
                return 6;
            }
            if (i7 != i1 + 12) {
                return 7;
            }
            if (i8 != i1 + 14) {
                return 8;
            }
            if (i9 != i1 + 16) {
                return 9;
            }
            if (d1 != d9 - 16) {
                return 11;
            }
            if (d2 != d9 - 14) {
                return 12;
            }
            if (d3 != d9 - 12) {
                return 13;
            }
            if (d4 != d9 - 10) {
                return 14;
            }
            if (d5 != d9 - 8) {
                return 15;
            }
            if (d6 != d9 - 6) {
                return 16;
            }
            if (d7 != d9 - 4) {
                return 17;
            }
            if (d8 != d9 - 2) {
                return 18;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [fun]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i1
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d1
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i2
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d2
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i3
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d3
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i4
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d4
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i5
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d5
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i6
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d6
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i7
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d7
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i8
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d8
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i9
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── d9
                │       ╰── Type
                │           ╰── Double
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <61> Var [i1]
                    │   │       ╰── <64> Var [d9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── call1
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <133> FunctionCall [fun]
                    │           │           ├── <74>  [+]
                    │           │           │   ├── <71> Var [i1]
                    │           │           │   ╰── <73> Constant Int [1]
                    │           │           ├── <76> Var [d1]
                    │           │           ├── <81>  [+]
                    │           │           │   ├── <78> Var [i2]
                    │           │           │   ╰── <80> Constant Int [1]
                    │           │           ├── <83> Var [d2]
                    │           │           ├── <88>  [+]
                    │           │           │   ├── <85> Var [i3]
                    │           │           │   ╰── <87> Constant Int [1]
                    │           │           ├── <90> Var [d3]
                    │           │           ├── <95>  [+]
                    │           │           │   ├── <92> Var [i4]
                    │           │           │   ╰── <94> Constant Int [1]
                    │           │           ├── <97> Var [d4]
                    │           │           ├── <102>  [+]
                    │           │           │   ├── <99> Var [i5]
                    │           │           │   ╰── <101> Constant Int [1]
                    │           │           ├── <104> Var [d5]
                    │           │           ├── <109>  [+]
                    │           │           │   ├── <106> Var [i6]
                    │           │           │   ╰── <108> Constant Int [1]
                    │           │           ├── <111> Var [d6]
                    │           │           ├── <116>  [+]
                    │           │           │   ├── <113> Var [i7]
                    │           │           │   ╰── <115> Constant Int [1]
                    │           │           ├── <118> Var [d7]
                    │           │           ├── <123>  [+]
                    │           │           │   ├── <120> Var [i8]
                    │           │           │   ╰── <122> Constant Int [1]
                    │           │           ├── <125> Var [d8]
                    │           │           ├── <130>  [+]
                    │           │           │   ├── <127> Var [i9]
                    │           │           │   ╰── <129> Constant Int [1]
                    │           │           ╰── <132> Var [d9]
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── call2
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <203> FunctionCall [fun]
                    │           │           ├── <141> Var [i1]
                    │           │           ├── <146>  [-]
                    │           │           │   ├── <143> Var [d1]
                    │           │           │   ╰── <145> Constant Int [1]
                    │           │           ├── <148> Var [i2]
                    │           │           ├── <153>  [-]
                    │           │           │   ├── <150> Var [d2]
                    │           │           │   ╰── <152> Constant Int [1]
                    │           │           ├── <155> Var [i3]
                    │           │           ├── <160>  [-]
                    │           │           │   ├── <157> Var [d3]
                    │           │           │   ╰── <159> Constant Int [1]
                    │           │           ├── <162> Var [i4]
                    │           │           ├── <167>  [-]
                    │           │           │   ├── <164> Var [d4]
                    │           │           │   ╰── <166> Constant Int [1]
                    │           │           ├── <169> Var [i5]
                    │           │           ├── <174>  [-]
                    │           │           │   ├── <171> Var [d5]
                    │           │           │   ╰── <173> Constant Int [1]
                    │           │           ├── <176> Var [i6]
                    │           │           ├── <181>  [-]
                    │           │           │   ├── <178> Var [d6]
                    │           │           │   ╰── <180> Constant Int [1]
                    │           │           ├── <183> Var [i7]
                    │           │           ├── <188>  [-]
                    │           │           │   ├── <185> Var [d7]
                    │           │           │   ╰── <187> Constant Int [1]
                    │           │           ├── <190> Var [i8]
                    │           │           ├── <195>  [-]
                    │           │           │   ├── <192> Var [d8]
                    │           │           │   ╰── <194> Constant Int [1]
                    │           │           ├── <197> Var [i9]
                    │           │           ╰── <202>  [-]
                    │           │               ├── <199> Var [d9]
                    │           │               ╰── <201> Constant Int [1]
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <207> Var [call1]
                    │           │   ╰── Then
                    │           │       ╰── Block
                    │           │           ╰── Return
                    │           │               ╰── <209> Var [call1]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <215> Var [call2]
                    │               ╰── Then
                    │                   ╰── Block
                    │                       ╰── Return
                    │                           ╰── <217> Var [call2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <233>  [!=]
                    │   │       ├── <226> Var [i2]
                    │   │       ╰── <232>  [+]
                    │   │           ├── <229> Var [i1]
                    │   │           ╰── <231> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <234> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <247>  [!=]
                    │   │       ├── <240> Var [i3]
                    │   │       ╰── <246>  [+]
                    │   │           ├── <243> Var [i1]
                    │   │           ╰── <245> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <248> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <261>  [!=]
                    │   │       ├── <254> Var [i4]
                    │   │       ╰── <260>  [+]
                    │   │           ├── <257> Var [i1]
                    │   │           ╰── <259> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <262> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <275>  [!=]
                    │   │       ├── <268> Var [i5]
                    │   │       ╰── <274>  [+]
                    │   │           ├── <271> Var [i1]
                    │   │           ╰── <273> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <276> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <289>  [!=]
                    │   │       ├── <282> Var [i6]
                    │   │       ╰── <288>  [+]
                    │   │           ├── <285> Var [i1]
                    │   │           ╰── <287> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <290> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <303>  [!=]
                    │   │       ├── <296> Var [i7]
                    │   │       ╰── <302>  [+]
                    │   │           ├── <299> Var [i1]
                    │   │           ╰── <301> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <304> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <317>  [!=]
                    │   │       ├── <310> Var [i8]
                    │   │       ╰── <316>  [+]
                    │   │           ├── <313> Var [i1]
                    │   │           ╰── <315> Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <318> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <331>  [!=]
                    │   │       ├── <324> Var [i9]
                    │   │       ╰── <330>  [+]
                    │   │           ├── <327> Var [i1]
                    │   │           ╰── <329> Constant Int [16]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <332> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <345>  [!=]
                    │   │       ├── <338> Var [d1]
                    │   │       ╰── <344>  [-]
                    │   │           ├── <341> Var [d9]
                    │   │           ╰── <343> Constant Int [16]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <346> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <359>  [!=]
                    │   │       ├── <352> Var [d2]
                    │   │       ╰── <358>  [-]
                    │   │           ├── <355> Var [d9]
                    │   │           ╰── <357> Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <360> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <373>  [!=]
                    │   │       ├── <366> Var [d3]
                    │   │       ╰── <372>  [-]
                    │   │           ├── <369> Var [d9]
                    │   │           ╰── <371> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <374> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <387>  [!=]
                    │   │       ├── <380> Var [d4]
                    │   │       ╰── <386>  [-]
                    │   │           ├── <383> Var [d9]
                    │   │           ╰── <385> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <388> Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <401>  [!=]
                    │   │       ├── <394> Var [d5]
                    │   │       ╰── <400>  [-]
                    │   │           ├── <397> Var [d9]
                    │   │           ╰── <399> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <402> Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <415>  [!=]
                    │   │       ├── <408> Var [d6]
                    │   │       ╰── <414>  [-]
                    │   │           ├── <411> Var [d9]
                    │   │           ╰── <413> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <416> Constant Int [16]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <429>  [!=]
                    │   │       ├── <422> Var [d7]
                    │   │       ╰── <428>  [-]
                    │   │           ├── <425> Var [d9]
                    │   │           ╰── <427> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <430> Constant Int [17]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <443>  [!=]
                    │   │       ├── <436> Var [d8]
                    │   │       ╰── <442>  [-]
                    │   │           ├── <439> Var [d9]
                    │   │           ╰── <441> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <444> Constant Int [18]
                    ╰── Return
                        ╰── <449> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_double_and_int_params_recursive_client() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9);
        int main(void) {
            double d = fun(1, 2.0, 3, 4.0, 5, 6.0, 7, 8.0, 9, 10.0, 11, 12.0, 13, 14.0, 15, 16.0, 17, 18.0);
            return (d == 78.00);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fun]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i1
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d1
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i2
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d2
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i3
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d3
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i4
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d4
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i5
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d5
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i6
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d6
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i7
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d7
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i8
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d8
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i9
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d9
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <88> FunctionCall [fun]
                    │           ├── <70> Constant Int [1]
                    │           ├── <71> Constant Double [+2e0]
                    │           ├── <72> Constant Int [3]
                    │           ├── <73> Constant Double [+4e0]
                    │           ├── <74> Constant Int [5]
                    │           ├── <75> Constant Double [+6e0]
                    │           ├── <76> Constant Int [7]
                    │           ├── <77> Constant Double [+8e0]
                    │           ├── <78> Constant Int [9]
                    │           ├── <79> Constant Double [+1e1]
                    │           ├── <80> Constant Int [11]
                    │           ├── <81> Constant Double [+1.2e1]
                    │           ├── <82> Constant Int [13]
                    │           ├── <83> Constant Double [+1.4e1]
                    │           ├── <84> Constant Int [15]
                    │           ├── <85> Constant Double [+1.6e1]
                    │           ├── <86> Constant Int [17]
                    │           ╰── <87> Constant Double [+1.8e1]
                    ╰── Return
                        ╰── <96>  [==]
                            ├── <92> Var [d]
                            ╰── <94> Constant Double [+7.8e1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_double_parameters() {
    let src = r#"
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h) {
            if (a != 1.0) {
                return 1;
            }
            if (b != 2.0) {
                return 2;
            }
            if (c != 3.0) {
                return 3;
            }
            if (d != 4.0) {
                return 4;
            }
            if (e != -1.0) {
                return 5;
            }
            if (f != -2.0) {
                return 6;
            }
            if (g != -3.0) {
                return 7;
            }
            if (h != -4.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [check_arguments]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Double
                │   ╰── Param
                │       ├── Name
                │       │   ╰── h
                │       ╰── Type
                │           ╰── Double
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <34>  [!=]
                    │   │       ├── <31> Var [a]
                    │   │       ╰── <33> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <35> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <41> Var [b]
                    │   │       ╰── <43> Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> Var [c]
                    │   │       ╰── <53> Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61> Var [d]
                    │   │       ╰── <63> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <71> Var [e]
                    │   │       ╰── <75> Unary [-]
                    │   │           ╰── <74> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <83> Var [f]
                    │   │       ╰── <87> Unary [-]
                    │   │           ╰── <86> Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <95> Var [g]
                    │   │       ╰── <99> Unary [-]
                    │   │           ╰── <98> Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <107> Var [h]
                    │   │       ╰── <111> Unary [-]
                    │   │           ╰── <110> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [8]
                    ╰── Return
                        ╰── <118> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_double_parameters_client() {
    let src = r#"
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h);
        int main(void) {
            return check_arguments(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_arguments]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── h
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <53> FunctionCall [check_arguments]
                            ├── <37> Constant Double [+1e0]
                            ├── <38> Constant Double [+2e0]
                            ├── <39> Constant Double [+3e0]
                            ├── <40> Constant Double [+4e0]
                            ├── <43> Unary [-]
                            │   ╰── <42> Constant Double [+1e0]
                            ├── <46> Unary [-]
                            │   ╰── <45> Constant Double [+2e0]
                            ├── <49> Unary [-]
                            │   ╰── <48> Constant Double [+3e0]
                            ╰── <52> Unary [-]
                                ╰── <51> Constant Double [+4e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_double_params_and_result() {
    let src = r#"
        double fmax(double x, double y);
        double get_max(double a, double b, double c, double d,
                       double e, double f, double g, double h,
                       double i, double j, double k)
        {
            double max = fmax(
                fmax(
                    fmax(
                        fmax(a, b),
                        fmax(c, d)),
                    fmax(
                        fmax(e, f),
                        fmax(g, h))),
                fmax(i, fmax(j, k)));
            return max;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fmax]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── y
            │           ╰── Type
            │               ╰── Double
            ╰── Function [get_max]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── h
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i
                │   │   ╰── Type
                │   │       ╰── Double
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── j
                │   │   ╰── Type
                │   │       ╰── Double
                │   ╰── Param
                │       ├── Name
                │       │   ╰── k
                │       ╰── Type
                │           ╰── Double
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── max
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <95> FunctionCall [fmax]
                    │           ├── <84> FunctionCall [fmax]
                    │           │   ├── <69> FunctionCall [fmax]
                    │           │   │   ├── <62> FunctionCall [fmax]
                    │           │   │   │   ├── <59> Var [a]
                    │           │   │   │   ╰── <61> Var [b]
                    │           │   │   ╰── <68> FunctionCall [fmax]
                    │           │   │       ├── <65> Var [c]
                    │           │   │       ╰── <67> Var [d]
                    │           │   ╰── <83> FunctionCall [fmax]
                    │           │       ├── <76> FunctionCall [fmax]
                    │           │       │   ├── <73> Var [e]
                    │           │       │   ╰── <75> Var [f]
                    │           │       ╰── <82> FunctionCall [fmax]
                    │           │           ├── <79> Var [g]
                    │           │           ╰── <81> Var [h]
                    │           ╰── <94> FunctionCall [fmax]
                    │               ├── <87> Var [i]
                    │               ╰── <93> FunctionCall [fmax]
                    │                   ├── <90> Var [j]
                    │                   ╰── <92> Var [k]
                    ╰── Return
                        ╰── <99> Var [max]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_double_params_and_result_client() {
    let src = r#"
        double get_max(double a, double b, double c, double d,
                       double e, double f, double g, double h,
                       double i, double j, double k);
        int main(void)
        {
            double result = get_max(100.3, 200.1, 0.01, 1.00004e5, 55.555, -4., 6543.2,
                                    9e9, 8e8, 7.6, 10e3 * 11e5);
            return result == 10e3 * 11e5;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_max]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── h
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ╰── Type
            │       │       ╰── Double
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── j
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── k
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <65> FunctionCall [get_max]
                    │           ├── <49> Constant Double [+1.003e2]
                    │           ├── <50> Constant Double [+2.001e2]
                    │           ├── <51> Constant Double [+1e-2]
                    │           ├── <52> Constant Double [+1.00004e5]
                    │           ├── <53> Constant Double [+5.5555e1]
                    │           ├── <56> Unary [-]
                    │           │   ╰── <55> Constant Double [+4e0]
                    │           ├── <57> Constant Double [+6.5432e3]
                    │           ├── <58> Constant Double [+9e9]
                    │           ├── <59> Constant Double [+8e8]
                    │           ├── <60> Constant Double [+7.6e0]
                    │           ╰── <64>  [*]
                    │               ├── <61> Constant Double [+1e4]
                    │               ╰── <63> Constant Double [+1.1e6]
                    ╰── Return
                        ╰── <75>  [==]
                            ├── <69> Var [result]
                            ╰── <74>  [*]
                                ├── <71> Constant Double [+1e4]
                                ╰── <73> Constant Double [+1.1e6]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_extern_double() {
    let src = r#"
        double d = 1e20;
    "#;
    let expected = r#"
        Program
            ╰── VarDeclaration
                ├── Name
                │   ╰── d
                ├── Type
                │   ╰── Double
                ╰── Initializer
                    ╰── <4> Constant Double [+1e20]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_extern_double_client() {
    let src = r#"
        
        extern double d;
        int main(void) {
            return d == 1e20;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <15>  [==]
                            ├── <12> Var [d]
                            ╰── <14> Constant Double [+1e20]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_use_arg_after_fun_call() {
    let src = r#"
        double fun(double x) {
            if (x > 2)
                return x;
            else {
                double ret = fun(x + 2);
                return ret + x;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [fun]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── x
                │       ╰── Type
                │           ╰── Double
                ╰── Body
                    ╰── If
                        ├── Condition
                        │   ╰── <13>  [>]
                        │       ├── <10> Var [x]
                        │       ╰── <12> Constant Int [2]
                        ├── Then
                        │   ╰── Return
                        │       ╰── <15> Var [x]
                        ╰── Else
                            ╰── Block
                                ├── VarDeclaration
                                │   ├── Name
                                │   │   ╰── ret
                                │   ├── Type
                                │   │   ╰── Double
                                │   ╰── Initializer
                                │       ╰── <26> FunctionCall [fun]
                                │           ╰── <25>  [+]
                                │               ├── <22> Var [x]
                                │               ╰── <24> Constant Int [2]
                                ╰── Return
                                    ╰── <34>  [+]
                                        ├── <30> Var [ret]
                                        ╰── <33> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_use_arg_after_fun_call_client() {
    let src = r#"
        double fun(double x);
        int main(void) {
            return fun(1.0);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fun]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── x
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <17> FunctionCall [fun]
                            ╰── <16> Constant Double [+1e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_special_values_infinity() {
    let src = r#"
        double inf = 2e308;
        double very_large = 1.79E308;
        double zero = 0.0;
        int main(void) {
            if (inf != 11e330) {
                return 1;
            }
            if (inf <= very_large) {
                return 2;
            }
            if(very_large * 10.0 != inf) {
                return 3;
            }
            if (1.0 / zero != inf) {
                return 4;
            }
            double negated_inf = -inf;
            double negated_inf2 = -1.0 / zero;
            if (negated_inf >= -very_large) {
                return 5;
            }
            if (negated_inf != negated_inf2) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── inf
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <4> Constant Double [+inf]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── very_large
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <10> Constant Double [+1.79e308]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <16> Constant Double [+0e0]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> Var [inf]
                    │   │       ╰── <27> Constant Double [+inf]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <29> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [<=]
                    │   │       ├── <35> Var [inf]
                    │   │       ╰── <38> Var [very_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <49>  [*]
                    │   │       │   ├── <46> Var [very_large]
                    │   │       │   ╰── <48> Constant Double [+1e1]
                    │   │       ╰── <52> Var [inf]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <63>  [/]
                    │   │       │   ├── <59> Constant Double [+1e0]
                    │   │       │   ╰── <62> Var [zero]
                    │   │       ╰── <66> Var [inf]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <68> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negated_inf
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <79> Unary [-]
                    │           ╰── <78> Var [inf]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negated_inf2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <91>  [/]
                    │           ├── <87> Unary [-]
                    │           │   ╰── <86> Constant Double [+1e0]
                    │           ╰── <90> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [>=]
                    │   │       ├── <95> Var [negated_inf]
                    │   │       ╰── <100> Unary [-]
                    │   │           ╰── <99> Var [very_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <108> Var [negated_inf]
                    │   │       ╰── <111> Var [negated_inf2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [6]
                    ╰── Return
                        ╰── <118> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_special_values_negative_zero() {
    let src = r#"
        double copysign(double x, double y);
        double zero = 0.0;
        int main(void) {
            double negative_zero = -zero;
            if (negative_zero != 0)
                return 1;
            if ( 1/negative_zero != -10e308 )
                return 2;
            if ( (-10)/negative_zero != 10e308)
                return 3;
            int fail = 0;
            negative_zero && (fail = 1);
            if (fail)
                return 4;
            if (negative_zero) {
                return 5;
            }
            if (zero != -0.0) {
                return 6;
            }
            double negated = copysign(4.0, -0.0);
            double positive = copysign(-5.0, 0.0);
            if (negated != -4.0) {
                return 7;
            }
            if (positive != 5.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [copysign]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── y
            │           ╰── Type
            │               ╰── Double
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── <16> Constant Double [+0e0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negative_zero
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <30> Unary [-]
                    │           ╰── <29> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [negative_zero]
                    │   │       ╰── <36> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <38> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <45>  [/]
                    │   │       │   ├── <41> Constant Int [1]
                    │   │       │   ╰── <44> Var [negative_zero]
                    │   │       ╰── <49> Unary [-]
                    │   │           ╰── <48> Constant Double [+inf]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <51> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61>  [/]
                    │   │       │   ├── <57> Unary [-]
                    │   │       │   │   ╰── <55> Constant Int [10]
                    │   │       │   ╰── <60> Var [negative_zero]
                    │   │       ╰── <63> Constant Double [+inf]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <65> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── fail
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <71> Constant Int [0]
                    ├── <83>  [&&]
                    │   ├── <75> Var [negative_zero]
                    │   ╰── <82> Assign [=]
                    │       ├── <78> Var [fail]
                    │       ╰── <80> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86> Var [fail]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <87> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91> Var [negative_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <98> Var [zero]
                    │   │       ╰── <102> Unary [-]
                    │   │           ╰── <101> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negated
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <117> FunctionCall [copysign]
                    │           ├── <113> Constant Double [+4e0]
                    │           ╰── <116> Unary [-]
                    │               ╰── <115> Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── positive
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <128> FunctionCall [copysign]
                    │           ├── <126> Unary [-]
                    │           │   ╰── <125> Constant Double [+5e0]
                    │           ╰── <127> Constant Double [+0e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137>  [!=]
                    │   │       ├── <132> Var [negated]
                    │   │       ╰── <136> Unary [-]
                    │   │           ╰── <135> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <138> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <147>  [!=]
                    │   │       ├── <144> Var [positive]
                    │   │       ╰── <146> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <148> Constant Int [8]
                    ╰── Return
                        ╰── <153> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_special_values_subnormal_not_zero() {
    let src = r#"
        int non_zero(double d) {
            return !d;
        }
        double multiply_by_large_num(double d) {
            return d * 2e20;
        }
        int main(void) {
            double subnormal = 2.5e-320;
            if (multiply_by_large_num(subnormal) != 4.99994433591341498562e-300) {
                return 2;
            }
            return non_zero(subnormal);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [non_zero]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Unary [!]
            │               ╰── <11> Var [d]
            ├── Function [multiply_by_large_num]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <28>  [*]
            │               ├── <25> Var [d]
            │               ╰── <27> Constant Double [+2e20]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subnormal
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <40> Constant Double [+2.5e-320]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> FunctionCall [multiply_by_large_num]
                    │   │       │   ╰── <45> Var [subnormal]
                    │   │       ╰── <48> Constant Double [+4.999944335913415e-300]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <50> Constant Int [2]
                    ╰── Return
                        ╰── <58> FunctionCall [non_zero]
                            ╰── <57> Var [subnormal]
    "#;
    assert_parse(src, expected);
}
