extern crate alloc;

mod frontend_accumulate_cached_errors {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:772:frontend_accumulate_cached_errors`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_accumulate_cached_errors

  #[cfg(test)]
  #[test]
  fn frontend_accumulate_cached_errors() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        local n: number = 'five'
        return {n=n}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/B"),
      String::from(
        r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 'seven'
        print(A, b)
    "#,
      ),
    );

    let result1 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);

    assert_eq!(2, result1.errors.len(), "{:?}", result1.errors);

    assert_eq!("Modules/A", result1.errors[0].module_name);
    assert_eq!("Modules/B", result1.errors[1].module_name);

    let result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);

    assert_eq!(2, result2.errors.len(), "{:?}", result2.errors);

    assert_eq!("Modules/A", result2.errors[0].module_name);
    assert_eq!("Modules/B", result2.errors[1].module_name);
  }
}

mod frontend_accumulate_cached_errors_in_consistent_order {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:801:frontend_accumulate_cached_errors_in_consistent_order`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_accumulate_cached_errors_in_consistent_order

  #[cfg(test)]
  #[test]
  fn frontend_accumulate_cached_errors_in_consistent_order() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        a = 1
        b = 2
        local Modules = script.Parent
        local A = require(Modules.B)
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/B"),
      String::from(
        r#"
        d = 3
        e = 4
        return {}
    "#,
      ),
    );

    let result1 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    assert_eq!(4, result1.errors.len(), "{:?}", result1.errors);

    assert_eq!("Modules/A", result1.errors[2].module_name);
    assert_eq!("Modules/A", result1.errors[3].module_name);

    assert_eq!("Modules/B", result1.errors[0].module_name);
    assert_eq!("Modules/B", result1.errors[1].module_name);

    let result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);
    assert_eq!(4, result2.errors.len(), "{:?}", result2.errors);

    for (left, right) in result1.errors.iter().zip(result2.errors.iter()) {
      assert_eq!(left, right);
    }
  }
}

mod frontend_any_annotation_breaks_cycle {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:246:frontend_any_annotation_breaks_cycle`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_any_annotation_breaks_cycle

  #[cfg(test)]
  #[test]
  fn frontend_any_annotation_breaks_cycle() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A) :: any
        return {hello = A.hello}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod frontend_ast_node_at_position {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1027:frontend_ast_node_at_position`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - translates_to -> rust_item frontend_ast_node_at_position

  #[cfg(test)]
  #[test]
  fn frontend_ast_node_at_position() {
    use alloc::string::String;

    use ulua_analysis::functions::find_node_at_position_ast_query::find_node_at_position_source_module_position;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local t = {}

        function t:aa() end

        t:
    "#,
      ),
      None,
    );

    let module = fixture.base.base.get_main_source_module();
    let mut pos = unsafe { (*(*module).root).base.base.location.end };
    let node = unsafe { find_node_at_position_source_module_position(&*module, pos) };

    assert!(!node.is_null());
    assert!(unsafe { !(*node).as_expr().is_null() });

    pos.column += 1;
    let node2 = unsafe { find_node_at_position_source_module_position(&*module, pos) };
    assert_eq!(node, node2);
  }
}

mod frontend_attribute_ices_to_the_correct_module {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1328:frontend_attribute_ices_to_the_correct_module`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - type_ref -> record InternalCompilerError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item frontend_attribute_ices_to_the_correct_module

  #[cfg(test)]
  #[test]
  fn frontend_attribute_ices_to_the_correct_module() {
    use alloc::string::String;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/one"),
      String::from(
        r#"
        require(game.two)
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/two"),
      String::from(
        r#"
        local a: _luau_ice
    "#,
      ),
    );

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/one"), None);
    }));

    let panic = result.expect_err("expected an InternalCompilerError");
    let ice = panic
      .downcast_ref::<InternalCompilerError>()
      .expect("expected InternalCompilerError panic payload");
    assert_eq!(Some(String::from("game/two")), ice.module_name.clone());
  }
}

mod frontend_automatically_check_cyclically_dependent_scripts {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:169:frontend_automatically_check_cyclically_dependent_scripts`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ModuleHasCyclicDependency (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item frontend_automatically_check_cyclically_dependent_scripts

  #[cfg(test)]
  #[test]
  fn frontend_automatically_check_cyclically_dependent_scripts() {
    use alloc::string::String;

    use ulua_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        require(Modules.C)
        return {}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        do local A = require(Modules.A) end
        return {}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/D"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        do local A = require(Modules.A) end
        return {}
    "#,
      ),
    );

    let result1 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);
    assert_eq!(4, result1.errors.len(), "{:?}", result1.errors);

    assert!(
      type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[0]).is_some(),
      "Should have been a ModuleHasCyclicDependency: {:?}",
      result1.errors[0]
    );
    assert!(
      type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[1]).is_some(),
      "Should have been a ModuleHasCyclicDependency: {:?}",
      result1.errors[1]
    );
    assert!(
      type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[2]).is_some(),
      "Should have been a ModuleHasCyclicDependency: {:?}",
      result1.errors[2]
    );

    let result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/D"), None);
    assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
  }
}

mod frontend_automatically_check_dependent_scripts {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:147:frontend_automatically_check_dependent_scripts`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item frontend_automatically_check_dependent_scripts

  #[cfg(test)]
  #[test]
  fn frontend_automatically_check_dependent_scripts() {
    use alloc::string::String;

    use ulua_analysis::functions::{first::first, to_string_to_string_alt_c::to_string_type_id};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Gui/Modules/B"));
    assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);

    let b_exports = first(b_module.return_type, true).expect("expected module return type");

    assert_eq!("{ b_value: number }", to_string_type_id(b_exports));
  }
}

mod frontend_check_module_references_allocator {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1539:frontend_check_module_references_allocator`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item frontend_check_module_references_allocator

  #[cfg(test)]
  #[test]
  fn frontend_check_module_references_allocator() {
    use alloc::{string::String, sync::Arc};

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyScript"),
      String::from(
        r#"
        print("Hello World")
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/workspace/MyScript"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/workspace/MyScript"));
    let source = fixture
      .get_frontend()
      .get_source_module(&String::from("game/workspace/MyScript"));
    assert!(!source.is_null());

    let source = unsafe { &*source };
    assert_eq!(
      Arc::as_ptr(
        module
          .allocator
          .as_ref()
          .expect("expected module allocator")
      ),
      Arc::as_ptr(&source.allocator)
    );
    assert_eq!(
      Arc::as_ptr(module.names.as_ref().expect("expected module names")),
      Arc::as_ptr(&source.names)
    );
  }
}

mod frontend_check_module_references_correct_ast_root {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1556:frontend_check_module_references_correct_ast_root`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
  //!   - translates_to -> rust_item frontend_check_module_references_correct_ast_root

  #[cfg(test)]
  #[test]
  fn frontend_check_module_references_correct_ast_root() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyScript"),
      String::from(
        r#"
        print("Hello World")
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/workspace/MyScript"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/workspace/MyScript"));
    let source = fixture
      .get_frontend()
      .get_source_module(&String::from("game/workspace/MyScript"));
    assert!(!source.is_null());

    let source = unsafe { &*source };
    assert_eq!(module.root, source.root);
  }
}

mod frontend_check_without_builtin_next {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1187:frontend_check_without_builtin_next`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record TestFileResolver (tests/Fixture.h)
  //!   - type_ref -> record TestConfigResolver (tests/Fixture.h)
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - translates_to -> rust_item frontend_check_without_builtin_next

  #[cfg(test)]
  #[test]
  fn frontend_check_without_builtin_next() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      records::{frontend::Frontend, frontend_options::FrontendOptions},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver,
    };

    let mut file_resolver = TestFileResolver::default();
    let mut config_resolver = TestConfigResolver::default();
    let mode = if FFlag::DebugLuauForceOldSolver.get() {
      SolverMode::Old
    } else {
      SolverMode::New
    };
    let mut frontend =
      Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
        mode,
        &mut file_resolver.base,
        &mut config_resolver.base,
        FrontendOptions::default(),
      );
    unsafe {
      frontend.wire_self_pointers();
    }

    file_resolver.source.insert(
      String::from("Module/A"),
      String::from("for k,v in 2 do end"),
    );
    file_resolver
      .source
      .insert(String::from("Module/B"), String::from("return next"));

    // We don't care about the result. That we haven't crashed is enough.
    frontend.check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    frontend.check_module_name_optional_frontend_options(&String::from("Module/B"), None);
  }
}

