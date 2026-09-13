use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn int64_op(&mut self, op: IrOp) -> i64 {
    let value: IrConst = self.const_op(op);

    // Avoid relying on CODEGEN_ASSERT implementation details across targets.
    debug_assert!(value.kind == IrConstKind::Int64);

    unsafe { value.value.value_int64 }
  }
}

#[unsafe(export_name = "ulua_ir_function_int_64_op")]
pub extern "C-unwind" fn ir_function_int_64_op() {}
