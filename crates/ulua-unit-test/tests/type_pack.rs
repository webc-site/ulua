extern crate alloc;

mod type_pack_content_reassignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:200:type_pack_content_reassignment`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> type_alias ErrorTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_pack_content_reassignment

  #[cfg(test)]
  #[test]
  fn type_pack_content_reassignment() {
    use ulua_analysis::{
      functions::{as_mutable_type_pack::as_mutable, get_type_pack::get_type_pack_id},
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
      (*as_mutable(future_error)).reassign(&my_error);
    }

    assert!(get_type_pack_id::<ErrorTypePack>(future_error).is_some());
    assert!(!unsafe { (*future_error).is_persistent() });
    assert_eq!(
      unsafe { (*future_error).owning_arena() },
      &mut arena as *mut TypeArena
    );
  }
}

mod type_pack_first_chases_bound_type_pack_vars {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:55:type_pack_first_chases_bound_type_pack_vars`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> type_alias BoundTypePack (Analysis/include/Luau/TypePack.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item type_pack_first_chases_bound_type_pack_vars

  #[cfg(test)]
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
}

mod type_pack_follows_bound_type_packs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:163:type_pack_follows_bound_type_packs`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> record Bound (Analysis/include/Luau/Unifiable.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_pack_follows_bound_type_packs

  #[cfg(test)]
  #[test]
  fn type_pack_follows_bound_type_packs() {
    use ulua_analysis::{
      functions::as_mutable_type_pack::as_mutable, type_aliases::type_pack_variant::TypePackVariant,
    };
    use ulua_unit_test::{
      functions::collect_type_pack::collect_type_pack, records::type_pack_fixture::TypePackFixture,
    };

    let mut fixture = TypePackFixture::new();
    let tail_tp = fixture.new_type_pack(alloc::vec![fixture.types[2], fixture.types[3]], None);
    let middle_tp = fixture.fresh_type_pack();
    unsafe {
      (*as_mutable(middle_tp)).operator_assign_type_pack_variant(TypePackVariant::Bound(tail_tp));
    }
    let head_tp = fixture.new_type_pack(alloc::vec![], Some(middle_tp));

    let count = collect_type_pack(head_tp).len();

    assert_eq!(2, count);
  }
}

mod type_pack_get_the_tail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:96:type_pack_get_the_tail`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - translates_to -> rust_item type_pack_get_the_tail

  #[cfg(test)]
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
    while it.operator_ne(&end_it) {
      count += 1;
      it.operator_inc();
    }

    assert_eq!(1, count);
    assert!(it.operator_eq(&end(type_pack)));
    assert_eq!(it.tail(), Some(free_tail));
  }
}

mod type_pack_iterate_over_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:68:type_pack_iterate_over_type_pack`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_pack_iterate_over_type_pack

  #[cfg(test)]
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
}

mod type_pack_iterate_over_type_pack_with_2_links {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:79:type_pack_iterate_over_type_pack_with_2_links`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_pack_iterate_over_type_pack_with_2_links

  #[cfg(test)]
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
}

mod type_pack_post_and_pre_increment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:180:type_pack_post_and_pre_increment`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - translates_to -> rust_item type_pack_post_and_pre_increment

  #[cfg(test)]
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
    let mut it2 = it1.operator_inc_i32();
    it2.operator_inc();
    let it3 = it2.clone();

    assert_eq!(*it2.operator_deref(), *it3.operator_deref());
  }
}

mod type_pack_skip_over_empty_head_typepack_with_tail {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:132:type_pack_skip_over_empty_head_typepack_with_tail`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_pack_skip_over_empty_head_typepack_with_tail

  #[cfg(test)]
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
}

mod type_pack_skip_over_empty_middle_link {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:147:type_pack_skip_over_empty_middle_link`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item type_pack_skip_over_empty_middle_link

  #[cfg(test)]
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
}

mod type_pack_std_distance {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:191:type_pack_std_distance`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - translates_to -> rust_item type_pack_std_distance

  #[cfg(test)]
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
}

mod type_pack_tail_can_be_nullopt {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:113:type_pack_tail_can_be_nullopt`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
  //!   - translates_to -> rust_item type_pack_tail_can_be_nullopt

  #[cfg(test)]
  #[test]
  fn type_pack_tail_can_be_nullopt() {
    use ulua_analysis::functions::end_type_pack::end;
    use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

    let mut fixture = TypePackFixture::new();
    let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[0]], None);

    let it = end(type_pack);
    assert_eq!(None, it.tail());
  }
}

mod type_pack_tail_is_end_for_free_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:121:type_pack_tail_is_end_for_free_type_pack`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
  //!   - translates_to -> rust_item type_pack_tail_is_end_for_free_type_pack

  #[cfg(test)]
  #[test]
  fn type_pack_tail_is_end_for_free_type_pack() {
    use ulua_analysis::functions::{begin_type_pack::begin, end_type_pack::end};
    use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

    let mut fixture = TypePackFixture::new();
    let type_pack = fixture.fresh_type_pack();

    let mut it = begin(type_pack);
    let end_it = end(type_pack);
    while it.operator_ne(&end_it) {
      it.operator_inc();
    }

    assert_eq!(it.tail(), Some(type_pack));
  }
}

mod type_pack_type_pack_hello {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:48:type_pack_type_pack_hello`
  //! Source: `tests/TypePack.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypePack.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypePack.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - translates_to -> rust_item type_pack_type_pack_hello

  #[cfg(test)]
  #[test]
  fn type_pack_type_pack_hello() {
    use ulua_analysis::records::{type_pack::TypePack, type_pack_var::TypePackVar};
    use ulua_unit_test::records::type_pack_fixture::TypePackFixture;

    let fixture = TypePackFixture::new();
    let tp = TypePackVar::from(TypePack::new(
      alloc::vec![fixture.types[0], fixture.types[1]],
      None,
    ));

    assert!(tp.type_pack_var_operator_eq(&tp));
  }
}
