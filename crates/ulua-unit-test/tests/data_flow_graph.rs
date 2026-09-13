extern crate alloc;

mod data_flow_graph_define_locals_in_local_stat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:62:data_flow_graph_define_locals_in_local_stat`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_define_locals_in_local_stat

  #[cfg(test)]
  #[test]
  fn data_flow_graph_define_locals_in_local_stat() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x = 5
        local y = x
    "#,
    );

    let _ = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
  }
}

mod data_flow_graph_define_parameters_in_functions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:72:data_flow_graph_define_parameters_in_functions`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_define_parameters_in_functions

  #[cfg(test)]
  #[test]
  fn data_flow_graph_define_parameters_in_functions() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local function f(x)
            local y = x
        end
    "#,
    );

    let _ = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
  }
}

mod data_flow_graph_dfg_captured_local_is_assigned_a_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:697:data_flow_graph_dfg_captured_local_is_assigned_a_function`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_dfg_captured_local_is_assigned_a_function

  #[cfg(test)]
  #[test]
  fn data_flow_graph_dfg_captured_local_is_assigned_a_function() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local f

        local function g()
            f()
        end

        function f()
        end
    "#,
    );

    let f1 = fixture.get_local_def(1, 0);
    let f2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let f3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(f1, f2);
    assert_ne!(f2, f3);

    let f2phi = unsafe { fixture.get_phi(f2) };
    assert!(!f2phi.is_null());
    unsafe {
      assert_eq!((*f2phi).operands.len(), 1);
      assert_eq!((&(*f2phi).operands)[0], f3);
    }
  }
}

mod data_flow_graph_dfg_function_definition_in_a_do_block {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:677:data_flow_graph_dfg_function_definition_in_a_do_block`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item data_flow_graph_dfg_function_definition_in_a_do_block

  #[cfg(test)]
  #[test]
  fn data_flow_graph_dfg_function_definition_in_a_do_block() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local f
        do
            function f()
            end
        end
        f()
    "#,
    );

    let x1 = fixture.get_local_def(1, 0);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(x1, x2);
    assert_ne!(x1, x3);
    assert_eq!(x2, x3);
  }
}

mod data_flow_graph_find_aliases {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:83:data_flow_graph_find_aliases`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_find_aliases

  #[cfg(test)]
  #[test]
  fn data_flow_graph_find_aliases() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x = 5
        local y = x
        local z = y
    "#,
    );

    let x = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let y = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);
    assert_ne!(x, y);
  }
}

mod data_flow_graph_function_captures_are_phi_nodes_of_all_versions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:436:data_flow_graph_function_captures_are_phi_nodes_of_all_versions`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_function_captures_are_phi_nodes_of_all_versions

  #[cfg(test)]
  #[test]
  fn data_flow_graph_function_captures_are_phi_nodes_of_all_versions() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x = 5

        function f()
            print(x)
            x = nil
        end

        f()
        x = "five"
    "#,
    );

    let x1 = fixture.get_local_def(1, 0);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);
    let x4 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(3)]);

    assert_ne!(x1, x2);
    assert_eq!(x2, x3);
    assert_ne!(x3, x4);

    let phi = unsafe { fixture.get_phi(x2) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], x1);
      assert_eq!((&(*phi).operands)[1], x4);
    }
  }
}

mod data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:466:data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties

  #[cfg(test)]
  #[test]
  fn data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties() {
    use ulua_ast::records::{ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal};
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}
        t.x = 5

        function f()
            print(t.x)
            t.x = nil
        end

        f()
        t.x = "five"
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(3)]);
    let x4 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(4)]);

    assert_ne!(x1, x2);
    assert_ne!(x2, x3);
    assert_ne!(x3, x4);

    let t1 = fixture.get_local_def(1, 0);
    let t2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    let phi = unsafe { fixture.get_phi(t2) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 1);
      assert_eq!((&(*phi).operands)[0], t1);
    }
  }
}

