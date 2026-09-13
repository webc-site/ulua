extern crate alloc;

mod type_var_content_reassignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:433:type_var_content_reassignment`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AnyType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_var_content_reassignment

  #[cfg(test)]
  #[test]
  fn type_var_content_reassignment() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{as_mutable_type::as_mutable_type_id, get_type_alt_j::get_type_id},
      records::{
        any_type::AnyType, builtin_types::BuiltinTypes, r#type::Type, type_arena::TypeArena,
        type_level::TypeLevel,
      },
    };

    let mut my_any = Type::from(AnyType::default());
    my_any.persistent = true;
    my_any.documentation_symbol = Some(String::from("@global/any"));

    let mut arena = TypeArena::default();
    let builtin_types = BuiltinTypes::new();
    let future_any =
      arena.fresh_type_not_null_builtin_types_type_level(&builtin_types, TypeLevel::default());

    unsafe {
      (*as_mutable_type_id(future_any)).reassign(&my_any);

      assert!(get_type_id::<AnyType>(future_any).is_some());
      assert!(!(*future_any).persistent);
      assert_eq!(
        (*future_any).documentation_symbol.as_deref(),
        Some("@global/any")
      );
      assert_eq!((*future_any).owning_arena, &mut arena as *mut TypeArena);
    }
  }
}

mod type_var_is_boolean_on_boolean_singletons {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:408:type_var_is_boolean_on_boolean_singletons`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record BooleanSingleton (Analysis/include/Luau/Type.h)
  //!   - calls -> function isBoolean (Analysis/src/Type.cpp)
  //!   - translates_to -> rust_item type_var_is_boolean_on_boolean_singletons

  #[cfg(test)]
  #[test]
  fn type_var_is_boolean_on_boolean_singletons() {
    use ulua_analysis::{
      functions::is_boolean::is_boolean,
      records::{boolean_singleton::BooleanSingleton, singleton_type::SingletonType, r#type::Type},
      type_aliases::singleton_variant::SingletonVariant,
    };

    let true_bool = Type::from(SingletonType {
      variant: SingletonVariant::V0(BooleanSingleton { value: true }),
    });

    assert!(is_boolean(&true_bool));
  }
}

mod type_var_is_boolean_on_unions_of_true_or_false_singletons {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:414:type_var_is_boolean_on_unions_of_true_or_false_singletons`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record BooleanSingleton (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function isBoolean (Analysis/src/Type.cpp)
  //!   - translates_to -> rust_item type_var_is_boolean_on_unions_of_true_or_false_singletons

  #[cfg(test)]
  #[test]
  fn type_var_is_boolean_on_unions_of_true_or_false_singletons() {
    use alloc::vec;

    use ulua_analysis::{
      functions::is_boolean::is_boolean,
      records::{
        boolean_singleton::BooleanSingleton, singleton_type::SingletonType, r#type::Type,
        union_type::UnionType,
      },
      type_aliases::singleton_variant::SingletonVariant,
    };

    let true_bool = Type::from(SingletonType {
      variant: SingletonVariant::V0(BooleanSingleton { value: true }),
    });
    let false_bool = Type::from(SingletonType {
      variant: SingletonVariant::V0(BooleanSingleton { value: false }),
    });
    let union_ = Type::from(UnionType {
      options: vec![&true_bool, &false_bool],
    });

    assert!(is_boolean(&union_));
  }
}

mod type_var_is_string_on_string_singletons {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:383:type_var_is_string_on_string_singletons`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_var_is_string_on_string_singletons

  #[cfg(test)]
  #[test]
  fn type_var_is_string_on_string_singletons() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::is_string::is_string,
      records::{singleton_type::SingletonType, string_singleton::StringSingleton, r#type::Type},
      type_aliases::singleton_variant::SingletonVariant,
    };

    let hello_string = Type::from(SingletonType {
      variant: SingletonVariant::V1(StringSingleton {
        value: String::from("hello"),
      }),
    });

