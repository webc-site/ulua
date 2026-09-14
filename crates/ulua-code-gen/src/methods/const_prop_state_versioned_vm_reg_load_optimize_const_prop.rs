use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::vm_reg_op::vm_reg_op,
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
  type_aliases::ir_ops::IrOps,
};

impl ConstPropState {
  pub fn versioned_vm_reg_load_ir_cmd_ir_op(&mut self, load_cmd: IrCmd, mut op: IrOp) -> IrInst {
    let version = self.regs[vm_reg_op(op) as usize].version;
    op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, (vm_reg_op(op) as u32) | (version << 8));

    let mut ops = IrOps::new();
    ops.push(op);

    IrInst {
      cmd: load_cmd,
      ops,
      ..IrInst::default()
    }
  }
}
