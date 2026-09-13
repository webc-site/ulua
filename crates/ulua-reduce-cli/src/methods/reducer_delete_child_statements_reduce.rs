use core::{cmp, mem::swap};

use ulua_ast::records::{ast_array::AstArray, ast_stat::AstStat, ast_stat_block::AstStatBlock};

use crate::{enums::test_result::TestResult, records::reducer::Reducer};

impl Reducer {
  pub fn delete_child_statements_ast_stat_block_usize(
    &mut self,
    block: *mut AstStatBlock,
    chunk_count: usize,
  ) -> (bool, usize) {
    unsafe {
      if (*block).body.size == 0 {
        return (false, chunk_count);
      }

      let mut current_chunk_count = chunk_count;
      let block_size = (*block).body.size;

      loop {
        let permutations = self.generate_spans(block_size, current_chunk_count);
        for (span1, span2) in permutations {
          let temp_statements = self.pruned_span(block, span1, span2);
          let _backup_body = (*block).body;

          let mut new_body = AstArray {
            data: temp_statements.as_ptr() as *mut *mut AstStat,
            size: temp_statements.len(),
          };

          swap(&mut (*block).body, &mut new_body);

          let result = self.run();
          if result == TestResult::BugFound {
            // The bug still reproduces without the statements we've culled. Commit.
            (*block).body.data = self.reallocate_statements(&temp_statements);
            (*block).body.size = temp_statements.len();
            return (true, cmp::max(2, current_chunk_count.saturating_sub(1)));
          } else {
            swap(&mut (*block).body, &mut new_body);
          }
        }

        current_chunk_count *= 2;
        if current_chunk_count > block_size {
          break;
        }
      }

      (false, block_size)
    }
  }
}
