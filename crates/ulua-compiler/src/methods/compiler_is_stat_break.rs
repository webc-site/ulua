use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
  },
  rtti::ast_node_as,
};

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn is_stat_break(&mut self, node: *mut AstStat) -> bool {
    unsafe {
      if node.is_null() {
        return false;
      }

      let stat_block = ast_node_as::<AstStatBlock>(node as *mut AstNode);
      if !stat_block.is_null() {
        let stat_block = &*stat_block;

        stat_block.body.size == 1
          && !ast_node_as::<AstStatBreak>(*stat_block.body.data.add(0) as *mut AstNode).is_null()
      } else {
        !ast_node_as::<AstStatBreak>(node as *mut AstNode).is_null()
      }
    }
  }
}
