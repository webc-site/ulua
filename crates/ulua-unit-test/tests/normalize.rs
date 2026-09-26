extern crate alloc;

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_any_intersect_t_is_t() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a : (any & string)
        local b : string
        local c : number
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");

  assert!(fixture.is_subtype(a, b));
  assert!(fixture.is_subtype(b, a));
  assert!(!fixture.is_subtype(a, c));
  assert!(!fixture.is_subtype(c, a));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_any_is_unknown_union_error() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local err = 5.nope.nope -- err is now an error type
        local a : any
        local b : (unknown | typeof(err))
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");

  assert!(fixture.is_subtype(a, b));
  assert!(fixture.is_subtype(b, a));
  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.base.require_type_string("err"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_bare_negated_boolean() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let expected = if fflag::LuauIntegerType2.get() {
    "(buffer | function | integer | number | string | table | thread | userdata)?"
  } else {
    "(buffer | function | number | string | table | thread | userdata)?"
  };

  assert_eq!(
    expected,
    to_string_type_id(fixture.normal(
      r#"
            Not<boolean>
        "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_crazy_metatable() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "never",
    to_string_type_id(fixture.normal("Mt<{}, number> & Mt<{}, string>"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_intersection_of_unions() {
  use ulua_analysis::{
    functions::{as_mutable_type::as_mutable_type_id, to_string_to_string::to_string_type_id},
    records::{
      blocked_type::BlockedType, intersection_type::IntersectionType, union_type::UnionType,
    },
    type_aliases::type_variant::TypeVariant,
  };
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let string_type = fixture.base.get_builtins().string_type;

  let bound_ty = fixture.arena.add_type(BlockedType::default());
  let union_ty = fixture.arena.add_type(UnionType {
    options: alloc::vec![string_type, bound_ty],
  });
  let intersection_ty = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![string_type, union_ty],
  });
  unsafe {
    (*as_mutable_type_id(bound_ty)).ty = TypeVariant::Bound(intersection_ty);
  }

  let nt = fixture
    .normalize(intersection_ty)
    .expect("expected normalized type");

  assert_eq!(
    "string",
    to_string_type_id(fixture.type_from_normal(nt.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_stack_overflow_1() {
  use ulua_analysis::{
    functions::get_mutable_type,
    records::{
      intersection_type::IntersectionType, property_type::Property, table_type::TableType,
    },
    type_aliases::props_type::Props,
  };
  use ulua_common::fint;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_int::ScopedFastInt,
  };

  let _sfi = ScopedFastInt::new(&fint::LuauTypeInferRecursionLimit, 165);

  let mut fixture = NormalizeFixture::default();
  fixture
    .unifier_state
    .set_recursion_limit(fint::LuauTypeInferRecursionLimit.get());

  let t1 = fixture.arena.add_type(TableType::new());
  let t2 = fixture.arena.add_type(TableType::new());
  let t3 = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![t1, t2],
  });

  let t1_table = get_mutable_type::get_mutable::<TableType>(t1).expect("expected t1 table type");
  let mut t1_props = Props::new();
  t1_props.insert(String::from("foo"), Property::readonly(t2));
  t1_table.props = t1_props;

  let t2_table = get_mutable_type::get_mutable::<TableType>(t2).expect("expected t2 table type");
  let mut t2_props = Props::new();
  t2_props.insert(String::from("foo"), Property::readonly(t1));
  t2_table.props = t2_props;

  let normalized = fixture.normalize(t3);
  assert!(normalized.is_some(), "expected normalized type");
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_stack_overflow_2() {
  use ulua_analysis::{
    functions::get_mutable_type,
    records::{
      intersection_type::IntersectionType, property_type::Property, table_type::TableType,
    },
    type_aliases::props_type::Props,
  };
  use ulua_common::fint;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_int::ScopedFastInt,
  };

  let _sfi = ScopedFastInt::new(&fint::LuauTypeInferRecursionLimit, 165);

  let mut fixture = NormalizeFixture::default();
  fixture
    .unifier_state
    .set_recursion_limit(fint::LuauTypeInferRecursionLimit.get());

  let t1 = fixture.arena.add_type(TableType::new());
  let t2 = fixture.arena.add_type(TableType::new());
  let t3 = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![t1, t2],
  });

  let t1_table = get_mutable_type::get_mutable::<TableType>(t1).expect("expected t1 table type");
  let mut t1_props = Props::new();
  t1_props.insert(String::from("foo"), Property::readonly(t3));
  t1_table.props = t1_props;

  let t2_table = get_mutable_type::get_mutable::<TableType>(t2).expect("expected t2 table type");
  let mut t2_props = Props::new();
  t2_props.insert(String::from("foo"), Property::readonly(t1));
  t2_table.props = t2_props;

  let normalized = fixture.normalize(t3);
  assert!(normalized.is_some(), "expected normalized type");
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_table() {
  // Upstream wraps this test in `#if 0`, so the compiled C++ suite executes no body.
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_table_normalizes_sensibly() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local Cyclic = {}
        function Cyclic.get()
            return Cyclic
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ty = fixture.require_type_string("Cyclic");
  let mut opts = ToStringOptions::new(true);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "t1 where t1 = { get: () -> t1 }",
      to_string_type_id_to_string_options(ty, &mut opts)
    );
  } else {
    assert_eq!(
      "t1 where t1 = {| get: () -> t1 |}",
      to_string_type_id_to_string_options(ty, &mut opts)
    );
  }
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_union() {
  use ulua_analysis::{
    functions::{as_mutable_type::as_mutable_type_id, to_string_to_string::to_string_type_id},
    records::{
      blocked_type::BlockedType, intersection_type::IntersectionType, union_type::UnionType,
    },
    type_aliases::type_variant::TypeVariant,
  };
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let (any_type, number_type) = {
    let builtins = fixture.base.get_builtins();
    (builtins.any_type, builtins.number_type)
  };

  let t = fixture.arena.add_type(BlockedType::default());
  let u = fixture.arena.add_type(UnionType {
    options: alloc::vec![number_type, t],
  });
  unsafe {
    (*as_mutable_type_id(t)).ty = TypeVariant::Intersection(IntersectionType {
      parts: alloc::vec![any_type, u],
    });
  }

  let nt = fixture.normalize(t).expect("expected normalized type");

  assert_eq!(
    "number",
    to_string_type_id(fixture.type_from_normal(nt.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_cyclic_union_of_intersection() {
  use ulua_analysis::{
    functions::{as_mutable_type::as_mutable_type_id, to_string_to_string::to_string_type_id},
    records::{
      blocked_type::BlockedType, intersection_type::IntersectionType, union_type::UnionType,
    },
    type_aliases::type_variant::TypeVariant,
  };
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let string_type = fixture.base.get_builtins().string_type;

  let bound_ty = fixture.arena.add_type(BlockedType::default());
  let intersect_ty = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![string_type, bound_ty],
  });
  let union_ty = fixture.arena.add_type(UnionType {
    options: alloc::vec![string_type, intersect_ty],
  });
  unsafe {
    (*as_mutable_type_id(bound_ty)).ty = TypeVariant::Bound(union_ty);
  }

  let nt = fixture
    .normalize(union_ty)
    .expect("expected normalized type");

  assert_eq!(
    "string",
    to_string_type_id(fixture.type_from_normal(nt.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_disjoint_negations_normalize_to_string() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "string",
    to_string_type_id(fixture.normal(
      r#"
        (string & Not<"hello"> & Not<"world">) | (string & Not<"goodbye">)
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_double_negation() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number",
    to_string_type_id(fixture.normal(
      r#"
        number & Not<Not<any>>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_error_suppression() {
  use ulua_common::fflag;
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture
    .base
    .check_string_optional_frontend_options("", None);

  let (any, err, str_ty, unk) = {
    let builtins = fixture.base.get_builtins();
    (
      builtins.any_type,
      builtins.error_type,
      builtins.string_type,
      builtins.unknown_type,
    )
  };

  assert!(!fixture.is_subtype(any, err));
  assert!(fixture.is_subtype(err, any));

  assert!(!fixture.is_subtype(any, str_ty));
  assert!(fixture.is_subtype(str_ty, any));

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(fixture.is_subtype(any, unk));
  } else {
    assert!(!fixture.is_subtype(any, unk));
  }

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(fixture.is_subtype(err, str_ty));
  } else {
    assert!(!fixture.is_subtype(err, str_ty));
  }

  assert!(!fixture.is_subtype(str_ty, err));

  assert!(!fixture.is_subtype(err, unk));
  assert!(!fixture.is_subtype(unk, err));

  assert!(fixture.is_subtype(str_ty, unk));
  assert!(!fixture.is_subtype(unk, str_ty));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_extern_types() {
  use ulua_unit_test::{
    functions::{
      create_some_extern_types::create_some_extern_types, exported_type_binding::exported_type,
    },
    records::is_subtype_fixture::IsSubtypeFixture,
  };

  let mut fixture = IsSubtypeFixture::default();

  {
    let frontend = fixture.base.get_frontend();
    create_some_extern_types(frontend);
  }

  fixture
    .base
    .check_string_optional_frontend_options("", None);

  let (p, c, u) = {
    let frontend = fixture.base.get_frontend();
    // (a) 类裸指针解引用绕道已消除：统一走 `exported_type` 门面的 Arc 共享引用读。
    let globals = &frontend.globals;
    (
      exported_type(globals, "Parent"),
      exported_type(globals, "Child"),
      exported_type(globals, "Unrelated"),
    )
  };

  assert!(fixture.is_subtype(c, p));
  assert!(!fixture.is_subtype(p, c));
  assert!(!fixture.is_subtype(u, p));
  assert!(!fixture.is_subtype(p, u));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_extern_types_and_never() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::normalize_fixture::NormalizeFixture,
  };

  let mut fixture = NormalizeFixture::default();
  create_some_extern_types(fixture.get_frontend());

  assert_eq!("never", to_string_type_id(fixture.normal("Parent & never")));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_extern_types_and_unknown() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::normalize_fixture::NormalizeFixture,
  };

  let mut fixture = NormalizeFixture::default();
  create_some_extern_types(fixture.get_frontend());

  assert_eq!(
    "Parent",
    to_string_type_id(fixture.normal("Parent & unknown"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_final_types_are_cached() {
  use alloc::sync::Arc;

  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let number_type = fixture.base.get_builtins().number_type;

  let na1 = fixture
    .normalize(number_type)
    .expect("expected normalized number");
  let na2 = fixture
    .normalize(number_type)
    .expect("expected normalized number");

  assert!(Arc::ptr_eq(&na1, &na2));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_free_type_and_not_truthy() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{intersection_type::IntersectionType, negation_type::NegationType},
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NormalizeFixture::default();

  let scope = fixture.get_global_scope();
  let builtins = fixture.base.builtin_types;
  let (free_ty, truthy_type) = unsafe {
    (
      fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&*builtins, scope),
      (*builtins).truthy_type,
    )
  };
  let not_truthy = fixture.arena.add_type(NegationType::new(truthy_type));
  let intersection_ty = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![free_ty, not_truthy],
  });

  let norm = fixture
    .normalize(intersection_ty)
    .expect("expected normalized type");
  let result = fixture.type_from_normal(norm.as_ref());

  assert_eq!("'a & (false?)", to_string_type_id(result));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_free_type_intersection_ordering() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::intersection_type::IntersectionType,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NormalizeFixture::default();

  let scope = fixture.get_global_scope();
  let builtins = fixture.base.builtin_types;
  let (free_ty, string_type) = unsafe {
    (
      fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&*builtins, scope),
      (*builtins).string_type,
    )
  };

  let order_a = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![free_ty, string_type],
  });
  let norm_a = fixture
    .normalize(order_a)
    .expect("expected normalized type");
  assert_eq!(
    "'a & string",
    to_string_type_id(fixture.type_from_normal(norm_a.as_ref()))
  );

  let order_b = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![string_type, free_ty],
  });
  let norm_b = fixture
    .normalize(order_b)
    .expect("expected normalized type");
  assert_eq!(
    "'a & string",
    to_string_type_id(fixture.type_from_normal(norm_b.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_functions() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        function a(x: number): number return x end
        function b(x: number): number return x end

        function c(x: number?): number return x end
        function d(x: number): number? return x end
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");
  let d = fixture.base.require_type_string("d");

  assert!(fixture.is_subtype(b, a));
  assert!(fixture.is_subtype(c, a));
  assert!(!fixture.is_subtype(d, a));
  assert!(fixture.is_subtype(a, d));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_fuzz_flatten_type_pack_cycle() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
function _(_).readu32<t0...>()
repeat
until function<t4>()
end
return if _ then _,_(_)
end
_(_(_(_)),``)
do end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "expected errors");
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_fuzz_union_type_pack_cycle() {
  // Upstream wraps this cyclic type-pack fuzzer in `#if 0`, so the compiled C++ suite executes no body.
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_higher_order_function_normalization() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, records::primitive_type::Type,
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  fixture.check_string_optional_frontend_options(
    r#"
        function apply(f, x)
            return f(x)
        end

        local a = apply(function(x: number) return x + x end, 5)
    "#,
    None,
  );

  let a_type = fixture.require_type_string("a");
  assert_eq!(
    Some(Type::Number),
    fixture.get_primitive_type(a_type),
    "Expected a number but got {}",
    to_string_type_id(a_type)
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_higher_order_function_with_annotation() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::fixture::Fixture;

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = Fixture::fixture_bool(false);
  fixture.check_string_optional_frontend_options(
    r#"
        function apply<a, b>(f: (a) -> b, x)
            return f(x)
        end
    "#,
    None,
  );

  assert_eq!(
    "<a, b>((a) -> b, a) -> b",
    to_string_type_id(fixture.require_type_string("apply"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_error() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let norm = fixture
    .to_normalized_type(r#"(string & AAA)"#, 1)
    .expect("expected normalized error type");

  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.type_from_normal(norm.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_function_and_top_function() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "() -> ()",
    to_string_type_id(fixture.normal(
      r#"
        fun & (() -> ())
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_function_and_top_function_reverse() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "() -> ()",
    to_string_type_id(fixture.normal(
      r#"
        (() -> ()) & fun
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_not_error() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let norm = fixture
    .to_normalized_type(r#"(string & Not<)"#, 1)
    .expect("expected normalized error type");

  assert_eq!(
    "*error-type*",
    to_string_type_id(fixture.type_from_normal(norm.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_truthy() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number | string | true",
    to_string_type_id(fixture.normal(
      r#"
        (string | number | boolean | nil) & Not<false | nil>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_truthy_expressed_as_intersection() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number | string | true",
    to_string_type_id(fixture.normal(
      r#"
        (string | number | boolean | nil) & Not<false> & Not<nil>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersect_with_not_unknown() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{intersection_type::IntersectionType, negation_type::NegationType},
  };
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let (unknown_type, number_type) = {
    let builtins = fixture.base.get_builtins();
    (builtins.unknown_type, builtins.number_type)
  };

  let not_unknown = fixture.arena.add_type(NegationType::new(unknown_type));
  let ty = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![number_type, not_unknown],
  });
  let normalized = fixture.normalize(ty).expect("expected normalized type");

  assert_eq!(
    "never",
    to_string_type_id(fixture.type_from_normal(normalized.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersection() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: number & string
        local b: number
        local c: string
        local d: number & nil
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");
  let d = fixture.base.require_type_string("d");

  assert!(!fixture.is_subtype(b, a));
  assert!(fixture.is_subtype(a, b));

  assert!(!fixture.is_subtype(c, a));
  assert!(fixture.is_subtype(a, c));

  assert!(fixture.is_subtype(d, a));
  assert!(fixture.is_subtype(a, d));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersection_combine_on_bound_self() {
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(

          r#"
export type t0 = (((any)&({_:l0.t0,n0:t0,_G:any,}))&({_:any,}))&(((any)&({_:l0.t0,n0:t0,_G:any,}))&({_:any,}))
    "#
,
      None,
  );

  assert!(!result.errors.is_empty(), "expected errors");
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersection_of_metatables_where_the_metatable_is_top_or_bottom() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "setmetatable<{  }, *error-type*>",
    to_string_type_id(fixture.normal("Mt<{}, any> & Mt<{}, err>"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_intersections_of_extern_types() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::normalize_fixture::NormalizeFixture,
  };

  let mut fixture = NormalizeFixture::default();
  create_some_extern_types(fixture.get_frontend());

  assert_eq!("Child", to_string_type_id(fixture.normal("Parent & Child")));
  assert_eq!(
    "never",
    to_string_type_id(fixture.normal("Child & Unrelated"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_metatable() {
  // Upstream wraps this expected-failure test in `#if 0`, so the compiled C++ suite executes no body.
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_mismatched_indexers() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::is_subtype_fixture::IsSubtypeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: {x: number}
        local b: {[string]: number}
        local c: {}
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");

  assert!(fixture.is_subtype(b, a));
  assert!(!fixture.is_subtype(a, b));

  assert!(!fixture.is_subtype(c, b));
  assert!(fixture.is_subtype(b, c));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_narrow_union_of_extern_types_with_intersection() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::normalize_fixture::NormalizeFixture,
  };

  let mut fixture = NormalizeFixture::default();
  create_some_extern_types(fixture.get_frontend());

  assert_eq!(
    "Child",
    to_string_type_id(fixture.normal("(Child | Unrelated) & Child"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negate_any() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number",
    to_string_type_id(fixture.normal(
      r#"
        number & Not<any>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negate_boolean() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "true",
    to_string_type_id(fixture.normal(
      r#"
        boolean & Not<false>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negate_boolean_2() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "never",
    to_string_type_id(fixture.normal(
      r#"
        true & Not<true>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negate_string() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number",
    to_string_type_id(fixture.normal(
      r#"
        (number | string) & Not<string>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negate_string_from_cofinite_string_intersection() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number",
    to_string_type_id(fixture.normal(
      r#"
        (number | (string & Not<"hello"> & Not<"world">)) & Not<string>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negated_function_is_anything_except_a_function() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let expected = if fflag::LuauIntegerType2.get() {
    "(boolean | buffer | integer | number | string | table | thread | userdata)?"
  } else {
    "(boolean | buffer | number | string | table | thread | userdata)?"
  };

  assert_eq!(
    expected,
    to_string_type_id(fixture.normal(
      r#"
        Not<fun>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negations_of_extern_types() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::normalize_fixture::NormalizeFixture,
  };

  let mut fixture = NormalizeFixture::default();
  create_some_extern_types(fixture.get_frontend());

  assert_eq!(
    "(Parent & ~Child) | Unrelated",
    to_string_type_id(fixture.normal("(Parent & Not<Child>) | Unrelated"))
  );

  if fflag::LuauIntegerType2.get() {
    assert_eq!(
      "((userdata & ~Child) | boolean | buffer | function | integer | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<Child>"))
    );
    assert_eq!(
      "never",
      to_string_type_id(fixture.normal("Not<Parent> & Child"))
    );
    assert_eq!(
      "((userdata & ~Parent) | Child | boolean | buffer | function | integer | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<Parent> | Child"))
    );
    assert_eq!(
      "(boolean | buffer | function | integer | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<cls>"))
    );
    assert_eq!(
      "(Parent | Unrelated | boolean | buffer | function | integer | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<cls & Not<Parent> & Not<Child> & Not<Unrelated>>"))
    );
  } else {
    assert_eq!(
      "((userdata & ~Child) | boolean | buffer | function | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<Child>"))
    );
    assert_eq!(
      "never",
      to_string_type_id(fixture.normal("Not<Parent> & Child"))
    );
    assert_eq!(
      "((userdata & ~Parent) | Child | boolean | buffer | function | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<Parent> | Child"))
    );
    assert_eq!(
      "(boolean | buffer | function | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<cls>"))
    );
    assert_eq!(
      "(Parent | Unrelated | boolean | buffer | function | number | string | table | thread)?",
      to_string_type_id(fixture.normal("Not<cls & Not<Parent> & Not<Child> & Not<Unrelated>>"))
    );
  }

  assert_eq!(
    "Child",
    to_string_type_id(fixture.normal("(Child | Unrelated) & Not<Unrelated>"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_negations_of_tables() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fflag;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let expected_errors = if !fflag::DebugLuauForceOldSolver.get() {
    1
  } else {
    0
  };
  assert!(
    fixture
      .to_normalized_type("Not<{}>", expected_errors)
      .is_none()
  );

  let expected = if fflag::LuauIntegerType2.get() {
    "(boolean | buffer | function | integer | number | string | thread | userdata)?"
  } else {
    "(boolean | buffer | function | number | string | thread | userdata)?"
  };

  assert_eq!(expected, to_string_type_id(fixture.normal("Not<tbl>")));
  assert_eq!("table", to_string_type_id(fixture.normal("Not<Not<tbl>>")));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_no_op_negation_is_dropped() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "number",
    to_string_type_id(fixture.normal(
      r#"
        number & Not<string>
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_non_final_types_can_be_normalized_but_are_not_cached() {
  use alloc::sync::Arc;

  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let scope = fixture.get_global_scope();
  let builtins = fixture.base.builtin_types;
  let a = unsafe {
    fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(&*builtins, scope)
  };

  let na1 = fixture.normalize(a).expect("expected normalized free type");
  let na2 = fixture.normalize(a).expect("expected normalized free type");

  assert!(!Arc::ptr_eq(&na1, &na2));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_normalize_blocked_types() {
  use ulua_analysis::records::blocked_type::BlockedType;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let blocked = fixture.arena.add_type(BlockedType::default());

  let norm = fixture
    .normalize(blocked)
    .expect("expected normalized type");

  assert_eq!(blocked, fixture.type_from_normal(norm.as_ref()));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_normalize_is_exactly_number() {
  use ulua_analysis::{
    functions::is_prim::is_number,
    records::{intersection_type::IntersectionType, union_type::UnionType},
  };
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let (number_type, any_type) = {
    let builtins = fixture.base.get_builtins();
    (builtins.number_type, builtins.any_type)
  };

  let number = fixture
    .normalize(number_type)
    .expect("expected normalized number");
  assert_eq!(is_number(number_type), number.is_exactly_number());

  let intersection = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![number_type, number_type],
  });
  let norm_intersection = fixture
    .normalize(intersection)
    .expect("expected normalized intersection");
  assert!(norm_intersection.is_exactly_number());

  let yoonion = fixture.arena.add_type(UnionType {
    options: alloc::vec![any_type, number_type],
  });
  let union_intersection = fixture
    .normalize(yoonion)
    .expect("expected normalized union");
  assert!(!union_intersection.is_exactly_number());
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_normalize_unknown() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let nt = fixture
    .to_normalized_type("Not<string> | Not<number>", 0)
    .expect("expected normalized type");

  assert!(nt.is_unknown());
  assert_eq!(
    "unknown",
    to_string_type_id(fixture.type_from_normal(nt.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_normalizer_should_be_able_to_detect_cyclic_tables_and_not_stack_overflow() {
  use ulua_common::{fflag, fint};
  use ulua_unit_test::{
    records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_int::ScopedFastInt,
  };

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let _sfi = ScopedFastInt::new(&fint::LuauTypeInferRecursionLimit, 0);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let _result = fixture.base.check_string_optional_frontend_options(

          r#"
--!strict

type Array<T> = { [number] : T}
type Object = { [number] : any}

type Set<T> = typeof(setmetatable(
	{} :: {
		size: number,
		-- method definitions
		add: (self: Set<T>, T) -> Set<T>,
		clear: (self: Set<T>) -> (),
		delete: (self: Set<T>, T) -> boolean,
		has: (self: Set<T>, T) -> boolean,
		ipairs: (self: Set<T>) -> any,
	},
	{} :: {
		__index: Set<T>,
		__iter: (self: Set<T>) -> (<K, V>({ [K]: V }, K?) -> (K, V), T),
	}
))

type Map<K, V> = typeof(setmetatable(
	{} :: {
		size: number,
		-- method definitions
		set: (self: Map<K, V>, K, V) -> Map<K, V>,
		get: (self: Map<K, V>, K) -> V | nil,
		clear: (self: Map<K, V>) -> (),
		delete: (self: Map<K, V>, K) -> boolean,
		[K]: V,
		has: (self: Map<K, V>, K) -> boolean,
		keys: (self: Map<K, V>) -> Array<K>,
		values: (self: Map<K, V>) -> Array<V>,
		entries: (self: Map<K, V>) -> Array<Tuple<K, V>>,
		ipairs: (self: Map<K, V>) -> any,
		_map: { [K]: V },
		_array: { [number]: K },
		__index: (self: Map<K, V>, key: K) -> V,
		__iter: (self: Map<K, V>) -> (<K, V>({ [K]: V }, K?) -> (K?, V), V),
		__newindex: (self: Map<K, V>, key: K, value: V) -> (),
	},
	{} :: {
		__index: Map<K, V>,
		__iter: (self: Map<K, V>) -> (<K, V>({ [K]: V }, K?) -> (K, V), V),
		__newindex: (self: Map<K, V>, key: K, value: V) -> (),
	}
))
type mapFn<T, U> = (element: T, index: number) -> U
type mapFnWithThisArg<T, U> = (thisArg: any, element: T, index: number) -> U

function fromSet<T, U>(
	value: Set<T>,
	mapFn: (mapFn<T, U> | mapFnWithThisArg<T, U>)?,
	thisArg: Object?
	-- FIXME Luau: need overloading so the return type on this is more sane and doesn't require manual casts
): Array<U> | Array<T> | Array<string>

    local array : { [number] : string} = {"foo"}
	return array
end

function instanceof(tbl: any, class: any): boolean
    return true
end

function fromArray<T, U>(
	value: Array<T>,
	mapFn: (mapFn<T, U> | mapFnWithThisArg<T, U>)?,
	thisArg: Object?
	-- FIXME Luau: need overloading so the return type on this is more sane and doesn't require manual casts
): Array<U> | Array<T> | Array<string>
	local array : {[number] : string} = {}
	return array
end

return function<T, U>(
	value: string | Array<T> | Set<T> | Map<any, any>,
	mapFn: (mapFn<T, U> | mapFnWithThisArg<T, U>)?,
	thisArg: Object?
	-- FIXME Luau: need overloading so the return type on this is more sane and doesn't require manual casts
): Array<U> | Array<T> | Array<string>
	if value == nil then
		error("cannot create array from a nil value")
	end
	local array: Array<U> | Array<T> | Array<string>

    if instanceof(value, Set) then
		array = fromSet(value :: Set<T>, mapFn, thisArg)
	else
		array = {}
	end


	return array
end
"#
,
      None,
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_primitives() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a = 41
        local b = 32

        local c = "hello"
        local d = "world"
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");
  let d = fixture.base.require_type_string("d");

  assert!(fixture.is_subtype(b, a));
  assert!(fixture.is_subtype(d, c));
  assert!(!fixture.is_subtype(d, a));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_read_only_props() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options_mut,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NormalizeFixture::default();

  assert_eq!(
    "{ x: string }",
    to_string_type_id_to_string_options_mut(
      fixture.normal("{ read x: string } & { x: string }"),
      ToStringOptions::new(true)
    )
  );
  assert_eq!(
    "{ x: string }",
    to_string_type_id_to_string_options_mut(
      fixture.normal("{ x: string } & { read x: string }"),
      ToStringOptions::new(true)
    )
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_read_only_props_2() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options_mut,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NormalizeFixture::default();

  assert_eq!(
    r#"{ x: "hello" }"#,
    to_string_type_id_to_string_options_mut(
      fixture.normal(r#"{ x: "hello" } & { x: string }"#),
      ToStringOptions::new(true)
    )
  );
  assert_eq!(
    "never",
    to_string_type_id_to_string_options_mut(
      fixture.normal(r#"{ x: "hello" } & { x: "world" }"#),
      ToStringOptions::new(true)
    )
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_read_only_props_3() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options_mut,
    records::to_string_options::ToStringOptions,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NormalizeFixture::default();

  assert_eq!(
    r#"{ read x: "hello" }"#,
    to_string_type_id_to_string_options_mut(
      fixture.normal(r#"{ read x: "hello" } & { read x: string }"#),
      ToStringOptions::new(true)
    )
  );
  assert_eq!(
    "never",
    to_string_type_id_to_string_options_mut(
      fixture.normal(r#"{ read x: "hello" } & { read x: "world" }"#),
      ToStringOptions::new(true)
    )
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_recurring_intersection() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        type A = any?
        type B = A & A
    "#,
    None,
  );

  let t = fixture.base.lookup_type("B").expect("expected type B");
  let nt = fixture.normalize(t).expect("expected normalized type");

  assert_eq!(
    "any",
    to_string_type_id(fixture.type_from_normal(nt.as_ref()))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_skip_force_normal_on_external_types() {
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::builtins_fixture::BuiltinsFixture,
  };

  let mut fixture = BuiltinsFixture::default();
  create_some_extern_types(fixture.get_frontend());

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
export type t0 = { a: Child }
export type t1 = { a: typeof(string.byte) }
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_specific_functions_cannot_be_negated() {
  use ulua_common::fflag;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let expected_errors = if !fflag::DebugLuauForceOldSolver.get() {
    1
  } else {
    0
  };

  assert!(
    fixture
      .to_normalized_type("Not<(boolean) -> boolean>", expected_errors)
      .is_none()
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_string_intersection_is_commutative() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();

  let c4 = to_string_type_id(fixture.normal(
    r#"
        string & (string & Not<"a"> & Not<"b">)
"#,
  ));
  let c4_reverse = to_string_type_id(fixture.normal(
    r#"
        (string & Not<"a"> & Not<"b">) & string
"#,
  ));
  assert_eq!(c4, c4_reverse);
  assert_eq!(r#"string & ~"a" & ~"b""#, c4);

  let c5 = to_string_type_id(fixture.normal(
    r#"
        (string & Not<"a"> & Not<"b">) & (string & Not<"b"> & Not<"c">)
"#,
  ));
  let c5_reverse = to_string_type_id(fixture.normal(
    r#"
        (string & Not<"b"> & Not<"c">) & (string & Not<"a"> & Not<"c">)
"#,
  ));
  assert_eq!(c5, c5_reverse);
  assert_eq!(r#"string & ~"a" & ~"b" & ~"c""#, c5);

  let c6 = to_string_type_id(fixture.normal(
    r#"
        ("a" | "b") & (string & Not<"b"> & Not<"c">)
"#,
  ));
  let c6_reverse = to_string_type_id(fixture.normal(
    r#"
        (string & Not<"b"> & Not<"c">) & ("a" | "b")
"#,
  ));
  assert_eq!(c6, c6_reverse);
  assert_eq!(r#""a""#, c6);

  let c7 = to_string_type_id(fixture.normal(
    r#"
        string & ("b" | "c")
"#,
  ));
  let c7_reverse = to_string_type_id(fixture.normal(
    r#"
        ("b" | "c") & string
"#,
  ));
  assert_eq!(c7, c7_reverse);
  assert_eq!(r#""b" | "c""#, c7);

  let c8 = to_string_type_id(fixture.normal(
    r#"
(string & Not<"a"> & Not<"b">) & ("b" | "c")
"#,
  ));
  let c8_reverse = to_string_type_id(fixture.normal(
    r#"
        ("b" | "c") & (string & Not<"a"> & Not<"b">)
"#,
  ));
  assert_eq!(c8, c8_reverse);
  assert_eq!(r#""c""#, c8);

  let c9 = to_string_type_id(fixture.normal(
    r#"
            ("a" | "b") & ("b" | "c")
    "#,
  ));
  let c9_reverse = to_string_type_id(fixture.normal(
    r#"
            ("b" | "c") & ("a" | "b")
    "#,
  ));
  assert_eq!(c9, c9_reverse);
  assert_eq!(r#""b""#, c9);

  let l = to_string_type_id(fixture.normal(
    r#"
         (string | number) & ("a" | true)
    "#,
  ));
  let r = to_string_type_id(fixture.normal(
    r#"
         ("a" | true) & (string | number)
    "#,
  ));
  assert_eq!(l, r);
  assert_eq!(r#""a""#, l);
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_table_indexers_are_invariant() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::is_subtype_fixture::IsSubtypeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: {[string]: number}
        local b: {[string]: any}
        local c: {[string]: number}
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");

  assert!(!fixture.is_subtype(b, a));
  assert!(!fixture.is_subtype(a, b));

  assert!(fixture.is_subtype(c, a));
  assert!(fixture.is_subtype(a, c));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_table_with_any_prop() {
  use ulua_common::fflag;
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: {x: number}
        local b: {x: any}
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(!fixture.is_subtype(a, b));
  } else {
    assert!(fixture.is_subtype(a, b));
  }
  assert!(!fixture.is_subtype(b, a));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_table_with_union_prop() {
  use ulua_common::fflag;
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: {x: number}
        local b: {x: number?}
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(!fixture.is_subtype(a, b));
  } else {
    assert!(fixture.is_subtype(a, b));
  }
  assert!(!fixture.is_subtype(b, a));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_tables() {
  use ulua_common::fflag;
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: {x: number}
        local b: {x: any}
        local c: {y: number}
        local d: {x: number, y: number}
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");
  let d = fixture.base.require_type_string("d");

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(!fixture.is_subtype(a, b));
  } else {
    assert!(fixture.is_subtype(a, b));
  }
  assert!(!fixture.is_subtype(b, a));

  assert!(!fixture.is_subtype(c, a));
  assert!(!fixture.is_subtype(a, c));

  assert!(fixture.is_subtype(d, a));
  assert!(!fixture.is_subtype(a, d));

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(!fixture.is_subtype(d, b));
  } else {
    assert!(fixture.is_subtype(d, b));
  }
  assert!(!fixture.is_subtype(b, d));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_top_table_type() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();

  assert_eq!("table", to_string_type_id(fixture.normal("{} | tbl")));
  assert_eq!("{  }", to_string_type_id(fixture.normal("{} & tbl")));
  assert_eq!("never", to_string_type_id(fixture.normal("number & tbl")));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_trivial_intersection_inhabited() {
  use ulua_analysis::records::{function_type::FunctionType, intersection_type::IntersectionType};
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();

  let (empty_type_pack, any_type_pack) = {
    let builtins = fixture.base.get_builtins();
    (builtins.empty_type_pack, builtins.any_type_pack)
  };

  let a = fixture.arena.add_type(FunctionType::function_type_new(
    empty_type_pack,
    any_type_pack,
    None,
    false,
  ));
  let c = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![a, a],
  });

  let n = fixture.normalize(c).expect("expected normalized type");
  assert!(fixture.is_inhabited(Some(n.as_ref())));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_truthy_table_property_and_optional_table_with_optional_prop() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{
      intersection_type::IntersectionType, property_type::Property, union_type::UnionType,
    },
    type_aliases::props_type::Props,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    functions::add_sealed_table_type::add_sealed_table_type,
    records::normalize_fixture::NormalizeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = NormalizeFixture::default();

  let (truthy_type, optional_number_type, nil_type) = {
    let builtins = fixture.base.get_builtins();
    (
      builtins.truthy_type,
      builtins.optional_number_type,
      builtins.nil_type,
    )
  };

  let mut t1_props = Props::new();
  t1_props.insert(String::from("x"), Property::rw_type_id(truthy_type));
  let t1 = add_sealed_table_type(&mut fixture.arena, &t1_props, None);

  let mut table_props = Props::new();
  table_props.insert(
    String::from("x"),
    Property::rw_type_id(optional_number_type),
  );
  let optional_table = add_sealed_table_type(&mut fixture.arena, &table_props, None);
  let t2 = fixture.arena.add_type(UnionType {
    options: alloc::vec![optional_table, nil_type],
  });

  let intersection = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![t2, t1],
  });

  let norm = fixture
    .normalize(intersection)
    .expect("expected normalized type");
  let ty = fixture.type_from_normal(norm.as_ref());

  assert_eq!("{ x: number }", to_string_type_id(ty));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_tyvar_limit_one_sided_intersection() {
  use alloc::vec::Vec;

  use ulua_analysis::records::{intersection_type::IntersectionType, union_type::UnionType};
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  let scope = fixture.get_global_scope();
  let builtins = fixture.base.builtin_types;

  let mut options = Vec::new();
  for _ in 0..120 {
    options.push(unsafe {
      fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&*builtins, scope)
    });
  }

  let unknown_type = unsafe { (*builtins).unknown_type };
  let union = fixture.arena.add_type(UnionType { options });
  let target = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![unknown_type, union],
  });

  let norm = fixture.normalize(target);
  assert!(
    norm.is_none(),
    "expected normalization to stop at the tyvar limit"
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_union() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: number | string
        local b: number
        local c: string
        local d: number?
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");
  let c = fixture.base.require_type_string("c");
  let d = fixture.base.require_type_string("d");

  assert!(fixture.is_subtype(b, a));
  assert!(!fixture.is_subtype(a, b));

  assert!(fixture.is_subtype(c, a));
  assert!(!fixture.is_subtype(a, c));

  assert!(!fixture.is_subtype(d, a));
  assert!(!fixture.is_subtype(a, d));

  assert!(fixture.is_subtype(b, d));
  assert!(!fixture.is_subtype(d, b));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_union_and_intersection() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
            local a: number & string
            local b: number | nil
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");

  assert!(!fixture.is_subtype(b, a));
  assert!(fixture.is_subtype(a, b));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_union_function_and_top_function() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "function",
    to_string_type_id(fixture.normal(
      r#"
        fun | (() -> ())
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_union_of_negation() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    "string",
    to_string_type_id(fixture.normal(
      r#"
        (string & Not<"hello">) | "hello"
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_union_of_negations() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    r#"string & ~"world""#,
    to_string_type_id(fixture.normal(
      r#"
        (string & Not<"hello"> & Not<"world">) | (string & Not<"goodbye"> & Not<"world">)
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_union_of_union() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::normalize_fixture::NormalizeFixture;

  let mut fixture = NormalizeFixture::default();
  assert_eq!(
    r#""alpha" | "beta" | "gamma""#,
    to_string_type_id(fixture.normal(
      r#"
        ("alpha" | "beta") | "gamma"
    "#
    ))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_unions_of_extern_types() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::{
    functions::create_some_extern_types::create_some_extern_types,
    records::normalize_fixture::NormalizeFixture,
  };

  let mut fixture = NormalizeFixture::default();
  create_some_extern_types(fixture.get_frontend());

  assert_eq!(
    "Parent | Unrelated",
    to_string_type_id(fixture.normal("Parent | Unrelated"))
  );
  assert_eq!(
    "Parent",
    to_string_type_id(fixture.normal("Parent | Child"))
  );
  assert_eq!(
    "Parent | Unrelated",
    to_string_type_id(fixture.normal("Parent | Child | Unrelated"))
  );
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_variadic_function_with_head() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::is_subtype_fixture::IsSubtypeFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: (...number) -> ()
        local b: (number, number) -> ()
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");

  assert!(!fixture.is_subtype(b, a));
  assert!(fixture.is_subtype(a, b));
}

// Source: `tests/Normalize.test.cpp`
#[test]
fn normalize_variadic_functions_with_no_head() {
  use ulua_unit_test::records::is_subtype_fixture::IsSubtypeFixture;

  let mut fixture = IsSubtypeFixture::default();

  fixture.base.check_string_optional_frontend_options(
    r#"
        local a: (...number) -> ()
        local b: (...number?) -> ()
    "#,
    None,
  );

  let a = fixture.base.require_type_string("a");
  let b = fixture.base.require_type_string("b");

  assert!(fixture.is_subtype(b, a));
  assert!(!fixture.is_subtype(a, b));
}
