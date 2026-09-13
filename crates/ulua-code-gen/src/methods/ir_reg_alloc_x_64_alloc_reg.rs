use ulua_common::FFlag::LuauCodegenVmExitSync;

use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_data::K_INVALID_INST_IDX, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64,
  },
};

impl IrRegAllocX64 {
  pub fn alloc_reg(&mut self, size: SizeX64, inst_idx: u32) -> RegisterX64 {
    if LuauCodegenVmExitSync.get() {
      self.alloc_action_count += 1;
    }

    if size == SizeX64::Xmmword {
      for i in 0..self.usable_xmm_reg_count as usize {
        if self.free_xmm_map[i] {
          self.free_xmm_map[i] = false;
          self.xmm_inst_users[i] = inst_idx;
          return RegisterX64 {
            bits: ((i as u8) << RegisterX64::INDEX_SHIFT) | (size as u8),
          };
        }
      }
    } else {
      for reg in IrRegAllocX64::K_GPR_ALLOC_ORDER.iter() {
        if self.free_gpr_map[reg.index() as usize] {
          self.free_gpr_map[reg.index() as usize] = false;
          self.gpr_inst_users[reg.index() as usize] = inst_idx;
          return RegisterX64 {
            bits: (reg.index() << RegisterX64::INDEX_SHIFT) | (size as u8),
          };
        }
      }
    }

    // Out of registers, spill the value with the furthest next use
    let reg_inst_users = if size == SizeX64::Xmmword {
      &self.xmm_inst_users
    } else {
      &self.gpr_inst_users
    };

    let furthest_use_target = self.find_instruction_with_furthest_next_use(reg_inst_users);
    if furthest_use_target != K_INVALID_INST_IDX {
      let reg = unsafe {
        let instructions = &(*self.function).instructions;
        instructions[furthest_use_target as usize].reg_x64
      };
      let mut reg = reg;
      reg.bits = (reg.index() << RegisterX64::INDEX_SHIFT) | (size as u8);

      return self.take_reg(reg, inst_idx);
    }

    CODEGEN_ASSERT!(false, "Out of registers to allocate");
    RegisterX64::NOREG
  }
}
