use std::collections::HashMap;

use crate::{records::bc_op::BcOp, type_aliases::reg::Reg};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockProducers {
  pub(crate) own: HashMap<Reg, BcOp>,
  pub(crate) cached: HashMap<Reg, BcOp>,
  pub(crate) multi_return: BcOp,
  pub(crate) multi_return_start: Reg,
  pub(crate) invalid_after: i32,
}

impl Default for BlockProducers {
  fn default() -> Self {
    Self {
      own: HashMap::default(),
      cached: HashMap::default(),
      multi_return: BcOp::new(),
      multi_return_start: 0,
      invalid_after: 255,
    }
  }
}