mod data_flow_graph_independent_locals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:96:data_flow_graph_independent_locals`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_independent_locals

  #[cfg(test)]
  #[test]
  fn data_flow_graph_independent_locals() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x = 5
        local y = 5

        local a = x
        local b = y
    "#,
    );

    let x = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let y = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);
    assert_ne!(x, y);
  }
}

mod data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:650:data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes

  #[cfg(test)]
  #[test]
  fn data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}

        local function f(k: string)
            if t[k] ~= nil then
                return
            end

            t[k] = 5
        end
    "#,
    );

    let t1 = fixture.get_local_def(1, 0);
    let t2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let t3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(3)]);

    assert_ne!(t1, t2);
    assert_eq!(t2, t3);

    let t2phi = unsafe { fixture.get_phi(t2) };
    assert!(!t2phi.is_null());
    unsafe {
      assert_eq!((*t2phi).operands.len(), 1);
      assert_eq!((&(*t2phi).operands)[0], t1);
    }
  }
}

mod data_flow_graph_local_f_which_is_prototyped_enclosed_by_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:501:data_flow_graph_local_f_which_is_prototyped_enclosed_by_function`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_local_f_which_is_prototyped_enclosed_by_function

  #[cfg(test)]
  #[test]
  fn data_flow_graph_local_f_which_is_prototyped_enclosed_by_function() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local f
        function f()
            if cond() then
                f()
            end
        end
    "#,
    );

    let f1 = fixture.get_local_def(1, 0);
    let f2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let f3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(f1, f2);
    assert_ne!(f2, f3);

    let phi = unsafe { fixture.get_phi(f3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 1);
      assert_eq!((&(*phi).operands)[0], f2);
    }
  }
}

mod data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_future_versions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:552:data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_future_versions`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_future_versions

  #[cfg(test)]
  #[test]
  fn data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_future_versions() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local f
        function f()
            if cond() then
                f()
            end
        end
        f = 5
    "#,
    );

    let f1 = fixture.get_local_def(1, 0);
    let f2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let f3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);
    let f4 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(3)]);

    assert_ne!(f1, f2);
    assert_ne!(f2, f3);
    assert_ne!(f3, f4);

    let phi = unsafe { fixture.get_phi(f3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], f2);
      assert_eq!((&(*phi).operands)[1], f4);
    }
  }
}

mod data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:525:data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions

  #[cfg(test)]
  #[test]
  fn data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local f
        f = 5
        function f()
            if cond() then
                f()
            end
        end
    "#,
    );

    let f1 = fixture.get_local_def(1, 0);
    let f2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let f3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);
    let f4 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(3)]);

    assert_ne!(f1, f2);
    assert_ne!(f2, f3);
    assert_ne!(f3, f4);

    let phi = unsafe { fixture.get_phi(f4) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 1);
      assert_eq!((&(*phi).operands)[0], f3);
    }
  }
}

mod data_flow_graph_mutate_local_not_owned_by_for {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:206:data_flow_graph_mutate_local_not_owned_by_for`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> method DataFlowGraphFixture::checkOperands (tests/DataFlowGraph.test.cpp)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_not_owned_by_for

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_not_owned_by_for() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x

        for i = 0, 5 do
            x = true
        end

        local y = x
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    let phi = unsafe { fixture.get_phi(x2) };
    assert!(!phi.is_null());
    unsafe { fixture.check_operands(phi, vec![x0, x1]) };
  }
}

mod data_flow_graph_mutate_local_not_owned_by_for_in {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:245:data_flow_graph_mutate_local_not_owned_by_for_in`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> method DataFlowGraphFixture::checkOperands (tests/DataFlowGraph.test.cpp)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_not_owned_by_for_in

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_not_owned_by_for_in() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x

        for i, v in t do
            x = true
        end

        local y = x
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    let phi = unsafe { fixture.get_phi(x2) };
    assert!(!phi.is_null());
    unsafe { fixture.check_operands(phi, vec![x0, x1]) };
  }
}

mod data_flow_graph_mutate_local_not_owned_by_repeat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:168:data_flow_graph_mutate_local_not_owned_by_repeat`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_not_owned_by_repeat

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_not_owned_by_repeat() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x

        repeat
            x = true
        until cond()

        local y = x
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(x0, x1);
    assert_eq!(x1, x2);
  }
}

