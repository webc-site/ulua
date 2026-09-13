use alloc::vec::Vec;
extern crate alloc;

mod autocomplete_ac_dont_overflow_on_recursive_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4507:autocomplete_ac_dont_overflow_on_recursive_union`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
  //!   - translates_to -> rust_item autocomplete_ac_dont_overflow_on_recursive_union
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_ac_dont_overflow_on_recursive_union() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::register_ac_extern_type_fixture_types::register_ac_extern_type_fixture_types,
      records::ac_extern_type_fixture::AcExternTypeFixture,
    };

    let mut fixture = AcExternTypeFixture::default();
    register_ac_extern_type_fixture_types(&mut fixture.base);

    fixture.base.check(String::from(
      r#"
        local table1: {ChildClass} = {}
        local table2 = {}

        for index, value in table2[1] do
            table.insert(table1, value)
            value.@1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(
        ac.entry_map.contains_key("BaseMethod"),
        "entries: {:?}",
        ac.entry_map.keys().collect::<Vec<_>>()
      );
      assert!(
        ac.entry_map.contains_key("Method"),
        "entries: {:?}",
        ac.entry_map.keys().collect::<Vec<_>>()
      );
    } else {
      assert!(ac.entry_map.is_empty());
    }
  }
}

mod autocomplete_ac_static_method_autocomplete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5280:autocomplete_ac_static_method_autocomplete`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_ac_static_method_autocomplete
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_ac_static_method_autocomplete() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        class Bar
            public value: number
            function new()
                return Bar { value = 0 }
            end
        end

        Bar.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("new"));
  }
}

mod autocomplete_anonymous_autofilled_args {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4037:autocomplete_anonymous_autofilled_args`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_args

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_args() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (number, string) -> ())
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: number, a1: string)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_args_multi_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4079:autocomplete_anonymous_autofilled_args_multi_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_args_multi_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_args_multi_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (number, string) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: number, a1: string): (string, number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_args_single_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4058:autocomplete_anonymous_autofilled_args_single_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_args_single_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_args_single_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (number, string) -> (string))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: number, a1: string): string  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_empty {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4016:autocomplete_anonymous_autofilled_empty`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_empty

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_empty() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: () -> ())
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(entry.insert_text.as_deref(), Some("function()  end"));
  }
}

mod autocomplete_anonymous_autofilled_generic_named_arg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4394:autocomplete_anonymous_autofilled_generic_named_arg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_generic_named_arg

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_generic_named_arg() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo<A>(f: (a: A) -> number, a: A)
	return f(a)
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a): number  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4436:autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function foo(a: <T...>(...: T...) -> number)
            return a(4, 5, 6)
        end

        foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    let expected_insert = if !FFlag::DebugLuauForceOldSolver.get() {
      "function(...: number): number  end"
    } else {
      "function(...): number  end"
    };
    assert_eq!(entry.insert_text.as_deref(), Some(expected_insert));
  }
}

mod autocomplete_anonymous_autofilled_generic_return_type {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4415:autocomplete_anonymous_autofilled_generic_return_type`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_generic_return_type

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_generic_return_type() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo<A>(f: () -> A)
	return f()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(entry.insert_text.as_deref(), Some("function()  end"));
  }
}

mod autocomplete_anonymous_autofilled_generic_type_pack_vararg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4373:autocomplete_anonymous_autofilled_generic_type_pack_vararg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_generic_type_pack_vararg

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_generic_type_pack_vararg() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo<A>(a: (...A) -> number, ...: A)
	return a(...)
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(...): number  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_multi_varargs_multi_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4142:autocomplete_anonymous_autofilled_multi_varargs_multi_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_multi_varargs_multi_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_multi_varargs_multi_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (string, ...number) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: string, ...: number): (string, number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_multi_varargs_multi_varargs_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4184:autocomplete_anonymous_autofilled_multi_varargs_multi_varargs_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_multi_varargs_multi_varargs_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_multi_varargs_multi_varargs_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (string, ...number) -> (boolean, ...number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: string, ...: number): (boolean, ...number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_multi_varargs_varargs_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4163:autocomplete_anonymous_autofilled_multi_varargs_varargs_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_multi_varargs_varargs_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_multi_varargs_varargs_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (string, ...number) -> ...number)
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: string, ...: number): ...number  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_named_args {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4205:autocomplete_anonymous_autofilled_named_args`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_named_args

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_named_args() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (foo: number, bar: string) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(foo: number, bar: string): (string, number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_noargs_multi_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4100:autocomplete_anonymous_autofilled_noargs_multi_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_noargs_multi_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_noargs_multi_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: () -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(): (string, number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_partially_args {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4226:autocomplete_anonymous_autofilled_partially_args`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_partially_args

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_partially_args() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (number, bar: string) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(a0: number, bar: string): (string, number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_partially_args_last {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4247:autocomplete_anonymous_autofilled_partially_args_last`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_partially_args_last

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_partially_args_last() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (foo: number, string) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(foo: number, a1: string): (string, number)  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_table_literal_args_autocomplete_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4291:autocomplete_anonymous_autofilled_table_literal_args`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_table_literal_args

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_table_literal_args() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (tbl: { x: number, y: number }) -> number) return a({x=2, y = 3}) end
foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(tbl: { x: number, y: number }): number  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_table_literal_args_autocomplete_test_alt_b {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4332:autocomplete_anonymous_autofilled_table_literal_args`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_table_literal_args

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_table_literal_args() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: () -> { x: number, y: number }) return {x=2, y = 3} end
foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(): { x: number, y: number }  end")
    );
  }
}

mod autocomplete_anonymous_autofilled_typeof_args {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4268:autocomplete_anonymous_autofilled_typeof_args`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_typeof_args

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_typeof_args() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = { a = 1, b = 2 }

local function foo(a: (foo: typeof(t)) -> ())
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(entry.insert_text.as_deref(), Some("function(foo)  end"));
  }
}

mod autocomplete_anonymous_autofilled_typeof_returns {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4309:autocomplete_anonymous_autofilled_typeof_returns`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_typeof_returns

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_typeof_returns() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = { a = 1, b = 2 }

local function foo(a: () -> typeof(t))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(entry.insert_text.as_deref(), Some("function()  end"));
  }
}

mod autocomplete_anonymous_autofilled_typeof_vararg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4350:autocomplete_anonymous_autofilled_typeof_vararg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_typeof_vararg

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_typeof_vararg() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = { a = 1, b = 2 }

local function foo(a: (...typeof(t)) -> ())
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(entry.insert_text.as_deref(), Some("function(...)  end"));
  }
}

mod autocomplete_anonymous_autofilled_varargs_multi_return {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4121:autocomplete_anonymous_autofilled_varargs_multi_return`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_anonymous_autofilled_varargs_multi_return

  #[cfg(test)]
  #[test]
  fn autocomplete_anonymous_autofilled_varargs_multi_return() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_entry_kind::AutocompleteEntryKind, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo(a: (...number) -> (string, number))
    a()
end

foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(...: number): (string, number)  end")
    );
  }
}

mod autocomplete_argument_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1437:autocomplete_argument_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_argument_types

  #[cfg(test)]
  #[test]
  fn autocomplete_argument_types() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function f(a: n@1
local b: string = "don't trip"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("nil"));
    assert!(ac.entry_map.contains_key("number"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_arguments_to_global_lambda {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1236:autocomplete_arguments_to_global_lambda`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_arguments_to_global_lambda

  #[cfg(test)]
  #[test]
  fn autocomplete_arguments_to_global_lambda() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        abc = function(def, ghi@1)
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());
  }
}

mod autocomplete_as_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1465:autocomplete_as_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_as_types

  #[cfg(test)]
  #[test]
  fn autocomplete_as_types() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a: any = 5
local b: number = (a :: n@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("nil"));
    assert!(ac.entry_map.contains_key("number"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4477:autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local data = { x = 1 }
local var = data;@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_autocomplete_at_end_of_stmt_should_continue_as_part_of_stmt {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4465:autocomplete_autocomplete_at_end_of_stmt_should_continue_as_part_of_stmt`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_at_end_of_stmt_should_continue_as_part_of_stmt

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_at_end_of_stmt_should_continue_as_part_of_stmt() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local data = { x = 1 }
local var = data.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("x"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_autocomplete_boolean_singleton {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3412:autocomplete_autocomplete_boolean_singleton`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_boolean_singleton

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_boolean_singleton() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_context::AutocompleteContext, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function f(x: true) end
f(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("true"));
    assert_eq!(ac.entry_map["true"].type_correct, TypeCorrectKind::Correct);
    assert!(ac.entry_map.contains_key("false"));
    assert_eq!(ac.entry_map["false"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_default_type_pack_parameters {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3044:autocomplete_autocomplete_default_type_pack_parameters`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_default_type_pack_parameters

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_default_type_pack_parameters() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type A<T... = ...@1> = () -> T
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("number"));
    assert!(ac.entry_map.contains_key("string"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_autocomplete_default_type_parameters {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3031:autocomplete_autocomplete_default_type_parameters`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_default_type_parameters

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_default_type_parameters() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type A<T = @1> = () -> T
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("number"));
    assert!(ac.entry_map.contains_key("string"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_autocomplete_deprecated_attribute {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4963:autocomplete_autocomplete_deprecated_attribute`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_deprecated_attribute
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_deprecated_attribute() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        \@dep@1
        function foo() return 42 end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("deprecated"));
    assert!(ac.entry_map.contains_key("checked"));
    assert!(ac.entry_map.contains_key("native"));
  }
}

mod autocomplete_autocomplete_deprecated_braced_attribute {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4989:autocomplete_autocomplete_deprecated_braced_attribute`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_deprecated_braced_attribute
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_deprecated_braced_attribute() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        \@[dep@1]
        function foo() return 42 end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("deprecated"));
    assert!(ac.entry_map.contains_key("checked"));
    assert!(ac.entry_map.contains_key("native"));
  }
}

mod autocomplete_autocomplete_documentation_symbols {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2781:autocomplete_autocomplete_documentation_symbols`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_documentation_symbols

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_documentation_symbols() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
      r#"
        declare y: {
            x: number,
        }
    "#,
    ));

    fixture.base.check(String::from(
      r#"
        local a = y.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert_eq!(
      ac.entry_map["x"].documentation_symbol,
      Some(String::from("@test/global/y.x"))
    );
  }
}

mod autocomplete_autocomplete_empty_attribute {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4950:autocomplete_autocomplete_empty_attribute`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_empty_attribute
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_empty_attribute() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        \@@1
        function foo() return 42 end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("deprecated"));
    assert!(ac.entry_map.contains_key("checked"));
    assert!(ac.entry_map.contains_key("native"));
  }
}

mod autocomplete_autocomplete_empty_braced_attribute {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4976:autocomplete_autocomplete_empty_braced_attribute`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_empty_braced_attribute
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_empty_braced_attribute() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        \@[@1]
        function foo() return 42 end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("deprecated"));
    assert!(ac.entry_map.contains_key("checked"));
    assert!(ac.entry_map.contains_key("native"));
  }
}

mod autocomplete_autocomplete_end_of_do_block {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1022:autocomplete_autocomplete_end_of_do_block`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_end_of_do_block

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_end_of_do_block() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("do @1"));

    let mut ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("end"));

    fixture.base.check(String::from(
      r#"
        function f()
            do
                @1
        end
        @2
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("end"));

    ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("end"));
  }
}

