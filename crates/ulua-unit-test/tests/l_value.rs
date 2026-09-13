extern crate alloc;

mod l_value_hashing_lvalue_global_prop_access {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:146:l_value_hashing_lvalue_global_prop_access`
  //! Source: `tests/LValue.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/LValue.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/LValue.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Field (Analysis/include/Luau/LValue.h)
  //!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record LValueHasher (Analysis/include/Luau/LValue.h)
  //!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
  //!   - translates_to -> rust_item l_value_hashing_lvalue_global_prop_access

  #[cfg(test)]
  #[test]
  fn l_value_hashing_lvalue_global_prop_access() {
    use std::{ffi::CString, sync::Arc};

    use ulua_analysis::{
      records::{
        builtin_types::BuiltinTypes, field::Field, l_value_hasher::LValueHasher, symbol::Symbol,
      },
      type_aliases::{l_value::LValue, refinement_map::RefinementMap},
    };
    use ulua_ast::records::ast_name::AstName;

    let t1 = CString::new("t").unwrap();
    let x1 = "x".to_string();
    let t_x1 = LValue::Field(Field {
      parent: Some(Arc::new(LValue::Symbol(Symbol::from_global(
        AstName::ast_name_c_char(t1.as_ptr()),
      )))),
      key: x1,
    });

    let t2 = CString::new("t").unwrap();
    let x2 = "x".to_string();
    let t_x2 = LValue::Field(Field {
      parent: Some(Arc::new(LValue::Symbol(Symbol::from_global(
        AstName::ast_name_c_char(t2.as_ptr()),
      )))),
      key: x2,
    });

    assert_eq!(t_x1, t_x1);
    assert_eq!(t_x1, t_x2);
    assert_eq!(t_x2, t_x2);

    let hasher = LValueHasher::default();
    assert_eq!(hasher.operator_call(&t_x1), hasher.operator_call(&t_x1));
    assert_eq!(hasher.operator_call(&t_x1), hasher.operator_call(&t_x2));
    assert_eq!(hasher.operator_call(&t_x2), hasher.operator_call(&t_x2));

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(t_x1, builtin_types.string_type());
    m.insert(t_x2, builtin_types.number_type());

    assert_eq!(1, m.len());
  }
}

mod l_value_hashing_lvalue_local_prop_access {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:173:l_value_hashing_lvalue_local_prop_access`
  //! Source: `tests/LValue.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/LValue.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/LValue.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Field (Analysis/include/Luau/LValue.h)
  //!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
  //!   - type_ref -> record LValueHasher (Analysis/include/Luau/LValue.h)
  //!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
  //!   - translates_to -> rust_item l_value_hashing_lvalue_local_prop_access

  #[cfg(test)]
  #[test]
  fn l_value_hashing_lvalue_local_prop_access() {
    use core::ptr::null_mut;
    use std::{ffi::CString, sync::Arc};

    use ulua_analysis::{
      records::{
        builtin_types::BuiltinTypes, field::Field, l_value_hasher::LValueHasher, symbol::Symbol,
      },
      type_aliases::{l_value::LValue, refinement_map::RefinementMap},
    };
    use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName, location::Location};

    let t1 = CString::new("t").unwrap();
    let x1 = "x".to_string();
    let mut localt1 = AstLocal::new(
      AstName::ast_name_c_char(t1.as_ptr()),
      Location::default(),
      null_mut(),
      0,
      0,
      null_mut(),
      false,
    );
    let t_x1 = LValue::Field(Field {
      parent: Some(Arc::new(LValue::Symbol(Symbol::from_local(&mut localt1)))),
      key: x1,
    });

    let t2 = CString::new("t").unwrap();
    let x2 = "x".to_string();
    let mut localt2 = AstLocal::new(
      AstName::ast_name_c_char(t2.as_ptr()),
      Location::default(),
      &mut localt1,
      0,
      0,
      null_mut(),
      false,
    );
    let t_x2 = LValue::Field(Field {
      parent: Some(Arc::new(LValue::Symbol(Symbol::from_local(&mut localt2)))),
      key: x2,
    });