    assert!(is_string(&hello_string));
  }
}

mod type_var_is_string_on_unions_of_various_string_singletons {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:389:type_var_is_string_on_unions_of_various_string_singletons`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_var_is_string_on_unions_of_various_string_singletons

  #[cfg(test)]
  #[test]
  fn type_var_is_string_on_unions_of_various_string_singletons() {
    use alloc::{string::String, vec};

    use ulua_analysis::{
      functions::is_string::is_string,
      records::{
        singleton_type::SingletonType, string_singleton::StringSingleton, r#type::Type,
        union_type::UnionType,
      },
      type_aliases::singleton_variant::SingletonVariant,
    };

    let hello_string = Type::from(SingletonType {
      variant: SingletonVariant::V1(StringSingleton {
        value: String::from("hello"),
      }),
    });
    let bye_string = Type::from(SingletonType {
      variant: SingletonVariant::V1(StringSingleton {
        value: String::from("bye"),
      }),
    });
    let union_ = Type::from(UnionType {
      options: vec![&hello_string, &bye_string],
    });

    assert!(is_string(&union_));
  }
}

mod type_var_iterate_over_union_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:60:type_var_iterate_over_union_type`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_iterate_over_union_type

  #[cfg(test)]
  #[test]
  fn type_var_iterate_over_union_type() {
    use alloc::{vec, vec::Vec};

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::union_type::UnionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let utv = UnionType {
      options: vec![
        builtins.number_type,
        builtins.string_type,
        builtins.any_type,
      ],
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(&utv);
    let end = end_union_type(&utv);
    while it.operator_ne(&end) {
      result.push(it.operator_deref());
      it.operator_inc();
    }

    assert_eq!(utv.options, result);
  }
}

mod type_var_iterating_over_nested_union_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:72:type_var_iterating_over_nested_union_types`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_iterating_over_nested_union_types

  #[cfg(test)]
  #[test]
  fn type_var_iterating_over_nested_union_types() {
    use alloc::{vec, vec::Vec};

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let subunion = Type::from(UnionType {
      options: vec![builtins.number_type, builtins.string_type],
    });
    let utv = UnionType {
      options: vec![builtins.any_type, &subunion],
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(&utv);
    let end = end_union_type(&utv);
    while it.operator_ne(&end) {
      result.push(it.operator_deref());
      it.operator_inc();
    }

    assert_eq!(3, result.len());
    assert_eq!(builtins.any_type, result[0]);
    assert_eq!(builtins.string_type, result[2]);
    assert_eq!(builtins.number_type, result[1]);
  }
}

mod type_var_iterating_over_nested_union_types_postfix_operator_plus_plus {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:91:type_var_iterating_over_nested_union_types_postfix_operator_plus_plus`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_iterating_over_nested_union_types_postfix_operator_plus_plus

  #[cfg(test)]
  #[test]
  fn type_var_iterating_over_nested_union_types_postfix_operator_plus_plus() {
    use alloc::{vec, vec::Vec};

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let subunion = Type::from(UnionType {
      options: vec![builtins.number_type, builtins.string_type],
    });
    let utv = UnionType {
      options: vec![builtins.any_type, &subunion],
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(&utv);
    let end = end_union_type(&utv);
    while it.operator_ne(&end) {
      let mut old = it.operator_inc_i32();
      result.push(old.operator_deref());
    }

    assert_eq!(3, result.len());
    assert_eq!(builtins.any_type, result[0]);
    assert_eq!(builtins.string_type, result[2]);
    assert_eq!(builtins.number_type, result[1]);
  }
}

mod type_var_iterator_descends_on_nested_in_first_operator_deref {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:132:type_var_iterator_descends_on_nested_in_first_operator_deref`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_iterator_descends_on_nested_in_first_operator_deref

  #[cfg(test)]
  #[test]
  fn type_var_iterator_descends_on_nested_in_first_operator() {
    use alloc::{vec, vec::Vec};

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    let tv1 = Type::from(UnionType {
      options: vec![builtins.string_type, builtins.number_type],
    });
    let tv2 = Type::from(UnionType {
      options: vec![&tv1, builtins.boolean_type],
    });
    let TypeVariant::Union(utv) = &tv2.ty else {
      unreachable!();
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(utv);
    let end = end_union_type(utv);
    while it.operator_ne(&end) {
      result.push(it.operator_deref());
      it.operator_inc();
    }

    assert_eq!(3, result.len());
    assert_eq!(builtins.string_type, result[0]);
    assert_eq!(builtins.number_type, result[1]);
    assert_eq!(builtins.boolean_type, result[2]);
  }
}

mod type_var_iterator_detects_cyclic_union_types_and_skips_over_them {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:110:type_var_iterator_detects_cyclic_union_types_and_skips_over_them`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_iterator_detects_cyclic_union_types_and_skips_over_them