mod data_flow_graph_mutate_local_not_owned_by_while {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:129:data_flow_graph_mutate_local_not_owned_by_while`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> method DataFlowGraphFixture::checkOperands (tests/DataFlowGraph.test.cpp)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_not_owned_by_while

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_not_owned_by_while() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x

        while cond() do
            x = true
        end

        local y = x
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    let phi = unsafe { fixture.get_phi(x2) };
    assert!(!phi.is_null());
    unsafe { fixture.check_operands(phi, vec![x0, x1]) };
  }
}

mod data_flow_graph_mutate_local_owned_by_for {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:227:data_flow_graph_mutate_local_owned_by_for`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_owned_by_for

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_owned_by_for() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        for i = 0, 5 do
            local x
            x = true
            x = 5
        end
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(x0, x1);
    assert_ne!(x1, x2);
  }
}

mod data_flow_graph_mutate_local_owned_by_for_in {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:266:data_flow_graph_mutate_local_owned_by_for_in`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_owned_by_for_in

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_owned_by_for_in() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        for i, v in t do
            local x
            x = true
            x = 5
        end
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(x0, x1);
    assert_ne!(x1, x2);
  }
}

mod data_flow_graph_mutate_local_owned_by_repeat {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:188:data_flow_graph_mutate_local_owned_by_repeat`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_owned_by_repeat

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_owned_by_repeat() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        repeat
            local x
            x = true
            x = 5
        until cond()
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(x0, x1);
    assert_ne!(x1, x2);
  }
}

mod data_flow_graph_mutate_local_owned_by_while {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:150:data_flow_graph_mutate_local_owned_by_while`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_local_owned_by_while

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_local_owned_by_while() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        while cond() do
            local x
            x = true
            x = 5
        end
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    assert_ne!(x0, x1);
    assert_ne!(x1, x2);
  }
}

mod data_flow_graph_mutate_non_preexisting_property_not_owned_by_while {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:306:data_flow_graph_mutate_non_preexisting_property_not_owned_by_while`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_non_preexisting_property_not_owned_by_while

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_non_preexisting_property_not_owned_by_while() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}

        while cond() do
            t.x = true
        end

        local y = t.x
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);

    assert_eq!(x1, x2);
  }
}

mod data_flow_graph_mutate_preexisting_property_not_owned_by_while {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:284:data_flow_graph_mutate_preexisting_property_not_owned_by_while`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> method DataFlowGraphFixture::checkOperands (tests/DataFlowGraph.test.cpp)
  //!   - translates_to -> rust_item data_flow_graph_mutate_preexisting_property_not_owned_by_while

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_preexisting_property_not_owned_by_while() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}
        t.x = 5

        while cond() do
            t.x = true
        end

        local y = t.x
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(3)]);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe { fixture.check_operands(phi, vec![x1, x2]) };
  }
}

mod data_flow_graph_mutate_property_of_table_owned_by_while {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:324:data_flow_graph_mutate_property_of_table_owned_by_while`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - translates_to -> rust_item data_flow_graph_mutate_property_of_table_owned_by_while

  #[cfg(test)]
  #[test]
  fn data_flow_graph_mutate_property_of_table_owned_by_while() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        while cond() do
            local t = {}
            t.x = true
            t.x = 5
        end
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);

    assert_ne!(x1, x2);
  }
}

mod data_flow_graph_phi {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:111:data_flow_graph_phi`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_phi

  #[cfg(test)]
  #[test]
  fn data_flow_graph_phi() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local x

        if a then
            x = true
        end

        local y = x
    "#,
    );

    let y = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);
    let phi = unsafe { fixture.get_phi(y) };
    assert!(!phi.is_null());
  }
}

mod data_flow_graph_phi_node_if_case_binding {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:580:data_flow_graph_phi_node_if_case_binding`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function query (tests/AstQueryDsl.h)
  //!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_phi_node_if_case_binding

  #[cfg(test)]
  #[test]
  fn data_flow_graph_phi_node_if_case_binding() {
    use ulua_ast::records::ast_expr_local::AstExprLocal;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
local x = nil
if true then
    if true then
        x = 5
    end
    print(x)
else
    print(x)
end
"#,
    );

