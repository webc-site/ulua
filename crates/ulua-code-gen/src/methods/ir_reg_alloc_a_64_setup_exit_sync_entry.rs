use crate::{
  enums::kind_a_64::KindA64,
  functions::update_last_use_locations_in_block::update_last_use_locations_in_block,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64, spill::Spill},
};
impl IrRegAllocA64 {
  pub fn setup_exit_sync_entry(&mut self, block_idx: u32) {
    update_last_use_locations_in_block(unsafe { &mut *self.function }, block_idx);

    let Some(args) = self.exit_sync_args.find(&block_idx).cloned() else {
      return;
    };

    for arg in args.iter() {
      let inst = unsafe {
        let instructions = &mut (*self.function).instructions;
        &mut instructions[arg.inst_idx as usize]
      };

      inst.reused_reg = false;
      inst.needs_reload = false;
      inst.spilled = false;

      if arg.reg != RegisterA64::NOREG {
        inst.reg_a64 = arg.reg;

        self.take_reg(arg.reg, arg.inst_idx);
      } else if arg.slot >= 0 {
        inst.reg_a64 = RegisterA64::NOREG;
        inst.spilled = true;

        self.spills.push(Spill {
          inst: arg.inst_idx,
          origin: arg.original_reg,
          slot: arg.slot,
        });

        // Mark the spill slot as occupied so restore() can free it
        let mask = if arg.original_reg.kind() == KindA64::Q {
          3u64
        } else {
          1u64
        } << (arg.slot as u64);

        CODEGEN_ASSERT!((self.free_spill_slots & mask) == mask);
        self.free_spill_slots &= !mask;
      } else {
        inst.reg_a64 = RegisterA64::NOREG;
        inst.needs_reload = true;

        // Re-record the restore location captured at snapshot time
        // Later instructions in the source block may have invalidated it in IrValueLocationTracking
        unsafe { &mut *self.function }.record_restore_location(arg.inst_idx, arg.restore_location);

        self.spills.push(Spill {
          inst: arg.inst_idx,
          origin: arg.original_reg,
          slot: arg.slot,
        });
      }
    }
  }
}
