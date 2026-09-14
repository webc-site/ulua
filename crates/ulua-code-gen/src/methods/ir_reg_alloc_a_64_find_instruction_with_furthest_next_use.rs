use ulua_common::FFlag::LuauCodegenVmExitSync;

use crate::{
  functions::get_next_inst_use::get_next_inst_use,
  records::{ir_reg_alloc_a_64::IrRegAllocA64, set::Set},
};

impl IrRegAllocA64 {
  pub fn find_instruction_with_furthest_next_use(&self, set: &mut Set) -> u32 {
    if self.curr_inst_idx == Self::K_INVALID_INST_IDX {
      return Self::K_INVALID_INST_IDX;
    }

    let mut furthest_use_target = Self::K_INVALID_INST_IDX;
    let mut furthest_use_location: u32 = 0;

    for &reg_inst_user in set.defs.iter() {
      // Cannot spill temporary registers or the register of the value that's defined in the current instruction
      if reg_inst_user == Self::K_INVALID_INST_IDX || reg_inst_user == self.curr_inst_idx {
        continue;
      }

      let mut in_vm_exit_sync = false;

      let next_use = get_next_inst_use(
        unsafe { &mut *self.function },
        reg_inst_user,
        self.curr_inst_idx,
        &mut in_vm_exit_sync,
      );

      // Cannot spill value that is about to be used in the current instruction
      if next_use == self.curr_inst_idx && (!LuauCodegenVmExitSync.get() || !in_vm_exit_sync) {
        continue;
      }

      if furthest_use_target == Self::K_INVALID_INST_IDX || next_use > furthest_use_location {
        furthest_use_location = next_use;
        furthest_use_target = reg_inst_user;
      }
    }

    furthest_use_target
  }
}
