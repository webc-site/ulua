extern crate alloc;

mod fragment_autocomplete_after_func_new_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:549:fragment_autocomplete_after_func_new_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_after_func_new_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_after_func_new_line() {
    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f()
end

"#,
      ),
      &Position { line: 3, column: 0 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 0 },
        end: Position { line: 3, column: 0 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_after_func_same_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:535:fragment_autocomplete_after_func_same_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_after_func_same_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_after_func_same_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f()
end
"#,
      ),
      &Position { line: 2, column: 3 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 3 },
        end: Position { line: 2, column: 3 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_after_local_func_new_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:644:fragment_autocomplete_after_local_func_new_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_after_local_func_new_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_after_local_func_new_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f()
end

"#,
      ),
      &Position { line: 3, column: 0 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 0 },
        end: Position { line: 3, column: 0 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_after_local_func_same_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:630:fragment_autocomplete_after_local_func_same_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_after_local_func_same_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_after_local_func_same_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f()
end
"#,
      ),
      &Position { line: 2, column: 3 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 3 },
        end: Position { line: 2, column: 3 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_anonymous_autofilled_generic_named_arg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4686:fragment_autocomplete_anonymous_autofilled_generic_named_arg`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_anonymous_autofilled_generic_named_arg() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
    const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

    let source = String::from(
      "
local function foo<A>(f: (a: A) -> number, a: A)
\treturn f(a)
end
    ",
    );

    let dest = String::from(
      "
local function foo<A>(f: (a: A) -> number, a: A)
\treturn f(a)
end

foo(@1)
    ",
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        let expected_insert = "function(a): number  end";
        LUAU_ASSERT!(frag.result.is_some());
        let ac_results = &frag.result.as_ref().unwrap().ac_results;
        LUAU_ASSERT!(
          ac_results
            .entry_map
            .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME)
        );
        let entry = &ac_results.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
        assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
        assert!(entry.type_correct == TypeCorrectKind::Correct);
        LUAU_ASSERT!(entry.insert_text.is_some());
        assert_eq!(
          expected_insert,
          entry.insert_text.as_ref().unwrap().as_str()
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_anonymous_autofilled_generic_return_type {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4719:fragment_autocomplete_anonymous_autofilled_generic_return_type`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_anonymous_autofilled_generic_return_type() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
    const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

    let source = String::from(
      "
local function foo<A>(f: () -> A)
\treturn f()
end
    ",
    );

    let dest = String::from(
      "
local function foo<A>(f: () -> A)
\treturn f()
end

foo(@1)
    ",
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        let expected_insert = "function()  end";
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(
          ac.entry_map
            .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME) as usize,
          1
        );
        let entry = &ac.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
        assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
        assert!(entry.type_correct == TypeCorrectKind::Correct);
        assert!(entry.insert_text.is_some());
        assert_eq!(
          expected_insert,
          entry.insert_text.as_ref().unwrap().as_str()
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_anonymous_autofilled_generic_type_pack_vararg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4653:fragment_autocomplete_anonymous_autofilled_generic_type_pack_vararg`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_anonymous_autofilled_generic_type_pack_vararg() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::{autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind},
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
    const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

    let source = String::from(
      r#"
local function foo<A>(a: (...A) -> number, ...: A)
	return a(...)
end
    "#,
    );

    let dest = String::from(
      r#"
local function foo<A>(a: (...A) -> number, ...: A)
	return a(...)
end

foo(@1)
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        let expected_insert = "function(...): number  end";
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(
          ac.entry_map
            .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME)
        );
        let entry = &ac.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
        assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
        assert!(entry.type_correct == TypeCorrectKind::Correct);
        assert!(entry.insert_text.is_some());
        assert_eq!(
          expected_insert,
          entry.insert_text.as_ref().unwrap().as_str()
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_bad_range_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2597:fragment_autocomplete_bad_range_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_bad_range_1() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::{
      functions::linear_search_for_binding::linear_search_for_binding,
      records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
    };

    let source = String::from(
      r#"
local t = 1
"#,
    );
    let updated = String::from(
      r#"
t
@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let opt =
          unsafe { linear_search_for_binding(frag.result.as_ref().unwrap().fresh_scope, "t") };
        LUAU_ASSERT!(opt.is_some());
        assert_eq!("number", to_string_type_id(opt.unwrap()));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_bad_range_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2620:fragment_autocomplete_bad_range_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_bad_range_2() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::{
      functions::linear_search_for_binding::linear_search_for_binding,
      records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
    };

    let source = String::from(
      r#"
local t = 1
"#,
    );
    let updated = String::from(
      r#"
local t = 1
t@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let opt =
          unsafe { linear_search_for_binding(frag.result.as_ref().unwrap().fresh_scope, "t") };
        LUAU_ASSERT!(opt.is_some());
        assert_eq!("number", to_string_type_id(opt.unwrap()));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_bad_range_3 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2644:fragment_autocomplete_bad_range_3`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_bad_range_3() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    // This test makes less sense since we don't have an updated check that
    // includes l
    // instead this will recommend nothing useful because `local t` hasn't
    // been typechecked in the fresh module
    let source = String::from(
      r#"
l
"#,
    );
    let updated = String::from(
      r#"
local t = 1
l@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.status == FragmentAutocompleteStatus::Success);
        LUAU_ASSERT!(frag.result.is_some());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_before_func {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:521:fragment_autocomplete_before_func`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_before_func

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_before_func() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f()
end
"#,
      ),
      &Position { line: 1, column: 0 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 0 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_before_local_func {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:616:fragment_autocomplete_before_local_func`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_before_local_func

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_before_local_func() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f()
end
"#,
      ),
      &Position { line: 1, column: 0 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 0 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_bidirectionally_inferred_table_member {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4590:fragment_autocomplete_bidirectionally_inferred_table_member`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_bidirectionally_inferred_table_member() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type Foo = { foo1: string, bar1: number }
type Bar = { foo2: boolean, bar2: string }
type Baz = { foo3: number, bar3: boolean }

local X: Foo & Bar & Baz = {}
"#,
    );

    let dest = String::from(
      r#"
type Foo = { foo1: string, bar1: number }
type Bar = { foo2: boolean, bar2: string }
type Baz = { foo3: number, bar3: boolean }

local X: Foo & Bar & Baz = { f@1 }

"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("foo1"));
        assert!(ac.entry_map.contains_key("foo2"));
        assert!(ac.entry_map.contains_key("foo3"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_block_diff_added_locals_1 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3407:fragment_autocomplete_block_diff_added_locals_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function blockDiffStart (Analysis/src/FragmentAutocomplete.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_block_diff_added_locals_1

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_1() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::block_diff_start::block_diff_start, records::source_module::SourceModule,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    let mut stale = SourceModule::new();
    let mut fresh = SourceModule::new();
    let old = fixture.base.parse_helper_(&mut stale, String::from(""));
    let new = fixture.base.parse_helper_(
      &mut fresh,
      String::from(
        r#"local x = 4
local y = 3
local z = 3"#,
      ),
    );

    let pos = unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[2]) };
    assert_eq!(Some(Position { line: 0, column: 0 }), pos);
  }
}

mod fragment_autocomplete_block_diff_added_locals_1_e_2_e {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3420:fragment_autocomplete_block_diff_added_locals_1_e_2_e`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_1_e_2_e() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#""#);
    let dest = String::from(
      r#"local f1 = 4
local f2 = "a"
local f3 = f@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(FragmentAutocompleteStatus::Success == result.status);
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("f1"));
        assert!(ac.entry_map.contains_key("f2"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_block_diff_added_locals_1_e_2_e_in_the_middle {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3443:fragment_autocomplete_block_diff_added_locals_1_e_2_e_in_the_middle`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_1_e_2_e_in_the_middle() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#""#);
    let dest = String::from(
      r#"local f1 = 4
local f2 = f@1
local f3 = f
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, result.status);
        LUAU_ASSERT!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("f1"));
        assert!(!ac.entry_map.contains_key("f3"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_block_diff_added_locals_2 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3466:fragment_autocomplete_block_diff_added_locals_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function blockDiffStart (Analysis/src/FragmentAutocomplete.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_block_diff_added_locals_2

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::block_diff_start::block_diff_start, records::source_module::SourceModule,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    let mut stale = SourceModule::new();
    let mut fresh = SourceModule::new();
    let old = fixture
      .base
      .parse_helper_(&mut stale, String::from("local x = 4"));
    let new = fixture.base.parse_helper_(
      &mut fresh,
      String::from(
        r#"local x = 4
local y = 3
local z = 3"#,
      ),
    );

    let pos = unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[1]) };
    assert_eq!(Some(Position { line: 1, column: 0 }), pos);
  }
}

mod fragment_autocomplete_block_diff_added_locals_3_fragment_autocomplete_test {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3479:fragment_autocomplete_block_diff_added_locals_3`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function blockDiffStart (Analysis/src/FragmentAutocomplete.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_block_diff_added_locals_3

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::block_diff_start::block_diff_start, records::source_module::SourceModule,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    let mut stale = SourceModule::new();
    let mut fresh = SourceModule::new();
    let old = fixture.base.parse_helper_(
      &mut stale,
      String::from(
        r#"local x = 4
local y = 2 + 1"#,
      ),
    );
    let new = fixture.base.parse_helper_(
      &mut fresh,
      String::from(
        r#"local x = 4
local y = 3
local z = 3
local foo = 8"#,
      ),
    );

    let pos = unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[3]) };
    assert_eq!(Some(Position { line: 1, column: 0 }), pos);
  }
}

mod fragment_autocomplete_block_diff_added_locals_3_fragment_autocomplete_test_alt_b {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3494:fragment_autocomplete_block_diff_added_locals_3`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_3() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"local f1 = 4
local f2 = 2 + 1"#,
    );
    let dest = String::from(
      r#"local f1 = 4
local f2 = 3
local f3 = 3
local foo = 8 + @1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, result.status);
        LUAU_ASSERT!(result.result.is_some());
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("f1"));
        assert!(ac_results.entry_map.contains_key("f2"));
        assert!(ac_results.entry_map.contains_key("f3"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_block_diff_added_locals_fake_similarity {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3518:fragment_autocomplete_block_diff_added_locals_fake_similarity`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function blockDiffStart (Analysis/src/FragmentAutocomplete.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_block_diff_added_locals_fake_similarity

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_added_locals_fake_similarity() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::block_diff_start::block_diff_start, records::source_module::SourceModule,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    let mut stale = SourceModule::new();
    let mut fresh = SourceModule::new();
    let old = fixture.base.parse_helper_(
      &mut stale,
      String::from(
        r#"local x = 4
local y = true
local z = 2 + 1"#,
      ),
    );
    let new = fixture.base.parse_helper_(
      &mut fresh,
      String::from(
        r#"local x = 4
local y = "tr"
local z = 3
local foo = 8"#,
      ),
    );

    let pos = unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[2]) };
    assert_eq!(Some(Position { line: 2, column: 0 }), pos);
  }
}

mod fragment_autocomplete_block_diff_test_both_empty {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3382:fragment_autocomplete_block_diff_test_both_empty`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function blockDiffStart (Analysis/src/FragmentAutocomplete.cpp)
  //!   - translates_to -> rust_item fragment_autocomplete_block_diff_test_both_empty

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_test_both_empty() {
    use alloc::string::String;
    use core::ptr::null_mut;

    use ulua_analysis::{
      functions::block_diff_start::block_diff_start, records::source_module::SourceModule,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    let mut stale = SourceModule::new();
    let mut fresh = SourceModule::new();
    let old = fixture.base.parse_helper_(&mut stale, String::from(""));
    let new = fixture.base.parse_helper_(&mut fresh, String::from(""));

    let pos = unsafe { block_diff_start(old.root, new.root, null_mut()) };
    assert!(pos.is_none());
  }
}

mod fragment_autocomplete_block_diff_test_both_empty_e_2_e {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3392:fragment_autocomplete_block_diff_test_both_empty_e_2_e`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_block_diff_test_both_empty_e_2_e() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#"@1"#);

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, result.status);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_can_autocomplete_nested_property_access {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1742:fragment_autocomplete_can_autocomplete_nested_property_access`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_autocomplete_nested_property_access() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
local tbl = { abc = { def = 1234, egh = false } }
"#,
    );
    let updated = String::from(
      r#"
local tbl = { abc = { def = 1234, egh = false } }
tbl.abc.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        LUAU_ASSERT!(!fragment.result.as_ref().unwrap().fresh_scope.is_null());

        assert_eq!(
          2,
          fragment.result.as_ref().unwrap().ac_results.entry_map.len()
        );
        assert!(
          fragment
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("def")
        );
        assert!(
          fragment
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("egh")
        );
        assert_eq!(
          fragment.result.as_ref().unwrap().ac_results.context,
          AutocompleteContext::Property
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_can_autocomplete_simple_property_access {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1715:fragment_autocomplete_can_autocomplete_simple_property_access`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_autocomplete_simple_property_access() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
local tbl = { abc = 1234}
"#,
    );
    let updated = String::from(
      r#"
local tbl = { abc = 1234}
tbl. @1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let ac_results = &fragment.result.as_ref().unwrap().ac_results;

        assert_eq!(1, ac_results.entry_map.len());
        assert!(ac_results.entry_map.contains_key("abc"));
        assert_eq!(AutocompleteContext::Property, ac_results.context);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_can_parse_complete_fragments {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1214:fragment_autocomplete_can_parse_complete_fragments`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_can_parse_complete_fragments

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_parse_complete_fragments() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{
        ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
        ast_expr_local::AstExprLocal,
        ast_node::AstNode,
        ast_stat_local::AstStatLocal,
        location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.check_with_options(&String::from(
      r#"
local x = 4
local y = 5
"#,
    ));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
      .base
      .parse_fragment(
        &String::from(
          r#"
local x = 4
local y = 5
local z = x + y
"#,
        ),
        &Position {
          line: 3,
          column: 15,
        },
        None,
      )
      .expect("expected fragment parse result");

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 0 },
        end: Position {
          line: 3,
          column: 15
        },
      },
      unsafe { (*fragment.root).base.base.location }
    );
    assert_eq!("local z = x + y", fragment.fragment_to_parse);
    assert_eq!(4, fragment.ancestry.len());
    assert!(!fragment.root.is_null());
    assert_eq!(1, unsafe { (*fragment.root).body.size });

    let stat =
      unsafe { ast_node_as::<AstStatLocal>((*fragment.root).body.as_slice()[0] as *mut AstNode) };
    assert!(!stat.is_null());
    assert_eq!(1, unsafe { (*stat).vars.size });
    assert_eq!(1, unsafe { (*stat).values.size });
    assert_eq!("z", unsafe {
      CStr::from_ptr((*(*stat).vars.as_slice()[0]).name.value)
        .to_str()
        .unwrap()
    });

    let bin = unsafe { ast_node_as::<AstExprBinary>((*stat).values.as_slice()[0] as *mut AstNode) };
    assert!(!bin.is_null());
    assert_eq!(AstExprBinaryOp::Add, unsafe { (*bin).op });

    let lhs = unsafe { ast_node_as::<AstExprLocal>((*bin).left as *mut AstNode) };
    let rhs = unsafe { ast_node_as::<AstExprLocal>((*bin).right as *mut AstNode) };
    assert!(!lhs.is_null());
    assert!(!rhs.is_null());
    assert_eq!("x", unsafe {
      CStr::from_ptr((*(*lhs).local).name.value).to_str().unwrap()
    });
    assert_eq!("y", unsafe {
      CStr::from_ptr((*(*rhs).local).name.value).to_str().unwrap()
    });
  }
}

mod fragment_autocomplete_can_parse_fragments_in_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1261:fragment_autocomplete_can_parse_fragments_in_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_can_parse_fragments_in_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_parse_fragments_in_line() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{
        ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
        ast_expr_global::AstExprGlobal,
        ast_expr_local::AstExprLocal,
        ast_node::AstNode,
        ast_stat_local::AstStatLocal,
        location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.check_with_options(&String::from(
      r#"
local x = 4
local y = 5
"#,
    ));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
      .base
      .parse_fragment(
        &String::from(
          r#"
local x = 4
local z = x + y
local y = 5
"#,
        ),
        &Position {
          line: 2,
          column: 15,
        },
        None,
      )
      .expect("expected fragment parse result");

    assert_eq!("local z = x + y", fragment.fragment_to_parse);
    assert_eq!(4, fragment.ancestry.len());
    assert!(!fragment.root.is_null());
    assert_eq!(
      Location {
        begin: Position { line: 2, column: 0 },
        end: Position {
          line: 2,
          column: 15
        },
      },
      unsafe { (*fragment.root).base.base.location }
    );
    assert_eq!(1, unsafe { (*fragment.root).body.size });

    let stat =
      unsafe { ast_node_as::<AstStatLocal>((*fragment.root).body.as_slice()[0] as *mut AstNode) };
    assert!(!stat.is_null());
    assert_eq!(1, unsafe { (*stat).vars.size });
    assert_eq!(1, unsafe { (*stat).values.size });
    assert_eq!("z", unsafe {
      CStr::from_ptr((*(*stat).vars.as_slice()[0]).name.value)
        .to_str()
        .unwrap()
    });

    let bin = unsafe { ast_node_as::<AstExprBinary>((*stat).values.as_slice()[0] as *mut AstNode) };
    assert!(!bin.is_null());
    assert_eq!(AstExprBinaryOp::Add, unsafe { (*bin).op });

    let lhs = unsafe { ast_node_as::<AstExprLocal>((*bin).left as *mut AstNode) };
    let rhs = unsafe { ast_node_as::<AstExprGlobal>((*bin).right as *mut AstNode) };
    assert!(!lhs.is_null());
    assert!(!rhs.is_null());
    assert_eq!("x", unsafe {
      CStr::from_ptr((*(*lhs).local).name.value).to_str().unwrap()
    });
    assert_eq!("y", unsafe {
      CStr::from_ptr((*rhs).name.value).to_str().unwrap()
    });
  }
}

mod fragment_autocomplete_can_parse_in_correct_scope {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1307:fragment_autocomplete_can_parse_in_correct_scope`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_can_parse_in_correct_scope

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_parse_in_correct_scope() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.check_with_options(&String::from(
      r#"
        local myLocal = 4
        function abc()
             local myInnerLocal = 1

        end
"#,
    ));

    let fragment = fixture
      .base
      .parse_fragment(
        &String::from(
          r#"
        local myLocal = 4
        function abc()
             local myInnerLocal = 1

        end
"#,
        ),
        &Position { line: 6, column: 0 },
        None,
      )
      .expect("expected fragment parse result");

    assert_eq!("", fragment.fragment_to_parse);
  }
}

mod fragment_autocomplete_can_parse_multi_line_fragment_override {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1397:fragment_autocomplete_can_parse_multi_line_fragment_override`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - calls -> method AstArray::rbegin (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_can_parse_multi_line_fragment_override

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_parse_multi_line_fragment_override() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
        ast_node::AstNode, ast_stat_expr::AstStatExpr, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture
      .base
      .check_with_options(&String::from("function abc(foo: string) end"));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
      .base
      .parse_fragment(
        &String::from(
          r#"function abc(foo: string) end
abc(
"foo"
)
abc("bar")
"#,
        ),
        &Position { line: 2, column: 5 },
        Some(Position { line: 3, column: 1 }),
      )
      .expect("expected fragment parse result");

    assert_eq!("abc(\n\"foo\"\n)", fragment.fragment_to_parse);
    assert!(!fragment.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(fragment.nearest_statement as *mut AstNode) }.is_null()
    );
    assert!(fragment.ancestry.len() >= 2);

    let back = *fragment.ancestry.last().unwrap();
    assert!(!unsafe { ast_node_as::<AstExprConstantString>(back) }.is_null());
    assert_eq!(Position { line: 2, column: 0 }, unsafe {
      (*back).location.begin
    });
    assert_eq!(Position { line: 2, column: 5 }, unsafe {
      (*back).location.end
    });

    let parent = fragment.ancestry[fragment.ancestry.len() - 2];
    assert!(!unsafe { ast_node_as::<AstExprCall>(parent) }.is_null());
    assert_eq!(Position { line: 1, column: 0 }, unsafe {
      (*parent).location.begin
    });
    assert_eq!(Position { line: 3, column: 1 }, unsafe {
      (*parent).location.end
    });
  }
}

mod fragment_autocomplete_can_parse_single_line_fragment_override {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1334:fragment_autocomplete_can_parse_single_line_fragment_override`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
  //!   - calls -> method AstArray::rbegin (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_can_parse_single_line_fragment_override

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_parse_single_line_fragment_override() {
    use alloc::string::String;
    use core::{slice, str};

    use ulua_ast::{
      enums::quote_style_ast::QuoteStyle,
      records::{
        ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
        ast_node::AstNode, ast_stat_expr::AstStatExpr, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture
      .base
      .check_with_options(&String::from("function abc(foo: string) end"));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let source = String::from(
      r#"function abc(foo: string) end
abc("foo")
abc("bar")
"#,
    );

    let call_fragment = fixture
      .base
      .parse_fragment(
        &source,
        &Position { line: 1, column: 6 },
        Some(Position {
          line: 1,
          column: 10,
        }),
      )
      .expect("expected call fragment parse result");

    assert_eq!("abc(\"foo\")", call_fragment.fragment_to_parse);
    assert!(!call_fragment.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(call_fragment.nearest_statement as *mut AstNode) }
        .is_null()
    );
    assert!(call_fragment.ancestry.len() >= 2);

    let back = *call_fragment.ancestry.last().unwrap();
    assert!(!unsafe { ast_node_as::<AstExprConstantString>(back) }.is_null());
    assert_eq!(Position { line: 1, column: 4 }, unsafe {
      (*back).location.begin
    });
    assert_eq!(Position { line: 1, column: 9 }, unsafe {
      (*back).location.end
    });

    let parent = call_fragment.ancestry[call_fragment.ancestry.len() - 2];
    assert!(!unsafe { ast_node_as::<AstExprCall>(parent) }.is_null());
    assert_eq!(Position { line: 1, column: 0 }, unsafe {
      (*parent).location.begin
    });
    assert_eq!(
      Position {
        line: 1,
        column: 10
      },
      unsafe { (*parent).location.end }
    );

    let string_fragment = fixture
      .base
      .parse_fragment(
        &source,
        &Position { line: 1, column: 6 },
        Some(Position { line: 1, column: 9 }),
      )
      .expect("expected string fragment parse result");

    assert_eq!("abc(\"foo\"", string_fragment.fragment_to_parse);
    assert!(!string_fragment.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(string_fragment.nearest_statement as *mut AstNode) }
        .is_null()
    );
    assert!(!string_fragment.ancestry.is_empty());

    let back = *string_fragment.ancestry.last().unwrap();
    let as_string = unsafe { ast_node_as::<AstExprConstantString>(back) };
    assert!(!as_string.is_null());

    assert_eq!(Position { line: 1, column: 4 }, unsafe {
      (*as_string).base.base.location.begin
    });
    assert_eq!(Position { line: 1, column: 9 }, unsafe {
      (*as_string).base.base.location.end
    });
    let value = unsafe {
      slice::from_raw_parts(
        (*as_string).value.data as *const u8,
        (*as_string).value.size,
      )
    };
    assert_eq!("foo", str::from_utf8(value).unwrap());
    assert_eq!(QuoteStyle::QuotedSimple, unsafe {
      (*as_string).quote_style
    });
  }
}

mod fragment_autocomplete_can_typecheck_fragment_inserted_inline {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1499:fragment_autocomplete_can_typecheck_fragment_inserted_inline`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_typecheck_fragment_inserted_inline() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::scope::Scope,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::linear_search_for_binding::linear_search_for_binding,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let res = fixture.base.check_with_options(&String::from(
      r#"
local x = 4
local y = 5
"#,
    ));

    assert_eq!(0, res.errors.len(), "{:?}", res.errors);

    let fragment = fixture.base.check_fragment(
      &String::from(
        r#"
local x = 4
local z = x
local y = 5
"#,
      ),
      Position {
        line: 2,
        column: 11,
      },
      None,
    );

    let scope_ptr = Arc::as_ptr(&fragment.fresh_scope) as *mut Scope;
    let correct = unsafe { linear_search_for_binding(scope_ptr, "z") };
    LUAU_ASSERT!(correct.is_some());
    assert_eq!("number", to_string_type_id(correct.unwrap()));
  }
}

mod fragment_autocomplete_can_typecheck_simple_fragment {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1473:fragment_autocomplete_can_typecheck_simple_fragment`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_can_typecheck_simple_fragment() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id, records::scope::Scope,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::linear_search_for_binding::linear_search_for_binding,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let res = fixture.base.check_with_options(&String::from(
      r#"
local x = 4
local y = 5
"#,
    ));

    assert_eq!(0, res.errors.len(), "{:?}", res.errors);

    let fragment = fixture.base.check_fragment(
      &String::from(
        r#"
local x = 4
local y = 5
local z = x + y
"#,
      ),
      Position {
        line: 3,
        column: 15,
      },
      None,
    );

    let scope_ptr = Arc::as_ptr(&fragment.fresh_scope) as *mut Scope;
    let opt = unsafe { linear_search_for_binding(scope_ptr, "z") };
    LUAU_ASSERT!(opt.is_some());
    assert_eq!("number", to_string_type_id(opt.unwrap()));
  }
}

mod fragment_autocomplete_class_autocomplete_between_definitions {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5355:fragment_autocomplete_class_autocomplete_between_definitions`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_autocomplete_between_definitions() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function printvalue(self)
        print(self.value)
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function printvalue(self)
        print(self.value)
    end
    s@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          !frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("self")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_autocomplete_classname_inside_method {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5390:fragment_autocomplete_class_autocomplete_classname_inside_method`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_autocomplete_classname_inside_method() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function new()
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function new()
        return B@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("Bar"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_instance_dot_includes_method_from_outside {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5207:fragment_autocomplete_class_instance_dot_includes_method_from_outside`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_instance_dot_includes_method_from_outside() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function doThing(self)
    end
end
local bar = Bar.new { value = 1 }
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function doThing(self)
    end
end
local bar = Bar.new { value = 1 }
bar.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac_results = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac_results.entry_map.contains_key("value"));
        assert!(ac_results.entry_map.contains_key("doThing"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_instance_dot_property_from_outside {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5175:fragment_autocomplete_class_instance_dot_property_from_outside`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_instance_dot_property_from_outside() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
end
local bar = Bar.new { value = 1 }
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
end
local bar = Bar.new { value = 1 }
bar.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("value"));
        assert!(!ac.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_instance_multiple_props_from_outside {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5243:fragment_autocomplete_class_instance_multiple_props_from_outside`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_instance_multiple_props_from_outside() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Point
    public x: number
    public y: number
    public z: number
end
local p = Point.new { x = 0, y = 0, z = 0 }
"#,
    );

    let dest = String::from(
      r#"--!strict
class Point
    public x: number
    public y: number
    public z: number
end
local p = Point.new { x = 0, y = 0, z = 0 }
p.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("y"));
        assert!(ac.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_method_args_not_in_scope_outside_class {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1125:fragment_autocomplete_class_method_args_not_in_scope_outside_class`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_class_method_args_not_in_scope_outside_class

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_method_args_not_in_scope_outside_class() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
class Bar
    function method(self)
    end
end
local x = 4
"#,
      ),
      &Position {
        line: 6,
        column: 10,
      },
    );

    assert!(
      !result
        .local_map
        .iter()
        .any(|(name, _)| { unsafe { CStr::from_ptr(name.value) }.to_bytes() == b"self" })
    );
  }
}

mod fragment_autocomplete_class_method_extra_args_visible_in_body {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5104:fragment_autocomplete_class_method_extra_args_visible_in_body`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_method_extra_args_visible_in_body() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Counter
    public value: number
    function increment(self, count: number)
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Counter
    public value: number
    function increment(self, count: number)
        @1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("self"));
        assert!(ac.entry_map.contains_key("count"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_method_self_dot_autocomplete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5031:fragment_autocomplete_class_method_self_dot_autocomplete`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_method_self_dot_autocomplete() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function doThing(self)
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function doThing(self)
        self.@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("value"));
        assert!(!ac.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_method_self_dot_multiple_properties {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5065:fragment_autocomplete_class_method_self_dot_multiple_properties`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_method_self_dot_multiple_properties() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Vec3
    public x: number
    public y: number
    public z: number
    function length(self)
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Vec3
    public x: number
    public y: number
    public z: number
    function length(self)
        self.@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("y"));
        assert!(ac.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_method_self_in_local_stack {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1105:fragment_autocomplete_class_method_self_in_local_stack`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item fragment_autocomplete_class_method_self_in_local_stack

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_method_self_in_local_stack() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
class Bar
    public value: number
    function doThing(self)
    end
end
"#,
      ),
      &Position { line: 4, column: 2 },
    );

    assert_eq!(1, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    let last = *result.local_stack.last().unwrap();
    assert_eq!(
      "self",
      unsafe { CStr::from_ptr((*last).name.value) }
        .to_str()
        .unwrap()
    );
  }
}

