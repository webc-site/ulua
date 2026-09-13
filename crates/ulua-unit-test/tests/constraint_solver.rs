extern crate alloc;

mod constraint_solver_constraint_basics {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:10:constraint_solver_constraint_basics`
  //! Source: `tests/ConstraintSolver.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ConstraintSolver.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/ConstraintSolver.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item constraint_solver_constraint_basics

  #[cfg(test)]
  #[test]
  fn constraint_solver_constraint_basics() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a = 55
        local b = a
    "#,
      ),
      None,
    );

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod constraint_solver_generic_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:20:constraint_solver_generic_function`
  //! Source: `tests/ConstraintSolver.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ConstraintSolver.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/ConstraintSolver.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item constraint_solver_generic_function

  #[cfg(test)]
  #[test]
  fn constraint_solver_generic_function() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id(a)
            return a
        end
    "#,
      ),
      None,
    );

    assert_eq!(
      "<a>(a) -> a",
      to_string_type_id(fixture.require_type_string(&String::from("id")))
    );
  }
}

mod constraint_solver_proper_let_generalization {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:32:constraint_solver_proper_let_generalization`
  //! Source: `tests/ConstraintSolver.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ConstraintSolver.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/ConstraintSolver.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item constraint_solver_proper_let_generalization

  #[cfg(test)]
  #[test]
  fn constraint_solver_proper_let_generalization() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function a(c)
            local function d(e)
                return c
            end

            return d
        end

        local b = a(5)
    "#,
      ),
      None,
    );

    assert_eq!(
      "(unknown) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod constraint_solver_table_prop_access_diamond {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:51:constraint_solver_table_prop_access_diamond`
  //! Source: `tests/ConstraintSolver.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ConstraintSolver.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/ConstraintSolver.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item constraint_solver_table_prop_access_diamond

  #[cfg(test)]
  #[test]
  fn constraint_solver_table_prop_access_diamond() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        export type ItemDetails = { Id: number }

        export type AssetDetails = ItemDetails & {}
        export type BundleDetails = ItemDetails & {}

        export type CatalogPage = { AssetDetails | BundleDetails }

        local function isRestricted(item: number) end

        -- Clear all item tiles and create new ones for the items in the specified page
        local function displayPage(catalogPage: CatalogPage)
            for _, itemDetails in catalogPage do
                if isRestricted(itemDetails.Id) then
                    continue
                end
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
