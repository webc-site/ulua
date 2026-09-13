extern crate alloc;

mod simplify_any_and_indeterminate_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:183:simplify_any_and_indeterminate_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_any_and_indeterminate_types

  #[cfg(test)]
  #[test]
  fn simplify_any_and_indeterminate_types() {
    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::union_type::UnionType};
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let any_ty = fixture.any_ty;
    let free_ty = fixture.free_ty;
    let generic_ty = fixture.generic_ty;
    let blocked_ty = fixture.blocked_ty;
    let pending_ty = fixture.pending_ty;
    let error_ty = fixture.error_ty;

    let actual = fixture.intersect_str(any_ty, free_ty);
    assert_eq!("'a | *error-type*", actual);
    let actual = fixture.intersect_str(free_ty, any_ty);
    assert_eq!("'a | *error-type*", actual);

    let actual = fixture.intersect_str(any_ty, generic_ty);
    assert_eq!("*error-type* | b", actual);
    let actual = fixture.intersect_str(generic_ty, any_ty);
    assert_eq!("*error-type* | b", actual);

    let actual = fixture.intersect(any_ty, blocked_ty);
    let any_rhs_blocked = get_type_id::<UnionType>(actual).expect("expected union");
    assert_eq!(2, any_rhs_blocked.options.len());
    assert_eq!(blocked_ty, any_rhs_blocked.options[0]);
    assert_eq!(error_ty, any_rhs_blocked.options[1]);

    let actual = fixture.intersect(blocked_ty, any_ty);
    let any_lhs_blocked = get_type_id::<UnionType>(actual).expect("expected union");
    assert_eq!(2, any_lhs_blocked.options.len());
    assert_eq!(blocked_ty, any_lhs_blocked.options[0]);
    assert_eq!(error_ty, any_lhs_blocked.options[1]);

    let actual = fixture.intersect(any_ty, pending_ty);
    let any_rhs_pending = get_type_id::<UnionType>(actual).expect("expected union");
    assert_eq!(2, any_rhs_pending.options.len());
    assert_eq!(pending_ty, any_rhs_pending.options[0]);
    assert_eq!(error_ty, any_rhs_pending.options[1]);

    let actual = fixture.intersect(pending_ty, any_ty);
    let any_lhs_pending = get_type_id::<UnionType>(actual).expect("expected union");
    assert_eq!(2, any_lhs_pending.options.len());
    assert_eq!(pending_ty, any_lhs_pending.options[0]);
    assert_eq!(error_ty, any_lhs_pending.options[1]);
  }
}

mod simplify_any_error_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:623:simplify_any_error_string`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item simplify_any_error_string

  #[cfg(test)]
  #[test]
  fn simplify_any_error_string() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::union_type::UnionType,
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let any_ty = fixture.any_ty;
    let error_ty = fixture.error_ty;
    let string_ty = fixture.string_ty;

    let err_string_ty = fixture.arena.add_type(UnionType {
      options: vec![error_ty, string_ty],
    });

    let res = fixture.intersect(any_ty, err_string_ty);

    assert_eq!(
      "*error-type* | string",
      to_string_type_id_to_string_options(res, &mut fixture.opts)
    );
  }
}

mod simplify_boolean_and_truthy_and_falsy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:174:simplify_boolean_and_truthy_and_falsy`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_boolean_and_truthy_and_falsy

  #[cfg(test)]
  #[test]
  fn simplify_boolean_and_truthy_and_falsy() {
    use alloc::vec;

    use ulua_analysis::records::union_type::UnionType;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let boolean_ty = fixture.boolean_ty;
    let nil_ty = fixture.nil_ty;
    let truthy_ty = fixture.truthy_ty;
    let true_ty = fixture.true_ty;
    let optional_boolean_ty = fixture.arena.add_type(UnionType {
      options: vec![boolean_ty, nil_ty],
    });

    let actual = fixture.intersect(boolean_ty, truthy_ty);
    assert_eq!(true_ty, actual);
    let actual = fixture.intersect(optional_boolean_ty, truthy_ty);
    assert_eq!(true_ty, actual);
  }
}

mod simplify_boolean_singletons {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:156:simplify_boolean_singletons`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_boolean_singletons

  #[cfg(test)]
  #[test]
  fn simplify_boolean_singletons() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let true_ty = fixture.true_ty;
    let false_ty = fixture.false_ty;
    let boolean_ty = fixture.boolean_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(true_ty, boolean_ty);
    assert_eq!(true_ty, actual);
    let actual = fixture.intersect(boolean_ty, true_ty);
    assert_eq!(true_ty, actual);

    let actual = fixture.intersect(false_ty, boolean_ty);
    assert_eq!(false_ty, actual);
    let actual = fixture.intersect(boolean_ty, false_ty);
    assert_eq!(false_ty, actual);

    let actual = fixture.intersect(false_ty, true_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(true_ty, false_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.union_(true_ty, boolean_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(boolean_ty, true_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(false_ty, boolean_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(boolean_ty, false_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(false_ty, true_ty);
    assert_eq!(boolean_ty, actual);
  }
}

mod simplify_bound_intersected_by_itself_should_be_itself {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:606:simplify_bound_intersected_by_itself_should_be_itself`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_bound_intersected_by_itself_should_be_itself

  #[cfg(test)]
  #[test]
  fn simplify_bound_intersected_by_itself_should_be_itself() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::blocked_type::BlockedType,
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let blocked = fixture.arena.add_type(BlockedType::default());

