extern crate alloc;

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_content_reassignment() {
  use ulua_analysis::{
    functions::{as_mutable_type_pack::as_mutable_type_pack, get_type_pack},
    records::{
      free_type_pack::FreeTypePack, type_arena::TypeArena, type_level::TypeLevel,
      type_pack_var::TypePackVar,
    },
    type_aliases::{error_type_pack::ErrorTypePack, type_pack_variant::TypePackVariant},
  };

  let my_error =
    TypePackVar::new_with_persistence(TypePackVariant::Error(ErrorTypePack::new()), true);

  let mut arena = TypeArena::default();

  let future_error = arena.add_type_pack_t(FreeTypePack::new(TypeLevel::default()));
  unsafe {
    (*as_mutable_type_pack(future_error)).reassign(&my_error);
  }

  assert!(get_type_pack::get::<ErrorTypePack>(future_error).is_some());
  assert!(!unsafe { (*future_error).is_persistent() });
  assert_eq!(unsafe { (*future_error).owning_arena() }, arena.arena_id);
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_first_chases_bound_type_pack_vars() {
  use ulua_analysis::{
    functions::first::first,
    records::{
      primitive_type::{PrimitiveType, Type as PrimitiveKind},
      r#type::Type,
      type_pack::TypePack,
      type_pack_var::TypePackVar,
    },
    type_aliases::type_pack_variant::TypePackVariant,
  };

  let nil_type = Type::from(PrimitiveType::primitive_type_type_item(
    PrimitiveKind::NilType,
  ));
  let nil_type_id = &nil_type as *const Type;

  let tp1 = TypePackVar::from(TypePack::new(alloc::vec![nil_type_id], None));
  let tp1_id = &tp1 as *const TypePackVar;

  let tp2 = TypePackVar::new(TypePackVariant::Bound(tp1_id));
  let tp2_id = &tp2 as *const TypePackVar;

  let tp3 = TypePackVar::from(TypePack::new(alloc::vec![], Some(tp2_id)));
  let tp3_id = &tp3 as *const TypePackVar;

  assert_eq!(first(tp3_id, true), Some(nil_type_id));
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_follows_bound_type_packs() {
  use ulua_analysis::{
    functions::as_mutable_type_pack::as_mutable_type_pack,
    type_aliases::type_pack_variant::TypePackVariant,
  };
  use ulua_unit_test::{
    functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
  };

  let mut fixture = TypePackFixture::new();
  let tail_tp = fixture.new_type_pack(alloc::vec![fixture.types[2], fixture.types[3]], None);
  let middle_tp = fixture.fresh_type_pack();
  unsafe {
    (*as_mutable_type_pack(middle_tp)).ty = TypePackVariant::Bound(tail_tp);
  }
  let head_tp = fixture.new_type_pack(alloc::vec![], Some(middle_tp));

  let count = collect_type_pack(head_tp).len();

  assert_eq!(2, count);
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_get_the_tail() {
  use ulua_analysis::functions::{begin_type_pack::begin, end_type_pack::end};
  use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

  let mut fixture = TypePackFixture::new();
  let free_tail = fixture.fresh_type_pack();
  let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0]], Some(free_tail));

  let mut it = begin(type_pack);
  let end_it = end(type_pack);
  let mut count = 0;
  while it != end_it {
    count += 1;
    it.advance();
  }

  assert_eq!(1, count);
  assert!(it == end(type_pack));
  assert_eq!(it.tail(), Some(free_tail));
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_iterate_over_type_pack() {
  use ulua_unit_test::{
    functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
  };

  let mut fixture = TypePackFixture::new();
  let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[1]], None);

  let res = collect_type_pack(type_pack);

  assert_eq!(2, res.len());
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_iterate_over_type_pack_with_2_links() {
  use ulua_unit_test::{
    functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
  };

  let mut fixture = TypePackFixture::new();
  let type_pack1 = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[1]], None);
  let type_pack2 = fixture.new_type_pack(
    alloc::vec![fixture.types[0], fixture.types[3]],
    Some(type_pack1),
  );

  let result = collect_type_pack(type_pack2);

  assert_eq!(4, result.len());
  assert_eq!(fixture.types[0], result[0]);
  assert_eq!(fixture.types[3], result[1]);
  assert_eq!(fixture.types[0], result[2]);
  assert_eq!(fixture.types[1], result[3]);
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_post_and_pre_increment() {
  use ulua_analysis::functions::begin_type_pack::begin;
  use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

  let mut fixture = TypePackFixture::new();
  let type_pack = fixture.new_type_pack(
    alloc::vec![
      fixture.types[0],
      fixture.types[1],
      fixture.types[2],
      fixture.types[3],
    ],
    None,
  );

  let mut it1 = begin(type_pack);
  let mut it2 = it1.advance_and_get_prev();
  it2.advance();
  let it3 = it2.clone();

  assert_eq!(*it2.current(), *it3.current());
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_skip_over_empty_head_typepack_with_tail() {
  use ulua_unit_test::{
    functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
  };

  let mut fixture = TypePackFixture::new();
  let tail_tp = fixture.new_type_pack(alloc::vec![fixture.types[2], fixture.types[3]], None);
  let head_tp = fixture.new_type_pack(alloc::vec![], Some(tail_tp));

  let count = collect_type_pack(head_tp).len();

  assert_eq!(2, count);
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_skip_over_empty_middle_link() {
  use ulua_unit_test::{
    functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
  };

  let mut fixture = TypePackFixture::new();
  let tail_tp = fixture.new_type_pack(alloc::vec![fixture.types[2], fixture.types[3]], None);
  let middle_tp = fixture.new_type_pack(alloc::vec![], Some(tail_tp));
  let head_tp = fixture.new_type_pack(
    alloc::vec![fixture.types[0], fixture.types[1]],
    Some(middle_tp),
  );

  let count = collect_type_pack(head_tp).len();

  assert_eq!(4, count);
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_std_distance() {
  use ulua_unit_test::{
    functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
  };

  let mut fixture = TypePackFixture::new();
  let type_pack = fixture.new_type_pack(
    alloc::vec![
      fixture.types[0],
      fixture.types[1],
      fixture.types[2],
      fixture.types[3],
    ],
    None,
  );

  let b_to_e = collect_type_pack(type_pack).len();

  assert_eq!(4, b_to_e);
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_tail_can_be_nullopt() {
  use ulua_analysis::functions::end_type_pack::end;
  use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

  let mut fixture = TypePackFixture::new();
  let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[0]], None);

  let it = end(type_pack);
  assert_eq!(None, it.tail());
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_tail_is_end_for_free_type_pack() {
  use ulua_analysis::functions::{begin_type_pack::begin, end_type_pack::end};
  use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

  let mut fixture = TypePackFixture::new();
  let type_pack = fixture.fresh_type_pack();

  let mut it = begin(type_pack);
  let end_it = end(type_pack);
  while it != end_it {
    it.advance();
  }

  assert_eq!(it.tail(), Some(type_pack));
}

// Source: `tests/TypePack.test.cpp`
#[test]
fn type_pack_type_pack_hello() {
  use ulua_analysis::records::{type_pack::TypePack, type_pack_var::TypePackVar};
  use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

  let fixture = TypePackFixture::new();
  let tp = TypePackVar::from(TypePack::new(
    alloc::vec![fixture.types[0], fixture.types[1]],
    None,
  ));

  assert_eq!(tp, tp);
}