  #[cfg(test)]
  #[test]
  fn type_var_iterator_detects_cyclic_union_types_and_skips_over_them() {
    use alloc::vec::Vec;

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    let mut atv = Type::from(UnionType::default());
    let mut btv = Type::from(UnionType::default());
    let atv_id = &atv as *const Type;
    let btv_id = &btv as *const Type;

    if let TypeVariant::Union(utv2) = &mut btv.ty {
      utv2.options.push(builtins.number_type);
      utv2.options.push(builtins.string_type);
      utv2.options.push(atv_id);
    } else {
      unreachable!();
    }

    if let TypeVariant::Union(utv1) = &mut atv.ty {
      utv1.options.push(btv_id);
    } else {
      unreachable!();
    }

    let TypeVariant::Union(utv2) = &btv.ty else {
      unreachable!();
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(utv2);
    let end = end_union_type(utv2);
    while it.operator_ne(&end) {
      result.push(it.operator_deref());
      it.operator_inc();
    }

    assert_eq!(2, result.len());
    assert_eq!(builtins.number_type, result[0]);
    assert_eq!(builtins.string_type, result[1]);
  }
}

mod type_var_proof_that_is_boolean_uses_all_of {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:423:type_var_proof_that_is_boolean_uses_all_of`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record BooleanSingleton (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function isBoolean (Analysis/src/Type.cpp)
  //!   - translates_to -> rust_item type_var_proof_that_is_boolean_uses_all_of

  #[cfg(test)]
  #[test]
  fn type_var_proof_that_is_boolean_uses_all_of() {
    use alloc::vec;

    use ulua_analysis::{
      functions::is_boolean::is_boolean,
      records::{
        boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType,
        singleton_type::SingletonType, r#type::Type, union_type::UnionType,
      },
      type_aliases::singleton_variant::SingletonVariant,
    };

    let true_bool = Type::from(SingletonType {
      variant: SingletonVariant::V0(BooleanSingleton { value: true }),
    });
    let false_bool = Type::from(SingletonType {
      variant: SingletonVariant::V0(BooleanSingleton { value: false }),
    });
    let string_type = Type::from(PrimitiveType {
      r#type: PrimitiveType::STRING,
      metatable: None,
    });
    let union_ = Type::from(UnionType {
      options: vec![&true_bool, &false_bool, &string_type],
    });

    assert!(!is_boolean(&union_));
  }
}

mod type_var_proof_that_is_string_uses_all_of {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:398:type_var_proof_that_is_string_uses_all_of`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_var_proof_that_is_string_uses_all_of