    let expected = to_string_type_id_to_string_options(blocked, &mut fixture.opts);
    let actual = fixture.intersect_str(blocked, blocked);
    assert_eq!(expected, actual);
  }
}

mod simplify_combine_disjoint_sealed_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:394:simplify_combine_disjoint_sealed_tables`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_combine_disjoint_sealed_tables

  #[cfg(test)]
  #[test]
  fn simplify_combine_disjoint_sealed_tables() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;

    let t1 = fixture.mk_table(&[("prop", string_ty)]);
    let t2 = fixture.mk_table(&[("second_prop", number_ty)]);

    let actual = fixture.intersect_str(t1, t2);
    assert_eq!("{ prop: string, second_prop: number }", actual);
  }
}

mod simplify_curious_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:491:simplify_curious_union`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_curious_union

  #[cfg(test)]
  #[test]
  fn simplify_curious_union() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{intersection_type::IntersectionType, union_type::UnionType},
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let free_ty = fixture.free_ty;
    let false_ty = fixture.false_ty;
    let nil_ty = fixture.nil_ty;
    let number_ty = fixture.number_ty;

    let false_intersection = fixture.arena.add_type(IntersectionType {
      parts: vec![free_ty, false_ty],
    });
    let nil_intersection = fixture.arena.add_type(IntersectionType {
      parts: vec![free_ty, nil_ty],
    });
    let curious = fixture.arena.add_type(UnionType {
      options: vec![false_intersection, nil_intersection],
    });

    let actual = fixture.union_(curious, number_ty);
    assert_eq!(
      "('a & false) | ('a & nil) | number",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
  }
}

mod simplify_cyclic_never_union_and_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:612:simplify_cyclic_never_union_and_string`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_cyclic_never_union_and_string

  #[cfg(test)]
  #[test]
  fn simplify_cyclic_never_union_and_string() {
    use alloc::vec;

    use ulua_analysis::{
      functions::get_mutable_type::get_mutable_type_id, records::union_type::UnionType,
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let never_ty = fixture.never_ty;
    let string_ty = fixture.string_ty;

    let left_type = fixture.arena.add_type(UnionType {
      options: vec![never_ty, never_ty],
    });
    let left_union = get_mutable_type_id::<UnionType>(left_type).expect("expected mutable union");
    left_union.options[0] = left_type;

    let actual = fixture.union_(left_type, string_ty);
    assert_eq!(string_ty, actual);
  }
}

mod simplify_error_and_indeterminate_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:260:simplify_error_and_indeterminate_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::isIntersection (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_error_and_indeterminate_types

  #[cfg(test)]
  #[test]
  fn simplify_error_and_indeterminate_types() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let error_ty = fixture.error_ty;
    let free_ty = fixture.free_ty;
    let generic_ty = fixture.generic_ty;
    let blocked_ty = fixture.blocked_ty;
    let pending_ty = fixture.pending_ty;

    let actual = fixture.intersect_str(error_ty, free_ty);
    assert_eq!("'a & *error-type*", actual);
    let actual = fixture.intersect_str(free_ty, error_ty);
    assert_eq!("'a & *error-type*", actual);

    let actual = fixture.intersect_str(error_ty, generic_ty);
    assert_eq!("*error-type* & b", actual);
    let actual = fixture.intersect_str(generic_ty, error_ty);
    assert_eq!("*error-type* & b", actual);

    let actual = fixture.intersect(error_ty, blocked_ty);
    assert!(fixture.is_intersection(actual));
    let actual = fixture.intersect(blocked_ty, error_ty);
    assert!(fixture.is_intersection(actual));

    let actual = fixture.intersect(error_ty, pending_ty);
    assert!(fixture.is_intersection(actual));
    let actual = fixture.intersect(pending_ty, error_ty);
    assert!(fixture.is_intersection(actual));
  }
}

mod simplify_error_and_other_tops_and_bottom_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:249:simplify_error_and_other_tops_and_bottom_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_error_and_other_tops_and_bottom_types

  #[cfg(test)]
  #[test]
  fn simplify_error_and_other_tops_and_bottom_types() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let error_ty = fixture.error_ty;
    let any_ty = fixture.any_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(error_ty, error_ty);
    assert_eq!(error_ty, actual);

    let actual = fixture.intersect(error_ty, any_ty);
    assert_eq!(error_ty, actual);
    let actual = fixture.intersect(any_ty, error_ty);
    assert_eq!(error_ty, actual);

    let actual = fixture.intersect(error_ty, never_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(never_ty, error_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_error_string_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:632:simplify_error_string_any`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item simplify_error_string_any

  #[cfg(test)]
  #[test]
  fn simplify_error_string_any() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::union_type::UnionType,
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let any_ty = fixture.any_ty;
    let error_ty = fixture.error_ty;
    let string_ty = fixture.string_ty;

    let err_string_ty = fixture.arena.add_type(UnionType {
      options: vec![error_ty, string_ty],
    });

    let res = fixture.intersect(err_string_ty, any_ty);

    assert_eq!(
      "*error-type* | string",
      to_string_type_id_to_string_options(res, &mut fixture.opts)
    );
  }
}

