use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst::BcInst, bc_op::BcOp, bc_ref::BcRef, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  pub fn is_get_var_arg(&mut self, target_op: BcOp) -> bool {
    if target_op.kind != BcOpKind::Inst {
      return false;
    }
    let inst: BcRef<BcInst> = self.target.inst(target_op);
    inst.operator_deref().op == LuauOpcode::LOP_GETVARARGS
  }
}
