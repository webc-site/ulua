//! `generate_spans`：ddmin 式分块删除的候选区间生成器（上游
//! `CLI/src/Reduce.cpp:246-283` 的 `Reducer::generateSpans`）。
//!
//! 纯切片算术、不依赖 `Reducer` 状态，故走公开自由函数即可，无需 AST 夹具。
//! 钉的是三条上游约定：chunk 长度 = `max(1, size / chunks)`、正/反两个方向的
//! 组合都要出、以及 cpp `append` 的「两区间皆空则不入列」退化保护。

use ulua_reduce_cli::generate_spans;

/// cpp `Reduce.cpp:249-251`：单元素以下没有可删的区间
#[test]
fn degenerate_sizes_produce_no_spans() {
  assert!(generate_spans(0, 4).is_empty());
  assert!(generate_spans(1, 4).is_empty());
}

/// chunk 长度 = `max(1, size / chunks)`：给定 4 个 chunk 时按 2 个语句一组切
#[test]
fn chunk_length_is_size_divided_by_chunks() {
  let spans = generate_spans(4, 2);

  // 「保前缀挖后缀」+「整块挖除」两轮，每轮 2 项
  assert_eq!(
    spans,
    [
      ((0, 0), (2, 4)), // 删掉后半 [2,4)
      ((0, 2), (4, 4)), // 删掉前缀 [0,2)
      ((0, 2), (4, 4)), // 第二轮：整块 [0,2)
      ((2, 4), (4, 4)), // 第二轮：整块 [2,4)
    ]
  );
}

/// `size / chunks == 0` 时 chunk 长度退化为 1：逐条删除，两个方向的组合齐全
#[test]
fn more_chunks_than_items_falls_back_to_single_step() {
  let spans = generate_spans(3, 10);

  assert_eq!(
    spans,
    [
      ((0, 0), (1, 3)),
      ((0, 1), (2, 3)),
      ((0, 2), (3, 3)),
      ((0, 1), (3, 3)),
      ((1, 2), (3, 3)),
      ((2, 3), (3, 3)),
    ]
  );
}

/// cpp `append` 的退化保护：两个区间都空时不入列（否则会产出一个「什么都不删」的候选）
#[test]
fn empty_pair_is_not_appended() {
  let spans = generate_spans(2, 1);

  // 第一轮 (0,0)+(2,2) 被丢弃，只剩整块删除
  assert_eq!(spans, [((0, 2), (2, 2))]);
}
