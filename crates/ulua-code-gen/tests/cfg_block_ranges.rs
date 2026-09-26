//! `BlockIteratorWrapper`（cpp `IrAnalysis.h` 的 `DomChildren`/`Predecessors`/
//! `Successors` 返回的 `gsl::span` 视图）与 `IrAnalysis::predecessors/successors/
//! domChildren` 三个 range 取器的行为对齐。
//!
//! 关键性质：块索引 → `[offsets[i], offsets[i+1])` 半开区间；区间内的重复项
//! 必须原样保留（CFG 多重边）；越出 `offsets` 长度要像 cpp `CODEGEN_ASSERT`
//! 一样炸，而不是返回一个越界切片。

use ulua_code_gen::{
  functions::{dom_children::dom_children, predecessors::predecessors, successors::successors},
  records::{block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo},
};

#[test]
fn cfg_ranges_preserve_order_and_remaining_slice() {
  let cfg = CfgInfo {
    successors: vec![3, 1, 2],
    successors_offsets: vec![0, 2, 2],
    ..CfgInfo::default()
  };
  let mut blocks = successors(&cfg, 0);
  assert_eq!(blocks.as_slice(), &[3, 1]);
  assert_eq!(blocks.next(), Some(3));
  assert_eq!(blocks.as_slice(), &[1]);
  assert_eq!(blocks.next(), Some(1));
  assert!(blocks.as_slice().is_empty());
  assert_eq!(blocks.next(), None);
  // offsets[1] == offsets[2]：空区间，不是越界
  assert!(successors(&cfg, 1).as_slice().is_empty());
  assert_eq!(successors(&cfg, 2).collect::<Vec<_>>(), [2]);
}

#[test]
fn all_cfg_ranges_preserve_duplicates_and_boundaries() {
  let cfg = CfgInfo {
    predecessors: vec![3, 1, 3, 2],
    predecessors_offsets: vec![0, 0, 3],
    successors: vec![3, 1, 3, 2],
    successors_offsets: vec![0, 0, 3],
    dom_children: vec![3, 1, 3, 2],
    dom_children_offsets: vec![0, 0, 3],
    ..CfgInfo::default()
  };
  for range in [predecessors, successors, dom_children] {
    assert!(range(&cfg, 0).empty());
    let mut blocks = range(&cfg, 1);
    assert_eq!(blocks.size_hint(), (3, Some(3)));
    assert_eq!(blocks.operator_index(2), 3);
    let saved = blocks.as_slice();
    assert_eq!(blocks.next(), Some(3));
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks.size(), 2);
    assert_eq!(blocks.operator_index(0), 1);
    assert_eq!(blocks.clone().collect::<Vec<_>>(), [1, 3]);
    // 已消费的 `as_slice()` 不影响原区间（视图是独立借用）
    assert_eq!(saved, &[3, 1, 3]);
    assert_eq!(blocks.next(), Some(1));
    assert_eq!(blocks.next(), Some(3));
    assert!(blocks.empty());
    assert_eq!(blocks.size_hint(), (0, Some(0)));
    assert_eq!(blocks.next(), None);
    assert_eq!(range(&cfg, 2).collect::<Vec<_>>(), [2]);
  }
}

#[test]
fn empty_cfg_storage_has_valid_empty_ranges() {
  let cfg = CfgInfo {
    predecessors_offsets: vec![0, 0],
    successors_offsets: vec![0, 0],
    dom_children_offsets: vec![0, 0],
    ..CfgInfo::default()
  };
  for range in [predecessors, successors, dom_children] {
    for block in [0, 1] {
      let mut blocks = range(&cfg, block);
      assert!(blocks.empty());
      assert_eq!(blocks.size(), 0);
      assert_eq!(blocks.as_slice(), &[]);
      assert_eq!(blocks.next(), None);
    }
  }
}

#[test]
// 锁定失败形态是切片下标越界，而非任意 panic
#[should_panic(expected = "index out of bounds")]
fn index_past_remaining_range_panics() {
  let cfg = CfgInfo {
    successors: vec![3, 1],
    successors_offsets: vec![0, 2],
    ..CfgInfo::default()
  };
  let mut blocks: BlockIteratorWrapper<'_> = successors(&cfg, 0);
  assert_eq!(blocks.next(), Some(3));
  // 剩余区间长度 1，下标 1 越界
  blocks.operator_index(1);
}

#[test]
// 锁定失败形态是切片区间越界（offsets 声明的 [0,2) 超出 successors 实长）
#[should_panic(expected = "out of range for slice")]
fn invalid_cfg_range_panics() {
  let cfg = CfgInfo {
    successors: vec![3],
    successors_offsets: vec![0, 2],
    ..CfgInfo::default()
  };
  successors(&cfg, 0);
}