mod autocomplete_autocomplete_end_with_fn_exprs {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1000:autocomplete_autocomplete_end_with_fn_exprs`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_end_with_fn_exprs

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_end_with_fn_exprs() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function f()  @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_autocomplete_end_with_lambda {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1011:autocomplete_autocomplete_end_with_lambda`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_end_with_lambda

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_end_with_lambda() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local a = function() local bar = foo en@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_autocomplete_exclude_break_continue_expr_func {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4842:autocomplete_autocomplete_exclude_break_continue_expr_func`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_expr_func

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_expr_func() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"while true do
        local _ = function ()
        @1
        end
    end"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("break"));
    assert!(!ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_exclude_break_continue_function_boundary {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4792:autocomplete_autocomplete_exclude_break_continue_function_boundary`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_function_boundary

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_function_boundary() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"for i = 1, 10 do
    local function helper()
        @1
    end
    end"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("break"));
    assert!(!ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_exclude_break_continue_in_incomplete_loop {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4883:autocomplete_autocomplete_exclude_break_continue_in_incomplete_loop`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_in_incomplete_loop

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_in_incomplete_loop() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"while foo() do
        @1"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("break"));
    assert!(!ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_exclude_break_continue_in_param {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4806:autocomplete_autocomplete_exclude_break_continue_in_param`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_in_param

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_in_param() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"while @1 do
        end"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("break"));
    assert!(!ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_exclude_break_continue_incomplete_for {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4827:autocomplete_autocomplete_exclude_break_continue_incomplete_for`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_incomplete_for

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_incomplete_for() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("for @1 in @2 do"));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac1.entry_map.contains_key("break"));
    assert!(!ac1.entry_map.contains_key("continue"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(!ac2.entry_map.contains_key("break"));
    assert!(!ac2.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_exclude_break_continue_incomplete_while {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4817:autocomplete_autocomplete_exclude_break_continue_incomplete_while`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_incomplete_while

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_incomplete_while() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("while @1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("break"));
    assert!(!ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_exclude_break_continue_outside_loop {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4776:autocomplete_autocomplete_exclude_break_continue_outside_loop`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_exclude_break_continue_outside_loop

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_exclude_break_continue_outside_loop() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"@1if true then
        @2
    end"#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac1.entry_map.contains_key("break"));
    assert!(!ac1.entry_map.contains_key("continue"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(!ac2.entry_map.contains_key("break"));
    assert!(!ac2.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_explicit_type_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2969:autocomplete_autocomplete_explicit_type_pack`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_explicit_type_pack

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_explicit_type_pack() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type A<T...> = () -> T...
local a: A<(number, s@1>
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("number"));
    assert!(ac.entry_map.contains_key("string"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_autocomplete_first_function_arg_expected_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2983:autocomplete_autocomplete_first_function_arg_expected_type`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_first_function_arg_expected_type

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_first_function_arg_expected_type() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo1() return 1 end
local function foo2() return "1" end

local function bar0() return "got" .. a end
local function bar1(a: number) return "got " .. a end
local function bar2(a: number, b: string) return "got " .. a .. b end

local t = {}
function t:bar1(a: number) return "got " .. a end

local r1 = bar0(@1)
local r2 = bar1(@2)
local r3 = bar2(@3)
local r4 = t:bar1(@4)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("foo1"));
    assert_eq!(ac1.entry_map["foo1"].type_correct, TypeCorrectKind::None);
    assert!(ac1.entry_map.contains_key("foo2"));
    assert_eq!(ac1.entry_map["foo2"].type_correct, TypeCorrectKind::None);

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac2.entry_map.contains_key("foo1"));
    assert_eq!(
      ac2.entry_map["foo1"].type_correct,
      TypeCorrectKind::CorrectFunctionResult
    );
    assert!(ac2.entry_map.contains_key("foo2"));
    assert_eq!(ac2.entry_map["foo2"].type_correct, TypeCorrectKind::None);

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);

    assert!(ac3.entry_map.contains_key("foo1"));
    assert_eq!(
      ac3.entry_map["foo1"].type_correct,
      TypeCorrectKind::CorrectFunctionResult
    );
    assert!(ac3.entry_map.contains_key("foo2"));
    assert_eq!(ac3.entry_map["foo2"].type_correct, TypeCorrectKind::None);

    let ac4 = fixture.base.autocomplete_marker(b'4' as c_char);

    assert!(ac4.entry_map.contains_key("foo1"));
    assert_eq!(
      ac4.entry_map["foo1"].type_correct,
      TypeCorrectKind::CorrectFunctionResult
    );
    assert!(ac4.entry_map.contains_key("foo2"));
    assert_eq!(ac4.entry_map["foo2"].type_correct, TypeCorrectKind::None);
  }
}

mod autocomplete_autocomplete_for_assignment {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4600:autocomplete_autocomplete_for_assignment`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_for_assignment

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_for_assignment() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function foobar(tbl: { tag: "left" | "right" })
            tbl.tag = "@1"
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("left"));
    assert!(ac.entry_map.contains_key("right"));
  }
}

mod autocomplete_autocomplete_for_in_middle_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:734:autocomplete_autocomplete_for_in_middle_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_for_in_middle_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_for_in_middle_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        for @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac1.entry_map.len());
    assert_eq!(ac1.context, AutocompleteContext::Unknown);

    fixture.base.check(String::from(
      r#"
        for x@1 @2
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac2.entry_map.len());
    assert_eq!(ac2.context, AutocompleteContext::Unknown);

    let ac2a = fixture.base.autocomplete_marker(b'2' as c_char);
    assert_eq!(1, ac2a.entry_map.len());
    assert_eq!(1, ac2a.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(ac2a.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        for x in y@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac3.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(0, ac3.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Expression);

    fixture.base.check(String::from(
      r#"
        for x in y @1
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac4.entry_map.len());
    assert_eq!(1, ac4.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        for x in f f@1
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac5.entry_map.len());
    assert_eq!(1, ac5.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac5.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        for x in y do  @1
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac6.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(ac6.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        for x in y do e@1
    "#,
    ));

    let ac7 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac7.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(1, ac7.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac7.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(1, ac7.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(ac7.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_autocomplete_for_middle_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:646:autocomplete_autocomplete_for_middle_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_for_middle_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_for_middle_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        for x @1=
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac1.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Unknown);

    fixture.base.check(String::from(
      r#"
        for x =@1 1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac2.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Unknown);

    fixture.base.check(String::from(
      r#"
        for x = 1,@1 2
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac3.entry_map.len());
    assert_eq!(1, ac3.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        for x = 1, @12,
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac4.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac4.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Expression);

    fixture.base.check(String::from(
      r#"
        for x = 1, 2, @15
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac5.entry_map.get("math").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac5.context, AutocompleteContext::Expression);

    fixture.base.check(String::from(
      r#"
        for x = 1, 2, 5 f@1
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac6.entry_map.len());
    assert_eq!(1, ac6.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac6.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        for x = 1, 2, 5 do      @1
    "#,
    ));

    let ac7 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac7.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac7.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"local Foo = 1
        for x = @11, @22, @35
    "#,
    ));

    for i in 0..3 {
      let marker = (b'1' + i) as c_char;
      let ac8 = fixture.base.autocomplete_marker(marker);
      assert_eq!(1, ac8.entry_map.get("Foo").map_or(0, |_| 1));
      assert_eq!(0, ac8.entry_map.get("do").map_or(0, |_| 1));
    }

    fixture.base.check(String::from(
      r#"local Foo = 1
        for x = @11, @22
    "#,
    ));

    for i in 0..2 {
      let marker = (b'1' + i) as c_char;
      let ac9 = fixture.base.autocomplete_marker(marker);
      assert_eq!(1, ac9.entry_map.get("Foo").map_or(0, |_| 1));
      assert_eq!(0, ac9.entry_map.get("do").map_or(0, |_| 1));
    }
  }
}

mod autocomplete_autocomplete_if_else_regression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2884:autocomplete_autocomplete_if_else_regression`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_if_else_regression

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_if_else_regression() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local abcdef = 0;
local temp = false
local even = true;
local a
a = if temp then even else@1
a = if temp then even else @2
a = if temp then even else abc@3
        "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac1.entry_map.contains_key("else"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(!ac2.entry_map.contains_key("else"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac3.entry_map.contains_key("abcdef"));
  }
}

mod autocomplete_autocomplete_if_middle_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:858:autocomplete_autocomplete_if_middle_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_if_middle_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_if_middle_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        if   @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac1.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(1, ac1.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Expression);

    fixture.base.check(String::from(
      r#"
        if x  @1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac2.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        if x t@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(3, ac3.entry_map.len());
    assert_eq!(1, ac3.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac3.entry_map.get("and").map_or(0, |_| 1));
    assert_eq!(1, ac3.entry_map.get("or").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        if x then
@1
        end
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac4.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac4.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        if x then
            t@1
        end
    "#,
    ));

    let ac4a = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac4a.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac4a.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac4a.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(1, ac4a.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(ac4a.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        if x then
@1
        elseif x then
        end
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac5.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac5.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac5.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        if t@1
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac6.entry_map.get("true").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("false").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac6.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_ifelse_expressions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2799:autocomplete_autocomplete_ifelse_expressions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_ifelse_expressions

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_ifelse_expressions() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local temp = false
local even = true;
local a = true
a = if t@1emp then t
a = if temp t@2
a = if temp then e@3
a = if temp then even e@4
a = if temp then even elseif t@5
a = if temp then even elseif true t@6
a = if temp then even elseif true then t@7
a = if temp then even elseif true then temp e@8
a = if temp then even elseif true then temp else e@9
        "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("temp"));
    assert!(ac1.entry_map.contains_key("true"));
    assert!(!ac1.entry_map.contains_key("then"));
    assert!(!ac1.entry_map.contains_key("else"));
    assert!(!ac1.entry_map.contains_key("elseif"));
    assert_eq!(ac1.context, AutocompleteContext::Expression);

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(!ac2.entry_map.contains_key("temp"));
    assert!(!ac2.entry_map.contains_key("true"));
    assert!(ac2.entry_map.contains_key("then"));
    assert!(!ac2.entry_map.contains_key("else"));
    assert!(!ac2.entry_map.contains_key("elseif"));
    assert_eq!(ac2.context, AutocompleteContext::Keyword);

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac3.entry_map.contains_key("even"));
    assert!(!ac3.entry_map.contains_key("then"));
    assert!(!ac3.entry_map.contains_key("else"));
    assert!(!ac3.entry_map.contains_key("elseif"));
    assert_eq!(ac3.context, AutocompleteContext::Expression);

    let ac4 = fixture.base.autocomplete_marker(b'4' as c_char);
    assert!(!ac4.entry_map.contains_key("even"));
    assert!(!ac4.entry_map.contains_key("then"));
    assert!(ac4.entry_map.contains_key("else"));
    assert!(ac4.entry_map.contains_key("elseif"));
    assert_eq!(ac4.context, AutocompleteContext::Keyword);

    let ac5 = fixture.base.autocomplete_marker(b'5' as c_char);
    assert!(ac5.entry_map.contains_key("temp"));
    assert!(ac5.entry_map.contains_key("true"));
    assert!(!ac5.entry_map.contains_key("then"));
    assert!(!ac5.entry_map.contains_key("else"));
    assert!(!ac5.entry_map.contains_key("elseif"));
    assert_eq!(ac5.context, AutocompleteContext::Expression);

    let ac6 = fixture.base.autocomplete_marker(b'6' as c_char);
    assert!(!ac6.entry_map.contains_key("temp"));
    assert!(!ac6.entry_map.contains_key("true"));
    assert!(ac6.entry_map.contains_key("then"));
    assert!(!ac6.entry_map.contains_key("else"));
    assert!(!ac6.entry_map.contains_key("elseif"));
    assert_eq!(ac6.context, AutocompleteContext::Keyword);

    let ac7 = fixture.base.autocomplete_marker(b'7' as c_char);
    assert!(ac7.entry_map.contains_key("temp"));
    assert!(ac7.entry_map.contains_key("true"));
    assert!(!ac7.entry_map.contains_key("then"));
    assert!(!ac7.entry_map.contains_key("else"));
    assert!(!ac7.entry_map.contains_key("elseif"));
    assert_eq!(ac7.context, AutocompleteContext::Expression);

    let ac8 = fixture.base.autocomplete_marker(b'8' as c_char);
    assert!(!ac8.entry_map.contains_key("even"));
    assert!(!ac8.entry_map.contains_key("then"));
    assert!(ac8.entry_map.contains_key("else"));
    assert!(ac8.entry_map.contains_key("elseif"));
    assert_eq!(ac8.context, AutocompleteContext::Keyword);

    let ac9 = fixture.base.autocomplete_marker(b'9' as c_char);
    assert!(!ac9.entry_map.contains_key("then"));
    assert!(!ac9.entry_map.contains_key("else"));
    assert!(!ac9.entry_map.contains_key("elseif"));
    assert_eq!(ac9.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_implicit_named_index_index_expr {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4652:autocomplete_autocomplete_implicit_named_index_index_expr`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Constraint (Analysis/include/Luau/Constraint.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_implicit_named_index_index_expr
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_implicit_named_index_index_expr() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type Constraint = "A" | "B" | "C"
        local foo : { [Constraint]: string } = {
            A = "Value for A",
            B = "Value for B",
            C = "Value for C",
        }
        foo["@1"]
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("A"));
    assert_eq!(ac.entry_map["A"].kind, AutocompleteEntryKind::String);
    assert!(ac.entry_map.contains_key("B"));
    assert_eq!(ac.entry_map["B"].kind, AutocompleteEntryKind::String);
    assert!(ac.entry_map.contains_key("C"));
    assert_eq!(ac.entry_map["C"].kind, AutocompleteEntryKind::String);
  }
}

mod autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4676:autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::{
      enums::autocomplete_entry_kind::AutocompleteEntryKind,
      functions::to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local foo = {
            ["Item/Foo"] = 42,
            ["Item/Bar"] = "it's true",
            ["Item/Baz"] = true,
        }
        foo["@1"]
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    for (key, expected_type) in [
      ("Item/Foo", "number"),
      ("Item/Bar", "string"),
      ("Item/Baz", "boolean"),
    ] {
      assert!(ac.entry_map.contains_key(key));
      let entry = &ac.entry_map[key];
      assert_eq!(entry.kind, AutocompleteEntryKind::Property);
      let ty = entry.r#type.expect("autocomplete entry should have a type");
      assert_eq!(expected_type, to_string_type_id(ty));
    }
  }
}

mod autocomplete_autocomplete_in_local_table {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4613:autocomplete_autocomplete_in_local_table`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_in_local_table

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_in_local_table() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type Entry = { field: number, prop: string }
        local x : {Entry} = {}
        x[1] = {
           f@1,
           p@2,
        }

        local t : { key1: boolean, thing2: CFrame, aaa3: vector } = {
            k@3,
            th@4,
        }
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("field"));
    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac2.entry_map.contains_key("prop"));
    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac3.entry_map.contains_key("key1"));
    let ac4 = fixture.base.autocomplete_marker(b'4' as c_char);
    assert!(ac4.entry_map.contains_key("thing2"));
  }
}

mod autocomplete_autocomplete_in_type_assertion {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4639:autocomplete_autocomplete_in_type_assertion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_in_type_assertion

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_in_type_assertion() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type Entry = { field: number, prop: string }
        return ( { f@1, p@2 } :: Entry )
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("field"));
    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac2.entry_map.contains_key("prop"));
  }
}

mod autocomplete_autocomplete_include_break_continue_in_loop {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4756:autocomplete_autocomplete_include_break_continue_in_loop`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_include_break_continue_in_loop

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_include_break_continue_in_loop() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"for x in y do
        @1
        if true then
            @2
        end
    end"#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("break"));
    assert!(ac1.entry_map.contains_key("continue"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac2.entry_map.contains_key("break"));
    assert!(ac2.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_include_break_continue_in_nests {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4868:autocomplete_autocomplete_include_break_continue_in_nests`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_include_break_continue_in_nests

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_include_break_continue_in_nests() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"while ((function ()
        while true do
            @1
        end
        end)()) do
    end"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("break"));
    assert!(ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_include_break_continue_in_repeat {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4856:autocomplete_autocomplete_include_break_continue_in_repeat`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_include_break_continue_in_repeat

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_include_break_continue_in_repeat() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"repeat
        @1
    until foo()"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("break"));
    assert!(ac.entry_map.contains_key("continue"));
  }
}

mod autocomplete_autocomplete_interpolated_string_as_singleton {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2950:autocomplete_autocomplete_interpolated_string_as_singleton`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_interpolated_string_as_singleton

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_interpolated_string_as_singleton() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        --!strict
        local function f(a: "cat" | "dog") end

        f(`@1`)
        f(`uhhh{'try'}@2`)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("cat"));
    assert_eq!(ac1.context, AutocompleteContext::String);

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac2.entry_map.is_empty());
    assert_eq!(ac2.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_interpolated_string_constant {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2904:autocomplete_autocomplete_interpolated_string_constant`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_interpolated_string_constant

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_interpolated_string_constant() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();

    fixture.base.check(String::from(r#"f(`@1`)"#));
    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.is_empty());
    assert_eq!(ac1.context, AutocompleteContext::String);

    fixture.base.check(String::from(r#"f(`@1 {"a"}`)"#));
    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac2.entry_map.is_empty());
    assert_eq!(ac2.context, AutocompleteContext::String);

    fixture.base.check(String::from(r#"f(`{"a"} @1`)"#));
    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac3.entry_map.is_empty());
    assert_eq!(ac3.context, AutocompleteContext::String);

    fixture.base.check(String::from(r#"f(`{"a"} @1 {"b"}`)"#));
    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac4.entry_map.is_empty());
    assert_eq!(ac4.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_interpolated_string_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2927:autocomplete_autocomplete_interpolated_string_expression`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_interpolated_string_expression

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_interpolated_string_expression() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture
      .base
      .check(String::from(r#"f(`expression = {@1}`)"#));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("table"));
    assert_eq!(ac.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_interpolated_string_expression_with_comments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2935:autocomplete_autocomplete_interpolated_string_expression_with_comments`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_interpolated_string_expression_with_comments

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_interpolated_string_expression_with_comments() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();

    fixture
      .base
      .check(String::from(r#"f(`expression = {--[[ bla bla bla ]]@1`)"#));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("table"));
    assert_eq!(ac1.context, AutocompleteContext::Expression);

    fixture
      .base
      .check(String::from(r#"f(`expression = {@1 --[[ bla bla bla ]]`)"#));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac2.entry_map.is_empty());
    assert!(ac2.entry_map.contains_key("table"));
    assert_eq!(ac2.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5176:autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"

local t0 = { thing = 5 }

type function evil(x)
    local tbl = types.newtable(nil, nil, nil)
    tbl:setwriteproperty(types.singleton("__index"), types.any)
    return tbl
end

type BadMTType = evil<{ thing : number}>
local function foo(t : BadMTType)
        local t2 = setmetatable({}, t)
        return t2
end

local x = foo(nil :: any)
x.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());
  }
}

mod autocomplete_autocomplete_method_in_unfinished_repeat_body_eof {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4910:autocomplete_autocomplete_method_in_unfinished_repeat_body_eof`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_method_in_unfinished_repeat_body_eof
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_method_in_unfinished_repeat_body_eof() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"local t = {}
        function t:Foo() end
        repeat
        t:@1"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("Foo"));
  }
}

mod autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4923:autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"local t = {}
        function t:Foo() end
        repeat
        t:@1
        "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("Foo"));
  }
}

mod autocomplete_autocomplete_method_in_unfinished_while_body {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4937:autocomplete_autocomplete_method_in_unfinished_while_body`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_method_in_unfinished_while_body
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_method_in_unfinished_while_body() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"local t = {}
        function t:Foo() end
        while true do
        t:@1"#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("Foo"));
  }
}

mod autocomplete_autocomplete_on_string_singletons {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3082:autocomplete_autocomplete_on_string_singletons`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function format (tests/StringUtils.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_on_string_singletons

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_on_string_singletons() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        --!strict
        local foo: "hello" | "bye" = "hello"
        foo:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("format"));
  }
}

mod autocomplete_autocomplete_oop_implicit_self {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3057:autocomplete_autocomplete_oop_implicit_self`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_oop_implicit_self

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_oop_implicit_self() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
--!strict
local Class = {}
Class.__index = Class
type Class = typeof(setmetatable({} :: { x: number }, Class))
function Class.new(x: number): Class
    return setmetatable({x = x}, Class)
end
function Class.getx(self: Class)
    return self.x
end
function test()
    local c = Class.new(42)
    local n = c:@1
    print(n)
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("getx"));
  }
}

mod autocomplete_autocomplete_prop_index_function_metamethod_is_variadic {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2472:autocomplete_autocomplete_prop_index_function_metamethod_is_variadic`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_prop_index_function_metamethod_is_variadic

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_prop_index_function_metamethod_is_variadic() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback,
      records::ac_builtins_fixture::ACBuiltinsFixture,
    };

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        type Foo = {x: number}
        local t = {}
        setmetatable(t, {
            __index = function(index: string): ...Foo
                return {x = 1}, {x = 2}
            end
        })

        local a = t. -- Line 9
        --          | Column 20
    "#,
      ),
    );

    let module = String::from("Module/A");
    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module,
        Position {
          line: 9,
          column: 20,
        },
        Box::new(null_callback),
      );

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("x"));
  }
}

mod autocomplete_autocomplete_react {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5221:autocomplete_autocomplete_react`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum State (Analysis/src/TypePath.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_react
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_react() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
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
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("foobar"));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("bazquxx"));

    let ac = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac.entry_map.contains_key("barbaz"));
  }
}

mod autocomplete_autocomplete_repeat_middle_keyword {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1060:autocomplete_autocomplete_repeat_middle_keyword`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_repeat_middle_keyword

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_repeat_middle_keyword() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        repeat @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac1.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(1, ac1.entry_map.get("until").map_or(0, |_| 1));

    fixture.base.check(String::from(
      r#"
        repeat f f@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac2.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(1, ac2.entry_map.get("until").map_or(0, |_| 1));

    fixture.base.check(String::from(
      r#"
        repeat
            u@1
        until
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac3.entry_map.get("until").map_or(0, |_| 1));
  }
}

mod autocomplete_autocomplete_response_perf_1 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3865:autocomplete_autocomplete_response_perf_1`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function formatAppend (Common/src/StringUtils.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_response_perf_1
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_response_perf_1() {
    use alloc::{format, string::String};
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let parts = 100;
    let mut source = String::new();

    for i in 0..parts {
      source.push_str(&format!("type T{} = {{ f{}: number }}\n", i, i));
    }

    source.push_str("type Instance = { new: (('s0', extra: Instance?) -> T0)");

    for i in 1..parts {
      source.push_str(&format!(" & (('s{}', extra: Instance?) -> T{})", i, i));
    }

    source.push_str(" }\n");
    source.push_str("local Instance: Instance = {} :: any\n");
    source.push_str("local function c(): boolean return t@1 end\n");

    let mut fixture = AcFixture::default();
    fixture.base.check(&source);

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("true"));
    assert!(ac.entry_map.contains_key("Instance"));
  }
}

mod autocomplete_autocomplete_string_singleton_disjoint_intersection_arg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5130:autocomplete_autocomplete_string_singleton_disjoint_intersection_arg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_disjoint_intersection_arg
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singleton_disjoint_intersection_arg() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _intersection =
      ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function f(_: "foo"&"baz") end
        f("@1")
        f(@2)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("foo"));
    assert!(ac.entry_map.contains_key("baz"));
    assert_eq!(ac.context, AutocompleteContext::String);

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("\"foo\""));
    assert!(ac.entry_map.contains_key("\"baz\""));
    assert_eq!(ac.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_string_singleton_equality {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3391:autocomplete_autocomplete_string_singleton_equality`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_equality

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singleton_equality() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type tagged = {tag:"cat", fieldx:number} | {tag:"dog", fieldy:number}
        local x: tagged = {tag="cat", fieldx=2}
        if x.tag == "@1" or "@2" ~= x.tag then end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("cat"));
    assert!(ac.entry_map.contains_key("dog"));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac.entry_map.contains_key("cat"));
    assert!(ac.entry_map.contains_key("dog"));
  }
}

mod autocomplete_autocomplete_string_singleton_escape {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3428:autocomplete_autocomplete_string_singleton_escape`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_escape

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singleton_escape() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type tag = "strange\t\"cat\"" | 'nice\t"dog"'
        local function f(x: tag) end
        f(@1)
        f("@2")
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("\"strange\\t\\\"cat\\\"\""));
    assert!(ac.entry_map.contains_key("\"nice\\t\\\"dog\\\"\""));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac.entry_map.contains_key("strange\\t\\\"cat\\\""));
    assert!(ac.entry_map.contains_key("nice\\t\\\"dog\\\""));
  }
}

mod autocomplete_autocomplete_string_singleton_intersection_multiple {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5089:autocomplete_autocomplete_string_singleton_intersection_multiple`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_intersection_multiple
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singleton_intersection_multiple() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function C(_: "Example"&"Example") end
        C("@1")
        C(@2)
        local x: "Example"&"Example" = "@3"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("Example"));
    assert_eq!(ac.context, AutocompleteContext::String);

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("\"Example\""));
    assert_eq!(ac.context, AutocompleteContext::Expression);

    let ac = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac.entry_map.contains_key("Example"));
    assert_eq!(ac.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_string_singleton_intersection_variable {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5076:autocomplete_autocomplete_string_singleton_intersection_variable`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_intersection_variable
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singleton_intersection_variable() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local _: "cat"&"cat" = "@1"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("cat"));
    assert_eq!(ac.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_string_singleton_keyof_intersection {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5154:autocomplete_autocomplete_string_singleton_keyof_intersection`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singleton_keyof_intersection
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singleton_keyof_intersection() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _intersection =
      ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        local foo = {
            Element1 = "Value1",
            Element2 = "Value2",
        }
        local function bar<T>(key: keyof<typeof(foo)>&T) end
        bar("@1")
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("Element1"));
    assert!(ac.entry_map.contains_key("Element2"));
    assert_eq!(ac.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_string_singletons {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3116:autocomplete_autocomplete_string_singletons`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singletons

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singletons() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type tag = "cat" | "dog"
        local function f(a: tag) end
        f("@1")
        f(@2)
        local x: tag = "@3"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("cat"));
    assert!(ac.entry_map.contains_key("dog"));
    assert_eq!(ac.context, AutocompleteContext::String);

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac.entry_map.contains_key("\"cat\""));
    assert!(ac.entry_map.contains_key("\"dog\""));
    assert_eq!(ac.context, AutocompleteContext::Expression);

    let ac = fixture.base.autocomplete_marker(b'3' as c_char);

    assert!(ac.entry_map.contains_key("cat"));
    assert!(ac.entry_map.contains_key("dog"));
    assert_eq!(ac.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_string_singletons_in_intersection {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5113:autocomplete_autocomplete_string_singletons_in_intersection`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singletons_in_intersection
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singletons_in_intersection() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _intersection =
      ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local _: "foo"&"baz" = "@1"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("foo"));
    assert!(ac.entry_map.contains_key("baz"));
    assert_eq!(ac.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_string_singletons_in_literal {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3095:autocomplete_autocomplete_string_singletons_in_literal`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> function fail (Config/src/Config.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_string_singletons_in_literal

  use ulua_common::FFlag;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_string_singletons_in_literal() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type tagged = {tag:"cat", fieldx:number} | {tag:"dog", fieldy:number}
        local x: tagged = {tag="@1"}
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("cat"));
    assert!(ac.entry_map.contains_key("dog"));
    assert_eq!(ac.context, AutocompleteContext::String);
  }
}

mod autocomplete_autocomplete_subtyping_recursion_limit {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3895:autocomplete_autocomplete_subtyping_recursion_limit`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function formatAppend (Common/src/StringUtils.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_subtyping_recursion_limit
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_subtyping_recursion_limit() {
    use alloc::{format, string::String};
    use core::ffi::c_char;

    use ulua_common::{DFInt, FFlag, FInt};
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_int::ScopedFastInt,
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let _type_infer_recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 10);
    let _subtyping_recursion_limit = ScopedFastInt::new(&DFInt::LuauSubtypingRecursionLimit, 10);

    let parts = 100;
    let mut source = String::new();

    source.push_str("function f()\n");

    let mut prefix = String::new();
    for i in 0..parts {
      prefix.push_str(&format!("(nil|({{a{}:number}}&", i));
    }
    prefix.push_str(&format!("(nil|{{a{}:number}})", parts));
    for _ in 0..parts {
      prefix.push_str("))");
    }

    source.push_str("local x1 : ");
    source.push_str(&prefix);
    source.push('\n');
    source.push_str("local y : {a1:number} = x@1\n");
    source.push_str("end\n");

    let mut fixture = AcFixture::default();
    fixture.base.check(&source);

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("true"));
    assert!(ac.entry_map.contains_key("x1"));
  }
}

mod autocomplete_autocomplete_suggest_hot_comments {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4895:autocomplete_autocomplete_suggest_hot_comments`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_suggest_hot_comments
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_suggest_hot_comments() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("--!@1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("strict"));
    assert!(ac.entry_map.contains_key("nonstrict"));
    assert!(ac.entry_map.contains_key("nocheck"));
    assert!(ac.entry_map.contains_key("native"));
    assert!(ac.entry_map.contains_key("nolint"));
    assert!(ac.entry_map.contains_key("optimize"));
  }
}

mod autocomplete_autocomplete_table_insert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5207:autocomplete_autocomplete_table_insert`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_table_insert
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_table_insert() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, { f@1 })
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("foobar"));
  }
}

mod autocomplete_autocomplete_until_expression {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:968:autocomplete_autocomplete_until_expression`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_until_expression

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_until_expression() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        repeat
        until   @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(ac.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_autocomplete_until_in_repeat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:956:autocomplete_autocomplete_until_in_repeat`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_until_in_repeat

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_until_in_repeat() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        repeat  @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac.entry_map.get("until").map_or(0, |_| 1));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_autocomplete_using_function_with_singleton_arg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5040:autocomplete_autocomplete_using_function_with_singleton_arg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_using_function_with_singleton_arg
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_using_function_with_singleton_arg() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function foo(...: "Val1") end
        foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("\"Val1\""));
  }
}

mod autocomplete_autocomplete_using_function_with_singleton_intersection_arg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5063:autocomplete_autocomplete_using_function_with_singleton_intersection_arg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_using_function_with_singleton_intersection_arg
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_using_function_with_singleton_intersection_arg() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauAutocompleteStringSingletonIntersection, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function foo(_: "Val1"&"Val1") end
        foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("\"Val1\""));
  }
}

mod autocomplete_autocomplete_using_function_with_singleton_union_arg {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5051:autocomplete_autocomplete_using_function_with_singleton_union_arg`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_using_function_with_singleton_union_arg
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_using_function_with_singleton_union_arg() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function foo(...: "Val1" | "Val2") end
        foo(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("\"Val1\""));
    assert!(ac.entry_map.contains_key("\"Val2\""));
  }
}

mod autocomplete_autocomplete_using_indexer_with_singleton_keys {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5002:autocomplete_autocomplete_using_indexer_with_singleton_keys`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_autocomplete_using_indexer_with_singleton_keys
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_using_indexer_with_singleton_keys() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("Val1"));
    assert!(ac.entry_map.contains_key("Val2"));
    assert!(ac.entry_map.contains_key("Val3"));
  }
}

mod autocomplete_autocomplete_via_bidirectional_self {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4720:autocomplete_autocomplete_via_bidirectional_self`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_via_bidirectional_self
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_via_bidirectional_self() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
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
            print("My balance is: " .. self.@1)
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("name"));
    assert!(ac.entry_map.contains_key("balance"));
  }
}

mod autocomplete_autocomplete_while_middle_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:807:autocomplete_autocomplete_while_middle_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_autocomplete_while_middle_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_autocomplete_while_middle_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        while@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac1.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Expression);

    fixture.base.check(String::from(
      r#"
        while true @1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(3, ac2.entry_map.len());
    assert_eq!(1, ac2.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(1, ac2.entry_map.get("and").map_or(0, |_| 1));
    assert_eq!(1, ac2.entry_map.get("or").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        while true do  @1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac3.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        while true d@1
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(3, ac4.entry_map.len());
    assert_eq!(1, ac4.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("and").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("or").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Keyword);

    fixture.base.check(String::from(
      r#"
        while t@1
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(0, ac5.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(1, ac5.entry_map.get("true").map_or(0, |_| 1));
    assert_eq!(1, ac5.entry_map.get("false").map_or(0, |_| 1));
  }
}

mod autocomplete_bias_toward_inner_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:554:autocomplete_bias_toward_inner_scope`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item autocomplete_bias_toward_inner_scope

  #[cfg(test)]
  #[test]
  fn autocomplete_bias_toward_inner_scope() {
    use core::ffi::c_char;

    use ulua_analysis::{
      enums::autocomplete_context::AutocompleteContext,
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local A = {one=1}

        function B()
            local A = {two=2}

            A  @1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("A"));
    assert_eq!(ac.context, AutocompleteContext::Statement);

    let ty = ac.entry_map["A"]
      .r#type
      .expect("A entry should have a type");
    let ty = follow_type_id(ty);
    let table = get_type_id::<TableType>(ty).expect("A should be a table");
    assert!(table.props.contains_key("two"));
  }
}

mod autocomplete_bidirectional_autocomplete_in_function_call {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4705:autocomplete_bidirectional_autocomplete_in_function_call`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_bidirectional_autocomplete_in_function_call
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_bidirectional_autocomplete_in_function_call() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function take(_: { choice: "left" | "right" }) end

        take({ choice = "@1" })
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("left"));
    assert!(ac.entry_map.contains_key("right"));
  }
}

mod autocomplete_class_autocomplete_classname_inside_method_autocomplete_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5302:autocomplete_class_autocomplete_classname_inside_method`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_class_autocomplete_classname_inside_method
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_class_autocomplete_classname_inside_method() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        class Bar
            function new()
                return Bar {}
            end
            function hmm(self)
                self:h@2
            end
        end

        class Bar
            function make()
                return Bar {}
            end
            function huh(self)
                self:h@3
            end
        end

        Bar.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("new"));
    assert!(!ac.entry_map.contains_key("make"));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("hmm"));
    assert!(!ac.entry_map.contains_key("huh"));

    let ac = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(!ac.entry_map.contains_key("huh"));
    assert!(!ac.entry_map.contains_key("hmm"));
  }
}

mod autocomplete_class_autocomplete_classname_inside_method_autocomplete_test_alt_b {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5347:autocomplete_class_autocomplete_classname_inside_method`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - translates_to -> rust_item autocomplete_class_autocomplete_classname_inside_method
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_class_autocomplete_classname_inside_method() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        class Bar
            public value: number
            function new()
                return B@1
            end
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("Bar"));
  }
}

mod autocomplete_cli_197197_autocomplete_generic_keyof {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5261:autocomplete_cli_197197_autocomplete_generic_keyof`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item autocomplete_cli_197197_autocomplete_generic_keyof
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_cli_197197_autocomplete_generic_keyof() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        local function ToggleButton<T>(Table: T, Key: keyof<T>)
            -- don't need to do anything here.
        end

        local tbl: { Changed: bool, RemoveTag: bool } = nil :: any

        ToggleButton(tbl, "@1")
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("Changed"));
    assert!(ac.entry_map.contains_key("RemoveTag"));
  }
}

mod autocomplete_comments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2464:autocomplete_comments`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_comments

  #[cfg(test)]
  #[test]
  fn autocomplete_comments() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture
      .base
      .base
      .file_resolver
      .source
      .insert(String::from("Comments"), String::from("--foo"));

    let module = String::from("Comments");
    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module,
        Position { line: 0, column: 5 },
        Box::new(null_callback),
      );

    assert_eq!(0, ac.entry_map.len());
  }
}

mod autocomplete_cyclic_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:418:autocomplete_cyclic_table`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_cyclic_table

  #[cfg(test)]
  #[test]
  fn autocomplete_cyclic_table() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local abc = {}
        local def = { abc = abc }
        abc.def = def
        abc.def. @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("abc"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_do_compatible_self_calls {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3519:autocomplete_do_compatible_self_calls`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_do_compatible_self_calls

  #[cfg(test)]
  #[test]
  fn autocomplete_do_compatible_self_calls() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
function t:m() end
t:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("m"));
    assert!(!ac.entry_map["m"].wrong_index_type);
    assert!(ac.entry_map["m"].indexed_with_self);
  }
}