mod frontend_checked_modules_have_the_correct_mode {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1353:frontend_checked_modules_have_the_correct_mode`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - translates_to -> rust_item frontend_checked_modules_have_the_correct_mode

  #[cfg(test)]
  #[test]
  fn frontend_checked_modules_have_the_correct_mode() {
    use alloc::string::String;

    use ulua_ast::enums::mode::Mode;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!nocheck
        local a: number = "five"
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        --!nonstrict
        local a = math.abs("five")
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/C"),
      String::from(
        r#"
        --!strict
        local a = 10
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/C"), None);

    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert_eq!(Mode::NoCheck, module_a.mode);

    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    assert_eq!(Mode::Nonstrict, module_b.mode);

    let module_c = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/C"));
    assert_eq!(Mode::Strict, module_c.mode);
  }
}

mod frontend_clear_stats {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1078:frontend_clear_stats`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Stats (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - calls -> method Frontend::clearStats (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_clear_stats

  #[cfg(test)]
  #[test]
  fn frontend_clear_stats() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        --!strict
        local B = require(script.Parent.B)
        local foo = B.foo + 1
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        --!strict
        return {foo = 1}
    "#,
      ),
    );

    let r1 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, r1.errors.len(), "{:?}", r1.errors);

    let stats1 = fixture.get_frontend().stats;
    assert_eq!(2, stats1.files);

    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/A"), None);
    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/B"), None);

    fixture.get_frontend().clear_stats();
    let r2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, r2.errors.len(), "{:?}", r2.errors);
    let stats2 = fixture.get_frontend().stats;

    assert_eq!(2, stats2.files);
  }
}

mod frontend_cycle_detection_between_check_and_nocheck {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:300:frontend_cycle_detection_between_check_and_nocheck`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_cycle_detection_between_check_and_nocheck

  #[cfg(test)]
  #[test]
  fn frontend_cycle_detection_between_check_and_nocheck() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod frontend_cycle_detection_disabled_in_nocheck {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:352:frontend_cycle_detection_disabled_in_nocheck`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_cycle_detection_disabled_in_nocheck

  #[cfg(test)]
  #[test]
  fn frontend_cycle_detection_disabled_in_nocheck() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod frontend_cycle_error_paths {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:400:frontend_cycle_error_paths`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ModuleHasCyclicDependency (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item frontend_cycle_error_paths

  #[cfg(test)]
  #[test]
  fn frontend_cycle_error_paths() {
    use alloc::string::String;

    use ulua_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let ce1 = type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[0])
      .expect("expected first cycle error");
    assert_eq!("game/Gui/Modules/B", result.errors[0].module_name);
    assert_eq!(2, ce1.cycle().len());
    assert_eq!("game/Gui/Modules/A", ce1.cycle()[0]);
    assert_eq!("game/Gui/Modules/B", ce1.cycle()[1]);

    let ce2 = type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[1])
      .expect("expected second cycle error");
    assert_eq!("game/Gui/Modules/A", result.errors[1].module_name);
    assert_eq!(2, ce2.cycle().len());
    assert_eq!("game/Gui/Modules/B", ce2.cycle()[0]);
    assert_eq!("game/Gui/Modules/A", ce2.cycle()[1]);
  }
}

mod frontend_cycle_errors_can_be_fixed {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:371:frontend_cycle_errors_can_be_fixed`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ModuleHasCyclicDependency (Analysis/include/Luau/Error.h)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_cycle_errors_can_be_fixed

  #[cfg(test)]
  #[test]
  fn frontend_cycle_errors_can_be_fixed() {
    use alloc::string::String;

    use ulua_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
      ),
    );

    let result1 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(2, result1.errors.len(), "{:?}", result1.errors);

    assert!(
      type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[0]).is_some(),
      "Should have been a ModuleHasCyclicDependency: {:?}",
      result1.errors[0]
    );
    assert!(
      type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[1]).is_some(),
      "Should have been a ModuleHasCyclicDependency: {:?}",
      result1.errors[1]
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        return {hello = 42}
    "#,
      ),
    );
    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/Gui/Modules/B"), None);

    let result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/A"), None);
    assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
  }
}

mod frontend_cycle_incremental_type_surface {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:431:frontend_cycle_incremental_type_surface`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_cycle_incremental_type_surface

  #[cfg(test)]
  #[test]
  fn frontend_cycle_incremental_type_surface() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        return {hello = 2}
    "#,
      ),
    );

    let mut result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        local me = require(game.A)
        return {hello = 2}
    "#,
      ),
    );
    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/A"), None);

    result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!result.errors.is_empty(), "expected errors");

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    let ty = fixture
      .base
      .base
      .require_type_module_ptr_string(&module, &String::from("me"));
    assert_eq!("any", to_string_type_id(ty));
  }
}

mod frontend_cycle_incremental_type_surface_exports {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:491:frontend_cycle_incremental_type_surface_exports`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_cycle_incremental_type_surface_exports

  #[cfg(test)]
  #[test]
  fn frontend_cycle_incremental_type_surface_exports() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
local b = require(game.B)
export type atype = { x: b.btype }
return {mod_a = 1}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
export type btype = { x: number }

local function bf()
    local a = require(game.A)
    local bfl : a.atype = nil
    return {bfl.x}
end
return {mod_b = 2}
    "#,
      ),
    );

    let mut result_a = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!result_a.errors.is_empty(), "expected errors");

    let mut result_b = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result_b.errors.is_empty(), "expected errors");

    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let ty_b = module_b
      .exported_type_bindings
      .get(&String::from("btype"))
      .expect("expected exported btype")
      .r#type();
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: number }",
      to_string_type_id_to_string_options(ty_b, &mut opts)
    );

    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    let ty_a = module_a
      .exported_type_bindings
      .get(&String::from("atype"))
      .expect("expected exported atype")
      .r#type();
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: any }",
      to_string_type_id_to_string_options(ty_a, &mut opts)
    );

    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/B"), None);
    result_b = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result_b.errors.is_empty(), "expected errors");

    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let ty_b = module_b
      .exported_type_bindings
      .get(&String::from("btype"))
      .expect("expected exported btype")
      .r#type();
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: number }",
      to_string_type_id_to_string_options(ty_b, &mut opts)
    );

    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    let ty_a = module_a
      .exported_type_bindings
      .get(&String::from("atype"))
      .expect("expected exported atype")
      .r#type();
    let mut opts = ToStringOptions {
      exhaustive: true,
      ..Default::default()
    };
    assert_eq!(
      "{ x: any }",
      to_string_type_id_to_string_options(ty_a, &mut opts)
    );

    let _ = &mut result_a;
  }
}

mod frontend_cycle_incremental_type_surface_longer {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:453:frontend_cycle_incremental_type_surface_longer`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - translates_to -> rust_item frontend_cycle_incremental_type_surface_longer

  #[cfg(test)]
  #[test]
  fn frontend_cycle_incremental_type_surface_longer() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        return {mod_a = 2}
    "#,
      ),
    );

    let mut result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.base.file_resolver.source.insert(
      String::from("game/B"),
      String::from(
        r#"
        local me = require(game.A)
        return {mod_b = 4}
    "#,
      ),
    );

    result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        local me = require(game.B)
        return {mod_a_prime = 3}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/A"), None);
    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/B"), None);

    result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert!(!result.errors.is_empty(), "expected errors");

    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    let ty_a = fixture
      .base
      .base
      .require_type_module_ptr_string(&module_a, &String::from("me"));
    assert_eq!("any", to_string_type_id(ty_a));

    result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result.errors.is_empty(), "expected errors");

    let module_b = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/B"));
    let ty_b = fixture
      .base
      .base
      .require_type_module_ptr_string(&module_b, &String::from("me"));
    assert_eq!("any", to_string_type_id(ty_b));
  }
}

mod frontend_dfg_data_cleared_on_retain_type_graphs_unset {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1572:frontend_dfg_data_cleared_on_retain_type_graphs_unset`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_dfg_data_cleared_on_retain_type_graphs_unset

  #[cfg(test)]
  #[test]
  fn frontend_dfg_data_cleared_on_retain_type_graphs_unset() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
local a = 1
local b = 2
local c = 3
return {x = a, y = b, z = c}
"#,
      ),
    );

    fixture.get_frontend().options.retain_full_type_graphs = true;
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert!(!module.def_arena.allocator.empty());
    assert!(!module.key_arena.empty());

    fixture.get_frontend().options.retain_full_type_graphs = false;
    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/A"), None);
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert!(module.def_arena.allocator.empty());
    assert!(module.key_arena.empty());
  }
}

