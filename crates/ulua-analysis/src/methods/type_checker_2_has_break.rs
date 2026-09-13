use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_if::AstStatIf,
  },
  rtti::{ast_node_as, ast_node_is},
};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn type_checker_2_has_break(&mut self, node: *mut AstStat) -> bool {
    unsafe {
      let block = ast_node_as::<AstStatBlock>(node as *mut AstNode);
      if !block.is_null() {
        let body = (*block).body;
        for &stat in body.as_slice() {
          if self.type_checker_2_has_break(stat) {
            return true;
          }
        }
        return false;
      }

      if ast_node_is::<AstStatBreak>(&(*node).base) {
        return true;
      }

      let if_stat = ast_node_as::<AstStatIf>(node as *mut AstNode);
      if !if_stat.is_null() {
        if self.type_checker_2_has_break((*if_stat).thenbody as *mut AstStat) {
          return true;
        }

        if !(*if_stat).elsebody.is_null() && self.type_checker_2_has_break((*if_stat).elsebody) {
          return true;
        }

        return false;
      }

      false
    }
  }
}