  #[cfg(test)]
  #[test]
  fn type_var_proof_that_is_string_uses_all_of() {
    use alloc::{string::String, vec};

    use ulua_analysis::{
      functions::is_string::is_string,
      records::{
        primitive_type::PrimitiveType, singleton_type::SingletonType,
        string_singleton::StringSingleton, r#type::Type, union_type::UnionType,
      },
      type_aliases::singleton_variant::SingletonVariant,
    };

    let hello_string = Type::from(SingletonType {
      variant: SingletonVariant::V1(StringSingleton {
        value: String::from("hello"),
      }),
    });
    let bye_string = Type::from(SingletonType {
      variant: SingletonVariant::V1(StringSingleton {
        value: String::from("bye"),
      }),
    });
    let boolean_type = Type::from(PrimitiveType {
      r#type: PrimitiveType::BOOLEAN,
      metatable: None,
    });
    let union_ = Type::from(UnionType {
      options: vec![&hello_string, &bye_string, &boolean_type],
    });

    assert!(!is_string(&union_));
  }
}

mod type_var_return_type_of_function_is_not_parenthesized_if_just_one_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:17:type_var_return_type_of_function_is_not_parenthesized_if_just_one_value`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_var_return_type_of_function_is_not_parenthesized_if_just_one_value

  #[cfg(test)]
  #[test]
  fn type_var_return_type_of_function_is_not_parenthesized_if_just_one_value() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{
        function_type::FunctionType, r#type::Type, type_pack::TypePack, type_pack_var::TypePackVar,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;

    let empty_argument_pack = TypePackVar::from(TypePack::new(vec![], None));
    let return_pack = TypePackVar::from(TypePack::new(vec![number_type], None));
    let returns_one = Type::from(FunctionType::function_type_new(
      &empty_argument_pack,
      &return_pack,
      None,
      false,
    ));

    let res = to_string_type_item(&returns_one);
    assert_eq!("() -> number", res);
  }
}

mod type_var_return_type_of_function_is_parenthesized_if_not_just_one_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:27:type_var_return_type_of_function_is_parenthesized_if_not_just_one_value`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_var_return_type_of_function_is_parenthesized_if_not_just_one_value

  #[cfg(test)]
  #[test]
  fn type_var_return_type_of_function_is_parenthesized_if_not_just_one_value() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{
        function_type::FunctionType, r#type::Type, type_pack::TypePack, type_pack_var::TypePackVar,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;

    let empty_argument_pack = TypePackVar::from(TypePack::new(vec![], None));
    let return_pack = TypePackVar::from(TypePack::new(vec![number_type, number_type], None));
    let returns_two = Type::from(FunctionType::function_type_new(
      &empty_argument_pack,
      &return_pack,
      None,
      false,
    ));

    let res = to_string_type_item(&returns_two);
    assert_eq!("() -> (number, number)", res);
  }
}

mod type_var_return_type_of_function_is_parenthesized_if_tail_is_free {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:37:type_var_return_type_of_function_is_parenthesized_if_tail_is_free`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> type_alias TypePackVariant (Analysis/include/Luau/TypePack.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_var_return_type_of_function_is_parenthesized_if_tail_is_free

  #[cfg(test)]
  #[test]
  fn type_var_return_type_of_function_is_parenthesized_if_tail_is_free() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{
        free_type_pack::FreeTypePack, function_type::FunctionType, r#type::Type,
        type_level::TypeLevel, type_pack::TypePack, type_pack_var::TypePackVar,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;

    let empty_argument_pack = TypePackVar::from(TypePack::new(vec![], None));
    let free_pack = TypePackVar::from(FreeTypePack::new(TypeLevel::default()));
    let return_pack = TypePackVar::from(TypePack::new(vec![number_type], Some(&free_pack)));
    let returns_two = Type::from(FunctionType::function_type_new(
      &empty_argument_pack,
      &return_pack,
      None,
      false,
    ));

    let res = to_string_type_item(&returns_two);
    assert_eq!("() -> (number, a...)", res);
  }
}

mod type_var_subset_check {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:49:type_var_subset_check`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function isSubset (Analysis/src/Type.cpp)
  //!   - translates_to -> rust_item type_var_subset_check

