use ulua_common::FFlag;

use crate::{
  enums::kind_a_64::KindA64,
  functions::{countlz_bit_utils::countlz_u32, countrz_bit_utils::countrz_u32},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64, set::Set},
};

impl IrRegAllocA64 {
  pub fn alloc_temp(&mut self, kind: KindA64) -> RegisterA64 {
    if FFlag::LuauCodegenVmExitSync.get() {
      self.alloc_action_count += 1;
    }

    let set = self.get_set(kind) as *mut Set;

    if unsafe { (*set).free } == 0 {
      // Try to find and spill a register that is not used in the current instruction and has the furthest next use
      let furthest_use_target = unsafe { self.find_instruction_with_furthest_next_use(&mut *set) };
      if furthest_use_target != Self::K_INVALID_INST_IDX {
        unsafe { self.spill_set_u32_u32(&mut *set, self.curr_inst_idx, furthest_use_target) };
        CODEGEN_ASSERT!(unsafe { (*set).free } != 0);
      } else {
        self.error = true;
        return RegisterA64 {
          bits: (kind as u8) & RegisterA64::KIND_MASK,
        };
      }
    }

    let mut reg = 31 - countlz_u32(unsafe { (*set).free });

    if FFlag::DebugCodegenChaosA64.get() {
      reg = countrz_u32(unsafe { (*set).free }); // allocate from low end; this causes extra conflicts for calls
    }

    unsafe {
      (*set).free &= !(1u32 << reg);
      (*set).temp |= 1u32 << reg;
      CODEGEN_ASSERT!((*set).defs[reg as usize] == Self::K_INVALID_INST_IDX);
    }

    RegisterA64 {
      bits: ((kind as u8) & RegisterA64::KIND_MASK) | ((reg as u8) << RegisterA64::INDEX_SHIFT),
    }
  }
}
