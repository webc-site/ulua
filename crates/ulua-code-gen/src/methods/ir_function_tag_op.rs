use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

impl IrFunction {
  pub fn tag_op(&self, op: IrOp) -> u8 {
    let value: IrConst = self.const_op(op);

    CODEGEN_ASSERT!(value.kind == IrConstKind::Tag);

    unsafe { value.value.value_tag }
  }
}

#[unsafe(export_name = "ulua_ir_function_tag_op")]
pub extern "C-unwind" fn ir_function_tag_op() {}