mod autocomplete_do_not_overwrite_context_sensitive_kws {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:591:autocomplete_do_not_overwrite_context_sensitive_kws`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record AutocompleteEntry (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_do_not_overwrite_context_sensitive_kws

  #[cfg(test)]
  #[test]
  fn autocomplete_do_not_overwrite_context_sensitive_kws() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function continue()
        end


@1    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    let entry = &ac.entry_map["continue"];

    assert_eq!(entry.kind, AutocompleteEntryKind::Binding);
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_do_not_suggest_internal_module_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2068:autocomplete_do_not_suggest_internal_module_type`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_do_not_suggest_internal_module_type

  #[cfg(test)]
  #[test]
  fn autocomplete_do_not_suggest_internal_module_type() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
type done = { x: number, y: number }
local function a(a: (done) -> number) return a({x=1, y=2}) end
local function b(a: ((done) -> number) -> number) return a(function(done) return 1 end) end
return {a = a, b = b}
    "#,
      ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_a, None);
    assert!(result.errors.is_empty());

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local ex = require(script.Parent.A)
ex.a(function(x:
    "#,
      ),
    );

    let module_b = String::from("Module/B");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);

    let ac1 = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_b,
        Position {
          line: 2,
          column: 16,
        },
        Box::new(null_callback),
      );

    assert!(!ac1.entry_map.contains_key("done"));

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/C"),
      String::from(
        r#"
local ex = require(script.Parent.A)
ex.b(function(x:
    "#,
      ),
    );

    let module_c = String::from("Module/C");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_c, None);

    let ac2 = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_c,
        Position {
          line: 2,
          column: 16,
        },
        Box::new(null_callback),
      );

    assert!(!ac2.entry_map.contains_key("(done) -> number"));
  }
}

