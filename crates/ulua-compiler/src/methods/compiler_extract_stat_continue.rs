use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_continue::AstStatContinue,
  },
  rtti::ast_node_as,
};

use crate::records::compiler::Compiler;

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn extract_stat_continue(&mut self, block: *mut AstStatBlock) -> *mut AstStatContinue {
    unsafe {
      let body = &(*block).body;
      if body.size == 1 {
        let stat_ptr = body.data.add(0);
        let stat_node = &mut **stat_ptr as *mut AstStat as *mut AstNode;

        ast_node_as::<AstStatContinue>(stat_node)
      } else {
        null_mut()
      }
    }
  }
}
