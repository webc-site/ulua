//! Source: `tests/ControlFlowGraph.test.cpp`

use ulua_analysis::records::{
  arena_handle::Handle, cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph,
};
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
  /// cpp `CfgFixture` 的 `root` 成员（`root = parse(code)`，
  /// `ControlFlowGraph.test.cpp:113`）：`None` = `build()` 之前无根块（cpp
  /// `nullptr` 初值的 Rust 缺席态），`Some` 为 arena 属主 `allocator` 保活的
  /// 根块别名句柄（`Handle` 模块契约：不拥有、不释放，地址随 bump 块稳定）。
  pub root: Option<Handle<AstStatBlock>>,
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
      // 「尚未 build」缺席态统一用 `Option::None` 表达（review.md §2，
      // cpp `root = nullptr` 成员初值与 unique_ptr 空回位的 Rust 形态）。
      root: None,
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