mod autocomplete_do_not_suggest_synthetic_table_name {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2140:autocomplete_do_not_suggest_synthetic_table_name`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_do_not_suggest_synthetic_table_name

  #[cfg(test)]
  #[test]
  fn autocomplete_do_not_suggest_synthetic_table_name() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local foo = { a = 1, b = 2 }
local bar: @1= foo
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(!ac.entry_map.contains_key("foo"));
  }
}

mod autocomplete_do_wrong_compatible_nonself_calls {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3581:autocomplete_do_wrong_compatible_nonself_calls`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item autocomplete_do_wrong_compatible_nonself_calls
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_do_wrong_compatible_nonself_calls() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
function t:m(x: string) end
t.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("m"));
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(ac.entry_map["m"].wrong_index_type);
    } else {
      assert!(!ac.entry_map["m"].wrong_index_type);
    }
    assert!(!ac.entry_map["m"].indexed_with_self);
  }
}

mod autocomplete_do_wrong_compatible_self_calls {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3565:autocomplete_do_wrong_compatible_self_calls`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_do_wrong_compatible_self_calls

  #[cfg(test)]
  #[test]
  fn autocomplete_do_wrong_compatible_self_calls() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
function t.m(x: typeof(t)) end
t:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("m"));
    assert!(!ac.entry_map["m"].wrong_index_type);
    assert!(ac.entry_map["m"].indexed_with_self);
  }
}

mod autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:625:autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment

  #[cfg(test)]
  #[test]
  fn autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        --[[ @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(0, ac.entry_map.len());
    assert_eq!(ac.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment_at_the_very_end_of_the_file {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:637:autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment_at_the_very_end_of_the_file`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment_at_the_very_end_of_the_file

  #[cfg(test)]
  #[test]
  fn autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment_at_the_very_end_of_the_file()
   {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("--[[@1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(0, ac.entry_map.len());
    assert_eq!(ac.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_dont_offer_any_suggestions_from_within_a_comment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:607:autocomplete_dont_offer_any_suggestions_from_within_a_comment`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_dont_offer_any_suggestions_from_within_a_comment

  #[cfg(test)]
  #[test]
  fn autocomplete_dont_offer_any_suggestions_from_within_a_comment() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:@1
        ]]
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(0, ac.entry_map.len());
    assert_eq!(ac.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_dont_suggest_local_before_its_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:237:autocomplete_dont_suggest_local_before_its_definition`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_dont_suggest_local_before_its_definition

  #[cfg(test)]
  #[test]
  fn autocomplete_dont_suggest_local_before_its_definition() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local myLocal = 4
        function abc()
@1            local myInnerLocal = 1
@2
        end
@3    "#,
    ));
    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("myLocal"));
    assert!(!ac.entry_map.contains_key("myInnerLocal"));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("myLocal"));
    assert!(ac.entry_map.contains_key("myInnerLocal"));

    let ac = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac.entry_map.contains_key("myLocal"));
    assert!(!ac.entry_map.contains_key("myInnerLocal"));
  }
}

mod autocomplete_empty_program {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:194:autocomplete_empty_program`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_empty_program

  #[cfg(test)]
  #[test]
  fn autocomplete_empty_program() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(" @1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_function_expr_params {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1247:autocomplete_function_expr_params`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_function_expr_params

  #[cfg(test)]
  #[test]
  fn autocomplete_function_expr_params() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        abc = function(def) @1
    "#,
    ));

    for i in 20..27 {
      assert!(
        fixture
          .base
          .autocomplete_position(1, i)
          .entry_map
          .is_empty()
      );
    }
    assert!(
      !fixture
        .base
        .autocomplete_marker(b'1' as c_char)
        .entry_map
        .is_empty()
    );

    fixture.base.check(String::from(
      r#"
        abc = function(def) @1
        end
    "#,
    ));

    for i in 20..27 {
      assert!(
        fixture
          .base
          .autocomplete_position(1, i)
          .entry_map
          .is_empty()
      );
    }
    assert!(
      !fixture
        .base
        .autocomplete_marker(b'1' as c_char)
        .entry_map
        .is_empty()
    );

    fixture.base.check(String::from(
      r#"
        abc = function(def)
@1
        end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_function_in_assignment_has_parentheses {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2170:autocomplete_function_in_assignment_has_parentheses`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_function_in_assignment_has_parentheses

  #[cfg(test)]
  #[test]
  fn autocomplete_function_in_assignment_has_parentheses() {
    use core::ffi::c_char;

    use ulua_analysis::enums::parentheses_recommendation::ParenthesesRecommendation;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function bar(a: number) return -a end
local abc = b@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("bar"));
    assert_eq!(
      ac.entry_map["bar"].parens,
      ParenthesesRecommendation::CursorInside
    );
  }
}

mod autocomplete_function_in_assignment_has_parentheses_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3448:autocomplete_function_in_assignment_has_parentheses_2`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_function_in_assignment_has_parentheses_2

  #[cfg(test)]
  #[test]
  fn autocomplete_function_in_assignment_has_parentheses_2() {
    use core::ffi::c_char;

    use ulua_analysis::enums::parentheses_recommendation::ParenthesesRecommendation;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local bar: ((number) -> number) & (number, number) -> number)
local abc = b@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("bar"));
    assert_eq!(
      ac.entry_map["bar"].parens,
      ParenthesesRecommendation::CursorInside
    );
  }
}

mod autocomplete_function_parameters {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:347:autocomplete_function_parameters`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item autocomplete_function_parameters

  #[cfg(test)]
  #[test]
  fn autocomplete_function_parameters() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        function abc(test)

@1        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("test"));
  }
}

mod autocomplete_function_result_passed_to_function_has_parentheses {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2183:autocomplete_function_result_passed_to_function_has_parentheses`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_function_result_passed_to_function_has_parentheses

  #[cfg(test)]
  #[test]
  fn autocomplete_function_result_passed_to_function_has_parentheses() {
    use core::ffi::c_char;

    use ulua_analysis::enums::parentheses_recommendation::ParenthesesRecommendation;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function foo() return 1 end
local function bar(a: number) return -a end
local abc = bar(@1)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("foo"));
    assert_eq!(
      ac.entry_map["foo"].parens,
      ParenthesesRecommendation::CursorAfter
    );
  }
}