mod fragment_autocomplete_class_second_method_self_dot {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5138:fragment_autocomplete_class_second_method_self_dot`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_second_method_self_dot() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function first(self)
    end
    function second(self)
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function first(self)
    end
    function second(self)
        self.@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("value"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_static_method_dot_autocomplete_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5280:fragment_autocomplete_class_static_method_dot_autocomplete_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_static_method_dot_autocomplete_1() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end

Bar.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("new")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_class_static_method_dot_autocomplete_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5316:fragment_autocomplete_class_static_method_dot_autocomplete_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_class_static_method_dot_autocomplete_2() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
      r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end
"#,
    );

    let dest = String::from(
      r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end

local _ = Bar.new()

Bar.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("new"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_correctly_grab_innermost_refinement {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4387:fragment_autocomplete_correctly_grab_innermost_refinement`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_correctly_grab_innermost_refinement() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
--!strict
type Type1 = { Type: "Type1", CommonKey: string, Type1Key: string }
type Type2 = { Type: "Type2", CommonKey: string, Type2Key: string }
type UnionType = Type1 | Type2

local foo: UnionType? = nil
if foo then
    if foo.Type == "Type2" then
    end
end
    "#,
    );

    let dest = String::from(
      r#"
--!strict
type Type1 = { Type: "Type1", CommonKey: string, Type1Key: string }
type Type2 = { Type: "Type2", CommonKey: string, Type2Key: string }
type UnionType = Type1 | Type2

local foo: UnionType? = nil
if foo then
    if foo.Type == "Type2" then
        foo.@1
    end
end
    "#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("Type2Key") as usize > 0);
        assert!(ac.entry_map.contains_key("Type1Key") as usize == 0);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1036:fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal, position::Position},
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
local x = 4
local y = 5
if x == 4 then
    local e = y
