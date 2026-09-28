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
  pub root: *mut AstStatBlock,
  /// `build` 布线的 CFG 指针（指向 `cfg_allocator` arena），经 `CfgFixture::cfg`
  /// 安全读取；与 `root` 同理需 `Debug` 可打印的裸指针形态。
  pub cfg_ptr: *mut ControlFlowGraph,
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
      root: null_mut(),
      cfg_ptr: null_mut(),
      freeze_arena: ScopedFastFlag::new(&fflag::DebugLuauFreezeArena, true),
    }
  }
}

impl CfgFixture {
  pub fn new() -> Self {
    Self::default()
  }
}
