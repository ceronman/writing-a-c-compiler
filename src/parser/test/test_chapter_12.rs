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
                    │       ╰── Constant UInt [10]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <12> Var [ui]
                        ╰── Block
                            ├── Case [4294967295]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ├── Case [1099511627775]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [2]
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
                        ╰── Constant Int [0]
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
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant UInt [4294967200]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <23>  [!=]
                    │   │       ├── <18> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <17> Cast
                    │   │       │           ├── Target
                    │   │       │           │   ╰── Int
                    │   │       │           ╰── Expression
                    │   │       │               ╰── <16> Var [ui]
                    │   │       ╰── <22> Unary [-]
                    │   │           ╰── Constant Long [96]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <33> Cast
                    │   │       │           ├── Target
                    │   │       │           │   ╰── Int
                    │   │       │           ╰── Expression
                    │   │       │               ╰── <32> Var [ui]
                    │   │       ╰── Constant ULong [18446744073709551520]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       │       ╰── <18> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <17> Var [i]
            │       ╰── Return
            │           ╰── <26>  [==]
            │               ├── <22> Var [result]
            │               ╰── <25> Var [expected]
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
            │       │       ╰── <47> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <46> Var [ui]
            │       ╰── Return
            │           ╰── <55>  [==]
            │               ├── <51> Var [result]
            │               ╰── <54> Var [expected]
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
            │           ╰── <77>  [==]
            │               ├── <73> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Long
            │               │   ╰── Expression
            │               │       ╰── <72> Var [ui]
            │               ╰── <76> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90> Unary [!]
                    │   │       ╰── <89> FunctionCall [int_to_ulong]
                    │   │           ├── Constant Int [10]
                    │   │           ╰── Constant ULong [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103> Unary [!]
                    │   │       ╰── <102> FunctionCall [int_to_ulong]
                    │   │           ├── <100> Unary [-]
                    │   │           │   ╰── Constant Int [10]
                    │   │           ╰── Constant ULong [18446744073709551606]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114> Unary [!]
                    │   │       ╰── <113> FunctionCall [uint_to_long]
                    │   │           ├── Constant UInt [4294967200]
                    │   │           ╰── Constant Long [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125> Unary [!]
                    │   │       ╰── <124> FunctionCall [uint_to_ulong]
                    │   │           ├── Constant UInt [4294967200]
                    │   │           ╰── Constant ULong [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137>  [!=]
                    │   │       ├── <134> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── Constant UInt [4294967200]
                    │   │       ╰── Constant ULong [4294967200]
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
            │       ╰── Constant UInt [5000]
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
            │       │           ╰── Constant UInt [4987]
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
            │       │   │       ╰── Constant Long [5000]
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
            │       ╰── Constant ULong [8589934580]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <21> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <20> Cast
                    │                   ├── Target
                    │                   │   ╰── Unsigned Int
                    │                   ╰── Expression
                    │                       ╰── <19> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> Var [b]
                    │   │       ╰── Constant ULong [4294967284]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── <43> Assign [=]
                    │   ├── <33> Var [b]
                    │   ╰── <42> Cast
                    │       ├── Target
                    │       │   ╰── Unsigned Long
                    │       ╰── Expression
                    │           ╰── <41> Cast
                    │               ├── Target
                    │               │   ╰── Int
                    │               ╰── Expression
                    │                   ╰── <40> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Var [b]
                    │   │       ╰── Constant ULong [18446744073709551604]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │           ╰── <19>  [==]
            │               ├── <15> Cast
            │               │   ├── Target
            │               │   │   ╰── Int
            │               │   ╰── Expression
            │               │       ╰── <14> Var [ui]
            │               ╰── <18> Var [expected]
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
            │           ╰── <41>  [==]
            │               ├── <37> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Int
            │               │   ╰── Expression
            │               │       ╰── <36> Var [i]
            │               ╰── <40> Var [expected]
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
            │           ╰── <63>  [==]
            │               ├── <59> Cast
            │               │   ├── Target
            │               │   │   ╰── Long
            │               │   ╰── Expression
            │               │       ╰── <58> Var [ul]
            │               ╰── <62> Var [expected]
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
            │           ╰── <85>  [==]
            │               ├── <81> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Long
            │               │   ╰── Expression
            │               │       ╰── <80> Var [l]
            │               ╰── <84> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98> Unary [!]
                    │   │       ╰── <97> FunctionCall [int_to_uint]
                    │   │           ├── Constant Int [10]
                    │   │           ╰── Constant UInt [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109> Unary [!]
                    │   │       ╰── <108> FunctionCall [uint_to_int]
                    │   │           ├── Constant UInt [10]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122> Unary [!]
                    │   │       ╰── <121> FunctionCall [long_to_ulong]
                    │   │           ├── <119> Unary [-]
                    │   │           │   ╰── Constant Long [1000]
                    │   │           ╰── Constant ULong [18446744073709550616]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <135> Unary [!]
                    │   │       ╰── <134> FunctionCall [ulong_to_long]
                    │   │           ├── Constant ULong [18446744073709550616]
                    │   │           ╰── <133> Unary [-]
                    │   │               ╰── Constant Long [1000]
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
            │       │       ╰── <18> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <17> Var [ul]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <22> Var [result]
            │               ╰── <25> Var [expected]
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
            │           ╰── <50>  [==]
            │               ├── <45> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Int
            │               │   ╰── Expression
            │               │       ╰── <44> Var [ul]
            │               ╰── <48> Var [expected]
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
            │           ╰── <72>  [==]
            │               ├── <68> Cast
            │               │   ├── Target
            │               │   │   ╰── Unsigned Int
            │               │   ╰── Expression
            │               │       ╰── <67> Var [l]
            │               ╰── <71> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85> Unary [!]
                    │   │       ╰── <84> FunctionCall [long_to_uint]
                    │   │           ├── Constant Long [100]
                    │   │           ╰── Constant UInt [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98> Unary [!]
                    │   │       ╰── <97> FunctionCall [long_to_uint]
                    │   │           ├── <95> Unary [-]
                    │   │           │   ╰── Constant Long [9223372036854774574]
                    │   │           ╰── Constant UInt [1234]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109> Unary [!]
                    │   │       ╰── <108> FunctionCall [ulong_to_int]
                    │   │           ├── Constant ULong [100]
                    │   │           ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120> Unary [!]
                    │   │       ╰── <119> FunctionCall [ulong_to_uint]
                    │   │           ├── Constant ULong [100]
                    │   │           ╰── Constant UInt [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131> Unary [!]
                    │   │       ╰── <130> FunctionCall [ulong_to_uint]
                    │   │           ├── Constant ULong [4294967200]
                    │   │           ╰── Constant UInt [4294967200]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144> Unary [!]
                    │   │       ╰── <143> FunctionCall [ulong_to_int]
                    │   │           ├── Constant ULong [4294967200]
                    │   │           ╰── <142> Unary [-]
                    │   │               ╰── Constant Int [96]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <155> Unary [!]
                    │   │       ╰── <154> FunctionCall [ulong_to_uint]
                    │   │           ├── Constant ULong [1152921506754330624]
                    │   │           ╰── Constant UInt [2147483648]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168> Unary [!]
                    │   │       ╰── <167> FunctionCall [ulong_to_int]
                    │   │           ├── Constant ULong [1152921506754330624]
                    │   │           ╰── <166> Unary [-]
                    │   │               ╰── Constant Long [2147483648]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <180> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Int
                    │           ╰── Expression
                    │               ╰── Constant ULong [17179869189]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <187>  [!=]
                    │   │       ├── <184> Var [ui]
                    │   │       ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [9]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant UInt [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [9223372036854775808]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25>  [&]
                    │   │       │   ├── <20> Var [ui]
                    │   │       │   ╰── <23> Var [ul]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38>  [|]
                    │   │       │   ├── <33> Var [ui]
                    │   │       │   ╰── <36> Var [ul]
                    │   │       ╰── Constant ULong [9223372041149743103]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <50> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <59>  [&]
                    │   │       │   ├── <54> Var [i]
                    │   │       │   ╰── <57> Var [ul]
                    │   │       ╰── <62> Var [ul]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <73>  [|]
                    │   │       │   ├── <68> Var [i]
                    │   │       │   ╰── <71> Var [ul]
                    │   │       ╰── <76> Var [i]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant UInt [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18>  [<<]
                    │   │       │   ├── <14> Var [ui]
                    │   │       │   ╰── Constant Long [2]
                    │   │       ╰── Constant Long [4294967292]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32>  [>>]
                    │   │       │   ├── <28> Var [ui]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [1073741823]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shiftcount
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── Constant Int [5]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53>  [>>]
                    │   │       │   ├── Constant UInt [1000000]
                    │   │       │   ╰── <51> Var [shiftcount]
                    │   │       ╰── Constant Int [31250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67>  [<<]
                    │   │       │   ├── Constant UInt [1000000]
                    │   │       │   ╰── <65> Var [shiftcount]
                    │   │       ╰── Constant Int [32000000]
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
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant UInt [1]
                    ├── <19> Assign [/=]
                    │   ├── <14> Var [x]
                    │   ╰── <18> Unary [-]
                    │       ╰── Constant Long [10]
                    ╰── Return
                        ╰── <26>  [==]
                            ├── <22> Var [x]
                            ╰── Constant UInt [3865470567]
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
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant Int [2]
                    ├── <17> Assign [>>=]
                    │   ├── <14> Var [i]
                    │   ╰── Constant UInt [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <25>  [!=]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <24> Unary [-]
                    │   │           ╰── Constant Int [1]
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
                    │       ╰── Constant ULong [18446744073709551615]
                    ├── <41> Assign [<<=]
                    │   ├── <38> Var [ul]
                    │   ╰── Constant Int [44]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Var [ul]
                    │   │       ╰── Constant ULong [18446726481523507200]
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
                    │       ╰── Constant ULong [18446460386757245432]
                    ├── <17> Assign [&=]
                    │   ├── <12> Var [ul]
                    │   ╰── <16> Unary [-]
                    │       ╰── Constant Int [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <23>  [!=]
                    │   │       ├── <20> Var [ul]
                    │   │       ╰── Constant ULong [18446460386757244952]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <33> Assign [|=]
                    │   ├── <30> Var [ul]
                    │   ╰── Constant UInt [4294967040]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> Var [ul]
                    │   │       ╰── Constant ULong [18446460386824683288]
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
                    │       ╰── Constant Int [123456]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [4042322160]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <62> Unary [-]
                    │           ╰── Constant Int [252645136]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70> Assign [^=]
                    │   │       ├── <66> Var [ui]
                    │   │       ╰── <69> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> Var [i]
                    │   │       ╰── Constant Int [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <94> Var [l]
                    │   │       ╰── <98> Unary [-]
                    │   │           ╰── Constant Int [252645136]
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
                    │       ╰── Constant UInt [4294967295]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <16> Postfix [++]
                    │   │       │           ╰── <14> Var [ui]
                    │   │       ╰── Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28> Var [ui]
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
            │           │   ╰── <9> Var [ui]
            │           ╰── Block
            │               ├── Case [5]
            │               │   ╰── Return
            │               │       ╰── Constant Int [0]
            │               ├── Case [4294967286]
            │               │   ╰── Return
            │               │       ╰── Constant Int [1]
            │               ├── Case [34359738378]
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
                    │   │       ├── <36> FunctionCall [switch_on_uint]
                    │   │       │   ╰── Constant Int [5]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45> FunctionCall [switch_on_uint]
                    │   │       │   ╰── Constant Long [4294967286]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> FunctionCall [switch_on_uint]
                    │   │       │   ╰── Constant Int [10]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <17>  [!=]
                    │   │       ├── <14> Postfix [--]
                    │   │       │   ╰── <12> Var [i]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> Unary [--]
                    │   │       │   ╰── <35> Var [i]
                    │   │       ╰── Constant UInt [4294967294]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Var [i]
                    │   │       ╰── Constant UInt [4294967294]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [18446744073709551614]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> Postfix [++]
                    │   │       │   ╰── <62> Var [l]
                    │   │       ╰── Constant ULong [18446744073709551614]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> Var [l]
                    │   │       ╰── Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <86> Unary [++]
                    │   │       │   ╰── <85> Var [l]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> Var [l]
                    │   │       ╰── Constant Int [0]
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
            │           ╰── <16>  [>]
            │               ├── <12> Var [i]
            │               ╰── <15> Var [u]
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
            │           ╰── <35>  [>]
            │               ├── <31> Var [i]
            │               ╰── <34> Var [ul]
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
            │           ╰── <54>  [>]
            │               ├── <50> Var [u]
            │               ╰── <53> Var [l]
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
            │           ╰── <73>  [<]
            │               ├── <69> Var [u]
            │               ╰── <72> Var [ul]
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
            │           ╰── <92>  [>]
            │               ├── <88> Var [l]
            │               ╰── <91> Var [ul]
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
            │       │       ╰── <{node_id}> Conditional [?]
            │       │           ├── <113> Var [flag]
            │       │           ├── Then
            │       │           │   ╰── <115> Var [i]
            │       │           ╰── Else
            │       │               ╰── <117> Var [ui]
            │       ╰── Return
            │           ╰── <126>  [==]
            │               ├── <122> Var [result]
            │               ╰── Constant Long [4294967295]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141> Unary [!]
                    │   │       ╰── <140> FunctionCall [int_gt_uint]
                    │   │           ├── <138> Unary [-]
                    │   │           │   ╰── Constant Int [100]
                    │   │           ╰── Constant UInt [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <155> Unary [!]
                    │   │       ╰── <154> FunctionCall [int_gt_ulong]
                    │   │           ├── <151> Unary [-]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── Constant ULong [18446744073709551606]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168> Unary [!]
                    │   │       ╰── <167> FunctionCall [uint_gt_long]
                    │   │           ├── Constant UInt [100]
                    │   │           ╰── <166> Unary [-]
                    │   │               ╰── Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <179> Unary [!]
                    │   │       ╰── <178> FunctionCall [uint_lt_ulong]
                    │   │           ├── Constant UInt [1073741824]
                    │   │           ╰── Constant ULong [34359738368]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192> Unary [!]
                    │   │       ╰── <191> FunctionCall [long_gt_ulong]
                    │   │           ├── <189> Unary [-]
                    │   │           │   ╰── Constant Long [1]
                    │   │           ╰── Constant ULong [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <206> Unary [!]
                    │   │       ╰── <205> FunctionCall [ternary_int_uint]
                    │   │           ├── Constant Int [1]
                    │   │           ├── <203> Unary [-]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── Constant UInt [1]
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
            │           ╰── <17>  [==]
            │               ├── <12> Var [converted]
            │               ╰── <15> Var [expected]
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
            │           ╰── <37>  [==]
            │               ├── <32> Var [converted]
            │               ╰── <35> Var [expected]
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
            │           ╰── <57>  [==]
            │               ├── <52> Var [converted]
            │               ╰── <55> Var [expected]
            ├── Function [return_extended_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <69> Var [u]
            ├── Function [return_extended_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <81> Var [i]
            ├── Function [return_truncated_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <93> Var [ul]
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
            │       │       ╰── <111> Var [ui]
            │       ╰── Return
            │           ╰── <119>  [==]
            │               ├── <115> Var [result]
            │               ╰── <118> Var [expected]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132> Unary [!]
                    │   │       ╰── <131> FunctionCall [check_int]
                    │   │           ├── Constant ULong [9223372036854775813]
                    │   │           ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143> Unary [!]
                    │   │       ╰── <142> FunctionCall [check_long]
                    │   │           ├── Constant UInt [2147483658]
                    │   │           ╰── Constant Long [2147483658]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156> Unary [!]
                    │   │       ╰── <155> FunctionCall [check_ulong]
                    │   │           ├── <153> Unary [-]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <167>  [!=]
                    │   │       ├── <164> FunctionCall [return_extended_uint]
                    │   │       │   ╰── Constant UInt [2147483658]
                    │   │       ╰── Constant Long [2147483658]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <177> FunctionCall [return_extended_int]
                    │   │       │   ╰── <176> Unary [-]
                    │   │       │       ╰── Constant Int [1]
                    │   │       ╰── Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <191> FunctionCall [return_truncated_ulong]
                    │           ╰── Constant ULong [1125902054326372]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <200>  [!=]
                    │   │       ├── <195> Var [l]
                    │   │       ╰── <199> Unary [-]
                    │   │           ╰── Constant Long [2147483548]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <211> Unary [!]
                    │   │       ╰── <210> FunctionCall [extend_on_assignment]
                    │   │           ├── Constant UInt [2147483658]
                    │   │           ╰── Constant Long [2147483658]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [4294967196]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <229>  [!=]
                    │   │       ├── <224> Var [i]
                    │   │       ╰── <228> Unary [-]
                    │   │           ╰── Constant Int [100]
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
            │       ╰── Constant Long [1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zero
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Long [0]
            ╰── Function [main]
                ╰── Body
                    ├── <24> Assign [=]
                    │   ├── <18> Var [negative_one]
                    │   ╰── <23> Unary [-]
                    │       ╰── <22> Var [negative_one]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30>  [>=]
                    │   │       ├── Constant ULong [68719476736]
                    │   │       ╰── <29> Var [negative_one]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [>=]
                    │   │       ├── <38> Unary [-]
                    │   │       │   ╰── Constant Long [2147483658]
                    │   │       ╰── <41> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54> Unary [!]
                    │   │       ╰── <53>  [+]
                    │   │           ├── Constant ULong [3]
                    │   │           ╰── Constant ULong [4294967293]
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
            │       ╰── Constant Long [1152921506754330636]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant UInt [2147483650]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372036854775900]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l2
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant UInt [2147483650]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Constant UInt [4294967294]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul2
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Constant Long [9223372036854775798]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i2
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372039002259606]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui2
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372039002259606]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> Var [u]
                    │   │       ╰── Constant UInt [2147483660]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <62> Var [i]
                    │   │       ╰── <66> Unary [-]
                    │   │           ╰── Constant Int [2147483646]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <72> Var [l]
                    │   │       ╰── <76> Unary [-]
                    │   │           ╰── Constant Long [9223372036854775716]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> Var [l2]
                    │   │       ╰── Constant Long [2147483650]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <90> Var [ul]
                    │   │       ╰── Constant ULong [4294967294]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> Var [ul2]
                    │   │       ╰── Constant ULong [9223372036854775798]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <106> Var [i2]
                    │   │       ╰── <110> Unary [-]
                    │   │           ╰── Constant Int [2147483498]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <116> Var [ui2]
                    │   │       ╰── Constant UInt [2147483798]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [8]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <33> Var [a]
                    │   │       ╰── Constant UInt [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <43> Var [b]
                    │   │       ╰── Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Var [c]
                    │   │       ╰── Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Var [d]
                    │   │       ╰── Constant ULong [9223372036854775808]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> Var [e]
                    │   │       ╰── Constant UInt [2147483648]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <83> Var [f]
                    │   │       ╰── Constant UInt [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <96>  [!=]
                    │   │       ├── <93> Var [g]
                    │   │       ╰── Constant UInt [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [!=]
                    │   │       ├── <103> Var [h]
                    │   │       ╰── Constant UInt [2147487744]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [!=]
                    │   │       ├── <113> Var [i]
                    │   │       ╰── Constant ULong [9223372041149743104]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ╰── Return
                        ╰── Constant Int [0]
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
                        ╰── <51> FunctionCall [accept_unsigned]
                            ├── Constant Int [1]
                            ├── <41> Unary [-]
                            │   ╰── Constant Int [1]
                            ├── <44> Unary [-]
                            │   ╰── Constant Int [1]
                            ├── Constant ULong [9223372036854775808]
                            ├── Constant ULong [2147483648]
                            ├── Constant Int [0]
                            ├── Constant Int [123456]
                            ├── Constant UInt [2147487744]
                            ╰── Constant ULong [9223372041149743104]
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
            │       ╰── Constant UInt [4294967200]
            ├── Function [return_uint]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Var [ui]
            ├── Function [return_uint_as_signed]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <21> Var [ui]
            ╰── Function [return_uint_as_long]
                ╰── Body
                    ╰── Return
                        ╰── <30> Var [ui]
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
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Var [ui]
                    │   │       ╰── Constant UInt [4294967200]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ├── <39> Assign [=]
                    │   ├── <34> Var [ui]
                    │   ╰── <38> Unary [-]
                    │       ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <48> Cast
                    │           ├── Target
                    │           │   ╰── Long
                    │           ╰── Expression
                    │               ╰── <47> FunctionCall [return_uint]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> Var [result]
                    │   │       ╰── Constant Long [4294967295]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ├── <67> Assign [=]
                    │   ├── <60> Var [result]
                    │   ╰── <66> Cast
                    │       ├── Target
                    │       │   ╰── Long
                    │       ╰── Expression
                    │           ╰── <65> FunctionCall [return_uint_as_signed]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <70> Var [result]
                    │   │       ╰── <74> Unary [-]
                    │   │           ╰── Constant Long [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ├── <84> Assign [=]
                    │   ├── <80> Var [result]
                    │   ╰── <83> FunctionCall [return_uint_as_long]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [result]
                    │   │       ╰── Constant Long [4294967295]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ╰── Return
                        ╰── Constant Int [1]
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
            │   │   ╰── Constant Int [5]
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
            │       ╰── Constant Int [7]
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
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> Var [i]
                    │   │       ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> Var [l]
                    │   │       ╰── Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── index
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <92>  [>]
                    │   │       ├── <89> Var [index]
                    │   │       ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <101> Assign [=]
                    │   │       ├── <94> Var [index]
                    │   │       ╰── <100>  [-]
                    │   │           ├── <97> Var [index]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Block
                    │       ╰── <110> Assign [=]
                    │           ├── <103> Var [counter]
                    │           ╰── <109>  [+]
                    │               ├── <106> Var [counter]
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <116> Var [counter]
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
            │       ╰── Constant Int [6]
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
            │       ╰── Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> Var [u]
                    │   │       ╰── Constant UInt [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
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
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Var [ul]
                    │   │       ╰── Constant ULong [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── index
                    │   │       ├── Type
                    │   │       │   ╰── Unsigned Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <89>  [<]
                    │   │       ├── <86> Var [index]
                    │   │       ╰── Constant UInt [4294967295]
                    │   ├── Condition
                    │   │   ╰── <98> Assign [=]
                    │   │       ├── <91> Var [index]
                    │   │       ╰── <97>  [-]
                    │   │           ├── <94> Var [index]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Block
                    │       ╰── <107> Assign [=]
                    │           ├── <100> Var [counter]
                    │           ╰── <106>  [+]
                    │               ├── <103> Var [counter]
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [!=]
                    │   │       ├── <113> Var [counter]
                    │   │       ╰── Constant Int [11]
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
            │           ╰── <29>  [==]
            │               ├── <25>  [+]
            │               │   ├── <22> Var [ui_a]
            │               │   ╰── Constant UInt [2147483653]
            │               ╰── Constant UInt [2147483663]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <46>  [==]
            │               ├── <42>  [-]
            │               │   ├── <38> Var [ul_a]
            │               │   ╰── <41> Var [ul_b]
            │               ╰── Constant ULong [18446744072635808792]
            ├── Function [multiplication]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <63>  [==]
            │               ├── <59>  [*]
            │               │   ├── <55> Var [ui_a]
            │               │   ╰── <58> Var [ui_b]
            │               ╰── Constant UInt [3221225472]
            ├── Function [division]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <80>  [==]
            │               ├── <76>  [/]
            │               │   ├── <72> Var [ui_a]
            │               │   ╰── <75> Var [ui_b]
            │               ╰── Constant Int [0]
            ├── Function [division_large_dividend]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <97>  [==]
            │               ├── <93>  [/]
            │               │   ├── <89> Var [ui_a]
            │               │   ╰── <92> Var [ui_b]
            │               ╰── Constant Int [2]
            ├── Function [division_by_literal]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <113>  [==]
            │               ├── <109>  [/]
            │               │   ├── <106> Var [ul_a]
            │               │   ╰── Constant ULong [5]
            │               ╰── Constant ULong [219902325555]
            ├── Function [remaind]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <130>  [==]
            │               ├── <126>  [%]
            │               │   ├── <122> Var [ul_b]
            │               │   ╰── <125> Var [ul_a]
            │               ╰── Constant ULong [5]
            ├── Function [complement]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <145>  [==]
            │               ├── <141> Unary [~]
            │               │   ╰── <140> Var [ui_a]
            │               ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <157> Assign [=]
                    │   ├── <154> Var [ui_a]
                    │   ╰── Constant UInt [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162> Unary [!]
                    │   │       ╰── <161> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <172> Assign [=]
                    │   ├── <169> Var [ul_a]
                    │   ╰── Constant ULong [18446744072635809792]
                    ├── <178> Assign [=]
                    │   ├── <175> Var [ul_b]
                    │   ╰── Constant ULong [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183> Unary [!]
                    │   │       ╰── <182> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <193> Assign [=]
                    │   ├── <190> Var [ui_a]
                    │   ╰── Constant UInt [1073741824]
                    ├── <199> Assign [=]
                    │   ├── <196> Var [ui_b]
                    │   ╰── Constant UInt [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <204> Unary [!]
                    │   │       ╰── <203> FunctionCall [multiplication]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <214> Assign [=]
                    │   ├── <211> Var [ui_a]
                    │   ╰── Constant UInt [100]
                    ├── <220> Assign [=]
                    │   ├── <217> Var [ui_b]
                    │   ╰── Constant UInt [4294967294]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <225> Unary [!]
                    │   │       ╰── <224> FunctionCall [division]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <235> Assign [=]
                    │   ├── <232> Var [ui_a]
                    │   ╰── Constant UInt [4294967294]
                    ├── <241> Assign [=]
                    │   ├── <238> Var [ui_b]
                    │   ╰── Constant UInt [2147483647]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <246> Unary [!]
                    │   │       ╰── <245> FunctionCall [division_large_dividend]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <256> Assign [=]
                    │   ├── <253> Var [ul_a]
                    │   ╰── Constant ULong [1099511627775]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <261> Unary [!]
                    │   │       ╰── <260> FunctionCall [division_by_literal]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── <271> Assign [=]
                    │   ├── <268> Var [ul_a]
                    │   ╰── Constant ULong [100]
                    ├── <277> Assign [=]
                    │   ├── <274> Var [ul_b]
                    │   ╰── Constant ULong [18446744073709551605]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <282> Unary [!]
                    │   │       ╰── <281> FunctionCall [remaind]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── <292> Assign [=]
                    │   ├── <289> Var [ui_a]
                    │   ╰── Constant UInt [4294967295]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <297> Unary [!]
                    │   │       ╰── <296> FunctionCall [complement]
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
            │           ╰── <29>  [==]
            │               ├── <26>  [+]
            │               │   ├── <22> Var [ui_a]
            │               │   ╰── <25> Var [ui_b]
            │               ╰── Constant UInt [0]
            ├── Function [subtraction]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <46>  [==]
            │               ├── <42>  [-]
            │               │   ├── <38> Var [ul_a]
            │               │   ╰── <41> Var [ul_b]
            │               ╰── Constant ULong [18446744073709551606]
            ├── Function [neg]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <60>  [==]
            │               ├── <57> Unary [-]
            │               │   ╰── <56> Var [ul_a]
            │               ╰── Constant ULong [18446744073709551615]
            ╰── Function [main]
                ╰── Body
                    ├── <72> Assign [=]
                    │   ├── <69> Var [ui_a]
                    │   ╰── Constant UInt [4294967293]
                    ├── <78> Assign [=]
                    │   ├── <75> Var [ui_b]
                    │   ╰── Constant UInt [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83> Unary [!]
                    │   │       ╰── <82> FunctionCall [addition]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <93> Assign [=]
                    │   ├── <90> Var [ul_a]
                    │   ╰── Constant ULong [10]
                    ├── <99> Assign [=]
                    │   ├── <96> Var [ul_b]
                    │   ╰── Constant ULong [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104> Unary [!]
                    │   │       ╰── <103> FunctionCall [subtraction]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <114> Assign [=]
                    │   ├── <111> Var [ul_a]
                    │   ╰── Constant ULong [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119> Unary [!]
                    │   │       ╰── <118> FunctionCall [neg]
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
            │       ╰── Constant UInt [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── large_uint
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Constant UInt [4294967294]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one_hundred_ulong
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Constant ULong [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── large_ulong
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Constant ULong [4294967294]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <34>  [<]
                    │   │       ├── <30> Var [large_uint]
                    │   │       ╰── <33> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [<=]
                    │   │       ├── <39> Var [large_uint]
                    │   │       ╰── <42> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [>=]
                    │   │       ├── <48> Var [one_hundred]
                    │   │       ╰── <51> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [>]
                    │   │       ├── <57> Var [one_hundred]
                    │   │       ╰── <60> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73> Unary [!]
                    │   │       ╰── <72>  [<=]
                    │   │           ├── <67> Var [one_hundred]
                    │   │           ╰── <70> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85> Unary [!]
                    │   │       ╰── <84>  [<]
                    │   │           ├── <79> Var [one_hundred]
                    │   │           ╰── <82> Var [large_uint]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97> Unary [!]
                    │   │       ╰── <96>  [>]
                    │   │           ├── <91> Var [large_uint]
                    │   │           ╰── <94> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109> Unary [!]
                    │   │       ╰── <108>  [>=]
                    │   │           ├── <103> Var [large_uint]
                    │   │           ╰── <106> Var [one_hundred]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [<]
                    │   │       ├── <114> Var [large_ulong]
                    │   │       ╰── <117> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [<=]
                    │   │       ├── <123> Var [large_ulong]
                    │   │       ╰── <126> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [>=]
                    │   │       ├── <132> Var [one_hundred_ulong]
                    │   │       ╰── <135> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [>]
                    │   │       ├── <141> Var [one_hundred_ulong]
                    │   │       ╰── <144> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157> Unary [!]
                    │   │       ╰── <156>  [<=]
                    │   │           ├── <151> Var [one_hundred_ulong]
                    │   │           ╰── <154> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <169> Unary [!]
                    │   │       ╰── <168>  [<]
                    │   │           ├── <163> Var [one_hundred_ulong]
                    │   │           ╰── <166> Var [large_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <181> Unary [!]
                    │   │       ╰── <180>  [>]
                    │   │           ├── <175> Var [large_ulong]
                    │   │           ╰── <178> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <193> Unary [!]
                    │   │       ╰── <192>  [>=]
                    │   │           ├── <187> Var [large_ulong]
                    │   │           ╰── <190> Var [one_hundred_ulong]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [16]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant ULong [8589934592]
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
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [a]
                    │   │       ╰── Constant ULong [8589934592]
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
                    │   │       ╰── Constant UInt [10]
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
                    │   ╰── <113>  [*]
                    │       ├── <110> Var [d]
                    │       ╰── Constant UInt [268435456]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Var [a]
                    │   │       ╰── Constant ULong [18446744065119617024]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <127> Var [b]
                    │   │       ╰── <131> Unary [-]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <139> Var [c]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <149> Var [d]
                    │   │       ╰── Constant UInt [2684354560]
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
            │           ╰── <11> Unary [!]
            │               ╰── <10> Var [ul]
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
            │       │   │   ╰── <23> Var [u]
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
            │           ╰── <48>  [&&]
            │               ├── <44> Var [ul]
            │               ╰── <47> Var [i]
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
            │           ╰── <67>  [||]
            │               ├── <63> Var [i]
            │               ╰── <66> Var [u]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [1152921504606846976]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [2147483648]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <96> FunctionCall [not]
                    │   │       ╰── <95> Var [ul]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107> Unary [!]
                    │   │       ╰── <106> FunctionCall [not]
                    │   │           ╰── <105> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118> Unary [!]
                    │   │       ╰── <117> FunctionCall [if_cond]
                    │   │           ╰── <116> Var [u]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127> FunctionCall [if_cond]
                    │   │       ╰── <126> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137> FunctionCall [and]
                    │   │       ├── <135> Var [zero]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149> Unary [!]
                    │   │       ╰── <148> FunctionCall [or]
                    │   │           ├── Constant Int [1]
                    │   │           ╰── <147> Var [u]
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
                    │       ╰── Constant UInt [2147483647]
                    ╰── Return
                        ╰── <19>  [==]
                            ├── <15>  [+]
                            │   ├── <12> Var [u]
                            │   ╰── Constant UInt [2]
                            ╰── Constant UInt [2147483649]
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
            │   │   ╰── Constant ULong [9223372036854775803]
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
                    │   │   ╰── <24>  [!=]
                    │   │       ├── <21> Var [x]
                    │   │       ╰── Constant ULong [9223372036854775803]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ├── <36> Assign [=]
                    │   ├── <29> Var [x]
                    │   ╰── <35>  [+]
                    │       ├── <32> Var [x]
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [!=]
                    │   │       ├── <39> Var [x]
                    │   │       ╰── Constant ULong [9223372036854775813]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [||]
                    │   │       ├── <47> Var [zero_long]
                    │   │       ╰── <50> Var [zero_int]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [0]
                    ╰── Return
                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}
