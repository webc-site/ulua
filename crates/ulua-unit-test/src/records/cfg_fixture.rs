//! Source: `tests/ControlFlowGraph.test.cpp`

use core::ptr::null_mut;

use ulua_analysis::records::{cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph};
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
};
use ulua_common::fflag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
pub struct CfgFixture {
  pub allocator: Allocator,
  pub names: AstNameTable,
  pub cfg_allocator: CfgAllocator,
  /// cpp `CfgFixture` 的 `root` 成员（`root = parse(code)`，`ControlFlowGraph.test.cpp`）。
  ///
  /// DELIBERATE DEVIATION / 保留理由（review.md §2 缺席态的暂缓项）：目标形态是
  /// `Option<NonNull<AstStatBlock>>`（`None` = `build()` 之前无根块）。实测把本字段、
  /// `cfg_ptr` 与 `DataFlowGraphFixture::module` 一起换型后，`cargo check -p
  /// ulua-unit-test --all-targets` 的错误落在 **9 个非本轮清单文件 / 17 处**：
  /// `methods::cfg_fixture_build`（5：`self.root = from_ref(self.parse(code))
  /// .cast_mut()` 写入、`make_cfg` 入参、`cfg()` 的 `is_null` 断言 + `&*self.cfg_ptr`）、
  /// `methods::cfg_fixture_get_definition_at_pos`（2：`assert!(!self.root.is_null())` +
  /// `&*self.root`）、`methods::data_flow_graph_fixture_{dfg,get_def,get_local_def}`
  /// （4）、以及跨 crate 的签名面 `functions::query`（2，`AstNodePtr` 只为 `*mut T`
  /// 实现）、`ulua_ast::rtti`（2）、`ulua_analysis::methods::cfg_builder`
  /// （`CfgBuilder::make_cfg(Handle<CfgAllocator>, &AstStatBlock) ->
  /// ControlFlowGraph`，本字段作为 `build` 内的首层 `&*self.root` 解引用点）、
  /// `ulua_analysis::methods::data_flow_graph_builder`
  /// （`DataFlowGraphBuilder::build` 首参已引用化为 `&AstStatBlock`）。这些文件本轮禁改，只动声明端会留下
  /// 不可编译的半截迁移；句柄化需与 `make_cfg`/`build`/`AstNodePtr` 同批改签名。
  pub root: *mut AstStatBlock,
  /// `build` 交付并归夹具持有的 `ControlFlowGraph`（cpp 测试里
  /// `auto cfg = build(...)` 的 `unique_ptr` 所有权在本 Rust 端口为按值 move：
  /// `make_cfg` 返回所有权值，`build` 显式 `Some(cfg)` 入位）。CFG 自身的
  /// 节点内存仍存活于 `cfg_allocator` arena，`Option` 析构只释放 CFG 的
  /// `Vec`/`DenseHashMap` 容器，不误触 arena（`ControlFlowGraph` 无自定义
  /// Drop，§2 所有权转手红线）。
  pub cfg: Option<ControlFlowGraph>,
  pub freeze_arena: ScopedFastFlag,
}

impl Default for CfgFixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);

    Self {
      allocator,
      names,
      cfg_allocator: CfgAllocator::default(),
      // 两个 arena 句柄槽位的「尚未 build」形态：`root` 的 `None` 语义因消费端
      // 与下游签名越出本轮清单仍用 nullptr 哨兵（字段文档见理由），CFG 槽位则
      // 已是所有权 `Option`，缺席态直接 `None`（review.md §2）。
      root: null_mut(),
      cfg: None,
      freeze_arena: ScopedFastFlag::new(&fflag::DebugLuauFreezeArena, true),
    }
  }
}

impl CfgFixture {
  pub fn new() -> Self {
    Self::default()
  }
}
