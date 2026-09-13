extern crate alloc;

mod unifier_2_number_t {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:63:unifier_2_number_t`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - calls -> method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
  //!   - type_ref -> enum UnifyResult (Analysis/include/Luau/Unifier2.h)
  //!   - translates_to -> rust_item unifier_2_number_t

  #[cfg(test)]
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
}

mod unifier_2_string_x_y {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:90:unifier_2_string_x_y`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item unifier_2_string_x_y

  #[cfg(test)]
  #[test]
  fn unifier2_string_x_y() {
    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::{flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id},
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
      fixture.to_string_type_id(unsafe { (*x_free).upper_bound })
    );

    let followed_y = unsafe { follow_type_pack_id(y) };
    let (head, tail) = flatten_type_pack_id(followed_y);

    assert_eq!(0, head.len());
    assert!(tail.is_none());
  }
}

mod unifier_2_t_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:53:unifier_2_t_number`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - calls -> method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
  //!   - type_ref -> enum UnifyResult (Analysis/include/Luau/Unifier2.h)
  //!   - translates_to -> rust_item unifier_2_t_number

  #[cfg(test)]
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
}

mod unifier_2_t_u {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:73:unifier_2_t_u`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - calls -> method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
  //!   - type_ref -> enum UnifyResult (Analysis/include/Luau/Unifier2.h)
  //!   - translates_to -> rust_item unifier_2_t_u

  #[cfg(test)]
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
}

mod unifier_2_unify_binds_free_subtype_tail_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:110:unifier_2_unify_binds_free_subtype_tail_pack`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item unifier_2_unify_binds_free_subtype_tail_pack

  #[cfg(test)]
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
}

mod unifier_2_unify_binds_free_supertype_tail_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:123:unifier_2_unify_binds_free_supertype_tail_pack`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item unifier_2_unify_binds_free_supertype_tail_pack

  #[cfg(test)]
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
}

mod unifier_2_unify_free_type_intersection_in_ub_from_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:136:unifier_2_unify_free_type_intersection_in_ub_from_union`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item unifier_2_unify_free_type_intersection_in_ub_from_union

  #[cfg(test)]
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
}

mod unifier_2_unify_free_type_lb_from_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:150:unifier_2_unify_free_type_lb_from_intersection`
  //! Source: `tests/Unifier2.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Unifier2.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/TypeArena.h
  //!   - includes -> source_file Analysis/include/Luau/Unifier2.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Unifier2.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record NegationType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item unifier_2_unify_free_type_lb_from_intersection

  #[cfg(test)]
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
}
