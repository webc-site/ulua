use crate::records::{const_prop_state::ConstPropState, ir_op::IrOp};

impl ConstPropState {
  pub fn update_tag(&mut self, op: IrOp, tag: u8) {
    if let Some(info) = self.try_get_register_info(op) {
      unsafe {
        (*info).tag = tag;
      }
    }
  }
}
