// 集成测试共享样板：fixture 前奏宏 fx_check!/bs_check! 由 common 上提，#[macro_export] 后按裸名调用。
mod common;
extern crate alloc;

use ulua_analysis::records::{
  block::Block, block_registry::resolve_block, control_flow_graph::ControlFlowGraph,
};

/// `cfg.blocks` 元素自 #17 续起为 `BlockId` u32 句柄：测试侧统一经
/// `resolve_block` 取只读视图，不再手写 `unsafe { &*ptr }`。
fn block_at(cfg: &ControlFlowGraph, idx: usize) -> &Block {
  resolve_block(cfg.blocks[idx])
    .expect("cfg.blocks 句柄为 fixture 构建期 register_block 发放的存活句柄")
}

/// 收形：`TEST_SUITE("CFGTypeCheckTest")` 中 7 个同形 `Fixture` 用例
/// （cpp 侧各以 `ScopedFastFlag{DebugLuauCFG, true}` 打开 CFG 驱动的类型检查；
/// Rust 侧走旧求解器等价路径，见文件末尾的缺口说明）为宏生成的独立 `#[test]`——
/// `Fixture::fixture_bool(false)` + check + 零错误 + 逐位置类型断言。
/// 出处行号逐例保留在调用点的 Source 注释中。
macro_rules! cfg_typecheck_case {
  ($name:ident, src: $src:expr, [ $( ($l:expr, $c:expr, $t:expr) ),+ $(,)? ]) => {
    #[test]
    fn $name() {
      use ulua_analysis::functions::to_string_to_string::to_string_type_id;
      use ulua_ast::records::position::Position;
      use ulua_unit_test::records::fixture::Fixture;

      let mut fixture = Fixture::fixture_bool(false);
      let result = fixture.check_string_optional_frontend_options($src, None);

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
      $(
        assert_eq!(
          $t,
          to_string_type_id(fixture.require_type_at_position_position(Position::new($l, $c)))
        );
      )+
    }
  };
}

// Source: `tests/ControlFlowGraph.test.cpp:240-277`
#[test]
fn control_flow_graph_basic_join() {
  use ulua_analysis::{
    enums::block_kind::BlockKind,
    records::{assign::Assign, declare::Declare, join::Join},
  };
  use ulua_unit_test::{
    functions::{
      check_join::check_join, check_predecessors::check_predecessors,
      check_successors::check_successors, def_versioned_name::def_versioned_name,
      require_inst::require_inst,
    },
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
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
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);
  let then_blk = crate::block_at(cfg, 1);
  let else_blk = crate::block_at(cfg, 2);
  let merge = crate::block_at(cfg, 3);

  assert_eq!(BlockKind::Entry, entry.kind);
  assert_eq!(BlockKind::Linear, then_blk.kind);
  assert_eq!(BlockKind::Linear, else_blk.kind);
  assert_eq!(BlockKind::Linear, merge.kind);

  check_successors(cfg, entry, &[1, 2]);
  check_successors(cfg, then_blk, &[3]);
  check_successors(cfg, else_blk, &[3]);
  check_predecessors(cfg, merge, &[1, 2]);

  assert_eq!(
    "t-0",
    def_versioned_name(require_inst::<Declare>(entry, 0).def)
  );
  assert_eq!(
    "t-1",
    def_versioned_name(require_inst::<Assign>(then_blk, 0).def)
  );
  assert_eq!(
    "t-2",
    def_versioned_name(require_inst::<Assign>(else_blk, 0).def)
  );

  let phi = require_inst::<Join>(merge, 0);
  check_join(phi, "t-3", &["t-1", "t-2"]);

  let decl_y = require_inst::<Declare>(merge, 1);
  assert_eq!("y-0", def_versioned_name(decl_y.def));
}

