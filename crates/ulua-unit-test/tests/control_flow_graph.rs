extern crate alloc;

mod control_flow_graph_basic_join {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:241:control_flow_graph_basic_join`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - type_ref -> enum BlockKind (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> function checkSuccessors (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function checkPredecessors (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item control_flow_graph_basic_join

  #[cfg(test)]
  #[test]
  fn control_flow_graph_basic_join() {
    use ulua_analysis::{
      enums::block_kind::BlockKind,
      records::{assign::Assign, declare::Declare, join::Join},
    };
    use ulua_unit_test::{
      functions::{
        check_join::check_join, check_predecessors::check_predecessors,
        check_successors::check_successors, require_inst::require_inst,
      },
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local t = 8
        if true then
            t = 9
        else
            t = "hello"
        end
        local y = t
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let entry = cfg.blocks[0];
    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];
    let merge = cfg.blocks[3];

    assert_eq!(BlockKind::Entry, unsafe { (*entry).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*then_blk).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*else_blk).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*merge).kind });

    unsafe { check_successors(cfg, entry, &[1, 2]) };
    unsafe { check_successors(cfg, then_blk, &[3]) };
    unsafe { check_successors(cfg, else_blk, &[3]) };
    unsafe { check_predecessors(cfg, merge, &[1, 2]) };

    assert_eq!("t-0", unsafe {
      (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });
    assert_eq!("t-1", unsafe {
      (*(*require_inst::<Assign>(then_blk, 0)).def).versioned_name()
    });
    assert_eq!("t-2", unsafe {
      (*(*require_inst::<Assign>(else_blk, 0)).def).versioned_name()
    });

    let phi = unsafe { require_inst::<Join>(merge, 0) };
    unsafe { check_join(phi, "t-3", &["t-1", "t-2"]) };

    let decl_y = unsafe { require_inst::<Declare>(merge, 1) };
    assert_eq!("y-0", unsafe { (*(*decl_y).def).versioned_name() });
  }
}

mod control_flow_graph_conjunction_emits_flow_per_side {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:415:control_flow_graph_conjunction_emits_flow_per_side`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_conjunction_emits_flow_per_side

  #[cfg(test)]
  #[test]
  fn control_flow_graph_conjunction_emits_flow_per_side() {
    use ulua_analysis::records::{declare::Declare, refine::Refine};
    use ulua_unit_test::{
      functions::{check_refine::check_refine, require_inst::require_inst},
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = nil
        local y = nil
        if x and y then
            local z = x
        end
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];

    unsafe {
      check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        true,
        None,
        false,
      )
    };
    unsafe {
      check_refine(
        require_inst::<Refine>(then_blk, 1),
        "y-1",
        "y-0",
        true,
        None,
        false,
      )
    };
    assert_eq!("z-0", unsafe {
      (*(*require_inst::<Declare>(then_blk, 2)).def).versioned_name()
    });
  }
}

mod control_flow_graph_if_falsy_single_branch {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:357:control_flow_graph_if_falsy_single_branch`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item control_flow_graph_if_falsy_single_branch

  #[cfg(test)]
  #[test]
  fn control_flow_graph_if_falsy_single_branch() {
    use ulua_analysis::records::{join::Join, refine::Refine};
    use ulua_unit_test::{
      functions::{check_join::check_join, check_refine::check_refine, require_inst::require_inst},
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = nil
        if not x then
            local y = x
        end
        local z = x
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];
    let merge = cfg.blocks[3];

    unsafe {
      check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        false,
        None,
        false,
      )
    };
    unsafe {
      check_refine(
        require_inst::<Refine>(else_blk, 0),
        "x-2",
        "x-0",
        true,
        None,
        false,
      )
    };

    let phi = unsafe { require_inst::<Join>(merge, 0) };
    unsafe { check_join(phi, "x-3", &["x-1", "x-2"]) };
  }
}

mod control_flow_graph_if_truthy_both_branches {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:328:control_flow_graph_if_truthy_both_branches`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function merge (tests/LValue.test.cpp)
  //!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
  //!   - translates_to -> rust_item control_flow_graph_if_truthy_both_branches

  #[cfg(test)]
  #[test]
  fn control_flow_graph_if_truthy_both_branches() {
    use ulua_analysis::records::{declare::Declare, join::Join, refine::Refine};
    use ulua_unit_test::{
      functions::{check_join::check_join, check_refine::check_refine, require_inst::require_inst},
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = nil
        if x then
            local y = x
        else
            local z = x
        end

        local y = x
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];
    let merge = cfg.blocks[3];

    unsafe {
      check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        true,
        None,
        false,
      )
    };
    assert_eq!("y-0", unsafe {
      (*(*require_inst::<Declare>(then_blk, 1)).def).versioned_name()
    });

