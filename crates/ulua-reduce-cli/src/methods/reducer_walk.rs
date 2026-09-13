use std::collections::VecDeque;

use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::ast_stat_visit};

use crate::records::{enqueuer::Enqueuer, reducer::Reducer};

impl Reducer {
  pub fn walk(&mut self, block: *mut AstStatBlock) {
    let mut queue: VecDeque<*mut AstStatBlock> = VecDeque::new();
    let mut enqueuer = Enqueuer::new(&mut queue as *mut VecDeque<*mut AstStatBlock>);

    queue.push_back(block);

    while let Some(b) = queue.pop_front() {
      loop {
        let mut result = self.delete_child_statements_ast_stat_block(b);
        result |= self.try_promoting_child_statements_ast_stat_block(b);

        if !result {
          break;
        }
      }

      unsafe {
        for stat in (*b).body.as_slice() {
          ast_stat_visit(*stat, &mut enqueuer);
        }
      }
    }
  }
}
