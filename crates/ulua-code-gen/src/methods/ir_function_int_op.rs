use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn int_op(&self, op: IrOp) -> i32 {
    let value = self.const_op(op);

    assert!(value.kind == IrConstKind::Int);

    unsafe { value.value.value_int }
  }
}

#[unsafe(export_name = "ulua_ir_function_int_op")]
pub extern "C-unwind" fn ir_function_int_op() {}
