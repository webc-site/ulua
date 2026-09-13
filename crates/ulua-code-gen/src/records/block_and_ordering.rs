use crate::records::block_ordering::BlockOrdering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockAndOrdering {
  pub(crate) block_idx: u32,
  pub(crate) ordering: BlockOrdering,
}
