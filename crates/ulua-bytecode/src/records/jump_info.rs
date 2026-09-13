use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::bc_op::BcOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JumpInfo {
  pub(crate) op: LuauOpcode,
  pub(crate) instruction_pc: u32,
  pub(crate) target_block: BcOp,
}