end
local z = x + x
if y == 5 then
    local q = x + y + z
end
"#,
      ),
      &Position {
        line: 8,
        column: 23,
      },
    );

    assert_eq!(6, result.ancestry.len());
    assert_eq!(3, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    assert!(!result.nearest_statement.is_null());
    let last = *result.local_stack.last().unwrap();
    assert_eq!(
      "z",
      unsafe { CStr::from_ptr((*last).name.value) }
        .to_str()
        .unwrap()
    );

    let local = unsafe { ast_node_as::<AstStatLocal>(result.nearest_statement as *mut AstNode) };
    assert!(!local.is_null());
    assert_eq!(1, unsafe { (*local).vars.size });
    let var = unsafe { *(*local).vars.data };
    assert_eq!(
      "q",
      unsafe { CStr::from_ptr((*var).name.value) }
        .to_str()
        .unwrap()
    );
  }
}

mod fragment_autocomplete_cursor_within_scope_tracks_locals_from_previous_scope {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1011:fragment_autocomplete_cursor_within_scope_tracks_locals_from_previous_scope`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_cursor_within_scope_tracks_locals_from_previous_scope

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_cursor_within_scope_tracks_locals_from_previous_scope() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal, position::Position},
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
local x = 4
local y = 5
if x == 4 then
    local e = y
end
"#,
      ),
      &Position {
        line: 4,
        column: 15,
      },
    );

    assert_eq!(5, result.ancestry.len());
    assert_eq!(2, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    assert!(!result.nearest_statement.is_null());
    let last = *result.local_stack.last().unwrap();
    assert_eq!(
      "y",
      unsafe { CStr::from_ptr((*last).name.value) }
        .to_str()
        .unwrap()
    );

    let local = unsafe { ast_node_as::<AstStatLocal>(result.nearest_statement as *mut AstNode) };
    assert!(!local.is_null());
    assert_eq!(1, unsafe { (*local).vars.size });
    let var = unsafe { *(*local).vars.data };
    assert_eq!(
      "e",
      unsafe { CStr::from_ptr((*var).name.value) }
        .to_str()
        .unwrap()
    );
  }
}

mod fragment_autocomplete_cyclic_table {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2314:fragment_autocomplete_cyclic_table`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_cyclic_table() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local abc = {}
        local def = { abc = abc }
        abc.def = def
        abc.def.@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("abc"));
        assert_eq!(ac.context, AutocompleteContext::Property);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_diff_multiple_blocks_on_same_line {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3626:fragment_autocomplete_diff_multiple_blocks_on_same_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_diff_multiple_blocks_on_same_line() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
do local function foo() end; local x = ""; end do local function bar() end"#,
    );
    let dest = String::from(
      r#"
do local function foo() end; local x = ""; end do local function bar() end local x = {a : number}; b @1end "#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|res: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, res.status);
        LUAU_ASSERT!(res.result.is_some());
        let ac = &res.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("bar"));
        assert!(!ac.entry_map.contains_key("foo"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_differ_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3361:fragment_autocomplete_differ_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_differ_1() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#""#);
    let dest = String::from(
      r#"local tbl = { foo = 1, bar = 2 };
tbl.b@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|status: &mut FragmentAutocompleteStatusResult| {
        assert!(FragmentAutocompleteStatus::Success == status.status);
        LUAU_ASSERT!(status.result.is_some());
        let ac = &status.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("foo"));
        assert!(ac.entry_map.contains_key("bar"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_do_not_recommend_results_in_multiline_comment {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2671:fragment_autocomplete_do_not_recommend_results_in_multiline_comment`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_do_not_recommend_results_in_multiline_comment() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"--[[
"#,
    );
    let dest = String::from(
      r#"--[[
a@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_dont_suggest_local_before_its_definition {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2099:fragment_autocomplete_dont_suggest_local_before_its_definition`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_dont_suggest_local_before_its_definition() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local myLocal = 4
        function abc()
@1             local myInnerLocal = 1
@2
        end
@3    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    // autocomplete after abc but before myInnerLocal
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let ac = &fragment.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("myLocal"));
        assert!(!ac.entry_map.contains_key("myInnerLocal"));
      }),
      None,
    );
    // autocomplete after my inner local
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '2',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let ac = &fragment.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("myLocal"));
        assert!(ac.entry_map.contains_key("myInnerLocal"));
      }),
      None,
    );

    // autocomplete after abc, but don't include myInnerLocal(in the hidden scope)
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '3',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let ac = &fragment.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("myLocal"));
        assert!(!ac.entry_map.contains_key("myInnerLocal"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_duped_alias {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2936:fragment_autocomplete_duped_alias`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_duped_alias() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::{
      fragment_autocomplete_status_result::FragmentAutocompleteStatusResult, scope::Scope,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type a = typeof({})

"#,
    );
    let dest = String::from(
      r#"
type a = typeof({})
type a = typeof({})@1
"#,
    );

    // Re-parsing and typechecking a type alias in the fragment that was defined in the base module will assert in ConstraintGenerator::checkAliases
    // unless we don't clone it This will let the incremental pass re-generate the type binding, and we will expect to see it in the type bindings
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let sc: *mut Scope = frag.result.as_ref().unwrap().fresh_scope;
        assert!(1 == unsafe { (*sc).private_type_bindings.contains_key("a") } as usize);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_empty_program {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2024:fragment_autocomplete_empty_program`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_empty_program() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      "",
      &String::from("@1"),
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("table"));
        assert!(ac.entry_map.contains_key("math"));
        assert_eq!(ac.context, AutocompleteContext::Statement);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_empty_program_1 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1149:fragment_autocomplete_empty_program_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_empty_program_1

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_empty_program_1() {
    use ulua_ast::records::position::Position;
    use ulua_common::FInt;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.check_with_options("");
    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);
    let fragment = fixture
      .base
      .parse_fragment(
        "",
        &Position {
          line: 0,
          column: 39,
        },
        None,
      )
      .expect("expected fragment parse result");

    assert_eq!("", fragment.fragment_to_parse);
  }
}

mod fragment_autocomplete_empty_program_2 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1158:fragment_autocomplete_empty_program_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_empty_program_2

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_empty_program_2() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FInt;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let mut fixture = FragmentAutocompleteFixture::default();
    let source = String::from(
      r#"

"#,
    );
    fixture.base.check_with_options(&source);
    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);
    let fragment = fixture
      .base
      .parse_fragment(
        &source,
        &Position {
          line: 1,
          column: 39,
        },
        None,
      )
      .expect("expected fragment parse result");

    assert_eq!("", fragment.fragment_to_parse);
  }
}

