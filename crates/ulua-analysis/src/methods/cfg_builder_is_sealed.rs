use crate::{records::cfg_builder::CfgBuilder, type_aliases::block_id::BlockId};

impl CfgBuilder {
  pub fn is_sealed(&self, b: BlockId) -> bool {
    self.sealed_blocks.contains(&b)
  }
}
