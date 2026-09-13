use crate::records::{const_prop_state::ConstPropState, ir_op::IrOp};

impl ConstPropState {
  pub fn save_tag(&mut self, op: IrOp, tag: u8) {
    if let Some(info) = self.try_get_register_info(op) {
      unsafe {
        if (*info).tag != tag {
          (*info).tag = tag;
          (*info).version += 1;
        }
      }
    }
  }
}