mod frontend_discard_type_graphs {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:903:frontend_discard_type_graphs`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item frontend_discard_type_graphs

  #[cfg(test)]
  #[test]
  fn frontend_discard_type_graphs() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      records::{frontend::Frontend, frontend_options::FrontendOptions},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver,
    };

    let mut file_resolver = TestFileResolver::default();
    let mut config_resolver = TestConfigResolver::default();
    let mode = if FFlag::DebugLuauForceOldSolver.get() {
      SolverMode::Old
    } else {
      SolverMode::New
    };
    let mut fe = Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
      mode,
      &mut file_resolver.base,
      &mut config_resolver.base,
      FrontendOptions::default(),
    );
    unsafe {
      fe.wire_self_pointers();
    }

    file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        local a = {1,2,3,4,5}
    "#,
      ),
    );

    let _result = fe.check_module_name_optional_frontend_options(&String::from("Module/A"), None);

    let module = fe.module_resolver.get_module(&String::from("Module/A"));

    assert_eq!(0, module.internal_types.types.size());
    assert_eq!(0, module.internal_types.type_packs.size());
    assert_eq!(0, module.ast_types.size());
    assert_eq!(0, module.ast_resolved_types.size());
    assert_eq!(0, module.ast_resolved_type_packs.size());
  }
}

mod frontend_dont_recheck_script_that_hasnt_been_marked_dirty {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:562:frontend_dont_recheck_script_that_hasnt_been_marked_dirty`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item frontend_dont_recheck_script_that_hasnt_been_marked_dirty

  #[cfg(test)]
  #[test]
  fn frontend_dont_recheck_script_that_hasnt_been_marked_dirty() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from(
            "Massively incorrect syntax haha oops!  However!  The getFrontend().doesn't know that this file needs reparsing!",
        ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Gui/Modules/B"));
    assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);
  }
}

mod frontend_dont_reparse_clean_file_when_linting {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:536:frontend_dont_reparse_clean_file_when_linting`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method LintOptions::enableWarning (Config/include/Luau/LinterConfig.h)
  //!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
  //!   - calls -> method Fixture::lint (tests/Fixture.cpp)
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - translates_to -> rust_item frontend_dont_reparse_clean_file_when_linting

  #[cfg(test)]
  #[test]
  fn frontend_dont_reparse_clean_file_when_linting() {
    use alloc::string::String;

    use ulua_analysis::records::frontend_options::FrontendOptions;
    use ulua_config::records::lint_warning::LintWarning;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        local t = {}

        for i=#t,1 do
        end

        for i=#t,1,-1 do
        end
    "#,
      ),
    );

    fixture.get_frontend();
    fixture
      .base
      .base
      .config_resolver
      .default_config
      .enabled_lint
      .enable_warning(LintWarning::CODE_FOR_RANGE);

    let opts = FrontendOptions {
      run_lint_checks: true,
      ..Default::default()
    };
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), Some(opts.clone()));

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        -- We have fixed the lint error, but we did not tell the Frontend that the file is changed!
        -- Therefore, we expect Frontend to reuse the results from previous lint.
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), Some(opts));
    assert_eq!(1, result.lint_result.warnings.len());
  }
}

mod frontend_environments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:984:frontend_environments`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::addEnvironment (Analysis/src/Frontend.cpp)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item frontend_environments

  #[cfg(test)]
  #[test]
  fn frontend_environments() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{freeze::freeze, unfreeze::unfreeze},
      records::frontend::Frontend,
    };
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    let test_scope = fixture.get_frontend().add_environment(String::from("test"));

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    unsafe {
      unfreeze((*frontend_ptr).globals.global_types_mut());
      let result = (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        test_scope,
        r#"
        export type Foo = number | string
    "#,
        String::from("@test"),
        false,
        false,
      );
      assert!(result.success, "{:?}", result);
      freeze((*frontend_ptr).globals.global_types_mut());
    }

    fixture.base.base.file_resolver.source.insert(
      String::from("A"),
      String::from(
        r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("B"),
      String::from(
        r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("C"),
      String::from(
        r#"
        --!strict
        local foo: Foo = 1
    "#,
      ),
    );

    fixture
      .base
      .base
      .file_resolver
      .environments
      .insert(String::from("A"), String::from("test"));

    let result_a = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("A"), None);
    assert_eq!(0, result_a.errors.len(), "{:?}", result_a.errors);

    let result_b = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("B"), None);
    assert_eq!(1, result_b.errors.len(), "{:?}", result_b.errors);

    let result_c = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("C"), None);
    assert_eq!(1, result_c.errors.len(), "{:?}", result_c.errors);
  }
}

mod frontend_export_value_modules_have_typed_require_surface {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:207:frontend_export_value_modules_have_typed_require_surface`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method AssemblyBuilderX64::inc (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item frontend_export_value_modules_have_typed_require_surface

  #[cfg(test)]
  #[test]
  fn frontend_export_value_modules_have_typed_require_surface() {
    use alloc::string::String;

    use ulua_analysis::functions::{first::first, to_string_to_string_alt_c::to_string_type_id};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/ModuleA"),
      String::from(
        r#"
        --!strict
        export local version = "1.0.0"
        export const answer = 42

        export function inc(x: number): number
            return x + 1
        end
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/ModuleB"),
      String::from(
        r#"
        --!strict
        local M = require(game.ModuleA)

        local version: string = M.version
        local answer: number = M.answer
        local nextValue: number = M.inc(answer)

        return version, nextValue
    "#,
      ),
    );

    let a_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/ModuleA"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/ModuleB"), None);
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let module_a = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/ModuleA"));

    let exports = first(module_a.return_type, true).expect("expected module return type");
    assert_eq!(
      "{ read answer: number, read inc: (number) -> number, read version: string }",
      to_string_type_id(exports)
    );
  }
}

mod frontend_find_a_require {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:94:frontend_find_a_require`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item frontend_find_a_require

  #[cfg(test)]
  #[test]
  fn frontend_find_a_require() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
      naive_file_resolver::NaiveFileResolver,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };
    let program = fixture.base.base.parse(
      r#"
        local M = require(Modules.Foo.Bar)
    "#,
      &ParseOptions::default(),
    );

    let mut naive_file_resolver = NaiveFileResolver::default();

    let result = unsafe {
      trace_requires(
        &mut naive_file_resolver.base.base,
        program,
        String::new(),
        &TypeCheckLimits::default(),
      )
    };
    assert_eq!(1, result.require_list.len());
    assert_eq!("Modules/Foo/Bar", result.require_list[0].0);
  }
}

