use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn double_op(&self, op: IrOp) -> f64 {
    let value = self.const_op(op);

    assert!(value.kind == IrConstKind::Double);

    unsafe { value.value.value_double }
  }
}

#[unsafe(export_name = "ulua_ir_function_double_op")]
pub extern "C-unwind" fn ir_function_double_op() {}
