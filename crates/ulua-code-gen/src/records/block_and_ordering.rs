use core::cmp::Ordering;

use crate::records::block_ordering::BlockOrdering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockAndOrdering {
  pub(crate) block_idx: u32,
  pub(crate) ordering: BlockOrdering,
}

impl PartialOrd for BlockAndOrdering {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for BlockAndOrdering {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    if self.ordering.depth != other.ordering.depth {
      self.ordering.depth.cmp(&other.ordering.depth)
    } else {
      self.ordering.pre_order.cmp(&other.ordering.pre_order)
    }
  }
}
