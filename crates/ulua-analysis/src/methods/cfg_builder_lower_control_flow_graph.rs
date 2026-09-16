use ulua_ast::records::{
  ast_node::AstNode, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
  ast_stat_block::AstStatBlock, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
  ast_stat_while::AstStatWhile,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::cfg_builder::CfgBuilder;
impl CfgBuilder {
  pub fn lower_ast_stat(&mut self, statement: *mut AstStat) {
    unsafe {
      let p = statement as *mut AstNode;

      if (*p).is::<AstStatBlock>() {
        self.lower_ast_stat_block(statement as *mut AstStatBlock);
      } else if (*p).is::<AstStatLocal>() {
        self.lower_ast_stat_local(statement as *mut AstStatLocal);
      } else if (*p).is::<AstStatAssign>() {
        self.lower_ast_stat_assign(statement as *mut AstStatAssign);
      } else if (*p).is::<AstStatIf>() {
        self.lower_ast_stat_if(statement as *mut AstStatIf);
      } else if (*p).is::<AstStatWhile>() {
        self.lower_ast_stat_while(statement as *mut AstStatWhile);
      } else {
        LUAU_ASSERT!(false);
      }
    }
  }
}
