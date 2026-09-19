use crate::records::{
  ast_stat_block::AstStatBlock,
  printer::{IntoNodePtr, Printer},
  writer::Writer,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_block_ast_stat_block<B: IntoNodePtr<AstStatBlock>>(&mut self, block: B) {
    // SAFETY: block 指向 arena 中存活的 AstStatBlock
    let block = unsafe { &mut *block.into_node_ptr() };
    for &stat in block.body.as_slice() {
      if !stat.is_null() {
        self.visualize_ast_stat(unsafe { &mut *stat });
      }
    }
    self.advance(block.base.base.location.end);
  }
}