mod frontend_find_a_require_inside_a_function {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:108:frontend_find_a_require_inside_a_function`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - translates_to -> rust_item frontend_find_a_require_inside_a_function

  #[cfg(test)]
  #[test]
  fn frontend_find_a_require_inside_a_function() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
      naive_file_resolver::NaiveFileResolver,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };
    let program = fixture.base.base.parse(
      r#"
        function foo()
            local M = require(Modules.Foo.Bar)
        end
    "#,
      &ParseOptions::default(),
    );

    let mut naive_file_resolver = NaiveFileResolver::default();

    let result = unsafe {
      trace_requires(
        &mut naive_file_resolver.base.base,
        program,
        String::new(),
        &TypeCheckLimits::default(),
      )
    };
    assert_eq!(1, result.require_list.len());
  }
}

mod frontend_get_required_scripts {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1471:frontend_get_required_scripts`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method Frontend::getRequiredScripts (Analysis/src/Frontend.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item frontend_get_required_scripts

  #[cfg(test)]
  #[test]
  fn frontend_get_required_scripts() {
    use alloc::string::String;

    use ulua_analysis::records::type_check_limits::TypeCheckLimits;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyScript"),
      String::from(
        r#"
        local MyModuleScript = require(game.workspace.MyModuleScript)
        local MyModuleScript2 = require(game.workspace.MyModuleScript2)
        MyModuleScript.myPrint()
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyModuleScript"),
      String::from(
        r#"
        local module = {}
        function module.myPrint()
            print("Hello World")
        end
        return module
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyModuleScript2"),
      String::from(
        r#"
        local module = {}
        return module
    "#,
      ),
    );

    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/workspace/MyScript"), None);
    let mut required_scripts = fixture.get_frontend().get_required_scripts(
      &String::from("game/workspace/MyScript"),
      &TypeCheckLimits::default(),
    );
    assert_eq!(2, required_scripts.len(), "{:?}", required_scripts);
    assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
    assert_eq!("game/workspace/MyModuleScript2", required_scripts[1]);

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/workspace/MyScript"), None);
    required_scripts = fixture.get_frontend().get_required_scripts(
      &String::from("game/workspace/MyScript"),
      &TypeCheckLimits::default(),
    );
    assert_eq!(2, required_scripts.len(), "{:?}", required_scripts);
    assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
    assert_eq!("game/workspace/MyModuleScript2", required_scripts[1]);
  }
}

mod frontend_get_required_scripts_dirty {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1507:frontend_get_required_scripts_dirty`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::getRequiredScripts (Analysis/src/Frontend.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_get_required_scripts_dirty

  #[cfg(test)]
  #[test]
  fn frontend_get_required_scripts_dirty() {
    use alloc::string::String;

    use ulua_analysis::records::type_check_limits::TypeCheckLimits;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyScript"),
      String::from(
        r#"
        print("Hello World")
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyModuleScript"),
      String::from(
        r#"
        local module = {}
        function module.myPrint()
            print("Hello World")
        end
        return module
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/workspace/MyScript"), None);
    let mut required_scripts = fixture.get_frontend().get_required_scripts(
      &String::from("game/workspace/MyScript"),
      &TypeCheckLimits::default(),
    );
    assert_eq!(0, required_scripts.len(), "{:?}", required_scripts);

    fixture.base.base.file_resolver.source.insert(
      String::from("game/workspace/MyScript"),
      String::from(
        r#"
        local MyModuleScript = require(game.workspace.MyModuleScript)
        MyModuleScript.myPrint()
    "#,
      ),
    );

    required_scripts = fixture.get_frontend().get_required_scripts(
      &String::from("game/workspace/MyScript"),
      &TypeCheckLimits::default(),
    );
    assert_eq!(0, required_scripts.len(), "{:?}", required_scripts);

    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/workspace/MyScript"), None);
    required_scripts = fixture.get_frontend().get_required_scripts(
      &String::from("game/workspace/MyScript"),
      &TypeCheckLimits::default(),
    );
    assert_eq!(1, required_scripts.len(), "{:?}", required_scripts);
    assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
  }
}

mod frontend_ignore_require_to_nonexistent_file {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:711:frontend_ignore_require_to_nonexistent_file`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_ignore_require_to_nonexistent_file

  #[cfg(test)]
  #[test]
  fn frontend_ignore_require_to_nonexistent_file() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        local Modules = script
        local B = require(Modules.B) :: any
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod frontend_imported_table_modification_2 {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1123:frontend_imported_table_modification_2`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item frontend_imported_table_modification_2

  #[cfg(test)]
  #[test]
  fn frontend_imported_table_modification_2() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.get_frontend().options.retain_full_type_graphs = false;

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
--!nonstrict
local a = {}
a.x = 1
return a;
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
--!nonstrict
local a = require(script.Parent.A)
local b = {}
function a:b() end -- this should error, since A doesn't define a:b()
return b
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/C"),
      String::from(
        r#"
--!nonstrict
local a = require(script.Parent.A)
local b = require(script.Parent.B)
a:b() -- this should error, since A doesn't define a:b()
    "#,
      ),
    );

    let result_a = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, result_a.errors.len(), "{:?}", result_a.errors);

    let result_b = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), None);
    assert!(!result_b.errors.is_empty(), "expected errors");

    let result_c = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/C"), None);
    assert!(!result_c.errors.is_empty(), "expected errors");
  }
}

mod frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:922:frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - type_ref -> record TypeIds (Analysis/include/Luau/TypeIds.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded

  #[cfg(test)]
  #[test]
  fn frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode,
      functions::to_string_error::to_string_type_error,
      records::{frontend::Frontend, frontend_options::FrontendOptions},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver,
    };

    let mut file_resolver = TestFileResolver::default();
    let mut config_resolver = TestConfigResolver::default();
    let mode = if FFlag::DebugLuauForceOldSolver.get() {
      SolverMode::Old
    } else {
      SolverMode::New
    };
    let mut fe = Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
      mode,
      &mut file_resolver.base,
      &mut config_resolver.base,
      FrontendOptions::default(),
    );
    unsafe {
      fe.wire_self_pointers();
    }

    file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        --!strict
        local a: {Count: number} = {count='five'}
    "#,
      ),
    );

    let result = fe.check_module_name_optional_frontend_options(&String::from("Module/A"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Table type '{ count: string }' not compatible with type '{ Count: number }' because the former is missing field 'Count'",
        to_string_type_error(&result.errors[0])
      );
    } else {
      assert_eq!(
        "Table type 'a' not compatible with type '{ Count: number }' because the former is missing field 'Count'",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod frontend_lint_results_are_only_for_checked_module {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:884:frontend_lint_results_are_only_for_checked_module`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
  //!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item frontend_lint_results_are_only_for_checked_module

  #[cfg(test)]
  #[test]
  fn frontend_lint_results_are_only_for_checked_module() {
    use alloc::string::String;

    use ulua_analysis::records::frontend_options::FrontendOptions;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
local _ = 0b10000000000000000000000000000000000000000000000000000000000000000
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
require(script.Parent.A)
local _ = 0x10000000000000000
    "#,
      ),
    );

    let opts = FrontendOptions {
      run_lint_checks: true,
      ..Default::default()
    };
    let mut result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), Some(opts.clone()));
    assert_eq!(1, result.lint_result.warnings.len());

    result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), Some(opts));
    assert_eq!(1, result.lint_result.warnings.len());
  }
}

mod frontend_mark_non_immediate_reverse_deps_as_dirty {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:609:frontend_mark_non_immediate_reverse_deps_as_dirty`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_mark_non_immediate_reverse_deps_as_dirty

  #[cfg(test)]
  #[test]
  fn frontend_mark_non_immediate_reverse_deps_as_dirty() {
    use alloc::{string::String, vec::Vec};

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/C"), None);

    let mut marked_dirty: Vec<String> = Vec::new();
    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/Gui/Modules/A"), Some(&mut marked_dirty));

    assert_eq!(3, marked_dirty.len(), "{:?}", marked_dirty);
    assert!(marked_dirty.contains(&String::from("game/Gui/Modules/A")));
    assert!(marked_dirty.contains(&String::from("game/Gui/Modules/B")));
    assert!(marked_dirty.contains(&String::from("game/Gui/Modules/C")));
  }
}

mod frontend_markdirty_early_return {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1306:frontend_markdirty_early_return`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_markdirty_early_return

  #[cfg(test)]
  #[test]
  fn frontend_markdirty_early_return() {
    use alloc::{string::String, vec::Vec};

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let module_name = String::from("game/Gui/Modules/A");
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      module_name.clone(),
      String::from(
        r#"
        return 1
    "#,
      ),
    );

    {
      let mut marked_dirty: Vec<String> = Vec::new();
      fixture
        .get_frontend()
        .mark_dirty(&module_name, Some(&mut marked_dirty));
      assert!(marked_dirty.is_empty(), "{:?}", marked_dirty);
    }

    fixture.get_frontend().parse_module_name(&module_name);

    {
      let mut marked_dirty: Vec<String> = Vec::new();
      fixture
        .get_frontend()
        .mark_dirty(&module_name, Some(&mut marked_dirty));
      assert!(!marked_dirty.is_empty());
    }
  }
}

mod frontend_no_separate_caches_with_the_new_solver {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1420:frontend_no_separate_caches_with_the_new_solver`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - translates_to -> rust_item frontend_no_separate_caches_with_the_new_solver

  #[cfg(test)]
  #[test]
  fn frontend_no_separate_caches_with_the_new_solver() {
    use alloc::string::String;

    use ulua_analysis::records::frontend_options::FrontendOptions;
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!nonstrict
        local exports = {}
        function exports.hello() end
        return exports
    "#,
      ),
    );

    let opts = FrontendOptions {
      for_autocomplete: true,
      ..Default::default()
    };
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), Some(opts));

    assert!(
      !fixture
        .get_frontend()
        .module_resolver_for_autocomplete
        .modules
        .contains_key(&String::from("game/A"))
    );

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert_eq!(Mode::Nonstrict, module.mode);
  }
}

mod frontend_no_use_after_free_with_type_fun_instantiation {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1165:frontend_no_use_after_free_with_type_fun_instantiation`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record FrontendFixture (tests/Frontend.test.cpp)
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_no_use_after_free_with_type_fun_instantiation

  #[cfg(test)]
  #[test]
  fn frontend_no_use_after_free_with_type_fun_instantiation() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _freeze_arena = ScopedFastFlag::new(&FFlag::DebugLuauFreezeArena, true);

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
export type Foo<V> = typeof(setmetatable({}, {}))
return false;
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
local A = require(script.Parent.A)
export type Foo<V> = A.Foo<V>
return false;
    "#,
      ),
    );

    // We don't care about the result. That we haven't crashed is enough.
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), None);
  }
}

