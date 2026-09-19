use crate::records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp};

impl IrFunction {
  pub fn int_op(&self, op: IrOp) -> i32 {
    match self.const_op(op) {
      IrConst::Int(value) => value,
      // 与原 assert 语义一致：release 下仍校验，类型不符即编译器内部不变量被破坏
      _ => panic!("int_op: 非 Int 常量"),
    }
  }
}

#[unsafe(export_name = "ulua_ir_function_int_op")]
pub extern "C-unwind" fn ir_function_int_op() {}
