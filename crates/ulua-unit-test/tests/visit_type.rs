extern crate alloc;

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_can_be_configured_not_to_skip_bound_types() {
  use ulua_analysis::type_aliases::bound_type::BoundType;
  use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();
  let number_type = fixture.get_builtins().number_type;

  let a = fixture.arena.add_type(BoundType::bound_t(number_type));

  let mut vis = TracingVisitor::new(true, false);
  vis.run_type_id(a);

  assert_eq!(2, vis.trace.len());
  assert_eq!("number", vis.trace[0]);
  assert_eq!("number", vis.trace[1]);
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_detects_cycles() {
  use alloc::{collections::BTreeMap, string::String};

  use ulua_analysis::{
    functions::{as_mutable_type::as_mutable_type_id, to_string_to_string::to_string_type_id},
    records::{blocked_type::BlockedType, function_type::FunctionType, property_type::Property},
    type_aliases::type_variant::TypeVariant,
  };
  use ulua_unit_test::{
    functions::add_sealed_table_type::add_sealed_table_type,
    records::{fixture::Fixture, tracing_visitor::TracingVisitor},
  };

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();
  let (number_type, empty_type_pack) = {
    let builtins = fixture.get_builtins();
    (builtins.number_type, builtins.empty_type_pack)
  };

  let f_type = fixture.arena.add_type(BlockedType::default());

  let mut props = BTreeMap::new();
  props.insert(String::from("method"), Property::rw_type_id(f_type));
  let t_type = add_sealed_table_type(&mut fixture.arena, &props, None);

  let args = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[t_type, number_type]);
  unsafe {
    (*as_mutable_type_id(f_type)).ty = TypeVariant::Function(FunctionType::function_type_new(
      args,
      empty_type_pack,
      None,
      false,
    ));
  }

  let mut vis = TracingVisitor::new(true, true);
  vis.run_type_id(f_type);

  assert_eq!(3, vis.trace.len());
  assert_eq!("t1 where t1 = ({ method: t1 }, number) -> ()", vis.trace[0]);
  assert_eq!("t1 where t1 = { method: (t1, number) -> () }", vis.trace[1]);
  assert_eq!("number", vis.trace[2]);

  assert_eq!(1, vis.cycles.len());
  assert_eq!(
    "t1 where t1 = ({ method: t1 }, number) -> ()",
    to_string_type_id(vis.cycles[0])
  );
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_dont_throw_when_limit_is_high_enough() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::fint::LuauVisitRecursionLimit;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  let _sfi = ScopedFastInt::new(&LuauVisitRecursionLimit, 8);
  let mut fixture = Fixture::fixture_bool(false);

  let result = fixture.check_string_optional_frontend_options(
    r#"
        local t : {a: {b: {c: {d: {e: boolean}}}}}
    "#,
    None,
  );
  assert!(result.errors.is_empty());

  let t_type = fixture.require_type_string("t");
  let _ = to_string_type_id(t_type);
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_skip_over_tables() {
  use ulua_unit_test::records::{fixture::Fixture, table_skipping_visitor::TableSkippingVisitor};

  let mut fixture = Fixture::fixture_bool(false);
  let a = fixture.parse_type("(number, string, {x: number, y: number}) -> {x: number, y: number}");

  let mut vis = TableSkippingVisitor::new();
  vis.run_type_id(a);

  assert_eq!(3, vis.trace.len());
  assert_eq!(
    "(number, string, { x: number, y: number }) -> { x: number, y: number }",
    vis.trace[0]
  );
  assert_eq!("number", vis.trace[1]);
  assert_eq!("string", vis.trace[2]);
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_skips_bound_types() {
  use ulua_analysis::type_aliases::bound_type::BoundType;
  use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();
  let number_type = fixture.get_builtins().number_type;

  let a = fixture.arena.add_type(BoundType::bound_t(number_type));

  let mut vis = TracingVisitor::new(true, true);
  vis.run_type_id(a);

  assert_eq!(1, vis.trace.len());
  assert_eq!("number", vis.trace[0]);
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_some_free_types_do_not_have_bounds() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{free_type::FreeType, r#type::Type, type_level::TypeLevel},
  };
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();
  let builtins = fixture.get_builtins();

  let t = Type::from(FreeType {
    level: TypeLevel::default(),
    lower_bound: builtins.never_type,
    upper_bound: builtins.unknown_type,
    ..FreeType::default()
  });

  let _ = to_string_type_id(&t as *const Type);
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_some_free_types_have_bounds() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    functions::to_string_to_string::to_string_type_id,
    records::{free_type::FreeType, scope::Scope, r#type::Type},
  };
  use ulua_common::fflag::DebugLuauForceOldSolver;
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

  let _sff = ScopedFastFlag::new(&DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();
  let builtins = fixture.get_builtins();
  let mut scope = Scope::scope_type_pack_id(builtins.any_type_pack);

  let t = Type::from(FreeType::free_type_scope_type_id_type_id_polarity(
    &mut scope,
    builtins.never_type,
    builtins.number_type,
    Polarity::Unknown,
  ));

  assert_eq!("('a <: number)", to_string_type_id(&t as *const Type));
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_throw_when_limit_is_exceeded() {
  use std::panic::catch_unwind;

  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_common::{fflag::DebugLuauForceOldSolver, fint::LuauVisitRecursionLimit};
  use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

  let mut fixture = Fixture::fixture_bool(false);

  if !DebugLuauForceOldSolver.get() {
    let result = fixture.check_string_optional_frontend_options(
      r#"
            local t : {a: {b: {c: {d: {e: boolean}}}}}
        "#,
      None,
    );
    assert!(result.errors.is_empty());

    let _sfi = ScopedFastInt::new(&LuauVisitRecursionLimit, 3);
    let t_type = fixture.require_type_string("t");

    assert!(catch_unwind(|| to_string_type_id(t_type)).is_err());
  } else {
    let _sfi = ScopedFastInt::new(&LuauVisitRecursionLimit, 3);

    let result = fixture.check_string_optional_frontend_options(
      r#"
            local t : {a: {b: {c: {d: {e: boolean}}}}}
        "#,
      None,
    );
    assert!(result.errors.is_empty());

    let t_type = fixture.require_type_string("t");

    assert!(catch_unwind(|| to_string_type_id(t_type)).is_err());
  }
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_trace_a_simple_function() {
  use ulua_unit_test::records::{fixture::Fixture, tracing_visitor::TracingVisitor};

  let mut fixture = Fixture::fixture_bool(false);
  let a = fixture.parse_type("(number, string) -> boolean");

  let mut vis = TracingVisitor::new(true, true);
  vis.run_type_id(a);

  assert_eq!(4, vis.trace.len());
  assert_eq!("(number, string) -> boolean", vis.trace[0]);
  assert_eq!("number", vis.trace[1]);
  assert_eq!("string", vis.trace[2]);
  assert_eq!("boolean", vis.trace[3]);
}

// Source: `tests/VisitType.test.cpp`
#[test]
fn visit_type_visit_once() {
  use alloc::{collections::BTreeMap, string::String};

  use ulua_analysis::records::{function_type::FunctionType, property_type::Property};
  use ulua_unit_test::{
    functions::add_sealed_table_type::add_sealed_table_type,
    records::{fixture::Fixture, tracing_visitor::TracingVisitor},
  };

  let mut fixture = Fixture::fixture_bool(false);
  fixture.get_frontend();
  let number_type = fixture.get_builtins().number_type;

  let mut props = BTreeMap::new();
  props.insert(String::from("x"), Property::rw_type_id(number_type));
  let x_table = add_sealed_table_type(&mut fixture.arena, &props, None);

  let arg_pack = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[x_table, x_table]);
  let ret_pack = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[x_table]);
  let fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
    arg_pack, ret_pack, None, false,
  ));

  {
    let mut vis = TracingVisitor::new(true, true);
    vis.run_type_id(fn_ty);

    assert_eq!(3, vis.trace.len());
    assert_eq!(
      "({ x: number }, { x: number }) -> { x: number }",
      vis.trace[0]
    );
    assert_eq!("{ x: number }", vis.trace[1]);
    assert_eq!("number", vis.trace[2]);
    assert_eq!(0, vis.cycles.len());
  }

  {
    let mut vis = TracingVisitor::new(false, true);
    vis.run_type_id(fn_ty);

    assert_eq!(7, vis.trace.len());
    assert_eq!(
      "({ x: number }, { x: number }) -> { x: number }",
      vis.trace[0]
    );
    assert_eq!("{ x: number }", vis.trace[1]);
    assert_eq!("{ x: number }", vis.trace[2]);
    assert_eq!("{ x: number }", vis.trace[3]);
    assert_eq!("number", vis.trace[4]);
    assert_eq!("number", vis.trace[5]);
    assert_eq!("number", vis.trace[6]);
    assert_eq!(0, vis.cycles.len());
  }
}
