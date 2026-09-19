use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::reducer::Reducer;

impl Reducer {
  pub fn try_promoting_child_statements_ast_stat_block(&mut self, b: *mut AstStatBlock) -> bool {
    let mut i: usize = 0;
    // body.size 每轮重读：提交晋升后 body 变长，新语句还要继续尝试
    // SAFETY: `b` 是 walk 队列里 parser 产出的存活 AstStatBlock
    while i < unsafe { (*b).body.size } {
      let promoted = self.try_promoting_child_statements_ast_stat_block_usize(b, i);
      if !promoted {
        i += 1;
      }
    }

    false
  }
}
