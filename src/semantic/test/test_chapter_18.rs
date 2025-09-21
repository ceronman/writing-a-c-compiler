use super::assert_error;

#[test]
fn test_invalid_struct_tags_array_of_undeclared() {
    assert_error(
        r#"
        int main(void) {
            struct s arr[2];
          //^^^^^^^^ Undeclared structure type 's'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_cast_undeclared() {
    assert_error(
        r#"
        int main(void) {
            (struct s)0;
           //^^^^^^^^ Undeclared structure type 's'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_deref_undeclared() {
    assert_error(
        r#"
        int main(void) {
            struct s *ptr = 0;
          //^^^^^^^^ Undeclared structure type 's'
            *ptr;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_extra_credit_sizeof_undeclared_union() {
    assert_error(
        r#"
        int main(void) {
            return sizeof(union c);
                        //^^^^^^^ Undeclared structure type 'c'
        }
        union c {
            int x;
        };
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_extra_credit_var_undeclared_union_type() {
    assert_error(
        r#"
        int main(void) {
            union s var;
          //^^^^^^^ Undeclared structure type 's'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_file_scope_var_type_undeclared() {
    assert_error(
        r#"
        struct s var;
      //^^^^^^^^ Undeclared structure type 's'
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_for_loop_scope() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0; i < 10; i = i + 1) {
                struct s {
                    int a;
                };
            }
            struct s x;
          //^^^^^^^^ Undeclared structure type 's'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_for_loop_scope_2() {
    assert_error(
        r#"
        int main(void) {
            void *ptr;
            for (;; ((struct s *)ptr)->i) {
                    //^^^^^^^^ Undeclared structure type 's'
                struct s {
                    int i;
                };
                struct s x = {1};
                ptr = &x;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_member_type_undeclared() {
    assert_error(
        r#"
        struct s {
            struct a b;
          //^^^^^^^^ Undeclared structure type 'a'
        };
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_param_undeclared() {
    assert_error(
        r#"
        int foo(struct s x) {
              //^^^^^^^^ Undeclared structure type 's'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_return_type_undeclared() {
    assert_error(
        r#"
        void exit(int status);
        struct s foo(void) {
      //^^^^^^^^ Undeclared structure type 's'
            exit(0);
        }
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_sizeof_undeclared() {
    assert_error(
        r#"
        int main(void) {
            return sizeof(struct c);
                        //^^^^^^^^ Undeclared structure type 'c'
        }
        struct c {
            int x;
        };
    "#,
    );
}

#[test]
fn test_invalid_struct_tags_var_type_undeclared() {
    assert_error(
        r#"
        int main(void) {
            struct s var;
          //^^^^^^^^ Undeclared structure type 's'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bad_union_member_access_nested_non_member() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        union u {
            struct s nested;
        };
        int main(void) {
            union u my_union = {{1}};
            return my_union.a;
                 //^^^^^^^^ Aggregate type 'u.1' does not have field 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bad_union_member_access_union_bad_member() {
    assert_error(
        r#"
        union s {
            int x;
            int y;
        };
        union t {
            int blah;
            int y;
        };
        int main(void) {
            union s foo = {1};
            return foo.blah;
                 //^^^ Aggregate type 's.0' does not have field 'blah'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bad_union_member_access_union_bad_pointer_member() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        union a {
          int x;
          int y;
        };
        union b {
          int m;
          int n;
        };
        int main(void) {
          union a *ptr = malloc(sizeof(union a));
          ptr->m = 10;
        //^^^ Aggregate type 'a.1' does not have field 'm'
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incompatible_union_types_assign_different_union_type() {
    assert_error(
        r#"
        union u1 {int a;};
        union u2 {int a;};
        int main(void){
            union u1 x = {10};
            union u2 y = {11};
            x = y;
              //^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incompatible_union_types_assign_scalar_to_union() {
    assert_error(
        r#"
        union u {int a; int b;};
        int main(void) {
            union u x = {1};
            x = 2;
              //^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incompatible_union_types_return_type_mismatch() {
    assert_error(
        r#"
        union u {
            int x;
        };
        union u return_union(void){
            union u {
                int x;
            };
            union u result = {10};
            return result;
                 //^^^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incompatible_union_types_union_branch_mismatch() {
    assert_error(
        r#"
        int main(void) {
            union u1 {
                int a;
            };
            union u2 {
                int a;
            };
            union u1 x = {10};
            union u2 y = {11};
            1 ? x : y;
          //^^^^^^^^^ Invalid types of the branches
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incompatible_union_types_union_pointer_branch_mismatch() {
    assert_error(
        r#"
        int main(void) {
            union u1;
            union u2;
            union u1 *ptr1 = 0;
            union u2 *ptr2 = 0;
            1 ? ptr1 : ptr2;
              //^^^^^^^^^^^ Expressions have incompatible types
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incomplete_unions_define_incomplete_union() {
    assert_error(
        r#"
        union u;
        union u my_union;
      //^^^^^^^ Incomplete aggregate type
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_incomplete_unions_sizeof_incomplete_union_type() {
    assert_error(
        r#"
        int main(void) {
            union u;
            return sizeof(union u);
                        //^^^^^^^ Cannot get size of an incomplete type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_invalid_union_lvalues_address_of_non_lvalue_union_member() {
    assert_error(
        r#"
        union u {
            int arr[3];
            double d;
        };
        union u get_union(void) {
            union u result = {{1, 2, 3}, 4.0};
                           //^^^^^^^^^^^^^^^^ Too many elements in the initializer
            return result;
        }
        int main(void) {
            int *ptr[3] = &get_union().arr;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_invalid_union_lvalues_assign_non_lvalue_union_member() {
    assert_error(
        r#"
        
        union inner {
            int y;
            long z;
        };
        union u {
            int x;
            union inner i;
        };
        union u return_union(void){
            union u result = {1};
            return result;
        }
        int main(void) {
            return_union().i.y = 1;
          //^^^^^^^^^^^^^^^^^^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_bitwise_op_structure() {
    assert_error(
        r#"
        
        struct s {int i;};
        int main(void) {
            struct s x = {100};
            int i = 1000;
            x & i;
          //^ Invalid operator type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_compound_assign_struct_rval() {
    assert_error(
        r#"
        
        struct s { int i; };
        int main(void) {
            int i = 100;
            struct s x = { 100 };
            i += x;
               //^ Cannot compound assign an aggregate type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_compound_assign_to_nested_struct() {
    assert_error(
        r#"
        struct inner {
            int i;
        };
        struct outer {
            struct inner s;
        };
        int main(void) {
            struct outer x = {{1}};
            x.s *= 10;
          //^^^ Assign compound operation cannot be used on pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_compound_assign_to_struct() {
    assert_error(
        r#"
        
        struct s { int i; };
        int main(void) {
            struct s x = {10};
            x += 10;
          //^ Cannot compound assign an aggregate type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_duplicate_struct_types_after_label() {
    assert_error(
        r#"
        int main(void) {
            struct s {
                int a;
            };
        foo:;
            struct s {
                 //^ Aggregate type 's.0' was already declared
                int b;
            };
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_postfix_decr_struct_arrow() {
    assert_error(
        r#"
        
        struct inner {int i;};
        struct outer{struct inner s;};
        int main(void) {
            struct outer my_struct = {{1}};
            struct outer *ptr = &my_struct;
            ptr->s--;
          //^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_postfix_incr_struct() {
    assert_error(
        r#"
        
        struct s {
            int i;
        };
        int main(void) {
            struct s my_struct = {1};
            my_struct++;
          //^^^^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_prefix_decr_struct() {
    assert_error(
        r#"
        
        struct s {
            int i;
        };
        int main(void) {
            struct s my_struct = {1};
            --my_struct;
            //^^^^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_prefix_incr_nested_struct() {
    assert_error(
        r#"
        
        struct inner {
            int i;
        };
        struct outer {
            struct inner s;
        };
        int main(void) {
            struct outer x = {{1}};
            ++x.s;
            //^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_other_features_switch_on_struct() {
    assert_error(
        r#"
        
        struct s {
            int i;
        };
        int main(void) {
            struct s x = {1};
            switch (x) {
                  //^ Switch statement requires an integer expression
                case 1:
                    return 0;
                default:
                    return 1;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_scalar_required_cast_between_unions() {
    assert_error(
        r#"
        union u1 {
            int a;
        };
        int main(void){
            union u1 var = {10};
            (union u1) var;
           //^^^^^^^^ Cannot cast a value to a non-scalar type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_scalar_required_cast_union_to_int() {
    assert_error(
        r#"
        
        union u {
            int i;
        };
        int main(void) {
            union u x = {10};
            return (int)x;
                      //^ Cannot cast non-scalar expression
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_scalar_required_compare_unions() {
    assert_error(
        r#"
        union u { long l; };
        int main(void){
            union u x = {1};
            x == x;
          //^ Invalid operator type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_scalar_required_switch_on_union() {
    assert_error(
        r#"
        
        union s {
            int i;
        };
        int main(void) {
            union s x = {1};
            switch (x) {
                  //^ Switch statement requires an integer expression
                case 1:
                    return 0;
                default:
                    return 1;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_scalar_required_union_as_controlling_expression() {
    assert_error(
        r#"
        union u {int x;};
        int main(void) {
            union u my_union = {10};
            if (my_union) {
              //^^^^^^^^ Invalid condition type
                return 1;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_initializer_too_long() {
    assert_error(
        r#"
        union u {
            int a;
            long b;
        };
        int main(void){
            union u x = {1, 2};
                      //^^^^^^ Too many elements in the initializer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_nested_init_wrong_type() {
    assert_error(
        r#"
        union u {
            double d;
            int i;
            char c;
        };
        struct s {
            int *ptr;
            union u arr[3];
        };
        int main(void) {
            int x;
            struct s my_struct = {&x, {{1.0}, {2.0}, {&x}}};
                                                    //^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_nested_union_init_too_long() {
    assert_error(
        r#"
        int main(void) {
            union u {
                double d; int x;
            };
            union u array_of_unions[3] = {
                {1.0, 2.0, 3.0}
              //^^^^^^^^^^^^^^^ Too many elements in the initializer
            };
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_scalar_union_initializer() {
    assert_error(
        r#"
        
        union u {int a;};
        int main(void){
            union u my_union = 1;
                             //^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_aggregate_init_wrong_type() {
    assert_error(
        r#"
        struct one_elem {
            long l;
        };
        struct three_elems {
            int one;
            int two;
            int three;
        };
        union one_or_three_elems {
            struct one_elem a;
            struct three_elems b;
        };
        int main(void) {
            static union one_or_three_elems my_union = {{1, 2, 3}};
                                                      //^^^^^^^^^ Too many elements in the initializer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_nested_init_not_const() {
    assert_error(
        r#"
        union u {
            long l;
        };
        struct has_union {
            int a;
            union u b;
            char c;
        };
        long some_var = 10l;
        struct has_union some_struct = {1,
                                        {some_var},
                                       //^^^^^^^^ Non-constant initializer on local static variable
                                        'a'};
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_nested_init_too_long() {
    assert_error(
        r#"
        
        union u {
            int a;
            long b;
        };
        struct s {
            int tag;
            union u contents;
        };
        struct s my_struct = {
            10,
            {1, 2}
          //^^^^^^ Too many elements in the initializer
        };
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_scalar_union_initializer() {
    assert_error(
        r#"
        
        union u {int a;};
        int main(void){
            static union u my_union = 1;
                                    //^ Invalid type of static declaration
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_too_long() {
    assert_error(
        r#"
        union u {
            int a;
            long b;
        };
        union u x = {1, 2};
                  //^^^^^^ Too many elements in the initializer
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_union_init_not_constant() {
    assert_error(
        r#"
        union u {int a; int b;};
        int main(void){
            int i = 10;
            static union u my_union = {i};
                                     //^ Non-constant initializer on local static variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_static_union_init_wrong_type() {
    assert_error(
        r#"
        union u {
            signed char *ptr;
            double d;
        };
        int main(void) {
            static union u my_union = {"A char array"};
                                     //^^^^^^^^^^^^^^ Can't initialize a non-character pointer to a string literal
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_initializers_union_init_wrong_type() {
    assert_error(
        r#"
        union u {
            long *ptr;
            double d;
        };
        int main(void) {
            union u my_union = {1.0};
                              //^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_conflicting_tag_decl_and_use() {
    assert_error(
        r#"
        struct x { int a; };
        int main(void) {
            union x foo;
          //^^^^^^^ Type is not a union type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_conflicting_tag_decl_and_use_self_reference(
) {
    assert_error(
        r#"
        int main(void) {
            struct s;
            {
                union s* ptr;
              //^^^^^^^ Type is not a union type
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_conflicting_tag_declarations() {
    assert_error(
        r#"
        
        struct x;
        union x;
            //^ Tag does not match previous declaration
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_struct_shadowed_by_union() {
    assert_error(
        r#"
        int main(void) {
            struct tag {int a;};
            {
                union tag {long l;};
                struct tag *x;
              //^^^^^^^^^^ Type is not a struct type
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_tag_decl_conflicts_with_def() {
    assert_error(
        r#"
        int main(void) {
            struct s;
            union s {
                //^ Tag does not match previous declaration
                int a;
            };
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_tag_def_conflicts_with_decl() {
    assert_error(
        r#"
        int main(void) {
            union s {
                int a;
            };
            struct s;
                 //^ Tag `s.0` does not match previous declaration
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_struct_conflicts_union_shadowed_by_incomplete_struct() {
    assert_error(
        r#"
        int main(void) {
            union tag {int a;};
            {
                struct tag;
                union tag *x;
              //^^^^^^^^^ Type is not a union type
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_tag_resolution_address_of_wrong_union_type() {
    assert_error(
        r#"
        union u {
            int i;
            char c;
        };
        int main(void) {
            union u foo = {0};
            union u {
                int i;
                char c;
            };
            union u *ptr = &foo;
                         //^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_tag_resolution_compare_struct_and_union_ptrs() {
    assert_error(
        r#"
        int main(void) {
            struct tag;
            struct tag *struct_ptr = 0;
            {
                union tag;
                union tag *union_ptr = 0;
                return (struct_ptr == union_ptr);
                      //^^^^^^^^^^^^^^^^^^^^^^^ Expressions have incompatible types
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_tag_resolution_conflicting_param_union_types() {
    assert_error(
        r#"
        struct s;
        int foo(struct s x);
        int main(void) {
            union s;
            int foo(union s x);
              //^^^ Conflicting declaration types for 'foo'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_tag_resolution_distinct_union_types() {
    assert_error(
        r#"
        int foo(void) {
            union s {
                int a;
                long b;
            };
            union s result = {1};
            return result.a;
        }
        int main(void) {
            union s;
            union s blah = {foo()};
          //^^^^^^^ Incomplete type
            return blah.a;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_tag_resolution_union_type_shadows_struct() {
    assert_error(
        r#"
        struct u {
            int a;
        };
        int main(void) {
            union u;
            union u my_union;
          //^^^^^^^ Incomplete type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_tag_resolution_union_wrong_member() {
    assert_error(
        r#"
        union u {
            int a;
        };
        int main(void) {
            union u foo = {1};
            union u {
                int b;
            };
            return foo.b;
                 //^^^ Aggregate type 'u.0' does not have field 'b'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_type_declarations_array_of_incomplete_union_type() {
    assert_error(
        r#"
        union u;
        int main(void) {
            union u(*arr)[3];
          //^^^^^^^^^^^^^^^^ Illegal array of incomplete types
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_type_declarations_duplicate_union_def() {
    assert_error(
        r#"
        int main(void) {
            union u {int a;};
            union u {int a;};
                //^ Aggregate type 'u.0' was already declared
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_union_type_declarations_member_name_conflicts() {
    assert_error(
        r#"
        
        union u {
            int a;
            int a;
              //^ Field name `a` already exists in the type definition
        };
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_assign_different_pointer_type() {
    assert_error(
        r#"
        struct s1;
        struct s2;
        int main(void) {
            struct s1 *p1 = 0;
            struct s2 *p2 = 0;
            p2 = p1;
               //^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_assign_different_struct_type() {
    assert_error(
        r#"
        struct s1 {
            int field;
        };
        struct s2 {
            int field;
        };
        int main(void) {
            struct s1 a = {1} ;
            struct s2 b;
            b = a;
              //^ Cannot convert type for assignment!
            return b.field;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_branch_mismatch() {
    assert_error(
        r#"
        struct s1 {
          int a;
        };
        struct s2 {
          int b;
        };
        int main(void) {
          struct s1 x = {1};
          struct s2 y = {2};
          1 ? x : y;
        //^^^^^^^^^ Invalid types of the branches
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_branch_mismatch_2() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s x = {1};
            1 ? x : (void) 2;
          //^^^^^^^^^^^^^^^^ Invalid types of the branches
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_compare_different_struct_pointers() {
    assert_error(
        r#"
        struct s1;
        struct s2;
        struct s1 *get_s1_ptr(void);
        struct s2 *get_s2_ptr(void);
        int main(void) {
          struct s1 *s1_ptr = get_s1_ptr();
          struct s2 *s2_ptr = get_s2_ptr();
          return s1_ptr == s2_ptr;
               //^^^^^^^^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_return_wrong_struct_type() {
    assert_error(
        r#"
        struct one {
          int x;
          int y;
        };
        struct two {
          int a;
          int b;
        };
        struct one return_struct(void) {
            struct two retval = {1, 2};
            return retval;
                 //^^^^^^ Cannot convert type for assignment!
        }
        int main(void) {
            return return_struct().x;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_struct_param_mismatch() {
    assert_error(
        r#"
        struct one {
          int x;
          int y;
        };
        struct two {
          int a;
          int b;
        };
        int take_struct_param(struct one param) {
            return param.x;
        }
        int main(void) {
            struct two arg = {1, 2};
            return take_struct_param(arg);
                                   //^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_types_struct_pointer_param_mismatch() {
    assert_error(
        r#"
        struct s1 {
            int a;
        };
        struct s2 {
            int a;
        };
        int get_a(struct s1 *ptr) {
            return ptr->a;
        }
        int main(void) {
            struct s2 arg = {1};
            return get_a(&arg);
                       //^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_compound_initializer_too_long() {
    assert_error(
        r#"
        struct pair {
            int a;
            int b;
        };
        int main(void) {
            struct pair p = {1, 2, 3};
                          //^^^^^^^^^ Too many elements in the initializer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_init_struct_with_string() {
    assert_error(
        r#"
        struct chars {
            char a;
            char b;
            char c;
            char null;
        };
        int main(void) {
            struct chars my_chars = "abc";
                                  //^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_initialize_nested_static_struct_member_wrong_type() {
    assert_error(
        r#"
        struct s {
          double d;
          void *arr[3];
        };
        struct s x = {0.0, {1.0}};
                          //^^^ Invalid type of static declaration
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_initialize_static_struct_with_zero() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        struct s x = 0;
                   //^ Invalid type of static declaration
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_initialize_struct_member_wrong_type() {
    assert_error(
        r#"
        struct s {
            signed char *char_ptr;
        };
        int main(void) {
            struct s x = {"It's a string"};
                        //^^^^^^^^^^^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_initialize_struct_with_scalar() {
    assert_error(
        r#"
        int main(void) {
            struct pair {
                int x;
                int y;
            };
            struct pair p = 1;
                          //^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_initialize_struct_wrong_type() {
    assert_error(
        r#"
        struct one {
          int x;
          int y;
        };
        struct two {
          int a;
          int b;
        };
        int main(void) {
          struct one x = {1, 2};
          struct two y = x;
                       //^ Cannot convert type for assignment!
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_nested_compound_initializer_too_long() {
    assert_error(
        r#"
        struct inner {
            int i;
        };
        struct outer {
            struct inner foo;
        };
        int main(void) {
            struct outer x = {{1, 2}};
                            //^^^^^^ Too many elements in the initializer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_nested_static_compound_initializer_too_long() {
    assert_error(
        r#"
        struct inner {
            int i;
        };
        struct outer {
            struct inner foo;
        };
        struct outer x = {{1, 2}};
                        //^^^^^^ Too many elements in the initializer
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_nested_struct_initializer_wrong_type() {
    assert_error(
        r#"
        struct inner {
          int a;
          int b;
        };
        struct outer {
          struct inner x;
        };
        int main(void) {
          struct outer x = {{1, 2}};
          struct outer y = {1, x};
                         //^^^^^^ Too many elements in the initializer
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_non_constant_static_elem_init() {
    assert_error(
        r#"
        struct pair {
          int a;
          int b;
        };
        struct pair x = {1, 2};
        struct outer {
            double d;
            struct pair inner;
        };
        struct outer y = {1.0, x};
                             //^ Non-constant initializer on local static variable
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_non_constant_static_init() {
    assert_error(
        r#"
        struct pair {
          int a;
          int b;
        };
        struct pair x = {1, 2};
        struct pair y = x;
                      //^ Non-constant initializer on local static variable
    "#,
    );
}

#[test]
fn test_invalid_types_initializers_static_initializer_too_long() {
    assert_error(
        r#"
        struct pair {
            int a;
            int b;
        };
        struct pair p = {1, 2, 3};
                      //^^^^^^^^^ Too many elements in the initializer
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_assign_to_incomplete_var() {
    assert_error(
        r#"
        struct s;
        extern struct s x;
        extern struct s y;
        int main(void) {
          x = y;
        //^^^^^ Type is not complete
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_cast_incomplete_struct() {
    assert_error(
        r#"
        struct s;
        extern struct s v;
        int main(void) {
          (void)v;
              //^ Incomplete aggregate type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_deref_incomplete_struct_pointer() {
    assert_error(
        r#"
        struct s;
        struct s *ptr = 0;
        int main(void) {
          *ptr;
        //^^^^ Incomplete aggregate type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_arg_funcall() {
    assert_error(
        r#"
        struct s;
        void f(struct s param);
        extern struct s extern_var;
        int main(void) {
          f(extern_var);
          //^^^^^^^^^^ Incomplete aggregate type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_array_element() {
    assert_error(
        r#"
        struct s;
        struct s arr[3];
      //^^^^^^^^^^^^^^^ Illegal array of incomplete types
        struct s {
            int a;
            int b;
        };
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_local_var() {
    assert_error(
        r#"
        struct s;
        int main(void) {
          struct s v;
        //^^^^^^^^ Incomplete type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_param() {
    assert_error(
        r#"
        struct s;
        int foo(struct s x) { return 0; }
              //^^^^^^^^ Aggregate type is not complete
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_ptr_addition() {
    assert_error(
        r#"
        struct s;
        extern struct s *ptr;
        int main(void) {
          return ptr + 0 == ptr;
               //^^^ Cannot add pointers to incomplete types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_ptr_subtraction() {
    assert_error(
        r#"
        struct s;
        extern struct s *ptr;
        int main(void) {
          return (ptr - ptr) == 0;
                //^^^ Incomplete pointer type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_return_type_fun_def() {
    assert_error(
        r#"
        void exit(int status);
        struct s;
        struct s return_struct_def(void) {
      //^^^^^^^^ Aggregate type is not complete
          exit(0);
        }
        int main(void) { return 0; }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_return_type_funcall() {
    assert_error(
        r#"
        struct s;
        struct s f(void);
        int main(void) {
          f();
        //^ Incomplete aggregate type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_struct_conditional() {
    assert_error(
        r#"
        struct s;
        extern struct s v1;
        extern struct s v2;
        int main(void) {
          1 ? v1 : v2;
            //^^ Incomplete aggregate type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_struct_full_expr() {
    assert_error(
        r#"
        struct s;
        extern struct s x;
        int main(void) {
          for (x;;)
             //^ Incomplete aggregate type
            ;
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_struct_member() {
    assert_error(
        r#"
        struct s;
        extern struct s foo;
        int main(void) {
          return foo.a;
               //^^^ Incomplete aggregate type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_subscript() {
    assert_error(
        r#"
        struct s;
        extern struct s *ptr;
        int main(void) { ptr[0]; }
                       //^^^ Cannot subscript a pointer to void type
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_incomplete_tentative_def() {
    assert_error(
        r#"
        struct s;
        static struct s x;
      //^^^^^^^^^^^^^^^ Incomplete aggregate type
        int main(void) { return 0; }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_initialize_incomplete() {
    assert_error(
        r#"
        struct s;
        extern struct s x = {1};
      //^^^^^^^^^^^^^^^ Incomplete aggregate type
        int main(void) { return 0; }
        struct s {
          int a;
        };
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_sizeof_incomplete() {
    assert_error(
        r#"
        struct s;
        int main(void) {
          return sizeof(struct s);
                      //^^^^^^^^ Cannot get size of an incomplete type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_incomplete_structs_sizeof_incomplete_expr() {
    assert_error(
        r#"
        struct s;
        struct s *get_ptr(void);
        int main(void) {
            struct s *struct_ptr = get_ptr();
            return sizeof(*struct_ptr);
                       //^^^^^^^^^^^^^ Cannot get size of an incomplete type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_lvalues_address_of_non_lvalue() {
    assert_error(
        r#"
        struct s {
          int arr[3];
          double d;
        };
        int main(void) {
          struct s x = {{1, 2, 3}, 4.0};
          struct s y = {{9, 8, 7}, 6.0};
          int *arr[3] = &((1 ? x : y).arr);
                      //^^^^^^^^^^^^^^^^^^ Can't take address of non-lvalue!
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_lvalues_assign_nested_non_lvalue() {
    assert_error(
        r#"
        struct inner {
          int x;
          int y;
        };
        struct outer {
          int a;
          struct inner b;
        };
        struct outer return_struct(void) {
          struct outer result = {1, {2, 3}};
          return result;
        }
        int main(void) {
          return_struct().b.x = 10;
        //^^^^^^^^^^^^^^^^^^^ Expression is not assignable
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_lvalues_assign_to_array() {
    assert_error(
        r#"
        struct chars {
          char char_array[5];
        };
        int main(void) {
          struct chars x = {{1, 2, 3, 4, 5}};
          char arr[5] = {9, 8, 7, 6, 5};
          x.char_array = arr;
        //^^^^^^^^^^^^ Type is not assignable
          return x.char_array[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_lvalues_assign_to_non_lvalue() {
    assert_error(
        r#"
        struct s {
          int arr[3];
          double d;
        };
        int main(void) {
          struct s x = {{1, 2, 3}, 4.0};
          struct s y = {{9, 8, 7}, 6.0};
          (1 ? x : y).d = 0.0;
        //^^^^^^^^^^^^^ Expression is not assignable
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_arrow_pointer_to_non_struct() {
    assert_error(
        r#"
        struct s {
            long l;
        };
        int main(void) {
            double d = 0.0;
            double* ptr = &d;
            return ptr->l;
                 //^^^ Type is not a struct or union
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_bad_member() {
    assert_error(
        r#"
        struct s {
            int x;
            int y;
        };
        struct t {
            int blah;
            int y;
        };
        int main(void) {
            struct s foo = {1, 2};
            return foo.blah;
                 //^^^ Aggregate type 's.0' does not have field 'blah'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_bad_pointer_member() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        struct a {
          int x;
          int y;
        };
        struct b {
          int m;
          int n;
        };
        int main(void) {
          struct a *ptr = malloc(sizeof(struct a));
          ptr->m = 10;
        //^^^ Aggregate type 'a.1' does not have field 'm'
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_member_of_non_struct() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        struct a {
          int x;
          int y;
        };
        int main(void) {
          struct a *ptr = malloc(sizeof(struct a));
          ptr.x = 10;
        //^^^ Type is not a struct or union
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_member_pointer_non_struct_pointer() {
    assert_error(
        r#"
        struct a {
          int x;
          int y;
        };
        int main(void) {
          struct a my_struct = {1, 2};
          return my_struct->x;
               //^^^^^^^^^ Type is not a pointer
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_nested_arrow_pointer_to_non_struct() {
    assert_error(
        r#"
        struct s {
            long l;
        };
        struct has_ptr {
            double *ptr;
        };
        int main(void) {
            double d = 0.0;
            struct has_ptr p_struct = { &d };
            return p_struct.ptr->l;
                 //^^^^^^^^^^^^ Type is not a struct or union
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_member_operators_postfix_precedence() {
    assert_error(
        r#"
        int main(void) {
            struct s {
                int a;
            };
            struct s x = {10};
            return &x->a;
                  //^ Type is not a pointer
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_struct_declaration_duplicate_member_name() {
    assert_error(
        r#"
        struct s {
          int x;
          double x;
               //^ Field name `x` already exists in the type definition
        };
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_struct_declaration_duplicate_struct_declaration() {
    assert_error(
        r#"
        int main(void) {
            struct x {
                int x;
            };
            struct x {
                 //^ Aggregate type 'x.0' was already declared
                int y;
            };
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_struct_declaration_invalid_array_member() {
    assert_error(
        r#"
        struct incomplete;
        struct s {
          struct incomplete (*array_pointer)[3];
        //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Illegal array of incomplete types
        };
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_and_struct() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s x = {1};
            return 0 && x;
                      //^ Invalid operator type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_assign_null_ptr_to_struct() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        struct s x = {1};
        int main(void) {
          x = 0;
            //^ Cannot convert type for assignment!
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_assign_scalar_to_struct() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        struct s x = {1};
        int main(void) {
          struct s *ptr = &x;
          *ptr = 2;
               //^ Cannot convert type for assignment!
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_cast_struct_to_scalar() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        int main(void) {
          struct s x = {1};
          int y = (int)x;
                     //^ Cannot cast non-scalar expression
          return y;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_cast_to_struct() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        struct s x;
        int main(void) { (struct s) x; }
                        //^^^^^^^^ Cannot cast a value to a non-scalar type
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_compare_structs() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s x = {1};
            struct s y = {2};
            return x == y;
                 //^ Invalid operator type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_not_struct() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s x = {1};
            return !x;
                  //^ Unary operator requires a scalar operator
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_pass_struct_as_scalar_param() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        int foo(int a) { return a; }
        int main(void) {
          struct s x = {1};
          return foo(x);
                   //^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_struct_as_int() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        int main(void) {
          struct s x = {1};
          (void)~x;
               //^ Unary operator requires an integer type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_struct_controlling_expression() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        int main(void) {
          struct s x = {1};
          if (x)
            //^ Invalid condition type
            return 1;
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_required_subscript_struct() {
    assert_error(
        r#"
        struct s {
          int a;
        };
        int main(void) {
          struct s x = {1};
          return x[0];
               //^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_address_of_wrong_type() {
    assert_error(
        r#"
        struct s {
            int i;
        };
        int main(void) {
            struct s foo = {0};
            struct s {
                int i;
            };
            struct s *ptr = &foo;
                          //^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_conflicting_fun_param_types() {
    assert_error(
        r#"
        struct s;
        int foo(struct s x);
        int main(void) {
            struct s;
            int foo(struct s x);
              //^^^ Conflicting declaration types for 'foo'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_conflicting_fun_ret_types() {
    assert_error(
        r#"
        struct s;
        struct s foo(void);
        int main(void) {
            struct s;
            struct s foo(void);
                   //^^^ Conflicting declaration types for 'foo'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_distinct_struct_types() {
    assert_error(
        r#"
        int foo(void) {
            struct s {
                int a;
                int b;
            };
            struct s result = {1, 2};
            return result.a + result.b;
        }
        int main(void) {
            struct s;
            struct s blah = {foo(), foo()};
          //^^^^^^^^ Incomplete type
            return blah.a;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_incomplete_shadows_complete() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s;
            struct s *x;
            x->a = 10;
          //^ Type is not a struct or union
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_incomplete_shadows_complete_cast() {
    assert_error(
        r#"
        
        void *malloc(unsigned long size);
        struct s {
            int a;
        };
        int main(void) {
            void *ptr = malloc(sizeof(struct s));
            struct s;
            ((struct s *)ptr)->a = 10;
          //^^^^^^^^^^^^^^^^^ Type is not a struct or union
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_member_name_wrong_scope() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct s foo = {1};
            struct s {
                int b;
            };
            return foo.b;
                 //^^^ Aggregate type 's.0' does not have field 'b'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_member_name_wrong_scope_nested() {
    assert_error(
        r#"
        struct s {
            int a;
        };
        int main(void) {
            struct outer {
                struct s inner;
            };
            struct outer foo = {{1}};
            struct s {
                int b;
            };
            struct outer *ptr = &foo;
            return ptr->inner.b;
                 //^^^^^^^^^^ Aggregate type 's.0' does not have field 'b'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_mismatched_return_type() {
    assert_error(
        r#"
        struct s {
          int a;
          int b;
        };
        struct s return_struct(void) {
          struct s {
            int a;
            int b;
          };
          struct s result = {1, 2};
          return result;
               //^^^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_shadow_struct() {
    assert_error(
        r#"
        struct s;
        struct s *ptr1 = 0;
        int main(void) {
          struct s;
          struct s *ptr2 = 0;
          return ptr1 == ptr2;
               //^^^^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_tag_resolution_shadowed_tag_branch_mismatch() {
    assert_error(
        r#"
        int main(void) {
            struct s {
                int i;
            };
            struct s struct1 = {1};
            {
                struct s {
                    int i;
                };
                struct s struct2 = {2};
                (void)(1 ? struct1 : struct2);
                    //^^^^^^^^^^^^^^^^^^^^^^^ Invalid types of the branches
            }
        }
    "#,
    );
}