  #[cfg(test)]
  #[test]
  fn type_var_subset_check() {
    use alloc::vec;

    use ulua_analysis::{functions::is_subset::is_subset, records::union_type::UnionType};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let super_ = UnionType {
      options: vec![
        builtins.number_type,
        builtins.string_type,
        builtins.boolean_type,
      ],
    };
    let sub = UnionType {
      options: vec![builtins.number_type, builtins.string_type],
    };
    let not_sub = UnionType {
      options: vec![builtins.number_type, builtins.nil_type],
    };

    assert!(is_subset(&super_, &sub));
    assert!(!is_subset(&super_, &not_sub));
  }
}

mod type_var_substitution_skip_failure {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:184:type_var_substitution_skip_failure`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record Anyification (Analysis/include/Luau/Anyification.h)
  //!   - translates_to -> rust_item type_var_substitution_skip_failure

  #[cfg(test)]
  #[test]
  fn type_var_substitution_skip_failure() {
    use alloc::{string::String, sync::Arc, vec};

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        anyification::Anyification, free_type::FreeType, function_type::FunctionType,
        generic_type::GenericType, module::Module, property_type::Property, table_type::TableType,
        r#type::Type, type_level::TypeLevel, type_pack::TypePack, type_pack_var::TypePackVar,
      },
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let never_type = builtins.never_type;
    let unknown_type = builtins.unknown_type;

    let ftv11 = Type::from(FreeType {
      level: TypeLevel::default(),
      lower_bound: never_type,
      upper_bound: unknown_type,
      ..FreeType::default()
    });
    let ftv11_id = &ftv11 as *const Type;

    let tp24 = TypePackVar::from(TypePack::new(vec![ftv11_id], None));
    let tp17 = TypePackVar::from(TypePack::new(vec![], None));

    let ftv23 = Type::from(FunctionType::function_type_new(&tp24, &tp17, None, false));
    let ftv23_id = &ftv23 as *const Type;

    let mut ttv_connection2 = Type::from(TableType::new());
    let _ttv_connection2_id = &ttv_connection2 as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_connection2.ty {
      ttv.instantiated_type_params.push(ftv11_id);
      ttv
        .props
        .insert(String::from("f"), Property::rw_type_id(ftv23_id));
    } else {
      unreachable!();
    }

    let tp21 = TypePackVar::from(TypePack::new(vec![ftv11_id], None));
    let tp20 = TypePackVar::from(TypePack::new(vec![], None));

    let ftv19 = Type::from(FunctionType::function_type_new(&tp21, &tp20, None, false));
    let ftv19_id = &ftv19 as *const Type;

    let mut ttv_signal = Type::from(TableType::new());
    let ttv_signal_id = &ttv_signal as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_signal.ty {
      ttv.instantiated_type_params.push(ftv11_id);
      ttv
        .props
        .insert(String::from("f"), Property::rw_type_id(ftv19_id));
    } else {
      unreachable!();
    }

    if let TypeVariant::Table(ttv) = &mut ttv_connection2.ty {
      ttv
        .props
        .insert(String::from("signal"), Property::rw_type_id(ttv_signal_id));
    } else {
      unreachable!();
    }

    let gtv_k2 = Type::from(GenericType::default());
    let gtv_k2_id = &gtv_k2 as *const Type;
    let gtv_v2 = Type::from(GenericType::default());
    let gtv_v2_id = &gtv_v2 as *const Type;

    let mut ttv_tween_result2 = Type::from(TableType::new());
    let ttv_tween_result2_id = &ttv_tween_result2 as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_tween_result2.ty {
      ttv.instantiated_type_params.push(gtv_k2_id);
      ttv.instantiated_type_params.push(gtv_v2_id);
    } else {
      unreachable!();
    }

    let tp13 = TypePackVar::from(TypePack::new(vec![ttv_tween_result2_id], None));
    let ftv12 = Type::from(FunctionType::function_type_new(&tp13, &tp17, None, false));
    let ftv12_id = &ftv12 as *const Type;

    let mut ttv_connection = Type::from(TableType::new());
    let ttv_connection_id = &ttv_connection as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_connection.ty {
      ttv.instantiated_type_params.push(ttv_tween_result2_id);
      ttv
        .props
        .insert(String::from("f"), Property::rw_type_id(ftv12_id));
      ttv
        .props
        .insert(String::from("signal"), Property::rw_type_id(ttv_signal_id));
    } else {
      unreachable!();
    }

    let tp9 = TypePackVar::from(TypePack::new(vec![], None));
    let tp10 = TypePackVar::from(TypePack::new(vec![ttv_connection_id], None));

    let ftv8 = Type::from(FunctionType::function_type_new(&tp9, &tp10, None, false));
    let ftv8_id = &ftv8 as *const Type;

    let mut ttv_tween = Type::from(TableType::new());
    let ttv_tween_id = &ttv_tween as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_tween.ty {
      ttv.instantiated_type_params.push(gtv_k2_id);
      ttv.instantiated_type_params.push(gtv_v2_id);
      ttv
        .props
        .insert(String::from("f"), Property::rw_type_id(ftv8_id));
    } else {
      unreachable!();
    }

    let tp4 = TypePackVar::from(TypePack::new(vec![], None));
    let tp5 = TypePackVar::from(TypePack::new(vec![ttv_tween_id], None));

    let ftv3 = Type::from(FunctionType::function_type_new(&tp4, &tp5, None, false));
    let ftv3_id = &ftv3 as *const Type;

    if let TypeVariant::Table(ttv) = &mut ttv_tween_result2.ty {
      ttv
        .props
        .insert(String::from("f"), Property::rw_type_id(ftv3_id));
    } else {
      unreachable!();
    }

    let gtv_k = Type::from(GenericType::default());
    let gtv_k_id = &gtv_k as *const Type;
    let gtv_v = Type::from(GenericType::default());
    let gtv_v_id = &gtv_v as *const Type;

    let mut ttv_tween_result = Type::from(TableType::new());
    let root = &ttv_tween_result as *const Type;
    if let TypeVariant::Table(ttv) = &mut ttv_tween_result.ty {
      ttv.instantiated_type_params.push(gtv_k_id);
      ttv.instantiated_type_params.push(gtv_v_id);
      ttv
        .props
        .insert(String::from("f"), Property::rw_type_id(ftv3_id));
    } else {
      unreachable!();
    }

    let mut current_module = Module::default();
    let (global_scope, builtin_types, ice_handler, any_type, any_type_pack) = {
      let frontend = fixture.get_frontend();
      let global_scope = frontend.globals.global_scope();
      let builtin_types = frontend.builtin_types;
      let ice_handler = &mut frontend.ice_handler as *mut _;
      let any_type = unsafe { (*builtin_types).any_type };
      let any_type_pack = unsafe { (*builtin_types).any_type_pack };
      (
        global_scope,
        builtin_types,
        ice_handler,
        any_type,
        any_type_pack,
      )
    };

    let mut anyification =
        Anyification::anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
            &mut current_module.internal_types,
            &Arc::clone(&global_scope),
            builtin_types,
            ice_handler,
            any_type,
            any_type_pack,
        );

    let any = anyification.substitute_type_id(root);

    assert!(!anyification.normalization_too_complex);
    assert!(any.is_some());
    assert_eq!(
      "{ f: t1 } where t1 = () -> { f: () -> { f: ({ f: t1 }) -> (), signal: { f: (any) -> () } } }",
      to_string_type_id(any.unwrap())
    );
  }
}

mod type_var_tagging_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:282:type_var_tagging_extern_types`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_var_tagging_extern_types

