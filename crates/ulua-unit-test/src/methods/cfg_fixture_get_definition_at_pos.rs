use ulua_analysis::{
  functions::find_node_at_position_ast_query::find_node_at_position_ast_stat_block_position,
  records::control_flow_graph::ControlFlowGraph, type_aliases::def_id_control_flow_graph::DefId,
};
use ulua_ast::records::position::Position;

use crate::records::cfg_fixture::CfgFixture;

impl CfgFixture {
  pub fn get_definition_at_pos(&self, cfg: &ControlFlowGraph, pos: Position) -> DefId {
    assert!(!self.root.is_null());

    let node = find_node_at_position_ast_stat_block_position(
      unsafe {
        // Safety: root 行 11 断言非空：build() 存入的 fixture allocator arena 存活 AstStatBlock（arena 块地址不动、比查询长寿），&* 物化只读借用，find_node_at_position 只读遍历。
        &*self.root
      },
      pos,
    );
    assert!(!node.is_null());

    // Safety: node 行 15 断言非空：find_node_at_position 返回的存活 AST 节点指针
    // （arena 内、查询链保活），as_expr() 读 class_index 判型取表达式视图，只读。
    let expr = unsafe { (*node).as_expr() }.expect("expr family");
    let expr = expr.as_ptr();

    // use_defs 的值即构建期 register_sym_def 发放的 SymDef 句柄，按值拷出。
    let def = cfg.use_defs.find(&expr);
    assert!(def.is_some());
    *def.unwrap()
  }
}
