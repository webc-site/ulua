use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{has_side_effects::has_side_effects, substitute::substitute, vm_reg_op::vm_reg_op},
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{const_prop_state::ConstPropState, ir_inst::IrInst, ir_op::IrOp},
  type_aliases::ir_ops::IrOps,
};

impl ConstPropState {
  pub fn substitute_or_record_vm_reg_load(&mut self, load_inst: &mut IrInst) -> bool {
    let reg_op = op_a(load_inst);
    CODEGEN_ASSERT!(reg_op.kind() == IrOpKind::VmReg);

    let reg = vm_reg_op(reg_op) as usize;
    let captured_regs = unsafe { &(*self.function).cfg.captured.regs };
    if (captured_regs[reg / 64] & (1u64 << (reg % 64))) != 0 {
      return false;
    }

    let version = self.regs[reg].version;
    CODEGEN_ASSERT!(version <= 0x00ff_ffff);
    let versioned_reg = IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, (reg as u32) | (version << 8));

    let mut ops = IrOps::new();
    ops.push(versioned_reg);
    if load_inst.cmd == IrCmd::LoadFloat && load_inst.ops.size() > 1 {
      ops.push(load_inst.ops.as_slice()[1]);
    }

    let versioned_load = IrInst {
      cmd: load_inst.cmd,
      ops,
      last_use: 0,
      use_count: 0,
      reg_x64: Default::default(),
      reg_a64: Default::default(),
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    };

    if let Some(prev_idx) = self.value_map.find(&versioned_load).copied() {
      let prev_is_valid = unsafe {
        let prev = &(&(*self.function).instructions)[prev_idx as usize];
        prev.use_count != 0 || has_side_effects(prev.cmd)
      };

      if prev_is_valid {
        if !self.inst_link.contains(&prev_idx) {
          self.create_reg_link(prev_idx, reg_op);
        }

        unsafe {
          substitute(
            &mut *self.function,
            load_inst,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
          );
        }
        return true;
      }
    }

    let inst_idx = unsafe { (&*self.function).get_inst_index(load_inst) };
    *self.value_map.get_or_insert(versioned_load) = inst_idx;
    self.create_reg_link(inst_idx, reg_op);
    false
  }
}