  #[cfg(test)]
  #[test]
  fn type_var_tagging_extern_types() {
    use ulua_analysis::{
      functions::{attach_tag_type::attach_tag, has_tag_type_alt_b::has_tag},
      records::{extern_type::ExternType, r#type::Type},
    };

    let base = Type::from(ExternType {
      name: "Base".into(),
      props: Default::default(),
      parent: None,
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: "Test".into(),
      definition_location: None,
      indexer: None,
      relation: None,
    });
    let base_id = &base as *const Type;

    assert!(!has_tag(base_id, "foo"));
    attach_tag(base_id, "foo");
    assert!(has_tag(base_id, "foo"));
  }
}

mod type_var_tagging_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:307:type_var_tagging_functions`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_var_tagging_functions

  #[cfg(test)]
  #[test]
  fn type_var_tagging_functions() {
    use alloc::vec;

    use ulua_analysis::{
      functions::{attach_tag_type::attach_tag, has_tag_type_alt_b::has_tag},
      records::{
        function_type::FunctionType, r#type::Type, type_pack::TypePack, type_pack_var::TypePackVar,
      },
    };

    let empty = TypePackVar::from(TypePack::new(vec![], None));
    let ftv = Type::from(FunctionType::function_type_new(&empty, &empty, None, false));
    let ty = &ftv as *const Type;

    assert!(!has_tag(ty, "foo"));
    attach_tag(ty, "foo");
    assert!(has_tag(ty, "foo"));
  }
}

mod type_var_tagging_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:316:type_var_tagging_props`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_var_tagging_props

