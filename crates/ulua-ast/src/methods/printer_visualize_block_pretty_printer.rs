use crate::records::{ast_stat_block::AstStatBlock, printer::Printer};

pub trait IntoAstStatBlockMut {
  fn into_ast_stat_block_mut(self) -> *mut AstStatBlock;
}

impl IntoAstStatBlockMut for *mut AstStatBlock {
  fn into_ast_stat_block_mut(self) -> *mut AstStatBlock {
    self
  }
}

impl IntoAstStatBlockMut for &mut AstStatBlock {
  fn into_ast_stat_block_mut(self) -> *mut AstStatBlock {
    self
  }
}

impl<'a> Printer<'a> {
  pub fn visualize_block_ast_stat_block<B: IntoAstStatBlockMut>(&mut self, block: B) {
    let block = unsafe { &mut *block.into_ast_stat_block_mut() };
    for &stat in block.body.as_slice() {
      if !stat.is_null() {
        self.visualize_ast_stat(unsafe { &mut *stat });
      }
    }
    self.advance(block.base.base.location.end);
  }
}
