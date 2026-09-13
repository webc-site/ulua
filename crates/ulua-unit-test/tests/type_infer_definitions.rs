extern crate alloc;

mod type_infer_definitions_class_definition_function_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:267:type_infer_definitions_class_definition_function_prop`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_class_definition_function_prop

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definition_function_prop() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            X: (number) -> string
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: Foo = Foo.new()
        local prop = x.X
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(number) -> string",
      to_string_type_id(fixture.require_type_string(&String::from("prop")))
    );
  }
}

mod type_infer_definitions_class_definition_indexer {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:485:type_infer_definitions_class_definition_indexer`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_definitions_class_definition_indexer

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definition_indexer() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::extern_type::ExternType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            [number]: string
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: Foo
        local y = x[1]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let etv = get_type_id::<ExternType>(fixture.require_type_string(&String::from("x")))
      .expect("expected x to have ExternType");
    let indexer = etv.indexer.expect("Foo indexer");
    assert_eq!("number", to_string_type_id(indexer.index_type));
    assert_eq!("string", to_string_type_id(indexer.index_result_type));

    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_definitions_class_definition_malformed_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:464:type_infer_definitions_class_definition_malformed_string`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method ParseError::getMessage (Ast/src/Parser.cpp)
  //!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function escape (Common/src/StringUtils.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_class_definition_malformed_string

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definition_malformed_string() {
    use alloc::string::String;

    use ulua_analysis::records::frontend::Frontend;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
      let target_scope = (*frontend_ptr).globals.global_scope();
      (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        r#"
        declare extern type Foo with
            ["a\0property"]: string
        end
    "#,
        String::from("@test"),
        false,
        false,
      )
    };

    assert!(!result.success);
    assert_eq!(1, result.parse_result.errors.len());
    assert_eq!(
      "String literal contains malformed escape sequence or \\0",
      result.parse_result.errors[0].get_message()
    );
  }
}

mod type_infer_definitions_class_definition_overload_metamethods {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:421:type_infer_definitions_class_definition_overload_metamethods`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_class_definition_overload_metamethods

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definition_overload_metamethods() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Vector3 with
        end

        declare extern type CFrame with
            function __mul(self, other: CFrame): CFrame
            function __mul(self, other: Vector3): Vector3
        end

        declare function newVector3(): Vector3
        declare function newCFrame(): CFrame
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local base = newCFrame()
        local shouldBeCFrame = base * newCFrame()
        local shouldBeVector = base * newVector3()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "CFrame",
      to_string_type_id(fixture.require_type_string(&String::from("shouldBeCFrame")))
    );
    assert_eq!(
      "Vector3",
      to_string_type_id(fixture.require_type_string(&String::from("shouldBeVector")))
    );
  }
}

mod type_infer_definitions_class_definition_string_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:447:type_infer_definitions_class_definition_string_props`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_class_definition_string_props

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definition_string_props() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            ["a property"]: string
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: Foo
        local y = x["a property"]
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod type_infer_definitions_class_definitions_cannot_extend_non_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:193:type_infer_definitions_class_definitions_cannot_extend_non_class`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_definitions_class_definitions_cannot_extend_non_class

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definitions_cannot_extend_non_class() {
    use alloc::string::String;

    use ulua_analysis::{
      records::frontend::Frontend, type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
      let target_scope = (*frontend_ptr).globals.global_scope();
      (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        r#"
        type NotAClass = {}

        declare extern type Foo extends NotAClass with
        end
    "#,
        String::from("@test"),
        false,
        false,
      )
    };

    assert!(!result.success);
    assert_eq!(0, result.parse_result.errors.len());
    let module = result.module.as_ref().expect("checked definition module");
    assert_eq!(1, module.errors.len(), "{:?}", module.errors);

    let ge = match &module.errors[0].data {
      TypeErrorData::GenericError(ge) => ge,
      other => panic!("expected GenericError, got {:?}", other),
    };
    assert_eq!(
      "Cannot use non-class type 'NotAClass' as a superclass of class 'Foo'",
      ge.message()
    );
  }
}