mod fragment_autocomplete_end_multiline_call {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:406:fragment_autocomplete_end_multiline_call`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_end_multiline_call

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_end_multiline_call() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_expr::AstStatExpr, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
abc(
"foo"
)
"#,
      ),
      &Position { line: 3, column: 1 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 3, column: 1 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(!region.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_end_of_do {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:453:fragment_autocomplete_end_of_do`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_end_of_do

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_end_of_do() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_block::AstStatBlock, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local x = 4
do
end
"#,
      ),
      &Position { line: 3, column: 3 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 3 },
        end: Position { line: 3, column: 3 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatBlock>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_expr_function {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3317:fragment_autocomplete_expr_function`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_expr_function() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local t = {}
type Input = {x : string}
function t.Do(fn : (Input) -> ())
    if t.x == "a" then
        return
    end
end

t.Do(function (f)
    f
end)
"#,
    );

    let dest = String::from(
      r#"
local t = {}
type Input = {x : string}
function t.Do(fn : (Input) -> ())
    if t.x == "a" then
        return
    end
end

t.Do(function (f)
    f.@1
end)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|status: &mut FragmentAutocompleteStatusResult| {
        assert!(FragmentAutocompleteStatus::Success == status.status);
        LUAU_ASSERT!(status.result.is_some());
        assert!(
          !status
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
        assert!(
          status
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("x")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_expr_in_should_rec_no_do {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4275:fragment_autocomplete_for_expr_in_should_rec_no_do`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_expr_in_should_rec_no_do() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i =
end
"#,
    );
    let dest = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("x"));
        assert!(ac_results.entry_map.contains_key("y"));
        assert!(ac_results.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_add {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4359:fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_add`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_add() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i = x.y do
end
"#,
    );
    let dest = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.y, x.@1 do
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("y"));
        assert!(ac.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_delete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4331:fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_delete`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_delete() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i = x.y, x.z do
end
"#,
    );
    let dest = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.y, x.@1 do
end
"#,
    );
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(
          !result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
        assert!(
          result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("x")
        );
        assert!(
          result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("y")
        );
        assert!(
          result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("z")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_expr_in_should_rec_with_do_in_step {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4303:fragment_autocomplete_for_expr_in_should_rec_with_do_in_step`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_expr_in_should_rec_with_do_in_step() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i = x.y, 100 do
end
"#,
    );
    let dest = String::from(
      r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.y, 100, x.@1 do
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("x"));
        assert!(ac_results.entry_map.contains_key("y"));
        assert!(ac_results.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_in_should_rec {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4254:fragment_autocomplete_for_in_should_rec`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_in_should_rec() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type T = { x : {[number] : number}, y: number}
local x : T = ({} :: T)
for _,n in pairs(x.@1) do
end
"#,
    );
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(
          !result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
        assert!(
          result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("x")
        );
        assert!(
          result
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("y")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_loop_recommends_fragment_autocomplete_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3239:fragment_autocomplete_for_loop_recommends`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_loop_recommends() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local testArr: {{a: number, b: number}} = {
{a = 1, b = 2},
{a = 2, b = 4},
}

for _, v in testArr do

end
"#,
    );

    let dest = String::from(
      r#"
local testArr: {{a: number, b: number}} = {
{a = 1, b = 2},
{a = 2, b = 4},
}

for _, v in testArr do
    print(v.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(result.status != FragmentAutocompleteStatus::InternalIce);
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("a"));
        assert!(ac.entry_map.contains_key("b"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_for_loop_recommends_fragment_autocomplete_test_alt_b {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3278:fragment_autocomplete_for_loop_recommends`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_for_loop_recommends() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local testArr: {string} = {
"a",
"b",
}

for _, v in testArr do

end
"#,
    );

    let dest = String::from(
      r#"
local testArr: {string} = {
"a",
"b",
}

for _, v in testArr do
    print(v:@1)
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(result.status != FragmentAutocompleteStatus::InternalIce);
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("upper"));
        assert!(ac.entry_map.contains_key("sub"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_ac_must_traverse_typeof_and_not_ice {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2909:fragment_autocomplete_fragment_ac_must_traverse_typeof_and_not_ice`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_ac_must_traverse_typeof_and_not_ice() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    // This test ensures that we traverse typeof expressions for defs that are being referred to in the fragment
    // In this case, we want to ensure we populate the incremental environment with the reference to `m`
    // Without this, we would ice as we will refer to the local `m` before it's declaration
    let source = String::from(
      r#"
--!strict
local m = {}
-- and here
function m:m1() end
type nt = typeof(m)

return m
"#,
    );
    let updated = String::from(
      r#"
--!strict
local m = {}
-- and here
function m:m1() end
type nt = typeof(m)
l @1
return m
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_ensures_memory_isolation {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3020:fragment_autocomplete_fragment_autocomplete_ensures_memory_isolation`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_ensures_memory_isolation() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
      records::{scope::Scope, to_string_options::ToStringOptions, type_arena::TypeArena},
      type_aliases::{module_ptr_module::ModulePtr, type_id::TypeId},
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::lookup_name::lookup_name,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let opt = ToStringOptions {
      exhaustive: true,
      function_type_arguments: true,
      max_table_length: 0,
      max_type_length: 0,
      ..ToStringOptions::default()
    };

    fn get_type_from_module(module: &ModulePtr, name: &str) -> Option<TypeId> {
      if !module.has_module_scope() {
        return None;
      }
      let scope = module.get_module_scope();
      unsafe { lookup_name(Arc::as_ptr(&scope) as *mut Scope, &String::from(name)) }
    }

    let source = String::from(
      r#"local module = {}
f
return module"#,
    );

    let updated1 = String::from(
      r#"local module = {}
function module.a
return module"#,
    );

    let updated2 = String::from(
      r#"local module = {}
function module.ab
return module"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    let check_and_examine =
      |fixture: &mut FragmentAutocompleteFixture, src: &str, id_name: &str, id_string: &str| {
        fixture.base.check_with_options(src);
        let id = fixture
          .base
          .base
          .base
          .get_type(&String::from(id_name), true);
        LUAU_ASSERT!(id.is_some());
        assert_eq!(
          to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
          String::from(id_string)
        );
      };

    let fragment_ac_and_check =
      |fixture: &mut FragmentAutocompleteFixture, updated: &str, pos: Position, id_name: &str| {
        let frag = fixture.base.autocomplete_fragment(updated, pos, None);
        LUAU_ASSERT!(frag.result.is_some());
        let frag_id =
          get_type_from_module(&frag.result.as_ref().unwrap().incremental_module, id_name);
        LUAU_ASSERT!(frag_id.is_some());

        let src_id = fixture
          .base
          .base
          .base
          .get_type(&String::from(id_name), true);
        LUAU_ASSERT!(src_id.is_some());

        let frag_id = frag_id.unwrap();
        let src_id = src_id.unwrap();
        unsafe {
          assert!((*frag_id).owning_arena != (*src_id).owning_arena);
          let internal_types_ptr = &frag
            .result
            .as_ref()
            .unwrap()
            .incremental_module
            .internal_types as *const TypeArena
            as *mut TypeArena;
          assert!(internal_types_ptr == (*frag_id).owning_arena);
        }
      };

    {
      let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
      fixture
        .base
        .base
        .get_frontend()
        .set_luau_solver_mode(SolverMode::Old);
      check_and_examine(&mut fixture, &source, "module", "{|  |}");
      // [TODO] CLI-140762 we shouldn't mutate stale module in autocompleteFragment
      // early return since the following checking will fail, which it shouldn't!
      fragment_ac_and_check(
        &mut fixture,
        &updated1,
        Position {
          line: 1,
          column: 17,
        },
        "module",
      );
      fragment_ac_and_check(
        &mut fixture,
        &updated2,
        Position {
          line: 1,
          column: 18,
        },
        "module",
      );
    }

    {
      let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
      fixture
        .base
        .base
        .get_frontend()
        .set_luau_solver_mode(SolverMode::New);
      check_and_examine(&mut fixture, &source, "module", "{  }");
      // [TODO] CLI-140762 we shouldn't mutate stale module in autocompleteFragment
      // early return since the following checking will fail, which it shouldn't!
      fragment_ac_and_check(
        &mut fixture,
        &updated1,
        Position {
          line: 1,
          column: 17,
        },
        "module",
      );
      fragment_ac_and_check(
        &mut fixture,
        &updated2,
        Position {
          line: 1,
          column: 18,
        },
        "module",
      );
    }
  }
}

mod fragment_autocomplete_fragment_autocomplete_handles_parse_errors {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2862:fragment_autocomplete_fragment_autocomplete_handles_parse_errors`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_handles_parse_errors() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FInt, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);
    let source = String::from(
      r#"

"#,
    );
    let updated = String::from(
      r#"
type A = <>random non code text here  @1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_react_narrow_fragment {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4967:fragment_autocomplete_fragment_autocomplete_react_narrow_fragment`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_react_narrow_fragment() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let src = String::from(
      "
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        \treturn nil
        end

        createElement(MyComponent, { })
    ",
    );

    let dest = String::from(
      "
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        \treturn nil
        end

        createElement(MyComponent, { f@1 })
    ",
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &src,
      &dest,
      '1',
      Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
        assert!(ac.result.is_some());
        let ac_results = &ac.result.as_ref().unwrap().ac_results;
        assert!(ac_results.entry_map.contains_key("foobar"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_react_properties {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4882:fragment_autocomplete_fragment_autocomplete_react_properties`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_react_properties() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let src = String::from(
      r#"
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        	return nil
        end

    "#,
    );

    let dest = String::from(
      r#"
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        	return nil
        end

        createElement(MyComponent, { f@1 })
        createElement(MyComponent, { barbaz = { b@2 } })
        createElement(MyComponent, { foobar = {}, b@3 })
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &src,
      &dest,
      '1',
      Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(ac.result.is_some());
        assert!(
          (ac
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("foobar") as usize)
            > 0
        );
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &src,
      &dest,
      '2',
      Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(ac.result.is_some());
        assert!(
          (ac
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("bazquxx") as usize)
            > 0
        );
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &src,
      &dest,
      '3',
      Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(ac.result.is_some());
        assert!(
          (ac
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("barbaz") as usize)
            > 0
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_shouldnt_crash_on_cross_module_mutation {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3091:fragment_autocomplete_fragment_autocomplete_shouldnt_crash_on_cross_module_mutation`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_shouldnt_crash_on_cross_module_mutation() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"local module = {}
function module.
return module
"#,
    );

    let updated = String::from(
      r#"local module = {}
function module.f@1
return module
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|_result: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_param {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4803:fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_param`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_param() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);

    let source = String::from(
      r#"
        local function C(_: "Example"&"Example") end
    "#,
    );

    let dest = String::from(
      r#"
        local function C(_: "Example"&"Example") end
        C(@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("\"Example\"")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_variable_annotation {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4830:fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_variable_annotation`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_variable_annotation()
  {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);

    let source = String::from(
      r#"
        local _: "foo"&"foo"
    "#,
    );
    let dest = String::from(
      r#"
        local _: "foo"&"foo" = "@1"
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac_results = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac_results.entry_map.contains_key("foo"));
        assert_eq!(AutocompleteContext::String, ac_results.context);
      }),
      Some(Position {
        line: 1,
        column: 33,
      }),
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_table_insert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4856:fragment_autocomplete_fragment_autocomplete_table_insert`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_table_insert() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let src = String::from(
      r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, {})
        end
    "#,
    );

    let dest = String::from(
      r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, { f@1 })
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &src,
      &dest,
      '1',
      Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(ac.result.is_some());
        assert!(
          (ac
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("foobar") as usize)
            > 0
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_using_function_call_with_variadic_args {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4779:fragment_autocomplete_fragment_autocomplete_using_function_call_with_variadic_args`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_using_function_call_with_variadic_args() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local function foo(...: "Val1" | "Val2") end
    "#,
    );

    let dest = String::from(
      r#"
        local function foo(...: "Val1" | "Val2") end
        foo(@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("\"Val1\"") as usize
            == 1
        );
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("\"Val2\"") as usize
            == 1
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_fragment_autocomplete_using_indexer_with_singleton_keys {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4752:fragment_autocomplete_fragment_autocomplete_using_indexer_with_singleton_keys`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_fragment_autocomplete_using_indexer_with_singleton_keys() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
    "#,
    );

    let dest = String::from(
      r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("Val1"));
        assert!(ac.entry_map.contains_key("Val2"));
        assert!(ac.entry_map.contains_key("Val3"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_free_type_in_old_solver_shouldnt_trigger_not_null_assertion {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3123:fragment_autocomplete_free_type_in_old_solver_shouldnt_trigger_not_null_assertion`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_free_type_in_old_solver_shouldnt_trigger_not_null_assertion() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"--!strict
local foo
local a, z = foo()

local e = foo().x

local f = foo().y

z
"#,
    );

    let dest = String::from(
      r#"--!strict
local foo
local a, z = foo()

local e = foo().x

local f = foo().y

z:a@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_function_parameter_not_recommending_out_of_scope_argument {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2573:fragment_autocomplete_function_parameter_not_recommending_out_of_scope_argument`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_function_parameter_not_recommending_out_of_scope_argument() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
--!strict
local function foo(abd: FakeVec)
end
local function bar(abc : FakeVec)
   a@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("abc")
        );
        assert!(
          !frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("abd")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_function_parameters {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2246:fragment_autocomplete_function_parameters`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_function_parameters() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        function abc(test)

@1        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("test"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_generalization_crash_when_old_solver_freetypes_have_no_bounds_set {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2989:fragment_autocomplete_generalization_crash_when_old_solver_freetypes_have_no_bounds_set`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_generalization_crash_when_old_solver_freetypes_have_no_bounds_set() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local UserInputService = game:GetService("UserInputService");

local Camera = workspace.CurrentCamera;

UserInputService.InputBegan:Connect(function(Input)
    if (Input.KeyCode == Enum.KeyCode.One) then
        local Up = Input.Foo
        local Vector = -(Up:Unit)
    end
end)
"#,
    );

    let dest = String::from(
      r#"
local UserInputService = game:GetService("UserInputService");

local Camera = workspace.CurrentCamera;

UserInputService.InputBegan:Connect(function(Input)
    if (Input.KeyCode == Enum.KeyCode.One) then
        local Up = Input.Foo
        local Vector = -(Up:Unit()) @1
    end
end)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|_frag: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_get_suggestions_for_the_very_start_of_the_script {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2403:fragment_autocomplete_get_suggestions_for_the_very_start_of_the_script`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_get_suggestions_for_the_very_start_of_the_script() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"@1

        function aaa() end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("table"));
        assert_eq!(ac.context, AutocompleteContext::Statement);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_global_functions_are_not_scoped_lexically {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2197:fragment_autocomplete_global_functions_are_not_scoped_lexically`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_global_functions_are_not_scoped_lexically() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        if true then
            function abc()

            end
        end
@1      "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("abc"));
        assert!(ac.entry_map.contains_key("table"));
        assert!(ac.entry_map.contains_key("math"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_hot_comment_should_rec {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4429:fragment_autocomplete_hot_comment_should_rec`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_hot_comment_should_rec() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#"--!@1"#);

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("strict"));
        assert!(ac.entry_map.contains_key("nonstrict"));
        assert!(ac.entry_map.contains_key("nocheck"));
        assert!(ac.entry_map.contains_key("native"));
        assert!(ac.entry_map.contains_key("nolint"));
        assert!(ac.entry_map.contains_key("optimize"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_ice_caused_by_mixed_mode_use {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3106:fragment_autocomplete_ice_caused_by_mixed_mode_use`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_ice_caused_by_mixed_mode_use() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    // C++ builds this source by concatenating string literals with explicit
    // escape sequences (`\n`, `\t`). Transcribed below with a literal tab and
    // newlines so the bytes match exactly.
    let source = String::from(
      "--[[\n\tPackage link auto-generated by Rotriever\n]]\nlocal PackageIndex = script.Parent._Index\n\nlocal Package = ",
    ) + "require(PackageIndex[\"ReactOtter\"][\"ReactOtter\"])\n\nexport type Goal = Package.Goal\nexport type SpringOptions "
      + "= Package.SpringOptions\n\n\nreturn Pa@1";

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_if_complete_inside_scope_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:905:fragment_autocomplete_if_complete_inside_scope_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_complete_inside_scope_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_complete_inside_scope_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local::AstStatLocal, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then
    local x =
end

"#,
      ),
      &Position {
        line: 2,
        column: 13,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 4 },
        end: Position {
          line: 2,
          column: 13
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocal>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_cond_no_then_recs_then {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3743:fragment_autocomplete_if_cond_no_then_recs_then`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_cond_no_then_recs_then() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"

    "#,
    );

    let dest = String::from(
      r#"
if x t@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(result.result.is_some());
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(ac_results.entry_map.contains_key("then"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_if_else_if {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:921:fragment_autocomplete_if_else_if`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_else_if

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_else_if() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then
elseif
end

"#,
      ),
      &Position { line: 2, column: 8 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 8 },
        end: Position { line: 2, column: 8 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_else_if_after_then {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:951:fragment_autocomplete_if_else_if_after_then`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_else_if_after_then

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_else_if_after_then() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then
elseif false then
end

"#,
      ),
      &Position {
        line: 2,
        column: 17,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 17
        },
        end: Position {
          line: 2,
          column: 17
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_else_if_after_then_new_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:967:fragment_autocomplete_if_else_if_after_then_new_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_else_if_after_then_new_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_else_if_after_then_new_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then
elseif false then

end

"#,
      ),
      &Position { line: 3, column: 0 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 0 },
        end: Position { line: 3, column: 0 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_else_if_no_end {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:937:fragment_autocomplete_if_else_if_no_end`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_else_if_no_end

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_else_if_no_end() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then
elseif
"#,
      ),
      &Position { line: 2, column: 8 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 8 },
        end: Position { line: 2, column: 8 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_else_if_table_prop_recs_no_then {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3792:fragment_autocomplete_if_else_if_table_prop_recs_no_then`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_else_if_table_prop_recs_no_then() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif
end
"#,
    );
    let dest = String::from(
      r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif t.xa t@1
end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(result.result.is_some());
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(!ac_results.entry_map.contains_key("xa"));
        assert!(!ac_results.entry_map.contains_key("y"));
        assert!(ac_results.entry_map.contains_key("then"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_if_else_if_table_prop_recs_with_then {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3827:fragment_autocomplete_if_else_if_table_prop_recs_with_then`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_else_if_table_prop_recs_with_then() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif  then
end
"#,
    );

    let dest = String::from(
      r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif t.@1  then
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("xa"));
        assert!(ac.entry_map.contains_key("y"));
        assert!(!ac.entry_map.contains_key("then"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_if_partial {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:840:fragment_autocomplete_if_partial`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_partial

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_partial() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if"#,
      ),
      &Position { line: 1, column: 2 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 2 },
        end: Position { line: 1, column: 2 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_partial_after_condition {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:878:fragment_autocomplete_if_partial_after_condition`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_partial_after_condition

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_partial_after_condition() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then
"#,
      ),
      &Position {
        line: 1,
        column: 12,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 12
        },
        end: Position {
          line: 1,
          column: 12
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_partial_in_condition_after {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:865:fragment_autocomplete_if_partial_in_condition_after`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_partial_in_condition_after

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_partial_in_condition_after() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true
"#,
      ),
      &Position { line: 1, column: 8 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 3 },
        end: Position { line: 1, column: 8 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_partial_in_condition_at {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:852:fragment_autocomplete_if_partial_in_condition_at`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_partial_in_condition_at

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_partial_in_condition_at() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true
"#,
      ),
      &Position { line: 1, column: 7 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 3 },
        end: Position { line: 1, column: 7 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_partial_new_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:891:fragment_autocomplete_if_partial_new_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_if_partial_new_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_partial_new_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_if::AstStatIf, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
if true then

"#,
      ),
      &Position { line: 2, column: 0 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 0 },
        end: Position { line: 2, column: 0 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatIf>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_if_then_recs_else {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3765:fragment_autocomplete_if_then_recs_else`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_if_then_recs_else() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
if x then

end
    "#,
    );

    let dest = String::from(
      r#"
if x then
e@1
end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("else"));
        assert!(ac.entry_map.contains_key("elseif"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_in_place_edit_of_for_loop_before_in_keyword_returns_fragment_starting_from_for {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4564:fragment_autocomplete_in_place_edit_of_for_loop_before_in_keyword_returns_fragment_starting_from_for`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_in_place_edit_of_for_loop_before_in_keyword_returns_fragment_starting_from_for()
   {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local x = {}
for i, value in x do
    print(i)
end
"#,
    );

    let dest = String::from(
      r#"
local x = {}
for @1, value in x do
    print(i)
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_inline_autocomplete_picks_the_right_scope_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1915:fragment_autocomplete_inline_autocomplete_picks_the_right_scope_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_inline_autocomplete_picks_the_right_scope_1() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{
        fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
        table_type::TableType,
      },
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
"#,
    );

    let updated = String::from(
      r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
    local a : T@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let result = fragment.result.as_ref().unwrap();
        LUAU_ASSERT!(!result.fresh_scope.is_null());
        LUAU_ASSERT!(result.ac_results.entry_map.contains_key("Table"));
        LUAU_ASSERT!(result.ac_results.entry_map["Table"].r#type.is_some());
        let ty = follow_type_id(result.ac_results.entry_map["Table"].r#type.unwrap());
        let tv = get_type_id::<TableType>(ty).expect("Table should be TableType");
        assert!(tv.props.contains_key("x"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_inline_autocomplete_picks_the_right_scope_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1949:fragment_autocomplete_inline_autocomplete_picks_the_right_scope_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_inline_autocomplete_picks_the_right_scope_2() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{
        fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
        table_type::TableType,
      },
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
"#,
    );

    let updated = String::from(
      r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
local a : T@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        assert!(fragment.result.is_some());
        let result = fragment.result.as_ref().unwrap();
        LUAU_ASSERT!(!result.fresh_scope.is_null());
        assert!(result.ac_results.entry_map.contains_key("Table"));
        assert!(result.ac_results.entry_map["Table"].r#type.is_some());
        let ty = follow_type_id(result.ac_results.entry_map["Table"].r#type.unwrap());
        let tv = get_type_id::<TableType>(ty).expect("Table should be TableType");
        assert!(tv.props.contains_key("a"));
        assert!(tv.props.contains_key("b"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_inline_prop_read_on_requires_provides_results {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4017:fragment_autocomplete_inline_prop_read_on_requires_provides_results`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_inline_prop_read_on_requires_provides_results() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::get_options::get_options,
      records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
    };

    let module_a = String::from(
      r#"
local mod = { prop1 = true}
mod.prop2 = "a"
function mod.foo(a: number)
    return a
end
return mod
"#,
    );

    let main_module = String::from(
      r#"

"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture
      .base
      .base
      .base
      .file_resolver
      .source
      .insert(String::from("MainModule"), main_module);
    fixture
      .base
      .base
      .base
      .file_resolver
      .source
      .insert(String::from("MainModule/A"), module_a);
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(
        &String::from("MainModule/A"),
        Some(get_options()),
      );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(
        &String::from("MainModule"),
        Some(get_options()),
      );

    let updated_main = String::from(
      r#"
require(script.A).
"#,
    );

    let result = fixture.base.autocomplete_fragment(
      &updated_main,
      Position {
        line: 1,
        column: 18,
      },
      None,
    );
    let ac_results = &result.result.as_ref().unwrap().ac_results;
    assert!(!ac_results.entry_map.is_empty());
    assert!(ac_results.entry_map.contains_key("prop1"));
    assert!(ac_results.entry_map.contains_key("prop2"));
    assert!(ac_results.entry_map.contains_key("foo"));
  }
}

mod fragment_autocomplete_inside_do {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:469:fragment_autocomplete_inside_do`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_inside_do

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_inside_do() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_block::AstStatBlock, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local x = 4
do

end
"#,
      ),
      &Position { line: 3, column: 3 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 3 },
        end: Position { line: 3, column: 3 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatBlock>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_inside_incomplete_do {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:438:fragment_autocomplete_inside_incomplete_do`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_inside_incomplete_do

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_inside_incomplete_do() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_block::AstStatBlock, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local x = 4
do
"#,
      ),
      &Position { line: 2, column: 2 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 2 },
        end: Position { line: 2, column: 2 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatBlock>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_interior_free_types_assertion_caused_by_free_type_inheriting_null_scope_from_table {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3151:fragment_autocomplete_interior_free_types_assertion_caused_by_free_type_inheriting_null_scope_from_table`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_interior_free_types_assertion_caused_by_free_type_inheriting_null_scope_from_table()
   {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y


"#,
    );

    let dest = String::from(
      r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y

z = a.P.E@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|_: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_isinstance_refines_for_autocomplete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5423:fragment_autocomplete_isinstance_refines_for_autocomplete`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_isinstance_refines_for_autocomplete() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff0 = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _sff1 = ScopedFastFlag::new(&FFlag::LuauAllowGlobalDeclarationToBeCalledClass, true);

    let source = String::from(
      r#"
class Point
    public x
    public y
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then

    end
end
"#,
    );

    let dest = String::from(
      r#"
class Point
    public x
    public y
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then
        v.@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("y"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_just_two_locals_fragment_autocomplete_test {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:358:fragment_autocomplete_just_two_locals`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_just_two_locals

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_just_two_locals() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local::AstStatLocal, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local x = 4
local y = 5
"#,
      ),
      &Position {
        line: 2,
        column: 11,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 2, column: 0 },
        end: Position {
          line: 2,
          column: 11
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(!region.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocal>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_just_two_locals_fragment_autocomplete_test_alt_b {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:990:fragment_autocomplete_just_two_locals`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item fragment_autocomplete_just_two_locals

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_just_two_locals() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal, position::Position},
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
local x = 4
local y = 5
"#,
      ),
      &Position {
        line: 2,
        column: 11,
      },
    );

    assert_eq!(3, result.ancestry.len());
    assert_eq!(1, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    assert!(!result.nearest_statement.is_null());

    let local = unsafe { ast_node_as::<AstStatLocal>(result.nearest_statement as *mut AstNode) };
    assert!(!local.is_null());
    assert_eq!(1, unsafe { (*local).vars.size });
    let var = unsafe { *(*local).vars.data };
    assert_eq!(
      "y",
      unsafe { CStr::from_ptr((*var).name.value) }
        .to_str()
        .unwrap()
    );
  }
}

mod fragment_autocomplete_leave_numbers_alone {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2060:fragment_autocomplete_leave_numbers_alone`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_leave_numbers_alone() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from("local a = 3.@1");

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.is_empty());
        assert_eq!(ac.context, AutocompleteContext::Unknown);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_len_operator_needs_to_provide_autocomplete_results {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4449:fragment_autocomplete_len_operator_needs_to_provide_autocomplete_results`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_len_operator_needs_to_provide_autocomplete_results() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type Pool = { numbers: { number }}

local function foobar(p)
    local pool = p :: Pool
    if #pool
end
"#,
    );
    let dest = String::from(
      r#"
type Pool = { numbers: { number }}

local function foobar(p)
    local pool = p :: Pool
    if #pool.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("numbers"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_local_funcs_show_up_in_local_stack {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1086:fragment_autocomplete_local_funcs_show_up_in_local_stack`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstStatReturn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_local_funcs_show_up_in_local_stack

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_local_funcs_show_up_in_local_stack() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_return::AstStatReturn, position::Position},
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
local function foo() return 4 end
local x = foo()
local function bar() return x + foo() end
"#,
      ),
      &Position {
        line: 3,
        column: 32,
      },
    );

    assert_eq!(8, result.ancestry.len());
    assert_eq!(3, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    let last = *result.local_stack.last().unwrap();
    assert_eq!(
      "bar",
      unsafe { CStr::from_ptr((*last).name.value) }
        .to_str()
        .unwrap()
    );
    let return_stat =
      unsafe { ast_node_as::<AstStatReturn>(result.nearest_statement as *mut AstNode) };
    assert!(!return_stat.is_null());
  }
}

mod fragment_autocomplete_local_functions_fall_out_of_scope {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2222:fragment_autocomplete_local_functions_fall_out_of_scope`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_local_functions_fall_out_of_scope() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        if true then
            local function abc()

            end
        end
@1      "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_ne!(0, ac.entry_map.len());
        assert!(!ac.entry_map.contains_key("abc"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_local_initializer_fragment_autocomplete_test {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1179:fragment_autocomplete_local_initializer`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_local_initializer

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_local_initializer() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.check_with_options(&String::from("local a ="));
    let fragment = fixture
      .base
      .parse_fragment(
        &String::from("local a ="),
        &Position { line: 0, column: 9 },
        None,
      )
      .expect("expected fragment parse result");

    assert_eq!("local a =", fragment.fragment_to_parse);
    assert_eq!(
      Location {
        begin: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 9 },
      },
      unsafe { (*fragment.root).base.base.location }
    );
  }
}

mod fragment_autocomplete_local_initializer_fragment_autocomplete_test_alt_b {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2041:fragment_autocomplete_local_initializer`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_local_initializer() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from("local a =@1");
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = frag.result.as_ref().unwrap().ac_results.clone();

        assert!(ac.entry_map.contains_key("table"));
        assert!(ac.entry_map.contains_key("math"));
        assert_eq!(ac.context, AutocompleteContext::Expression);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_method_call_inside_function_body {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2445:fragment_autocomplete_method_call_inside_function_body`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_method_call_inside_function_body() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()

        end
    "#,
    );

    let updated = String::from(
      r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()
            game:@1
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_ne!(0, ac.entry_map.len());

        assert!(!ac.entry_map.contains_key("math"));
        assert_eq!(ac.context, AutocompleteContext::Property);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_method_in_unfinished_repeat_body_eof {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4509:fragment_autocomplete_method_in_unfinished_repeat_body_eof`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_method_in_unfinished_repeat_body_eof() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local t = {}
function t:Foo() end
repeat"#,
    );

    let dest = String::from(
      r#"
local t = {}
function t:Foo() end
repeat
t:@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("Foo"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_method_in_unfinished_repeat_body_not_eof {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4533:fragment_autocomplete_method_in_unfinished_repeat_body_not_eof`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_method_in_unfinished_repeat_body_not_eof() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local t = {}
function t:Foo() end
repeat
t

local function whatever() end
"#,
    );

    let dest = String::from(
      r#"
local t = {}
function t:Foo() end
repeat
t:@1

local function whatever() end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("Foo"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_midway_multiline_call {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:389:fragment_autocomplete_midway_multiline_call`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_midway_multiline_call

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_midway_multiline_call() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_expr::AstStatExpr, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
abc(
"foo"
)
"#,
      ),
      &Position { line: 2, column: 4 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 2, column: 4 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(!region.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_midway_through_call {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:423:fragment_autocomplete_midway_through_call`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_midway_through_call

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_midway_through_call() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_expr::AstStatExpr, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
abc("foo")
"#,
      ),
      &Position { line: 1, column: 6 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 6 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(!region.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_mixed_mode_basic_example_append {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1529:fragment_autocomplete_mixed_mode_basic_example_append`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_mixed_mode_basic_example_append() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      enums::solver_mode::SolverMode, functions::to_string_to_string_alt_c::to_string_type_id,
      records::scope::Scope,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::linear_search_for_binding::linear_search_for_binding,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.base.get_frontend().set_luau_solver_mode(
      if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      },
    );
    let res = fixture.base.check_old_solver(&String::from(
      r#"
local x = 4
local y = 5
"#,
    ));

    assert_eq!(0, res.errors.len(), "{:?}", res.errors);

    let fragment = fixture.base.check_fragment(
      &String::from(
        r#"
local x = 4
local y = 5
local z = x + y
"#,
      ),
      Position {
        line: 3,
        column: 15,
      },
      None,
    );

    let scope_ptr = Arc::as_ptr(&fragment.fresh_scope) as *mut Scope;
    let opt = unsafe { linear_search_for_binding(scope_ptr, "z") };
    LUAU_ASSERT!(opt.is_some());
    assert_eq!("number", to_string_type_id(opt.unwrap()));
  }
}

mod fragment_autocomplete_mixed_mode_basic_example_inlined {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1556:fragment_autocomplete_mixed_mode_basic_example_inlined`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_mixed_mode_basic_example_inlined() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      enums::solver_mode::SolverMode, functions::to_string_to_string_alt_c::to_string_type_id,
      records::scope::Scope,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::linear_search_for_binding::linear_search_for_binding,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.base.get_frontend().set_luau_solver_mode(
      if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      },
    );
    let _res = fixture.base.check_old_solver(&String::from(
      r#"
local x = 4
local y = 5
"#,
    ));

    let fragment = fixture.base.check_fragment(
      &String::from(
        r#"
local x = 4
local z = x
local y = 5
"#,
      ),
      Position {
        line: 2,
        column: 11,
      },
      None,
    );

    let scope_ptr = Arc::as_ptr(&fragment.fresh_scope) as *mut Scope;
    let correct = unsafe { linear_search_for_binding(scope_ptr, "z") };
    LUAU_ASSERT!(correct.is_some());
    assert_eq!("number", to_string_type_id(correct.unwrap()));
  }
}

mod fragment_autocomplete_mixed_mode_can_autocomplete_simple_property_access {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1581:fragment_autocomplete_mixed_mode_can_autocomplete_simple_property_access`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_mixed_mode_can_autocomplete_simple_property_access() {
    use alloc::string::String;

    use ulua_analysis::enums::{
      autocomplete_context::AutocompleteContext, solver_mode::SolverMode,
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.base.get_frontend().set_luau_solver_mode(
      if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      },
    );
    let res = fixture.base.check_old_solver(&String::from(
      r#"
local tbl = { abc = 1234}
"#,
    ));

    assert_eq!(0, res.errors.len(), "{:?}", res.errors);

    let fragment = fixture.base.autocomplete_fragment(
      &String::from(
        r#"
local tbl = { abc = 1234}
tbl.
"#,
      ),
      Position { line: 2, column: 5 },
      None,
    );
    LUAU_ASSERT!(fragment.result.is_some());
    let result = fragment.result.as_ref().unwrap();
    LUAU_ASSERT!(!result.fresh_scope.is_null());

    assert_eq!(1, result.ac_results.entry_map.len());
    assert!(result.ac_results.entry_map.contains_key("abc"));
    assert_eq!(AutocompleteContext::Property, result.ac_results.context);
  }
}

mod fragment_autocomplete_multiple_fragment_autocomplete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1643:fragment_autocomplete_multiple_fragment_autocomplete`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_multiple_fragment_autocomplete() {
    use alloc::{string::String, sync::Arc};

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
      records::{scope::Scope, to_string_options::ToStringOptions},
      type_aliases::{module_ptr_module::ModulePtr, type_id::TypeId},
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::lookup_name::lookup_name,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let opt = ToStringOptions {
      exhaustive: true,
      function_type_arguments: true,
      max_table_length: 0,
      max_type_length: 0,
      ..Default::default()
    };

    fn get_type_from_module(module: &ModulePtr, name: &str) -> Option<TypeId> {
      if !module.has_module_scope() {
        return None;
      }
      let scope = module.get_module_scope();
      unsafe { lookup_name(Arc::as_ptr(&scope) as *mut Scope, &String::from(name)) }
    }

    let source = String::from(
      r#"local module = {}
f
return module"#,
    );

    let updated1 = String::from(
      r#"local module = {}
function module.a
return module"#,
    );

    let updated2 = String::from(
      r#"local module = {}
function module.ab
return module"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    {
      let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
      fixture
        .base
        .base
        .get_frontend()
        .set_luau_solver_mode(SolverMode::Old);

      // checkAndExamine(source, "module", "{|  |}")
      fixture.base.check_with_options(&source);
      let id = fixture
        .base
        .base
        .base
        .get_type(&String::from("module"), true);
      LUAU_ASSERT!(id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
        String::from("{|  |}")
      );

      // fragmentACAndCheck(updated1, Position{1, 17}, "module", "{|  |}", "{| a: (%error-id%: unknown) -> () |}")
      {
        let frag = fixture.base.autocomplete_fragment(
          &updated1,
          Position {
            line: 1,
            column: 17,
          },
          None,
        );
        LUAU_ASSERT!(frag.result.is_some());
        let frag_id =
          get_type_from_module(&frag.result.as_ref().unwrap().incremental_module, "module");
        LUAU_ASSERT!(frag_id.is_some());
        assert_eq!(
          to_string_type_id_to_string_options_mut(frag_id.unwrap(), opt.clone()),
          String::from("{| a: (%error-id%: unknown) -> () |}")
        );

        let src_id = fixture
          .base
          .base
          .base
          .get_type(&String::from("module"), true);
        LUAU_ASSERT!(src_id.is_some());
        assert_eq!(
          to_string_type_id_to_string_options_mut(src_id.unwrap(), opt.clone()),
          String::from("{|  |}")
        );
      }

      // fragmentACAndCheck(updated2, Position{1, 18}, "module", "{|  |}", "{| ab: (%error-id%: unknown) -> () |}")
      {
        let frag = fixture.base.autocomplete_fragment(
          &updated2,
          Position {
            line: 1,
            column: 18,
          },
          None,
        );
        LUAU_ASSERT!(frag.result.is_some());
        let frag_id =
          get_type_from_module(&frag.result.as_ref().unwrap().incremental_module, "module");
        LUAU_ASSERT!(frag_id.is_some());
        assert_eq!(
          to_string_type_id_to_string_options_mut(frag_id.unwrap(), opt.clone()),
          String::from("{| ab: (%error-id%: unknown) -> () |}")
        );

        let src_id = fixture
          .base
          .base
          .base
          .get_type(&String::from("module"), true);
        LUAU_ASSERT!(src_id.is_some());
        assert_eq!(
          to_string_type_id_to_string_options_mut(src_id.unwrap(), opt.clone()),
          String::from("{|  |}")
        );
      }
    }
    {
      let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
      fixture
        .base
        .base
        .get_frontend()
        .set_luau_solver_mode(SolverMode::New);

      // checkAndExamine(source, "module", "{  }")
      fixture.base.check_with_options(&source);
      let id = fixture
        .base
        .base
        .base
        .get_type(&String::from("module"), true);
      LUAU_ASSERT!(id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
        String::from("{  }")
      );
      // [TODO] CLI-140762 Fragment autocomplete still doesn't return correct result when LuauSolverV2 is on
      // #if 0 (fragmentACAndCheck calls disabled in C++)
    }
  }
}

mod fragment_autocomplete_multiple_functions_complex {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1768:fragment_autocomplete_multiple_functions_complex`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_multiple_functions_complex() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let text = String::from(
      r#"@1 local function f1(a1)@2
    local l1 = 1;@3
    g1 = 1;@4
end
@5
local function f2(a2)
    local l2 = 1;@6
    g2 = 1;
end @7
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(!strings.contains_key("f1"));
        assert!(!strings.contains_key("a1"));
        assert!(!strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(!strings.contains_key("f2"));
        assert!(!strings.contains_key("a2"));
        assert!(!strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '2',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(strings.contains_key("f1"));
        assert!(strings.contains_key("a1"));
        assert!(!strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(!strings.contains_key("f2"));
        assert!(!strings.contains_key("a2"));
        assert!(!strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '3',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(strings.contains_key("f1"));
        assert!(strings.contains_key("a1"));
        assert!(strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(!strings.contains_key("f2"));
        assert!(!strings.contains_key("a2"));
        assert!(!strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '4',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(strings.contains_key("f1"));
        assert!(strings.contains_key("a1"));
        assert!(strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(!strings.contains_key("f2"));
        assert!(!strings.contains_key("a2"));
        assert!(!strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '5',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(strings.contains_key("f1"));
        assert!(!strings.contains_key("a1"));
        assert!(!strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(!strings.contains_key("f2"));
        assert!(!strings.contains_key("a2"));
        assert!(!strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '6',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(strings.contains_key("f1"));
        assert!(!strings.contains_key("a1"));
        assert!(!strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(strings.contains_key("f2"));
        assert!(strings.contains_key("a2"));
        assert!(strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &text,
      &text,
      '7',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
        assert!(strings.contains_key("f1"));
        assert!(!strings.contains_key("a1"));
        assert!(!strings.contains_key("l1"));
        assert!(strings.contains_key("g1"));
        assert!(strings.contains_key("f2"));
        assert!(!strings.contains_key("a2"));
        assert!(!strings.contains_key("l2"));
        assert!(strings.contains_key("g2"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_mutually_recursive_alias {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2962:fragment_autocomplete_mutually_recursive_alias`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_mutually_recursive_alias() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::{
      fragment_autocomplete_status_result::FragmentAutocompleteStatusResult, scope::Scope,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type U = {f : number, g : U}

"#,
    );
    let dest = String::from(
      r#"
type U = {f : number, g : V}
type V = {h : number, i : U?} @1
"#,
    );

    // Re-parsing and typechecking a type alias in the fragment that was defined in the base module will assert in ConstraintGenerator::checkAliases
    // unless we don't clone it This will let the incremental pass re-generate the type binding, and we will expect to see it in the type bindings
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(!frag.result.as_ref().unwrap().fresh_scope.is_null());
        let scope: *mut Scope = frag.result.as_ref().unwrap().fresh_scope;
        assert!(1 == unsafe { (*scope).private_type_bindings.contains_key("U") } as usize);
        assert!(1 == unsafe { (*scope).private_type_bindings.contains_key("V") } as usize);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_nearest_enclosing_statement_can_be_non_local {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1065:fragment_autocomplete_nearest_enclosing_statement_can_be_non_local`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_nearest_enclosing_statement_can_be_non_local

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_nearest_enclosing_statement_can_be_non_local() {
    use alloc::string::String;
    use core::ffi::CStr;

    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_if::AstStatIf, position::Position},
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      &String::from(
        r#"
local x = 4
local y = 5
if x == 4 then
"#,
      ),
      &Position { line: 3, column: 4 },
    );

    assert_eq!(4, result.ancestry.len());
    assert_eq!(2, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    assert!(!result.nearest_statement.is_null());
    let last = *result.local_stack.last().unwrap();
    assert_eq!(
      "y",
      unsafe { CStr::from_ptr((*last).name.value) }
        .to_str()
        .unwrap()
    );

    let if_stmt = unsafe { ast_node_as::<AstStatIf>(result.nearest_statement as *mut AstNode) };
    assert!(!if_stmt.is_null());
  }
}

mod fragment_autocomplete_nested_blocks_else_difficult_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3684:fragment_autocomplete_nested_blocks_else_difficult_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_nested_blocks_else_difficult_2() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local function foo(t : {foo : number})
    do
        if t then
        end
    end
end
"#,
    );
    let dest = String::from(
      r#"
local function foo(t : {foo : number})
    do
        if t then
        else
            local x = 4
            return x + t@1.
        end
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|res: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, res.status);
        LUAU_ASSERT!(res.result.is_some());
        let ac_results = &res.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("foo"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_nested_blocks_else_simple {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3648:fragment_autocomplete_nested_blocks_else_simple`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_nested_blocks_else_simple() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local function foo(t : {foo : string})
    local x = t.foo
    do
        if t then
        end
    end
end
"#,
    );
    let dest = String::from(
      r#"
local function foo(t : {foo : string})
    local x = t.foo
    do
        if t then
            x:@1
        end
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|res: &mut FragmentAutocompleteStatusResult| {
        assert!(FragmentAutocompleteStatus::Success == res.status);
        assert!(res.result.is_some());
        let ac = &res.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("gsub"));
        assert!(ac.entry_map.contains_key("len"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_nested_recursive_function_fragment_autocomplete_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1984:fragment_autocomplete_nested_recursive_function`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_nested_recursive_function() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
function foo()
@1end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(fragment.result.is_some());
        let ac_results = &fragment.result.as_ref().unwrap().ac_results;
        assert!(ac_results.entry_map.contains_key("foo"));
        assert_eq!(AutocompleteContext::Statement, ac_results.context);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_nested_recursive_function_fragment_autocomplete_test_alt_b {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2151:fragment_autocomplete_nested_recursive_function`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_nested_recursive_function() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local function outer()
            local function inner()
@1            end
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("inner"));
        assert!(ac.entry_map.contains_key("outer"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_no_recs_for_comments {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2770:fragment_autocomplete_no_recs_for_comments`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_no_recs_for_comments() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
-- sel @1
-- retur @2
-- fo @3
--[[ sel @4]]
local @5 -- hell@6o
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '2',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '3',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '4',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '5',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          !frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '6',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_no_recs_for_comments_blocks {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2712:fragment_autocomplete_no_recs_for_comments_blocks`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_no_recs_for_comments_blocks() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
--[[
comment 1
@1]]@2 local
-- [[ comment 2]]
--
-- sdfsdfsdf
--[[comment 3]]
--[[  @3
foo
@4bar
baz
]]
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '2',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(
          !frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '3',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '4',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_no_recs_for_comments_in_incremental_fragment {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2841:fragment_autocomplete_no_recs_for_comments_in_incremental_fragment`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_no_recs_for_comments_in_incremental_fragment() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
local x = 5
if x == 5
"#,
    );
    let updated = String::from(
      r#"
local x = 5
if x == 5 then -- a comment @1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_no_recs_for_comments_simple {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2691:fragment_autocomplete_no_recs_for_comments_simple`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_no_recs_for_comments_simple() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
-- sel
-- retur
-- fo
-- if @1
-- end
-- the
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_none());
      }),
      None,
    );
  }
}

mod fragment_autocomplete_not_null_assertion_caused_by_leaking_free_type_from_stale_module {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3719:fragment_autocomplete_not_null_assertion_caused_by_leaking_free_type_from_stale_module`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_not_null_assertion_caused_by_leaking_free_type_from_stale_module() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
local Players = game:GetService("Players")

Players.PlayerAdded:Connect(function(Player)
    for_,v in script.PlayerValue:GetChildren()do
        v
    end
end)
"#,
    );

    let dest = String::from(
      r#"
local Players = game:GetService("Players")

Players.PlayerAdded:Connect(function(Player)
    for_,v in script.PlayerValue:GetChildren()do
        v:l@1
    end
end)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|_result: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_not_null_nil_scope_assertion_caused_by_free_type_inheriting_null_scope_from_table {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3178:fragment_autocomplete_not_null_nil_scope_assertion_caused_by_free_type_inheriting_null_scope_from_table`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_not_null_nil_scope_assertion_caused_by_free_type_inheriting_null_scope_from_table()
   {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y


"#,
    );

    let dest = String::from(
      r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y

z = a.P.E@1
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|_frag: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_oss_1850 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4622:fragment_autocomplete_oss_1850`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_oss_1850() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type t = { name: "t", } | { name: "ts", person: "dog" }

local t:t
if t.name == "ts" then
end
    "#,
    );
    let dest = String::from(
      r#"
type t = { name: "t", } | { name: "ts", person: "dog" }

local t:t
if t.name == "ts" then
    t.@1
end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("name"));
        assert!(ac_results.entry_map.contains_key("person"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_partial_for_in_in_body {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:826:fragment_autocomplete_partial_for_in_in_body`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatForIn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_for_in_in_body

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_for_in_in_body() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_for_in::AstStatForIn, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
for i,v in {1,2,3} do
"#,
      ),
      &Position {
        line: 1,
        column: 21,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 21
        },
        end: Position {
          line: 1,
          column: 21
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatForIn>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_for_in_in_condition_1 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:784:fragment_autocomplete_partial_for_in_in_condition_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatForIn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_for_in_in_condition_1

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_for_in_in_condition_1() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_for_in::AstStatForIn, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
for i,v in {1,2,3}
"#,
      ),
      &Position {
        line: 1,
        column: 18,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 18
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatForIn>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_for_in_in_condition_2 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:798:fragment_autocomplete_partial_for_in_in_condition_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatForIn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_for_in_in_condition_2

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_for_in_in_condition_2() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_for_in::AstStatForIn, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
for i,v in
"#,
      ),
      &Position {
        line: 1,
        column: 10,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 10
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatForIn>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_for_in_in_condition_3 {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:812:fragment_autocomplete_partial_for_in_in_condition_3`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatForIn (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_for_in_in_condition_3

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_for_in_in_condition_3() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_for_in::AstStatForIn, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
for i,
"#,
      ),
      &Position { line: 1, column: 6 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 6 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatForIn>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_for_numeric_in_body {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:769:fragment_autocomplete_partial_for_numeric_in_body`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFor (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_for_numeric_in_body

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_for_numeric_in_body() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_for::AstStatFor, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
for c = 1,3 do
"#,
      ),
      &Position {
        line: 1,
        column: 14,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 14
        },
        end: Position {
          line: 1,
          column: 14
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFor>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_for_numeric_in_condition {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:755:fragment_autocomplete_partial_for_numeric_in_condition`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFor (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_for_numeric_in_condition

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_for_numeric_in_condition() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_for::AstStatFor, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
for c = 1,3
"#,
      ),
      &Position {
        line: 1,
        column: 11,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 10
        },
        end: Position {
          line: 1,
          column: 11
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFor>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_statement_after_do {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:503:fragment_autocomplete_partial_statement_after_do`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_statement_after_do

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_statement_after_do() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local::AstStatLocal, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local x = 4
do

end
local x =
"#,
      ),
      &Position { line: 5, column: 9 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 5, column: 0 },
        end: Position { line: 5, column: 9 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocal>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_statement_inside_do {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:486:fragment_autocomplete_partial_statement_inside_do`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_statement_inside_do

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_statement_inside_do() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local::AstStatLocal, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local x = 4
do
    local x =
end
"#,
      ),
      &Position {
        line: 3,
        column: 13,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 3, column: 4 },
        end: Position {
          line: 3,
          column: 13
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocal>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_partial_while_in_condition {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:726:fragment_autocomplete_partial_while_in_condition`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatWhile (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_partial_while_in_condition

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_partial_while_in_condition() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_while::AstStatWhile, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
while t
"#,
      ),
      &Position { line: 1, column: 7 },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 7 },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatWhile>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_require_tracing {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2885:fragment_autocomplete_require_tracing`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_require_tracing() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();

    fixture.base.base.base.file_resolver.source.insert(
      String::from("MainModule/A"),
      String::from(
        r#"
return { x = 0 }
    "#,
      ),
    );

    fixture.base.base.base.file_resolver.source.insert(
      String::from("MainModule"),
      String::from(
        r#"
local result = require(script.A)
local x = 1 + result.@1
    "#,
      ),
    );

    let main_module = fixture
      .base
      .base
      .base
      .file_resolver
      .source
      .get("MainModule")
      .unwrap()
      .clone();

    fixture.base.autocomplete_fragment_in_both_solvers(
      &main_module,
      &main_module,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        assert!(frag.result.as_ref().unwrap().ac_results.entry_map.len() == 1);
        assert!(
          frag
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("x")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_respects_frontend_options {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1435:fragment_autocomplete_respects_frontend_options`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_respects_frontend_options() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      functions::try_fragment_autocomplete::try_fragment_autocomplete,
      records::{fragment_context::FragmentContext, frontend_options::FrontendOptions},
    };
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback,
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    // NOTE: This does not pass the new solver because it is exercising behavior
    // that is only meaningful under the old solver (whether the correct
    // module resolver is used).
    //
    // C++ `DOES_NOT_PASS_NEW_SOLVER_GUARD()` =>
    // `ScopedFastFlag{FFlag::DebugLuauForceOldSolver, !FFlag::DebugLuauForceAllNewSolverTests}`.
    let _guard = ScopedFastFlag::new(
      &FFlag::DebugLuauForceOldSolver,
      !FFlag::DebugLuauForceAllNewSolverTests.get(),
    );

    let source = String::from(
      r#"
local tbl = { abc = 1234}
t
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture
      .base
      .base
      .base
      .file_resolver
      .source
      .insert(String::from("game/A"), source.clone());

    let opts = FrontendOptions {
      for_autocomplete: true,
      ..Default::default()
    };

    {
      let frontend = fixture.base.base.get_frontend();
      frontend.set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
      frontend
        .check_module_name_optional_frontend_options(&String::from("game/A"), Some(opts.clone()));
      assert!(
        frontend
          .module_resolver_for_autocomplete
          .modules
          .contains_key(&String::from("game/A"))
      );
      assert!(!frontend.module_resolver.modules.contains_key("game/A"));
    }

    let parse_result = fixture.base.parse_helper(source.clone());
    let context =
      FragmentContext::new_with_options(source.as_str(), &parse_result, Some(opts.clone()), None);

    let frontend = fixture.base.base.get_frontend();
    let frag = try_fragment_autocomplete(
      frontend,
      &String::from("game/A"),
      Position { line: 2, column: 1 },
      context,
      Box::new(null_callback),
    );

    LUAU_ASSERT!(frag.result.is_some());
    let result = frag.result.as_ref().unwrap();
    assert_eq!("game/A", result.incremental_module.name);

    let frontend = fixture.base.base.get_frontend();
    assert!(
      frontend
        .module_resolver_for_autocomplete
        .modules
        .contains_key("game/A")
    );
    assert!(!frontend.module_resolver.modules.contains_key("game/A"));
  }
}

mod fragment_autocomplete_self_types_provide_rich_autocomplete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4048:fragment_autocomplete_self_types_provide_rich_autocomplete`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_self_types_provide_rich_autocomplete() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()

end
"#,
    );
    let dest = String::from(
      r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()
    self.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("Prop"));
        assert!(ac_results.entry_map.contains_key("Start"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_self_with_colon_good_recommendations {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4157:fragment_autocomplete_self_with_colon_good_recommendations`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_self_with_colon_good_recommendations() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()

end
"#,
    );
    let dest = String::from(
      r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()
    self:@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("Prop"));
        assert!(ac.entry_map["Prop"].wrong_index_type);
        assert!(ac.entry_map.contains_key("Start"));
        assert!(!ac.entry_map["Start"].wrong_index_type);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_self_with_fancy_metatable_setting_new_solver {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4088:fragment_autocomplete_self_with_fancy_metatable_setting_new_solver`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_self_with_fancy_metatable_setting_new_solver() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
        type IAccount = {
            __index: IAccount,
            new : (string, number) -> Account,
            report: (self: Account) -> (),
        }

        export type Account = setmetatable<{
            name: string,
            balance: number
        }, IAccount>;

        local Account = {} :: IAccount
        Account.__index = Account

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance
            return setmetatable(self, Account)
        end

        function Account:report()
            print("My balance is: " .. )
        end
"#,
    );

    let dest = String::from(
      r#"
        type IAccount = {
            __index: IAccount,
            new : (string, number) -> Account,
            report: (self: Account) -> (),
        }

        export type Account = setmetatable<{
            name: string,
            balance: number
        }, IAccount>;

        local Account = {} :: IAccount
        Account.__index = Account

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance
            return setmetatable(self, Account)
        end

        function Account:report()
            print("My balance is: " .. self.@1 )
        end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("new"));
        assert!(ac.entry_map.contains_key("report"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_single_line_local_and_annot {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:712:fragment_autocomplete_single_line_local_and_annot`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatError (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_single_line_local_and_annot

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_single_line_local_and_annot() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_error::AstStatError, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
type Part = {x : number}
local part : Part = {x = 3}; pa
"#,
      ),
      &Position {
        line: 2,
        column: 32,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 29
        },
        end: Position {
          line: 2,
          column: 32
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatError>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_singleline_call {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:374:fragment_autocomplete_singleline_call`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_singleline_call

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_singleline_call() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_expr::AstStatExpr, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
abc("foo")
"#,
      ),
      &Position {
        line: 1,
        column: 10,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 10
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(!region.nearest_statement.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatExpr>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_statement_in_empty_fragment_is_non_null {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1190:fragment_autocomplete_statement_in_empty_fragment_is_non_null`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_statement_in_empty_fragment_is_non_null

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_statement_in_empty_fragment_is_non_null() {
    use alloc::string::String;

    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_block::AstStatBlock, position::Position},
      rtti::ast_node_as,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let source = String::from(
      r#"

"#,
    );
    let result = fixture.base.check_with_options(&source);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
      .base
      .parse_fragment(&source, &Position { line: 1, column: 0 }, None)
      .expect("expected fragment parse result");

    assert_eq!("", fragment.fragment_to_parse);
    assert_eq!(1, fragment.ancestry.len());
    assert!(!fragment.root.is_null());
    assert_eq!(0, unsafe { (*fragment.root).body.size });
    let stat_body = unsafe { ast_node_as::<AstStatBlock>(fragment.root as *mut AstNode) };
    assert!(!stat_body.is_null());
  }
}

mod fragment_autocomplete_str_metata_table_finished_defining {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3581:fragment_autocomplete_str_metata_table_finished_defining`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_str_metata_table_finished_defining() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"local function foobar(): string return "" end
local foo = f"#,
    );
    let dest = String::from(
      r#"local function foobar(): string return "" end
local foo = foobar()
foo:@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|res: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, res.status);
        LUAU_ASSERT!(res.result.is_some());
        let ac_results = &res.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("len"));
        assert!(ac_results.entry_map.contains_key("gsub"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_str_metata_table_redef {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3604:fragment_autocomplete_str_metata_table_redef`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_str_metata_table_redef() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#"local x = 42"#);
    let dest = String::from(
      r#"local x = 42
local x = ""
x:@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|res: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, res.status);
        assert!(res.result.is_some());
        let ac = &res.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("len"));
        assert!(ac.entry_map.contains_key("gsub"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_string_interpolation_format_provides_autocomplete_results {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4199:fragment_autocomplete_string_interpolation_format_provides_autocomplete_results`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_string_interpolation_format_provides_autocomplete_results() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type Foo = {x : number, x1 : string, x2 : boolean}
local e: Foo = {x = 1, x1 = "1", x2 = true}
local s =
"#,
    );

    let dest = String::from(
      r#"
type Foo = {x : number, x1 : string, x2 : boolean}
local e : Foo = {x = 1, x1 = "1", x2 = true}
local s = `{e.@1 }`
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("x1"));
        assert!(ac.entry_map.contains_key("x2"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_string_interpolation_format_provides_results_inside_of_function_call {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4227:fragment_autocomplete_string_interpolation_format_provides_results_inside_of_function_call`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_string_interpolation_format_provides_results_inside_of_function_call() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type T = {x : number, y : number, z : number}
local e = {x = 1, y = 2, z = 3}
print(`{e.x}`)
"#,
    );

    let dest = String::from(
      r#"
type T = {x : number, y : number, z : number}
local e = {x = 1, y = 2, z = 3}
print(`{e.x} {e.@1}`)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("y"));
        assert!(ac.entry_map.contains_key("z"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_string_literal_with_override {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2003:fragment_autocomplete_string_literal_with_override`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_string_literal_with_override() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
function foo(bar: string) end
foo("a@1bc")
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|fragment: &mut FragmentAutocompleteStatusResult| {
        assert!(fragment.result.is_some());
        let ac_results = &fragment.result.as_ref().unwrap().ac_results;
        assert!(ac_results.entry_map.is_empty());
        assert_eq!(AutocompleteContext::String, ac_results.context);
      }),
      Some(Position { line: 2, column: 9 }),
    );
  }
}

mod fragment_autocomplete_studio_ice_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2424:fragment_autocomplete_studio_ice_1`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_studio_ice_1() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
--Woop
\@native
local function test()

end
"#,
    );

    let updated = String::from(
      r#"
--Woop
\@native
local function test()

end
function a@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|_result: &mut FragmentAutocompleteStatusResult| {}),
      None,
    );
  }
}

mod fragment_autocomplete_table_intersection {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2369:fragment_autocomplete_table_intersection`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_table_intersection() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)

        end
    "#,
    );
    let updated = String::from(
      r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)
            abc.@1
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(3, ac.entry_map.len());
        assert!(ac.entry_map.contains_key("a1"));
        assert!(ac.entry_map.contains_key("b2"));
        assert!(ac.entry_map.contains_key("c3"));
        assert_eq!(AutocompleteContext::Property, ac.context);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_table_union {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2337:fragment_autocomplete_table_union`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_table_union() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : string, c3 : string }
        function func(abc : t1 | t2)

        end
    "#,
    );
    let updated = String::from(
      r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : string, c3 : string }
        function func(abc : t1 | t2)
            abc.@1
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &updated,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(1, ac.entry_map.len());
        assert!(ac.entry_map.contains_key("b2"));
        assert_eq!(ac.context, AutocompleteContext::Property);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_tagged_union_completion_first_branch_of_union_new_solver {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3939:fragment_autocomplete_tagged_union_completion_first_branch_of_union_new_solver`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_tagged_union_completion_first_branch_of_union_new_solver() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    // TODO: CLI-155619 - Fragment autocomplete needs to use stale refinement information for modules typechecked in the new solver as well
    let source = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then

end
"#,
    );

    let dest = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then
    result.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert_eq!(ac.entry_map.contains_key("type") as usize, 1);
        assert_eq!(ac.entry_map.contains_key("value") as usize, 1);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_tagged_union_completion_first_branch_of_union_old_solver {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3862:fragment_autocomplete_tagged_union_completion_first_branch_of_union_old_solver`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_tagged_union_completion_first_branch_of_union_old_solver() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then

end
"#,
    );

    let dest = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then
    result.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_old_solver(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert_eq!(ac.entry_map.contains_key("type") as usize, 1);
        assert_eq!(ac.entry_map.contains_key("value") as usize, 1);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_tagged_union_completion_second_branch_of_union_new_solver {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3978:fragment_autocomplete_tagged_union_completion_second_branch_of_union_new_solver`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_tagged_union_completion_second_branch_of_union_new_solver() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then

end
"#,
    );

    let dest = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then
    result.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("type"));
        assert!(ac.entry_map.contains_key("error"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_tagged_union_completion_second_branch_of_union_old_solver {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3900:fragment_autocomplete_tagged_union_completion_second_branch_of_union_old_solver`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_tagged_union_completion_second_branch_of_union_old_solver() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then

end
"#,
    );

    let dest = String::from(
      r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then
    result.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_old_solver(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert_eq!(ac.entry_map.contains_key("type") as usize, 1);
        assert_eq!(ac.entry_map.contains_key("error") as usize, 1);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_tbl_function_parameter {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2479:fragment_autocomplete_tbl_function_parameter`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_tbl_function_parameter() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
--!strict
type Foo = {x : number, y : number}
local function func(abc : Foo)
   abc.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac_results = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(2, ac_results.entry_map.len());
        assert!(ac_results.entry_map.contains_key("x"));
        assert!(ac_results.entry_map.contains_key("y"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_tbl_local_function_parameter {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2503:fragment_autocomplete_tbl_local_function_parameter`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_tbl_local_function_parameter() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
--!strict
type Foo = {x : number, y : number}
local function func(abc : Foo)
   abc.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(2, ac.entry_map.len());
        assert!(ac.entry_map.contains_key("x"));
        assert!(ac.entry_map.contains_key("y"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_thrown_parse_error_leads_to_null_root {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1171:fragment_autocomplete_thrown_parse_error_leads_to_null_root`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item fragment_autocomplete_thrown_parse_error_leads_to_null_root

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_thrown_parse_error_leads_to_null_root() {
    use alloc::string::String;

    use ulua_ast::records::position::Position;
    use ulua_common::FInt;
    use ulua_unit_test::{
      records::fragment_autocomplete_fixture::FragmentAutocompleteFixture,
      type_aliases::scoped_fast_int::ScopedFastInt,
    };

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.check_with_options(&String::from("type A =  "));
    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);
    let fragment = fixture.base.parse_fragment(
      &String::from("type A = <>function<> more garbage here"),
      &Position {
        line: 0,
        column: 39,
      },
      None,
    );

    assert!(fragment.is_none());
  }
}

mod fragment_autocomplete_type_correct_local_rank_assert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3558:fragment_autocomplete_type_correct_local_rank_assert`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_type_correct_local_rank_assert() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#""#);
    let dest = String::from(
      r#"local function target(a: number, b: string) return a + #b end
local bar1 = 'hello'
local bar2 = 4
return target(bar@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|status: &mut FragmentAutocompleteStatusResult| {
        assert!(FragmentAutocompleteStatus::Success == status.status);
        LUAU_ASSERT!(status.result.is_some());
        assert!(
          !status
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .is_empty()
        );
        assert!(
          status
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("bar1")
        );
        assert!(
          status
            .result
            .as_ref()
            .unwrap()
            .ac_results
            .entry_map
            .contains_key("bar2")
        );
      }),
      None,
    );
  }
}

mod fragment_autocomplete_type_correct_local_return_assert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3535:fragment_autocomplete_type_correct_local_return_assert`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_type_correct_local_return_assert() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::fragment_autocomplete_status::FragmentAutocompleteStatus,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(r#""#);
    let dest = String::from(
      r#"local function target(a: number, b: string) return a + #b end
local function bar1(a: string) reutrn a .. 'x' end
local function bar2(a: number) return -a end
return target(bar@1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|status: &mut FragmentAutocompleteStatusResult| {
        assert!(FragmentAutocompleteStatus::Success == status.status);
        LUAU_ASSERT!(status.result.is_some());
        let ac = &status.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("bar1"));
        assert!(ac.entry_map.contains_key("bar2"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_typecheck_fragment_handles_unusable_module {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1608:fragment_autocomplete_typecheck_fragment_handles_unusable_module`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_typecheck_fragment_handles_unusable_module() {
    use alloc::string::String;

    use ulua_analysis::enums::fragment_type_check_status::FragmentTypeCheckStatus;
    use ulua_ast::records::position::Position;
    use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};
    use ulua_unit_test::{
      functions::get_options::get_options,
      records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
    };

    let source_a = String::from("MainModule");
    let source_b = String::from("game/Gui/Modules/B");

    let source_a_text = String::from(
      r#"
local Modules = game:GetService('Gui').Modules
local B = require(Modules.B)
return { hello = B }
"#,
    );
    let source_b_text = String::from(r#"return {hello = "hello"}"#);

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture
      .base
      .base
      .base
      .file_resolver
      .source
      .insert(source_a.clone(), source_a_text.clone());
    fixture
      .base
      .base
      .base
      .file_resolver
      .source
      .insert(source_b.clone(), source_b_text.clone());

    let for_autocomplete = get_options().for_autocomplete;

    {
      let frontend = fixture.base.base.get_frontend();
      let _result =
        frontend.check_module_name_optional_frontend_options(&source_a, Some(get_options()));
      assert!(!frontend.is_dirty(&source_a, for_autocomplete));
    }

    // C++ `getModuleResolver(frontend)` selects `moduleResolver` under the new
    // solver and `moduleResolverForAutocomplete` under the old solver. The
    // weak_ptr / expired() semantics map to `Arc::downgrade` / `upgrade`.
    let weak_module = {
      let frontend = fixture.base.base.get_frontend();
      let resolver = if !FFlag::DebugLuauForceOldSolver.get() {
        &frontend.module_resolver
      } else {
        &frontend.module_resolver_for_autocomplete
      };
      let module = resolver.modules.get(&source_b);
      LUAU_ASSERT!(module.is_some());
      use alloc::sync::Arc;
      Arc::downgrade(module.unwrap())
    };
    // `REQUIRE(!weakModule.expired())`
    assert!(weak_module.upgrade().is_some());

    {
      let frontend = fixture.base.base.get_frontend();
      frontend.mark_dirty(&source_b, None);
      assert!(frontend.is_dirty(&source_a, for_autocomplete));

      frontend.check_module_name_optional_frontend_options(&source_b, Some(get_options()));
    }
    // `CHECK(weakModule.expired())`
    assert!(weak_module.upgrade().is_none());

    let (status, _) = fixture.base.typecheck_fragment_for_module(
      &source_a,
      &source_a_text,
      Position { line: 0, column: 0 },
      None,
    );
    assert_eq!(FragmentTypeCheckStatus::SkipAutocomplete, status);

    let (status2, _) = fixture.base.typecheck_fragment_for_module(
      &source_b,
      &source_b_text,
      Position {
        line: 3,
        column: 20,
      },
      None,
    );
    assert_eq!(FragmentTypeCheckStatus::Success, status2);
  }
}

mod fragment_autocomplete_unary_minus_operator_needs_to_provide_autocomplete_results {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4479:fragment_autocomplete_unary_minus_operator_needs_to_provide_autocomplete_results`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_unary_minus_operator_needs_to_provide_autocomplete_results() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
type Pool = { x : number }

local function foobar(p)
    local pool = p :: Pool
    if -pool
end
"#,
    );
    let dest = String::from(
      r#"
type Pool = { x : number }

local function foobar(p)
    local pool = p :: Pool
    if -pool.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      Box::new(|result: &mut FragmentAutocompleteStatusResult| {
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("x"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_unsealed_table {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2267:fragment_autocomplete_unsealed_table`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_unsealed_table() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local tbl = {}
        tbl.prop = 5
        tbl.@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(1, ac.entry_map.len());
        assert!(ac.entry_map.contains_key("prop"));
        assert_eq!(AutocompleteContext::Property, ac.context);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_unsealed_table_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2290:fragment_autocomplete_unsealed_table_2`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_unsealed_table_2() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local tbl = {}
        local inner = { prop = 5 }
        tbl.inner = inner
        tbl.inner.@1
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(1, ac.entry_map.len());
        assert!(ac.entry_map.contains_key("prop"));
        assert_eq!(ac.context, AutocompleteContext::Property);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_user_defined_globals {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2078:fragment_autocomplete_user_defined_globals`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_user_defined_globals() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from("local myLocal = 4;@1 ");

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;

        assert!(ac.entry_map.contains_key("myLocal"));
        assert!(ac.entry_map.contains_key("table"));
        assert!(ac.entry_map.contains_key("math"));
        assert_eq!(ac.context, AutocompleteContext::Statement);
      }),
      None,
    );
  }
}

mod fragment_autocomplete_user_defined_local_functions_in_own_definition {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2174:fragment_autocomplete_user_defined_local_functions_in_own_definition`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_user_defined_local_functions_in_own_definition() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let source = String::from(
      r#"
        local function abc()
@1
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    // Autocomplete inside of abc
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("abc"));
        assert!(ac.entry_map.contains_key("table"));
        assert!(ac.entry_map.contains_key("math"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_user_defined_type_function_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3205:fragment_autocomplete_user_defined_type_function_local`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function getOptions (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record FragmentAutocompleteStatusResult (Analysis/include/Luau/FragmentAutocomplete.h)
  //!   - calls -> method FragmentAutocompleteFixtureImpl::autocompleteFragment (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> enum FragmentAutocompleteStatus (Analysis/include/Luau/FragmentAutocomplete.h)
  //!   - translates_to -> rust_item fragment_autocomplete_user_defined_type_function_local

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_user_defined_type_function_local() {
    use alloc::string::String;

    use ulua_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let source = String::from(
      r#"--!strict
type function foo(x: type): type
    if x.tag == "singleton" then
        local t = x:value()

        return types.unionof(types.singleton(t), types.singleton(nil))
    end

    return types.number
end
"#,
    );

    let dest = String::from(
      r#"--!strict
type function foo(x: type): type
    if x.tag == "singleton" then
        local t = x:value()
        x
        return types.unionof(types.singleton(t), types.singleton(nil))
    end

    return types.number
end
"#,
    );

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_with_options(&source);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture
      .base
      .autocomplete_fragment(&dest, Position { line: 4, column: 9 }, None);
    assert_ne!(FragmentAutocompleteStatus::InternalIce, result.status);
  }
}

mod fragment_autocomplete_vec_3_function_parameter {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2527:fragment_autocomplete_vec_3_function_parameter`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_vec_3_function_parameter() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_common::macros::luau_assert::LUAU_ASSERT;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
--!strict
local function func(abc : FakeVec)
   abc.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    // C++ `FragmentAutocompleteBuiltinsFixture::getFrontend()` virtually loads the `FakeVec`
    // class declaration into the (auto)globals on first frontend access; prime it here so the
    // shared frontend used by `autocomplete_fragment_in_both_solvers` has `FakeVec` available.
    fixture.get_frontend();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(2, ac.entry_map.len());
        assert!(ac.entry_map.contains_key("zero"));
        assert!(ac.entry_map.contains_key("dot"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_vec_3_local_function_parameter {
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2550:fragment_autocomplete_vec_3_local_function_parameter`
  //! Source: `tests/FragmentAutocomplete.test.cpp`

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_vec_3_local_function_parameter() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use ulua_unit_test::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;

    let source = String::from(
      r#"
--!strict
local function func(abc : FakeVec)
   abc.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac_results = &frag.result.as_ref().unwrap().ac_results;
        assert_eq!(2, ac_results.entry_map.len());
        assert!(ac_results.entry_map.contains_key("zero"));
        assert!(ac_results.entry_map.contains_key("dot"));
      }),
      None,
    );
  }
}

mod fragment_autocomplete_while_inside_condition_same_line {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:740:fragment_autocomplete_while_inside_condition_same_line`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatWhile (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_while_inside_condition_same_line

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_while_inside_condition_same_line() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_while::AstStatWhile, location::Location, position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
while true do
end
"#,
      ),
      &Position {
        line: 1,
        column: 13,
      },
    );

    assert_eq!(
      Location {
        begin: Position {
          line: 1,
          column: 13
        },
        end: Position {
          line: 1,
          column: 13
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatWhile>(region.nearest_statement as *mut AstNode) }.is_null()
    );
  }
}

mod fragment_autocomplete_while_writing_func {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:564:fragment_autocomplete_while_writing_func`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_while_writing_func

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_while_writing_func() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f(arg1,
"#,
      ),
      &Position {
        line: 1,
        column: 17,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 17
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_while_writing_local_func {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:659:fragment_autocomplete_while_writing_local_func`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_while_writing_local_func

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_while_writing_local_func() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f(arg1,
"#,
      ),
      &Position {
        line: 1,
        column: 22,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 22
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_writing_func_annotation {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:577:fragment_autocomplete_writing_func_annotation`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_writing_func_annotation

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_writing_func_annotation() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f(arg1 : T
"#,
      ),
      &Position {
        line: 1,
        column: 19,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 19
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_writing_func_return {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:590:fragment_autocomplete_writing_func_return`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_writing_func_return

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_writing_func_return() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f(arg1 : T) :
"#,
      ),
      &Position {
        line: 1,
        column: 22,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 22
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_writing_func_return_pack {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:603:fragment_autocomplete_writing_func_return_pack`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_writing_func_return_pack

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_writing_func_return_pack() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_function::AstStatFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
function f(arg1 : T) : T...
"#,
      ),
      &Position {
        line: 1,
        column: 27,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 27
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_writing_local_func_annotation {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:672:fragment_autocomplete_writing_local_func_annotation`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_writing_local_func_annotation

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_writing_local_func_annotation() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f(arg1 : T
"#,
      ),
      &Position {
        line: 1,
        column: 25,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 25
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_writing_local_func_return {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:685:fragment_autocomplete_writing_local_func_return`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_writing_local_func_return

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_writing_local_func_return() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f(arg1 : T) :
"#,
      ),
      &Position {
        line: 1,
        column: 28,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 28
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}

mod fragment_autocomplete_writing_local_func_return_pack {
  //! Generated skeleton item.
  //! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:698:fragment_autocomplete_writing_local_func_return_pack`
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/FileResolver.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method FragmentAutocompleteFixtureImpl::getAutocompleteRegion (tests/FragmentAutocomplete.test.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstStatLocalFunction (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item fragment_autocomplete_writing_local_func_return_pack

  #[cfg(test)]
  #[test]
  fn fragment_autocomplete_writing_local_func_return_pack() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_local_function::AstStatLocalFunction, location::Location,
        position::Position,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;

    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture.base.get_autocomplete_region(
      String::from(
        r#"
local function f(arg1 : T) : T...
"#,
      ),
      &Position {
        line: 1,
        column: 33,
      },
    );

    assert_eq!(
      Location {
        begin: Position { line: 1, column: 0 },
        end: Position {
          line: 1,
          column: 33
        },
      },
      region.fragment_location
    );
    assert!(!region.parent_block.is_null());
    assert!(
      !unsafe { ast_node_as::<AstStatLocalFunction>(region.nearest_statement as *mut AstNode) }
        .is_null()
    );
  }
}
