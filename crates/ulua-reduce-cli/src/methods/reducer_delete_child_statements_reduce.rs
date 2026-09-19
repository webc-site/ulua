use core::{cmp::max, mem::swap};

use ulua_ast::records::{ast_array::AstArray, ast_stat::AstStat, ast_stat_block::AstStatBlock};

use crate::{enums::test_result::TestResult, records::reducer::Reducer};

impl Reducer {
  pub fn delete_child_statements_ast_stat_block_usize(
    &mut self,
    block: *mut AstStatBlock,
    chunk_count: usize,
  ) -> (bool, usize) {
    // SAFETY: `block` 是 walk 队列里 parser 产出的存活 AstStatBlock
    let block_size = unsafe { (*block).body.size };
    if block_size == 0 {
      return (false, chunk_count);
    }

    let mut current_chunk_count = chunk_count;

    loop {
      let permutations = self.generate_spans(block_size, current_chunk_count);
      for (span1, span2) in permutations {
        let temp_statements = self.pruned_span(block, span1, span2);

        let mut new_body = AstArray {
          data: temp_statements.as_ptr() as *mut *mut AstStat,
          size: temp_statements.len(),
        };

        // 试删后跑一遍用例；swap 提交/回滚保证 body 指针始终指向 allocator 内存
        swap(unsafe { &mut (*block).body }, &mut new_body);

        if self.run() == TestResult::BugFound {
          // The bug still reproduces without the statements we've culled. Commit.
          let committed = self.reallocate_statements(&temp_statements);
          unsafe {
            (*block).body.data = committed;
            (*block).body.size = temp_statements.len();
          }
          return (true, max(2, current_chunk_count.saturating_sub(1)));
        }

        // The statements we've culled are critical for the reproduction of the bug.
        swap(unsafe { &mut (*block).body }, &mut new_body);
      }

      current_chunk_count *= 2;
      if current_chunk_count > block_size {
        break;
      }
    }

    (false, block_size)
  }
}