mod frontend_nocheck_cycle_used_by_checked {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:318:frontend_nocheck_cycle_used_by_checked`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item frontend_nocheck_cycle_used_by_checked

  #[cfg(test)]
  #[test]
  fn frontend_nocheck_cycle_used_by_checked() {
    use alloc::string::String;

    use ulua_analysis::functions::{first::first, to_string_to_string_alt_c::to_string_type_id};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        local B = require(Modules.B)
        return {a=A, b=B}
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/C"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let c_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Gui/Modules/C"));
    let c_exports = first(c_module.return_type, true).expect("expected C module return type");

    assert_eq!(
      "{ a: { hello: any }, b: { hello: any } }",
      to_string_type_id(c_exports)
    );
  }
}

mod frontend_nocheck_modules_are_typed {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:263:frontend_nocheck_modules_are_typed`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - translates_to -> rust_item frontend_nocheck_modules_are_typed

  #[cfg(test)]
  #[test]
  fn frontend_nocheck_modules_are_typed() {
    use alloc::string::String;

    use ulua_analysis::functions::{first::first, to_string_to_string_alt_c::to_string_type_id};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        --!nocheck
        export type Foo = number
        return {hello = "hi"}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!nonstrict
        export type Foo = number
        return {hello = "hi"}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        local B = require(Modules.B)
        local five : A.Foo = 5
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/C"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let a_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Gui/Modules/A"));
    let a_exports = first(a_module.return_type, true).expect("expected A module return type");

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Gui/Modules/B"));
    let b_exports = first(b_module.return_type, true).expect("expected B module return type");

    assert_eq!(to_string_type_id(a_exports), to_string_type_id(b_exports));
  }
}

mod frontend_parse_just_a_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1888:frontend_parse_just_a_type`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
  //!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
  //!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
  //!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
  //!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method FrontendFixture::parseType (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_parse_just_a_type

  #[cfg(test)]
  #[test]
  fn frontend_parse_just_a_type() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{
        builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
        type_arena::TypeArena, type_check_limits::TypeCheckLimits,
      },
    };
    use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let src = "(number, string) -> boolean?";

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    let mut arena = TypeArena::default();
    let mut allocator = Allocator::new();
    let mut names = AstNameTable::new(&mut allocator);
    let _builtin_types = BuiltinTypes::new();
    let mut ice_handler = InternalErrorReporter::default();
    let limits = TypeCheckLimits::default();

    let ty = fixture.get_frontend().parse_type(
      &mut allocator,
      &mut names,
      &mut ice_handler,
      limits,
      &mut arena,
      src,
    );

    assert_eq!("(number, string) -> boolean?", to_string_type_id(ty));
  }
}

mod frontend_parse_only {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1275:frontend_parse_only`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item frontend_parse_only

  #[cfg(test)]
  #[test]
  fn frontend_parse_only() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local a: number = 'oh no a type error'
        return {a=a}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 2
    "#,
      ),
    );

    fixture
      .get_frontend()
      .parse_module_name(&String::from("game/Gui/Modules/B"));

    assert!(
      fixture
        .get_frontend()
        .source_nodes
        .contains_key(&String::from("game/Gui/Modules/A"))
    );
    assert!(
      fixture
        .get_frontend()
        .source_nodes
        .contains_key(&String::from("game/Gui/Modules/B"))
    );

    let node = fixture
      .get_frontend()
      .source_nodes
      .get(&String::from("game/Gui/Modules/B"))
      .cloned()
      .expect("expected source node");
    assert!(
      node
        .require_set
        .contains(&String::from("game/Gui/Modules/A"))
    );
    assert_eq!(1, node.require_locations.len());
    assert_eq!(
      Location::new(
        Position {
          line: 2,
          column: 18,
        },
        Position {
          line: 2,
          column: 36,
        },
      ),
      node.require_locations[0].1
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!("game/Gui/Modules/A", result.errors[0].module_name);
    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod frontend_parse_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1905:frontend_parse_types`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> method FrontendFixture::parseType (tests/Frontend.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record InternalCompilerError (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> type_alias ErrorType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item frontend_parse_types

  #[cfg(test)]
  #[test]
  fn frontend_parse_types() {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::{
      functions::{get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id},
      records::{
        internal_error_reporter::InternalErrorReporter, type_arena::TypeArena,
        type_check_limits::TypeCheckLimits,
      },
      type_aliases::error_type::ErrorType,
    };
    use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };
    let mut allocator = Allocator::new();
    let mut names = AstNameTable::new(&mut allocator);
    let mut arena = TypeArena::default();

    let mut parse_type = |fixture: &mut FrontendFixture, src: &str| {
      let mut ice_handler = InternalErrorReporter::default();
      fixture.get_frontend().parse_type(
        &mut allocator,
        &mut names,
        &mut ice_handler,
        TypeCheckLimits::default(),
        &mut arena,
        src,
      )
    };

    let ty1 = parse_type(&mut fixture, "(number, boolean?) -> string");
    assert_eq!("(number, boolean?) -> string", to_string_type_id(ty1));

    assert!(
      catch_unwind(AssertUnwindSafe(|| parse_type(
        &mut fixture,
        "illegal Luau Syntax here"
      )))
      .is_err()
    );

    let ty3 = parse_type(&mut fixture, "blah<blahblah, number>");
    assert!(get_type_id::<ErrorType>(ty3).is_some());

    assert!(
      catch_unwind(AssertUnwindSafe(|| parse_type(
        &mut fixture,
        "number, boolean?) -> string"
      )))
      .is_err()
    );
    assert!(
      catch_unwind(AssertUnwindSafe(|| parse_type(
        &mut fixture,
        "{size: number?"
      )))
      .is_err()
    );
  }
}

mod frontend_produce_errors_for_unchanged_file_with_a_syntax_error {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:653:frontend_produce_errors_for_unchanged_file_with_a_syntax_error`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_produce_errors_for_unchanged_file_with_a_syntax_error

  #[cfg(test)]
  #[test]
  fn frontend_produce_errors_for_unchanged_file_with_a_syntax_error() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from("oh no a blatant syntax error!!"),
    );

    let one = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);
    let two = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    assert!(
      !one.errors.is_empty(),
      "expected first check to report errors"
    );
    assert!(
      !two.errors.is_empty(),
      "expected second check to report errors"
    );
  }
}

mod frontend_produce_errors_for_unchanged_file_with_errors {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:664:frontend_produce_errors_for_unchanged_file_with_errors`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item frontend_produce_errors_for_unchanged_file_with_errors

  #[cfg(test)]
  #[test]
  fn frontend_produce_errors_for_unchanged_file_with_errors() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from("local p: number = 'oh no a type error'"),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    fixture.base.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from(
            "local p = 4 -- We have fixed the problem, but we didn't tell the getFrontend(). so it will not recheck this file!",
        ),
    );
    let second_result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);

    assert_eq!(1, second_result.errors.len(), "{:?}", second_result.errors);
  }
}

mod frontend_queue_check_cycle_delayed {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1842:frontend_queue_check_cycle_delayed`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::checkQueuedModules (Analysis/src/Frontend.cpp)
  //!   - calls -> method Frontend::getCheckResult (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_queue_check_cycle_delayed

  #[cfg(test)]
  #[test]
  fn frontend_queue_check_cycle_delayed() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        --!strict
        return {c_value = 5}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        local B = require(Modules.B)
        return {a_value = B.hello + C.c_value}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        local A = require(Modules.A)
        return {b_value = A.hello + C.c_value}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .queue_module_check_module_name(&String::from("game/Gui/Modules/B"));
    fixture.get_frontend().check_queued_modules(
      None,
      Box::new(|tasks| {
        for task in tasks {
          task();
        }
      }),
      Box::new(|_, _| true),
    );