mod autocomplete_function_type_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1479:autocomplete_function_type_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_function_type_types

  #[cfg(test)]
  #[test]
  fn autocomplete_function_type_types() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a: (n@1
local b: (number, (n@2
local c: (number, (number) -> n@3
local d: (number, (number) -> (number, n@4
local e: (n: n@5
    "#,
    ));

    for marker in b'1'..=b'5' {
      let ac = fixture.base.autocomplete_marker(marker as c_char);

      assert!(ac.entry_map.contains_key("nil"));
      assert!(ac.entry_map.contains_key("number"));
    }
  }
}

mod autocomplete_generic_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1515:autocomplete_generic_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_generic_types

  #[cfg(test)]
  #[test]
  fn autocomplete_generic_types() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
function f<Tee, Use>(a: T@1
local b: string = "don't trip"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("Tee"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_get_frontend_use_correct_global_scope {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3957:autocomplete_get_frontend_use_correct_global_scope`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_get_frontend_use_correct_global_scope
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_get_frontend_use_correct_global_scope() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
      r#"
        declare extern type Instance with
            Name: string
        end
    "#,
    ));

    fixture.base.check(String::from(
      r#"
        local a: unknown = nil
        if typeof(a) == "Instance" then
            local b = a.@1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(ac.entry_map.len(), 1);
    assert!(ac.entry_map.contains_key("Name"));
  }
}

mod autocomplete_get_member_completions_autocomplete_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:360:autocomplete_get_member_completions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_get_member_completions

  #[cfg(test)]
  #[test]
  fn autocomplete_get_member_completions() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        local a = table.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(17, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("find"));
    assert!(ac.entry_map.contains_key("pack"));
    assert!(!ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_get_member_completions_autocomplete_test_alt_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1302:autocomplete_get_member_completions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_get_member_completions

  #[cfg(test)]
  #[test]
  fn autocomplete_get_member_completions() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local a = 12.@13
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());
  }
}

mod autocomplete_get_string_completions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:466:autocomplete_get_string_completions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_get_string_completions

  #[cfg(test)]
  #[test]
  fn autocomplete_get_string_completions() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        local a = ("foo"):@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(17, ac.entry_map.len());
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_get_suggestions_for_new_statement {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:478:autocomplete_get_suggestions_for_new_statement`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_get_suggestions_for_new_statement

  #[cfg(test)]
  #[test]
  fn autocomplete_get_suggestions_for_new_statement() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("@1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_ne!(0, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("table"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_get_suggestions_for_the_very_start_of_the_script {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:490:autocomplete_get_suggestions_for_the_very_start_of_the_script`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_get_suggestions_for_the_very_start_of_the_script

  #[cfg(test)]
  #[test]
  fn autocomplete_get_suggestions_for_the_very_start_of_the_script() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"@1

        function aaa() end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("table"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_global_function_params {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1192:autocomplete_global_function_params`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_global_function_params

  #[cfg(test)]
  #[test]
  fn autocomplete_global_function_params() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        function abc(def)
    "#,
    ));

    for i in 17..25 {
      assert!(
        fixture
          .base
          .autocomplete_position(1, i)
          .entry_map
          .is_empty()
      );
    }
    assert!(
      !fixture
        .base
        .autocomplete_position(1, 26)
        .entry_map
        .is_empty()
    );

    fixture.base.check(String::from(
      r#"
        function abc(def)
        end
    "#,
    ));

    for i in 17..25 {
      assert!(
        fixture
          .base
          .autocomplete_position(1, i)
          .entry_map
          .is_empty()
      );
    }
    assert!(
      !fixture
        .base
        .autocomplete_position(1, 26)
        .entry_map
        .is_empty()
    );

    fixture.base.check(String::from(
      r#"
        function abc(def)
@1
        end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac2.entry_map.get("abc").map_or(0, |_| 1));
    assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        function abc(def, ghi@1)
        end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac3.entry_map.is_empty());
    assert_eq!(ac3.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_global_functions_are_not_scoped_lexically {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:313:autocomplete_global_functions_are_not_scoped_lexically`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_global_functions_are_not_scoped_lexically

  #[cfg(test)]
  #[test]
  fn autocomplete_global_functions_are_not_scoped_lexically() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        if true then
            function abc()

            end
        end
@1    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("abc"));
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
  }
}

mod autocomplete_globals_are_order_independent {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3748:autocomplete_globals_are_order_independent`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_globals_are_order_independent

  #[cfg(test)]
  #[test]
  fn autocomplete_globals_are_order_independent() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local myLocal = 4
        function abc0()
            local myInnerLocal = 1
@1
        end

        function abc1()
            local myInnerLocal = 1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("myLocal"));
    assert!(ac.entry_map.contains_key("myInnerLocal"));
    assert!(ac.entry_map.contains_key("abc0"));
    assert!(ac.entry_map.contains_key("abc1"));
  }
}

mod autocomplete_if_then_else_elseif_completions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2528:autocomplete_if_then_else_elseif_completions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item autocomplete_if_then_else_elseif_completions

  #[cfg(test)]
  #[test]
  fn autocomplete_if_then_else_elseif_completions() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local elsewhere = false

if true then
    return 1
el@1
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("else"));
    assert!(ac1.entry_map.contains_key("elseif"));
    assert!(!ac1.entry_map.contains_key("elsewhere"));

    fixture.base.check(String::from(
      r#"
local elsewhere = false

if true then
    return 1
else
    return 2
el@1
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac2.entry_map.contains_key("else"));
    assert!(!ac2.entry_map.contains_key("elseif"));
    assert!(ac2.entry_map.contains_key("elsewhere"));

    fixture.base.check(String::from(
      r#"
local elsewhere = false

if true then
    print("1")
elif true then
    print("2")
el@1
end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac3.entry_map.contains_key("else"));
    assert!(ac3.entry_map.contains_key("elseif"));
    assert!(ac3.entry_map.contains_key("elsewhere"));
  }
}

mod autocomplete_if_then_else_full_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2492:autocomplete_if_then_else_full_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_if_then_else_full_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_if_then_else_full_keywords() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local thenceforth = false
local elsewhere = false
local doover = false
local endurance = true

if 1 then@1
else@2
end

while false do@3
end

repeat@4
until
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac1.entry_map.len());
    assert!(ac1.entry_map.contains_key("then"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac2.entry_map.contains_key("else"));
    assert!(ac2.entry_map.contains_key("elseif"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac3.entry_map.contains_key("do"));

    let ac4 = fixture.base.autocomplete_marker(b'4' as c_char);
    assert!(ac4.entry_map.contains_key("do"));
  }
}

mod autocomplete_keyword_members {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2395:autocomplete_keyword_members`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_keyword_members

  #[cfg(test)]
  #[test]
  fn autocomplete_keyword_members() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a = { done = 1, forever = 2 }
local b = a.do@1
local c = a.for@2
local d = a.@3
do
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(2, ac1.entry_map.len());
    assert!(ac1.entry_map.contains_key("done"));
    assert!(ac1.entry_map.contains_key("forever"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert_eq!(2, ac2.entry_map.len());
    assert!(ac2.entry_map.contains_key("done"));
    assert!(ac2.entry_map.contains_key("forever"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);

    assert_eq!(2, ac3.entry_map.len());
    assert!(ac3.entry_map.contains_key("done"));
    assert!(ac3.entry_map.contains_key("forever"));
  }
}

mod autocomplete_keyword_methods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2425:autocomplete_keyword_methods`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_keyword_methods

  #[cfg(test)]
  #[test]
  fn autocomplete_keyword_methods() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a = {}
function a:done() end
local b = a:do@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("done"));
  }
}

mod autocomplete_keyword_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2439:autocomplete_keyword_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_keyword_types

  #[cfg(test)]
  #[test]
  fn autocomplete_keyword_types() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
export type done = { x: number, y: number }
export type other = { z: number, w: number }
return {}
    "#,
      ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_a, None);
    assert!(result.errors.is_empty());

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local aaa = require(script.Parent.A)
local a: aaa.do
    "#,
      ),
    );

    let module_b = String::from("Module/B");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);

    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_b,
        Position {
          line: 2,
          column: 15,
        },
        Box::new(null_callback),
      );

    assert_eq!(2, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("done"));
    assert!(ac.entry_map.contains_key("other"));
  }
}

mod autocomplete_leave_numbers_alone {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:216:autocomplete_leave_numbers_alone`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_leave_numbers_alone

  #[cfg(test)]
  #[test]
  fn autocomplete_leave_numbers_alone() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("local a = 3.@11"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.is_empty());
    assert_eq!(ac.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_library_non_self_calls_are_fine {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3653:autocomplete_library_non_self_calls_are_fine`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function getn (VM/src/ltablib.cpp)
  //!   - translates_to -> rust_item autocomplete_library_non_self_calls_are_fine

  #[cfg(test)]
  #[test]
  fn autocomplete_library_non_self_calls_are_fine() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
string.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("byte"));
    assert!(!ac.entry_map["byte"].wrong_index_type);
    assert!(!ac.entry_map["byte"].indexed_with_self);
    assert!(ac.entry_map.contains_key("char"));
    assert!(!ac.entry_map["char"].wrong_index_type);
    assert!(!ac.entry_map["char"].indexed_with_self);
    assert!(ac.entry_map.contains_key("sub"));
    assert!(!ac.entry_map["sub"].wrong_index_type);
    assert!(!ac.entry_map["sub"].indexed_with_self);

    fixture.base.check(String::from(
      r#"
table.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("remove"));
    assert!(!ac.entry_map["remove"].wrong_index_type);
    assert!(!ac.entry_map["remove"].indexed_with_self);
    assert!(ac.entry_map.contains_key("getn"));
    assert!(!ac.entry_map["getn"].wrong_index_type);
    assert!(!ac.entry_map["getn"].indexed_with_self);
    assert!(ac.entry_map.contains_key("insert"));
    assert!(!ac.entry_map["insert"].wrong_index_type);
    assert!(!ac.entry_map["insert"].indexed_with_self);
  }
}

mod autocomplete_library_self_calls_are_invalid {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3688:autocomplete_library_self_calls_are_invalid`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item autocomplete_library_self_calls_are_invalid

  #[cfg(test)]
  #[test]
  fn autocomplete_library_self_calls_are_invalid() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
string:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("byte"));
    assert!(ac.entry_map["byte"].wrong_index_type);
    assert!(ac.entry_map["byte"].indexed_with_self);
    assert!(ac.entry_map.contains_key("char"));
    assert!(ac.entry_map["char"].wrong_index_type);
    assert!(ac.entry_map["char"].indexed_with_self);
    assert!(ac.entry_map.contains_key("sub"));
    assert!(!ac.entry_map["sub"].wrong_index_type);
    assert!(ac.entry_map["sub"].indexed_with_self);
  }
}

mod autocomplete_local_function_autocomplete_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1089:autocomplete_local_function`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_local_function

  #[cfg(test)]
  #[test]
  fn autocomplete_local_function() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local f@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac1.entry_map.len());
    assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));

    fixture.base.check(String::from(
      r#"
        local f@1, cd
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac2.entry_map.is_empty());
  }
}

mod autocomplete_local_function_autocomplete_test_alt_b {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1107:autocomplete_local_function`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item autocomplete_local_function

  #[cfg(test)]
  #[test]
  fn autocomplete_local_function() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function @1
    "#,
    ));

    let mut ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());

    fixture.base.check(String::from(
      r#"
        local function @1s@2
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());

    ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.is_empty());

    fixture.base.check(String::from(
      r#"
        local function @1()@2
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());

    ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("end"));

    fixture.base.check(String::from(
      r#"
        local function something@1
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());

    fixture.base.check(String::from(
      r#"
        local tbl = {}
        function tbl.something@1() end
    "#,
    ));

    ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.is_empty());
  }
}

mod autocomplete_local_function_params {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1152:autocomplete_local_function_params`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_local_function_params

  #[cfg(test)]
  #[test]
  fn autocomplete_local_function_params() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function @1a@2bc(@3d@4ef)@5 @6
    "#,
    ));

    for marker in *b"1234" {
      assert!(
        fixture
          .base
          .autocomplete_marker(marker as c_char)
          .entry_map
          .is_empty()
      );
    }

    assert!(
      !fixture
        .base
        .autocomplete_marker(b'5' as c_char)
        .entry_map
        .is_empty()
    );
    assert!(
      !fixture
        .base
        .autocomplete_marker(b'6' as c_char)
        .entry_map
        .is_empty()
    );

    fixture.base.check(String::from(
      r#"
        local function abc(def)
@1        end
    "#,
    ));

    for i in 23..31 {
      assert!(
        fixture
          .base
          .autocomplete_position(1, i)
          .entry_map
          .is_empty()
      );
    }
    assert!(
      !fixture
        .base
        .autocomplete_position(1, 32)
        .entry_map
        .is_empty()
    );

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac2.entry_map.get("abc").map_or(0, |_| 1));
    assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Statement);

    fixture.base.check(String::from(
      r#"
        local function abc(def, ghi@1)
        end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac3.entry_map.is_empty());
    assert_eq!(ac3.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_local_functions_fall_out_of_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:331:autocomplete_local_functions_fall_out_of_scope`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_local_functions_fall_out_of_scope

  #[cfg(test)]
  #[test]
  fn autocomplete_local_functions_fall_out_of_scope() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        if true then
            local function abc()

            end
        end
@1    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_ne!(0, ac.entry_map.len());
    assert!(!ac.entry_map.contains_key("abc"));
  }
}

mod autocomplete_local_initializer_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1292:autocomplete_local_initializer_2`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_local_initializer_2

  #[cfg(test)]
  #[test]
  fn autocomplete_local_initializer_2() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local a=@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("table"));
  }
}

mod autocomplete_local_initializer_autocomplete_test {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:206:autocomplete_local_initializer`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_local_initializer

  #[cfg(test)]
  #[test]
  fn autocomplete_local_initializer() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("local a = @1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_local_initializer_autocomplete_test_alt_b {
  //! Ported from `tests/Autocomplete.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1281:autocomplete_local_initializer`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_local_initializer
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_local_initializer() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local a = t@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("true"));
  }
}

mod autocomplete_local_names {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:980:autocomplete_local_names`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_local_names

  #[cfg(test)]
  #[test]
  fn autocomplete_local_names() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local ab@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac1.entry_map.len());
    assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Unknown);

    fixture.base.check(String::from(
      r#"
        local ab, cd@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac2.entry_map.is_empty());
    assert_eq!(ac2.context, AutocompleteContext::Unknown);
  }
}

mod autocomplete_local_types_builtin {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1329:autocomplete_local_types_builtin`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_local_types_builtin

  #[cfg(test)]
  #[test]
  fn autocomplete_local_types_builtin() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a: n@1
local b: string = "don't trip"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("nil"));
    assert!(ac.entry_map.contains_key("number"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_method_call_inside_function_body {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:503:autocomplete_method_call_inside_function_body`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_method_call_inside_function_body

  #[cfg(test)]
  #[test]
  fn autocomplete_method_call_inside_function_body() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()
            game:  @1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_ne!(0, ac.entry_map.len());
    assert!(!ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_method_call_inside_if_conditional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:521:autocomplete_method_call_inside_if_conditional`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_method_call_inside_if_conditional

  #[cfg(test)]
  #[test]
  fn autocomplete_method_call_inside_if_conditional() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
        if table:  @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_ne!(0, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("concat"));
    assert!(!ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_module_type_members {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1412:autocomplete_module_type_members`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_module_type_members

  #[cfg(test)]
  #[test]
  fn autocomplete_module_type_members() {
    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
export type A = { x: number, y: number }
export type B = { z: number, w: number }
return {}
    "#,
      ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_a, None);
    assert!(result.errors.is_empty());

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local aaa = require(script.Parent.A)
local a: aaa.
    "#,
      ),
    );

    let module_b = String::from("Module/B");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);

    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_b,
        Position {
          line: 2,
          column: 13,
        },
        Box::new(null_callback),
      );

    assert_eq!(2, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("A"));
    assert!(ac.entry_map.contains_key("B"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_modules_with_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1389:autocomplete_modules_with_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_modules_with_types

  #[cfg(test)]
  #[test]
  fn autocomplete_modules_with_types() {
    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
export type A = { x: number, y: number }
export type B = { z: number, w: number }
return {}
    "#,
      ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_a, None);
    assert!(result.errors.is_empty());

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local aaa = require(script.Parent.A)
local a: aa
    "#,
      ),
    );

    let module_b = String::from("Module/B");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);

    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_b,
        Position {
          line: 2,
          column: 11,
        },
        Box::new(null_callback),
      );

    assert!(ac.entry_map.contains_key("aaa"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_nested_member_completions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:375:autocomplete_nested_member_completions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_nested_member_completions

  #[cfg(test)]
  #[test]
  fn autocomplete_nested_member_completions() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local tbl = { abc = { def = 1234, egh = false } }
        tbl.abc. @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(2, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("def"));
    assert!(ac.entry_map.contains_key("egh"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_nested_recursive_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:272:autocomplete_nested_recursive_function`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_nested_recursive_function

  #[cfg(test)]
  #[test]
  fn autocomplete_nested_recursive_function() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function outer()
            local function inner()
@1            end
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("inner"));
    assert!(ac.entry_map.contains_key("outer"));
  }
}

mod autocomplete_no_function_name_suggestions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2344:autocomplete_no_function_name_suggestions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_no_function_name_suggestions

  #[cfg(test)]
  #[test]
  fn autocomplete_no_function_name_suggestions() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
function na@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.is_empty());

    fixture.base.check(String::from(
      r#"
local function @1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.is_empty());

    fixture.base.check(String::from(
      r#"
local function na@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.is_empty());
  }
}

mod autocomplete_no_incompatible_self_calls {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3534:autocomplete_no_incompatible_self_calls`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_no_incompatible_self_calls

  #[cfg(test)]
  #[test]
  fn autocomplete_no_incompatible_self_calls() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
function t.m() end
t:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("m"));
    assert!(ac.entry_map["m"].wrong_index_type);
    assert!(ac.entry_map["m"].indexed_with_self);
  }
}

mod autocomplete_no_incompatible_self_calls_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3549:autocomplete_no_incompatible_self_calls_2`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_no_incompatible_self_calls_2

  #[cfg(test)]
  #[test]
  fn autocomplete_no_incompatible_self_calls_2() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local f: (() -> number) & ((number) -> number) = function(x: number?) return 2 end
local t = {}
t.f = f
t:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("f"));
    assert!(ac.entry_map["f"].wrong_index_type);
    assert!(ac.entry_map["f"].indexed_with_self);
  }
}

mod autocomplete_no_incompatible_self_calls_on_class {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3461:autocomplete_no_incompatible_self_calls_on_class`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_no_incompatible_self_calls_on_class

  #[cfg(test)]
  #[test]
  fn autocomplete_no_incompatible_self_calls_on_class() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
      r#"
declare extern type Foo with
    function one(self): number
    two: () -> number
end
    "#,
    ));

    fixture.base.check(String::from(
      r#"
local function f(t: Foo)
    t:@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("one"));
    assert!(ac.entry_map.contains_key("two"));
    assert!(!ac.entry_map["one"].wrong_index_type);
    assert!(ac.entry_map["two"].wrong_index_type);
    assert!(ac.entry_map["one"].indexed_with_self);
    assert!(ac.entry_map["two"].indexed_with_self);

    fixture.base.check(String::from(
      r#"
local function f(t: Foo)
    t.@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("one"));
    assert!(ac.entry_map.contains_key("two"));
    assert!(ac.entry_map["one"].wrong_index_type);
    assert!(!ac.entry_map["two"].wrong_index_type);
    assert!(!ac.entry_map["one"].indexed_with_self);
    assert!(!ac.entry_map["two"].indexed_with_self);
  }
}

mod autocomplete_no_wrong_compatible_self_calls_with_generics {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3600:autocomplete_no_wrong_compatible_self_calls_with_generics`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_no_wrong_compatible_self_calls_with_generics

