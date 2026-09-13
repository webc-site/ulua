use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{records::bc_op::BcOp, type_aliases::bc_ops::BcOps};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BcInst {
  pub op: LuauOpcode,
  pub block: BcOp,
  pub ops: BcOps,
  pub last_use: u32,
  pub use_count: u32,
  pub line: u32,
}

impl Default for BcInst {
  fn default() -> Self {
    Self {
      op: LuauOpcode::LOP_NOP,
      block: BcOp::new(),
      ops: Default::default(),
      last_use: 0,
      use_count: 0,
      line: 0,
    }
  }
}
