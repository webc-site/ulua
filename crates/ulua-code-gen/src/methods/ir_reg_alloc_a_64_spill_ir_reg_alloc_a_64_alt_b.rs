use core::mem::size_of;

use ulua_vm::{
  records::{global_state::global_State, lua_state::lua_State},
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64, ir_cmd::IrCmd, ir_op_kind::IrOpKind,
    ir_value_kind::IrValueKind, kind_a_64::KindA64,
  },
  functions::{
    alloc_spill::alloc_spill, get_reload_offset::get_reload_offset, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    address_a_64::AddressA64, ir_inst::IrInst, ir_reg_alloc_a_64::IrRegAllocA64,
    register_a_64::RegisterA64, set::Set, spill::Spill,
  },
};

const K_INVALID_SPILL: i8 = 64;
const K_NO_SPILL_SLOT: i8 = -1;
const K_STASH_SLOTS: i32 = 9;
const K_TEMP_SLOTS: i32 = 1;
const S_TEMPORARY_DATA: i32 = K_STASH_SLOTS * 8;
const S_SPILL_AREA_DATA: i32 = (K_STASH_SLOTS + K_TEMP_SLOTS) * 8;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: kind as u8 | (index << RegisterA64::INDEX_SHIFT),
  }
}

const WZR: RegisterA64 = reg(KindA64::W, 31);
const W17: RegisterA64 = reg(KindA64::W, 17);
const X16: RegisterA64 = reg(KindA64::X, 16);
const X17: RegisterA64 = reg(KindA64::X, 17);
const SP: RegisterA64 = reg(KindA64::None, 31);
const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_BASE: RegisterA64 = reg(KindA64::X, 25);

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
  pub fn spill_set_u32_u32(&mut self, set: &mut Set, index: u32, target_inst_idx: u32) {
    let def: *mut IrInst = unsafe {
      let instructions = &mut (*self.function).instructions;
      &mut instructions[target_inst_idx as usize]
    };
    let reg = unsafe { (*def).reg_a64.index() };

    CODEGEN_ASSERT!(unsafe { !(*def).reused_reg });
    CODEGEN_ASSERT!(unsafe { !(*def).spilled });
    CODEGEN_ASSERT!(unsafe { !(*def).needs_reload });

    if unsafe { (*def).last_use } == index {
      // Instead of spilling the register to never reload it, we assume the register is not needed anymore.
    } else if unsafe { (&*self.function).has_restore_location_ir_inst_bool(&*def, true) } {
      let loc = unsafe { (&*self.function).find_restore_location_ir_inst_bool(&*def, true) };

      // If the value restore location is lazy, we need to materialize it.
      if loc.lazy {
        CODEGEN_ASSERT!(loc.op.kind() == IrOpKind::VmReg);
        CODEGEN_ASSERT!(loc.conversion_cmd == IrCmd::NOP);

        let store_reg = vm_reg_op(loc.op);
        let addr = mem(
          R_BASE,
          store_reg * size_of::<TValue>() as i32 + get_reload_offset(loc.kind),
        );

        unsafe { (*self.build).str((*def).reg_a64, addr) };

        // Partial value store should not have an interpretation in VM/GC and is protected by 'nil' tag.
        if loc.kind != IrValueKind::Tvalue {
          unsafe {
            (*self.build).str(
              WZR,
              mem(
                R_BASE,
                store_reg * size_of::<TValue>() as i32 + core::mem::offset_of!(TValue, tt) as i32,
              ),
            );
          }
        }

        unsafe { (&mut *self.function).materialize_restore_location(target_inst_idx) };
      }

      // When checking if value has a restore operation to spill it, we only allow it in the same block.
      // Instead of spilling the register to stack, we can reload it from VM stack/constants.
      // We still need to record the spill for restore(start) to work.
      let s = Spill {
        inst: target_inst_idx,
        origin: unsafe { (*def).reg_a64 },
        slot: K_NO_SPILL_SLOT,
      };
      self.spills.push(s);

      unsafe { (*def).needs_reload = true };

      if !self.stats.is_null() {
        unsafe { (*self.stats).spills_to_restore += 1 };
      }
    } else {
      let mut slot = unsafe { alloc_spill(&mut self.free_spill_slots, (*def).reg_a64.kind()) };
      if slot < 0 {
        slot = K_INVALID_SPILL as i32;
        self.error = true;
      }

      if self.is_extra_spill_slot(slot as u32) {
        let extra_offset = self.get_extra_spill_address_offset(slot as u32);

        // Tricky situation, no registers left, but need a register to calculate an address.
        // We will try to take x17 unless it's actually the register being spilled.
        let emergency_temp = if unsafe { (*def).reg_a64 } == X17 || unsafe { (*def).reg_a64 } == W17
        {
          X16
        } else {
          X17
        };

        unsafe {
          (*self.build).str(emergency_temp, s_temporary());

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

          (*self.build).str((*def).reg_a64, mem(emergency_temp, extra_offset));

          (*self.build).ldr(emergency_temp, s_temporary());
        }
      } else {
        unsafe {
          (*self.build).str((*def).reg_a64, mem(SP, S_SPILL_AREA_DATA + slot * 8));
        }
      }

      let s = Spill {
        inst: target_inst_idx,
        origin: unsafe { (*def).reg_a64 },
        slot: slot as i8,
      };
      self.spills.push(s);

      unsafe { (*def).spilled = true };

      if !self.stats.is_null() {
        unsafe {
          (*self.stats).spills_to_slot += 1;

          if slot != K_INVALID_SPILL as i32
            && (slot + 1) as u32 > (*self.stats).max_spill_slots_used
          {
            (*self.stats).max_spill_slots_used = (slot + 1) as u32;
          }
        }
      }
    }

    unsafe { (*def).reg_a64 = RegisterA64::NOREG };

    set.free |= 1u32 << reg;
    set.defs[reg as usize] = IrRegAllocA64::K_INVALID_INST_IDX;
  }
}
