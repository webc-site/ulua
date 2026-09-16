use crate::records::{block::Block, cfg_builder::CfgBuilder};

impl CfgBuilder {
  pub fn is_sealed(&self, b: *mut Block) -> bool {
    self.sealed_blocks.contains(&b)
  }
}