mod type_infer_definitions_class_definitions_cannot_overload_non_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:153:type_infer_definitions_class_definitions_cannot_overload_non_function`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_class_definitions_cannot_overload_non_function

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definitions_cannot_overload_non_function() {
    use alloc::string::String;

    use ulua_analysis::{
      records::frontend::Frontend, type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
      let target_scope = (*frontend_ptr).globals.global_scope();
      (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        r#"
        declare extern type A with
            X: number
            X: string
        end
    "#,
        String::from("@test"),
        false,
        false,
      )
    };

    assert!(!result.success);
    assert_eq!(0, result.parse_result.errors.len());
    let module = result.module.as_ref().expect("checked definition module");

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, module.errors.len(), "{:?}", module.errors);
    } else {
      assert_eq!(1, module.errors.len(), "{:?}", module.errors);
    }

    let ge = match &module.errors[0].data {
      TypeErrorData::GenericError(ge) => ge,
      other => panic!("expected GenericError, got {:?}", other),
    };
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Cannot overload read type of non-function extern type member 'X'",
        ge.message()
      );
      let ge2 = match &module.errors[1].data {
        TypeErrorData::GenericError(ge) => ge,
        other => panic!("expected GenericError, got {:?}", other),
      };
      assert_eq!(
        "Cannot overload write type of non-function extern type member 'X'",
        ge2.message()
      );
    } else {
      assert_eq!(
        "Cannot overload non-function class member 'X'",
        ge.message()
      );
    }
  }
}

mod type_infer_definitions_class_definitions_reference_other_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:511:type_infer_definitions_class_definitions_reference_other_extern_types`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_class_definitions_reference_other_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_class_definitions_reference_other_extern_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Channel with
            Messages: { Message }
            OnMessage: (message: Message) -> ()
        end

        declare extern type Message with
            Text: string
            Channel: Channel
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: Channel
        local b = a.Messages[1]
        local c = b.Channel
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Channel",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
      "Message",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "Channel",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_definitions_cli_142285_reduce_minted_union_func {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:572:type_infer_definitions_cli_142285_reduce_minted_union_func`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
  //!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item type_infer_definitions_cli_142285_reduce_minted_union_func

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_cli_142285_reduce_minted_union_func() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function middle(a: number, b: number): number
            return math.ceil((a + b) / 2 - 0.5)
        end

        local function find<T>(array: {T}, item: T): number?
            local l, m, r = 1, middle(1, #array), #array
            while l <= r do
                if item <= array[m] then
                    if item == array[m] then return m end
                    m, r = middle(l, m-1), m-1
                else
                    l, m = middle(m+1, r), m+1
                end
            end
        return nil
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = match &result.errors[0].data {
      TypeErrorData::CannotInferBinaryOperation(err) => err,
      other => panic!("expected CannotInferBinaryOperation, got {:?}", other),
    };
    assert_eq!(Some("item"), err.suggested_to_annotate());
    assert_eq!(AstExprBinaryOp::CompareLe, err.op());
  }
}

mod type_infer_definitions_declaring_generic_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:240:type_infer_definitions_declaring_generic_functions`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_declaring_generic_functions

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_declaring_generic_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare function f<a, b>(a: a, b: b): string
        declare function g<a..., b...>(...: a...): b...
        declare function h<a, b>(a: a, b: b): (b, a)
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = f(1, true)
        local y: number, z: string = g("foo", 123)
        local w, u = h(1, true)

        local f = f
        local g = g
        local h = h
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_string(&String::from("w")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("u")))
    );
    assert_eq!(
      "<a, b>(a, b) -> string",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
    assert_eq!(
      "<a..., b...>(a...) -> (b...)",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
    assert_eq!(
      "<a, b>(a, b) -> (b, a)",
      to_string_type_id(fixture.require_type_string(&String::from("h")))
    );
  }
}

mod type_infer_definitions_definition_file_class_function_args {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:288:type_infer_definitions_definition_file_class_function_args`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - translates_to -> rust_item type_infer_definitions_definition_file_class_function_args

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definition_file_class_function_args() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            function foo1(self, x: number): number
            function foo2(self, x: number, y: string): number

            y: (a: number, b: string) -> string
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: Foo = Foo.new()
        local methodRef1 = x.foo1
        local methodRef2 = x.foo2
        local prop = x.y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      function_type_arguments: true,
      ..Default::default()
    };
    assert_eq!(
      "(self: Foo, x: number) -> number",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("methodRef1")),
        &mut opts
      )
    );
    assert_eq!(
      "(self: Foo, x: number, y: string) -> number",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("methodRef2")),
        &mut opts
      )
    );
    assert_eq!(
      "(a: number, b: string) -> string",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("prop")),
        &mut opts
      )
    );
  }
}

mod type_infer_definitions_definition_file_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:115:type_infer_definitions_definition_file_extern_types`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_definition_file_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definition_file_extern_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
            X: number

            function inheritance(self): number
        end

        declare extern type Bar extends Foo with
            Y: number

