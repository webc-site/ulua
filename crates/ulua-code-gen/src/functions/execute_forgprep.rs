//! Source: `CodeGen/src/CodeGenUtils.cpp`

use core::ptr::null_mut;

use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_d::luau_insn_d};
use ulua_vm::{
  enums::tms::TMS,
  functions::{lua_d_call::lua_d_call, lua_g_typeerror_l::lua_g_typeerror_l},
  macros::{
    clvalue::clvalue, fasttm::fasttm, hvalue::hvalue, lu_tag_iterator::LU_TAG_ITERATOR,
    setnilvalue::setnilvalue, setobj_2_s::setobj2s, setpvalue::setpvalue,
    ttisfunction::ttisfunction, ttisnil::ttisnil, ttistable::ttistable, ttisuserdata::ttisuserdata,
    uvalue::uvalue,
  },
  records::lua_table::LuaTable,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{vm_protect::vm_protect, vm_protect_pc::vm_protect_pc, vm_reg::VM_REG},
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_forgprep(
  l: *mut lua_State,
  pc: *const Instruction,
  mut base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  unsafe {
    let _cl = clvalue!((*(*l).ci).func);

    let mut pc = pc;
    let insn = *pc;
    pc = pc.add(1);

    let mut ra = VM_REG!(luau_insn_a(insn) as i32, l, base) as *mut TValue;

    if ttisfunction!(ra) {
      // will be called during FORGLOOP
    } else {
      let mt: *mut LuaTable = if ttistable!(ra) {
        (*hvalue!(ra)).metatable
      } else if ttisuserdata!(ra) {
        uvalue!(ra as *const TValue).metatable
      } else {
        null_mut()
      };

      let fn_iter = fasttm(l, mt, TMS::TmIter as i32);

      if !fn_iter.is_null() {
        setobj2s!(l, ra.add(1), ra);
        setobj2s!(l, ra, fn_iter);

        (*l).top = ra.add(2); // func + self arg

        vm_protect!(l, pc, base, lua_d_call_protected(l, ra));
        (*l).top = (*(*l).ci).top;

        // recompute ra since stack might have been reallocated
        ra = VM_REG!(luau_insn_a(insn) as i32, l, base) as *mut TValue;

        // protect against __iter returning nil
        if ttisnil!(ra) {
          vm_protect_pc(l, pc);
          lua_g_typeerror_l(l, ra as *const TValue, "call");
        }
      } else if !fasttm(l, mt, TMS::TmCall as i32).is_null() {
        // table or userdata with __call, will be called during FORGLOOP
      } else if ttistable!(ra) {
        // set up registers for builtin iteration
        setobj2s!(l, ra.add(1), ra);
        setpvalue!(ra.add(2), null_mut(), LU_TAG_ITERATOR);
        setnilvalue!(ra);
      } else {
        vm_protect_pc(l, pc);
        lua_g_typeerror_l(l, ra as *const TValue, "iterate over");
      }
    }

    pc = pc.offset(luau_insn_d(insn) as isize);
    pc
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
unsafe fn lua_d_call_protected(l: *mut lua_State, ra: StkId) {
  unsafe {
    lua_d_call(l, ra, 3);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeFORGPREP")]
pub unsafe extern "C-unwind" fn execute_forgprep_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_forgprep(l, pc, base, k) }
}