  #[cfg(test)]
  #[test]
  fn autocomplete_no_wrong_compatible_self_calls_with_generics() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
function t.m<T>(a: T) end
t:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("m"));
    assert!(ac.entry_map["m"].wrong_index_type);
    assert!(ac.entry_map["m"].indexed_with_self);
  }
}

mod autocomplete_not_the_var_we_are_defining {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2576:autocomplete_not_the_var_we_are_defining`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_not_the_var_we_are_defining

  #[cfg(test)]
  #[test]
  fn autocomplete_not_the_var_we_are_defining() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture
      .base
      .base
      .file_resolver
      .source
      .insert(String::from("Module/A"), String::from("abc,de"));

    let module = String::from("Module/A");
    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module,
        Position { line: 0, column: 6 },
        Box::new(null_callback),
      );

    assert!(!ac.entry_map.contains_key("de"));
  }
}

mod autocomplete_optional_members {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2306:autocomplete_optional_members`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_optional_members

  #[cfg(test)]
  #[test]
  fn autocomplete_optional_members() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a = { x = 2, y = 3 }
type A = typeof(a)
local b: A? = a
return b.@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(2, ac1.entry_map.len());
    assert!(ac1.entry_map.contains_key("x"));
    assert!(ac1.entry_map.contains_key("y"));

    fixture.base.check(String::from(
      r#"
local a = { x = 2, y = 3 }
type A = typeof(a)
local b: nil | A = a
return b.@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(2, ac2.entry_map.len());
    assert!(ac2.entry_map.contains_key("x"));
    assert!(ac2.entry_map.contains_key("y"));

    fixture.base.check(String::from(
      r#"
local b: nil | nil
return b.@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(0, ac3.entry_map.len());
  }
}

mod autocomplete_private_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1343:autocomplete_private_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_private_types

  #[cfg(test)]
  #[test]
  fn autocomplete_private_types() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
do
    type num = number
    local a: n@1u
    local b: nu@2m
end
local a: nu@3
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("num"));
    assert!(ac1.entry_map.contains_key("number"));

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac2.entry_map.contains_key("num"));
    assert!(ac2.entry_map.contains_key("number"));

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);

    assert!(!ac3.entry_map.contains_key("num"));
    assert!(ac3.entry_map.contains_key("number"));
  }
}

mod autocomplete_recommend_statement_starting_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:578:autocomplete_recommend_statement_starting_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_recommend_statement_starting_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_recommend_statement_starting_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("@1"));
    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("local"));
    assert_eq!(ac.context, AutocompleteContext::Statement);

    fixture.base.check(String::from("local i = @1"));
    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac2.entry_map.contains_key("local"));
    assert_eq!(ac2.context, AutocompleteContext::Expression);
  }
}

mod autocomplete_recursive_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:260:autocomplete_recursive_function`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_recursive_function

  #[cfg(test)]
  #[test]
  fn autocomplete_recursive_function() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        function foo()
@1        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("foo"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_recursive_function_global {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2584:autocomplete_recursive_function_global`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_recursive_function_global

  #[cfg(test)]
  #[test]
  fn autocomplete_recursive_function_global() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("global"),
      String::from(
        r#"function abc()

end
"#,
      ),
    );

    let module = String::from("global");
    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module,
        Position { line: 1, column: 0 },
        Box::new(null_callback),
      );

    assert!(ac.entry_map.contains_key("abc"));
  }
}

mod autocomplete_recursive_function_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2597:autocomplete_recursive_function_local`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_recursive_function_local

  #[cfg(test)]
  #[test]
  fn autocomplete_recursive_function_local() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback, records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("local"),
      String::from(
        r#"local function abc()

end
"#,
      ),
    );

    let module = String::from("local");
    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module,
        Position { line: 1, column: 0 },
        Box::new(null_callback),
      );

    assert!(ac.entry_map.contains_key("abc"));
  }
}

mod autocomplete_require_by_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3800:autocomplete_require_by_string`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record RequireCompletion (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias AutocompleteEntryMap (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record AutocompleteResult (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_require_by_string
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_require_by_string() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback,
      records::ac_builtins_fixture::ACBuiltinsFixture,
    };

    fn check_entries(entry_map: &AutocompleteEntryMap, completions: &[(&str, &str)]) {
      assert_eq!(completions.len(), entry_map.len());

      for (label, insert_text) in completions {
        let entry = entry_map
          .get(*label)
          .unwrap_or_else(|| panic!("missing require completion `{label}`"));
        assert_eq!(entry.insert_text.as_deref(), Some(*insert_text));
      }
    }

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("MainModule"),
      String::from(
        r#"
        local info = "MainModule serves as the root directory"
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("MainModule/Folder"),
      String::from(
        r#"
        local info = "MainModule/Folder serves as a subdirectory"
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("MainModule/Folder/Requirer"),
      String::from(
        r#"
        local res0 = require("@")

        local res1 = require(".")
        local res2 = require("./")
        local res3 = require("./Sib")

        local res4 = require("..")
        local res5 = require("../")
        local res6 = require("../Sib")
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("MainModule/Folder/SiblingDependency"),
      String::from(
        r#"
        return {"result"}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("MainModule/ParentDependency"),
      String::from(
        r#"
        return {"result"}
    "#,
      ),
    );

    let module_name = String::from("MainModule/Folder/Requirer");

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 1,
          column: 31,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("@defaultalias", "@defaultalias"),
        ("./", "./"),
        ("../", "../"),
      ],
    );

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 3,
          column: 31,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("@defaultalias", "@defaultalias"),
        ("./", "./"),
        ("../", "../"),
      ],
    );

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 4,
          column: 32,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("..", "."),
        ("Requirer", "./Requirer"),
        ("SiblingDependency", "./SiblingDependency"),
      ],
    );

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 5,
          column: 35,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("..", "."),
        ("Requirer", "./Requirer"),
        ("SiblingDependency", "./SiblingDependency"),
      ],
    );

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 7,
          column: 32,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("@defaultalias", "@defaultalias"),
        ("./", "./"),
        ("../", "../"),
      ],
    );

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 8,
          column: 33,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("..", "../.."),
        ("Folder", "../Folder"),
        ("ParentDependency", "../ParentDependency"),
      ],
    );

    let ac_result = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_name,
        Position {
          line: 9,
          column: 36,
        },
        Box::new(null_callback),
      );
    check_entries(
      &ac_result.entry_map,
      &[
        ("..", "../.."),
        ("Folder", "../Folder"),
        ("ParentDependency", "../ParentDependency"),
      ],
    );
  }
}

mod autocomplete_require_tracing {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4490:autocomplete_require_tracing`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item autocomplete_require_tracing

  #[cfg(test)]
  #[test]
  fn autocomplete_require_tracing() {
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback,
      records::ac_builtins_fixture::ACBuiltinsFixture,
    };

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
return { x = 0 }
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local result = require(script.Parent.A)
local x = 1 + result.
    "#,
      ),
    );

    let ac = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &String::from("Module/B"),
        Position {
          line: 2,
          column: 21,
        },
        Box::new(null_callback),
      );

    assert_eq!(ac.entry_map.len(), 1);
    assert!(ac.entry_map.contains_key("x"));
  }
}

mod autocomplete_return_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1451:autocomplete_return_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_return_types

  #[cfg(test)]
  #[test]
  fn autocomplete_return_types() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function f(a: number): n@1
local b: string = "don't trip"
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("nil"));
    assert!(ac.entry_map.contains_key("number"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_simple {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3505:autocomplete_simple`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_simple

  #[cfg(test)]
  #[test]
  fn autocomplete_simple() {
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
function t:m() end
t:m()
    "#,
    ));
  }
}

mod autocomplete_skip_current_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2371:autocomplete_skip_current_local`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item autocomplete_skip_current_local

  #[cfg(test)]
  #[test]
  fn autocomplete_skip_current_local() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local other = 1
local name = na@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(!ac1.entry_map.contains_key("name"));
    assert!(ac1.entry_map.contains_key("other"));

    fixture.base.check(String::from(
      r#"
local other = 1
local name, test = na@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(!ac2.entry_map.contains_key("name"));
    assert!(!ac2.entry_map.contains_key("test"));
    assert!(ac2.entry_map.contains_key("other"));
  }
}

mod autocomplete_sometimes_the_metatable_is_an_error {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1312:autocomplete_sometimes_the_metatable_is_an_error`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_sometimes_the_metatable_is_an_error

  #[cfg(test)]
  #[test]
  fn autocomplete_sometimes_the_metatable_is_an_error() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local T = {}
        T.__index = T

        function T.new()
            return setmetatable({x=6}, X) -- oops!
        end
        local t = T.new()
        t.  @1
    "#,
    ));

    fixture.base.autocomplete_marker(b'1' as c_char);
  }
}

mod autocomplete_source_module_preservation_and_invalidation {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3710:autocomplete_source_module_preservation_and_invalidation`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item autocomplete_source_module_preservation_and_invalidation
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_source_module_preservation_and_invalidation() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local a = { x = 2, y = 4 }
a.@1
    "#,
    ));

    fixture.base.get_frontend().clear();

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(2, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));

    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("MainModule"), None);

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));

    fixture
      .base
      .get_frontend()
      .mark_dirty(&String::from("MainModule"), None);

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));

    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("MainModule"), None);

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));
  }
}

mod autocomplete_statement_between_two_statements {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:535:autocomplete_statement_between_two_statements`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_statement_between_two_statements

  #[cfg(test)]
  #[test]
  fn autocomplete_statement_between_two_statements() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        function getmyscripts() end

        g@1

        getmyscripts()
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_ne!(0, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("getmyscripts"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_stop_at_first_stat_when_recommending_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1047:autocomplete_stop_at_first_stat_when_recommending_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_stop_at_first_stat_when_recommending_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_stop_at_first_stat_when_recommending_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        repeat
            for x @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert_eq!(1, ac1.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("until").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Keyword);
  }
}

mod autocomplete_strict_mode_force {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3929:autocomplete_strict_mode_force`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_strict_mode_force

  #[cfg(test)]
  #[test]
  fn autocomplete_strict_mode_force() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
--!nonstrict
local a: {x: number} = {x=1}
local b = a
local c = b.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("x"));
  }
}

mod autocomplete_string_completion_outside_quotes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3977:autocomplete_string_completion_outside_quotes`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> type_alias StringCompletionCallback (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias AutocompleteEntryMap (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record AutocompleteEntry (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_string_completion_outside_quotes
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_string_completion_outside_quotes() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::{
      enums::autocomplete_entry_kind::AutocompleteEntryKind,
      records::autocomplete_entry::AutocompleteEntry,
      type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
    };
    use ulua_unit_test::{
      functions::autocomplete_attach_require_call_tag::autocomplete_attach_require_call_tag,
      records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
      r#"
        declare function require(path: string): any
    "#,
    ));

    autocomplete_attach_require_call_tag(fixture.base.get_frontend());

    fixture.base.check(String::from(
      r#"
        local x = require(@1"@2"@3)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker_callback(
      b'2' as c_char,
      Box::new(|_tag, _extern_type, _contents| {
        let mut results = AutocompleteEntryMap::new();
        results.insert(
          String::from("test"),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::String,
            ..Default::default()
          },
        );
        Some(results)
      }),
    );

    assert_eq!(ac.entry_map.len(), 1);
    assert!(ac.entry_map.contains_key("test"));

    let ac = fixture.base.autocomplete_marker_callback(
      b'1' as c_char,
      Box::new(|_tag, _extern_type, _contents| {
        let mut results = AutocompleteEntryMap::new();
        results.insert(
          String::from("test"),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::String,
            ..Default::default()
          },
        );
        Some(results)
      }),
    );

    assert_eq!(ac.entry_map.len(), 0);

    let ac = fixture.base.autocomplete_marker_callback(
      b'3' as c_char,
      Box::new(|_tag, _extern_type, _contents| {
        let mut results = AutocompleteEntryMap::new();
        results.insert(
          String::from("test"),
          AutocompleteEntry {
            kind: AutocompleteEntryKind::String,
            ..Default::default()
          },
        );
        Some(results)
      }),
    );

    assert_eq!(ac.entry_map.len(), 0);
  }
}

mod autocomplete_string_contents_is_available_to_callback {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3769:autocomplete_string_contents_is_available_to_callback`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> type_alias AutocompleteEntryMap (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_string_contents_is_available_to_callback
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_string_contents_is_available_to_callback() {
    use alloc::{rc::Rc, string::String};
    use core::{cell::Cell, ffi::c_char};

    use ulua_unit_test::{
      functions::autocomplete_attach_require_call_tag::autocomplete_attach_require_call_tag,
      records::ac_fixture::AcFixture,
    };

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
      r#"
        declare function require(path: string): any
    "#,
    ));

    autocomplete_attach_require_call_tag(fixture.base.get_frontend());

    fixture.base.check(String::from(
      r#"
        local x = require("testing/@1")
    "#,
    ));

    let is_correct = Rc::new(Cell::new(false));
    let is_correct_for_callback = Rc::clone(&is_correct);
    fixture.base.autocomplete_marker_callback(
      b'1' as c_char,
      Box::new(move |_tag, _extern_type, contents| {
        is_correct_for_callback.set(contents.as_deref() == Some("testing/"));
        None
      }),
    );

    assert!(is_correct.get());
  }
}

mod autocomplete_string_prim_non_self_calls_are_avoided {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3636:autocomplete_string_prim_non_self_calls_are_avoided`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_string_prim_non_self_calls_are_avoided

  #[cfg(test)]
  #[test]
  fn autocomplete_string_prim_non_self_calls_are_avoided() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local s = "hello"
s.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("char"));
    assert!(!ac.entry_map["char"].wrong_index_type);
    assert!(!ac.entry_map["char"].indexed_with_self);
    assert!(ac.entry_map.contains_key("sub"));
    assert!(ac.entry_map["sub"].wrong_index_type);
    assert!(!ac.entry_map["sub"].indexed_with_self);
  }
}

mod autocomplete_string_prim_self_calls_are_fine {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3616:autocomplete_string_prim_self_calls_are_fine`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_string_prim_self_calls_are_fine

  #[cfg(test)]
  #[test]
  fn autocomplete_string_prim_self_calls_are_fine() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local s = "hello"
s:@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("byte"));
    assert!(!ac.entry_map["byte"].wrong_index_type);
    assert!(ac.entry_map["byte"].indexed_with_self);
    assert!(ac.entry_map.contains_key("char"));
    assert!(ac.entry_map["char"].wrong_index_type);
    assert!(ac.entry_map["char"].indexed_with_self);
    assert!(ac.entry_map.contains_key("sub"));
    assert!(!ac.entry_map["sub"].wrong_index_type);
    assert!(ac.entry_map["sub"].indexed_with_self);
  }
}

mod autocomplete_string_singleton_as_table_key {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3158:autocomplete_string_singleton_as_table_key`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_string_singleton_as_table_key

  #[cfg(test)]
  #[test]
  fn autocomplete_string_singleton_as_table_key() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type Direction = "up" | "down"