    let x1 = fixture.get_local_def(1, 0);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(1)]);
    let x3 = fixture.get_def::<AstExprLocal>(vec![nth_t::<AstExprLocal>(2)]);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((&(*phi).operands)[0], x2);
      assert_eq!((&(*phi).operands)[1], x1);
    }
  }
}

mod data_flow_graph_phi_node_if_case_table_prop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:603:data_flow_graph_phi_node_if_case_table_prop`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_phi_node_if_case_table_prop

  #[cfg(test)]
  #[test]
  fn data_flow_graph_phi_node_if_case_table_prop() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
local t = {}
t.x = true
if true then
    if true then
        t.x = 5
    end
    print(t.x)
else
    print(t.x)
end
"#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(3)]);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], x1);
      assert_eq!((&(*phi).operands)[1], x2);
    }
  }
}

mod data_flow_graph_phi_node_if_case_table_prop_literal {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:629:data_flow_graph_phi_node_if_case_table_prop_literal`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_phi_node_if_case_table_prop_literal

  #[cfg(test)]
  #[test]
  fn data_flow_graph_phi_node_if_case_table_prop_literal() {
    use ulua_ast::records::{
      ast_expr_constant_bool::AstExprConstantBool, ast_expr_index_name::AstExprIndexName,
    };
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
local t = { x = true }
if true then
    t.x = 5
end
print(t.x)

"#,
    );

    let x1 = fixture.get_def::<AstExprConstantBool>(vec![nth_t::<AstExprConstantBool>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], x1);
      assert_eq!((&(*phi).operands)[1], x2);
    }
  }
}

mod data_flow_graph_property_lookup_on_a_phi_node {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:340:data_flow_graph_property_lookup_on_a_phi_node`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_property_lookup_on_a_phi_node

  #[cfg(test)]
  #[test]
  fn data_flow_graph_property_lookup_on_a_phi_node() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}
        t.x = 5

        if cond() then
            t.x = 7
        end

        print(t.x)
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(3)]);

    assert_ne!(x1, x2);
    assert_ne!(x2, x3);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], x1);
      assert_eq!((&(*phi).operands)[1], x2);
    }
  }
}

mod data_flow_graph_property_lookup_on_a_phi_node_2 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:367:data_flow_graph_property_lookup_on_a_phi_node_2`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_property_lookup_on_a_phi_node_2

  #[cfg(test)]
  #[test]
  fn data_flow_graph_property_lookup_on_a_phi_node_2() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}

        if cond() then
            t.x = 5
        else
            t.x = 7
        end

        print(t.x)
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(3)]);

    assert_ne!(x1, x2);
    assert_ne!(x2, x3);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], x2);
      assert_eq!((&(*phi).operands)[1], x1);
    }
  }
}

mod data_flow_graph_property_lookup_on_a_phi_node_3 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:395:data_flow_graph_property_lookup_on_a_phi_node_3`
  //! Source: `tests/DataFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/DataFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Def.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/AstQueryDsl.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/DataFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
  //!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
  //!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item data_flow_graph_property_lookup_on_a_phi_node_3

  #[cfg(test)]
  #[test]
  fn data_flow_graph_property_lookup_on_a_phi_node_3() {
    use ulua_ast::records::ast_expr_index_name::AstExprIndexName;
    use ulua_unit_test::{
      functions::nth::nth_t, records::data_flow_graph_fixture::DataFlowGraphFixture,
    };

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
      r#"
        local t = {}
        t.x = 3

        if cond() then
            t.x = 5
            t.y = 7
        else
            t.z = 42
        end

        print(t.x)
        print(t.y)
        print(t.z)
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(2)]);
    let y1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(3)]);
    let z1 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(4)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(5)]);
    let y2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(6)]);
    let z2 = fixture.get_def::<AstExprIndexName>(vec![nth_t::<AstExprIndexName>(7)]);

    assert_ne!(x1, x2);
    assert_ne!(x2, x3);
    assert_eq!(y1, y2);
    assert_eq!(z1, z2);

    let phi = unsafe { fixture.get_phi(x3) };
    assert!(!phi.is_null());
    unsafe {
      assert_eq!((*phi).operands.len(), 2);
      assert_eq!((&(*phi).operands)[0], x1);
      assert_eq!((&(*phi).operands)[1], x2);
    }
  }
}
