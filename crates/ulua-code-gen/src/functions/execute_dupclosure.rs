use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B, luau_insn_d::LUAU_INSN_D,
    luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  functions::{lua_f_new_lclosure::lua_f_new_lclosure, lua_o_rawequal_obj::luaO_rawequalObj},
  macros::{
    clvalue::clvalue, lua_c_barrier::luaC_barrier, lua_c_check_gc::luaC_checkGC,
    setclvalue::setclvalue, setobj::setobj,
  },
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{vm_protect::vm_protect, vm_protect_pc::vm_protect_pc, vm_reg::VM_REG},
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_dupclosure(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe {
    let l_ptr = l;
    let cl = clvalue!((*(*l_ptr).ci).func);

    let mut pc_ptr = pc;
    let insn = *pc_ptr;
    pc_ptr = pc_ptr.add(1);
    let ra = VM_REG!(LUAU_INSN_A(insn) as i32, l_ptr, base);
    let kv = k.add(LUAU_INSN_D(insn) as usize);

    let kcl = clvalue!(kv as *const TValue);

    vm_protect_pc(l_ptr, pc_ptr);

    let mut ncl = if (*kcl).env == (*cl).env {
      kcl
    } else {
      let kp = {
        let l = &(*kcl).inner.l;
        l.p
      };
      lua_f_new_lclosure(l_ptr, (*kcl).nupvalues as i32, (*cl).env, kp)
    };
    setclvalue!(l_ptr, ra, ncl);

    let mut ui: i32 = 0;
    while ui < (*kcl).nupvalues as i32 {
      let uinsn = *pc_ptr.add(ui as usize);
      ulua_common::LUAU_ASSERT!(LUAU_INSN_OP(uinsn) == LuauOpcode::LOP_CAPTURE as u32);
      ulua_common::LUAU_ASSERT!(
        LUAU_INSN_A(uinsn) == LuauCaptureType::LCT_VAL as u32
          || LUAU_INSN_A(uinsn) == LuauCaptureType::LCT_UPVAL as u32
      );

      let uv: *mut TValue = if LUAU_INSN_A(uinsn) == LuauCaptureType::LCT_VAL as u32 {
        VM_REG!(LUAU_INSN_B(uinsn), l_ptr, base) as *mut TValue
      } else {
        let l = &mut (*cl).inner.l;
        l.uprefs.as_mut_ptr().add(LUAU_INSN_B(uinsn) as usize)
      };

      let uref = {
        let l = &mut (*ncl).inner.l;
        l.uprefs.as_mut_ptr().add(ui as usize)
      };

      if ncl == kcl && luaO_rawequalObj(uref as *const TValue, uv as *const TValue) != 0 {
        ui += 1;
        continue;
      }

      if ncl == kcl && (*kcl).preload == 0 {
        let kp = {
          let l = &(*kcl).inner.l;
          l.p
        };
        ncl = lua_f_new_lclosure(l_ptr, (*kcl).nupvalues as i32, (*cl).env, kp);
        setclvalue!(l_ptr, ra, ncl);

        ui = 0;
        continue;
      }

      setobj!(l_ptr, uref, uv as *const TValue);
      luaC_barrier!(l_ptr, ncl, uv as *const TValue);
      ui += 1;
    }

    (*ncl).preload = 0;

    if kcl != ncl {
      vm_protect!(l_ptr, pc_ptr, {
        luaC_checkGC!(l_ptr);
      });
    }

    pc_ptr.add((*kcl).nupvalues as usize)
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeDUPCLOSURE")]
pub unsafe extern "C-unwind" fn execute_dupclosure_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_dupclosure(l, pc, base, k) }
}
