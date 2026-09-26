use core::cmp::max;

use crate::{
  functions::{generate_spans::generate_spans, pruned_span::pruned_span},
  records::{node::Block, reducer::Reducer},
};

impl Reducer {
  /// cpp `Reducer::deleteChildStatements(block, chunkCount)`：以 ddmin 式
  /// 区间组合尝试删去 `block` body 中的语句。返回「是否删成了任何语句」
  /// 与下一轮的 chunk 数。试删/提交/回滚统一走 `Reducer::try_body`。
  fn delete_with_chunks(&mut self, block: &mut Block, chunk_count: usize) -> (bool, usize) {
    let block_size = block.get().body.len();
    if block_size == 0 {
      return (false, chunk_count);
    }

    let mut current_chunk_count = chunk_count;

    loop {
      for (span1, span2) in generate_spans(block_size, current_chunk_count) {
        let temp_statements = pruned_span(block.get(), span1, span2);

        if self.try_body(block, &temp_statements) {
          // 删掉这些语句后 bug 仍可复现：提交。
          return (true, max(2, current_chunk_count.saturating_sub(1)));
        }
        // 否则 try_body 已回滚：这些语句对复现 bug 是关键。
      }

      current_chunk_count *= 2;
      if current_chunk_count > block_size {
        break;
      }
    }

    (false, block_size)
  }

  /// cpp 重载 `deleteChildStatements(AstStatBlock*)`：反复减半分块直到无进展；
  /// 只要有任何一轮删成即返回 true。
  pub(crate) fn delete_child_statements(&mut self, block: &mut Block) -> bool {
    let mut chunk_count: usize = 2;
    let mut result = false;

    loop {
      let (work_done, new_chunk_count) = self.delete_with_chunks(block, chunk_count);
      if !work_done {
        return result;
      }
      result = true;
      chunk_count = new_chunk_count;
    }
  }
}
