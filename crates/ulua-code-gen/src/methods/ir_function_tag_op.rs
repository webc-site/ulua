use crate::records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp};

impl IrFunction {
  pub fn tag_op(&self, op: IrOp) -> u8 {
    match self.const_op(op) {
      IrConst::Tag(value) => value,
      // 与原 CODEGEN_ASSERT 语义一致：release 下仍校验
      _ => panic!("tag_op: 非 Tag 常量"),
    }
  }
}

#[unsafe(export_name = "ulua_ir_function_tag_op")]
pub extern "C-unwind" fn ir_function_tag_op() {}
