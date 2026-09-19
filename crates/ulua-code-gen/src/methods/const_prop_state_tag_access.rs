use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{const_prop_state::ConstPropState, ir_op::IrOp},
  traits::tag_access::TagAccess,
};

impl ConstPropState {
  /// 与 cpp 闭包捕获的 `build.vmReg(i)` 等价：纯 `IrOp` 构造，无需 IrBuilder
  fn vm_reg_op(index: usize) -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::VmReg, index as u32)
  }
}

/// cpp OptimizeConstProp.cpp：get 读 `state.regs[i].tag`，set 走 `state.updateTag(vmReg(i), tag)`
impl TagAccess for ConstPropState {
  fn get_tag(&self, i: usize) -> u8 {
    self.regs[i].tag
  }

  fn set_tag(&mut self, i: usize, tag: u8) {
    self.update_tag(Self::vm_reg_op(i), tag);
  }
}
