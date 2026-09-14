use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn delete_child_statements_ast_stat_block(&mut self, b: *mut AstStatBlock) -> bool {
    let mut result = false;
    let mut chunk_count: usize = 2;

    loop {
      let (work_done, new_chunk_count) =
        self.delete_child_statements_ast_stat_block_usize(b, chunk_count);
      if work_done {
        result = true;
        chunk_count = new_chunk_count;
        continue;
      } else {
        break;
      }
    }

    result
  }
}