    let result = fixture
      .get_frontend()
      .get_check_result(&String::from("game/Gui/Modules/B"), true, false)
      .expect("expected queued check result");
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cyclic module dependency: game/Gui/Modules/B -> game/Gui/Modules/A",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Cyclic module dependency: game/Gui/Modules/A -> game/Gui/Modules/B",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod frontend_queue_check_cycle_instant {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1817:frontend_queue_check_cycle_instant`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::checkQueuedModules (Analysis/src/Frontend.cpp)
  //!   - calls -> method Frontend::getCheckResult (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_queue_check_cycle_instant

  #[cfg(test)]
  #[test]
  fn frontend_queue_check_cycle_instant() {
    use alloc::{boxed::Box, string::String};

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {a_value = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .queue_module_check_module_name(&String::from("game/Gui/Modules/B"));
    fixture.get_frontend().check_queued_modules(
      None,
      Box::new(|tasks| {
        for task in tasks {
          task();
        }
      }),
      Box::new(|_, _| true),
    );

    let result = fixture
      .get_frontend()
      .get_check_result(&String::from("game/Gui/Modules/B"), true, false)
      .expect("expected queued check result");
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cyclic module dependency: game/Gui/Modules/B -> game/Gui/Modules/A",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      "Cyclic module dependency: game/Gui/Modules/A -> game/Gui/Modules/B",
      to_string_type_error(&result.errors[1])
    );
  }
}

mod frontend_queue_check_propagates_ice {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1873:frontend_queue_check_propagates_ice`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> function fromString (tests/Fixture.cpp)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - calls -> method Frontend::checkQueuedModules (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record InternalCompilerError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item frontend_queue_check_propagates_ice

  #[cfg(test)]
  #[test]
  fn frontend_queue_check_propagates_ice() {
    use alloc::{boxed::Box, string::String};
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    let module_name = String::from("MainModule");
    fixture.base.base.file_resolver.source.insert(
      module_name.clone(),
      String::from(
        r#"
        --!strict
        local a: _luau_ice = 55
    "#,
      ),
    );
    fixture.get_frontend().mark_dirty(&module_name, None);
    fixture
      .get_frontend()
      .queue_module_check_module_name(&String::from("MainModule"));

    let result = catch_unwind(AssertUnwindSafe(|| {
      fixture.get_frontend().check_queued_modules(
        None,
        Box::new(|tasks| {
          for task in tasks {
            task();
          }
        }),
        Box::new(|_, _| true),
      );
    }));

    let panic = result.expect_err("expected InternalCompilerError");
    assert!(
      panic.downcast_ref::<InternalCompilerError>().is_some(),
      "expected InternalCompilerError panic payload"
    );
  }
}

mod frontend_queue_check_simple {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1796:frontend_queue_check_simple`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::checkQueuedModules (Analysis/src/Frontend.cpp)
  //!   - calls -> method Frontend::getCheckResult (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_queue_check_simple

  #[cfg(test)]
  #[test]
  fn frontend_queue_check_simple() {
    use alloc::{boxed::Box, string::String};

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        --!strict
        return {hello=5, world=true}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .queue_module_check_module_name(&String::from("game/Gui/Modules/B"));
    fixture.get_frontend().check_queued_modules(
      None,
      Box::new(|tasks| {
        for task in tasks {
          task();
        }
      }),
      Box::new(|_, _| true),
    );

    let result = fixture
      .get_frontend()
      .get_check_result(&String::from("game/Gui/Modules/B"), true, false)
      .expect("expected queued check result");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod frontend_re_report_type_error_in_required_file {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:750:frontend_re_report_type_error_in_required_file`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_re_report_type_error_in_required_file

  #[cfg(test)]
  #[test]
  fn frontend_re_report_type_error_in_required_file() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        local n: number = 'five'
        return {n=n}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/B"),
      String::from(
        r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        print(A.n)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);
    assert_eq!(1, result2.errors.len(), "{:?}", result2.errors);

    assert_eq!("Modules/A", result.errors[0].module_name);
  }
}

mod frontend_real_source {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:122:frontend_real_source`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - translates_to -> rust_item frontend_real_source

  #[cfg(test)]
  #[test]
  fn frontend_real_source() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
      naive_file_resolver::NaiveFileResolver,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };
    let program = fixture.base.base.parse(
        r#"
        return function()
            local Modules = game:GetService("CoreGui").Gui.Modules

            local Roact = require(Modules.Common.Roact)
            local Rodux = require(Modules.Common.Rodux)

            local AppReducer = require(Modules.LuaApp.AppReducer)
            local AEAppReducer = require(Modules.LuaApp.Reducers.AEReducers.AEAppReducer)
            local AETabList = require(Modules.LuaApp.Components.Avatar.UI.Views.Portrait.AETabList)
            local mockServices = require(Modules.LuaApp.TestHelpers.mockServices)
            local DeviceOrientationMode = require(Modules.LuaApp.DeviceOrientationMode)
            local MockAvatarEditorTheme = require(Modules.LuaApp.TestHelpers.MockAvatarEditorTheming)
            local FFlagAvatarEditorEnableThemes = settings():GetFFlag("AvatarEditorEnableThemes2")
        end
    "#,
        &ParseOptions::default(),
    );

    let mut naive_file_resolver = NaiveFileResolver::default();

    let result = unsafe {
      trace_requires(
        &mut naive_file_resolver.base.base,
        program,
        String::new(),
        &TypeCheckLimits::default(),
      )
    };
    assert_eq!(8, result.require_list.len());
  }
}

mod frontend_recheck_if_dependent_script_is_dirty {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:583:frontend_recheck_if_dependent_script_is_dirty`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item frontend_recheck_if_dependent_script_is_dirty

  #[cfg(test)]
  #[test]
  fn frontend_recheck_if_dependent_script_is_dirty() {
    use alloc::string::String;

    use ulua_analysis::functions::{first::first, to_string_to_string_alt_c::to_string_type_id};
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello='hi!'}"),
    );
    fixture
      .get_frontend()
      .mark_dirty(&String::from("game/Gui/Modules/A"), None);

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);

    let b_module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/Gui/Modules/B"));
    assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);

    let b_exports = first(b_module.return_type, true).expect("expected module return type");

    assert_eq!("{ b_value: string }", to_string_type_id(b_exports));
  }
}

mod frontend_reexport_cyclic_type {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1200:frontend_reexport_cyclic_type`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_reexport_cyclic_type

  #[cfg(test)]
  #[test]
  fn frontend_reexport_cyclic_type() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        type F<T> = (set: G<T>) -> ()

        export type G<T> = {
            forEach: (a: F<T>) -> (),
        }

        function X<T>(a: F<T>): ()
        end

        return X
    "#,
      ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        --!strict
        local A = require(script.Parent.A)

        export type G<T> = A.G<T>

        return {
            A = A,
        }
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod frontend_reexport_type_alias {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1231:frontend_reexport_type_alias`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_reexport_type_alias

  #[cfg(test)]
  #[test]
  fn frontend_reexport_type_alias() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        type KeyOfTestEvents = "test-file-start" | "test-file-success" | "test-file-failure" | "test-case-result"
        type MyAny = any

        export type TestFileEvent<T = KeyOfTestEvents> = (
            eventName: T,
            args: any --[[ ROBLOX TODO: Unhandled node for type: TSIndexedAccessType ]] --[[ TestEvents[T] ]]
        ) -> MyAny

        return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        --!strict
        local A = require(script.Parent.A)

        export type TestFileEvent = A.TestFileEvent
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod frontend_report_require_to_nonexistent_file {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:697:frontend_report_require_to_nonexistent_file`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record UnknownRequire (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item frontend_report_require_to_nonexistent_file

  #[cfg(test)]
  #[test]
  fn frontend_report_require_to_nonexistent_file() {
    use alloc::string::String;

    use ulua_analysis::records::unknown_require::UnknownRequire;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
        r#"
        local Modules = script
        local B = require(Modules.B)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/A"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert!(
      type_error_data_ref::<UnknownRequire>(&result.errors[0]).is_some(),
      "Should have been an UnknownRequire: {:?}",
      result.errors[0]
    );
  }
}

mod frontend_report_syntax_error_in_required_file {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:722:frontend_report_syntax_error_in_required_file`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item frontend_report_syntax_error_in_required_file

  #[cfg(test)]
  #[test]
  fn frontend_report_syntax_error_in_required_file() {
    use alloc::string::String;

    use ulua_analysis::records::syntax_error::SyntaxError;
    use ulua_unit_test::{
      functions::type_error_data_ref::type_error_data_ref,
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from("oh no a gross breach of syntax"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("Modules/B"),
      String::from(
        r#"
        local Modules = script.Parent
        local A = require(Modules.A)
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);
    assert!(!result.errors.is_empty(), "expected errors");

    assert_eq!("Modules/A", result.errors[0].module_name);

    assert!(
      result
        .errors
        .iter()
        .any(|error| type_error_data_ref::<SyntaxError>(error).is_some()),
      "Expected a syntax error: {:?}",
      result.errors
    );
  }
}

mod frontend_reports_errors_from_multiple_sources {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:677:frontend_reports_errors_from_multiple_sources`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_reports_errors_from_multiple_sources

  #[cfg(test)]
  #[test]
  fn frontend_reports_errors_from_multiple_sources() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
        r#"
        local a: number = 'oh no a type error'
        return {a=a}
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 'another one!  This is quite distressing!'
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/B"), None);
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!("game/Gui/Modules/A", result.errors[0].module_name);
    assert_eq!("game/Gui/Modules/B", result.errors[1].module_name);
  }
}

mod frontend_separate_caches_for_autocomplete {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1387:frontend_separate_caches_for_autocomplete`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
  //!   - translates_to -> rust_item frontend_separate_caches_for_autocomplete

  #[cfg(test)]
  #[test]
  fn frontend_separate_caches_for_autocomplete() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode, records::frontend_options::FrontendOptions,
    };
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/A"),
      String::from(
        r#"
        --!nonstrict
        local exports = {}
        function exports.hello() end
        return exports
    "#,
      ),
    );

    let opts = FrontendOptions {
      for_autocomplete: true,
      ..Default::default()
    };
    fixture
      .get_frontend()
      .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), Some(opts));

    assert!(
      !fixture
        .get_frontend()
        .module_resolver
        .modules
        .contains_key(&String::from("game/A"))
    );

    let ac_module = fixture
      .get_frontend()
      .module_resolver_for_autocomplete
      .get_module(&String::from("game/A"));
    assert_eq!(Mode::Strict, ac_module.mode);

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("game/A"));
    assert_eq!(Mode::Nonstrict, module.mode);
  }
}

