use crate::{
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn uint_op(&self, op: IrOp) -> u32 {
    let value = self.const_op(op);
    debug_assert!(matches!(value, IrConst::Uint(_)));
    match value {
      IrConst::Uint(v) => v,
      // 原 release 下为 union 误读 UB；此处确定回退零值，debug 下由断言拦截
      _ => 0,
    }
  }
}
