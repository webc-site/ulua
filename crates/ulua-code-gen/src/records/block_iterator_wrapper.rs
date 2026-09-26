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

impl BlockIteratorWrapper<'_> {
  pub fn empty(&self) -> bool {
    self.as_slice().is_empty()
  }

  pub fn operator_index(&self, pos: usize) -> u32 {
    self.as_slice()[pos]
  }

  pub fn size(&self) -> usize {
    self.len()
  }
}
