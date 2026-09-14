use crate::{
  functions::vm_reg_op::vm_reg_op,
  records::{ir_op::IrOp, remove_dead_store_state::RemoveDeadStoreState},
};

impl RemoveDeadStoreState {
  pub fn use_(&mut self, op: IrOp, offset: i32) {
    let reg = vm_reg_op(op) + offset;
    self.use_reg(reg as u8);
  }
}
