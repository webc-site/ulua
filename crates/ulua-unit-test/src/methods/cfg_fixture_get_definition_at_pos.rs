use ulua_analysis::{
  functions::find_node_at_position_ast_query_alt_b::find_node_at_position_ast_stat_block_position,
  records::{control_flow_graph::ControlFlowGraph, sym_def::SymDef},
};
use ulua_ast::records::position::Position;

use crate::records::cfg_fixture::CfgFixture;

impl CfgFixture {
  pub fn get_definition_at_pos(&self, cfg: &ControlFlowGraph, pos: Position) -> *mut SymDef {
    assert!(!self.root.is_null());

    let node = find_node_at_position_ast_stat_block_position(unsafe { &*self.root }, pos);
    assert!(!node.is_null());

    let expr = unsafe { (*node).as_expr() };
    assert!(!expr.is_null());

    let def = cfg.use_defs.find(&expr);
    assert!(def.is_some());
    *def.unwrap()
  }
}
