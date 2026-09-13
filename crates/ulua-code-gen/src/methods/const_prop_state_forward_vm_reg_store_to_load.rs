use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::vm_reg_op::vm_reg_op,
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a, op_b::op_b},
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
  type_aliases::ir_ops::IrOps,
};

impl ConstPropState {
  pub fn forward_vm_reg_store_to_load(&mut self, store_inst: &mut IrInst, load_cmd: IrCmd) {
    let store_reg = op_a(store_inst);
    let stored_value = op_b(store_inst.clone());

    CODEGEN_ASSERT!(store_reg.kind() == IrOpKind::VmReg);
    CODEGEN_ASSERT!(stored_value.kind() == IrOpKind::Inst);

    let reg = vm_reg_op(store_reg) as usize;
    let captured_regs = unsafe { &(*self.function).cfg.captured.regs };
    if (captured_regs[reg / 64] & (1u64 << (reg % 64))) != 0 {
      return;
    }

    let mut versioned_reg = store_reg;
    versioned_reg = IrOp::ir_op_ir_op_kind_u32(
      IrOpKind::VmReg,
      (vm_reg_op(versioned_reg) as u32) | (self.regs[reg].version << 8),
    );
    let mut ops = IrOps::new();
    ops.push(versioned_reg);
    let key = IrInst {
      cmd: load_cmd,
      ops,
      last_use: 0,
      use_count: 0,
      reg_x64: Default::default(),
      reg_a64: Default::default(),
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    };
    *self.value_map.get_or_insert(key) = stored_value.index();
  }
}
