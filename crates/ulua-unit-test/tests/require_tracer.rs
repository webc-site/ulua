extern crate alloc;

mod require_tracer_follow_group {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:214:require_tracer_follow_group`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_follow_group

  #[cfg(test)]
  #[test]
  fn require_tracer_follow_group() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local R = (((game).Test))
        require(R)
    "#,
    );
    unsafe {
      assert_eq!(2, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
      assert!(!local.is_null());

      let value = *(*local).values.data as *mut AstNode;
      assert_eq!("game/Test", result.exprs.find(&value).unwrap().name);
    }
  }
}

mod require_tracer_follow_string_indexexpr {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:198:require_tracer_follow_string_indexexpr`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_follow_string_indexexpr

  #[cfg(test)]
  #[test]
  fn require_tracer_follow_string_indexexpr() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local R = game["Test"]
        require(R)
    "#,
    );
    unsafe {
      assert_eq!(2, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
      assert!(!local.is_null());

      let value = *(*local).values.data as *mut AstNode;
      assert_eq!("game/Test", result.exprs.find(&value).unwrap().name);
    }
  }
}

mod require_tracer_follow_type_annotation {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:230:require_tracer_follow_type_annotation`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_follow_type_annotation

  #[cfg(test)]
  #[test]
  fn require_tracer_follow_type_annotation() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local R = game.Test :: (typeof(game.Redirect))
        require(R)
    "#,
    );
    unsafe {
      assert_eq!(2, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
      assert!(!local.is_null());

      let value = *(*local).values.data as *mut AstNode;
      assert_eq!("game/Redirect", result.exprs.find(&value).unwrap().name);
    }
  }
}

mod require_tracer_follow_type_annotation_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:246:require_tracer_follow_type_annotation_2`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record Test (tests/NotNull.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_follow_type_annotation_2

  #[cfg(test)]
  #[test]
  fn require_tracer_follow_type_annotation_2() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local R = game.Test :: (typeof(game.Redirect))
        local N = R.Nested
        require(N)
    "#,
    );
    unsafe {
      assert_eq!(3, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data.add(1) as *mut AstNode);
      assert!(!local.is_null());

      let value = *(*local).values.data as *mut AstNode;
      assert_eq!(
        "game/Redirect/Nested",
        result.exprs.find(&value).unwrap().name
      );
    }
  }
}

