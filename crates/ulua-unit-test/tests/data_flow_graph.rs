extern crate alloc;

mod data_flow_graph_define_locals_in_local_stat {
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
  //! Source: `tests/DataFlowGraph.test.cpp`

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
