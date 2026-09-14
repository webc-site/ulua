use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn import_op(&self, op: IrOp) -> u32 {
    let value: IrConst = self.const_op(op);
    debug_assert!(value.kind == IrConstKind::Import);
    unsafe { value.value.value_uint }
  }
}