            function foo(self, x: number): number
            function foo(self, x: string): string

            function __add(self, other: Bar): Bar
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: Bar
        local prop: number = x.Y
        local inheritedProp: number = x.X
        local method: number = x:foo(1)
        local method2: string = x:foo("string")
        local metamethod: Bar = x + x
        local inheritedMethod: number = x:inheritance()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("prop")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("inheritedProp")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("method")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("method2")))
    );
    assert_eq!(
      "Bar",
      to_string_type_id(fixture.require_type_string(&String::from("metamethod")))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("inheritedMethod")))
    );
  }
}

mod type_infer_definitions_definition_file_has_source_module_name_set {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:538:type_infer_definitions_definition_file_has_source_module_name_set`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item type_infer_definitions_definition_file_has_source_module_name_set

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definition_file_has_source_module_name_set() {
    use alloc::string::String;

    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::extern_type::ExternType};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.load_definition(
      &String::from(
        r#"
        declare extern type Foo with
        end
    "#,
      ),
      false,
    );

    assert!(result.success);
    assert_eq!("@test", result.source_module.name);
    assert_eq!("@test", result.source_module.human_readable_name);

    let frontend = fixture.get_frontend();
    let foo_ty = frontend
      .globals
      .global_scope()
      .lookup_type(&String::from("Foo"))
      .expect("Foo type binding");
    let etv = get_type_id::<ExternType>(foo_ty.r#type()).expect("expected Foo extern type");
    assert_eq!("@test", &etv.definition_module_name);
  }
}

mod type_infer_definitions_definition_file_loading {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:44:type_infer_definitions_definition_file_loading`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_definition_file_loading

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definition_file_loading() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      get_global_binding::get_global_binding, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare foo: number
        export type Asdf = number | string
        declare function bar(x: number): string
        declare foo2: typeof(foo)
        declare function var(...: any): string
    "#,
      ),
      false,
    );

    let frontend = fixture.get_frontend();
    let global_foo_ty = get_global_binding(&mut frontend.globals, "foo");
    assert_eq!("number", to_string_type_id(global_foo_ty));

    let global_asdf_ty = frontend
      .globals
      .global_scope()
      .lookup_type(&String::from("Asdf"))
      .expect("Asdf type binding");
    assert_eq!(
      "number | string",
      to_string_type_id(global_asdf_ty.r#type())
    );

    let global_bar_ty = get_global_binding(&mut frontend.globals, "bar");
    assert_eq!("(number) -> string", to_string_type_id(global_bar_ty));

    let global_foo2_ty = get_global_binding(&mut frontend.globals, "foo2");
    assert_eq!("number", to_string_type_id(global_foo2_ty));

    let global_var_ty = get_global_binding(&mut frontend.globals, "var");
    assert_eq!("(...any) -> string", to_string_type_id(global_var_ty));

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: number = foo + 1
        local y: string = bar(x)
        local z: Asdf = x
        z = y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_definitions_definition_file_simple {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:17:type_infer_definitions_definition_file_simple`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_definition_file_simple

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definition_file_simple() {
    use alloc::string::String;

    use ulua_analysis::functions::{
      get_global_binding::get_global_binding, to_string_to_string_alt_c::to_string_type_id,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare foo: number
        declare function bar(x: number): string
        declare foo2: typeof(foo)
    "#,
      ),
      false,
    );

    let frontend = fixture.get_frontend();
    let global_foo_ty = get_global_binding(&mut frontend.globals, "foo");
    assert_eq!("number", to_string_type_id(global_foo_ty));

    let global_bar_ty = get_global_binding(&mut frontend.globals, "bar");
    assert_eq!("(number) -> string", to_string_type_id(global_bar_ty));

    let global_foo2_ty = get_global_binding(&mut frontend.globals, "foo2");
    assert_eq!("number", to_string_type_id(global_foo2_ty));

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: number = foo - 1
        local y: string = bar(x)
        local z: number | string = x
        z = y
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_definitions_definitions_documentation_symbols {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:318:type_infer_definitions_definitions_documentation_symbols`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_definitions_definitions_documentation_symbols

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definitions_documentation_symbols() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_mutable_type::get_mutable_type_id, try_get_global_binding::try_get_global_binding,
      },
      records::{extern_type::ExternType, table_type::TableType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare x: string

        export type Foo = string | number

        declare extern type Bar with
            prop: string
        end

        declare y: {
            x: number,
        }
    "#,
      ),
      false,
    );

    let frontend = fixture.get_frontend();

    let x_binding = try_get_global_binding(&mut frontend.globals, "x").expect("x binding");
    assert_eq!(
      Some(String::from("@test/global/x")),
      x_binding.documentation_symbol
    );

    let foo_ty = frontend
      .globals
      .global_scope()
      .lookup_type(&String::from("Foo"))
      .expect("Foo type binding");
    assert_eq!(Some(String::from("@test/globaltype/Foo")), unsafe {
      (*foo_ty.r#type()).documentation_symbol.clone()
    });

    let bar_ty = frontend
      .globals
      .global_scope()
      .lookup_type(&String::from("Bar"))
      .expect("Bar type binding");
    assert_eq!(Some(String::from("@test/globaltype/Bar")), unsafe {
      (*bar_ty.r#type()).documentation_symbol.clone()
    });

    let bar_class =
      get_mutable_type_id::<ExternType>(bar_ty.r#type()).expect("expected Bar extern type");
    let bar_prop = bar_class.props.get("prop").expect("Bar.prop");
    assert_eq!(
      Some(String::from("@test/globaltype/Bar.prop")),
      bar_prop.documentation_symbol.clone()
    );

    let y_binding = try_get_global_binding(&mut frontend.globals, "y").expect("y binding");
    assert_eq!(
      Some(String::from("@test/global/y")),
      y_binding.documentation_symbol
    );

    let y_table =
      get_mutable_type_id::<TableType>(y_binding.type_id).expect("expected y table type");
    let y_prop = y_table.props.get("x").expect("y.x");
    assert_eq!(
      Some(String::from("@test/global/y.x")),
      y_prop.documentation_symbol.clone()
    );
  }
}

mod type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:362:type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_mutable_type::get_mutable_type_id,
      records::{extern_type::ExternType, function_type::FunctionType},
    };
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type MyClass with
            function myMethod(self)
        end

        declare function myFunc(): MyClass
    "#,
      ),
      false,
    );

    let frontend = fixture.get_frontend();
    let my_class_ty = frontend
      .globals
      .global_scope()
      .lookup_type(&String::from("MyClass"))
      .expect("MyClass type binding");
    assert_eq!(Some(String::from("@test/globaltype/MyClass")), unsafe {
      (*my_class_ty.r#type()).documentation_symbol.clone()
    });

    let class = get_mutable_type_id::<ExternType>(my_class_ty.r#type())
      .expect("expected MyClass extern type");
    let method = class.props.get("myMethod").expect("myMethod");
    assert_eq!(
      Some(String::from("@test/globaltype/MyClass.myMethod")),
      method.documentation_symbol.clone()
    );

    let method_ty = method.read_ty.expect("myMethod read type");
    let function =
      get_mutable_type_id::<FunctionType>(method_ty).expect("expected myMethod function type");
    let definition = function.definition().expect("myMethod function definition");

    assert_eq!(
      Some(&String::from("@test")),
      definition.definition_module_name()
    );
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 12,
        },
        end: Position {
          line: 2,
          column: 35,
        },
      },
      definition.definition_location()
    );
    assert_eq!(None, definition.vararg_location());
    assert_eq!(
      Location {
        begin: Position {
          line: 2,
          column: 21,
        },
        end: Position {
          line: 2,
          column: 29,
        },
      },
      definition.original_name_location()
    );
  }
}

