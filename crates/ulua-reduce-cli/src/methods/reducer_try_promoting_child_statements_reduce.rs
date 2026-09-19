use alloc::vec::Vec;
use core::mem::swap;

use ulua_ast::records::{ast_array::AstArray, ast_stat_block::AstStatBlock};

use crate::{enums::test_result::TestResult, records::reducer::Reducer};

impl Reducer {
  pub fn try_promoting_child_statements_ast_stat_block_usize(
    &mut self,
    b: *mut AstStatBlock,
    index: usize,
  ) -> bool {
    // SAFETY: `b` 是 walk 队列里 parser 产出的存活 AstStatBlock;
    // 调用方保证 `index < b.body.size`
    let body_slice = unsafe { (*b).body.as_slice() };
    let mut temp_stats = Vec::from(body_slice);

    // 取出第 index 条语句，原位换入其内部嵌套的语句
    let removed = temp_stats[index];
    temp_stats.remove(index);

    let nested_stats = self.get_nested_stats(removed);
    temp_stats.splice(index..index, nested_stats);

    let mut temp_array = AstArray {
      data: temp_stats.as_mut_ptr(),
      size: temp_stats.len(),
    };

    // 试替换后跑一遍用例；swap 提交/回滚保证 body 指针有效
    swap(unsafe { &mut (*b).body }, &mut temp_array);

    if self.run() == TestResult::BugFound {
      let committed = self.reallocate_statements(&temp_stats);
      unsafe {
        (*b).body.data = committed;
        (*b).body.size = temp_stats.len();
      }
      true
    } else {
      swap(unsafe { &mut (*b).body }, &mut temp_array);
      false
    }
  }
}