    unsafe {
      check_refine(
        require_inst::<Refine>(else_blk, 0),
        "x-2",
        "x-0",
        false,
        None,
        false,
      )
    };
    assert_eq!("z-0", unsafe {
      (*(*require_inst::<Declare>(else_blk, 1)).def).versioned_name()
    });

    let phi = unsafe { require_inst::<Join>(merge, 0) };
    unsafe { check_join(phi, "x-3", &["x-1", "x-2"]) };
    assert_eq!("y-0", unsafe {
      (*(*require_inst::<Declare>(merge, 1)).def).versioned_name()
    });
  }
}

mod control_flow_graph_multi_assignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:220:control_flow_graph_multi_assignment`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Position (Ast/include/Luau/Location.h)
  //!   - translates_to -> rust_item control_flow_graph_multi_assignment

  #[cfg(test)]
  #[test]
  fn control_flow_graph_multi_assignment() {
    use ulua_analysis::records::{assign::Assign, declare::Declare};
    use ulua_ast::records::position::Position;
    use ulua_unit_test::{functions::require_inst::require_inst, records::cfg_fixture::CfgFixture};

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local a, b = 1, 2
        a, b = b, a
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];

    assert_eq!("a-0", unsafe {
      (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });
    assert_eq!("b-0", unsafe {
      (*(*require_inst::<Declare>(entry, 1)).def).versioned_name()
    });
    assert_eq!("a-1", unsafe {
      (*(*require_inst::<Assign>(entry, 2)).def).versioned_name()
    });
    assert_eq!("b-1", unsafe {
      (*(*require_inst::<Assign>(entry, 3)).def).versioned_name()
    });

    assert_eq!("b-0", unsafe {
      (*fixture.get_definition_at_pos(cfg, Position::new(2, 15))).versioned_name()
    });
    assert_eq!("a-0", unsafe {
      (*fixture.get_definition_at_pos(cfg, Position::new(2, 18))).versioned_name()
    });
  }
}

mod control_flow_graph_reassignment_from_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:204:control_flow_graph_reassignment_from_local`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_reassignment_from_local

  #[cfg(test)]
  #[test]
  fn control_flow_graph_reassignment_from_local() {
    use ulua_analysis::records::{assign::Assign, declare::Declare};
    use ulua_unit_test::{functions::require_inst::require_inst, records::cfg_fixture::CfgFixture};

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = 1
        local y = 2
        x = y
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];

    assert_eq!("x-0", unsafe {
      (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });
    assert_eq!("y-0", unsafe {
      (*(*require_inst::<Declare>(entry, 1)).def).versioned_name()
    });
    assert_eq!("x-1", unsafe {
      (*(*require_inst::<Assign>(entry, 2)).def).versioned_name()
    });
  }
}

mod control_flow_graph_simple_reassignment {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:188:control_flow_graph_simple_reassignment`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_simple_reassignment

  #[cfg(test)]
  #[test]
  fn control_flow_graph_simple_reassignment() {
    use ulua_analysis::records::{assign::Assign, declare::Declare};
    use ulua_unit_test::{functions::require_inst::require_inst, records::cfg_fixture::CfgFixture};

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = 4
        x = 5
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];

    let decl = unsafe { require_inst::<Declare>(entry, 0) };
    assert_eq!("x-0", unsafe { (*(*decl).def).versioned_name() });

    let assign = unsafe { require_inst::<Assign>(entry, 1) };
    assert_eq!("x-1", unsafe { (*(*assign).def).versioned_name() });
  }
}

mod control_flow_graph_single_local {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:157:control_flow_graph_single_local`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> enum BlockKind (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> method Block::getInstructions (Analysis/src/ControlFlowGraph.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_single_local

  #[cfg(test)]
  #[test]
  fn control_flow_graph_single_local() {
    use ulua_analysis::{enums::block_kind::BlockKind, records::declare::Declare};
    use ulua_unit_test::{functions::require_inst::require_inst, records::cfg_fixture::CfgFixture};

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = 4
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];
    assert_eq!(BlockKind::Entry, unsafe { (*entry).kind });
    assert_eq!(1, unsafe { (*entry).get_instructions().len() });

    let decl = unsafe { require_inst::<Declare>(entry, 0) };
    assert_eq!("x-0", unsafe { (*(*decl).def).versioned_name() });
  }
}

mod control_flow_graph_two_locals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:172:control_flow_graph_two_locals`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_two_locals

  #[cfg(test)]
  #[test]
  fn control_flow_graph_two_locals() {
    use ulua_analysis::records::declare::Declare;
    use ulua_unit_test::{functions::require_inst::require_inst, records::cfg_fixture::CfgFixture};

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = 4
        local y = 5
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];

    let decl_x = unsafe { require_inst::<Declare>(entry, 0) };
    assert_eq!("x-0", unsafe { (*(*decl_x).def).versioned_name() });

    let decl_y = unsafe { require_inst::<Declare>(entry, 1) };
    assert_eq!("y-0", unsafe { (*(*decl_y).def).versioned_name() });
  }
}

mod control_flow_graph_type_guard_inequality_flips_sense {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:398:control_flow_graph_type_guard_inequality_flips_sense`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_type_guard_inequality_flips_sense

