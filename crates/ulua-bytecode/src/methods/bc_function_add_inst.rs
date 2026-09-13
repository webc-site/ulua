use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_inst::BcInst, bc_op::BcOp},
};

impl BcFunction {
  pub fn add_inst(&mut self) -> BcOp {
    self.instructions.push(BcInst {
      op: LuauOpcode::LOP_NOP,
      block: BcOp::new(),
      ops: Default::default(),
      last_use: 0,
      use_count: 0,
      line: 0,
    });
    BcOp::bc_op_bc_op_kind_u32(BcOpKind::Inst, (self.instructions.len() - 1) as u32)
  }
}
