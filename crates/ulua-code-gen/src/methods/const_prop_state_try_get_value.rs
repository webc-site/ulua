use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{const_prop_state::ConstPropState, ir_op::IrOp},
};

impl ConstPropState {
  pub fn try_get_value(&mut self, op: IrOp) -> IrOp {
    if let Some(info) = self.try_get_register_info(op) {
      return unsafe { (*info).value };
    }

    IrOp::ir_op_kind_u32(IrOpKind::None, 0)
  }
}
