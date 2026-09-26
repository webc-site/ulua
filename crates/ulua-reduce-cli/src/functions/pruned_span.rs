use alloc::vec::Vec;

use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::{records::node::Stat, type_aliases::span::Span};

/// 取 `block.body` 落在 `span1`、`span2` 两个半开区间内的语句。
/// 只做切片读取，不依赖 `Reducer` 状态，故为自由函数且接收共享引用
/// （上游 `Reducer::prunedSpan`，`CLI/src/Reduce.cpp:287-298`）。
/// 返回 arena 句柄（坐标值），无任何解引用。
pub fn pruned_span(block: &AstStatBlock, span1: Span, span2: Span) -> Vec<Stat> {
  let body = block.body.as_slice();

  let mut result: Vec<Stat> = Vec::with_capacity(span1.1 - span1.0 + (span2.1 - span2.0));
  // 两个半开区间整段拷贝（cpp 逐元素 push_back 的等价批量形式）
  result.extend(
    body[span1.0..span1.1]
      .iter()
      .map(|stat| Stat::from_ref(stat.get())),
  );
  result.extend(
    body[span2.0..span2.1]
      .iter()
      .map(|stat| Stat::from_ref(stat.get())),
  );
  result
}
