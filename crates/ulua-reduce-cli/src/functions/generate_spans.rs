use alloc::vec::Vec;
use core::cmp::{max, min};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::span::Span;

/// 生成 `chunks` 个近似等长的半开区间对，覆盖 `[0, size)`，并同时给出反向组合。
/// 纯切片算术，不依赖 `Reducer` 状态，故为自由函数（上游 `Reducer::generateSpans`，
/// `CLI/src/Reduce.cpp:246-283`）。
pub fn generate_spans(size: usize, chunks: usize) -> Vec<(Span, Span)> {
  if size <= 1 {
    return Vec::new();
  }

  LUAU_ASSERT!(chunks > 0);
  let chunk_length = max(1, size / chunks);

  let mut result: Vec<(Span, Span)> = Vec::new();

  // 空对空 (0,0)-(size,size) 的退化组合不入列
  let mut append = |a: Span, b: Span| {
    if !(a.0 == a.1 && b.0 == b.1) {
      result.push((a, b));
    }
  };

  // 前缀保留、后缀挖除：(0,i) + (end,size)
  for i in (0..size).step_by(chunk_length) {
    let end = min(i + chunk_length, size);
    append((0, i), (end, size));
  }

  // 区间挖除本身：(i,end) + 空
  for i in (0..size).step_by(chunk_length) {
    let end = min(i + chunk_length, size);
    append((i, end), (size, size));
  }

  result
}
