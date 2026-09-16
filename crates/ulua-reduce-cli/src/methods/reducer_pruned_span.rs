use alloc::vec::Vec;

use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock};

use crate::{records::reducer::Reducer, type_aliases::span::Span};

impl Reducer {
  pub fn pruned_span(
    &self,
    block: *mut AstStatBlock,
    span1: Span,
    span2: Span,
  ) -> Vec<*mut AstStat> {
    // SAFETY: `block` 来自 parser 产出的存活 AST; span 索引由
    // `generate_spans` 生成, 恒在 `block.body.size` 界内
    let body = unsafe {
      if block.is_null() {
        return Vec::new();
      }
      (*block).body.as_slice()
    };

    let mut result: Vec<*mut AstStat> =
      Vec::with_capacity((span1.1 - span1.0) + (span2.1 - span2.0));
    // 两个半开区间整段拷贝（cpp 逐元素 push_back 的等价批量形式）
    result.extend_from_slice(&body[span1.0..span1.1]);
    result.extend_from_slice(&body[span2.0..span2.1]);
    result
  }
}