        local a: {[Direction]: boolean} = {[@1] = true}
        local b: {[Direction]: boolean} = {["@2"] = true}
        local c: {[Direction]: boolean} = {u@3 = true}
        local d: {[Direction]: boolean} = {[u@4] = true}

        local e: {[Direction]: boolean} = {[@5]}
        local f: {[Direction]: boolean} = {["@6"]}
        local g: {[Direction]: boolean} = {u@7}
        local h: {[Direction]: boolean} = {[u@8]}
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("\"up\""));
    assert!(ac.entry_map.contains_key("\"down\""));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("up"));
    assert!(ac.entry_map.contains_key("down"));

    let ac = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac.entry_map.contains_key("up"));
    assert!(ac.entry_map.contains_key("down"));

    let ac = fixture.base.autocomplete_marker(b'4' as c_char);
    assert!(!ac.entry_map.contains_key("up"));
    assert!(!ac.entry_map.contains_key("down"));
    assert!(ac.entry_map.contains_key("\"up\""));
    assert!(ac.entry_map.contains_key("\"down\""));

    let ac = fixture.base.autocomplete_marker(b'5' as c_char);
    assert!(ac.entry_map.contains_key("\"up\""));
    assert!(ac.entry_map.contains_key("\"down\""));

    let ac = fixture.base.autocomplete_marker(b'6' as c_char);
    assert!(ac.entry_map.contains_key("up"));
    assert!(ac.entry_map.contains_key("down"));

    let ac = fixture.base.autocomplete_marker(b'7' as c_char);
    assert!(ac.entry_map.contains_key("up"));
    assert!(ac.entry_map.contains_key("down"));

    let ac = fixture.base.autocomplete_marker(b'8' as c_char);
    assert!(!ac.entry_map.contains_key("up"));
    assert!(!ac.entry_map.contains_key("down"));
    assert!(ac.entry_map.contains_key("\"up\""));
    assert!(ac.entry_map.contains_key("\"down\""));
  }
}

mod autocomplete_string_singleton_as_table_key_iso {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3145:autocomplete_string_singleton_as_table_key_iso`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item autocomplete_string_singleton_as_table_key_iso

  #[cfg(test)]
  #[test]
  fn autocomplete_string_singleton_as_table_key_iso() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type Direction = "up" | "down"
        local b: {[Direction]: boolean} = {["@2"] = true}
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac.entry_map.contains_key("up"));
    assert!(ac.entry_map.contains_key("down"));
  }
}

mod autocomplete_string_singleton_in_if_statement {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3222:autocomplete_string_singleton_in_if_statement`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AutocompleteResult (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_string_singleton_in_if_statement
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_string_singleton_in_if_statement() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        --!strict

        type Direction = "left" | "right"

        local dir: Direction = "left"

        if dir == @1"@2"@3 then end
        local a: {[Direction]: boolean} = {[@4"@5"@6]}

        if dir == @7`@8`@9 then end
        local a: {[Direction]: boolean} = {[@A`@B`@C]}
    "#,
    ));

    let mut check_marker = |marker: u8, has_left: bool, has_right: bool| {
      let ac = fixture.base.autocomplete_marker(marker as c_char);
      assert_eq!(ac.entry_map.contains_key("left"), has_left);
      assert_eq!(ac.entry_map.contains_key("right"), has_right);
    };

    check_marker(b'1', false, false);
    check_marker(b'2', true, true);
    check_marker(b'3', false, false);
    check_marker(b'4', false, false);
    check_marker(b'5', true, true);
    check_marker(b'6', false, false);
    check_marker(b'7', false, false);
    check_marker(b'8', true, true);
    check_marker(b'9', false, false);
    check_marker(b'A', false, false);
    check_marker(b'B', true, true);
    check_marker(b'C', false, false);
  }
}

mod autocomplete_string_singleton_in_if_statement_2 {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3306:autocomplete_string_singleton_in_if_statement_2`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record AutocompleteResult (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_string_singleton_in_if_statement_2
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_string_singleton_in_if_statement_2() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        --!strict

        type Direction = "left" | "right"

        local dir: Direction
        -- typestate here means dir is actually typed as `"left"`
        dir = "left"

        if dir == @1"@2"@3 then end
        local a: {[Direction]: boolean} = {[@4"@5"@6]}

        if dir == @7`@8`@9 then end
        local a: {[Direction]: boolean} = {[@A`@B`@C]}
    "#,
    ));

    let mut check_marker = |marker: u8, has_left: bool, has_right: bool| {
      let ac = fixture.base.autocomplete_marker(marker as c_char);
      assert_eq!(ac.entry_map.contains_key("left"), has_left);
      assert_eq!(ac.entry_map.contains_key("right"), has_right);
    };

    check_marker(b'1', false, false);
    check_marker(b'2', true, false);
    check_marker(b'3', false, false);
    check_marker(b'4', false, false);
    check_marker(b'5', true, true);
    check_marker(b'6', false, false);
    check_marker(b'7', false, false);
    check_marker(b'8', true, false);
    check_marker(b'9', false, false);
    check_marker(b'A', false, false);
    check_marker(b'B', true, true);
    check_marker(b'C', false, false);
  }
}

mod autocomplete_suggest_exported_types {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3944:autocomplete_suggest_exported_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_suggest_exported_types

  #[cfg(test)]
  #[test]
  fn autocomplete_suggest_exported_types() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
export type Type = {a: number}
local a: T@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("Type"));
    assert_eq!(ac.context, AutocompleteContext::Type);
  }
}

mod autocomplete_suggest_external_module_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2102:autocomplete_suggest_external_module_type`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_suggest_external_module_type

  #[cfg(test)]
  #[test]
  fn autocomplete_suggest_external_module_type() {
    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{
      functions::null_callback_autocomplete_test::null_callback,
      records::ac_builtins_fixture::ACBuiltinsFixture,
    };

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
export type done = { x: number, y: number }
local function a(a: (done) -> number) return a({x=1, y=2}) end
local function b(a: ((done) -> number) -> number) return a(function(done) return 1 end) end
return {a = a, b = b}
    "#,
      ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_a, None);
    assert!(result.errors.is_empty());

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local ex = require(script.Parent.A)
ex.a(function(x:
    "#,
      ),
    );

    let module_b = String::from("Module/B");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_b, None);

    let ac1 = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_b,
        Position {
          line: 2,
          column: 16,
        },
        Box::new(null_callback),
      );

    assert!(!ac1.entry_map.contains_key("done"));
    assert!(ac1.entry_map.contains_key("ex.done"));
    assert_eq!(
      ac1.entry_map["ex.done"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/C"),
      String::from(
        r#"
local ex = require(script.Parent.A)
ex.b(function(x:
    "#,
      ),
    );

    let module_c = String::from("Module/C");
    fixture
      .base
      .get_frontend()
      .check_module_name_optional_frontend_options(&module_c, None);

    let ac2 = fixture
      .base
      .autocomplete_module_name_position_string_completion_callback(
        &module_c,
        Position {
          line: 2,
          column: 16,
        },
        Box::new(null_callback),
      );

    assert!(!ac2.entry_map.contains_key("(done) -> number"));
    assert!(ac2.entry_map.contains_key("(ex.done) -> number"));
    assert_eq!(
      ac2.entry_map["(ex.done) -> number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_suggest_table_keys {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2608:autocomplete_suggest_table_keys`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Inference (Analysis/include/Luau/ConstraintGenerator.h)
  //!   - translates_to -> rust_item autocomplete_suggest_table_keys

  use ulua_common::FFlag;

  #[cfg(test)]
  #[test]
  fn autocomplete_suggest_table_keys() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_context::AutocompleteContext,
      parentheses_recommendation::ParenthesesRecommendation,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = AcFixture::default();

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { f@1 }
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("first"));
    assert!(ac1.entry_map.contains_key("second"));
    assert_eq!(ac1.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: number } & { second: number }
local t: Test = { f@1 }
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac2.entry_map.contains_key("first"));
    assert!(ac2.entry_map.contains_key("second"));
    assert_eq!(ac2.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number } | { second: number, third: number }
local t: Test = { s@1 }
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac3.entry_map.contains_key("second"));
    assert!(!ac3.entry_map.contains_key("first"));
    assert!(!ac3.entry_map.contains_key("third"));
    assert_eq!(ac3.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: (number) -> number, second: number }
local t: Test = { f@1 }
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac4.entry_map.contains_key("first"));
    assert_eq!(
      ac4.entry_map["first"].parens,
      ParenthesesRecommendation::None
    );
    assert_eq!(ac4.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { f@1 = 2 }
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac5.entry_map.contains_key("first"));
    assert!(ac5.entry_map.contains_key("second"));
    assert_eq!(ac5.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { ["f@1"] }
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac6.entry_map.contains_key("first"));
    assert!(ac6.entry_map.contains_key("second"));
    assert_eq!(ac6.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { "f@1" }
    "#,
    ));

    let ac7 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac7.entry_map.contains_key("first"));
    assert!(!ac7.entry_map.contains_key("second"));
    assert_eq!(ac7.context, AutocompleteContext::String);

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { first = 2, s@1 }
    "#,
    ));

    let ac8 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac8.entry_map.contains_key("first"));
    assert!(ac8.entry_map.contains_key("second"));
    assert_eq!(ac8.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { first@1 }
    "#,
    ));

    let ac9 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac9.entry_map.contains_key("first"));
    assert!(ac9.entry_map.contains_key("second"));
    assert_eq!(ac9.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
local t = {
    { first = 5, second = 10 },
    { f@1 }
}
    "#,
    ));

    let ac10 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac10.entry_map.contains_key("first"));
    assert!(ac10.entry_map.contains_key("second"));
    assert_eq!(ac10.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
local t = {
    [2] = { first = 5, second = 10 },
    [5] = { f@1 }
}
    "#,
    ));

    let ac11 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac11.entry_map.contains_key("first"));
    assert!(ac11.entry_map.contains_key("second"));
    assert_eq!(ac11.context, AutocompleteContext::Property);
  }
}

mod autocomplete_suggest_table_keys_no_initial_character {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2739:autocomplete_suggest_table_keys_no_initial_character`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_suggest_table_keys_no_initial_character

  #[cfg(test)]
  #[test]
  fn autocomplete_suggest_table_keys_no_initial_character() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { @1 }
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("first"));
    assert!(ac.entry_map.contains_key("second"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_suggest_table_keys_no_initial_character_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2752:autocomplete_suggest_table_keys_no_initial_character_2`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_suggest_table_keys_no_initial_character_2

