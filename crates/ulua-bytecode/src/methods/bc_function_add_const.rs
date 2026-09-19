use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_function::{BcFunction, VmConst},
    bc_op::BcOp,
  },
};

impl BcFunction {
  /// cpp `BcFunction::addConst`：追加 VM 常量并返回其引用。
  pub fn add_const(&mut self, value: VmConst) -> BcOp {
    self.constants.push(value);
    BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmConst, (self.constants.len() - 1) as u32)
  }
}
