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
                    │       ╰── <10> Unary [~]
                    │           ╰── Constant Double [+1e1]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <13>  [&]
                    │           ├── Constant Double [+1e1]
                    │           ╰── <12> Unary [-]
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <13>  [|]
                    │           ├── Constant Double [+0e0]
                    │           ╰── <12> Unary [-]
                    │               ╰── Constant Double [+0e0]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <11>  [<<]
                    │           ├── Constant Double [+5e0]
                    │           ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
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
                        ╰── <8>  [<<]
                            ├── Constant Int [1]
                            ╰── Constant Double [+2e0]
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
                        ╰── <10>  [^]
                            ├── Constant Double [+1e10]
                            ╰── <9> Unary [-]
                                ╰── Constant Double [+1e10]
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
                    │       ╰── Constant Double [+1e0]
                    ├── <15> Assign [&=]
                    │   ├── <12> Var [d]
                    │   ╰── Constant Int [0]
                    ╰── Return
                        ╰── <21> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <20> Var [d]
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
                    │       ╰── Constant Int [0]
                    ├── <15> Assign [|=]
                    │   ├── <12> Var [i]
                    │   ╰── Constant Double [+2e0]
                    ╰── Return
                        ╰── <21> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <20> Var [i]
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
                    │       ╰── Constant Double [+1e0]
                    ├── <15> Assign [<<=]
                    │   ├── <12> Var [d]
                    │   ╰── Constant Int [1]
                    ╰── Return
                        ╰── <18> Var [d]
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
                    │       ╰── Constant Double [+5e0]
                    ├── <15> Assign [&=]
                    │   ├── <12> Var [d]
                    │   ╰── Constant Int [2]
                    ╰── Return
                        ╰── <21> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <20> Var [d]
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
                    │       ╰── Constant Int [5]
                    ├── <15> Assign [&=]
                    │   ├── <12> Var [i]
                    │   ╰── Constant Double [+1e0]
                    ╰── Return
                        ╰── <18> Var [i]
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
                    │       ╰── Constant Int [1000]
                    ├── <15> Assign [>>=]
                    │   ├── <12> Var [i]
                    │   ╰── Constant Double [+2e0]
                    ╰── Return
                        ╰── <18> Var [i]
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
                    │       ╰── Constant Int [10]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <12> Var [x]
                        ╰── Block
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [4]
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
                    │       ╰── Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [d]
                    │   ╰── Block
                    │       ╰── Case [10]
                    │           ╰── Return
                    │               ╰── Constant Int [0]
                    ╰── Return
                        ╰── Constant Int [1]
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
                    │       ╰── Constant Double [+1e1]
                    ├── <19> Assign [=]
                    │   ├── <12> Var [d]
                    │   ╰── <18>  [%]
                    │       ├── <15> Var [d]
                    │       ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <11>  [%]
                    │           ├── Constant Double [+3e0]
                    │           ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53> Unary [!]
                    │   │       ╰── <52>  [&&]
                    │   │           ├── <43>  [&&]
                    │   │           │   ├── <35>  [==]
                    │   │           │   │   ├── <31> Var [a]
                    │   │           │   │   ╰── <34> Var [b]
                    │   │           │   ╰── <42>  [==]
                    │   │           │       ├── <38> Var [a]
                    │   │           │       ╰── <41> Var [c]
                    │   │           ╰── <50>  [==]
                    │   │               ├── <46> Var [a]
                    │   │               ╰── <49> Var [d]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70>  [+]
                    │   │       │   ├── <66>  [+]
                    │   │       │   │   ├── <62>  [+]
                    │   │       │   │   │   ├── <58> Var [a]
                    │   │       │   │   │   ╰── <61> Var [b]
                    │   │       │   │   ╰── <65> Var [c]
                    │   │       │   ╰── <69> Var [d]
                    │   │       ╰── Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1.25e-1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1.25e-1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1.25e-1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1.25e-1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125> Unary [!]
                    │   │       ╰── <124>  [&&]
                    │   │           ├── <115>  [&&]
                    │   │           │   ├── <107>  [==]
                    │   │           │   │   ├── <103> Var [e]
                    │   │           │   │   ╰── <106> Var [f]
                    │   │           │   ╰── <114>  [==]
                    │   │           │       ├── <110> Var [e]
                    │   │           │       ╰── <113> Var [g]
                    │   │           ╰── <122>  [==]
                    │   │               ├── <118> Var [e]
                    │   │               ╰── <121> Var [h]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [!=]
                    │   │       ├── <142>  [+]
                    │   │       │   ├── <138>  [+]
                    │   │       │   │   ├── <134>  [+]
                    │   │       │   │   │   ├── <130> Var [e]
                    │   │       │   │   │   ╰── <133> Var [f]
                    │   │       │   │   ╰── <137> Var [g]
                    │   │       │   ╰── <141> Var [h]
                    │   │       ╰── Constant Double [+5e-1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │   │   ╰── <8>  [!=]
                    │   │       ├── Constant Double [+1.0000000000000004e0]
                    │   │       ╰── Constant Double [+1.0000000000000004e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <17>  [!=]
                    │   │       ├── Constant Double [+9.223372036854778e18]
                    │   │       ╰── Constant Double [+9.223372036854778e18]
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
            │       ╰── Constant Double [+3e0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <16> Unary [-]
                    │           ╰── Constant Long [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <24> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <34> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <33> Var [glob]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── k
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <44> Var [l]
                    │   │       ╰── <48> Unary [-]
                    │   │           ╰── Constant Long [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <56> Var [i]
                    │   │       ╰── <60> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <68> Var [j]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Var [k]
                    │   │       ╰── Constant Int [20]
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
            │           ╰── <12> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <11> Var [d]
            ├── Function [double_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <27> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <26> Var [d]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <40> FunctionCall [double_to_long]
                    │           ╰── Constant Double [+2.1484290993e9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Var [l]
                    │   │       ╰── Constant Long [2148429099]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <60> FunctionCall [double_to_int]
                    │           ╰── <59> Unary [-]
                    │               ╰── Constant Double [+2.000009999e5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <64> Var [i]
                    │   │       ╰── <68> Unary [-]
                    │   │           ╰── Constant Int [200000]
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
            │           ╰── <12> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <11> Var [d]
            ├── Function [double_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <27> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <26> Var [d]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> FunctionCall [double_to_uint]
                    │   │       │   ╰── Constant Double [+1.09e1]
                    │   │       ╰── Constant UInt [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <48> FunctionCall [double_to_uint]
                    │   │       │   ╰── Constant Double [+2.1474837505e9]
                    │   │       ╰── Constant Long [2147483750]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59> FunctionCall [double_to_ulong]
                    │   │       │   ╰── Constant Double [+3.43597383685e10]
                    │   │       ╰── Constant ULong [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> FunctionCall [double_to_ulong]
                    │   │       │   ╰── Constant Double [+3.4587645138215895e18]
                    │   │       ╰── Constant ULong [3458764513821589504]
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
            │       ╰── Constant Double [+5e3]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── should_spill
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <62> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <61> Var [glob]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <72>  [-]
            │       │           ├── <69> Var [glob]
            │       │           ╰── Constant Int [4999]
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
            │       │       ╰── <226>  [-]
            │       │           ├── <223> Var [glob]
            │       │           ╰── Constant Int [4987]
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
            │       │   │       ╰── Constant Int [5000]
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
            │           ╰── <12> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <11> Var [i]
            ├── Function [long_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <27> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <26> Var [l]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <39> FunctionCall [int_to_double]
                    │   │       │   ╰── <38> Unary [-]
                    │   │       │       ╰── Constant Int [100000]
                    │   │       ╰── <43> Unary [-]
                    │   │           ╰── Constant Double [+1e5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <54> FunctionCall [long_to_double]
                    │   │       │   ╰── <53> Unary [-]
                    │   │       │       ╰── Constant Long [9007199254751227]
                    │   │       ╰── <58> Unary [-]
                    │   │           ╰── Constant Double [+9.007199254751228e15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <71> Cast
                    │           ├── Target
                    │           │   ╰── Double
                    │           ╰── Expression
                    │               ╰── Constant Long [1152921504606846977]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <75> Var [d]
                    │   │       ╰── Constant Double [+1.152921504606847e18]
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
            │           ╰── <12> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <11> Var [ui]
            ├── Function [ulong_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <27> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <26> Var [ul]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> FunctionCall [uint_to_double]
                    │   │       │   ╰── Constant UInt [1000]
                    │   │       ╰── Constant Double [+1e3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <48> FunctionCall [uint_to_double]
                    │   │       │   ╰── Constant UInt [4294967200]
                    │   │       ╰── Constant Double [+4.2949672e9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59> FunctionCall [ulong_to_double]
                    │   │       │   ╰── Constant ULong [138512825844]
                    │   │       ╰── Constant Double [+1.38512825844e11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> FunctionCall [ulong_to_double]
                    │   │       │   ╰── Constant ULong [10223372036854775816]
                    │   │       ╰── Constant Double [+1.0223372036854776e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> FunctionCall [ulong_to_double]
                    │   │       │   ╰── Constant ULong [9223372036854776832]
                    │   │       ╰── Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95>  [!=]
                    │   │       ├── <92> FunctionCall [ulong_to_double]
                    │   │       │   ╰── Constant ULong [9223372036854776833]
                    │   │       ╰── Constant Double [+9.223372036854778e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [!=]
                    │   │       ├── <103> FunctionCall [ulong_to_double]
                    │   │       │   ╰── Constant ULong [9223372036854776831]
                    │   │       ╰── Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> FunctionCall [ulong_to_double]
                    │   │       │   ╰── Constant ULong [9223372036854776830]
                    │   │       ╰── Constant Double [+9.223372036854776e18]
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
                    │       ╰── Constant Double [+1e1]
                    ├── <15> Assign [/=]
                    │   ├── <12> Var [d]
                    │   ╰── Constant Double [+4e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Var [d]
                    │   │       ╰── Constant Double [+2.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <31> Assign [*=]
                    │   ├── <28> Var [d]
                    │   ╰── Constant Double [+1e4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [d]
                    │   │       ╰── Constant Double [+2.5e4]
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
                    │       ╰── Constant Double [+1.0005e3]
                    ├── <15> Assign [+=]
                    │   ├── <12> Var [d]
                    │   ╰── Constant Int [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Var [d]
                    │   │       ╰── Constant Double [+2.0005e3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [18446744073709551586]
                    ├── <37> Assign [-=]
                    │   ├── <34> Var [ul]
                    │   ╰── Constant Double [+1.5e19]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Var [ul]
                    │   │       ╰── Constant ULong [3446744073709551616]
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
                    │       ╰── Constant Int [10]
                    ├── <59> Assign [+=]
                    │   ├── <56> Var [i]
                    │   ╰── Constant Double [+9.9999e-1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> Var [i]
                    │   │       ╰── Constant Int [10]
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
                    │   │   ╰── Constant Double [+7.5e-1]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <18>  [!=]
                    │   │       ├── <15> Postfix [++]
                    │   │       │   ╰── <13> Var [d]
                    │   │       ╰── Constant Double [+7.5e-1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> Var [d]
                    │   │       ╰── Constant Double [+1.75e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <40> Assign [=]
                    │   ├── <35> Var [d]
                    │   ╰── <39> Unary [-]
                    │       ╰── Constant Double [+1.002e2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <45> Unary [++]
                    │   │       │   ╰── <44> Var [d]
                    │   │       ╰── <49> Unary [-]
                    │   │           ╰── Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <57> Var [d]
                    │   │       ╰── <61> Unary [-]
                    │   │           ╰── Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <71> Postfix [--]
                    │   │       │   ╰── <69> Var [d]
                    │   │       ╰── <75> Unary [-]
                    │   │           ╰── Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <83> Var [d]
                    │   │       ╰── <87> Unary [-]
                    │   │           ╰── Constant Double [+1.002e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102>  [!=]
                    │   │       ├── <97> Unary [--]
                    │   │       │   ╰── <96> Var [d]
                    │   │       ╰── <101> Unary [-]
                    │   │           ╰── Constant Double [+1.012e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <109> Var [d]
                    │   │       ╰── <113> Unary [-]
                    │   │           ╰── Constant Double [+1.012e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── <124> Assign [=]
                    │   ├── <121> Var [d]
                    │   ╰── Constant Double [+1e-21]
                    ├── <129> Postfix [++]
                    │   ╰── <127> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135>  [!=]
                    │   │       ├── <132> Var [d]
                    │   │       ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── <145> Assign [=]
                    │   ├── <142> Var [d]
                    │   ╰── Constant Double [+1e21]
                    ├── <150> Postfix [--]
                    │   ╰── <148> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <153> Var [d]
                    │   │       ╰── Constant Double [+1e21]
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
                    │   │   ╰── Constant Double [+0e0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <27>  [/]
                    │           ├── Constant Double [+0e0]
                    │           ╰── <26> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [||]
                    │   │       ├── <55>  [||]
                    │   │       │   ├── <48>  [||]
                    │   │       │   │   ├── <41>  [||]
                    │   │       │   │   │   ├── <34>  [<]
                    │   │       │   │   │   │   ├── <31> Var [nan]
                    │   │       │   │   │   │   ╰── Constant Double [+0e0]
                    │   │       │   │   │   ╰── <40>  [==]
                    │   │       │   │   │       ├── <37> Var [nan]
                    │   │       │   │   │       ╰── Constant Double [+0e0]
                    │   │       │   │   ╰── <47>  [>]
                    │   │       │   │       ├── <44> Var [nan]
                    │   │       │   │       ╰── Constant Double [+0e0]
                    │   │       │   ╰── <54>  [<=]
                    │   │       │       ├── <51> Var [nan]
                    │   │       │       ╰── Constant Double [+0e0]
                    │   │       ╰── <61>  [>=]
                    │   │           ├── <58> Var [nan]
                    │   │           ╰── Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [||]
                    │   │       ├── <91>  [||]
                    │   │       │   ├── <84>  [||]
                    │   │       │   │   ├── <77>  [||]
                    │   │       │   │   │   ├── <70>  [<]
                    │   │       │   │   │   │   ├── Constant Int [1]
                    │   │       │   │   │   │   ╰── <69> Var [nan]
                    │   │       │   │   │   ╰── <76>  [==]
                    │   │       │   │   │       ├── Constant Int [1]
                    │   │       │   │   │       ╰── <75> Var [nan]
                    │   │       │   │   ╰── <83>  [>]
                    │   │       │   │       ├── Constant Int [1]
                    │   │       │   │       ╰── <82> Var [nan]
                    │   │       │   ╰── <90>  [<=]
                    │   │       │       ├── Constant Int [1]
                    │   │       │       ╰── <89> Var [nan]
                    │   │       ╰── <97>  [>=]
                    │   │           ├── Constant Int [1]
                    │   │           ╰── <96> Var [nan]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [==]
                    │   │       ├── <103> Var [nan]
                    │   │       ╰── <106> Var [nan]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119> Unary [!]
                    │   │       ╰── <118>  [!=]
                    │   │           ├── <113> Var [nan]
                    │   │           ╰── <116> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130> Unary [!]
                    │   │       ╰── <129> FunctionCall [double_isnan]
                    │   │           ╰── <128> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144> Unary [!]
                    │   │       ╰── <143> FunctionCall [double_isnan]
                    │   │           ╰── <142>  [*]
                    │   │               ├── Constant Int [4]
                    │   │               ╰── <141> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <158> Unary [!]
                    │   │       ╰── <157> FunctionCall [double_isnan]
                    │   │           ╰── <156>  [/]
                    │   │               ├── Constant Double [+2.2e3]
                    │   │               ╰── <155> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171> Unary [!]
                    │   │       ╰── <170> FunctionCall [double_isnan]
                    │   │           ╰── <169> Unary [-]
                    │   │               ╰── <168> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180> Unary [!]
                    │   │       ╰── <179> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <187> Var [nan]
                    │   ├── Then
                    │   │   ╰── Block
                    │   ╰── Else
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan_is_nonzero
                    │   ╰── Type
                    │       ╰── Int
                    ├── For
                    │   ├── Init
                    │   │   ╰── <203> Assign [=]
                    │   │       ├── <200> Var [nan_is_nonzero]
                    │   │       ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <205> Var [nan]
                    │   ╰── Block
                    │       ├── <210> Assign [=]
                    │       │   ├── <207> Var [nan_is_nonzero]
                    │       │   ╰── Constant Int [1]
                    │       ╰── Break
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <219> Unary [!]
                    │   │       ╰── <218> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── <229> Assign [=]
                    │   ├── <226> Var [nan_is_nonzero]
                    │   ╰── Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <232> Var [nan]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── <237> Assign [=]
                    │           │   ├── <234> Var [nan_is_nonzero]
                    │           │   ╰── Constant Int [1]
                    │           ╰── Break
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <246> Unary [!]
                    │   │       ╰── <245> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── <258> Assign [=]
                    │   ├── <253> Var [nan_is_nonzero]
                    │   ╰── <257> Unary [-]
                    │       ╰── Constant Int [1]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── <268> Assign [=]
                    │   │       │   ├── <261> Var [nan_is_nonzero]
                    │   │       │   ╰── <267>  [+]
                    │   │       │       ├── <264> Var [nan_is_nonzero]
                    │   │       │       ╰── Constant Int [1]
                    │   │       ╰── If
                    │   │           ├── Condition
                    │   │           │   ╰── <271> Var [nan_is_nonzero]
                    │   │           ╰── Then
                    │   │               ╰── Block
                    │   │                   ╰── Break
                    │   ╰── Condition
                    │       ╰── <279> Var [nan]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <284> Unary [!]
                    │   │       ╰── <283> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ├── <298> Assign [=]
                    │   ├── <291> Var [nan_is_nonzero]
                    │   ╰── <{node_id}> Conditional [?]
                    │       ├── <294> Var [nan]
                    │       ├── Then
                    │       │   ╰── Constant Int [1]
                    │       ╰── Else
                    │           ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <303> Unary [!]
                    │   │       ╰── <302> Var [nan_is_nonzero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [14]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │   │   ╰── Constant Double [+0e0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <27>  [/]
                    │           ├── Constant Double [+0e0]
                    │           ╰── <26> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38> Unary [!]
                    │   │       ╰── <37> FunctionCall [double_isnan]
                    │   │           ╰── <36> Assign [+=]
                    │   │               ├── <33> Var [nan]
                    │   │               ╰── Constant Double [+9.92e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53> Unary [!]
                    │   │       ╰── <52> FunctionCall [double_isnan]
                    │   │           ╰── <51> Assign [-=]
                    │   │               ├── <47> Var [nan]
                    │   │               ╰── <50> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67> Unary [!]
                    │   │       ╰── <66> FunctionCall [double_isnan]
                    │   │           ╰── <65> Assign [*=]
                    │   │               ├── <62> Var [nan]
                    │   │               ╰── Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81> Unary [!]
                    │   │       ╰── <80> FunctionCall [double_isnan]
                    │   │           ╰── <79> Assign [/=]
                    │   │               ├── <76> Var [nan]
                    │   │               ╰── Constant Double [+0e0]
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
                    │   │   ╰── Constant Double [+0e0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nan
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <27>  [/]
                    │           ├── Constant Double [+0e0]
                    │           ╰── <26> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37> Unary [!]
                    │   │       ╰── <36> FunctionCall [double_isnan]
                    │   │           ╰── <35> Unary [++]
                    │   │               ╰── <34> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50> Unary [!]
                    │   │       ╰── <49> FunctionCall [double_isnan]
                    │   │           ╰── <48> Unary [--]
                    │   │               ╰── <47> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63> Unary [!]
                    │   │       ╰── <62> FunctionCall [double_isnan]
                    │   │           ╰── <61> Postfix [++]
                    │   │               ╰── <59> Var [nan]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76> Unary [!]
                    │   │       ╰── <75> FunctionCall [double_isnan]
                    │   │           ╰── <74> Postfix [--]
                    │   │               ╰── <72> Var [nan]
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
            │       ╰── Constant Double [+1e-1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_two
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+2e-1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_three
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+3e-1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── two
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+2e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── three
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+3e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── four
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+4e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── twelveE30
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+1.2e31]
            ├── Function [addition]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <56>  [==]
            │               ├── <52>  [+]
            │               │   ├── <48> Var [point_one]
            │               │   ╰── <51> Var [point_two]
            │               ╰── Constant Double [+3.0000000000000004e-1]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <72>  [==]
            │               ├── <68>  [-]
            │               │   ├── <65> Var [four]
            │               │   ╰── Constant Double [+1e0]
            │               ╰── Constant Double [+3e0]
            ├── Function [multiplication]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <88>  [==]
            │               ├── <84>  [*]
            │               │   ├── Constant Double [+1e-2]
            │               │   ╰── <83> Var [point_three]
            │               ╰── Constant Double [+3e-3]
            ├── Function [division]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <104>  [==]
            │               ├── <100>  [/]
            │               │   ├── Constant Double [+7e0]
            │               │   ╰── <99> Var [two]
            │               ╰── Constant Double [+3.5e0]
            ├── Function [negation]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── neg
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <118> Unary [-]
            │       │           ╰── <117> Var [twelveE30]
            │       ╰── Return
            │           ╰── <128> Unary [!]
            │               ╰── <127>  [+]
            │                   ├── Constant Double [+1.2e31]
            │                   ╰── <125> Var [neg]
            ├── Function [complex_expression]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── complex_expression
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <152>  [-]
            │       │           ├── <145>  [+]
            │       │           │   ├── <140> Var [two]
            │       │           │   ╰── <143> Var [three]
            │       │           ╰── <151>  [*]
            │       │               ├── Constant Double [+1.275e2]
            │       │               ╰── <150> Var [four]
            │       ╰── Return
            │           ╰── <161>  [==]
            │               ├── <156> Var [complex_expression]
            │               ╰── <160> Unary [-]
            │                   ╰── Constant Double [+5.05e2]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172> Unary [!]
                    │   │       ╰── <171> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <181> Unary [!]
                    │   │       ╰── <180> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190> Unary [!]
                    │   │       ╰── <189> FunctionCall [multiplication]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <199> Unary [!]
                    │   │       ╰── <198> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <208> Unary [!]
                    │   │       ╰── <207> FunctionCall [negation]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <217> Unary [!]
                    │   │       ╰── <216> FunctionCall [complex_expression]
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
            │       ╰── Constant Double [+5.5e6]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── fifty_fourE4
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+5.4e5]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── tiny
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+4e-5]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── four
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+4e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── point_one
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+1e-1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [<]
                    │   │       ├── <36> Var [fifty_fiveE5]
                    │   │       ╰── <39> Var [fifty_fourE4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [>]
                    │   │       ├── <47> Var [four]
                    │   │       ╰── Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [<=]
                    │   │       ├── <57> Var [tiny]
                    │   │       ╰── Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [>=]
                    │   │       ├── <67> Var [fifty_fourE4]
                    │   │       ╰── <70> Var [fifty_fiveE5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [==]
                    │   │       ├── <78> Var [tiny]
                    │   │       ╰── Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <88> Var [point_one]
                    │   │       ╰── <91> Var [point_one]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105> Unary [!]
                    │   │       ╰── <104>  [>]
                    │   │           ├── <100> Var [tiny]
                    │   │           ╰── Constant Double [+5e-6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120> Unary [!]
                    │   │       ╰── <119>  [<]
                    │   │           ├── <114> Unary [-]
                    │   │           │   ╰── Constant Double [+4e-5]
                    │   │           ╰── <117> Var [four]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134> Unary [!]
                    │   │       ╰── <133>  [<=]
                    │   │           ├── <128> Var [tiny]
                    │   │           ╰── <131> Var [tiny]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148> Unary [!]
                    │   │       ╰── <147>  [>=]
                    │   │           ├── <142> Var [fifty_fiveE5]
                    │   │           ╰── <145> Var [fifty_fiveE5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <161> Unary [!]
                    │   │       ╰── <160>  [==]
                    │   │           ├── Constant Double [+1e-1]
                    │   │           ╰── <158> Var [point_one]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <174> Unary [!]
                    │   │       ╰── <173>  [!=]
                    │   │           ├── <169> Var [tiny]
                    │   │           ╰── Constant Double [+3e-5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183>  [<]
                    │   │       ├── Constant Double [+3e-5]
                    │   │       ╰── Constant Double [+3e-12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant Double [+0e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── non_zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+1e-20]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+1e0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── rounded_to_zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+0e0]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37> Var [rounded_to_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44> Var [non_zero]
                    │   ├── Then
                    │   │   ╰── Block
                    │   ╰── Else
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61> Unary [!]
                    │   │       ╰── <60> Var [non_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73> Unary [!]
                    │   │       ╰── <72> Unary [!]
                    │   │           ╰── <70> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85> Unary [!]
                    │   │       ╰── <84> Unary [!]
                    │   │           ╰── <82> Var [rounded_to_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98> Unary [!]
                    │   │       ╰── <97>  [&&]
                    │   │           ├── <93> Var [non_zero]
                    │   │           ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108>  [&&]
                    │   │       ├── Constant Double [+3e0]
                    │   │       ╰── <107> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [&&]
                    │   │       ├── <115> Var [rounded_to_zero]
                    │   │       ╰── Constant Double [+1e13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [&&]
                    │   │       ├── Constant ULong [18446744073709551615]
                    │   │       ╰── <127> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141> Unary [!]
                    │   │       ╰── <140>  [&&]
                    │   │           ├── <136> Var [non_zero]
                    │   │           ╰── Constant Long [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154> Unary [!]
                    │   │       ╰── <153>  [||]
                    │   │           ├── Constant Double [+5e0]
                    │   │           ╰── <151> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <165>  [||]
                    │   │       ├── <161> Var [zero]
                    │   │       ╰── <164> Var [rounded_to_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <178> Unary [!]
                    │   │       ╰── <177>  [||]
                    │   │           ├── <173> Var [rounded_to_zero]
                    │   │           ╰── Constant Double [+1e-4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <191> Unary [!]
                    │   │       ╰── <190>  [||]
                    │   │           ├── <186> Var [non_zero]
                    │   │           ╰── Constant UInt [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <203> Unary [!]
                    │   │       ╰── <202>  [||]
                    │   │           ├── Constant Int [0]
                    │   │           ╰── Constant Double [+5e-7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [16]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── d
                    │   │       ├── Type
                    │   │       │   ╰── Double
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Double [+1e2]
                    │   ├── Condition
                    │   │   ╰── <22>  [>]
                    │   │       ├── <19> Var [d]
                    │   │       ╰── Constant Double [+0e0]
                    │   ├── Condition
                    │   │   ╰── <31> Assign [=]
                    │   │       ├── <24> Var [d]
                    │   │       ╰── <30>  [-]
                    │   │           ├── <27> Var [d]
                    │   │           ╰── Constant Double [+1e0]
                    │   ╰── Block
                    │       ╰── <40> Assign [=]
                    │           ├── <33> Var [a]
                    │           ╰── <39>  [+]
                    │               ├── <36> Var [a]
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── <46> Var [a]
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
                    │       ╰── Constant Double [+2e0]
                    ╰── Return
                        ╰── <19>  [==]
                            ├── <15>  [*]
                            │   ├── <12> Var [x]
                            │   ╰── Constant Double [+2e0]
                            ╰── Constant Double [+4e0]
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
            │       │   │   ╰── Constant Double [+5e-1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ret
            │       │   ├── Type
            │       │   │   ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <16> Var [d]
            │       ├── <27> Assign [=]
            │       │   ├── <20> Var [d]
            │       │   ╰── <26>  [+]
            │       │       ├── <23> Var [d]
            │       │       ╰── Constant Double [+1e0]
            │       ╰── Return
            │           ╰── <30> Var [ret]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d1
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <42> FunctionCall [return_static_variable]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <49> FunctionCall [return_static_variable]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d3
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <56> FunctionCall [return_static_variable]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Var [d1]
                    │   │       ╰── Constant Double [+5e-1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> Var [d2]
                    │   │       ╰── Constant Double [+1.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> Var [d3]
                    │   │       ╰── Constant Double [+2.5e0]
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
            │       │   │   ╰── <48>  [!=]
            │       │   │       ├── <45> Var [d1]
            │       │   │       ╰── Constant Double [+1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <58>  [!=]
            │       │   │       ├── <55> Var [d2]
            │       │   │       ╰── Constant Double [+2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <68>  [!=]
            │       │   │       ├── <65> Var [d3]
            │       │   │       ╰── Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <78>  [!=]
            │       │   │       ├── <75> Var [d4]
            │       │   │       ╰── Constant Double [+4e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <88>  [!=]
            │       │   │       ├── <85> Var [d5]
            │       │   │       ╰── Constant Double [+5e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <98>  [!=]
            │       │   │       ├── <95> Var [d6]
            │       │   │       ╰── Constant Double [+6e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <108>  [!=]
            │       │   │       ├── <105> Var [d7]
            │       │   │       ╰── Constant Double [+7e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <118>  [!=]
            │       │   │       ├── <115> Var [d8]
            │       │   │       ╰── Constant Double [+8e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <128>  [!=]
            │       │   │       ├── <125> Var [i1]
            │       │   │       ╰── Constant Int [101]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <138>  [!=]
            │       │   │       ├── <135> Var [i2]
            │       │   │       ╰── Constant Int [102]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <148>  [!=]
            │       │   │       ├── <145> Var [i3]
            │       │   │       ╰── Constant Int [103]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <158>  [!=]
            │       │   │       ├── <155> Var [i4]
            │       │   │       ╰── Constant Int [104]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [12]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <168>  [!=]
            │       │   │       ├── <165> Var [i5]
            │       │   │       ╰── Constant Int [105]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [13]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <196> FunctionCall [check_arguments]
                            ├── Constant Double [+1e0]
                            ├── Constant Double [+2e0]
                            ├── Constant Int [101]
                            ├── Constant Double [+3e0]
                            ├── Constant Double [+4e0]
                            ├── Constant Int [102]
                            ├── Constant Int [103]
                            ├── Constant Int [104]
                            ├── Constant Double [+5e0]
                            ├── Constant Double [+6e0]
                            ├── Constant Double [+7e0]
                            ├── Constant Int [105]
                            ╰── Constant Double [+8e0]
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
            │       │   │   ╰── <64>  [!=]
            │       │   │       ├── <60> Var [i1]
            │       │   │       ╰── <63> Var [d9]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── call1
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Initializer
            │       │           │       ╰── <132> FunctionCall [fun]
            │       │           │           ├── <73>  [+]
            │       │           │           │   ├── <70> Var [i1]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <75> Var [d1]
            │       │           │           ├── <80>  [+]
            │       │           │           │   ├── <77> Var [i2]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <82> Var [d2]
            │       │           │           ├── <87>  [+]
            │       │           │           │   ├── <84> Var [i3]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <89> Var [d3]
            │       │           │           ├── <94>  [+]
            │       │           │           │   ├── <91> Var [i4]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <96> Var [d4]
            │       │           │           ├── <101>  [+]
            │       │           │           │   ├── <98> Var [i5]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <103> Var [d5]
            │       │           │           ├── <108>  [+]
            │       │           │           │   ├── <105> Var [i6]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <110> Var [d6]
            │       │           │           ├── <115>  [+]
            │       │           │           │   ├── <112> Var [i7]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <117> Var [d7]
            │       │           │           ├── <122>  [+]
            │       │           │           │   ├── <119> Var [i8]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <124> Var [d8]
            │       │           │           ├── <129>  [+]
            │       │           │           │   ├── <126> Var [i9]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ╰── <131> Var [d9]
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── call2
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Initializer
            │       │           │       ╰── <202> FunctionCall [fun]
            │       │           │           ├── <140> Var [i1]
            │       │           │           ├── <145>  [-]
            │       │           │           │   ├── <142> Var [d1]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <147> Var [i2]
            │       │           │           ├── <152>  [-]
            │       │           │           │   ├── <149> Var [d2]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <154> Var [i3]
            │       │           │           ├── <159>  [-]
            │       │           │           │   ├── <156> Var [d3]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <161> Var [i4]
            │       │           │           ├── <166>  [-]
            │       │           │           │   ├── <163> Var [d4]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <168> Var [i5]
            │       │           │           ├── <173>  [-]
            │       │           │           │   ├── <170> Var [d5]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <175> Var [i6]
            │       │           │           ├── <180>  [-]
            │       │           │           │   ├── <177> Var [d6]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <182> Var [i7]
            │       │           │           ├── <187>  [-]
            │       │           │           │   ├── <184> Var [d7]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <189> Var [i8]
            │       │           │           ├── <194>  [-]
            │       │           │           │   ├── <191> Var [d8]
            │       │           │           │   ╰── Constant Int [1]
            │       │           │           ├── <196> Var [i9]
            │       │           │           ╰── <201>  [-]
            │       │           │               ├── <198> Var [d9]
            │       │           │               ╰── Constant Int [1]
            │       │           ├── If
            │       │           │   ├── Condition
            │       │           │   │   ╰── <206> Var [call1]
            │       │           │   ╰── Then
            │       │           │       ╰── Block
            │       │           │           ╰── Return
            │       │           │               ╰── <208> Var [call1]
            │       │           ╰── If
            │       │               ├── Condition
            │       │               │   ╰── <214> Var [call2]
            │       │               ╰── Then
            │       │                   ╰── Block
            │       │                       ╰── Return
            │       │                           ╰── <216> Var [call2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <232>  [!=]
            │       │   │       ├── <225> Var [i2]
            │       │   │       ╰── <231>  [+]
            │       │   │           ├── <228> Var [i1]
            │       │   │           ╰── Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <246>  [!=]
            │       │   │       ├── <239> Var [i3]
            │       │   │       ╰── <245>  [+]
            │       │   │           ├── <242> Var [i1]
            │       │   │           ╰── Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <260>  [!=]
            │       │   │       ├── <253> Var [i4]
            │       │   │       ╰── <259>  [+]
            │       │   │           ├── <256> Var [i1]
            │       │   │           ╰── Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <274>  [!=]
            │       │   │       ├── <267> Var [i5]
            │       │   │       ╰── <273>  [+]
            │       │   │           ├── <270> Var [i1]
            │       │   │           ╰── Constant Int [8]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <288>  [!=]
            │       │   │       ├── <281> Var [i6]
            │       │   │       ╰── <287>  [+]
            │       │   │           ├── <284> Var [i1]
            │       │   │           ╰── Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <302>  [!=]
            │       │   │       ├── <295> Var [i7]
            │       │   │       ╰── <301>  [+]
            │       │   │           ├── <298> Var [i1]
            │       │   │           ╰── Constant Int [12]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <316>  [!=]
            │       │   │       ├── <309> Var [i8]
            │       │   │       ╰── <315>  [+]
            │       │   │           ├── <312> Var [i1]
            │       │   │           ╰── Constant Int [14]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <330>  [!=]
            │       │   │       ├── <323> Var [i9]
            │       │   │       ╰── <329>  [+]
            │       │   │           ├── <326> Var [i1]
            │       │   │           ╰── Constant Int [16]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <344>  [!=]
            │       │   │       ├── <337> Var [d1]
            │       │   │       ╰── <343>  [-]
            │       │   │           ├── <340> Var [d9]
            │       │   │           ╰── Constant Int [16]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <358>  [!=]
            │       │   │       ├── <351> Var [d2]
            │       │   │       ╰── <357>  [-]
            │       │   │           ├── <354> Var [d9]
            │       │   │           ╰── Constant Int [14]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [12]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <372>  [!=]
            │       │   │       ├── <365> Var [d3]
            │       │   │       ╰── <371>  [-]
            │       │   │           ├── <368> Var [d9]
            │       │   │           ╰── Constant Int [12]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [13]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <386>  [!=]
            │       │   │       ├── <379> Var [d4]
            │       │   │       ╰── <385>  [-]
            │       │   │           ├── <382> Var [d9]
            │       │   │           ╰── Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [14]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <400>  [!=]
            │       │   │       ├── <393> Var [d5]
            │       │   │       ╰── <399>  [-]
            │       │   │           ├── <396> Var [d9]
            │       │   │           ╰── Constant Int [8]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [15]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <414>  [!=]
            │       │   │       ├── <407> Var [d6]
            │       │   │       ╰── <413>  [-]
            │       │   │           ├── <410> Var [d9]
            │       │   │           ╰── Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [16]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <428>  [!=]
            │       │   │       ├── <421> Var [d7]
            │       │   │       ╰── <427>  [-]
            │       │   │           ├── <424> Var [d9]
            │       │   │           ╰── Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [17]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <442>  [!=]
            │       │   │       ├── <435> Var [d8]
            │       │   │       ╰── <441>  [-]
            │       │   │           ├── <438> Var [d9]
            │       │   │           ╰── Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [18]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <475> FunctionCall [fun]
                            ├── Constant Int [1]
                            ├── Constant Double [+2e0]
                            ├── Constant Int [3]
                            ├── Constant Double [+4e0]
                            ├── Constant Int [5]
                            ├── Constant Double [+6e0]
                            ├── Constant Int [7]
                            ├── Constant Double [+8e0]
                            ├── Constant Int [9]
                            ├── Constant Double [+1e1]
                            ├── Constant Int [11]
                            ├── Constant Double [+1.2e1]
                            ├── Constant Int [13]
                            ├── Constant Double [+1.4e1]
                            ├── Constant Int [15]
                            ├── Constant Double [+1.6e1]
                            ├── Constant Int [17]
                            ╰── Constant Double [+1.8e1]
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
            │           ╰── <51> FunctionCall [check_arguments]
            │               ├── Constant Double [+1e0]
            │               ├── Constant Double [+2e0]
            │               ├── Constant Double [+3e0]
            │               ├── Constant Double [+4e0]
            │               ├── <41> Unary [-]
            │               │   ╰── Constant Double [+1e0]
            │               ├── <44> Unary [-]
            │               │   ╰── Constant Double [+2e0]
            │               ├── <47> Unary [-]
            │               │   ╰── Constant Double [+3e0]
            │               ╰── <50> Unary [-]
            │                   ╰── Constant Double [+4e0]
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
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> Var [a]
                    │   │       ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97>  [!=]
                    │   │       ├── <94> Var [b]
                    │   │       ╰── Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Var [c]
                    │   │       ╰── Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Var [d]
                    │   │       ╰── Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <129>  [!=]
                    │   │       ├── <124> Var [e]
                    │   │       ╰── <128> Unary [-]
                    │   │           ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141>  [!=]
                    │   │       ├── <136> Var [f]
                    │   │       ╰── <140> Unary [-]
                    │   │           ╰── Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <148> Var [g]
                    │   │       ╰── <152> Unary [-]
                    │   │           ╰── Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <165>  [!=]
                    │   │       ├── <160> Var [h]
                    │   │       ╰── <164> Unary [-]
                    │   │           ╰── Constant Double [+4e0]
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
            │       │   │   ╰── <42>  [!=]
            │       │   │       ├── <39> Var [a]
            │       │   │       ╰── Constant Double [+0e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <52>  [!=]
            │       │   │       ├── <49> Var [b]
            │       │   │       ╰── Constant Double [+1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <62>  [!=]
            │       │   │       ├── <59> Var [c]
            │       │   │       ╰── Constant Double [+2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <72>  [!=]
            │       │   │       ├── <69> Var [d]
            │       │   │       ╰── Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <82>  [!=]
            │       │   │       ├── <79> Var [e]
            │       │   │       ╰── Constant Double [+4e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <92>  [!=]
            │       │   │       ├── <89> Var [f]
            │       │   │       ╰── Constant Double [+5e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <102>  [!=]
            │       │   │       ├── <99> Var [g]
            │       │   │       ╰── Constant Double [+6e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <112>  [!=]
            │       │   │       ├── <109> Var [h]
            │       │   │       ╰── Constant Double [+7e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <122>  [!=]
            │       │   │       ├── <119> Var [i]
            │       │   │       ╰── Constant Double [+8e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <132>  [!=]
            │       │   │       ├── <129> Var [j]
            │       │   │       ╰── Constant Double [+9e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <142>  [!=]
            │       │   │       ├── <139> Var [k]
            │       │   │       ╰── Constant Double [+1e1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [11]
            │       ╰── Return
            │           ╰── Constant Int [0]
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
            │           ╰── <203> FunctionCall [callee]
            │               ├── Constant Double [+0e0]
            │               ├── Constant Double [+1e0]
            │               ├── Constant Double [+2e0]
            │               ├── Constant Double [+3e0]
            │               ├── Constant Double [+4e0]
            │               ├── Constant Double [+5e0]
            │               ├── <182>  [+]
            │               │   ├── <179> Var [e]
            │               │   ╰── Constant Double [+1e0]
            │               ├── <187>  [+]
            │               │   ├── <184> Var [d]
            │               │   ╰── Constant Double [+3e0]
            │               ├── <192>  [+]
            │               │   ├── <189> Var [c]
            │               │   ╰── Constant Double [+5e0]
            │               ├── <197>  [+]
            │               │   ├── <194> Var [b]
            │               │   ╰── Constant Double [+7e0]
            │               ╰── <202>  [+]
            │                   ├── <199> Var [a]
            │                   ╰── Constant Double [+9e0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <217> FunctionCall [target]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ├── Constant Int [4]
                            ╰── Constant Int [5]
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
            │           ╰── Constant Double [+1.234e78]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <17> FunctionCall [d]
                    ╰── Return
                        ╰── <24>  [==]
                            ├── <21> Var [retval]
                            ╰── Constant Double [+1.234e78]
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
                    │       ╰── <37> FunctionCall [fma]
                    │           ├── Constant Double [+5e0]
                    │           ├── Constant Double [+1e22]
                    │           ╰── Constant Double [+4e6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ldexp_result
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <46> FunctionCall [ldexp]
                    │           ├── Constant Double [+9.2e74]
                    │           ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> Var [fma_result]
                    │   │       ╰── Constant Double [+5.0000000000000004e22]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Var [ldexp_result]
                    │   │       ╰── Constant Double [+2.944e76]
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
            │           │   ╰── <12>  [>]
            │           │       ├── <9> Var [x]
            │           │       ╰── Constant Int [2]
            │           ├── Then
            │           │   ╰── Return
            │           │       ╰── <14> Var [x]
            │           ╰── Else
            │               ╰── Block
            │                   ├── VarDeclaration
            │                   │   ├── Name
            │                   │   │   ╰── ret
            │                   │   ├── Type
            │                   │   │   ╰── Double
            │                   │   ╰── Initializer
            │                   │       ╰── <25> FunctionCall [fun]
            │                   │           ╰── <24>  [+]
            │                   │               ├── <21> Var [x]
            │                   │               ╰── Constant Int [2]
            │                   ╰── Return
            │                       ╰── <33>  [+]
            │                           ├── <29> Var [ret]
            │                           ╰── <32> Var [x]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <46> FunctionCall [fun]
                            ╰── Constant Double [+1e0]
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
            │           ╰── <16>  [<]
            │               ├── <12> Var [d]
            │               ╰── <15> Var [l]
            ├── Function [tern_double_flag]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── flag
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <37> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <{node_id}> Conditional [?]
            │                       ├── <30> Var [flag]
            │                       ├── Then
            │                       │   ╰── <33> Unary [-]
            │                       │       ╰── Constant Int [30]
            │                       ╰── Else
            │                           ╰── Constant ULong [10]
            ├── Function [tern_double_result]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── flag
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <{node_id}> Conditional [?]
            │               ├── <49> Var [flag]
            │               ├── Then
            │               │   ╰── Constant Double [+5e0]
            │               ╰── Else
            │                   ╰── Constant ULong [9223372036854777850]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ten
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant Int [10]
            ├── Function [multiply]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <73>  [*]
            │       │           ├── Constant Double [+1.075e1]
            │       │           ╰── <72> Var [ten]
            │       ╰── Return
            │           ╰── <80>  [==]
            │               ├── <77> Var [i]
            │               ╰── Constant Int [107]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95> FunctionCall [lt]
                    │   │       ├── <91> Unary [-]
                    │   │       │   ╰── Constant Double [+9.007199254751228e15]
                    │   │       ╰── <94> Unary [-]
                    │   │           ╰── Constant Long [9007199254751227]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [!=]
                    │   │       ├── <103> FunctionCall [tern_double_flag]
                    │   │       │   ╰── Constant Double [+2e1]
                    │   │       ╰── Constant Double [+1.8446744073709552e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> FunctionCall [tern_double_flag]
                    │   │       │   ╰── Constant Double [+0e0]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125> FunctionCall [tern_double_result]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <139>  [!=]
                    │   │       ├── <136> FunctionCall [tern_double_result]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Double [+9.223372036854778e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148> Unary [!]
                    │   │       ╰── <147> FunctionCall [multiply]
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
            │       ╰── Constant ULong [10000]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Unary [-]
                    │           ╰── Constant Int [50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <31>  [*]
                    │           ├── <28>  [+]
                    │           │   ├── <23> Var [ul]
                    │           │   ╰── <26> Var [i]
                    │           ╰── Constant Double [+3.125e0]
                    ╰── Return
                        ╰── <38>  [==]
                            ├── <35> Var [d]
                            ╰── Constant Double [+3.109375e4]
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
            │           ╰── <24>  [&&]
            │               ├── <15>  [==]
            │               │   ├── <12> Var [l]
            │               │   ╰── Constant Int [2]
            │               ╰── <23>  [==]
            │                   ├── <18> Var [d]
            │                   ╰── <22> Unary [-]
            │                       ╰── Constant Double [+6e0]
            ├── Function [return_double]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant ULong [18446744073709551586]
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
            │       │       ╰── Constant Int [0]
            │       ├── <54> Assign [=]
            │       │   ├── <50> Var [i]
            │       │   ╰── <53> Var [arg]
            │       ╰── Return
            │           ╰── <60>  [==]
            │               ├── <57> Var [i]
            │               ╰── Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75> Unary [!]
                    │   │       ╰── <74> FunctionCall [check_args]
                    │   │           ├── Constant Double [+2.4e0]
                    │   │           ╰── <73> Unary [-]
                    │   │               ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> FunctionCall [return_double]
                    │   │       ╰── Constant Double [+1.8446744073709552e19]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95> Unary [!]
                    │   │       ╰── <94> FunctionCall [check_assignment]
                    │   │           ╰── Constant Double [+4.9e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant ULong [18446744073709551586]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <108> Var [d]
                    │   │       ╰── Constant Double [+1.8446744073709552e19]
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
            │       ╰── Constant Int [2147483647]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d2
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant UInt [4294967295]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d3
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Long [4611686018427389440]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d4
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Long [4611686018427389955]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d5
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372036854775810]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d6
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant ULong [4611686018427389955]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d7
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372036854776832]
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
            │   │   ╰── Constant Double [+4.9e0]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── u
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Constant Double [+4.2949672923e9]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Double [+4.61168601842739e18]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Constant Double [+1.844674407370955e19]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Var [d1]
                    │   │       ╰── Constant Double [+2.147483647e9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [d2]
                    │   │       ╰── Constant Double [+4.294967295e9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [d3]
                    │   │       ╰── Constant Double [+4.61168601842739e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <107> Var [d4]
                    │   │       ╰── <110> Var [d3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121>  [!=]
                    │   │       ├── <118> Var [d5]
                    │   │       ╰── Constant Double [+9.223372036854776e18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <128> Var [d6]
                    │   │       ╰── <131> Var [d3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143>  [!=]
                    │   │       ├── <139> Var [d7]
                    │   │       ╰── <142> Var [d5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <150> Var [uninitialized]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <160>  [!=]
                    │   │       ├── <157> Var [i]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <170>  [!=]
                    │   │       ├── <167> Var [u]
                    │   │       ╰── Constant UInt [4294967292]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <177> Var [l]
                    │   │       ╰── Constant Long [4611686018427389952]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190>  [!=]
                    │   │       ├── <187> Var [ul]
                    │   │       ╰── Constant ULong [18446744073709549568]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <60> Var [i1]
                    │   │       ╰── <63> Var [d9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── call1
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <132> FunctionCall [fun]
                    │           │           ├── <73>  [+]
                    │           │           │   ├── <70> Var [i1]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <75> Var [d1]
                    │           │           ├── <80>  [+]
                    │           │           │   ├── <77> Var [i2]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <82> Var [d2]
                    │           │           ├── <87>  [+]
                    │           │           │   ├── <84> Var [i3]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <89> Var [d3]
                    │           │           ├── <94>  [+]
                    │           │           │   ├── <91> Var [i4]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <96> Var [d4]
                    │           │           ├── <101>  [+]
                    │           │           │   ├── <98> Var [i5]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <103> Var [d5]
                    │           │           ├── <108>  [+]
                    │           │           │   ├── <105> Var [i6]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <110> Var [d6]
                    │           │           ├── <115>  [+]
                    │           │           │   ├── <112> Var [i7]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <117> Var [d7]
                    │           │           ├── <122>  [+]
                    │           │           │   ├── <119> Var [i8]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <124> Var [d8]
                    │           │           ├── <129>  [+]
                    │           │           │   ├── <126> Var [i9]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ╰── <131> Var [d9]
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── call2
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <202> FunctionCall [fun]
                    │           │           ├── <140> Var [i1]
                    │           │           ├── <145>  [-]
                    │           │           │   ├── <142> Var [d1]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <147> Var [i2]
                    │           │           ├── <152>  [-]
                    │           │           │   ├── <149> Var [d2]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <154> Var [i3]
                    │           │           ├── <159>  [-]
                    │           │           │   ├── <156> Var [d3]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <161> Var [i4]
                    │           │           ├── <166>  [-]
                    │           │           │   ├── <163> Var [d4]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <168> Var [i5]
                    │           │           ├── <173>  [-]
                    │           │           │   ├── <170> Var [d5]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <175> Var [i6]
                    │           │           ├── <180>  [-]
                    │           │           │   ├── <177> Var [d6]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <182> Var [i7]
                    │           │           ├── <187>  [-]
                    │           │           │   ├── <184> Var [d7]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <189> Var [i8]
                    │           │           ├── <194>  [-]
                    │           │           │   ├── <191> Var [d8]
                    │           │           │   ╰── Constant Int [1]
                    │           │           ├── <196> Var [i9]
                    │           │           ╰── <201>  [-]
                    │           │               ├── <198> Var [d9]
                    │           │               ╰── Constant Int [1]
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <206> Var [call1]
                    │           │   ╰── Then
                    │           │       ╰── Block
                    │           │           ╰── Return
                    │           │               ╰── <208> Var [call1]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <214> Var [call2]
                    │               ╰── Then
                    │                   ╰── Block
                    │                       ╰── Return
                    │                           ╰── <216> Var [call2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <232>  [!=]
                    │   │       ├── <225> Var [i2]
                    │   │       ╰── <231>  [+]
                    │   │           ├── <228> Var [i1]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <246>  [!=]
                    │   │       ├── <239> Var [i3]
                    │   │       ╰── <245>  [+]
                    │   │           ├── <242> Var [i1]
                    │   │           ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <260>  [!=]
                    │   │       ├── <253> Var [i4]
                    │   │       ╰── <259>  [+]
                    │   │           ├── <256> Var [i1]
                    │   │           ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <274>  [!=]
                    │   │       ├── <267> Var [i5]
                    │   │       ╰── <273>  [+]
                    │   │           ├── <270> Var [i1]
                    │   │           ╰── Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <288>  [!=]
                    │   │       ├── <281> Var [i6]
                    │   │       ╰── <287>  [+]
                    │   │           ├── <284> Var [i1]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <302>  [!=]
                    │   │       ├── <295> Var [i7]
                    │   │       ╰── <301>  [+]
                    │   │           ├── <298> Var [i1]
                    │   │           ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <316>  [!=]
                    │   │       ├── <309> Var [i8]
                    │   │       ╰── <315>  [+]
                    │   │           ├── <312> Var [i1]
                    │   │           ╰── Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <330>  [!=]
                    │   │       ├── <323> Var [i9]
                    │   │       ╰── <329>  [+]
                    │   │           ├── <326> Var [i1]
                    │   │           ╰── Constant Int [16]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <344>  [!=]
                    │   │       ├── <337> Var [d1]
                    │   │       ╰── <343>  [-]
                    │   │           ├── <340> Var [d9]
                    │   │           ╰── Constant Int [16]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <358>  [!=]
                    │   │       ├── <351> Var [d2]
                    │   │       ╰── <357>  [-]
                    │   │           ├── <354> Var [d9]
                    │   │           ╰── Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <372>  [!=]
                    │   │       ├── <365> Var [d3]
                    │   │       ╰── <371>  [-]
                    │   │           ├── <368> Var [d9]
                    │   │           ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <386>  [!=]
                    │   │       ├── <379> Var [d4]
                    │   │       ╰── <385>  [-]
                    │   │           ├── <382> Var [d9]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <400>  [!=]
                    │   │       ├── <393> Var [d5]
                    │   │       ╰── <399>  [-]
                    │   │           ├── <396> Var [d9]
                    │   │           ╰── Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <414>  [!=]
                    │   │       ├── <407> Var [d6]
                    │   │       ╰── <413>  [-]
                    │   │           ├── <410> Var [d9]
                    │   │           ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [16]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <428>  [!=]
                    │   │       ├── <421> Var [d7]
                    │   │       ╰── <427>  [-]
                    │   │           ├── <424> Var [d9]
                    │   │           ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [17]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <442>  [!=]
                    │   │       ├── <435> Var [d8]
                    │   │       ╰── <441>  [-]
                    │   │           ├── <438> Var [d9]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [18]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <86> FunctionCall [fun]
                    │           ├── Constant Int [1]
                    │           ├── Constant Double [+2e0]
                    │           ├── Constant Int [3]
                    │           ├── Constant Double [+4e0]
                    │           ├── Constant Int [5]
                    │           ├── Constant Double [+6e0]
                    │           ├── Constant Int [7]
                    │           ├── Constant Double [+8e0]
                    │           ├── Constant Int [9]
                    │           ├── Constant Double [+1e1]
                    │           ├── Constant Int [11]
                    │           ├── Constant Double [+1.2e1]
                    │           ├── Constant Int [13]
                    │           ├── Constant Double [+1.4e1]
                    │           ├── Constant Int [15]
                    │           ├── Constant Double [+1.6e1]
                    │           ├── Constant Int [17]
                    │           ╰── Constant Double [+1.8e1]
                    ╰── Return
                        ╰── <94>  [==]
                            ├── <90> Var [d]
                            ╰── Constant Double [+7.8e1]
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
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Var [a]
                    │   │       ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Var [b]
                    │   │       ╰── Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> Var [c]
                    │   │       ╰── Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Var [d]
                    │   │       ╰── Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <70> Var [e]
                    │   │       ╰── <74> Unary [-]
                    │   │           ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <82> Var [f]
                    │   │       ╰── <86> Unary [-]
                    │   │           ╰── Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <94> Var [g]
                    │   │       ╰── <98> Unary [-]
                    │   │           ╰── Constant Double [+3e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <106> Var [h]
                    │   │       ╰── <110> Unary [-]
                    │   │           ╰── Constant Double [+4e0]
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
                        ╰── <51> FunctionCall [check_arguments]
                            ├── Constant Double [+1e0]
                            ├── Constant Double [+2e0]
                            ├── Constant Double [+3e0]
                            ├── Constant Double [+4e0]
                            ├── <41> Unary [-]
                            │   ╰── Constant Double [+1e0]
                            ├── <44> Unary [-]
                            │   ╰── Constant Double [+2e0]
                            ├── <47> Unary [-]
                            │   ╰── Constant Double [+3e0]
                            ╰── <50> Unary [-]
                                ╰── Constant Double [+4e0]
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
                    │       ╰── <93> FunctionCall [fmax]
                    │           ├── <82> FunctionCall [fmax]
                    │           │   ├── <67> FunctionCall [fmax]
                    │           │   │   ├── <60> FunctionCall [fmax]
                    │           │   │   │   ├── <57> Var [a]
                    │           │   │   │   ╰── <59> Var [b]
                    │           │   │   ╰── <66> FunctionCall [fmax]
                    │           │   │       ├── <63> Var [c]
                    │           │   │       ╰── <65> Var [d]
                    │           │   ╰── <81> FunctionCall [fmax]
                    │           │       ├── <74> FunctionCall [fmax]
                    │           │       │   ├── <71> Var [e]
                    │           │       │   ╰── <73> Var [f]
                    │           │       ╰── <80> FunctionCall [fmax]
                    │           │           ├── <77> Var [g]
                    │           │           ╰── <79> Var [h]
                    │           ╰── <92> FunctionCall [fmax]
                    │               ├── <85> Var [i]
                    │               ╰── <91> FunctionCall [fmax]
                    │                   ├── <88> Var [j]
                    │                   ╰── <90> Var [k]
                    ╰── Return
                        ╰── <97> Var [max]
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
                    │       ╰── <63> FunctionCall [get_max]
                    │           ├── Constant Double [+1.003e2]
                    │           ├── Constant Double [+2.001e2]
                    │           ├── Constant Double [+1e-2]
                    │           ├── Constant Double [+1.00004e5]
                    │           ├── Constant Double [+5.5555e1]
                    │           ├── <54> Unary [-]
                    │           │   ╰── Constant Double [+4e0]
                    │           ├── Constant Double [+6.5432e3]
                    │           ├── Constant Double [+9e9]
                    │           ├── Constant Double [+8e8]
                    │           ├── Constant Double [+7.6e0]
                    │           ╰── <62>  [*]
                    │               ├── Constant Double [+1e4]
                    │               ╰── Constant Double [+1.1e6]
                    ╰── Return
                        ╰── <73>  [==]
                            ├── <67> Var [result]
                            ╰── <72>  [*]
                                ├── Constant Double [+1e4]
                                ╰── Constant Double [+1.1e6]
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
                    ╰── Constant Double [+1e20]
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
                        ╰── <14>  [==]
                            ├── <11> Var [d]
                            ╰── Constant Double [+1e20]
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
                        │   ╰── <12>  [>]
                        │       ├── <9> Var [x]
                        │       ╰── Constant Int [2]
                        ├── Then
                        │   ╰── Return
                        │       ╰── <14> Var [x]
                        ╰── Else
                            ╰── Block
                                ├── VarDeclaration
                                │   ├── Name
                                │   │   ╰── ret
                                │   ├── Type
                                │   │   ╰── Double
                                │   ╰── Initializer
                                │       ╰── <25> FunctionCall [fun]
                                │           ╰── <24>  [+]
                                │               ├── <21> Var [x]
                                │               ╰── Constant Int [2]
                                ╰── Return
                                    ╰── <33>  [+]
                                        ├── <29> Var [ret]
                                        ╰── <32> Var [x]
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
                        ╰── <15> FunctionCall [fun]
                            ╰── Constant Double [+1e0]
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
            │       ╰── Constant Double [+inf]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── very_large
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+1.79e308]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero
            │   ├── Type
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Double [+0e0]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Var [inf]
                    │   │       ╰── Constant Double [+inf]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [<=]
                    │   │       ├── <34> Var [inf]
                    │   │       ╰── <37> Var [very_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <48>  [*]
                    │   │       │   ├── <45> Var [very_large]
                    │   │       │   ╰── Constant Double [+1e1]
                    │   │       ╰── <51> Var [inf]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <62>  [/]
                    │   │       │   ├── Constant Double [+1e0]
                    │   │       │   ╰── <61> Var [zero]
                    │   │       ╰── <65> Var [inf]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negated_inf
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <78> Unary [-]
                    │           ╰── <77> Var [inf]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negated_inf2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <90>  [/]
                    │           ├── <86> Unary [-]
                    │           │   ╰── Constant Double [+1e0]
                    │           ╰── <89> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [>=]
                    │   │       ├── <94> Var [negated_inf]
                    │   │       ╰── <99> Unary [-]
                    │   │           ╰── <98> Var [very_large]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <107> Var [negated_inf]
                    │   │       ╰── <110> Var [negated_inf2]
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
            │       ╰── Constant Double [+0e0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negative_zero
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <28> Unary [-]
                    │           ╰── <27> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> Var [negative_zero]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <43>  [/]
                    │   │       │   ├── Constant Int [1]
                    │   │       │   ╰── <42> Var [negative_zero]
                    │   │       ╰── <47> Unary [-]
                    │   │           ╰── Constant Double [+inf]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59>  [/]
                    │   │       │   ├── <55> Unary [-]
                    │   │       │   │   ╰── Constant Int [10]
                    │   │       │   ╰── <58> Var [negative_zero]
                    │   │       ╰── Constant Double [+inf]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── fail
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <81>  [&&]
                    │   ├── <73> Var [negative_zero]
                    │   ╰── <80> Assign [=]
                    │       ├── <76> Var [fail]
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84> Var [fail]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89> Var [negative_zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <96> Var [zero]
                    │   │       ╰── <100> Unary [-]
                    │   │           ╰── Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negated
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <115> FunctionCall [copysign]
                    │           ├── Constant Double [+4e0]
                    │           ╰── <114> Unary [-]
                    │               ╰── Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── positive
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <126> FunctionCall [copysign]
                    │           ├── <124> Unary [-]
                    │           │   ╰── Constant Double [+5e0]
                    │           ╰── Constant Double [+0e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135>  [!=]
                    │   │       ├── <130> Var [negated]
                    │   │       ╰── <134> Unary [-]
                    │   │           ╰── Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [!=]
                    │   │       ├── <142> Var [positive]
                    │   │       ╰── Constant Double [+5e0]
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
            │           ╰── <11> Unary [!]
            │               ╰── <10> Var [d]
            ├── Function [multiply_by_large_num]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <26>  [*]
            │               ├── <23> Var [d]
            │               ╰── Constant Double [+2e20]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subnormal
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+2.5e-320]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <43> FunctionCall [multiply_by_large_num]
                    │   │       │   ╰── <42> Var [subnormal]
                    │   │       ╰── Constant Double [+4.999944335913415e-300]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ╰── Return
                        ╰── <55> FunctionCall [non_zero]
                            ╰── <54> Var [subnormal]
    "#;
    assert_parse(src, expected);
}