mod simplify_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:516:simplify_extern_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_extern_types

  #[cfg(test)]
  #[test]
  fn simplify_extern_types() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let child_class_ty = fixture.child_class_ty;
    let parent_class_ty = fixture.parent_class_ty;
    let unrelated_class_ty = fixture.unrelated_class_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(child_class_ty, parent_class_ty);
    assert_eq!(child_class_ty, actual);

    let actual = fixture.intersect(parent_class_ty, child_class_ty);
    assert_eq!(child_class_ty, actual);

    let actual = fixture.union_(child_class_ty, parent_class_ty);
    assert_eq!(parent_class_ty, actual);

    let actual = fixture.union_(parent_class_ty, child_class_ty);
    assert_eq!(parent_class_ty, actual);

    let actual = fixture.intersect(child_class_ty, unrelated_class_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_free_type_bound_by_any_with_any {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:597:simplify_free_type_bound_by_any_with_any`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_free_type_bound_by_any_with_any

  #[cfg(test)]
  #[test]
  fn simplify_free_type_bound_by_any_with_any() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let free_ty = fixture.free_ty;
    let any_ty = fixture.any_ty;

    let actual = fixture.intersect_str(free_ty, any_ty);
    assert_eq!("'a | *error-type*", actual);
    let actual = fixture.intersect_str(any_ty, free_ty);
    assert_eq!("'a | *error-type*", actual);

    let actual = fixture.intersect_str(free_ty, any_ty);
    assert_eq!("'a | *error-type*", actual);
    let actual = fixture.intersect_str(any_ty, free_ty);
    assert_eq!("'a | *error-type*", actual);
  }
}

mod simplify_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:324:simplify_functions`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_functions

  #[cfg(test)]
  #[test]
  fn simplify_functions() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let fn1_ty = fixture.fn1_ty;
    let fn2_ty = fixture.fn2_ty;
    let function_ty = fixture.function_ty;

    let actual = fixture.intersect(fn1_ty, function_ty);
    assert_eq!(fn1_ty, actual);
    let actual = fixture.intersect(function_ty, fn1_ty);
    assert_eq!(fn1_ty, actual);

    let actual = fixture.intersect_str(fn1_ty, fn2_ty);
    assert_eq!("(() -> ()) & ((...any) -> ())", actual);
  }
}

mod simplify_intersect_parts_empty_table_non_empty {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:674:simplify_intersect_parts_empty_table_non_empty`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item simplify_intersect_parts_empty_table_non_empty

  #[cfg(test)]
  #[test]
  fn simplify_intersect_parts_empty_table_non_empty() {
    use alloc::vec;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        simplify_intersection_simplify_alt_b::simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{
        property_type::Property, table_type::TableType, to_string_options::ToStringOptions,
        type_arena::TypeArena, type_ids::TypeIds, union_type::UnionType,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut base = Fixture::fixture_bool(false);
    base.get_frontend();
    let builtins = base.builtin_types;
    let mut arena = TypeArena::default();

    let mut empty = TableType::new();
    empty.state = TableState::Sealed;
    let empty_table = arena.add_type(empty);

    let number_type = unsafe { (*builtins).number_type };
    let string_type = unsafe { (*builtins).string_type };
    let mut non_empty = TableType::new();
    non_empty.props.insert(
      "p".to_string(),
      Property::rw_type_id(arena.add_type(UnionType {
        options: vec![number_type, string_type],
      })),
    );
    non_empty.state = TableState::Sealed;
    let non_empty_table = arena.add_type(non_empty);

    let mut parts = TypeIds::new();
    parts.type_ids_initializer_list_type_id(&[non_empty_table, empty_table]);
    let result = simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
      builtins, &mut arena, parts,
    )
    .result;

    assert_eq!(
      "{ p: number | string }",
      to_string_type_id_to_string_options(result, &mut ToStringOptions::default())
    );
  }
}

mod simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:542:simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type

  #[cfg(test)]
  #[test]
  fn simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type()
  {
    use alloc::vec;

    use ulua_analysis::records::intersection_type::IntersectionType;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let free_ty = fixture.free_ty;
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let never_ty = fixture.never_ty;

    let t1 = fixture.arena.add_type(IntersectionType {
      parts: vec![free_ty, string_ty],
    });

    let actual = fixture.intersect(t1, number_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_negated_function_does_not_intersect_cleanly_with_truthy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:366:simplify_negated_function_does_not_intersect_cleanly_with_truthy`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::isIntersection (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_negated_function_does_not_intersect_cleanly_with_truthy

  #[cfg(test)]
  #[test]
  fn simplify_negated_function_does_not_intersect_cleanly_with_truthy() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let function_ty = fixture.function_ty;
    let truthy_ty = fixture.truthy_ty;

    let negated_function_ty = fixture.mk_negation(function_ty);
    let actual = fixture.intersect(negated_function_ty, truthy_ty);
    assert!(fixture.is_intersection(actual));
  }
}

mod simplify_negated_top_function_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:333:simplify_negated_top_function_type`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_negated_top_function_type

  #[cfg(test)]
  #[test]
  fn simplify_negated_top_function_type() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let function_ty = fixture.function_ty;
    let number_ty = fixture.number_ty;
    let falsy_ty = fixture.falsy_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;

    let negated_function_ty = fixture.mk_negation(function_ty);

    let actual = fixture.intersect(number_ty, negated_function_ty);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(negated_function_ty, number_ty);
    assert_eq!(number_ty, actual);

    let actual = fixture.intersect(falsy_ty, negated_function_ty);
    assert_eq!(falsy_ty, actual);
    let actual = fixture.intersect(negated_function_ty, falsy_ty);
    assert_eq!(falsy_ty, actual);

    let f = fixture.mk_function(string_ty, number_ty);

    let actual = fixture.intersect(f, negated_function_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(negated_function_ty, f);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_negations {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:500:simplify_negations`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_negations

  #[cfg(test)]
  #[test]
  fn simplify_negations() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;

    let not_number_ty = fixture.mk_negation(number_ty);
    let not_string_ty = fixture.mk_negation(string_ty);

    let actual = fixture.intersect(number_ty, not_number_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(number_ty, not_string_ty);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(not_string_ty, number_ty);
    assert_eq!(number_ty, actual);
  }
}

