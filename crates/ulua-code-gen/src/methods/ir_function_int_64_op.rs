use crate::records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp};

impl IrFunction {
  pub fn int64_op(&mut self, op: IrOp) -> i64 {
    let value = self.const_op(op);
    debug_assert!(matches!(value, IrConst::Int64(_)));
    match value {
      IrConst::Int64(v) => v,
      // 原 release 下为 union 误读 UB；此处确定回退零值，debug 下由断言拦截
      _ => 0,
    }
  }
}

#[unsafe(export_name = "ulua_ir_function_int_64_op")]
pub extern "C-unwind" fn ir_function_int_64_op() {}
