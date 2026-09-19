extern crate alloc;

mod txn_log_colliding_coincident_logs_do_not_create_degenerate_unions {
  //! Source: `tests/TxnLog.test.cpp`

  #[test]
  fn txn_log_colliding_coincident_logs_do_not_create_degenerate_unions() {
    use core::mem::replace;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::txn_log::TxnLog,
      type_aliases::bound_type::BoundType,
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::txn_log_fixture::TxnLogFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    let mut fixture = TxnLogFixture::new();

    fixture
      .log
      .replace_type_id_t(fixture.a, BoundType::bound_t(fixture.b));
    fixture
      .log2
      .replace_type_id_t(fixture.a, BoundType::bound_t(fixture.b));

    let log2 = replace(&mut fixture.log2, TxnLog::new());
    unsafe {
      fixture
        .log
        .concat_as_union(log2, &mut fixture.arena as *mut _);
    }

    fixture.log.commit();

    assert_eq!("'a", to_string_type_id(fixture.a));
    assert_eq!("'a", to_string_type_id(fixture.b));
  }
}

mod txn_log_colliding_union_incoming_type_has_lesser_scope {
  //! Source: `tests/TxnLog.test.cpp`

  #[test]
  fn txn_log_colliding_union_incoming_type_has_lesser_scope() {
    use core::mem::replace;

    use ulua_analysis::{
      functions::get_type_alt_j::get_type_id,
      records::{free_type::FreeType, txn_log::TxnLog},
      type_aliases::{bound_type::BoundType, type_id::TypeId},
    };
    use ulua_common::fflag;
    use ulua_unit_test::{
      records::txn_log_fixture::TxnLogFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    let mut fixture = TxnLogFixture::new();

    fixture
      .log
      .replace_type_id_t(fixture.a, BoundType::bound_t(fixture.c));
    fixture
      .log2
      .replace_type_id_t(fixture.c, BoundType::bound_t(fixture.a));

    assert!(!fixture.log.pending_type_id(fixture.a).is_null());

    let log2 = replace(&mut fixture.log2, TxnLog::new());
    unsafe {
      fixture
        .log
        .concat_as_union(log2, &mut fixture.arena as *mut _);
    }

    assert!(fixture.log.pending_type_id(fixture.a).is_null());

    let pending = fixture.log.pending_type_id(fixture.c);
    assert!(!pending.is_null());

    let bound = unsafe {
      fixture
        .log
        .txn_log_get::<BoundType, TypeId>(fixture.c)
        .as_ref()
    };
    assert_eq!(fixture.a, bound.unwrap().bound_to);

    fixture.log.commit();

    assert!(get_type_id::<FreeType>(fixture.a).is_some());

    let bound = get_type_id::<BoundType>(fixture.c);
    assert_eq!(fixture.a, bound.unwrap().bound_to);
  }
}

mod txn_log_replacing_persistent_types_is_allowed_but_makes_the_log_radioactive {
  //! Source: `tests/TxnLog.test.cpp`

  #[test]
  fn txn_log_replacing_persistent_types_is_allowed_but_makes_the_log_radioactive() {
    use ulua_analysis::{functions::persist_type::persist, type_aliases::bound_type::BoundType};
    use ulua_unit_test::records::txn_log_fixture::TxnLogFixture;

    let mut fixture = TxnLogFixture::new();

    persist(fixture.g);

    fixture
      .log
      .replace_type_id_t(fixture.g, BoundType::bound_t(fixture.a));

    assert!(fixture.log.is_radioactive());
  }
}
