use ulua_common::FFlag::LuauCodegenVmExitSync;

use crate::{
  functions::get_next_inst_use::get_next_inst_use, records::ir_reg_alloc_x_64::IrRegAllocX64,
};

impl IrRegAllocX64 {
  pub fn find_instruction_with_furthest_next_use(&self, reg_inst_users: &[u32; 16]) -> u32 {
    if self.curr_inst_idx == u32::MAX {
      return u32::MAX;
    }

    let mut furthest_use_target = u32::MAX;
    let mut furthest_use_location: u32 = 0;

    for &reg_inst_user in reg_inst_users.iter() {
      // Cannot spill temporary registers or the register of the value that's defined in the current instruction
      if reg_inst_user == u32::MAX || reg_inst_user == self.curr_inst_idx {
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

      if furthest_use_target == u32::MAX || next_use > furthest_use_location {
        furthest_use_location = next_use;
        furthest_use_target = reg_inst_user;
      }
    }

    furthest_use_target
  }
}
