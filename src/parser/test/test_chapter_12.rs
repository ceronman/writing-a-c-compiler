use super::{assert_error, assert_parse};

#[test]
fn test_invalid_labels_extra_credit_switch_duplicate_cases() {
    let src = r#"
        int main(void) {
            unsigned int ui = 10u;
            switch(ui) {
                case 4294967295u:
                    return 0;
                case 1099511627775l:
                    return 1;
                default: return 2;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant UInt [10]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [ui]
                        ╰── Block
                            ├── Case [4294967295]
                            │   ╰── Return
                            │       ╰── <15> Constant Int [0]
                            ├── Case [1099511627775]
                            │   ╰── Return
                            │       ╰── <19> Constant Int [1]
                            ╰── Default
                                ╰── Return
                                    ╰── <22> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_parse_bad_specifiers() {
    assert_error(
        r#"
        int main(void) {
            int i = 0;
            return (signed unsigned) i;
                  //^^^^^^^^^^^^^^^ Invalid type specifier
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_bad_specifiers_2() {
    assert_error(
        r#"
        int main(void) {
            unsigned long unsigned i = 0;
          //^^^^^^^^^^^^^^^^^^^^^^ Invalid type specifier
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_signed_unsigned() {
    let src = r#"
        unsigned x;
        int x;
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_uint_ulong() {
    let src = r#"
        
        unsigned int foo(void);
        unsigned long foo(void) {
            return 0;
        }
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <21> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_chained_casts() {
    let src = r#"
        unsigned int ui = 4294967200u;
        int main(void) {
            if ((long) (signed) ui != -96l)
                return 1;
            if ((unsigned long) (signed) ui != 18446744073709551520ul)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <4> Constant UInt [4294967200]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <24>  [!=]
                    │   │       ├── <19> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <18> Cast
                    │   │       │           ├── Target
                    │   │       │           │   ╰── Int
                    │   │       │           ╰── Expression
                    │   │       │               ╰── <17> Var [ui]
                    │   │       ╰── <23> Unary [-]
                    │   │           ╰── <22> Constant Long [96]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <25> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <34> Cast
                    │   │       │           ├── Target
                    │   │       │           │   ╰── Int
                    │   │       │           ╰── Expression
                    │   │       │               ╰── <33> Var [ui]
                    │   │       ╰── <37> Constant ULong [18446744073709551520]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <39> Constant Int [2]
                    ╰── Return
                        ╰── <42> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_extension() {
    let src = r#"
        int int_to_ulong(int i, unsigned long expected) {
            unsigned long result = (unsigned long) i;
            return result == expected;
        }
        int uint_to_long(unsigned int ui, long expected) {
            long result = (long) ui;
            return result == expected;
        }
        int uint_to_ulong(unsigned ui, unsigned long expected){
            return (unsigned long) ui == expected;
        }
        int main(void) {
            if (!int_to_ulong(10, 10ul)) {
                return 1;
            }
            if (!int_to_ulong(-10, 18446744073709551606ul)) {
                return 2;
            }
            if (!uint_to_long(4294967200u, 4294967200l)) {
                return 3;
            }
            if (!uint_to_ulong(4294967200u, 4294967200ul)) {
                return 4;
            }
            if ((unsigned long) 4294967200u != 4294967200ul) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [int_to_ulong]
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
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <19> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <18> Var [i]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <23> Var [result]
            │               ╰── <26> Var [expected]
            ├── Function [uint_to_long]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ui
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <49> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <48> Var [ui]
            │       ╰── Return
            │           ╰── <57>  [==]
            │               ├── <53> Var [result]
            │               ╰── <56> Var [expected]
            ├── Function [uint_to_ulong]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ui
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <80>  [==]
            │               ├── <76> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Long
            │               │   ╰── Expression
            │               │       ╰── <75> Var [ui]
            │               ╰── <79> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94> Unary [!]
                    │   │       ╰── <93> FunctionCall [int_to_ulong]
                    │   │           ├── <91> Constant Int [10]
                    │   │           ╰── <92> Constant ULong [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <95> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107> Unary [!]
                    │   │       ╰── <106> FunctionCall [int_to_ulong]
                    │   │           ├── <104> Unary [-]
                    │   │           │   ╰── <103> Constant Int [10]
                    │   │           ╰── <105> Constant ULong [18446744073709551606]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118> Unary [!]
                    │   │       ╰── <117> FunctionCall [uint_to_long]
                    │   │           ├── <115> Constant UInt [4294967200]
                    │   │           ╰── <116> Constant Long [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <119> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <129> Unary [!]
                    │   │       ╰── <128> FunctionCall [uint_to_ulong]
                    │   │           ├── <126> Constant UInt [4294967200]
                    │   │           ╰── <127> Constant ULong [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <130> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141>  [!=]
                    │   │       ├── <138> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <137> Constant UInt [4294967200]
                    │   │       ╰── <140> Constant ULong [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <142> Constant Int [5]
                    ╰── Return
                        ╰── <147> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_rewrite_movz_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        unsigned glob = 5000u;
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
            int thirteen = glob - 4987u;
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
            if (should_spill != 5000l) {
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
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <49> Constant UInt [5000]
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
            │       │           ╰── <227> Constant UInt [4987]
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
            │       │   │       ╰── <375> Constant Long [5000]
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
fn test_valid_explicit_casts_round_trip_casts() {
    let src = r#"
        unsigned long a = 8589934580ul;
        int main(void) {
            unsigned long b = (unsigned long) (unsigned int) a;
            if (b != 4294967284ul)
                return 1;
            b = (unsigned long) (signed int) a;
            if (b != 18446744073709551604ul)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <4> Constant ULong [8589934580]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <22> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <21> Cast
                    │                   ├── Target
                    │                   │   ╰── Unsigned Int
                    │                   ╰── Expression
                    │                       ╰── <20> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Var [b]
                    │   │       ╰── <28> Constant ULong [4294967284]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <30> Constant Int [1]
                    ├── <44> Assign [=]
                    │   ├── <34> Var [b]
                    │   ╰── <43> Cast
                    │       ├── Target
                    │       │   ╰── Unsigned Long
                    │       ╰── Expression
                    │           ╰── <42> Cast
                    │               ├── Target
                    │               │   ╰── Int
                    │               ╰── Expression
                    │                   ╰── <41> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Var [b]
                    │   │       ╰── <49> Constant ULong [18446744073709551604]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <51> Constant Int [2]
                    ╰── Return
                        ╰── <54> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_same_size_conversion() {
    let src = r#"
        int uint_to_int(unsigned int ui, int expected) {
            return (int) ui == expected;
        }
        int int_to_uint(int i, unsigned int expected) {
            return (unsigned int) i == expected;
        }
        int ulong_to_long(unsigned long ul, signed long expected) {
            return (signed long) ul == expected;
        }
        int long_to_ulong(long l, unsigned long expected) {
            return (unsigned long) l == expected;
        }
        int main(void) {
            if (!int_to_uint(10, 10u)) {
                return 1;
            }
            if (!uint_to_int(10u, 10)) {
                return 2;
            }
            if (!long_to_ulong(-1000l, 18446744073709550616ul)) {
                return 3;
            }
            if (!ulong_to_long(18446744073709550616ul, -1000l)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [uint_to_int]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ui
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <20>  [==]
            │               ├── <16> Cast
            │               │   ├── Target
            │               │   │   ╰── Int
            │               │   ╰── Expression
            │               │       ╰── <15> Var [ui]
            │               ╰── <19> Var [expected]
            ├── Function [int_to_uint]
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
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <43>  [==]
            │               ├── <39> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Int
            │               │   ╰── Expression
            │               │       ╰── <38> Var [i]
            │               ╰── <42> Var [expected]
            ├── Function [ulong_to_long]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ul
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <66>  [==]
            │               ├── <62> Cast
            │               │   ├── Target
            │               │   │   ╰── Long
            │               │   ╰── Expression
            │               │       ╰── <61> Var [ul]
            │               ╰── <65> Var [expected]
            ├── Function [long_to_ulong]
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
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <89>  [==]
            │               ├── <85> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Long
            │               │   ╰── Expression
            │               │       ╰── <84> Var [l]
            │               ╰── <88> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103> Unary [!]
                    │   │       ╰── <102> FunctionCall [int_to_uint]
                    │   │           ├── <100> Constant Int [10]
                    │   │           ╰── <101> Constant UInt [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114> Unary [!]
                    │   │       ╰── <113> FunctionCall [uint_to_int]
                    │   │           ├── <111> Constant UInt [10]
                    │   │           ╰── <112> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127> Unary [!]
                    │   │       ╰── <126> FunctionCall [long_to_ulong]
                    │   │           ├── <124> Unary [-]
                    │   │           │   ╰── <123> Constant Long [1000]
                    │   │           ╰── <125> Constant ULong [18446744073709550616]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <128> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140> Unary [!]
                    │   │       ╰── <139> FunctionCall [ulong_to_long]
                    │   │           ├── <135> Constant ULong [18446744073709550616]
                    │   │           ╰── <138> Unary [-]
                    │   │               ╰── <137> Constant Long [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <141> Constant Int [4]
                    ╰── Return
                        ╰── <146> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_explicit_casts_truncate() {
    let src = r#"
        
        int ulong_to_int(unsigned long ul, int expected) {
            int result = (int) ul;
            return (result == expected);
        }
        int ulong_to_uint(unsigned long ul, unsigned expected) {
            return ((unsigned int) ul == expected);
        }
        int long_to_uint(long l, unsigned int expected) {
            return (unsigned int) l == expected;
        }
        int main(void) {
            if (!long_to_uint(100l, 100u)) {
                return 1;
            }
            if (!long_to_uint(-9223372036854774574l, 1234u)) {
                return 2;
            }
            if (!ulong_to_int(100ul, 100)) {
                return 3;
            }
            if (!ulong_to_uint(100ul, 100u)) {
                return 4;
            }
            if (!ulong_to_uint(4294967200ul, 4294967200u)) {
                return 5;
            }
            if (!ulong_to_int(4294967200ul, -96)) {
                return 6;
            }
            if (!ulong_to_uint(1152921506754330624ul, 2147483648u)) {
                return 7;
            }
            if (!ulong_to_int(1152921506754330624ul, -2147483648)){
                return 8;
            }
            unsigned int ui = (unsigned int)17179869189ul;
            if (ui != 5)
                return 9;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [ulong_to_int]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ul
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
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
            │       │               ╰── <18> Var [ul]
            │       ╰── Return
            │           ╰── <28>  [==]
            │               ├── <23> Var [result]
            │               ╰── <26> Var [expected]
            ├── Function [ulong_to_uint]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ul
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <52>  [==]
            │               ├── <47> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Int
            │               │   ╰── Expression
            │               │       ╰── <46> Var [ul]
            │               ╰── <50> Var [expected]
            ├── Function [long_to_uint]
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
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <75>  [==]
            │               ├── <71> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Int
            │               │   ╰── Expression
            │               │       ╰── <70> Var [l]
            │               ╰── <74> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89> Unary [!]
                    │   │       ╰── <88> FunctionCall [long_to_uint]
                    │   │           ├── <86> Constant Long [100]
                    │   │           ╰── <87> Constant UInt [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102> Unary [!]
                    │   │       ╰── <101> FunctionCall [long_to_uint]
                    │   │           ├── <99> Unary [-]
                    │   │           │   ╰── <98> Constant Long [9223372036854774574]
                    │   │           ╰── <100> Constant UInt [1234]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <103> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113> Unary [!]
                    │   │       ╰── <112> FunctionCall [ulong_to_int]
                    │   │           ├── <110> Constant ULong [100]
                    │   │           ╰── <111> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <114> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124> Unary [!]
                    │   │       ╰── <123> FunctionCall [ulong_to_uint]
                    │   │           ├── <121> Constant ULong [100]
                    │   │           ╰── <122> Constant UInt [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <125> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135> Unary [!]
                    │   │       ╰── <134> FunctionCall [ulong_to_uint]
                    │   │           ├── <132> Constant ULong [4294967200]
                    │   │           ╰── <133> Constant UInt [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <136> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148> Unary [!]
                    │   │       ╰── <147> FunctionCall [ulong_to_int]
                    │   │           ├── <143> Constant ULong [4294967200]
                    │   │           ╰── <146> Unary [-]
                    │   │               ╰── <145> Constant Int [96]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <149> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <159> Unary [!]
                    │   │       ╰── <158> FunctionCall [ulong_to_uint]
                    │   │           ├── <156> Constant ULong [1152921506754330624]
                    │   │           ╰── <157> Constant UInt [2147483648]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <160> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172> Unary [!]
                    │   │       ╰── <171> FunctionCall [ulong_to_int]
                    │   │           ├── <167> Constant ULong [1152921506754330624]
                    │   │           ╰── <170> Unary [-]
                    │   │               ╰── <169> Constant Long [2147483648]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <173> Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <184> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Int
                    │           ╰── Expression
                    │               ╰── <183> Constant ULong [17179869189]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <191>  [!=]
                    │   │       ├── <188> Var [ui]
                    │   │       ╰── <190> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <192> Constant Int [9]
                    ╰── Return
                        ╰── <195> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_unsigned_ops() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            unsigned long ul = 9223372036854775808ul;
            if ((ui & ul) != 0)
                return 1;
            if ((ui | ul) != 9223372041149743103ul)
                return 2;
            signed int i = -1;
            if ((i & ul) != ul)
                return 3;
            if ((i | ul) != i)
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
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant UInt [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <17> Constant ULong [9223372036854775808]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26>  [&]
                    │   │       │   ├── <21> Var [ui]
                    │   │       │   ╰── <24> Var [ul]
                    │   │       ╰── <28> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <30> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [!=]
                    │   │       ├── <39>  [|]
                    │   │       │   ├── <34> Var [ui]
                    │   │       │   ╰── <37> Var [ul]
                    │   │       ╰── <41> Constant ULong [9223372041149743103]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <43> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <51> Unary [-]
                    │           ╰── <50> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <60>  [&]
                    │   │       │   ├── <55> Var [i]
                    │   │       │   ╰── <58> Var [ul]
                    │   │       ╰── <63> Var [ul]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <65> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <74>  [|]
                    │   │       │   ├── <69> Var [i]
                    │   │       │   ╰── <72> Var [ul]
                    │   │       ╰── <77> Var [i]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <79> Constant Int [4]
                    ╰── Return
                        ╰── <82> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_unsigned_shift() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            if ((ui << 2l) != 4294967292) {
                return 1;
            }
            if ((ui >> 2) != 1073741823) {
                return 2;
            }
            static int shiftcount = 5;
            if ((1000000u >> shiftcount) != 31250) {
                return 3;
            }
            if ((1000000u << shiftcount) != 32000000) {
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
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant UInt [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19>  [<<]
                    │   │       │   ├── <15> Var [ui]
                    │   │       │   ╰── <17> Constant Long [2]
                    │   │       ╰── <21> Constant Long [4294967292]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <33>  [>>]
                    │   │       │   ├── <29> Var [ui]
                    │   │       │   ╰── <31> Constant Int [2]
                    │   │       ╰── <35> Constant Int [1073741823]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <37> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shiftcount
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <46> Constant Int [5]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54>  [>>]
                    │   │       │   ├── <49> Constant UInt [1000000]
                    │   │       │   ╰── <52> Var [shiftcount]
                    │   │       ╰── <56> Constant Int [31250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <68>  [<<]
                    │   │       │   ├── <63> Constant UInt [1000000]
                    │   │       │   ╰── <66> Var [shiftcount]
                    │   │       ╰── <70> Constant Int [32000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <72> Constant Int [4]
                    ╰── Return
                        ╰── <77> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_uint() {
    let src = r#"
        int main(void) {
            unsigned int x = -1u;
            x /= -10l;
            return (x == 3865470567u);
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
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant UInt [1]
                    ├── <20> Assign [/=]
                    │   ├── <15> Var [x]
                    │   ╰── <19> Unary [-]
                    │       ╰── <18> Constant Long [10]
                    ╰── Return
                        ╰── <27>  [==]
                            ├── <23> Var [x]
                            ╰── <25> Constant UInt [3865470567]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitshift() {
    let src = r#"
        int main(void) {
            int i = -2;
            i >>= 3u;
            if (i != -1) {
                return 1;
            }
            unsigned long ul = 18446744073709551615UL;
            ul <<= 44;
            if (ul != 18446726481523507200ul) {
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
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [2]
                    ├── <18> Assign [>>=]
                    │   ├── <15> Var [i]
                    │   ╰── <17> Constant UInt [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <26>  [!=]
                    │   │       ├── <21> Var [i]
                    │   │       ╰── <25> Unary [-]
                    │   │           ╰── <24> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <27> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <35> Constant ULong [18446744073709551615]
                    ├── <42> Assign [<<=]
                    │   ├── <39> Var [ul]
                    │   ╰── <41> Constant Int [44]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45> Var [ul]
                    │   │       ╰── <47> Constant ULong [18446726481523507200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <49> Constant Int [2]
                    ╰── Return
                        ╰── <54> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise() {
    let src = r#"
        int main(void) {
            unsigned long ul = 18446460386757245432ul;
            ul &= -1000;
            if (ul != 18446460386757244952ul ) {
                return 1;
            }
            ul |= 4294967040u;
            if (ul != 18446460386824683288ul ) {
                return 2;
            }
            int i = 123456;
            unsigned int ui = 4042322160u;
            long l = -252645136;
            if (ui ^= l) {
                return 3;
            }
            if (ui) {
                return 4;
            }
            if (i != 123456) {
                return 5;
            }
            if (l != -252645136) {
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
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <9> Constant ULong [18446460386757245432]
                    ├── <18> Assign [&=]
                    │   ├── <13> Var [ul]
                    │   ╰── <17> Unary [-]
                    │       ╰── <16> Constant Int [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <24>  [!=]
                    │   │       ├── <21> Var [ul]
                    │   │       ╰── <23> Constant ULong [18446460386757244952]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <25> Constant Int [1]
                    ├── <34> Assign [|=]
                    │   ├── <31> Var [ul]
                    │   ╰── <33> Constant UInt [4294967040]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> Var [ul]
                    │   │       ╰── <39> Constant ULong [18446460386824683288]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <49> Constant Int [123456]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <55> Constant UInt [4042322160]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <63> Unary [-]
                    │           ╰── <62> Constant Int [252645136]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71> Assign [^=]
                    │   │       ├── <67> Var [ui]
                    │   │       ╰── <70> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <72> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Var [i]
                    │   │       ╰── <87> Constant Int [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <95> Var [l]
                    │   │       ╰── <99> Unary [-]
                    │   │           ╰── <98> Constant Int [252645136]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [6]
                    ╰── Return
                        ╰── <106> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_postfix_precedence() {
    let src = r#"
        int main(void) {
            unsigned int ui = 4294967295U;
            if (((unsigned long)ui++) != 4294967295U) {
                return 1;
            }
            if (ui) {
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
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant UInt [4294967295]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <17> Postfix [++]
                    │   │       │           ╰── <15> Var [ui]
                    │   │       ╰── <21> Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [2]
                    ╰── Return
                        ╰── <35> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_uint() {
    let src = r#"
        int switch_on_uint(unsigned int ui) {
            switch (ui) {
                case 5u:
                    return 0;
                case 4294967286l:
                    return 1;
                case 34359738378ul:
                    return 2;
                default:
                    return 3;
            }
        }
        int main(void) {
            if (switch_on_uint(5) != 0)
                return 1;
            if (switch_on_uint(4294967286) != 1)
                return 2;
            if (switch_on_uint(10) != 2)
                return 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [switch_on_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ui
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <10> Var [ui]
            │           ╰── Block
            │               ├── Case [5]
            │               │   ╰── Return
            │               │       ╰── <12> Constant Int [0]
            │               ├── Case [4294967286]
            │               │   ╰── Return
            │               │       ╰── <16> Constant Int [1]
            │               ├── Case [34359738378]
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
                    │   │       ├── <38> FunctionCall [switch_on_uint]
                    │   │       │   ╰── <37> Constant Int [5]
                    │   │       ╰── <40> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <42> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> FunctionCall [switch_on_uint]
                    │   │       │   ╰── <46> Constant Long [4294967286]
                    │   │       ╰── <49> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <51> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> FunctionCall [switch_on_uint]
                    │   │       │   ╰── <55> Constant Int [10]
                    │   │       ╰── <58> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <60> Constant Int [3]
                    ╰── Return
                        ╰── <63> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_unsigned_incr_decr() {
    let src = r#"
        
        int main(void) {
            unsigned int i = 0;
            if (i-- != 0) {
                return 1;
            }
            if (i != 4294967295U) {
                return 2;
            }
            if (--i != 4294967294U) {
                return 3;
            }
            if (i != 4294967294U) {
                return 4;
            }
            unsigned long l = 18446744073709551614UL;
            if (l++ != 18446744073709551614UL) {
                return 5;
            }
            if (l != 18446744073709551615UL) {
                return 6;
            }
            if (++l != 0) {
                return 7;
            }
            if (l != 0) {
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
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <18>  [!=]
                    │   │       ├── <15> Postfix [--]
                    │   │       │   ╰── <13> Var [i]
                    │   │       ╰── <17> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <19> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> Var [i]
                    │   │       ╰── <27> Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <29> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> Unary [--]
                    │   │       │   ╰── <36> Var [i]
                    │   │       ╰── <39> Constant UInt [4294967294]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Var [i]
                    │   │       ╰── <49> Constant UInt [4294967294]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <59> Constant ULong [18446744073709551614]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> Postfix [++]
                    │   │       │   ╰── <63> Var [l]
                    │   │       ╰── <67> Constant ULong [18446744073709551614]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <75> Var [l]
                    │   │       ╰── <77> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Unary [++]
                    │   │       │   ╰── <86> Var [l]
                    │   │       ╰── <89> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <91> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [l]
                    │   │       ╰── <99> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [8]
                    ╰── Return
                        ╰── <106> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_common_type() {
    let src = r#"
        int int_gt_uint(int i, unsigned int u) {
            return i > u;
        }
        int int_gt_ulong(int i, unsigned long ul) {
            return i > ul;
        }
        int uint_gt_long(unsigned int u, long l) {
            return u > l;
        }
        int uint_lt_ulong(unsigned int u, unsigned long ul) {
            return u < ul;
        }
        int long_gt_ulong(long l, unsigned long ul) {
            return l > ul;
        }
        int ternary_int_uint(int flag, int i, unsigned int ui) {
            long result = flag ? i : ui;
            return (result == 4294967295l);
        }
        int main(void) {
            if (!int_gt_uint(-100, 100u)) {
                return 1;
            }
            if (!(int_gt_ulong(-1, 18446744073709551606ul))) {
                return 2;
            }
            if (!uint_gt_long(100u, -100l)) {
                return 3;
            }
            if (!uint_lt_ulong(1073741824u, 34359738368ul)) {
                return 4;
            }
            if (!long_gt_ulong(-1l, 1000ul)) {
                return 5;
            }
            if (!ternary_int_uint(1, -1, 1u)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [int_gt_uint]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <17>  [>]
            │               ├── <13> Var [i]
            │               ╰── <16> Var [u]
            ├── Function [int_gt_ulong]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <37>  [>]
            │               ├── <33> Var [i]
            │               ╰── <36> Var [ul]
            ├── Function [uint_gt_long]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── u
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <57>  [>]
            │               ├── <53> Var [u]
            │               ╰── <56> Var [l]
            ├── Function [uint_lt_ulong]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── u
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <77>  [<]
            │               ├── <73> Var [u]
            │               ╰── <76> Var [ul]
            ├── Function [long_gt_ulong]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <97>  [>]
            │               ├── <93> Var [l]
            │               ╰── <96> Var [ul]
            ├── Function [ternary_int_uint]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── flag
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ui
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <124> Conditional [?]
            │       │           ├── <119> Var [flag]
            │       │           ├── Then
            │       │           │   ╰── <121> Var [i]
            │       │           ╰── Else
            │       │               ╰── <123> Var [ui]
            │       ╰── Return
            │           ╰── <132>  [==]
            │               ├── <128> Var [result]
            │               ╰── <130> Constant Long [4294967295]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148> Unary [!]
                    │   │       ╰── <147> FunctionCall [int_gt_uint]
                    │   │           ├── <145> Unary [-]
                    │   │           │   ╰── <144> Constant Int [100]
                    │   │           ╰── <146> Constant UInt [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <149> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162> Unary [!]
                    │   │       ╰── <161> FunctionCall [int_gt_ulong]
                    │   │           ├── <158> Unary [-]
                    │   │           │   ╰── <157> Constant Int [1]
                    │   │           ╰── <159> Constant ULong [18446744073709551606]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <163> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <175> Unary [!]
                    │   │       ╰── <174> FunctionCall [uint_gt_long]
                    │   │           ├── <170> Constant UInt [100]
                    │   │           ╰── <173> Unary [-]
                    │   │               ╰── <172> Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <186> Unary [!]
                    │   │       ╰── <185> FunctionCall [uint_lt_ulong]
                    │   │           ├── <183> Constant UInt [1073741824]
                    │   │           ╰── <184> Constant ULong [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <187> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <199> Unary [!]
                    │   │       ╰── <198> FunctionCall [long_gt_ulong]
                    │   │           ├── <196> Unary [-]
                    │   │           │   ╰── <195> Constant Long [1]
                    │   │           ╰── <197> Constant ULong [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <200> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <213> Unary [!]
                    │   │       ╰── <212> FunctionCall [ternary_int_uint]
                    │   │           ├── <207> Constant Int [1]
                    │   │           ├── <210> Unary [-]
                    │   │           │   ╰── <209> Constant Int [1]
                    │   │           ╰── <211> Constant UInt [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <214> Constant Int [6]
                    ╰── Return
                        ╰── <219> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_convert_by_assignment() {
    let src = r#"
        int check_int(int converted, int expected) {
            return (converted == expected);
        }
        int check_long(long converted, long expected) {
            return (converted == expected);
        }
        int check_ulong(unsigned long converted, unsigned long expected) {
            return (converted == expected);
        }
        long return_extended_uint(unsigned int u) {
            return u;
        }
        unsigned long return_extended_int(int i) {
            return i;
        }
        int return_truncated_ulong(unsigned long ul) {
            return ul;
        }
        int extend_on_assignment(unsigned int ui, long expected) {
            long result = ui;
            return result == expected;
        }
        int main(void) {
            if (!check_int(9223372036854775813ul, 5)) {
                return 1;
            }
            if (!check_long(2147483658u, 2147483658l)) {
                return 2;
            }
            if (!check_ulong(-1, 18446744073709551615UL)) {
                return 3;
            }
            if (return_extended_uint(2147483658u) != 2147483658l) {
                return 4;
            }
            if (return_extended_int(-1) != 18446744073709551615UL) {
                return 5;
            }
            long l = return_truncated_ulong(1125902054326372ul);
            if (l != -2147483548l) {
                return 6;
            }
            if (!extend_on_assignment(2147483658u, 2147483658l)){
                return 7;
            }
            int i = 4294967196u;
            if (i != -100) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_int]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <18>  [==]
            │               ├── <13> Var [converted]
            │               ╰── <16> Var [expected]
            ├── Function [check_long]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <39>  [==]
            │               ├── <34> Var [converted]
            │               ╰── <37> Var [expected]
            ├── Function [check_ulong]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <60>  [==]
            │               ├── <55> Var [converted]
            │               ╰── <58> Var [expected]
            ├── Function [return_extended_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <73> Var [u]
            ├── Function [return_extended_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <86> Var [i]
            ├── Function [return_truncated_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <99> Var [ul]
            ├── Function [extend_on_assignment]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ui
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <118> Var [ui]
            │       ╰── Return
            │           ╰── <126>  [==]
            │               ├── <122> Var [result]
            │               ╰── <125> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140> Unary [!]
                    │   │       ╰── <139> FunctionCall [check_int]
                    │   │           ├── <137> Constant ULong [9223372036854775813]
                    │   │           ╰── <138> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <141> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151> Unary [!]
                    │   │       ╰── <150> FunctionCall [check_long]
                    │   │           ├── <148> Constant UInt [2147483658]
                    │   │           ╰── <149> Constant Long [2147483658]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <164> Unary [!]
                    │   │       ╰── <163> FunctionCall [check_ulong]
                    │   │           ├── <161> Unary [-]
                    │   │           │   ╰── <160> Constant Int [1]
                    │   │           ╰── <162> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <165> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <175>  [!=]
                    │   │       ├── <172> FunctionCall [return_extended_uint]
                    │   │       │   ╰── <171> Constant UInt [2147483658]
                    │   │       ╰── <174> Constant Long [2147483658]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <188>  [!=]
                    │   │       ├── <185> FunctionCall [return_extended_int]
                    │   │       │   ╰── <184> Unary [-]
                    │   │       │       ╰── <183> Constant Int [1]
                    │   │       ╰── <187> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <189> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <199> FunctionCall [return_truncated_ulong]
                    │           ╰── <198> Constant ULong [1125902054326372]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <208>  [!=]
                    │   │       ├── <203> Var [l]
                    │   │       ╰── <207> Unary [-]
                    │   │           ╰── <206> Constant Long [2147483548]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <209> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <219> Unary [!]
                    │   │       ╰── <218> FunctionCall [extend_on_assignment]
                    │   │           ├── <216> Constant UInt [2147483658]
                    │   │           ╰── <217> Constant Long [2147483658]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <220> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <228> Constant UInt [4294967196]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <237>  [!=]
                    │   │       ├── <232> Var [i]
                    │   │       ╰── <236> Unary [-]
                    │   │           ╰── <235> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <238> Constant Int [8]
                    ╰── Return
                        ╰── <243> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_promote_constants() {
    let src = r#"
        long negative_one = 1l;
        long zero = 0l;
        int main(void) {
            negative_one = -negative_one;
            if (68719476736u >= negative_one) {
                return 1;
            }
            if (-2147483658 >= zero) {
                return 2;
            }
            if (!(3ul + 4294967293ul)) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── negative_one
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <4> Constant Long [1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <10> Constant Long [0]
            ╰── Function [main]
                ╰── Body
                    ├── <25> Assign [=]
                    │   ├── <19> Var [negative_one]
                    │   ╰── <24> Unary [-]
                    │       ╰── <23> Var [negative_one]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [>=]
                    │   │       ├── <27> Constant ULong [68719476736]
                    │   │       ╰── <30> Var [negative_one]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [>=]
                    │   │       ├── <39> Unary [-]
                    │   │       │   ╰── <38> Constant Long [2147483658]
                    │   │       ╰── <42> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55> Unary [!]
                    │   │       ╰── <54>  [+]
                    │   │           ├── <50> Constant ULong [3]
                    │   │           ╰── <52> Constant ULong [4294967293]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <56> Constant Int [3]
                    ╰── Return
                        ╰── <61> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_implicit_casts_static_initializers() {
    let src = r#"
        unsigned int u = 1152921506754330636l;
        int i = 2147483650u;
        long l = 9223372036854775900u;
        long l2 = 2147483650u;
        unsigned long ul = 4294967294u;
        unsigned long ul2 = 9223372036854775798l;
        int i2 = 9223372039002259606ul;
        unsigned ui2 = 9223372039002259606ul;
        int main(void)
        {
            if (u != 2147483660u)
                return 1;
            if (i != -2147483646)
                return 2;
            if (l != -9223372036854775716l)
                return 3;
            if (l2 != 2147483650l)
                return 4;
            if (ul != 4294967294ul)
                return 5;
            if (ul2 != 9223372036854775798ul)
                return 6;
            if (i2 != -2147483498)
                return 7;
            if (ui2 != 2147483798u)
                return 8;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── u
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <4> Constant Long [1152921506754330636]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <10> Constant UInt [2147483650]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <16> Constant ULong [9223372036854775900]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l2
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <22> Constant UInt [2147483650]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <28> Constant UInt [4294967294]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul2
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <34> Constant Long [9223372036854775798]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i2
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <40> Constant ULong [9223372039002259606]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui2
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <46> Constant ULong [9223372039002259606]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <55> Var [u]
                    │   │       ╰── <57> Constant UInt [2147483660]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <59> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <63> Var [i]
                    │   │       ╰── <67> Unary [-]
                    │   │           ╰── <66> Constant Int [2147483646]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <69> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <73> Var [l]
                    │   │       ╰── <77> Unary [-]
                    │   │           ╰── <76> Constant Long [9223372036854775716]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <79> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <83> Var [l2]
                    │   │       ╰── <85> Constant Long [2147483650]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <87> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <91> Var [ul]
                    │   │       ╰── <93> Constant ULong [4294967294]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <95> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102>  [!=]
                    │   │       ├── <99> Var [ul2]
                    │   │       ╰── <101> Constant ULong [9223372036854775798]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <103> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <107> Var [i2]
                    │   │       ╰── <111> Unary [-]
                    │   │           ╰── <110> Constant Int [2147483498]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <113> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Var [ui2]
                    │   │       ╰── <119> Constant UInt [2147483798]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <121> Constant Int [8]
                    ╰── Return
                        ╰── <124> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_unsigned_args() {
    let src = r#"
        int accept_unsigned(unsigned int a, unsigned int b, unsigned long c, unsigned long d,
                         unsigned int e, unsigned int f, unsigned long g, unsigned int h,
                         unsigned long i) {
            if (a != 1u) {
                return 1;
            }
            if (b != 4294967295U) {
                return 2;
            }
            if (c != 18446744073709551615UL) {
                return 3;
            }
            if (d != 9223372036854775808ul) {
                return 4;
            }
            if (e != 2147483648u) {
                return 5;
            }
            if (f != 0u) {
                return 8;
            }
            if (g != 123456u) {
                return 9;
            }
            if (h != 2147487744u) {
                return 10;
            }
            if (i != 9223372041149743104ul) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [accept_unsigned]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Unsigned Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Unsigned Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Unsigned Long
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Unsigned Long
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Unsigned Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Unsigned Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Unsigned Long
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── h
                │   │   ╰── Type
                │   │       ╰── Unsigned Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── i
                │       ╰── Type
                │           ╰── Unsigned Long
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [a]
                    │   │       ╰── <36> Constant UInt [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <38> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Var [b]
                    │   │       ╰── <46> Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> Var [c]
                    │   │       ╰── <56> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> Var [d]
                    │   │       ╰── <66> Constant ULong [9223372036854775808]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <68> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> Var [e]
                    │   │       ╰── <76> Constant UInt [2147483648]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> Var [f]
                    │   │       ╰── <86> Constant UInt [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97>  [!=]
                    │   │       ├── <94> Var [g]
                    │   │       ╰── <96> Constant UInt [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <98> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Var [h]
                    │   │       ╰── <106> Constant UInt [2147487744]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Var [i]
                    │   │       ╰── <116> Constant ULong [9223372041149743104]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [11]
                    ╰── Return
                        ╰── <123> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_unsigned_args_client() {
    let src = r#"
        
        int accept_unsigned(unsigned int a, unsigned int b, unsigned long c, unsigned long d,
                         unsigned int e, unsigned int f, unsigned long g, unsigned int h,
                         unsigned long i);
        int main(void) {
            return accept_unsigned(1, -1, -1, 9223372036854775808ul, 2147483648ul, 0, 123456, 2147487744u, 9223372041149743104ul);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [accept_unsigned]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Unsigned Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Unsigned Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Unsigned Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Unsigned Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── h
            │       │   ╰── Type
            │       │       ╰── Unsigned Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── i
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <53> FunctionCall [accept_unsigned]
                            ├── <40> Constant Int [1]
                            ├── <43> Unary [-]
                            │   ╰── <42> Constant Int [1]
                            ├── <46> Unary [-]
                            │   ╰── <45> Constant Int [1]
                            ├── <47> Constant ULong [9223372036854775808]
                            ├── <48> Constant ULong [2147483648]
                            ├── <49> Constant Int [0]
                            ├── <50> Constant Int [123456]
                            ├── <51> Constant UInt [2147487744]
                            ╰── <52> Constant ULong [9223372041149743104]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_unsigned_global_var() {
    let src = r#"
        unsigned int ui = 4294967200u;
        unsigned int return_uint(void) {
            return ui;
        }
        int return_uint_as_signed(void) {
            return ui;
        }
        long return_uint_as_long(void) {
            return ui;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <4> Constant UInt [4294967200]
            ├── Function [return_uint]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Var [ui]
            ├── Function [return_uint_as_signed]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <23> Var [ui]
            ╰── Function [return_uint_as_long]
                ╰── Body
                    ╰── Return
                        ╰── <33> Var [ui]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_unsigned_global_var_client() {
    let src = r#"
        extern unsigned int ui;
        unsigned int return_uint(void);
        int return_uint_as_signed(void);
        long return_uint_as_long(void);
        int main(void) {
            if (ui != 4294967200u)
                return 0;
            ui = -1;
            long result = (long) return_uint();
            if (result != 4294967295l)
                return 0;
            result = (long) return_uint_as_signed();
            if (result != -1l)
                return 0;
            result = return_uint_as_long();
            if (result != 4294967295l)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Extern
            ├── Function [return_uint]
            ├── Function [return_uint_as_signed]
            ├── Function [return_uint_as_long]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Var [ui]
                    │   │       ╰── <32> Constant UInt [4294967200]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <34> Constant Int [0]
                    ├── <43> Assign [=]
                    │   ├── <38> Var [ui]
                    │   ╰── <42> Unary [-]
                    │       ╰── <41> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <52> Cast
                    │           ├── Target
                    │           │   ╰── Long
                    │           ╰── Expression
                    │               ╰── <51> FunctionCall [return_uint]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> Var [result]
                    │   │       ╰── <58> Constant Long [4294967295]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <60> Constant Int [0]
                    ├── <71> Assign [=]
                    │   ├── <64> Var [result]
                    │   ╰── <70> Cast
                    │       ├── Target
                    │       │   ╰── Long
                    │       ╰── Expression
                    │           ╰── <69> FunctionCall [return_uint_as_signed]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <74> Var [result]
                    │   │       ╰── <78> Unary [-]
                    │   │           ╰── <77> Constant Long [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <80> Constant Int [0]
                    ├── <88> Assign [=]
                    │   ├── <84> Var [result]
                    │   ╰── <87> FunctionCall [return_uint_as_long]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <91> Var [result]
                    │   │       ╰── <93> Constant Long [4294967295]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <95> Constant Int [0]
                    ╰── Return
                        ╰── <98> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_type_specifiers_signed_type_specifiers() {
    let src = r#"
        static int i;
        signed extern i;
        int static signed i = 5;
        signed int static i;
        long signed l;
        long l = 7;
        int long l;
        signed long int l;
        int main(void) {
            int signed extern i;
            extern signed long l;
            if (i != 5) {
                return 1;
            }
            if (l != 7) {
                return 2;
            }
            int counter = 0;
            for (signed int index = 10; index > 0; index = index - 1) {
                counter = counter + 1;
            }
            if (counter != 10) {
                return 3;
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
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <15> Constant Int [5]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ╰── Type
            │       ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <30> Constant Int [7]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ╰── Type
            │       ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ╰── Type
            │       ╰── Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Extern
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> Var [i]
                    │   │       ╰── <59> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <61> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> Var [l]
                    │   │       ╰── <69> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <71> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <79> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── index
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <85> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <93>  [>]
                    │   │       ├── <90> Var [index]
                    │   │       ╰── <92> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <102> Assign [=]
                    │   │       ├── <95> Var [index]
                    │   │       ╰── <101>  [-]
                    │   │           ├── <98> Var [index]
                    │   │           ╰── <100> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <111> Assign [=]
                    │           ├── <104> Var [counter]
                    │           ╰── <110>  [+]
                    │               ├── <107> Var [counter]
                    │               ╰── <109> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Var [counter]
                    │   │       ╰── <119> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [3]
                    ╰── Return
                        ╰── <126> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_type_specifiers_unsigned_type_specifiers() {
    let src = r#"
        unsigned u;
        int unsigned u;
        unsigned int u = 6;
        unsigned long ul;
        long unsigned ul;
        long int unsigned ul;
        unsigned int long ul = 4;
        int main(void) {
            if (u != 6u) {
                return 1;
            }
            long extern unsigned ul;
            unsigned long extern ul;
            int extern unsigned long ul;
            if (ul != 4ul) {
                return 2;
            }
            int counter = 0;
            for (unsigned int index = 10; index < 4294967295U; index = index - 1) {
                counter = counter + 1;
            }
            if (counter != 11) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── u
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── u
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── u
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <12> Constant Int [6]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <30> Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [!=]
                    │   │       ├── <39> Var [u]
                    │   │       ╰── <41> Constant UInt [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <43> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Extern
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Extern
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Extern
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> Var [ul]
                    │   │       ╰── <66> Constant ULong [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <68> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <76> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── index
                    │   │       ├── Type
                    │   │       │   ╰── Unsigned Int
                    │   │       ╰── Initializer
                    │   │           ╰── <82> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <90>  [<]
                    │   │       ├── <87> Var [index]
                    │   │       ╰── <89> Constant UInt [4294967295]
                    │   ├── Condition
                    │   │   ╰── <99> Assign [=]
                    │   │       ├── <92> Var [index]
                    │   │       ╰── <98>  [-]
                    │   │           ├── <95> Var [index]
                    │   │           ╰── <97> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <108> Assign [=]
                    │           ├── <101> Var [counter]
                    │           ╰── <107>  [+]
                    │               ├── <104> Var [counter]
                    │               ╰── <106> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Var [counter]
                    │   │       ╰── <116> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [3]
                    ╰── Return
                        ╰── <123> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_arithmetic_ops() {
    let src = r#"
        unsigned int ui_a;
        unsigned int ui_b;
        unsigned long ul_a;
        unsigned long ul_b;
        int addition(void) {
            return (ui_a + 2147483653u == 2147483663u);
        }
        int subtraction(void) {
            return (ul_a - ul_b == 18446744072635808792ul);
        }
        int multiplication(void) {
            return (ui_a * ui_b == 3221225472u);
        }
        int division(void) {
            return (ui_a / ui_b == 0);
        }
        int division_large_dividend(void) {
            return (ui_a / ui_b == 2);
        }
        int division_by_literal(void) {
            return (ul_a / 5ul == 219902325555ul);
        }
        int remaind(void) {
            return (ul_b % ul_a == 5ul);
        }
        int complement(void) {
            return (~ui_a == 0);
        }
        int main(void) {
            ui_a = 10u;
            if (!addition()) {
                return 1;
            }
            ul_a = 18446744072635809792ul;
            ul_b = 1000ul;
            if (!subtraction()) {
                return 2;
            }
            ui_a = 1073741824u;
            ui_b = 3u;
            if (!multiplication()) {
                return 3;
            }
            ui_a = 100u;
            ui_b = 4294967294u;
            if (!division()) {
                return 4;
            }
            ui_a = 4294967294u;
            ui_b = 2147483647u;
            if (!division_large_dividend()) {
                return 5;
            }
            ul_a = 1099511627775ul;
            if (!division_by_literal()) {
                return 6;
            }
            ul_a = 100ul;
            ul_b = 18446744073709551605ul;
            if (!remaind()) {
                return 7;
            }
            ui_a = 4294967295U;
            if (!complement()) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui_a
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui_b
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul_a
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul_b
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── Function [addition]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <30>  [==]
            │               ├── <26>  [+]
            │               │   ├── <23> Var [ui_a]
            │               │   ╰── <25> Constant UInt [2147483653]
            │               ╰── <28> Constant UInt [2147483663]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <48>  [==]
            │               ├── <44>  [-]
            │               │   ├── <40> Var [ul_a]
            │               │   ╰── <43> Var [ul_b]
            │               ╰── <46> Constant ULong [18446744072635808792]
            ├── Function [multiplication]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <66>  [==]
            │               ├── <62>  [*]
            │               │   ├── <58> Var [ui_a]
            │               │   ╰── <61> Var [ui_b]
            │               ╰── <64> Constant UInt [3221225472]
            ├── Function [division]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <84>  [==]
            │               ├── <80>  [/]
            │               │   ├── <76> Var [ui_a]
            │               │   ╰── <79> Var [ui_b]
            │               ╰── <82> Constant Int [0]
            ├── Function [division_large_dividend]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <102>  [==]
            │               ├── <98>  [/]
            │               │   ├── <94> Var [ui_a]
            │               │   ╰── <97> Var [ui_b]
            │               ╰── <100> Constant Int [2]
            ├── Function [division_by_literal]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <119>  [==]
            │               ├── <115>  [/]
            │               │   ├── <112> Var [ul_a]
            │               │   ╰── <114> Constant ULong [5]
            │               ╰── <117> Constant ULong [219902325555]
            ├── Function [remaind]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <137>  [==]
            │               ├── <133>  [%]
            │               │   ├── <129> Var [ul_b]
            │               │   ╰── <132> Var [ul_a]
            │               ╰── <135> Constant ULong [5]
            ├── Function [complement]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <153>  [==]
            │               ├── <149> Unary [~]
            │               │   ╰── <148> Var [ui_a]
            │               ╰── <151> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <166> Assign [=]
                    │   ├── <163> Var [ui_a]
                    │   ╰── <165> Constant UInt [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171> Unary [!]
                    │   │       ╰── <170> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <172> Constant Int [1]
                    ├── <181> Assign [=]
                    │   ├── <178> Var [ul_a]
                    │   ╰── <180> Constant ULong [18446744072635809792]
                    ├── <187> Assign [=]
                    │   ├── <184> Var [ul_b]
                    │   ╰── <186> Constant ULong [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192> Unary [!]
                    │   │       ╰── <191> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <193> Constant Int [2]
                    ├── <202> Assign [=]
                    │   ├── <199> Var [ui_a]
                    │   ╰── <201> Constant UInt [1073741824]
                    ├── <208> Assign [=]
                    │   ├── <205> Var [ui_b]
                    │   ╰── <207> Constant UInt [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <213> Unary [!]
                    │   │       ╰── <212> FunctionCall [multiplication]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <214> Constant Int [3]
                    ├── <223> Assign [=]
                    │   ├── <220> Var [ui_a]
                    │   ╰── <222> Constant UInt [100]
                    ├── <229> Assign [=]
                    │   ├── <226> Var [ui_b]
                    │   ╰── <228> Constant UInt [4294967294]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <234> Unary [!]
                    │   │       ╰── <233> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <235> Constant Int [4]
                    ├── <244> Assign [=]
                    │   ├── <241> Var [ui_a]
                    │   ╰── <243> Constant UInt [4294967294]
                    ├── <250> Assign [=]
                    │   ├── <247> Var [ui_b]
                    │   ╰── <249> Constant UInt [2147483647]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <255> Unary [!]
                    │   │       ╰── <254> FunctionCall [division_large_dividend]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <256> Constant Int [5]
                    ├── <265> Assign [=]
                    │   ├── <262> Var [ul_a]
                    │   ╰── <264> Constant ULong [1099511627775]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <270> Unary [!]
                    │   │       ╰── <269> FunctionCall [division_by_literal]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <271> Constant Int [6]
                    ├── <280> Assign [=]
                    │   ├── <277> Var [ul_a]
                    │   ╰── <279> Constant ULong [100]
                    ├── <286> Assign [=]
                    │   ├── <283> Var [ul_b]
                    │   ╰── <285> Constant ULong [18446744073709551605]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <291> Unary [!]
                    │   │       ╰── <290> FunctionCall [remaind]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <292> Constant Int [7]
                    ├── <301> Assign [=]
                    │   ├── <298> Var [ui_a]
                    │   ╰── <300> Constant UInt [4294967295]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <306> Unary [!]
                    │   │       ╰── <305> FunctionCall [complement]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <307> Constant Int [8]
                    ╰── Return
                        ╰── <312> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_arithmetic_wraparound() {
    let src = r#"
        unsigned int ui_a;
        unsigned int ui_b;
        unsigned long ul_a;
        unsigned long ul_b;
        int addition(void) {
            return ui_a + ui_b == 0u;
        }
        int subtraction(void) {
            return (ul_a - ul_b == 18446744073709551606ul);
        }
        int neg(void) {
            return -ul_a == 18446744073709551615UL;
        }
        int main(void) {
            ui_a = 4294967293u;
            ui_b = 3u;
            if (!addition()) {
                return 1;
            }
            ul_a = 10ul;
            ul_b = 20ul;
            if (!subtraction()) {
                return 2;
            }
            ul_a = 1ul;
            if (!neg()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui_a
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui_b
            │   ╰── Type
            │       ╰── Unsigned Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul_a
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul_b
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── Function [addition]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <30>  [==]
            │               ├── <27>  [+]
            │               │   ├── <23> Var [ui_a]
            │               │   ╰── <26> Var [ui_b]
            │               ╰── <29> Constant UInt [0]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <48>  [==]
            │               ├── <44>  [-]
            │               │   ├── <40> Var [ul_a]
            │               │   ╰── <43> Var [ul_b]
            │               ╰── <46> Constant ULong [18446744073709551606]
            ├── Function [neg]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <63>  [==]
            │               ├── <60> Unary [-]
            │               │   ╰── <59> Var [ul_a]
            │               ╰── <62> Constant ULong [18446744073709551615]
            ╰── Function [main]
                ╰── Body
                    ├── <76> Assign [=]
                    │   ├── <73> Var [ui_a]
                    │   ╰── <75> Constant UInt [4294967293]
                    ├── <82> Assign [=]
                    │   ├── <79> Var [ui_b]
                    │   ╰── <81> Constant UInt [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87> Unary [!]
                    │   │       ╰── <86> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [1]
                    ├── <97> Assign [=]
                    │   ├── <94> Var [ul_a]
                    │   ╰── <96> Constant ULong [10]
                    ├── <103> Assign [=]
                    │   ├── <100> Var [ul_b]
                    │   ╰── <102> Constant ULong [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108> Unary [!]
                    │   │       ╰── <107> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <109> Constant Int [2]
                    ├── <118> Assign [=]
                    │   ├── <115> Var [ul_a]
                    │   ╰── <117> Constant ULong [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123> Unary [!]
                    │   │       ╰── <122> FunctionCall [neg]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <124> Constant Int [3]
                    ╰── Return
                        ╰── <129> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_comparisons() {
    let src = r#"
        unsigned int one_hundred = 100u;
        unsigned int large_uint = 4294967294u;
        unsigned long one_hundred_ulong = 100ul;
        unsigned long large_ulong = 4294967294ul;
        int main(void) {
            if (large_uint < one_hundred)
                return 1;
            if (large_uint <= one_hundred)
                return 2;
            if (one_hundred >= large_uint)
                return 3;
            if (one_hundred > large_uint)
                return 4;
            if (!(one_hundred <= large_uint))
                return 5;
            if (!(one_hundred < large_uint))
                return 6;
            if (!(large_uint > one_hundred))
                return 7;
            if (!(large_uint >= one_hundred))
                return 8;
            if (large_ulong < one_hundred_ulong)
                return 9;
            if (large_ulong <= one_hundred_ulong)
                return 10;
            if (one_hundred_ulong >= large_ulong)
                return 11;
            if (one_hundred_ulong > large_ulong)
                return 12;
            if (!(one_hundred_ulong <= large_ulong))
                return 13;
            if (!(one_hundred_ulong < large_ulong))
                return 14;
            if (!(large_ulong > one_hundred_ulong))
                return 15;
            if (!(large_ulong >= one_hundred_ulong))
                return 16;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one_hundred
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <4> Constant UInt [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── large_uint
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <10> Constant UInt [4294967294]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one_hundred_ulong
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <16> Constant ULong [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── large_ulong
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <22> Constant ULong [4294967294]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [<]
                    │   │       ├── <31> Var [large_uint]
                    │   │       ╰── <34> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <36> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [<=]
                    │   │       ├── <40> Var [large_uint]
                    │   │       ╰── <43> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <45> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [>=]
                    │   │       ├── <49> Var [one_hundred]
                    │   │       ╰── <52> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <54> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [>]
                    │   │       ├── <58> Var [one_hundred]
                    │   │       ╰── <61> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <63> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74> Unary [!]
                    │   │       ╰── <73>  [<=]
                    │   │           ├── <68> Var [one_hundred]
                    │   │           ╰── <71> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <75> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86> Unary [!]
                    │   │       ╰── <85>  [<]
                    │   │           ├── <80> Var [one_hundred]
                    │   │           ╰── <83> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <87> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98> Unary [!]
                    │   │       ╰── <97>  [>]
                    │   │           ├── <92> Var [large_uint]
                    │   │           ╰── <95> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <99> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110> Unary [!]
                    │   │       ╰── <109>  [>=]
                    │   │           ├── <104> Var [large_uint]
                    │   │           ╰── <107> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <111> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [<]
                    │   │       ├── <115> Var [large_ulong]
                    │   │       ╰── <118> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <120> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [<=]
                    │   │       ├── <124> Var [large_ulong]
                    │   │       ╰── <127> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <129> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137>  [>=]
                    │   │       ├── <133> Var [one_hundred_ulong]
                    │   │       ╰── <136> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <138> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <146>  [>]
                    │   │       ├── <142> Var [one_hundred_ulong]
                    │   │       ╰── <145> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <147> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <158> Unary [!]
                    │   │       ╰── <157>  [<=]
                    │   │           ├── <152> Var [one_hundred_ulong]
                    │   │           ╰── <155> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <159> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <170> Unary [!]
                    │   │       ╰── <169>  [<]
                    │   │           ├── <164> Var [one_hundred_ulong]
                    │   │           ╰── <167> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <171> Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <182> Unary [!]
                    │   │       ╰── <181>  [>]
                    │   │           ├── <176> Var [large_ulong]
                    │   │           ╰── <179> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <183> Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <194> Unary [!]
                    │   │       ╰── <193>  [>=]
                    │   │           ├── <188> Var [large_ulong]
                    │   │           ╰── <191> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <195> Constant Int [16]
                    ╰── Return
                        ╰── <198> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_locals() {
    let src = r#"
        int main(void) {
            unsigned long a = 8589934592ul;
            int b = -1;
            long c = -8589934592l;
            unsigned int d = 10u;
            if (a != 8589934592ul) {
                return 1;
            }
            if (b != -1){
                return 2;
            }
            if (c != -8589934592l) {
                return 3;
            }
            if (d != 10u) {
                return 4;
            }
            a = -a;
            b = b - 1;
            c = c + 8589934594l;
            d = d * 268435456u;
            if (a != 18446744065119617024ul) {
                return 5;
            }
            if (b != -2) {
                return 6;
            }
            if (c != 2) {
                return 7;
            }
            if (d != 2684354560u) {
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
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <9> Constant ULong [8589934592]
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
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <31> Constant UInt [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> Var [a]
                    │   │       ╰── <37> Constant ULong [8589934592]
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
                    │   │       ╰── <71> Constant UInt [10]
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
                    │   ╰── <114>  [*]
                    │       ├── <111> Var [d]
                    │       ╰── <113> Constant UInt [268435456]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121>  [!=]
                    │   │       ├── <118> Var [a]
                    │   │       ╰── <120> Constant ULong [18446744065119617024]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <122> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <128> Var [b]
                    │   │       ╰── <132> Unary [-]
                    │   │           ╰── <131> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143>  [!=]
                    │   │       ├── <140> Var [c]
                    │   │       ╰── <142> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <144> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <150> Var [d]
                    │   │       ╰── <152> Constant UInt [2684354560]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [8]
                    ╰── Return
                        ╰── <159> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_logical() {
    let src = r#"
        int not(unsigned long ul) {
            return !ul;
        }
        int if_cond(unsigned u) {
            if (u) {
                return 1;
            }
            return 0;
        }
        int and(unsigned long ul, int i) {
            return ul && i;
        }
        int or(int i, unsigned u) {
            return i || u;
        }
        int main(void) {
            unsigned long ul = 1152921504606846976ul;
            unsigned int u = 2147483648u;
            unsigned long zero = 0l;
            if (not(ul)) {
                return 1;
            }
            if (!not(zero)) {
                return 2;
            }
            if(!if_cond(u)) {
                return 3;
            }
            if(if_cond(zero)) {
                return 4;
            }
            if (and(zero, 1)) {
                return 5;
            }
            if (!or(1, u)) {
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
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Unary [!]
            │               ╰── <11> Var [ul]
            ├── Function [if_cond]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <25> Var [u]
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
            │   │   │   │   ╰── ul
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <51>  [&&]
            │               ├── <47> Var [ul]
            │               ╰── <50> Var [i]
            ├── Function [or]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <71>  [||]
            │               ├── <67> Var [i]
            │               ╰── <70> Var [u]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <83> Constant ULong [1152921504606846976]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <89> Constant UInt [2147483648]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <95> Constant Long [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101> FunctionCall [not]
                    │   │       ╰── <100> Var [ul]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112> Unary [!]
                    │   │       ╰── <111> FunctionCall [not]
                    │   │           ╰── <110> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123> Unary [!]
                    │   │       ╰── <122> FunctionCall [if_cond]
                    │   │           ╰── <121> Var [u]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <124> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132> FunctionCall [if_cond]
                    │   │       ╰── <131> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142> FunctionCall [and]
                    │   │       ├── <140> Var [zero]
                    │   │       ╰── <141> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154> Unary [!]
                    │   │       ╰── <153> FunctionCall [or]
                    │   │           ├── <150> Constant Int [1]
                    │   │           ╰── <152> Var [u]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <155> Constant Int [6]
                    ╰── Return
                        ╰── <160> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_simple() {
    let src = r#"
        int main(void) {
            unsigned u = 2147483647u;
            return (u + 2u == 2147483649u);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant UInt [2147483647]
                    ╰── Return
                        ╰── <20>  [==]
                            ├── <16>  [+]
                            │   ├── <13> Var [u]
                            │   ╰── <15> Constant UInt [2]
                            ╰── <18> Constant UInt [2147483649]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unsigned_expressions_static_variables() {
    let src = r#"
        
        static unsigned long x = 9223372036854775803ul;
        unsigned long zero_long;
        unsigned zero_int;
        int main(void)
        {
            if (x != 9223372036854775803ul)
                return 0;
            x = x + 10;
            if (x != 9223372036854775813ul)
                return 0;
            if (zero_long || zero_int)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ├── Initializer
            │   │   ╰── <5> Constant ULong [9223372036854775803]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero_long
            │   ╰── Type
            │       ╰── Unsigned Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero_int
            │   ╰── Type
            │       ╰── Unsigned Int
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <25>  [!=]
                    │   │       ├── <22> Var [x]
                    │   │       ╰── <24> Constant ULong [9223372036854775803]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <26> Constant Int [0]
                    ├── <37> Assign [=]
                    │   ├── <30> Var [x]
                    │   ╰── <36>  [+]
                    │       ├── <33> Var [x]
                    │       ╰── <35> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Var [x]
                    │   │       ╰── <42> Constant ULong [9223372036854775813]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <44> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [||]
                    │   │       ├── <48> Var [zero_long]
                    │   │       ╰── <51> Var [zero_int]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <53> Constant Int [0]
                    ╰── Return
                        ╰── <56> Constant Int [1]
    "#;
    assert_parse(src, expected);
}