  #[cfg(test)]
  #[test]
  fn autocomplete_suggest_table_keys_no_initial_character_2() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type Test = { first: number, second: number }
local t: Test = { first = 1, @1 }
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("first"));
    assert!(ac.entry_map.contains_key("second"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_suggest_table_keys_no_initial_character_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2765:autocomplete_suggest_table_keys_no_initial_character_3`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_suggest_table_keys_no_initial_character_3

  #[cfg(test)]
  #[test]
  fn autocomplete_suggest_table_keys_no_initial_character_3() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type Properties = { TextScaled: boolean, Text: string }
local function create(props: Properties) end

create({ @1 })
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.is_empty());
    assert!(ac.entry_map.contains_key("TextScaled"));
    assert!(ac.entry_map.contains_key("Text"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_table_intersection {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:448:autocomplete_table_intersection`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_table_intersection

  #[cfg(test)]
  #[test]
  fn autocomplete_table_intersection() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)
            abc.  @1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(3, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("a1"));
    assert!(ac.entry_map.contains_key("b2"));
    assert!(ac.entry_map.contains_key("c3"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_table_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:432:autocomplete_table_union`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_table_union

  #[cfg(test)]
  #[test]
  fn autocomplete_table_union() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : string, c3 : string }
        function func(abc : t1 | t2)
            abc.  @1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("b2"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_type_correct_argument_type_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1820:autocomplete_type_correct_argument_type_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_argument_type_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_argument_type_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local function d(a: n@1, b)
    return target(a, b)
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local function d(a, b: s@1)
    return target(a, b)
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("string"));
    assert_eq!(
      ac2.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local function d(a:@1 @2, b)
    return target(a, b)
end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("number"));
    assert_eq!(
      ac3.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac4 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac4.entry_map.contains_key("number"));
    assert_eq!(
      ac4.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local function d(a, b: @1)@2: number
    return target(a, b)
end
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac5.entry_map.contains_key("string"));
    assert_eq!(
      ac5.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac6 = fixture.base.autocomplete_marker(b'2' as c_char);
    let string_type_correct = ac6
      .entry_map
      .get("string")
      .map(|entry| entry.type_correct)
      .unwrap_or(TypeCorrectKind::None);

    assert_eq!(string_type_correct, TypeCorrectKind::None);
  }
}

mod autocomplete_type_correct_expected_argument_type_pack_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1940:autocomplete_type_correct_expected_argument_type_pack_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_expected_argument_type_pack_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_expected_argument_type_pack_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(callback: (...number) -> number) return callback(1, 2, 3) end

local x = target(function(...:n@1)
    return a
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(callback: (...number) -> number) return callback(1, 2, 3) end

local x = target(function(a:number, b:number, ...:@1)
    return a + b
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("number"));
    assert_eq!(
      ac2.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_expected_argument_type_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1884:autocomplete_type_correct_expected_argument_type_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_expected_argument_type_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_expected_argument_type_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(callback: (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(callback: (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: n@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("number"));
    assert_eq!(
      ac2.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(callback: (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: n@1, b: @2)
    return a + #b
end)
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("number"));
    assert_eq!(
      ac3.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac4 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac4.entry_map.contains_key("string"));
    assert_eq!(
      ac4.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(callback: (...number) -> number) return callback(1, 2, 3) end

local x = target(function(a: n@1)
    return a
end
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac5.entry_map.contains_key("number"));
    assert_eq!(
      ac5.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_expected_argument_type_suggestion_optional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2027:autocomplete_type_correct_expected_argument_type_suggestion_optional`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_expected_argument_type_suggestion_optional

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_expected_argument_type_suggestion_optional() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
        r#"
local function target(callback: nil | (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("number"));
    assert_eq!(
      ac.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_expected_argument_type_suggestion_self {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2041:autocomplete_type_correct_expected_argument_type_suggestion_self`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_expected_argument_type_suggestion_self

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_expected_argument_type_suggestion_self() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local t = {}
t.x = 5
function t:target(callback: (a: number, b: string) -> number) return callback(self.x, "hello") end

local x = t:target(function(a: @1, b:@2 ) end)
local y = t.target(t, function(a: number, b: @3) end)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac2.entry_map.contains_key("string"));
    assert_eq!(
      ac2.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);

    assert!(ac3.entry_map.contains_key("string"));
    assert_eq!(
      ac3.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_expected_return_type_pack_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1998:autocomplete_type_correct_expected_return_type_pack_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_expected_return_type_pack_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_expected_return_type_pack_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(callback: () -> ...number) return callback() end

local x = target(function(): ...n@1
    return 1, 2, 3
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(callback: () -> ...number) return callback() end

local x = target(function(): (number, number, ...n@1
    return 1, 2, 3
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("number"));
    assert_eq!(
      ac2.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_expected_return_type_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1969:autocomplete_type_correct_expected_return_type_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_expected_return_type_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_expected_return_type_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(callback: () -> number) return callback() end

local x = target(function(): n@1
    return 1
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function target(callback: () -> (number, number)) return callback() end

local x = target(function(): (number, n@1
    return 1, 2
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("number"));
    assert_eq!(
      ac2.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_full_type_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1794:autocomplete_type_correct_full_type_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_full_type_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_full_type_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local b:@1 @2= "str"
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("string"));
    assert_eq!(
      ac1.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac2.entry_map.contains_key("string"));
    assert_eq!(
      ac2.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local b: @1= function(a: number) return -a end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("(number) -> number"));
    assert_eq!(
      ac3.entry_map["(number) -> number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_function_no_parenthesis {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2152:autocomplete_type_correct_function_no_parenthesis`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum ParenthesesRecommendation (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_function_no_parenthesis

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_function_no_parenthesis() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      parentheses_recommendation::ParenthesesRecommendation, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(a: (number) -> number) return a(4) end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("bar1"));
    assert_eq!(ac.entry_map["bar1"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac.entry_map["bar1"].parens, ParenthesesRecommendation::None);
    assert_eq!(ac.entry_map["bar2"].type_correct, TypeCorrectKind::None);
  }
}

mod autocomplete_type_correct_function_return_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1630:autocomplete_type_correct_function_return_types`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_function_return_types

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_function_return_types() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("bar1"));
    assert_eq!(
      ac1.entry_map["bar1"].type_correct,
      TypeCorrectKind::CorrectFunctionResult
    );
    assert_eq!(ac1.entry_map["bar2"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(bar1, b@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("bar2"));
    assert_eq!(
      ac2.entry_map["bar2"].type_correct,
      TypeCorrectKind::CorrectFunctionResult
    );
    assert_eq!(ac2.entry_map["bar1"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end
local function bar1(a: number): (...number) return -a, a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("bar1"));
    assert_eq!(
      ac3.entry_map["bar1"].type_correct,
      TypeCorrectKind::CorrectFunctionResult
    );
    assert_eq!(ac3.entry_map["bar2"].type_correct, TypeCorrectKind::None);
  }
}

mod autocomplete_type_correct_function_type_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1746:autocomplete_type_correct_function_type_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_function_type_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_function_type_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local b: (n@1) -> number = function(a: number, b: string) return a + #b end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
      ac1.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local b: (number, s@1 = function(a: number, b: string) return a + #b end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("string"));
    assert_eq!(
      ac2.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local b: (number, string) -> b@1 = function(a: number, b: string): boolean return a + #b == 0 end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("boolean"));
    assert_eq!(
      ac3.entry_map["boolean"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local b: (number, ...s@1) = function(a: number, ...: string) return a end
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac4.entry_map.contains_key("string"));
    assert_eq!(
      ac4.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local b: (number) -> ...s@1 = function(a: number): ...string return "a", "b", "c" end
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac5.entry_map.contains_key("string"));
    assert_eq!(
      ac5.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_keywords {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2217:autocomplete_type_correct_keywords`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_keywords

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_keywords() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function a(x: boolean) end
local function b(x: number?) end
local function c(x: (number) -> string) end
local function d(x: ((number) -> string)?) end
local function e(x: ((number) -> string) & ((boolean) -> number)) end

local tru = {}
local ni = false

local ac = a(t@1)
local bc = b(n@2)
local cc = c(f@3)
local dc = d(f@4)
local ec = e(f@5)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac1.entry_map.contains_key("tru"));
    assert_eq!(ac1.entry_map["tru"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac1.entry_map["true"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      ac1.entry_map["false"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac2 = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac2.entry_map.contains_key("ni"));
    assert_eq!(ac2.entry_map["ni"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac2.entry_map["nil"].type_correct, TypeCorrectKind::Correct);

    let ac3 = fixture.base.autocomplete_marker(b'3' as c_char);
    assert!(ac3.entry_map.contains_key("false"));
    assert_eq!(ac3.entry_map["false"].type_correct, TypeCorrectKind::None);
    assert_eq!(
      ac3.entry_map["function"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac4 = fixture.base.autocomplete_marker(b'4' as c_char);
    assert_eq!(
      ac4.entry_map["function"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac5 = fixture.base.autocomplete_marker(b'5' as c_char);
    assert_eq!(
      ac5.entry_map["function"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_local_type_suggestion {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1675:autocomplete_type_correct_local_type_suggestion`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_local_type_suggestion

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_local_type_suggestion() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local b: s@1 = "str"
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("string"));
    assert_eq!(
      ac1.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function f() return "str" end
local b: s@1 = f()
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("string"));
    assert_eq!(
      ac2.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local b: s@1, c: n@2 = "str", 2
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("string"));
    assert_eq!(
      ac3.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac4 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac4.entry_map.contains_key("number"));
    assert_eq!(
      ac4.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function f() return 1, "str", 3 end
local a: b@1, b: n@2, c: s@3, d: n@4 = false, f()
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac5.entry_map.contains_key("boolean"));
    assert_eq!(
      ac5.entry_map["boolean"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac6 = fixture.base.autocomplete_marker(b'2' as c_char);

    assert!(ac6.entry_map.contains_key("number"));
    assert_eq!(
      ac6.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac7 = fixture.base.autocomplete_marker(b'3' as c_char);

    assert!(ac7.entry_map.contains_key("string"));
    assert_eq!(
      ac7.entry_map["string"].type_correct,
      TypeCorrectKind::Correct
    );

    let ac8 = fixture.base.autocomplete_marker(b'4' as c_char);

    assert!(ac8.entry_map.contains_key("number"));
    assert_eq!(
      ac8.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );

    fixture.base.check(String::from(
      r#"
local function f(): ...number return 1, 2, 3 end
local a: boolean, b: n@1 = false, f()
    "#,
    ));

    let ac9 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac9.entry_map.contains_key("number"));
    assert_eq!(
      ac9.entry_map["number"].type_correct,
      TypeCorrectKind::Correct
    );
  }
}

mod autocomplete_type_correct_sealed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2197:autocomplete_type_correct_sealed_table`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_type_correct_sealed_table

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_sealed_table() {
    use core::ffi::c_char;

    use ulua_analysis::{
      enums::solver_mode::SolverMode, functions::to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function f(a: { x: number, y: number }) return a.x + a.y end
local fp: @1= f
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    // The skeleton fixture currently runs the translated old solver path.
    if !FFlag::DebugLuauForceOldSolver.get()
      && fixture.base.get_frontend().get_luau_solver_mode() == SolverMode::New
    {
      assert_eq!(
        "({ x: number, y: number }) -> number",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
      );
    } else {
      assert_eq!(
        "({ x: number, y: number }) -> (...any)",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
      );
    }

    assert!(
      ac.entry_map
        .contains_key("({ x: number, y: number }) -> number")
    );
  }
}

mod autocomplete_type_correct_suggestion_for_overloads {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2259:autocomplete_type_correct_suggestion_for_overloads`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_suggestion_for_overloads

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_suggestion_for_overloads() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_common::FFlag;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local target: ((number) -> string) & ((string) -> number))

local one = 4
local two = "hello"
return target(o@1)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("one"));
    assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::Correct);

    fixture.base.check(String::from(
      r#"
local target: ((number) -> string) & ((number) -> number))

local one = 4
local two = "hello"
return target(o@1)
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("one"));
    assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local target: ((number, number) -> string) & ((string) -> number))

local one = 4
local two = "hello"
return target(1, o@1)
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("one"));
    assert_eq!(ac3.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac3.entry_map["two"].type_correct, TypeCorrectKind::None);
  }
}

mod autocomplete_type_correct_suggestion_in_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1528:autocomplete_type_correct_suggestion_in_argument`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - calls -> function match (VM/src/lstrlib.cpp)
  //!   - translates_to -> rust_item autocomplete_type_correct_suggestion_in_argument

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_suggestion_in_argument() {
    use core::ffi::c_char;

    use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local one = 4
local two = "hello"
return target(o@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("one"));
    assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local one = 4
local two = "hello"
return target(one, t@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("two"));
    assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local a = { one = 4, two = "hello" }
return target(a.@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac3.entry_map.contains_key("one"));
    assert_eq!(ac3.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac3.entry_map["two"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local function target(a: number, b: string) return a + #b end

local a = { one = 4, two = "hello" }
return target(a.one, a.@1
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac4.entry_map.contains_key("two"));
    assert_eq!(ac4.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac4.entry_map["one"].type_correct, TypeCorrectKind::None);

    fixture.base.check(String::from(
      r#"
local function target(a: string?) return #b end

local a = { one = 4, two = "hello" }
return target(a.@1
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac5.entry_map.contains_key("two"));
    assert_eq!(ac5.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac5.entry_map["one"].type_correct, TypeCorrectKind::None);
  }
}

mod autocomplete_type_correct_suggestion_in_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1601:autocomplete_type_correct_suggestion_in_table`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_type_correct_suggestion_in_table

  #[cfg(test)]
  #[test]
  fn autocomplete_type_correct_suggestion_in_table() {
    use core::ffi::c_char;

    use ulua_analysis::enums::{
      autocomplete_context::AutocompleteContext, type_correct_kind::TypeCorrectKind,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type Foo = { a: number, b: string }
local a = { one = 4, two = "hello" }
local b: Foo = { a = a.@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac1.entry_map.contains_key("one"));
    assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac1.context, AutocompleteContext::Property);

    fixture.base.check(String::from(
      r#"
type Foo = { a: number, b: string }
local a = { one = 4, two = "hello" }
local b: Foo = { b = a.@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac2.entry_map.contains_key("two"));
    assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac2.context, AutocompleteContext::Property);
  }
}

mod autocomplete_type_function_eval_in_autocomplete {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4578:autocomplete_type_function_eval_in_autocomplete`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item autocomplete_type_function_eval_in_autocomplete
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_type_function_eval_in_autocomplete() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
type function foo(x)
    local tbl = types.newtable(nil, nil, nil)
    tbl:setproperty(types.singleton("boolean"), x)
    tbl:setproperty(types.singleton("number"), types.number)
    return tbl
end

local function test(a: foo<string>)
    return a.@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("boolean"));
    assert!(ac.entry_map.contains_key("number"));
  }
}

mod autocomplete_type_function_has_types_definitions {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4533:autocomplete_type_function_has_types_definitions`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_type_function_has_types_definitions
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_type_function_has_types_definitions() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(String::from(
      r#"
type function foo()
    types.@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("singleton"));
  }
}

mod autocomplete_type_function_private_scope {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4547:autocomplete_type_function_private_scope`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item autocomplete_type_function_private_scope
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_type_function_private_scope() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_analysis::{
      functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
      records::{binding::Binding, frontend::Frontend},
    };
    use ulua_ast::records::location::Location;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_builtins_fixture::ACBuiltinsFixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    {
      let frontend = fixture.base.get_frontend() as *mut Frontend;
      unsafe {
        let any_type = (*(*frontend).builtin_types).any_type;
        let binding = || Binding {
          type_id: any_type,
          location: Location::default(),
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        };

        add_global_binding_builtin_definitions_alt_b(
          &mut (*frontend).globals,
          "thisAlsoShouldNotBeThere",
          binding(),
        );
        add_global_binding_builtin_definitions_alt_b(
          &mut (*frontend).globals_for_autocomplete,
          "thisAlsoShouldNotBeThere",
          binding(),
        );
      }
    }

    fixture.base.check(String::from(
      r#"
local function thisShouldNotBeThere() end

type function thisShouldBeThere() end

type function foo()
    this@1
end

this@2
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(!ac.entry_map.contains_key("thisShouldNotBeThere"));
    assert!(!ac.entry_map.contains_key("thisAlsoShouldNotBeThere"));
    assert!(ac.entry_map.contains_key("thisShouldBeThere"));

    let ac = fixture.base.autocomplete_marker(b'2' as c_char);
    assert!(ac.entry_map.contains_key("thisShouldNotBeThere"));
    assert!(ac.entry_map.contains_key("thisAlsoShouldNotBeThere"));
    assert!(!ac.entry_map.contains_key("thisShouldBeThere"));
  }
}

mod autocomplete_type_scoping_easy {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1370:autocomplete_type_scoping_easy`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item autocomplete_type_scoping_easy

  #[cfg(test)]
  #[test]
  fn autocomplete_type_scoping_easy() {
    use core::ffi::c_char;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::table_type::TableType,
    };
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
    local a: T@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("Table"));
    let ty = ac.entry_map["Table"]
      .r#type
      .expect("Table entry should have a type");
    let ty = follow_type_id(ty);
    let table = get_type_id::<TableType>(ty).expect("Table should be a table");
    assert!(table.props.contains_key("x"));
  }
}

mod autocomplete_unsealed_table {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:389:autocomplete_unsealed_table`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_unsealed_table

  #[cfg(test)]
  #[test]
  fn autocomplete_unsealed_table() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local tbl = {}
        tbl.prop = 5
        tbl.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("prop"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_unsealed_table_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:403:autocomplete_unsealed_table_2`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - translates_to -> rust_item autocomplete_unsealed_table_2

  #[cfg(test)]
  #[test]
  fn autocomplete_unsealed_table_2() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local tbl = {}
        local inner = { prop = 5 }
        tbl.inner = inner
        tbl.inner. @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("prop"));
    assert_eq!(ac.context, AutocompleteContext::Property);
  }
}

mod autocomplete_user_defined_globals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:225:autocomplete_user_defined_globals`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
  //!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
  //!   - translates_to -> rust_item autocomplete_user_defined_globals

  #[cfg(test)]
  #[test]
  fn autocomplete_user_defined_globals() {
    use core::ffi::c_char;

    use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from("local myLocal = 4; @1"));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("myLocal"));
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Statement);
  }
}

mod autocomplete_user_defined_local_functions_in_own_definition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:286:autocomplete_user_defined_local_functions_in_own_definition`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_user_defined_local_functions_in_own_definition

  #[cfg(test)]
  #[test]
  fn autocomplete_user_defined_local_functions_in_own_definition() {
    use core::ffi::c_char;

    use ulua_unit_test::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        local function abc()
@1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("abc"));
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));

    fixture.base.check(String::from(
      r#"
        local abc = function()
@1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);

    assert!(ac.entry_map.contains_key("abc"));
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("math"));
  }
}

mod autocomplete_we_know_the_fields_of_a_class_instance {
  //! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5016:autocomplete_we_know_the_fields_of_a_class_instance`
  //! Source: `tests/Autocomplete.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Autocomplete.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
  //!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file Common/include/Luau/StringUtils.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/Autocomplete.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
  //!   - translates_to -> rust_item autocomplete_we_know_the_fields_of_a_class_instance
  use super::*;

  #[cfg(test)]
  #[test]
  fn autocomplete_we_know_the_fields_of_a_class_instance() {
    use alloc::string::String;
    use core::ffi::c_char;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::ac_fixture::AcFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(String::from(
      r#"
        class Point2d
            public x: number
            public y: number
        end

        local p = Point2d.new { x=3, y=4 }

        local q = p.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as c_char);
    assert!(ac.entry_map.contains_key("x"));
    assert!(ac.entry_map.contains_key("y"));
    assert!(!ac.entry_map.contains_key("z"));
  }
}