mod frontend_stats_are_not_reset_between_checks {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1049:frontend_stats_are_not_reset_between_checks`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
  //!   - type_ref -> record Stats (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_stats_are_not_reset_between_checks

  #[cfg(test)]
  #[test]
  fn frontend_stats_are_not_reset_between_checks() {
    use alloc::string::String;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        --!strict
        local B = require(script.Parent.B)
        local foo = B.foo + 1
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        --!strict
        return {foo = 1}
    "#,
      ),
    );

    let r1 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, r1.errors.len(), "{:?}", r1.errors);

    let stats1 = fixture.get_frontend().stats;
    assert_eq!(2, stats1.files);

    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/A"), None);
    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/B"), None);

    let r2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, r2.errors.len(), "{:?}", r2.errors);
    let stats2 = fixture.get_frontend().stats;

    assert_eq!(4, stats2.files);
  }
}

mod frontend_test_dependents_stored_on_node_as_graph_updates {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1658:frontend_test_dependents_stored_on_node_as_graph_updates`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
  //!   - type_ref -> record SourceNode (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item frontend_test_dependents_stored_on_node_as_graph_updates

  #[cfg(test)]
  #[test]
  fn frontend_test_dependents_stored_on_node_as_graph_updates() {
    use alloc::{
      string::{String, ToString},
      vec::Vec,
    };
    use std::collections::BTreeMap;

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    let update_source = |fixture: &mut FrontendFixture, name: &str, source: &str| {
      fixture
        .base
        .base
        .file_resolver
        .source
        .insert(name.to_string(), source.to_string());
      fixture.get_frontend().mark_dirty(&name.to_string(), None);
    };

    let validate_matches_require_lists = |fixture: &mut FrontendFixture, message: &str| {
      let frontend = fixture.get_frontend();
      let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();

      for (module_name, node) in &frontend.source_nodes {
        for dep in node.require_set.iter() {
          dependents
            .entry(dep.clone())
            .or_default()
            .push(module_name.clone());
        }
      }

      for (module_name, node) in &frontend.source_nodes {
        if let Some(expected_dependents) = dependents.get(module_name) {
          for dep in expected_dependents {
            assert!(
              node.dependents.contains(dep),
              "Mismatch in dependents for {module_name}: {message}"
            );
          }
        }
      }
    };

    let validate_second_depends_on_first =
      |fixture: &mut FrontendFixture, from: &str, to: &str, expected: bool| {
        let frontend = fixture.get_frontend();
        let from_node = frontend
          .source_nodes
          .get(from)
          .unwrap_or_else(|| panic!("expected source node {from}"));
        assert_eq!(
          expected,
          from_node.dependents.contains(&to.to_string()),
          "Expected {from} to {}have a reverse dependency on {to}",
          if expected { "" } else { "not " }
        );
      };

    // C -> B -> A
    {
      update_source(
        &mut fixture,
        "game/Gui/Modules/A",
        "return {hello=5, world=true}",
      );
      update_source(
        &mut fixture,
        "game/Gui/Modules/B",
        r#"
            return require(game:GetService('Gui').Modules.A)
        "#,
      );
      update_source(
        &mut fixture,
        "game/Gui/Modules/C",
        r#"
            local Modules = game:GetService('Gui').Modules
            local B = require(Modules.B)
            return {c_value = B}
        "#,
      );
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&"game/Gui/Modules/C".to_string(), None);

      validate_matches_require_lists(&mut fixture, "Initial check");

      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/A",
        "game/Gui/Modules/B",
        true,
      );
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/B",
        "game/Gui/Modules/C",
        true,
      );
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/C",
        "game/Gui/Modules/A",
        false,
      );
    }

    // C -> B, A
    {
      update_source(
        &mut fixture,
        "game/Gui/Modules/B",
        r#"
            return 1
        "#,
      );
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&"game/Gui/Modules/C".to_string(), None);

      validate_matches_require_lists(&mut fixture, "Removing dependency B->A");
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/A",
        "game/Gui/Modules/B",
        false,
      );
    }

    // C -> B -> A
    {
      update_source(
        &mut fixture,
        "game/Gui/Modules/B",
        r#"
            return require(game:GetService('Gui').Modules.A)
        "#,
      );
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&"game/Gui/Modules/C".to_string(), None);

      validate_matches_require_lists(&mut fixture, "Adding back B->A");
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/A",
        "game/Gui/Modules/B",
        true,
      );
    }

    // C -> B -> A, D -> (C,B,A)
    {
      update_source(
        &mut fixture,
        "game/Gui/Modules/D",
        r#"
            local C = require(game:GetService('Gui').Modules.C)
            local B = require(game:GetService('Gui').Modules.B)
            local A = require(game:GetService('Gui').Modules.A)
            return {d_value = C.c_value}
        "#,
      );
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&"game/Gui/Modules/D".to_string(), None);

      validate_matches_require_lists(&mut fixture, "Adding D->C, D->B, D->A");
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/A",
        "game/Gui/Modules/D",
        true,
      );
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/B",
        "game/Gui/Modules/D",
        true,
      );
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/C",
        "game/Gui/Modules/D",
        true,
      );
    }

    // B -> A, C <-> D
    {
      update_source(
        &mut fixture,
        "game/Gui/Modules/D",
        "return require(game:GetService('Gui').Modules.C)",
      );
      update_source(
        &mut fixture,
        "game/Gui/Modules/C",
        "return require(game:GetService('Gui').Modules.D)",
      );
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&"game/Gui/Modules/D".to_string(), None);

      validate_matches_require_lists(&mut fixture, "Adding cycle D->C, C->D");
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/C",
        "game/Gui/Modules/D",
        true,
      );
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/D",
        "game/Gui/Modules/C",
        true,
      );
    }

    // B -> A, C -> D, D -> error
    {
      update_source(
        &mut fixture,
        "game/Gui/Modules/D",
        "return require(game:GetService('Gui').Modules.C.)",
      );
      fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&"game/Gui/Modules/D".to_string(), None);

      validate_matches_require_lists(&mut fixture, "Adding error dependency D->C.");
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/D",
        "game/Gui/Modules/C",
        true,
      );
      validate_second_depends_on_first(
        &mut fixture,
        "game/Gui/Modules/C",
        "game/Gui/Modules/D",
        false,
      );
    }
  }
}

mod frontend_test_invalid_dependency_tracking_per_module_resolver {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1772:frontend_test_invalid_dependency_tracking_per_module_resolver`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
  //!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method Frontend::allModuleDependenciesValid (Analysis/src/Frontend.cpp)
  //!   - translates_to -> rust_item frontend_test_invalid_dependency_tracking_per_module_resolver

  #[cfg(test)]
  #[test]
  fn frontend_test_invalid_dependency_tracking_per_module_resolver() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::solver_mode::SolverMode, records::frontend_options::FrontendOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture
      .get_frontend()
      .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::New
      } else {
        SolverMode::Old
      });

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from("return require(game:GetService('Gui').Modules.A)"),
    );

    let mut opts = FrontendOptions {
      for_autocomplete: false,
      ..Default::default()
    };
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(
        &String::from("game/Gui/Modules/B"),
        Some(opts.clone()),
      );
    assert!(
      fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/B"), opts.for_autocomplete)
    );
    assert!(
      !fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/B"), !opts.for_autocomplete)
    );

    opts.for_autocomplete = true;
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(
        &String::from("game/Gui/Modules/A"),
        Some(opts.clone()),
      );

    assert!(
      !fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/B"), opts.for_autocomplete)
    );
    assert!(
      fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/B"), !opts.for_autocomplete)
    );
    assert!(
      fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/A"), !opts.for_autocomplete)
    );
    assert!(
      fixture
        .get_frontend()
        .all_module_dependencies_valid(&String::from("game/Gui/Modules/A"), opts.for_autocomplete)
    );
  }
}

mod frontend_test_lint_uses_correct_config {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:849:frontend_test_lint_uses_correct_config`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> method LintOptions::enableWarning (Config/include/Luau/LinterConfig.h)
  //!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
  //!   - calls -> method LintOptions::disableWarning (Config/include/Luau/LinterConfig.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record LintOptions (Config/include/Luau/LinterConfig.h)
  //!   - translates_to -> rust_item frontend_test_lint_uses_correct_config