  #[cfg(test)]
  #[test]
  fn type_var_tagging_props() {
    use ulua_analysis::{
      functions::{
        attach_tag_type_alt_b::attach_tag_property_string,
        has_tag_type_alt_c::has_tag_property_string,
      },
      records::property_type::Property,
    };

    let mut prop = Property::default();
    assert!(!has_tag_property_string(&prop, "foo"));
    attach_tag_property_string(&mut prop, "foo");
    assert!(has_tag_property_string(&prop, "foo"));
  }
}

mod type_var_tagging_subextern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:290:type_var_tagging_subextern_types`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_var_tagging_subextern_types

  #[cfg(test)]
  #[test]
  fn type_var_tagging_subextern_types() {
    use ulua_analysis::{
      functions::{attach_tag_type::attach_tag, has_tag_type_alt_b::has_tag},
      records::{extern_type::ExternType, r#type::Type},
    };

    let base = Type::from(ExternType {
      name: "Base".into(),
      props: Default::default(),
      parent: None,
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: "Test".into(),
      definition_location: None,
      indexer: None,
      relation: None,
    });
    let base_id = &base as *const Type;
    let derived = Type::from(ExternType {
      name: "Derived".into(),
      props: Default::default(),
      parent: Some(base_id),
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: "Test".into(),
      definition_location: None,
      indexer: None,
      relation: None,
    });
    let derived_id = &derived as *const Type;

    assert!(!has_tag(base_id, "foo"));
    assert!(!has_tag(derived_id, "foo"));

    attach_tag(base_id, "foo");
    assert!(has_tag(base_id, "foo"));
    assert!(has_tag(derived_id, "foo"));

    attach_tag(derived_id, "bar");
    assert!(!has_tag(base_id, "bar"));
    assert!(has_tag(derived_id, "bar"));
  }
}

mod type_var_tagging_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:274:type_var_tagging_tables`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_var_tagging_tables

  #[cfg(test)]
  #[test]
  fn type_var_tagging_tables() {
    use ulua_analysis::{
      functions::{attach_tag_type::attach_tag, has_tag_type_alt_b::has_tag},
      records::{table_type::TableType, r#type::Type},
    };

    let ttv = Type::from(TableType::new());
    let ty = &ttv as *const Type;

    assert!(!has_tag(ty, "foo"));
    attach_tag(ty, "foo");
    assert!(has_tag(ty, "foo"));
  }
}

mod type_var_union_type_iterator_with_empty_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:159:type_var_union_type_iterator_with_empty_union`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_union_type_iterator_with_empty_union

  #[cfg(test)]
  #[test]
  fn type_var_union_type_iterator_with_empty_union() {
    use alloc::vec::Vec;

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
      type_aliases::type_variant::TypeVariant,
    };

    let tv = Type::from(UnionType::default());
    let TypeVariant::Union(utv) = &tv.ty else {
      unreachable!();
    };

    let mut actual = Vec::new();
    let mut it = begin_union_type(utv);
    let end = end_union_type(utv);
    while it.operator_ne(&end) {
      actual.push(it.operator_deref());
      it.operator_inc();
    }

    assert!(actual.is_empty());
  }
}

mod type_var_union_type_iterator_with_only_cyclic_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:168:type_var_union_type_iterator_with_only_cyclic_union`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_union_type_iterator_with_only_cyclic_union

