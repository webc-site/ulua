use ulua_vm::records::{global_state::global_State, lua_state::lua_State};

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64, ir_cmd::IrCmd, ir_value_kind::IrValueKind,
    kind_a_64::KindA64,
  },
  functions::{
    cast_reg::cast_reg, free_spill::free_spill, get_cmd_value_kind::get_cmd_value_kind,
    get_reload_address::get_reload_address,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, ir_inst::IrInst, ir_reg_alloc_a_64::IrRegAllocA64,
    register_a_64::RegisterA64, spill::Spill, value_restore_location::ValueRestoreLocation,
  },
};

const K_INVALID_SPILL: i8 = 64;
const K_STASH_SLOTS: i32 = 9;
const K_TEMP_SLOTS: i32 = 1;
const S_TEMPORARY_DATA: i32 = K_STASH_SLOTS * 8;
const S_SPILL_AREA_DATA: i32 = (K_STASH_SLOTS + K_TEMP_SLOTS) * 8;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << RegisterA64::INDEX_SHIFT),
  }
}

const X17: RegisterA64 = reg(KindA64::X, 17);
const XZR: RegisterA64 = reg(KindA64::X, 31);
const SP: RegisterA64 = reg(KindA64::None, 31);
const R_STATE: RegisterA64 = reg(KindA64::X, 19);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
  AddressA64 {
    kind: AddressKindA64::Imm,
    base,
    offset: RegisterA64::NOREG,
    data,
  }
}

fn s_temporary() -> AddressA64 {
  mem(SP, S_TEMPORARY_DATA)
}

impl IrRegAllocA64 {
  pub fn restore_ir_reg_alloc_a_64_spill_register_a_64(&mut self, s: &Spill, reg: RegisterA64) {
    let inst: *mut IrInst = unsafe {
      let instructions = &mut (*self.function).instructions;
      &mut instructions[s.inst as usize]
    };
    CODEGEN_ASSERT!(unsafe { (*inst).reg_a64 } == RegisterA64::NOREG);

    if s.slot >= 0 {
      if self.is_extra_spill_slot(s.slot as u32) {
        let extra_offset = self.get_extra_spill_address_offset(s.slot as u32);

        // Need to calculate an address, but everything might be taken.
        // If we are restoring an integer register, we can just use it as a temporary.
        let emergency_temp = if reg.kind() == KindA64::W {
          cast_reg(KindA64::X, reg)
        } else if reg.kind() == KindA64::X {
          reg
        } else {
          X17
        };

        if reg.kind() != KindA64::W && reg.kind() != KindA64::X {
          unsafe { (*self.build).str(emergency_temp, s_temporary()) };
        }

        unsafe {
          (*self.build).ldr(
            emergency_temp,
            mem(R_STATE, core::mem::offset_of!(lua_State, global) as i32),
          );
          (*self.build).ldr(
            emergency_temp,
            mem(
              emergency_temp,
              core::mem::offset_of!(global_State, ecbdata) as i32,
            ),
          );

          (*self.build).ldr(reg, mem(emergency_temp, extra_offset));

          if reg.kind() != KindA64::W && reg.kind() != KindA64::X {
            (*self.build).ldr(emergency_temp, s_temporary());
          }
        }
      } else {
        unsafe { (*self.build).ldr(reg, mem(SP, S_SPILL_AREA_DATA + s.slot as i32 * 8)) };
      }

      if s.slot != K_INVALID_SPILL {
        free_spill(&mut self.free_spill_slots, reg.kind(), s.slot as u8);
      }
    } else {
      CODEGEN_ASSERT!(unsafe { !(*inst).spilled && (*inst).needs_reload });

      // When restoring the value, we allow cross-block restore because we have commited to the target location at spill time.
      let restore_location: ValueRestoreLocation =
        unsafe { (&*self.function).find_restore_location_ir_inst_bool(&*inst, false) };

      let addr: AddressA64 = get_reload_address(restore_location);
      CODEGEN_ASSERT!(addr.base != XZR);

      let spill_value_kind: IrValueKind = unsafe { get_cmd_value_kind((*inst).cmd) };

      if spill_value_kind == IrValueKind::Int && restore_location.kind == IrValueKind::Double {
        // Handle restore of an int/uint value from a location storing a double number.
        let temp = self.alloc_temp(KindA64::D);
        unsafe { (*self.build).ldr(temp, addr) };

        if restore_location.conversion_cmd == IrCmd::IntToNum {
          unsafe { (*self.build).fcvtzs(reg, temp) };
        } else if restore_location.conversion_cmd == IrCmd::UintToNum {
          unsafe { (*self.build).fcvtzs(cast_reg(KindA64::X, reg), temp) };
        } else {
          CODEGEN_ASSERT!(false);
        }

        // Temporary might have taken a spot needed for other registers in spill restore process.
        self.free_temp(temp);
      } else {
        unsafe { (*self.build).ldr(reg, addr) };
      }
    }

    unsafe {
      (*inst).spilled = false;
      (*inst).needs_reload = false;
      (*inst).reg_a64 = reg;
    }
  }
}
