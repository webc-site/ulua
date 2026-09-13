use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::vm_reg_op::vm_reg_op,
  records::{ir_op::IrOp, remove_dead_store_state::RemoveDeadStoreState},
};

impl RemoveDeadStoreState {
  pub fn maybe_use(&mut self, op: IrOp) {
    if op.kind() == IrOpKind::VmReg {
      let reg = vm_reg_op(op);
      self.use_reg(reg as u8);
    }
  }
}