mod require_tracer_follow_typeof {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:134:require_tracer_follow_typeof`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstType (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeTypeof (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_follow_typeof

  #[cfg(test)]
  #[test]
  fn require_tracer_follow_typeof() {
    use std::ffi::CStr;

    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{
        ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
        ast_stat_local::AstStatLocal, ast_type_typeof::AstTypeTypeof,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local R: typeof(require(workspace.CoolThing).UsefulObject)
    "#,
    );
    unsafe {
      assert_eq!(1, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
      assert!(!local.is_null());
      assert_eq!(1, (*local).vars.size);

      let ann = (*(*(*local).vars.data)).annotation;
      assert!(!ann.is_null());

      let typeof_annotation = ast_node_as::<AstTypeTypeof>(ann as *mut AstNode);
      assert!(!typeof_annotation.is_null());

      let index_name = ast_node_as::<AstExprIndexName>((*typeof_annotation).expr as *mut AstNode);
      assert!(!index_name.is_null());
      assert_eq!(
        "UsefulObject",
        CStr::from_ptr((*index_name).index.value).to_str().unwrap()
      );

      let call = ast_node_as::<AstExprCall>((*index_name).expr as *mut AstNode);
      assert!(!call.is_null());
      assert_eq!(1, (*call).args.size);

      let arg = *(*call).args.data as *mut AstNode;
      assert_eq!("workspace/CoolThing", result.exprs.find(&arg).unwrap().name);
    }
  }
}

mod require_tracer_follow_typeof_in_return_type {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:165:require_tracer_follow_typeof_in_return_type`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePack (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstTypeTypeof (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_follow_typeof_in_return_type

  #[cfg(test)]
  #[test]
  fn require_tracer_follow_typeof_in_return_type() {
    use std::ffi::CStr;

    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{
        ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
        ast_stat_function::AstStatFunction, ast_type_pack_explicit::AstTypePackExplicit,
        ast_type_typeof::AstTypeTypeof,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        function foo(): typeof(require(workspace.CoolThing).UsefulObject)
        end
    "#,
    );
    unsafe {
      assert_eq!(1, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let func = ast_node_as::<AstStatFunction>(*(*block).body.data as *mut AstNode);
      assert!(!func.is_null());

      let ret_annotation = (*(*func).func).return_annotation;
      assert!(!ret_annotation.is_null());

      let tp = ast_node_as::<AstTypePackExplicit>(ret_annotation as *mut AstNode);
      assert!(!tp.is_null());
      assert_eq!(1, (*tp).type_list.types.size);

      let typeof_annotation =
        ast_node_as::<AstTypeTypeof>(*(*tp).type_list.types.data as *mut AstNode);
      assert!(!typeof_annotation.is_null());

      let index_name = ast_node_as::<AstExprIndexName>((*typeof_annotation).expr as *mut AstNode);
      assert!(!index_name.is_null());
      assert_eq!(
        "UsefulObject",
        CStr::from_ptr((*index_name).index.value).to_str().unwrap()
      );

      let call = ast_node_as::<AstExprCall>((*index_name).expr as *mut AstNode);
      assert!(!call.is_null());
      assert_eq!(1, (*call).args.size);

      let arg = *(*call).args.data as *mut AstNode;
      assert_eq!("workspace/CoolThing", result.exprs.find(&arg).unwrap().name);
    }
  }
}

mod require_tracer_trace_function_arguments {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:112:require_tracer_trace_function_arguments`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
  //!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_trace_function_arguments

  #[cfg(test)]
  #[test]
  fn require_tracer_trace_function_arguments() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{ast_expr_call::AstExprCall, ast_node::AstNode, ast_stat_local::AstStatLocal},
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local M = require(workspace.Game.Thing)
    "#,
    );
    unsafe {
      assert_eq!(1, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
      assert!(!local.is_null());
      assert_eq!(1, (*local).vars.size);
      assert_eq!(1, (*local).values.size);

      let call = ast_node_as::<AstExprCall>(*(*local).values.data as *mut AstNode);
      assert!(!call.is_null());
      assert_eq!(1, (*call).args.size);

      let arg = *(*call).args.data as *mut AstNode;
      assert_eq!(
        "workspace/Game/Thing",
        result.exprs.find(&arg).unwrap().name
      );
    }
  }
}

mod require_tracer_trace_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:56:require_tracer_trace_local`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item require_tracer_trace_local

  #[cfg(test)]
  #[test]
  fn require_tracer_trace_local() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{
        ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
        ast_stat_local::AstStatLocal,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local m = workspace.Foo.Bar.Baz
        require(m)
    "#,
    );

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };
    assert!(!result.exprs.empty());

    unsafe {
      let loc = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
      assert!(!loc.is_null());
      assert_eq!(1, (*loc).vars.size);
      assert_eq!(1, (*loc).values.size);

      let mut value = ast_node_as::<AstExprIndexName>(*(*loc).values.data as *mut AstNode);
      assert!(!value.is_null());
      assert_eq!(
        "workspace/Foo/Bar/Baz",
        result.exprs.find(&(value as *mut AstNode)).unwrap().name
      );

      value = ast_node_as::<AstExprIndexName>((*value).expr as *mut AstNode);
      assert!(!value.is_null());
      assert_eq!(
        "workspace/Foo/Bar",
        result.exprs.find(&(value as *mut AstNode)).unwrap().name
      );

      value = ast_node_as::<AstExprIndexName>((*value).expr as *mut AstNode);
      assert!(!value.is_null());
      assert_eq!(
        "workspace/Foo",
        result.exprs.find(&(value as *mut AstNode)).unwrap().name
      );

      let workspace = ast_node_as::<AstExprGlobal>((*value).expr as *mut AstNode);
      assert!(!workspace.is_null());
      assert_eq!(
        "workspace",
        result
          .exprs
          .find(&(workspace as *mut AstNode))
          .unwrap()
          .name
      );
    }
  }
}

mod require_tracer_trace_transitive_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:92:require_tracer_trace_transitive_local`
  //! Source: `tests/RequireTracer.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RequireTracer.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/RequireTracer.test.cpp
  //! - outgoing:
  //!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
  //!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
  //!   - type_ref -> record Foo (tests/Variant.test.cpp)
  //!   - type_ref -> record Bar (tests/Variant.test.cpp)
  //!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
  //!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item require_tracer_trace_transitive_local

  #[cfg(test)]
  #[test]
  fn require_tracer_trace_transitive_local() {
    use ulua_analysis::{
      functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
    };
    use ulua_ast::{
      records::{ast_node::AstNode, ast_stat_local::AstStatLocal},
      rtti::ast_node_as,
    };
    use ulua_unit_test::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
      r#"
        local m = workspace.Foo.Bar.Baz
        local n = m.Quux
        require(n)
    "#,
    );

    unsafe {
      assert_eq!(3, (*block).body.size);
    }

    let result = unsafe {
      trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
      )
    };

    unsafe {
      let local = ast_node_as::<AstStatLocal>(*(*block).body.data.add(1) as *mut AstNode);
      assert!(!local.is_null());
      assert_eq!(1, (*local).vars.size);

      let value = *(*local).values.data as *mut AstNode;
      assert!(result.exprs.contains(&value));
      assert_eq!(
        "workspace/Foo/Bar/Baz/Quux",
        result.exprs.find(&value).unwrap().name
      );
    }
  }
}
