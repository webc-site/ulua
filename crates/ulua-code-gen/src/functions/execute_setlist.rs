use core::ptr::null;

use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
};
use ulua_vm::{
  functions::lua_h_resizearray::lua_h_resizearray,
  macros::{
    clvalue::clvalue, hvalue::hvalue, lua_c_barrierfast::lua_c_barrierfast,
    lua_multret::LUA_MULTRET, setobj_2_t::setobj2t, ttistable::ttistable,
  },
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{vm_protect_pc::vm_protect_pc, vm_reg::VM_REG},
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_setlist(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  unsafe {
    let l_ptr = l;
    let cl = clvalue!((*(*l_ptr).ci).func);
    let _ = cl; // unused in this function

    let mut pc_ptr = pc;
    let insn = *pc_ptr;
    pc_ptr = pc_ptr.add(1);

    let ra = VM_REG!(luau_insn_a(insn) as i32, l_ptr, base);
    // note: this can point to l->top if c == LUA_MULTRET making VM_REG unsafe to use
    let rb = base.add(luau_insn_b(insn) as usize);
    let mut c = (luau_insn_c(insn) as i32) - 1;
    let index = *pc_ptr;
    pc_ptr = pc_ptr.add(1);

    if c == LUA_MULTRET {
      c = (*l_ptr).top.offset_from(rb) as i32;
      (*l_ptr).top = (*(*l_ptr).ci).top;
    }

    let h = hvalue!(ra as *const TValue);

    // TODO: we really don't need this anymore
    if !ttistable!(ra as *const TValue) {
      return null(); // temporary workaround to weaken a rather powerful exploitation primitive in case of a MITM attack on bytecode
    }

    let last = index as i32 + c - 1;
    if last > (*h).sizearray {
      // vm_protect_pc expects the local crate's lua_State pointer
      vm_protect_pc(l, pc_ptr); // luaH_resizearray may fail due to OOM

      lua_h_resizearray(l_ptr, h, last);
    }

    let array = (*h).array;

    // 逐槽拷贝 `setobj2t(L, &array[index + i - 1], rb + i)`：两端指针递增，
    // 省去每轮重复的 index+i-1 地址算术（c 为 0 时不写入）
    let mut dst = array.add((index as i32 - 1) as usize);
    let mut src = rb;
    for _ in 0..c {
      setobj2t!(l_ptr, dst, src);
      dst = dst.add(1);
      src = src.add(1);
    }

    lua_c_barrierfast!(l_ptr, h);
    pc_ptr
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeSETLIST")]
pub unsafe extern "C-unwind" fn execute_setlist_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_setlist(l, pc, base, k) }
}