mod simplify_negations_of_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:527:simplify_negations_of_extern_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_negations_of_extern_types

  #[cfg(test)]
  #[test]
  fn simplify_negations_of_extern_types() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let child_class_ty = fixture.child_class_ty;
    let parent_class_ty = fixture.parent_class_ty;
    let never_ty = fixture.never_ty;

    let not_child_class_ty = fixture.mk_negation(child_class_ty);
    let not_parent_class_ty = fixture.mk_negation(parent_class_ty);

    let actual = fixture.intersect(child_class_ty, not_parent_class_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(not_parent_class_ty, child_class_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect_str(not_child_class_ty, parent_class_ty);
    assert_eq!("Parent & ~Child", actual);

    let actual = fixture.intersect_str(parent_class_ty, not_child_class_ty);
    assert_eq!("Parent & ~Child", actual);

    let actual = fixture.intersect(not_child_class_ty, not_parent_class_ty);
    assert_eq!(not_parent_class_ty, actual);

    let actual = fixture.intersect(not_parent_class_ty, not_child_class_ty);
    assert_eq!(not_parent_class_ty, actual);
  }
}

mod simplify_nested_table_tag_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:452:simplify_nested_table_tag_test`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_nested_table_tag_test

  #[cfg(test)]
  #[test]
  fn simplify_nested_table_tag_test() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let hello_ty = fixture.hello_ty;
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;

    let subtable1 = fixture.mk_table(&[("tag", hello_ty), ("subprop", number_ty)]);
    let t1 = fixture.mk_table(&[("subtable", subtable1), ("prop", string_ty)]);
    let subtable2 = fixture.mk_table(&[("tag", hello_ty)]);
    let t2 = fixture.mk_table(&[("subtable", subtable2)]);

    let actual = fixture.intersect(t1, t2);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(t2, t1);
    assert_eq!(t1, actual);
  }
}

mod simplify_nil {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:148:simplify_nil`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_nil

  #[cfg(test)]
  #[test]
  fn simplify_nil() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let nil_ty = fixture.nil_ty;
    let never_ty = fixture.never_ty;
    let number_ty = fixture.number_ty;
    let true_ty = fixture.true_ty;
    let table_ty = fixture.table_ty;

    let actual = fixture.intersect(nil_ty, nil_ty);
    assert_eq!(nil_ty, actual);
    let actual = fixture.intersect(nil_ty, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(nil_ty, true_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(nil_ty, table_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_non_disjoint_tables_do_not_simplify {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:402:simplify_non_disjoint_tables_do_not_simplify`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_non_disjoint_tables_do_not_simplify

  #[cfg(test)]
  #[test]
  fn simplify_non_disjoint_tables_do_not_simplify() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let unknown_ty = fixture.unknown_ty;
    let number_ty = fixture.number_ty;

    let t1 = fixture.mk_table(&[("prop", string_ty)]);
    let t2 = fixture.mk_table(&[("prop", unknown_ty), ("second_prop", number_ty)]);

    let actual = fixture.intersect_str(t1, t2);
    assert_eq!(
      "{ prop: string } & { prop: unknown, second_prop: number }",
      actual
    );
  }
}

mod simplify_non_disjoint_tables_do_not_simplify_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:413:simplify_non_disjoint_tables_do_not_simplify_2`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_non_disjoint_tables_do_not_simplify_2

  #[cfg(test)]
  #[test]
  fn simplify_non_disjoint_tables_do_not_simplify_2() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let unknown_ty = fixture.unknown_ty;

    let t1 = fixture.mk_table(&[("prop", string_ty), ("third_prop", number_ty)]);
    let t2 = fixture.mk_table(&[("prop", unknown_ty), ("second_prop", number_ty)]);

    let actual = fixture.intersect_str(t1, t2);
    assert_eq!(
      "{ prop: string, third_prop: number } & { prop: unknown, second_prop: number }",
      actual
    );
  }
}

mod simplify_optional_overloaded_function_and_top_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:349:simplify_optional_overloaded_function_and_top_function`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_optional_overloaded_function_and_top_function

  #[cfg(test)]
  #[test]
  fn simplify_optional_overloaded_function_and_top_function() {
    use alloc::vec;

    use ulua_analysis::records::{intersection_type::IntersectionType, union_type::UnionType};
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let nil_ty = fixture.nil_ty;
    let function_ty = fixture.function_ty;

    let f1 = fixture.mk_function(number_ty, string_ty);
    let f2 = fixture.mk_function(string_ty, number_ty);

    let f12 = fixture.arena.add_type(IntersectionType {
      parts: vec![f1, f2],
    });
    let t = fixture.arena.add_type(UnionType {
      options: vec![f12, nil_ty],
    });

    let not_function_ty = fixture.mk_negation(function_ty);

    let actual = fixture.intersect(t, not_function_ty);
    assert_eq!(nil_ty, actual);
    let actual = fixture.intersect(not_function_ty, t);
    assert_eq!(nil_ty, actual);
  }
}

mod simplify_overload_negation_refinement_is_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:122:simplify_overload_negation_refinement_is_never`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
  //!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_overload_negation_refinement_is_never

  #[cfg(test)]
  #[test]
  fn simplify_overload_negation_refinement_is_never() {
    use alloc::vec;

    use ulua_analysis::records::{intersection_type::IntersectionType, union_type::UnionType};
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let error_ty = fixture.error_ty;
    let function_ty = fixture.function_ty;
    let never_ty = fixture.never_ty;

    let f1 = fixture.mk_function(string_ty, number_ty);
    let f2 = fixture.mk_function(number_ty, string_ty);
    let intersection = fixture.arena.add_type(IntersectionType {
      parts: vec![f1, f2],
    });
    let union_t = fixture.arena.add_type(UnionType {
      options: vec![error_ty, function_ty],
    });
    let negation_t = fixture.mk_negation(union_t);

    let actual = fixture.intersect(intersection, negation_t);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_primitives {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:283:simplify_primitives`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_primitives

  #[cfg(test)]
  #[test]
  fn simplify_primitives() {
    use ulua_analysis::records::primitive_type::PrimitiveType;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;
    let function_ty = fixture.function_ty;
    let table_ty = fixture.table_ty;
    let any_ty = fixture.any_ty;
    let nil_ty = fixture.nil_ty;

    let number_ty_duplicate = fixture.arena.add_type(PrimitiveType {
      r#type: PrimitiveType::NUMBER,
      metatable: None,
    });

    let actual = fixture.intersect(number_ty, number_ty_duplicate);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(number_ty, string_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(never_ty, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(number_ty, never_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(never_ty, function_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(function_ty, never_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(never_ty, table_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(table_ty, never_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect_str(any_ty, number_ty);
    assert_eq!("*error-type* | number", actual);
    let actual = fixture.intersect_str(number_ty, any_ty);
    assert_eq!("*error-type* | number", actual);

    let actual = fixture.intersect(string_ty, nil_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(nil_ty, string_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_primitives_and_falsy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:307:simplify_primitives_and_falsy`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_primitives_and_falsy

  #[cfg(test)]
  #[test]
  fn simplify_primitives_and_falsy() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let falsy_ty = fixture.falsy_ty;
    let never_ty = fixture.never_ty;
    let nil_ty = fixture.nil_ty;

    let actual = fixture.intersect(number_ty, falsy_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(falsy_ty, number_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(nil_ty, falsy_ty);
    assert_eq!(nil_ty, actual);
    let actual = fixture.intersect(falsy_ty, nil_ty);
    assert_eq!(nil_ty, actual);
  }
}

mod simplify_primitives_and_singletons {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:316:simplify_primitives_and_singletons`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_primitives_and_singletons

  #[cfg(test)]
  #[test]
  fn simplify_primitives_and_singletons() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let hello_ty = fixture.hello_ty;
    let string_ty = fixture.string_ty;
    let world_ty = fixture.world_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(hello_ty, string_ty);
    assert_eq!(hello_ty, actual);
    let actual = fixture.intersect(string_ty, hello_ty);
    assert_eq!(hello_ty, actual);

    let actual = fixture.intersect(world_ty, hello_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_read_x_child_x_parent {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:657:simplify_read_x_child_x_parent`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_read_x_child_x_parent

  #[cfg(test)]
  #[test]
  fn simplify_read_x_child_x_parent() {
    use alloc::sync::Arc;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{property_type::Property, scope::Scope},
    };
    use ulua_unit_test::{
      functions::create_some_extern_types::create_some_extern_types,
      records::simplify_fixture::SimplifyFixture,
    };

    let mut fixture = SimplifyFixture::default();

    let (parent_ty, child_ty) = {
      let frontend = fixture.base.get_frontend();
      create_some_extern_types(frontend);

      let module_scope = frontend.globals.global_scope();
      let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

      unsafe {
        (
          (*module_scope_ptr)
            .exported_type_bindings
            .get("Parent")
            .expect("Parent exported type binding")
            .r#type(),
          (*module_scope_ptr)
            .exported_type_bindings
            .get("Child")
            .expect("Child exported type binding")
            .r#type(),
        )
      }
    };

    let left_ty = fixture.mk_table_props(&[("x", Property::readonly(child_ty))]);
    let right_ty = fixture.mk_table(&[("x", parent_ty)]);

    let actual = fixture.intersect(left_ty, right_ty);
    assert_eq!(
      "{ read x: Child } & { x: Parent }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
  }
}

mod simplify_relate_coincident_minus_one_prop_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:727:simplify_relate_coincident_minus_one_prop_tables`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_relate_coincident_minus_one_prop_tables

  #[cfg(test)]
  #[test]
  fn simplify_relate_coincident_minus_one_prop_tables() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::property_type::Property,
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let boolean_ty = fixture.boolean_ty;
    let string_ty = fixture.string_ty;

    let left_ty = fixture.mk_table_props(&[
      ("x", Property::rw_type_id(number_ty)),
      ("y", Property::rw_type_id(boolean_ty)),
    ]);
    let right_ty = fixture.mk_table_props(&[
      ("x", Property::rw_type_id(number_ty)),
      ("y", Property::rw_type_id(boolean_ty)),
      ("z", Property::rw_type_id(string_ty)),
    ]);

    let actual = fixture.intersect_str(left_ty, right_ty);
    assert_eq!(
      "{ x: number, y: boolean } & { x: number, y: boolean, z: string }",
      actual
    );
    let actual = fixture.intersect_str(right_ty, left_ty);
    assert_eq!(
      "{ x: number, y: boolean } & { x: number, y: boolean, z: string }",
      actual
    );

    let actual = fixture.union_(left_ty, right_ty);
    assert_eq!(
      "{ x: number, y: boolean } | { x: number, y: boolean, z: string }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
    let actual = fixture.union_(right_ty, left_ty);
    assert_eq!(
      "{ x: number, y: boolean } | { x: number, y: boolean, z: string }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
  }
}

mod simplify_relate_read_only_number_with_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:710:simplify_relate_read_only_number_with_number`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_relate_read_only_number_with_number

  #[cfg(test)]
  #[test]
  fn simplify_relate_read_only_number_with_number() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{property_type::Property, union_type::UnionType},
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let nil_ty = fixture.nil_ty;

    let left_ty = fixture.mk_table_props(&[("x", Property::readonly(number_ty))]);
    let optional_number = fixture.arena.add_type(UnionType {
      options: vec![nil_ty, number_ty],
    });
    let right_ty = fixture.mk_table_props(&[("x", Property::rw_type_id(optional_number))]);

    let actual = fixture.intersect_str(left_ty, right_ty);
    assert_eq!("{ read x: number } & { x: number? }", actual);
    let actual = fixture.intersect_str(right_ty, left_ty);
    assert_eq!("{ read x: number } & { x: number? }", actual);

    let actual = fixture.union_(left_ty, right_ty);
    assert_eq!(
      "{ read x: number } | { x: number? }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
    let actual = fixture.union_(right_ty, left_ty);
    assert_eq!(
      "{ read x: number } | { x: number? }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
  }
}

mod simplify_relate_write_only_number_with_number {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:688:simplify_relate_write_only_number_with_number`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_relate_write_only_number_with_number

  #[cfg(test)]
  #[test]
  fn simplify_relate_write_only_number_with_number() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{property_type::Property, union_type::UnionType},
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let nil_ty = fixture.nil_ty;

    let left_ty = fixture.mk_table_props(&[("x", Property::writeonly(number_ty))]);
    let optional_number = fixture.arena.add_type(UnionType {
      options: vec![nil_ty, number_ty],
    });
    let right_ty = fixture.mk_table_props(&[("x", Property::rw_type_id(optional_number))]);

    let actual = fixture.intersect_str(left_ty, right_ty);
    assert_eq!("{ write x: number } & { x: number? }", actual);
    let actual = fixture.intersect_str(right_ty, left_ty);
    assert_eq!("{ write x: number } & { x: number? }", actual);

    let actual = fixture.union_(left_ty, right_ty);
    assert_eq!(
      "{ write x: number } | { x: number? }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
    let actual = fixture.union_(right_ty, left_ty);
    assert_eq!(
      "{ write x: number } | { x: number? }",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
  }
}

mod simplify_simplify_stops_at_cycles {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:571:simplify_simplify_stops_at_cycles`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_simplify_stops_at_cycles

  #[cfg(test)]
  #[test]
  fn simplify_simplify_stops_at_cycles() {
    use ulua_analysis::{
      functions::get_mutable_type::get_mutable_type_id,
      records::{property_type::Property, table_type::TableType},
    };
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let unknown_ty = fixture.unknown_ty;
    let any_ty = fixture.any_ty;

    let t = fixture.mk_table(&[]);
    let tt = get_mutable_type_id::<TableType>(t).expect("expected table");

    let t2 = fixture.mk_table(&[]);
    let t2t = get_mutable_type_id::<TableType>(t2).expect("expected table");

    tt.props
      .insert("cyclic".to_string(), Property::rw_type_id(t2));
    t2t
      .props
      .insert("cyclic".to_string(), Property::rw_type_id(t));

    let actual = fixture.intersect(t, unknown_ty);
    assert_eq!(t, actual);

    let actual = fixture.intersect(unknown_ty, t);
    assert_eq!(t, actual);

    let actual = fixture.intersect(t2, unknown_ty);
    assert_eq!(t2, actual);

    let actual = fixture.intersect(unknown_ty, t2);
    assert_eq!(t2, actual);

    let expected = "*error-type* | t1 where t1 = { cyclic: { cyclic: t1 } }";

    let actual = fixture.intersect_str(t, any_ty);
    assert_eq!(expected, actual);

    let actual = fixture.intersect_str(any_ty, t);
    assert_eq!(expected, actual);

    let actual = fixture.intersect_str(t2, any_ty);
    assert_eq!(expected, actual);

    let actual = fixture.intersect_str(any_ty, t2);
    assert_eq!(expected, actual);
  }
}

mod simplify_some_tables_are_really_never {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:552:simplify_some_tables_are_really_never`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_some_tables_are_really_never

  #[cfg(test)]
  #[test]
  fn simplify_some_tables_are_really_never() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let any_ty = fixture.any_ty;
    let unknown_ty = fixture.unknown_ty;
    let number_ty = fixture.number_ty;
    let never_ty = fixture.never_ty;

    let not_any_ty = fixture.mk_negation(any_ty);
    let t1 = fixture.mk_table(&[("someKey", not_any_ty)]);

    let actual = fixture.intersect(t1, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(number_ty, t1);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(t1, t1);
    assert_eq!(t1, actual);

    let not_unknown_ty = fixture.mk_negation(unknown_ty);
    let t2 = fixture.mk_table(&[("someKey", not_unknown_ty)]);

    let actual = fixture.intersect(t2, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(number_ty, t2);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(t2, t2);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_table_with_a_tag {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:441:simplify_table_with_a_tag`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_table_with_a_tag

  #[cfg(test)]
  #[test]
  fn simplify_table_with_a_tag() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let hello_ty = fixture.hello_ty;

    let t1 = fixture.mk_table(&[("tag", string_ty), ("prop", number_ty)]);
    let t2 = fixture.mk_table(&[("tag", hello_ty)]);

    let actual = fixture.intersect_str(t1, t2);
    assert_eq!("{ prop: number, tag: string } & { tag: \"hello\" }", actual);
    let actual = fixture.intersect_str(t2, t1);
    assert_eq!("{ prop: number, tag: string } & { tag: \"hello\" }", actual);
  }
}

mod simplify_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:376:simplify_tables`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item simplify_tables

  #[cfg(test)]
  #[test]
  fn simplify_tables() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let table_ty = fixture.table_ty;
    let never_ty = fixture.never_ty;
    let function_ty = fixture.function_ty;
    let hello_ty = fixture.hello_ty;

    let t1 = fixture.mk_table(&[("tag", string_ty)]);

    let actual = fixture.intersect(t1, table_ty);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(t1, function_ty);
    assert_eq!(never_ty, actual);

    let t2 = fixture.mk_table(&[("tag", hello_ty)]);

    let actual = fixture.intersect(t1, t2);
    assert_eq!(t2, actual);
    let actual = fixture.intersect(t2, t1);
    assert_eq!(t2, actual);

    let t3 = fixture.mk_table(&[]);

    let actual = fixture.intersect(t1, t3);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(t3, t1);
    assert_eq!(t1, actual);
  }
}

mod simplify_tables_and_top_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:421:simplify_tables_and_top_table`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_tables_and_top_table

  #[cfg(test)]
  #[test]
  fn simplify_tables_and_top_table() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let table_ty = fixture.table_ty;
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let never_ty = fixture.never_ty;

    let not_table_type = fixture.mk_negation(table_ty);
    let t1 = fixture.mk_table(&[("prop", string_ty), ("another", number_ty)]);

    let actual = fixture.intersect(t1, table_ty);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(table_ty, t1);
    assert_eq!(t1, actual);

    let actual = fixture.intersect(t1, not_table_type);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(not_table_type, t1);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_tables_and_truthy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:433:simplify_tables_and_truthy`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_tables_and_truthy

  #[cfg(test)]
  #[test]
  fn simplify_tables_and_truthy() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let truthy_ty = fixture.truthy_ty;

    let t1 = fixture.mk_table(&[("prop", string_ty), ("another", number_ty)]);

    let actual = fixture.intersect(t1, truthy_ty);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(truthy_ty, t1);
    assert_eq!(t1, actual);
  }
}

mod simplify_top_class_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:511:simplify_top_class_type`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_top_class_type

  #[cfg(test)]
  #[test]
  fn simplify_top_class_type() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let class_ty = fixture.class_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(class_ty, string_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_two_unions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:483:simplify_two_unions`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_two_unions

  #[cfg(test)]
  #[test]
  fn simplify_two_unions() {
    use alloc::vec;

    use ulua_analysis::records::union_type::UnionType;
    use ulua_common::DFInt;
    use ulua_unit_test::{
      records::simplify_fixture::SimplifyFixture, type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let _sfi = ScopedFastInt::new(&DFInt::LuauSimplificationComplexityLimit, 10);
    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let boolean_ty = fixture.boolean_ty;
    let string_ty = fixture.string_ty;
    let nil_ty = fixture.nil_ty;
    let table_ty = fixture.table_ty;
    let falsy_ty = fixture.falsy_ty;

    let t1 = fixture.arena.add_type(UnionType {
      options: vec![number_ty, boolean_ty, string_ty, nil_ty, table_ty],
    });

    let actual = fixture.intersect_str(t1, falsy_ty);
    assert_eq!("false?", actual);
  }
}

mod simplify_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:473:simplify_union`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_union

  #[cfg(test)]
  #[test]
  fn simplify_union() {
    use alloc::vec;

    use ulua_analysis::records::union_type::UnionType;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let nil_ty = fixture.nil_ty;
    let table_ty = fixture.table_ty;
    let truthy_ty = fixture.truthy_ty;
    let optional_string_ty = unsafe { (*fixture.base.builtin_types).optional_string_type };

    let t1 = fixture.arena.add_type(UnionType {
      options: vec![number_ty, string_ty, nil_ty, table_ty],
    });

    let actual = fixture.intersect(t1, nil_ty);
    assert_eq!(nil_ty, actual);

    let actual = fixture.intersect(optional_string_ty, truthy_ty);
    assert_eq!(string_ty, actual);
  }
}

mod simplify_union_where_lhs_elements_are_a_subset_of_the_rhs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:218:simplify_union_where_lhs_elements_are_a_subset_of_the_rhs`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item simplify_union_where_lhs_elements_are_a_subset_of_the_rhs

  #[cfg(test)]
  #[test]
  fn simplify_union_where_lhs_elements_are_a_subset_of_the_rhs() {
    use ulua_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;

    let lhs = fixture.union_(number_ty, string_ty);
    let rhs = fixture.union_(string_ty, number_ty);
    let actual = fixture.union_(lhs, rhs);

    assert_eq!(
      "number | string",
      to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
  }
}

mod simplify_unknown_and_concrete_simplify_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:241:simplify_unknown_and_concrete`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_unknown_and_concrete

  #[cfg(test)]
  #[test]
  fn simplify_unknown_and_concrete() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let unknown_ty = fixture.unknown_ty;
    let true_ty = fixture.true_ty;

    let actual = fixture.intersect(number_ty, unknown_ty);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(unknown_ty, number_ty);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(true_ty, unknown_ty);
    assert_eq!(true_ty, actual);
    let actual = fixture.intersect(unknown_ty, true_ty);
    assert_eq!(true_ty, actual);
  }
}

mod simplify_unknown_and_concrete_simplify_test_alt_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:275:simplify_unknown_and_concrete`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_unknown_and_concrete

  #[cfg(test)]
  #[test]
  fn simplify_unknown_and_concrete() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let error_ty = fixture.error_ty;
    let true_ty = fixture.true_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(number_ty, error_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(error_ty, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(true_ty, error_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(error_ty, true_ty);
    assert_eq!(never_ty, actual);
  }
}

mod simplify_unknown_and_indeterminate_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:226:simplify_unknown_and_indeterminate_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_unknown_and_indeterminate_types

  #[cfg(test)]
  #[test]
  fn simplify_unknown_and_indeterminate_types() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let unknown_ty = fixture.unknown_ty;
    let free_ty = fixture.free_ty;
    let generic_ty = fixture.generic_ty;
    let blocked_ty = fixture.blocked_ty;
    let pending_ty = fixture.pending_ty;

    let actual = fixture.intersect(unknown_ty, free_ty);
    assert_eq!(free_ty, actual);
    let actual = fixture.intersect(free_ty, unknown_ty);
    assert_eq!(free_ty, actual);

    let actual = fixture.intersect(unknown_ty, generic_ty);
    assert_eq!(generic_ty, actual);
    let actual = fixture.intersect(generic_ty, unknown_ty);
    assert_eq!(generic_ty, actual);

    let actual = fixture.intersect(unknown_ty, blocked_ty);
    assert_eq!(blocked_ty, actual);
    let actual = fixture.intersect(unknown_ty, blocked_ty);
    assert_eq!(blocked_ty, actual);

    let actual = fixture.intersect(unknown_ty, pending_ty);
    assert_eq!(pending_ty, actual);
    let actual = fixture.intersect(unknown_ty, pending_ty);
    assert_eq!(pending_ty, actual);
  }
}

mod simplify_unknown_and_other_tops_and_bottom_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:133:simplify_unknown_and_other_tops_and_bottom_types`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_unknown_and_other_tops_and_bottom_types

  #[cfg(test)]
  #[test]
  fn simplify_unknown_and_other_tops_and_bottom_types() {
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let unknown_ty = fixture.unknown_ty;
    let any_ty = fixture.any_ty;
    let never_ty = fixture.never_ty;
    let error_ty = fixture.error_ty;

    let actual = fixture.intersect(unknown_ty, unknown_ty);
    assert_eq!(unknown_ty, actual);

    let actual = fixture.intersect_str(unknown_ty, any_ty);
    assert_eq!("any", actual);
    let actual = fixture.intersect_str(any_ty, unknown_ty);
    assert_eq!("any", actual);

    let actual = fixture.intersect(unknown_ty, never_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(never_ty, unknown_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(unknown_ty, error_ty);
    assert_eq!(error_ty, actual);
    let actual = fixture.intersect(error_ty, unknown_ty);
    assert_eq!(error_ty, actual);
  }
}

mod simplify_x_number_y_number_read_x_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:649:simplify_x_number_y_number_read_x_unknown`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_x_number_y_number_read_x_unknown

  #[cfg(test)]
  #[test]
  fn simplify_x_number_y_number_read_x_unknown() {
    use ulua_analysis::records::property_type::Property;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let unknown_ty = fixture.unknown_ty;

    let left_ty = fixture.mk_table(&[("x", number_ty), ("y", number_ty)]);
    let right_ty = fixture.mk_table_props(&[("x", Property::readonly(unknown_ty))]);

    let actual = fixture.intersect(left_ty, right_ty);
    assert_eq!(left_ty, actual);
  }
}

mod simplify_x_number_y_number_x_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:641:simplify_x_number_y_number_x_unknown`
  //! Source: `tests/Simplify.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Simplify.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Simplify.h
  //! - incoming:
  //!   - declares <- source_file tests/Simplify.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
  //!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
  //!   - translates_to -> rust_item simplify_x_number_y_number_x_unknown

  #[cfg(test)]
  #[test]
  fn simplify_x_number_y_number_x_unknown() {
    use ulua_analysis::records::property_type::Property;
    use ulua_unit_test::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let unknown_ty = fixture.unknown_ty;

    let left_ty = fixture.mk_table(&[("x", number_ty), ("y", number_ty)]);
    let right_ty = fixture.mk_table_props(&[("x", Property::rw_type_id(unknown_ty))]);

    let actual = fixture.intersect(left_ty, right_ty);
    assert_eq!(left_ty, actual);
  }
}
