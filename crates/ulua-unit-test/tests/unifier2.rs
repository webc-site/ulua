extern crate alloc;

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_number_t() {
  use ulua_analysis::enums::unify_result::UnifyResult;
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();
  let (right, free_right) = fixture.fresh_type();

  assert_eq!(
    UnifyResult::Ok,
    fixture.u2.unify(fixture.builtin_types.number_type, right)
  );

  assert_eq!(
    "number",
    fixture.to_string_type_id(unsafe { (*free_right).lower_bound })
  );
  assert_eq!(
    "unknown",
    fixture.to_string_type_id(unsafe { (*free_right).upper_bound })
  );
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_string_x_y() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    functions::{flatten_type_pack::flatten_type_pack_id, follow_type_pack},
    records::function_type::FunctionType,
  };
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();
  let string_to_unit = {
    let args = fixture
      .arena
      .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.string_type]);
    let rets = fixture.arena.add_type_pack_initializer_list_type_id(&[]);
    fixture
      .arena
      .add_type(FunctionType::function_type_new(args, rets, None, false))
  };

  let (x, x_free) = fixture.fresh_type();
  let y = fixture
    .arena
    .fresh_type_pack(&mut *fixture.scope, Polarity::Unknown);

  let x_to_y = {
    let args = fixture.arena.add_type_pack_initializer_list_type_id(&[x]);
    fixture
      .arena
      .add_type(FunctionType::function_type_new(args, y, None, false))
  };

  fixture.u2.unify(string_to_unit, x_to_y);

  assert_eq!(
    "string",
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`fixture` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    fixture.to_string_type_id(unsafe { (*x_free).upper_bound })
  );

  let followed_y = follow_type_pack::follow(y);
  let (head, tail) = flatten_type_pack_id(followed_y);

  assert_eq!(0, head.len());
  assert!(tail.is_none());
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_t_number() {
  use ulua_analysis::enums::unify_result::UnifyResult;
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();
  let (left, free_left) = fixture.fresh_type();

  assert_eq!(
    UnifyResult::Ok,
    fixture.u2.unify(left, fixture.builtin_types.number_type)
  );

  assert_eq!(
    "never",
    fixture.to_string_type_id(unsafe { (*free_left).lower_bound })
  );
  assert_eq!(
    "number",
    fixture.to_string_type_id(unsafe { (*free_left).upper_bound })
  );
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_t_u() {
  use ulua_analysis::enums::unify_result::UnifyResult;
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();
  let (left, free_left) = fixture.fresh_type();
  let (right, free_right) = fixture.fresh_type();

  assert_eq!(UnifyResult::Ok, fixture.u2.unify(left, right));

  assert_eq!(
    "t1 where t1 = ('a <: (t1 <: 'b))",
    fixture.to_string_type_id(left)
  );
  assert_eq!(
    "t1 where t1 = (('a <: t1) <: 'b)",
    fixture.to_string_type_id(right)
  );

  assert_eq!(
    "never",
    fixture.to_string_type_id(unsafe { (*free_left).lower_bound })
  );
  assert_eq!(
    "t1 where t1 = (('a <: t1) <: 'b)",
    fixture.to_string_type_id(unsafe { (*free_left).upper_bound })
  );

  assert_eq!(
    "t1 where t1 = ('a <: (t1 <: 'b))",
    fixture.to_string_type_id(unsafe { (*free_right).lower_bound })
  );
  assert_eq!(
    "unknown",
    fixture.to_string_type_id(unsafe { (*free_right).upper_bound })
  );
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_unify_binds_free_subtype_tail_pack() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    records::{free_type::FreeType, type_pack::TypePack},
  };
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();
  let number_pack = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.number_type]);

  let free_tail = fixture
    .arena
    .fresh_type_pack(&mut *fixture.scope, Polarity::Unknown);
  let free_head = fixture
    .arena
    .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
      &mut *fixture.scope,
      fixture.builtin_types.never_type,
      fixture.builtin_types.unknown_type,
      Polarity::Unknown,
    ));
  let free_and_free = fixture
    .arena
    .add_type_pack_t(TypePack::new(alloc::vec![free_head], Some(free_tail)));

  fixture.u2.unify_pack(free_and_free, number_pack);

  assert_eq!(
    "('a <: number)",
    fixture.to_string_type_pack_id(free_and_free)
  );
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_unify_binds_free_supertype_tail_pack() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    records::{free_type::FreeType, type_pack::TypePack},
  };
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();
  let number_pack = fixture
    .arena
    .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.number_type]);

  let free_tail = fixture
    .arena
    .fresh_type_pack(&mut *fixture.scope, Polarity::Unknown);
  let free_head = fixture
    .arena
    .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
      &mut *fixture.scope,
      fixture.builtin_types.never_type,
      fixture.builtin_types.unknown_type,
      Polarity::Unknown,
    ));
  let free_and_free = fixture
    .arena
    .add_type_pack_t(TypePack::new(alloc::vec![free_head], Some(free_tail)));

  fixture.u2.unify_pack(number_pack, free_and_free);

  assert_eq!(
    "(number <: 'a)",
    fixture.to_string_type_pack_id(free_and_free)
  );
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_unify_free_type_intersection_in_ub_from_union() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    records::{free_type::FreeType, intersection_type::IntersectionType, union_type::UnionType},
  };
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();

  let free_ty = fixture
    .arena
    .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
      &mut *fixture.scope,
      fixture.builtin_types.never_type,
      fixture.builtin_types.unknown_type,
      Polarity::Unknown,
    ));
  let sub_ty = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![free_ty, fixture.builtin_types.truthy_type],
  });
  let super_ty = fixture.arena.add_type(UnionType {
    options: alloc::vec![
      fixture.builtin_types.number_type,
      fixture.builtin_types.nil_type
    ],
  });

  fixture.u2.unify(sub_ty, super_ty);

  assert_eq!("('a <: never)", fixture.to_string_type_id(free_ty));
}

// Source: `tests/Unifier2.test.cpp`
#[test]
fn unifier2_unify_free_type_lb_from_intersection() {
  use ulua_analysis::{
    enums::polarity::Polarity,
    records::{
      free_type::FreeType, intersection_type::IntersectionType, negation_type::NegationType,
      singleton_type::SingletonType, string_singleton::StringSingleton, union_type::UnionType,
    },
    type_aliases::singleton_variant::SingletonVariant,
  };
  use ulua_unit_test::records::unifier_2_fixture::Unifier2Fixture;

  let mut fixture = Unifier2Fixture::new();

  let free_ty = fixture
    .arena
    .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
      &mut *fixture.scope,
      fixture.builtin_types.never_type,
      fixture.builtin_types.unknown_type,
      Polarity::Unknown,
    ));
  let super_ty = fixture.arena.add_type(UnionType {
    options: alloc::vec![free_ty, fixture.builtin_types.nil_type],
  });
  let foo_singleton = fixture
    .arena
    .add_type(SingletonType::new(SingletonVariant::V1(
      StringSingleton::new("foo".into()),
    )));
  let not_foo = fixture.arena.add_type(NegationType::new(foo_singleton));
  let sub_ty = fixture.arena.add_type(IntersectionType {
    parts: alloc::vec![fixture.builtin_types.string_type, not_foo],
  });

  fixture.u2.unify(sub_ty, super_ty);

  assert_eq!(
    "(string & ~\"foo\" <: 'a)",
    fixture.to_string_type_id(free_ty)
  );
}
