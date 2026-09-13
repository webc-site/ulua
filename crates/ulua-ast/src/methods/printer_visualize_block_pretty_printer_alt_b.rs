use crate::{
  records::{ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, printer::Printer},
  rtti::ast_node_as,
};

pub trait IntoAstStatMut {
  fn into_ast_stat_mut(self) -> *mut AstStat;
}

impl IntoAstStatMut for *mut AstStat {
  fn into_ast_stat_mut(self) -> *mut AstStat {
    self
  }
}

impl IntoAstStatMut for &mut AstStat {
  fn into_ast_stat_mut(self) -> *mut AstStat {
    self
  }
}

impl<'a> Printer<'a> {
  pub fn visualize_block_ast_stat<S: IntoAstStatMut>(&mut self, stat: S) {
    let stat = stat.into_ast_stat_mut();
    let block = unsafe { ast_node_as::<AstStatBlock>(stat as *mut AstNode) };
    if !block.is_null() {
      let block_ref = unsafe { &mut *block };
      self.visualize_block_ast_stat_block(block_ref);
      return;
    }

    ulua_common::LUAU_ASSERT!(false);
  }
}