    assert_eq!(t_x1, t_x1);
    assert_ne!(t_x1, t_x2);
    assert_eq!(t_x2, t_x2);

    let hasher = LValueHasher::default();
    assert_eq!(hasher.operator_call(&t_x1), hasher.operator_call(&t_x1));
    assert_ne!(hasher.operator_call(&t_x1), hasher.operator_call(&t_x2));
    assert_eq!(hasher.operator_call(&t_x2), hasher.operator_call(&t_x2));

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(t_x1, builtin_types.string_type());
    m.insert(t_x2, builtin_types.number_type());

    assert_eq!(2, m.len());
  }
}

mod l_value_luau_merge_hashmap_order {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:49:l_value_luau_merge_hashmap_order`
  //! Source: `tests/LValue.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/LValue.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/LValue.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
  //!   - calls -> function mkSymbol (tests/LValue.test.cpp)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - translates_to -> rust_item l_value_luau_merge_hashmap_order

  #[cfg(test)]
  #[test]
  fn l_value_luau_merge_hashmap_order() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
      type_aliases::{l_value::LValue, refinement_map::RefinementMap},
    };
    use ulua_unit_test::functions::{merge::merge, mk_symbol::mk_symbol};

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
    m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.number_type());

    let mut other = RefinementMap::new();
    other.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
    other.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
    other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

    let mut arena = TypeArena::default();
    merge(&mut arena, &mut m, &other);

    assert_eq!(3, m.len());
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));

    assert_eq!(
      "string",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
    );
    assert_eq!(
      "string",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
    );
    assert_eq!(
      "boolean | number",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
    );
  }
}

mod l_value_luau_merge_hashmap_order2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:79:l_value_luau_merge_hashmap_order2`
  //! Source: `tests/LValue.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/LValue.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/LValue.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
  //!   - calls -> function mkSymbol (tests/LValue.test.cpp)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - translates_to -> rust_item l_value_luau_merge_hashmap_order2

  #[cfg(test)]
  #[test]
  fn l_value_luau_merge_hashmap_order2() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
      type_aliases::{l_value::LValue, refinement_map::RefinementMap},
    };
    use ulua_unit_test::functions::{merge::merge, mk_symbol::mk_symbol};

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
    m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
    m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.number_type());

    let mut other = RefinementMap::new();
    other.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
    other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

    let mut arena = TypeArena::default();
    merge(&mut arena, &mut m, &other);

    assert_eq!(3, m.len());
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));

    assert_eq!(
      "string",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
    );
    assert_eq!(
      "string",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
    );
    assert_eq!(
      "boolean | number",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
    );
  }
}

mod l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:109:l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start`
  //! Source: `tests/LValue.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/LValue.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/LValue.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
  //!   - calls -> function mkSymbol (tests/LValue.test.cpp)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - translates_to -> rust_item l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start

  #[cfg(test)]
  #[test]
  fn l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
      type_aliases::{l_value::LValue, refinement_map::RefinementMap},
    };
    use ulua_unit_test::functions::{merge::merge, mk_symbol::mk_symbol};

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
    m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.number_type());
    m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

    let mut other = RefinementMap::new();
    other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.string_type());
    other.insert(LValue::Symbol(mk_symbol("d")), builtin_types.number_type());
    other.insert(LValue::Symbol(mk_symbol("e")), builtin_types.boolean_type());

    let mut arena = TypeArena::default();
    merge(&mut arena, &mut m, &other);

    assert_eq!(5, m.len());
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("d"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("e"))));

    assert_eq!(
      "string",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
    );
    assert_eq!(
      "number",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
    );
    assert_eq!(
      "boolean | string",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
    );
    assert_eq!(
      "number",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("d"))).unwrap())
    );
    assert_eq!(
      "boolean",
      to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("e"))).unwrap())
    );
  }
}