  #[cfg(test)]
  #[test]
  fn frontend_test_lint_uses_correct_config() {
    use alloc::string::String;

    use ulua_analysis::records::frontend_options::FrontendOptions;
    use ulua_config::records::{
      config::Config, lint_options::LintOptions, lint_warning::LintWarning,
    };
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        local t = {}

        for i=#t,1 do
        end
    "#,
      ),
    );

    let mut config = Config::default();
    config
      .enabled_lint
      .enable_warning(LintWarning::CODE_FOR_RANGE);
    fixture
      .base
      .base
      .config_resolver
      .config_files
      .insert(String::from("Module/A"), config);

    let mut opts = FrontendOptions {
      run_lint_checks: true,
      ..Default::default()
    };
    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts.clone()));
    assert_eq!(1, result.lint_result.warnings.len());

    fixture
      .base
      .base
      .config_resolver
      .config_files
      .get_mut(&String::from("Module/A"))
      .expect("expected config")
      .enabled_lint
      .disable_warning(LintWarning::CODE_FOR_RANGE);
    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/A"), None);

    let result2 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts.clone()));
    assert_eq!(0, result2.lint_result.warnings.len());

    let mut override_options = LintOptions::default();
    override_options.enable_warning(LintWarning::CODE_FOR_RANGE);
    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/A"), None);

    opts.enabled_lint_warnings = Some(override_options);
    let result3 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts.clone()));
    assert_eq!(1, result3.lint_result.warnings.len());

    let mut override_options = LintOptions::default();
    override_options.disable_warning(LintWarning::CODE_FOR_RANGE);
    fixture
      .get_frontend()
      .mark_dirty(&String::from("Module/A"), None);

    opts.enabled_lint_warnings = Some(override_options);
    let result4 = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), Some(opts));
    assert_eq!(0, result4.lint_result.warnings.len());
  }
}

mod frontend_test_prune_parent_segments {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:833:frontend_test_prune_parent_segments`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item frontend_test_prune_parent_segments

  #[cfg(test)]
  #[test]
  fn frontend_test_prune_parent_segments() {
    use ulua_unit_test::functions::path_expr_to_module_name_fixture::path_expr_to_module_name_module_name_vector_string_view;

    assert_eq!(
      Some("Modules/Enum/ButtonState".to_string()),
      path_expr_to_module_name_module_name_vector_string_view(
        "",
        &vec![
          "Modules",
          "LuaApp",
          "DeprecatedDarkTheme",
          "Parent",
          "Parent",
          "Enum",
          "ButtonState",
        ],
      )
    );
    assert_eq!(
      Some("workspace/Foo/Bar/Baz".to_string()),
      path_expr_to_module_name_module_name_vector_string_view(
        "workspace/Foo/Quux",
        &vec!["script", "Parent", "Bar", "Baz"],
      )
    );
    assert_eq!(
      None,
      path_expr_to_module_name_module_name_vector_string_view("", &vec![])
    );
    assert_eq!(
      Some("script".to_string()),
      path_expr_to_module_name_module_name_vector_string_view("", &vec!["script"])
    );
    assert_eq!(
      Some("script/Parent".to_string()),
      path_expr_to_module_name_module_name_vector_string_view("", &vec!["script", "Parent"])
    );
    assert_eq!(
      Some("script".to_string()),
      path_expr_to_module_name_module_name_vector_string_view(
        "",
        &vec!["script", "Parent", "Parent"],
      )
    );
    assert_eq!(
      Some("script".to_string()),
      path_expr_to_module_name_module_name_vector_string_view(
        "",
        &vec!["script", "Test", "Parent"]
      )
    );
    assert_eq!(
      Some("script/Parent".to_string()),
      path_expr_to_module_name_module_name_vector_string_view(
        "",
        &vec!["script", "Test", "Parent", "Parent"],
      )
    );
    assert_eq!(
      Some("script/Parent".to_string()),
      path_expr_to_module_name_module_name_vector_string_view(
        "",
        &vec!["script", "Test", "Parent", "Test", "Parent", "Parent",],
      )
    );
  }
}

mod frontend_test_traverse_dependents {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1599:frontend_test_traverse_dependents`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::traverseDependents (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record SourceNode (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item frontend_test_traverse_dependents

  #[cfg(test)]
  #[test]
  fn frontend_test_traverse_dependents() {
    use alloc::{boxed::Box, string::String, vec::Vec};

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/D"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        return {d_value = C.c_value}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/D"), None);

    let mut visited: Vec<String> = Vec::new();
    let visited_ptr = &mut visited as *mut Vec<String>;
    fixture.get_frontend().traverse_dependents(
      &String::from("game/Gui/Modules/B"),
      Box::new(move |node| {
        unsafe {
          (*visited_ptr).push(node.name.clone());
        }
        true
      }),
    );

    assert_eq!(
      vec![
        String::from("game/Gui/Modules/B"),
        String::from("game/Gui/Modules/C"),
        String::from("game/Gui/Modules/D"),
      ],
      visited
    );
  }
}

mod frontend_test_traverse_dependents_early_exit {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1631:frontend_test_traverse_dependents_early_exit`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method Frontend::traverseDependents (Analysis/src/Frontend.cpp)
  //!   - type_ref -> record SourceNode (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item frontend_test_traverse_dependents_early_exit

  #[cfg(test)]
  #[test]
  fn frontend_test_traverse_dependents_early_exit() {
    use alloc::{boxed::Box, string::String, vec::Vec};

    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/B"),
      String::from(
        r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
      ),
    );
    fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/C"),
      String::from(
        r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
      ),
    );

    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/C"), None);

    let mut visited: Vec<String> = Vec::new();
    let visited_ptr = &mut visited as *mut Vec<String>;
    fixture.get_frontend().traverse_dependents(
      &String::from("game/Gui/Modules/A"),
      Box::new(move |node| {
        unsafe {
          (*visited_ptr).push(node.name.clone());
        }
        node.name != "game/Gui/Modules/B"
      }),
    );

    assert_eq!(
      vec![
        String::from("game/Gui/Modules/A"),
        String::from("game/Gui/Modules/B"),
      ],
      visited
    );
  }
}

mod frontend_trace_requires_in_nonstrict_mode {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:950:frontend_trace_requires_in_nonstrict_mode`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - translates_to -> rust_item frontend_trace_requires_in_nonstrict_mode

  #[cfg(test)]
  #[test]
  fn frontend_trace_requires_in_nonstrict_mode() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        --!nonstrict
        local module = {}

        function module.f(arg: number)
            print('f', arg)
        end

        return module
    "#,
      ),
    );

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/B"),
      String::from(
        r#"
        --!nonstrict
        local A = require(script.Parent.A)

        print(A.g(5))       -- Key 'g' not found
        print(A.f('five'))  -- Type mismatch number and string
        print(A.f(5))       -- OK
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/B"), None);

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(4, result.errors[0].location.begin.line);
    assert_eq!(5, result.errors[1].location.begin.line);
  }
}

mod frontend_typecheck_twice_for_ast_types {
  //! Ported from `tests/Frontend.test.cpp`.
  //! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1108:frontend_typecheck_twice_for_ast_types`
  //! Source: `tests/Frontend.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Frontend.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Common/include/Luau/Common.h
  //!   - includes -> source_file Common/include/Luau/DenseHash.h
  //!   - includes -> source_file Analysis/include/Luau/Frontend.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //! - incoming:
  //!   - declares <- source_file tests/Frontend.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
  //!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
  //!   - translates_to -> rust_item frontend_typecheck_twice_for_ast_types

  #[cfg(test)]
  #[test]
  fn frontend_typecheck_twice_for_ast_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::{
      builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    };

    let mut fixture = FrontendFixture {
      base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
        r#"
        local a = 1
    "#,
      ),
    );

    let result = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("Module/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = fixture
      .get_frontend()
      .module_resolver
      .get_module(&String::from("Module/A"));

    assert_eq!(1, module.ast_types.size());
    let (_, ty) = module
      .ast_types
      .iter()
      .next()
      .expect("expected one ast type entry");
    assert_eq!("number", to_string_type_id(*ty));
  }
}