mod type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:394:type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
  //!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        export type Evil = string
    "#,
      ),
      false,
    );

    let frontend = fixture.get_frontend();
    let ty = frontend
      .globals
      .global_scope()
      .lookup_type(&String::from("Evil"))
      .expect("Evil type binding");

    assert_eq!(None, unsafe { (*ty.r#type()).documentation_symbol.clone() });
  }
}

mod type_infer_definitions_extern_read_write_dual_attribute {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:693:type_infer_definitions_extern_read_write_dual_attribute`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_definitions_extern_read_write_dual_attribute

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_extern_read_write_dual_attribute() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type dual_attribute with
            read value: boolean
            write value: number
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local da: dual_attribute
local x: boolean = da.value
local y: number = da.value
da.value = 5
da.value = false
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      result.errors[0].data,
      TypeErrorData::TypeMismatch(_)
    ));
    assert!(matches!(
      result.errors[1].data,
      TypeErrorData::TypeMismatch(_)
    ));
    assert_eq!(4, result.errors[0].location.begin.line);
    assert_eq!(6, result.errors[1].location.begin.line);
  }
}

mod type_infer_definitions_extern_writeonly_props {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:661:type_infer_definitions_extern_writeonly_props`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_definitions_extern_writeonly_props

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_extern_writeonly_props() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type noread with
            write value: number
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local function read(v: buffer | boolean)
end

local function foo(bar: noread)
    bar.value = 42
    bar.value += -15
    read(bar.value)
    read(bar.value > 15)
end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      result.errors[0].data,
      TypeErrorData::PropertyAccessViolation(_)
    ));
    assert!(matches!(
      result.errors[1].data,
      TypeErrorData::PropertyAccessViolation(_)
    ));
    assert!(matches!(
      result.errors[2].data,
      TypeErrorData::PropertyAccessViolation(_)
    ));
    assert_eq!(7, result.errors[0].location.begin.line);
    assert_eq!(8, result.errors[1].location.begin.line);
    assert_eq!(9, result.errors[2].location.begin.line);
  }
}

