use core::ptr::null_mut;

use ulua_analysis::records::{
  data_flow_graph::DataFlowGraph, def_arena::DefArena,
  internal_error_reporter::InternalErrorReporter, refinement_key_arena::RefinementKeyArena,
};
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
};
use ulua_common::FFlag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
#[repr(C)]
pub struct DataFlowGraphFixture {
  pub dcr: ScopedFastFlag,
  pub def_arena: DefArena,
  pub key_arena: RefinementKeyArena,
  pub handle: InternalErrorReporter,
  pub allocator: Allocator,
  pub names: AstNameTable,
  pub module: *mut AstStatBlock,
  pub graph: Option<DataFlowGraph>,
}

impl Default for DataFlowGraphFixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);

    Self {
      dcr: ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
      def_arena: DefArena::default(),
      key_arena: RefinementKeyArena::default(),
      handle: InternalErrorReporter::default(),
      allocator,
      names,
      module: null_mut(),
      graph: None,
    }
  }
}

impl DataFlowGraphFixture {
  pub fn new() -> Self {
    Self::default()
  }
}