  #[cfg(test)]
  #[test]
  fn type_var_union_type_iterator_with_only_cyclic_union() {
    use alloc::vec::Vec;

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
      type_aliases::type_variant::TypeVariant,
    };

    let mut tv = Type::from(UnionType::default());
    let tv_id = &tv as *const Type;
    if let TypeVariant::Union(utv) = &mut tv.ty {
      utv.options.push(tv_id);
      utv.options.push(tv_id);
    } else {
      unreachable!();
    }

    let TypeVariant::Union(utv) = &tv.ty else {
      unreachable!();
    };
    let mut actual = Vec::new();
    let mut it = begin_union_type(utv);
    let end = end_union_type(utv);
    while it.operator_ne(&end) {
      actual.push(it.operator_deref());
      it.operator_inc();
    }

    assert!(actual.is_empty());
  }
}

mod type_var_union_type_iterator_with_vector_iter_ctor {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:148:type_var_union_type_iterator_with_vector_iter_ctor`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_var_union_type_iterator_with_vector_iter_ctor

  #[cfg(test)]
  #[test]
  fn type_var_union_type_iterator_with_vector_iter_ctor() {
    use alloc::{vec, vec::Vec};

    use ulua_analysis::{
      functions::{begin_type::begin_union_type, end_type::end_union_type},
      records::{r#type::Type, union_type::UnionType},
      type_aliases::type_variant::TypeVariant,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    let tv1 = Type::from(UnionType {
      options: vec![builtins.string_type, builtins.number_type],
    });
    let tv2 = Type::from(UnionType {
      options: vec![&tv1, builtins.boolean_type],
    });
    let TypeVariant::Union(utv) = &tv2.ty else {
      unreachable!();
    };

    let mut actual = Vec::new();
    let mut it = begin_union_type(utv);
    let end = end_union_type(utv);
    while it.operator_ne(&end) {
      actual.push(it.operator_deref());
      it.operator_inc();
    }
    let expected = vec![
      builtins.string_type,
      builtins.number_type,
      builtins.boolean_type,
    ];

    assert_eq!(actual, expected);
  }
}

mod type_var_visit_once {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:363:type_var_visit_once`
  //! Source: `tests/TypeVar.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeVar.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/VisitType.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeVar.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record VisitCountTracker (tests/TypeVar.test.cpp)
  //!   - translates_to -> rust_item type_var_visit_once

  #[cfg(test)]
  #[test]
  fn type_var_visit_once() {
    use alloc::string::String;

    use ulua_unit_test::records::{fixture::Fixture, visit_count_tracker::VisitCountTracker};

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      r#"
type T = { a: number, b: () -> () }
local b: (T, T, T) -> T
"#,
    );
    let result = fixture.check_string_optional_frontend_options(&source, None);
    assert!(
      result.errors.is_empty(),
      "expected no errors, got {:?}",
      result.errors
    );

    let b_type = fixture.require_type_string(&String::from("b"));

    let mut tester = VisitCountTracker::new();
    tester.traverse(b_type);

    for count in tester.ty_visits.values() {
      assert_eq!(1, *count);
    }

    for count in tester.tp_visits.values() {
      assert_eq!(1, *count);
    }
  }
}
