use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn try_promoting_child_statements_ast_stat_block(&mut self, b: *mut AstStatBlock) -> bool {
    let mut i: usize = 0;
    unsafe {
      while i < (*b).body.size {
        let promoted = self.try_promoting_child_statements_ast_stat_block_usize(b, i);
        if !promoted {
          i += 1;
        }
      }
    }

    false
  }
}