mod type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:81:type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> function tryGetGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::try_get_global_binding::try_get_global_binding, records::frontend::Frontend,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let parse_fail_result = unsafe {
      let target_scope = (*frontend_ptr).globals.global_scope();
      (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        r#"
        declare foo
    "#,
        String::from("@test"),
        false,
        false,
      )
    };

    assert!(!parse_fail_result.success);
    let foo_ty = unsafe { try_get_global_binding(&mut (*frontend_ptr).globals, "foo") };
    assert!(foo_ty.is_none());

    let check_fail_result = unsafe {
      let target_scope = (*frontend_ptr).globals.global_scope();
      (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        r#"
        local foo: string = 123
        declare bar: typeof(foo)
    "#,
        String::from("@test"),
        false,
        false,
      )
    };

    assert!(!check_fail_result.success);
    let bar_ty = unsafe { try_get_global_binding(&mut (*frontend_ptr).globals, "bar") };
    assert!(bar_ty.is_none());
  }
}

mod type_infer_definitions_no_cyclic_defined_extern_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:219:type_infer_definitions_no_cyclic_defined_extern_types`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_no_cyclic_defined_extern_types

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_no_cyclic_defined_extern_types() {
    use alloc::string::String;

    use ulua_analysis::records::frontend::Frontend;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
      let target_scope = (*frontend_ptr).globals.global_scope();
      (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        r#"
        declare extern type Foo extends Bar with
        end

        declare extern type Bar extends Foo with
        end
    "#,
        String::from("@test"),
        false,
        false,
      )
    };

    assert!(!result.success);
  }
}

mod type_infer_definitions_recursive_redefinition_reduces_rightfully {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:559:type_infer_definitions_recursive_redefinition_reduces_rightfully`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_definitions_recursive_redefinition_reduces_rightfully

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_recursive_redefinition_reduces_rightfully() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t: {[string]: string} = {}

        local function f()
            t = t
        end

        t = t
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_definitions_single_class_type_identity_in_global_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:405:type_infer_definitions_single_class_type_identity_in_global_types`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_single_class_type_identity_in_global_types

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_single_class_type_identity_in_global_types() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
declare extern type Cls with
end

declare GetCls: () -> (Cls)
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local s : Cls = GetCls()
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_definitions_vector3_overflow {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:603:type_infer_definitions_vector3_overflow`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_definitions_vector3_overflow

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_vector3_overflow() {
    use alloc::string::String;

    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 0);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type Vector3 with
            function __add(self, other: Vector3): Vector3
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local function graphPoint(t : number, points : { Vector3 }) : Vector3
    local n : number = #points - 1
    local p : Vector3 = (nil :: any)
    for i = 0, n do
        local x = points[i + 1]
        p = p and p + x or x
    end
    return p
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_definitions_vector_readonly {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:630:type_infer_definitions_vector_readonly`
  //! Source: `tests/TypeInfer.definitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_definitions_vector_readonly

  #[cfg(test)]
  #[test]
  fn type_infer_definitions_vector_readonly() {
    use alloc::string::String;

    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
      &String::from(
        r#"
        declare extern type vector with
            read x: number
        end
    "#,
      ),
      false,
    );

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!strict
local function read(n: number | boolean)
end

local function foo(vec: vector)
    read(vec.x)
    read(vec.x > 42)
    vec.x = 15
    vec.x -= 15
end
    "#,
      ),
      None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
      result.errors[0].data,
      TypeErrorData::PropertyAccessViolation(_)
    ));
    assert!(matches!(
      result.errors[1].data,
      TypeErrorData::PropertyAccessViolation(_)
    ));
    assert_eq!(8, result.errors[0].location.begin.line);
    assert_eq!(9, result.errors[1].location.begin.line);
  }
}
