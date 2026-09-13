use crate::{
  enums::ir_value_kind::K_VALUE_DWORD_SIZE,
  functions::{
    get_cmd_value_kind::get_cmd_value_kind,
    update_last_use_locations_in_block::update_last_use_locations_in_block,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_reg_alloc_x_64::IrRegAllocX64, ir_spill_x_64::IrSpillX64, register_x_64::RegisterX64,
  },
};

impl IrRegAllocX64 {
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

      if arg.reg != RegisterX64::NOREG {
        inst.reg_x64 = arg.reg;

        self.take_reg(arg.reg, arg.inst_idx);
      } else if arg.stack_slot != IrSpillX64::K_NO_STACK_SLOT {
        inst.reg_x64 = RegisterX64::NOREG;
        inst.spilled = true;

        let mut spill = IrSpillX64 {
          inst_idx: arg.inst_idx,
          value_kind: get_cmd_value_kind(inst.cmd),
          spill_id: 0,
          stack_slot: arg.stack_slot,
          original_loc: arg.original_reg,
        };

        spill.spill_id = self.next_spill_id;
        self.next_spill_id += 1;

        // Mark the spill slot as occupied so restore can free it
        let end = spill.stack_slot as u32 + K_VALUE_DWORD_SIZE[spill.value_kind as usize];
        let mut pos = spill.stack_slot as u32;
        while pos < end {
          let mask = 1u64 << (pos % 64);
          CODEGEN_ASSERT!(self.used_spill_slot_halfs[(pos / 64) as usize] & mask == 0);
          self.used_spill_slot_halfs[(pos / 64) as usize] |= mask;
          pos += 1;
        }

        self.spills.push(spill);
      } else {
        // Value has a restore address (rematerializable)
        inst.reg_x64 = RegisterX64::NOREG;
        inst.needs_reload = true;

        // Re-record the restore location captured at snapshot time
        // Later instructions in the source block may have invalidated it in IrValueLocationTracking
        unsafe { &mut *self.function }.record_restore_location(arg.inst_idx, arg.restore_location);

        let mut spill = IrSpillX64 {
          inst_idx: arg.inst_idx,
          value_kind: get_cmd_value_kind(inst.cmd),
          spill_id: 0,
          stack_slot: IrSpillX64::K_NO_STACK_SLOT,
          original_loc: arg.original_reg,
        };

        spill.spill_id = self.next_spill_id;
        self.next_spill_id += 1;

        self.spills.push(spill);
      }
    }
  }
}