  #[cfg(test)]
  #[test]
  fn control_flow_graph_type_guard_inequality_flips_sense() {
    use ulua_analysis::records::refine::Refine;
    use ulua_unit_test::{
      functions::{check_refine::check_refine, require_inst::require_inst},
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = nil
        if type(x) ~= "string" then
            local y = x
        end
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];

    unsafe {
      check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        false,
        Some("string"),
        false,
      )
    };
    unsafe {
      check_refine(
        require_inst::<Refine>(else_blk, 0),
        "x-2",
        "x-0",
        true,
        Some("string"),
        false,
      )
    };
  }
}

mod control_flow_graph_typeof_guard_emits_type_proposition {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:381:control_flow_graph_typeof_guard_emits_type_proposition`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_typeof_guard_emits_type_proposition

  #[cfg(test)]
  #[test]
  fn control_flow_graph_typeof_guard_emits_type_proposition() {
    use ulua_analysis::records::refine::Refine;
    use ulua_unit_test::{
      functions::{check_refine::check_refine, require_inst::require_inst},
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = nil
        if typeof(x) == "string" then
            local y = x
        end
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];

    unsafe {
      check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        true,
        Some("string"),
        true,
      )
    };
    unsafe {
      check_refine(
        require_inst::<Refine>(else_blk, 0),
        "x-2",
        "x-0",
        false,
        Some("string"),
        true,
      )
    };
  }
}

mod control_flow_graph_while_loop {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:280:control_flow_graph_while_loop`
  //! Source: `tests/ControlFlowGraph.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ControlFlowGraph.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file Analysis/include/Luau/AstQuery.h
  //!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
  //!   - includes -> source_file Ast/include/Luau/Parser.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ControlFlowGraph.test.cpp
  //! - outgoing:
  //!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> enum BlockKind (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
  //!   - calls -> function checkSuccessors (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
  //!   - calls -> function checkPredecessors (tests/ControlFlowGraph.test.cpp)
  //!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
  //!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - translates_to -> rust_item control_flow_graph_while_loop

  #[cfg(test)]
  #[test]
  fn control_flow_graph_while_loop() {
    use ulua_analysis::{
      enums::block_kind::BlockKind,
      records::{assign::Assign, declare::Declare, join::Join, refine::Refine},
    };
    use ulua_unit_test::{
      functions::{
        check_join::check_join, check_predecessors::check_predecessors, check_refine::check_refine,
        check_successors::check_successors, require_inst::require_inst,
      },
      records::cfg_fixture::CfgFixture,
    };

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
      r#"
        local x = nil
        while not x do
            x = 5
        end
        local y = x
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let entry = cfg.blocks[0];
    let header = cfg.blocks[1];
    let body = cfg.blocks[2];
    let exit = cfg.blocks[3];

    assert_eq!(BlockKind::Entry, unsafe { (*entry).kind });
    assert_eq!(BlockKind::Condition, unsafe { (*header).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*body).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*exit).kind });

    unsafe { check_successors(cfg, entry, &[1]) };
    unsafe { check_successors(cfg, header, &[2, 3]) };
    unsafe { check_successors(cfg, body, &[1]) };
    unsafe { check_predecessors(cfg, header, &[0, 2]) };

    assert_eq!("x-0", unsafe {
      (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });

    let phi = unsafe { require_inst::<Join>(header, 0) };
    unsafe { check_join(phi, "x-1", &["x-0", "x-3"]) };

    let body_refine = unsafe { require_inst::<Refine>(body, 0) };
    unsafe {
      check_refine(body_refine, "x-2", "x-1", false, None, false);
    }
    assert_eq!("x-3", unsafe {
      (*(*require_inst::<Assign>(body, 1)).def).versioned_name()
    });

    let exit_refine = unsafe { require_inst::<Refine>(exit, 0) };
    unsafe {
      check_refine(exit_refine, "x-4", "x-1", true, None, false);
    }
    assert_eq!("y-0", unsafe {
      (*(*require_inst::<Declare>(exit, 1)).def).versioned_name()
    });
  }
}
