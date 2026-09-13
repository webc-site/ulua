use crate::{
  enums::ir_value_kind::K_VALUE_DWORD_SIZE,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    exit_sync_arg_x_64::ExitSyncArgX64, ir_inst::IrInst, ir_reg_alloc_x_64::IrRegAllocX64,
    ir_spill_x_64::IrSpillX64, register_x_64::RegisterX64,
    value_restore_location::ValueRestoreLocation,
  },
};

impl IrRegAllocX64 {
  pub fn record_and_free_last_use(
    &mut self,
    block_idx: u32,
    target: &mut IrInst,
    origin_inst_idx: u32,
  ) {
    let mut arg = ExitSyncArgX64 {
      inst_idx: unsafe { &*self.function }.get_inst_index(target),
      reg: RegisterX64::NOREG,
      stack_slot: 0,
      original_reg: RegisterX64::NOREG,
      restore_location: ValueRestoreLocation::default(),
    };

    if target.spilled || target.needs_reload {
      let mut i = 0;
      while i < self.spills.len() {
        if self.spills[i].inst_idx == arg.inst_idx {
          let spill = self.spills[i].clone();

          arg.original_reg = spill.original_loc;
          arg.stack_slot = spill.stack_slot;

          // Capture restore location state at the current instruction
          if arg.stack_slot == IrSpillX64::K_NO_STACK_SLOT {
            arg.restore_location =
              unsafe { &*self.function }.find_restore_location_ir_inst_bool(target, false);
          }

          // If this was the last use, free register by not restoring it fully and remove the spill record
          if self.is_last_use_reg(target, origin_inst_idx) {
            if arg.stack_slot != IrSpillX64::K_NO_STACK_SLOT {
              let end =
                arg.stack_slot as usize + K_VALUE_DWORD_SIZE[spill.value_kind as usize] as usize;

              for pos in arg.stack_slot as usize..end {
                let bit = pos as u64;
                let idx = bit / 64;
                let mask = 1u64 << (bit % 64);
                self.used_spill_slot_halfs[idx as usize] &= !mask;
              }
            }

            CODEGEN_ASSERT!(target.reg_x64 == RegisterX64::NOREG);
            target.spilled = false;
            target.needs_reload = false;

            self.spills[i] = self.spills[self.spills.len() - 1].clone();
            self.spills.pop();
          }

          break;
        }
        i += 1;
      }
    } else {
      CODEGEN_ASSERT!(target.reg_x64 != RegisterX64::NOREG);
      arg.reg = target.reg_x64;
      arg.original_reg = target.reg_x64;

      if self.is_last_use_reg(target, origin_inst_idx) {
        self.free_reg(target.reg_x64);
        target.reg_x64 = RegisterX64::NOREG;
      }
    }

    let entry = self.exit_sync_args.get_or_insert(block_idx);
    entry.push(arg);
  }
}