// Source: `tests/ControlFlowGraph.test.cpp:323-335`
#[test]
fn control_flow_graph_call_expression_records_uses() {
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::def_versioned_name::def_versioned_name, records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local f = nil
        local x = 1
        local y = f(x)
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  // f and x are read on the RHS of `local y = f(x)`
  assert_eq!(
    "f-0",
    def_versioned_name(fixture.get_definition_at_pos(cfg, Position::new(3, 18)))
  );
  assert_eq!(
    "x-0",
    def_versioned_name(fixture.get_definition_at_pos(cfg, Position::new(3, 20)))
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:509-527`
#[test]
fn control_flow_graph_conjunction_emits_flow_per_side() {
  use ulua_analysis::records::{declare::Declare, refine::Refine};
  use ulua_unit_test::{
    functions::{
      check_refine::check_refine, def_versioned_name::def_versioned_name,
      require_inst::require_inst,
    },
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = nil
        local y = nil
        if x and y then
            local z = x
        end
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let then_blk = crate::block_at(cfg, 1);

  check_refine(
    require_inst::<Refine>(then_blk, 0),
    "x-1",
    "x-0",
    true,
    None,
    false,
  );
  check_refine(
    require_inst::<Refine>(then_blk, 1),
    "y-1",
    "y-0",
    true,
    None,
    false,
  );
  assert_eq!(
    "z-0",
    def_versioned_name(require_inst::<Declare>(then_blk, 2).def)
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:471-490`
#[test]
fn control_flow_graph_dump_renders_type_guard_as_a_call() {
  use ulua_analysis::functions::dump_cfg::dump_cfg;
  use ulua_unit_test::records::cfg_fixture::CfgFixture;

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = nil
        if typeof(x) == "string" then
            local y = x
        end
    "#,
  );
  let cfg = fixture.cfg();

  let dump = dump_cfg(cfg);

  // 回归 DumpCFG 的 dumpRefinement() 格式化缺陷：typeof 守卫必须渲染成调用式
  // `typeof(x-0) == "string"`，而不是畸形的 `x-0 typeof == "string"`。
  assert!(
    dump.contains(r#"typeof(x-0) == "string""#),
    "dumpCFG 产出了畸形的 refinement 文本：\n{dump}"
  );
  assert!(
    !dump.contains("x-0 typeof"),
    "dumpCFG 把变量排在了守卫之前：\n{dump}"
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:337-346`
#[test]
fn control_flow_graph_grouped_expression_records_use() {
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::def_versioned_name::def_versioned_name, records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = 1
        local y = (x)
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  assert_eq!(
    "x-0",
    def_versioned_name(fixture.get_definition_at_pos(cfg, Position::new(2, 19)))
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:430-452`
#[test]
fn control_flow_graph_if_falsy_single_branch() {
  use ulua_analysis::records::{join::Join, refine::Refine};
  use ulua_unit_test::{
    functions::{check_join::check_join, check_refine::check_refine, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = nil
        if not x then
            local y = x
        end
        local z = x
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let then_blk = crate::block_at(cfg, 1);
  let else_blk = crate::block_at(cfg, 2);
  let merge = crate::block_at(cfg, 3);

  check_refine(
    require_inst::<Refine>(then_blk, 0),
    "x-1",
    "x-0",
    false,
    None,
    false,
  );
  check_refine(
    require_inst::<Refine>(else_blk, 0),
    "x-2",
    "x-0",
    true,
    None,
    false,
  );

  let phi = require_inst::<Join>(merge, 0);
  check_join(phi, "x-3", &["x-1", "x-2"]);
}

// Source: `tests/ControlFlowGraph.test.cpp:401-428`
#[test]
fn control_flow_graph_if_truthy_both_branches() {
  use ulua_analysis::records::{declare::Declare, join::Join, refine::Refine};
  use ulua_unit_test::{
    functions::{
      check_join::check_join, check_refine::check_refine, def_versioned_name::def_versioned_name,
      require_inst::require_inst,
    },
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
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
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let then_blk = crate::block_at(cfg, 1);
  let else_blk = crate::block_at(cfg, 2);
  let merge = crate::block_at(cfg, 3);

  check_refine(
    require_inst::<Refine>(then_blk, 0),
    "x-1",
    "x-0",
    true,
    None,
    false,
  );
  assert_eq!(
    "y-0",
    def_versioned_name(require_inst::<Declare>(then_blk, 1).def)
  );

  check_refine(
    require_inst::<Refine>(else_blk, 0),
    "x-2",
    "x-0",
    false,
    None,
    false,
  );
  assert_eq!(
    "z-0",
    def_versioned_name(require_inst::<Declare>(else_blk, 1).def)
  );

  let phi = require_inst::<Join>(merge, 0);
  check_join(phi, "x-3", &["x-1", "x-2"]);
  assert_eq!(
    "y-0",
    def_versioned_name(require_inst::<Declare>(merge, 1).def)
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:219-238`
#[test]
fn control_flow_graph_multi_assignment() {
  use ulua_analysis::records::{assign::Assign, declare::Declare};
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::{def_versioned_name::def_versioned_name, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local a, b = 1, 2
        a, b = b, a
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);

  assert_eq!(
    "a-0",
    def_versioned_name(require_inst::<Declare>(entry, 0).def)
  );
  assert_eq!(
    "b-0",
    def_versioned_name(require_inst::<Declare>(entry, 1).def)
  );
  assert_eq!(
    "a-1",
    def_versioned_name(require_inst::<Assign>(entry, 2).def)
  );
  assert_eq!(
    "b-1",
    def_versioned_name(require_inst::<Assign>(entry, 3).def)
  );

  assert_eq!(
    "b-0",
    def_versioned_name(fixture.get_definition_at_pos(cfg, Position::new(2, 15)))
  );
  assert_eq!(
    "a-0",
    def_versioned_name(fixture.get_definition_at_pos(cfg, Position::new(2, 18)))
  );
}

// 缺口（cpp-gaps-r4 探针挂账，对照 `tests/ControlFlowGraph.test.cpp:348-377`）：
// `trivial_phi_if_else_unmodified` / `trivial_phi_while_loop_unmodified` 两例
// 不可移植——上游新增 `CFGBuilder::trimTrivialJoin`（cpp
// `Analysis/src/ControlFlowGraph.cpp:614-648`，`fillJoinOperands` 于 :610 调用，
// 将平凡 phi 置 `Dead` 并把 `cfg->forwards` 折到单一操作数），本端口
// `ulua-analysis/src/methods/cfg_builder.rs:219` 的 `trim_trivial_join` 为空桩
// （调用点在 `cfg_builder_fill_join_operands.rs:63`）。cpp-gaps-r4 实测
//（临时探针，源串与 Position 逐字照抄 cpp）：两例的读用分别解析到 `x-1`
//（cpp 期望 `x-0`），即平凡 join 未裁剪、读解析到 phi 自身的版本化 def。
// 属特性未移植，不用错误期望占位；`trim_trivial_join` 落地后连同本注记
// 一并补回两例（并撤本段挂账）。

// Source: `tests/ControlFlowGraph.test.cpp:379-395`
#[test]
fn control_flow_graph_nontrivial_phi_one_branch_modifies() {
  use ulua_analysis::records::join::Join;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::{
    functions::{
      check_join::check_join, def_versioned_name::def_versioned_name, require_inst::require_inst,
    },
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = 1
        if true then
            x = 2
        end
        local w = x
    "#,
  );
  let cfg = fixture.cfg();

  // x 只在 then 分支被改写（x-1），else 仍是 x-0，故 phi 非平凡。
  let merge = crate::block_at(cfg, 3);
  let phi = require_inst::<Join>(merge, 0);
  check_join(phi, "x-2", &["x-1", "x-0"]);

  assert_eq!(
    "x-2",
    def_versioned_name(fixture.get_definition_at_pos(cfg, Position::new(5, 18)))
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:203-217`
#[test]
fn control_flow_graph_reassignment_from_local() {
  use ulua_analysis::records::{assign::Assign, declare::Declare};
  use ulua_unit_test::{
    functions::{def_versioned_name::def_versioned_name, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = 1
        local y = 2
        x = y
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);

  assert_eq!(
    "x-0",
    def_versioned_name(require_inst::<Declare>(entry, 0).def)
  );
  assert_eq!(
    "y-0",
    def_versioned_name(require_inst::<Declare>(entry, 1).def)
  );
  assert_eq!(
    "x-1",
    def_versioned_name(require_inst::<Assign>(entry, 2).def)
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:187-201`
#[test]
fn control_flow_graph_simple_reassignment() {
  use ulua_analysis::records::{assign::Assign, declare::Declare};
  use ulua_unit_test::{
    functions::{def_versioned_name::def_versioned_name, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = 4
        x = 5
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);

  let decl = require_inst::<Declare>(entry, 0);
  assert_eq!("x-0", def_versioned_name(decl.def));

  let assign = require_inst::<Assign>(entry, 1);
  assert_eq!("x-1", def_versioned_name(assign.def));
}

// Source: `tests/ControlFlowGraph.test.cpp:156-169`
#[test]
fn control_flow_graph_single_local() {
  use ulua_analysis::{enums::block_kind::BlockKind, records::declare::Declare};
  use ulua_unit_test::{
    functions::{def_versioned_name::def_versioned_name, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = 4
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);
  assert_eq!(BlockKind::Entry, entry.kind);
  assert_eq!(1, entry.get_instructions().len());

  let decl = require_inst::<Declare>(entry, 0);
  assert_eq!("x-0", def_versioned_name(decl.def));
}

// 缺口（未移植）：`tests/ControlFlowGraph.test.cpp:348-363` / `365-377`
// `trivial_phi_if_else_unmodified` / `trivial_phi_while_loop_unmodified` 依赖 cpp
// `Analysis/src/ControlFlowGraph.cpp` 的 Cytron 式 gated-SSA 重写：
// `trimTrivialJoin`（含 `Instruction::Dead` 原地改写 + `cfg->forwards` 转发）、
// `recordUses`/`usingInstructions`（供平凡 phi 递归裁剪）、
// `fillJoinOperands(block, instr, j) -> DefId`、`emitJoin -> (InstrId, Join*)`、
// `seal` 中重判 inst 仍为 Join、`getUseDef`/`getLhsDef` 走 `resolve()`。
// Rust 侧 `cfg_builder_trim_trivial_join` 仍是空壳、`ControlFlowGraph` 无
// `forwards`/`resolve`、`Instruction` 无 `Dead` 变体，故平凡 phi 不会被折叠，
// 读用解析到 phi 本身（x-1）而非 x-0。属较大算法同步，暂不用错误期望值占位。

// Source: `tests/ControlFlowGraph.test.cpp:171-185`
#[test]
fn control_flow_graph_two_locals() {
  use ulua_analysis::records::declare::Declare;
  use ulua_unit_test::{
    functions::{def_versioned_name::def_versioned_name, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = 4
        local y = 5
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(1, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);

  let decl_x = require_inst::<Declare>(entry, 0);
  assert_eq!("x-0", def_versioned_name(decl_x.def));

  let decl_y = require_inst::<Declare>(entry, 1);
  assert_eq!("y-0", def_versioned_name(decl_y.def));
}

// Source: `tests/ControlFlowGraph.test.cpp:492-507`
#[test]
fn control_flow_graph_type_guard_inequality_flips_sense() {
  use ulua_analysis::records::refine::Refine;
  use ulua_unit_test::{
    functions::{check_refine::check_refine, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = nil
        if type(x) ~= "string" then
            local y = x
        end
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let then_blk = crate::block_at(cfg, 1);
  let else_blk = crate::block_at(cfg, 2);

  check_refine(
    require_inst::<Refine>(then_blk, 0),
    "x-1",
    "x-0",
    false,
    Some("string"),
    false,
  );
  check_refine(
    require_inst::<Refine>(else_blk, 0),
    "x-2",
    "x-0",
    true,
    Some("string"),
    false,
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:454-469`
#[test]
fn control_flow_graph_typeof_guard_emits_type_proposition() {
  use ulua_analysis::records::refine::Refine;
  use ulua_unit_test::{
    functions::{check_refine::check_refine, require_inst::require_inst},
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = nil
        if typeof(x) == "string" then
            local y = x
        end
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let then_blk = crate::block_at(cfg, 1);
  let else_blk = crate::block_at(cfg, 2);

  check_refine(
    require_inst::<Refine>(then_blk, 0),
    "x-1",
    "x-0",
    true,
    Some("string"),
    true,
  );
  check_refine(
    require_inst::<Refine>(else_blk, 0),
    "x-2",
    "x-0",
    false,
    Some("string"),
    true,
  );
}

// Source: `tests/ControlFlowGraph.test.cpp:279-321`
#[test]
fn control_flow_graph_while_loop() {
  use ulua_analysis::{
    enums::block_kind::BlockKind,
    records::{assign::Assign, declare::Declare, join::Join, refine::Refine},
  };
  use ulua_unit_test::{
    functions::{
      check_join::check_join, check_predecessors::check_predecessors, check_refine::check_refine,
      check_successors::check_successors, def_versioned_name::def_versioned_name,
      require_inst::require_inst,
    },
    records::cfg_fixture::CfgFixture,
  };

  let mut fixture = CfgFixture::default();
  fixture.build(
    r#"
        local x = nil
        while not x do
            x = 5
        end
        local y = x
    "#,
  );
  let cfg = fixture.cfg();

  assert_eq!(4, cfg.blocks.len());

  let entry = crate::block_at(cfg, 0);
  let header = crate::block_at(cfg, 1);
  let body = crate::block_at(cfg, 2);
  let exit = crate::block_at(cfg, 3);

  assert_eq!(BlockKind::Entry, entry.kind);
  assert_eq!(BlockKind::Condition, header.kind);
  assert_eq!(BlockKind::Linear, body.kind);
  assert_eq!(BlockKind::Linear, exit.kind);

  check_successors(cfg, entry, &[1]);
  check_successors(cfg, header, &[2, 3]);
  check_successors(cfg, body, &[1]);
  check_predecessors(cfg, header, &[0, 2]);

  assert_eq!(
    "x-0",
    def_versioned_name(require_inst::<Declare>(entry, 0).def)
  );

  let phi = require_inst::<Join>(header, 0);
  check_join(phi, "x-1", &["x-0", "x-3"]);

  let body_refine = require_inst::<Refine>(body, 0);
  check_refine(body_refine, "x-2", "x-1", false, None, false);
  assert_eq!(
    "x-3",
    def_versioned_name(require_inst::<Assign>(body, 1).def)
  );

  let exit_refine = require_inst::<Refine>(exit, 0);
  check_refine(exit_refine, "x-4", "x-1", true, None, false);
  assert_eq!(
    "y-0",
    def_versioned_name(require_inst::<Declare>(exit, 1).def)
  );
}

// ---- TEST_SUITE("CFGTypeCheckTest")：C++ 用 `ScopedFastFlag{DebugLuauCFG, true}`
// 打开基于 CFG 的类型检查；本仓库 Rust 侧尚未接线该开关（见文件末尾的移植说明）。

// Source: `tests/ControlFlowGraph.test.cpp:533-547`
cfg_typecheck_case!(
  control_flow_graph_is_truthy_constraint,
  src: r#"
local v : string?
if v then
    local s = v
else
    local s = v
end
"#,
  [(3, 14, "string"), (5, 14, "nil")]
);

// Source: `tests/ControlFlowGraph.test.cpp:549-563`
cfg_typecheck_case!(
  control_flow_graph_invert_is_truthy_constraint,
  src: r#"
local v : string?
if not v then
    local s = v
else
    local s = v
end
"#,
  [(3, 14, "nil"), (5, 14, "string")]
);

// Source: `tests/ControlFlowGraph.test.cpp:565-579`
cfg_typecheck_case!(
  control_flow_graph_parenthesized_expressions_are_followed_through,
  src: r#"
local v : string?
if (not v) then
    local s = v
else
    local s = v
end
"#,
  [(3, 14, "nil"), (5, 14, "string")]
);

// Source: `tests/ControlFlowGraph.test.cpp:581-600`
cfg_typecheck_case!(
  control_flow_graph_and_constraint,
  src: r#"
local a : string?
local b : number?
if a and b then
    local x = a
    local y = b
else
    local x = a
    local y = b
end
"#,
  [(4, 14, "string"), (5, 14, "number"), (7, 14, "string?"), (8, 14, "number?")]
);

// Source: `tests/ControlFlowGraph.test.cpp:602-621`
cfg_typecheck_case!(
  control_flow_graph_not_and_constraint,
  src: r#"
local a : string?
local b : number?
if not (a and b) then
    local x = a
    local y = b
else
    local x = a
    local y = b
end
"#,
  [(4, 14, "string?"), (5, 14, "number?"), (7, 14, "string"), (8, 14, "number")]
);

// Source: `tests/ControlFlowGraph.test.cpp:623-634`
cfg_typecheck_case!(
  control_flow_graph_is_truthy_while_loop,
  src: r#"
local v : string?
while v do
    local s = v
end
"#,
  [(3, 14, "string")]
);

// Source: `tests/ControlFlowGraph.test.cpp:636-647`
cfg_typecheck_case!(
  control_flow_graph_invert_is_truthy_while_loop,
  src: r#"
local v : string?
while not v do
    local s = v
end
"#,
  [(3, 14, "nil")]
);

// Source: `tests/ControlFlowGraph.test.cpp:649-658`
#[test]
fn control_flow_graph_assert_truthy() {
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let (_fixture, result) = bs_check!(
    r#"
local foo : string?
assert(foo)
local bar : string = foo
"#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/ControlFlowGraph.test.cpp:660-673`
#[test]
fn control_flow_graph_assert_truthy_then_type_guard() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local a : (number | string)?
assert(a)
local b = a
assert(type(a) == "number")
local c = a
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number | string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 10))
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 10))
    )
  );
}

// 缺口（未接线）：`tests/ControlFlowGraph.test.cpp:676-689` / `691-706`
// `interesting_refinement` / `while_back_edge` 位于 cpp 注释
// "stuff we aren't able to infer with the existing type inference system" 段内，
// 两者都靠 `ScopedFastFlag{FFlag::DebugLuauCFG, true}` 走 CFG 驱动的类型检查：
// `Frontend.cpp:2343,2369`（构造 ControlFlowGraph 并传入 ConstraintGenerator）、
// `ConstraintGenerator.cpp:431,467,578,594,1438,1473,1718,1946,2031,3265,3820`
// 的 `DebugLuauCFG` 分支（含 `Refine`/`Join` 到精化类型的读取）。
// Rust 侧 `fflag::DebugLuauCFG` 未定义，`ConstraintGenerator` 无 CFG 字段，
// `CfgBuilder::make_cfg` 目前仅被测试 fixture 调用，故这两个期望值无法达成：
// 旧求解器分别给出 `number?`（连带 `x + 4` 报 UninhabitedTypeFunction）与
// `number`。属未移植特性，不用错误期望值占位。
