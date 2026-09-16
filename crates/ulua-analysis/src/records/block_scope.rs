use crate::records::{block::Block, cfg_builder::CfgBuilder};

#[derive(Debug, Clone)]
pub struct BlockScope {
  pub builder: *mut CfgBuilder,
  pub saved: *mut Block,
}
