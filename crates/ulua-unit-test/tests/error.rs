extern crate alloc;

mod error_binary_op_type_function_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:39:error_binary_op_type_function_errors`
  //! Source: `tests/Error.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Error.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Error.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item error_binary_op_type_function_errors

  #[cfg(test)]
  #[test]
  fn error_binary_op_type_function_errors() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag::DebugLuauForceOldSolver;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local x = 1 + "foo"
    "#,
      ),
      None,
    );

    assert_eq!(result.errors.len(), 1);

    if !DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod error_metatable_names_show_instead_of_tables {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:19:error_metatable_names_show_instead_of_tables`
  //! Source: `tests/Error.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Error.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Error.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item error_metatable_names_show_instead_of_tables

  #[cfg(test)]
  #[test]
  fn error_metatable_names_show_instead_of_tables() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local Account = {}
Account.__index = Account
function Account.deposit(self: Account, x: number)
	self.balance += x
end
type Account = typeof(setmetatable({} :: { balance: number }, Account))
local x: Account = 5
"#,
      ),
      None,
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      "Expected this to be 'Account', but got 'number'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod error_type_error_code_should_return_nonzero_code {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:13:error_type_error_code_should_return_nonzero_code`
  //! Source: `tests/Error.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Error.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Error.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - translates_to -> rust_item error_type_error_code_should_return_nonzero_code

  #[cfg(test)]
  #[test]
  fn error_type_error_code_should_return_nonzero_code() {
    use ulua_analysis::records::{
      type_error::TypeError,
      unknown_symbol::{Context, UnknownSymbol},
    };
    use ulua_ast::records::{location::Location, position::Position};

    let e = TypeError::type_error_location_type_error_data(
      Location {
        begin: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 1 },
      },
      UnknownSymbol::new("Foo".to_string(), Context::Binding).into(),
    );

    assert!(e.code() >= 1000);
  }
}

mod error_unary_op_type_function_errors {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:59:error_unary_op_type_function_errors`
  //! Source: `tests/Error.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Error.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Error.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item error_unary_op_type_function_errors

  #[cfg(test)]
  #[test]
  fn error_unary_op_type_function_errors() {
    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag::DebugLuauForceOldSolver;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local x = -"foo"
    "#,
      ),
      None,
    );

    if !DebugLuauForceOldSolver.get() {
      assert_eq!(result.errors.len(), 2);
      assert_eq!(
        "Operator '-' could not be applied to operand of type string; there is no corresponding overload for __unm",
        to_string_type_error(&result.errors[0])
      );
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[1])
      );
    } else {
      assert_eq!(result.errors.len(), 1);
      assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}
