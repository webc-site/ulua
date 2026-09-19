use core::{iter::FusedIterator, slice::Iter};

#[derive(Debug, Clone, Default)]
pub struct BlockIteratorWrapper<'a> {
  iter: Iter<'a, u32>,
}

impl<'a> BlockIteratorWrapper<'a> {
  pub(crate) fn new(blocks: &'a [u32]) -> Self {
    Self {
      iter: blocks.iter(),
    }
  }

  pub fn as_slice(&self) -> &'a [u32] {
    self.iter.as_slice()
  }
}

impl Iterator for BlockIteratorWrapper<'_> {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    self.iter.next().copied()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

impl ExactSizeIterator for BlockIteratorWrapper<'_> {}

impl FusedIterator for BlockIteratorWrapper<'_> {}

#[cfg(test)]
mod tests {
  use super::BlockIteratorWrapper;
  use crate::{
    functions::{dom_children::dom_children, predecessors::predecessors, successors::successors},
    records::cfg_info::CfgInfo,
  };

  #[test]
  fn default_range_is_empty() {
    let mut blocks = BlockIteratorWrapper::default();
    assert!(blocks.empty());
    assert_eq!(blocks.size(), 0);
    assert_eq!(blocks.as_slice(), &[]);
    assert_eq!(blocks.next(), None);
  }

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
      assert_eq!(saved, &[3, 1, 3]);
      assert_eq!(blocks.next(), Some(1));
      assert_eq!(blocks.next(), Some(3));
      assert!(blocks.empty());
      assert_eq!(blocks.size_hint(), (0, Some(0)));
      assert_eq!(blocks.next(), None);
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
  #[should_panic]
  fn index_past_remaining_range_panics() {
    let mut blocks = BlockIteratorWrapper::new(&[3, 1]);
    assert_eq!(blocks.next(), Some(3));
    blocks.operator_index(1);
  }

  #[test]
  #[should_panic]
  fn invalid_cfg_range_panics() {
    let cfg = CfgInfo {
      successors: vec![3],
      successors_offsets: vec![0, 2],
      ..CfgInfo::default()
    };
    successors(&cfg, 0);
  }
}
