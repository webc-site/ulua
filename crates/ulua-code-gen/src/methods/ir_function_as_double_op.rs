use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn as_double_op(&mut self, op: IrOp) -> Option<f64> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    match self.const_op(op) {
      IrConst::Double(value) => Some(value),
      _ => None,
    }
  }
}

#[unsafe(export_name = "ulua_ir_function_as_double_op")]
pub extern "C-unwind" fn ir_function_as_double_op() {}

impl IrFunction {
  pub fn const_op(&self, op: IrOp) -> IrConst {
    self.constants[op.index() as usize]
  }
}
