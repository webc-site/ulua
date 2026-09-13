use ulua_common::macros::{
  luau_assert::LUAU_ASSERT, luau_insn_a::LUAU_INSN_A, luau_insn_c::LUAU_INSN_C,
};
use ulua_vm::{
  functions::lua_v_gettable::lua_v_gettable,
  macros::{clvalue::clvalue, sethvalue::sethvalue, ttisstring::ttisstring},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{vm_patch_c::vm_patch_c, vm_protect::vm_protect, vm_reg::VM_REG},
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_getglobal(
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
    let aux = *pc_ptr;
    pc_ptr = pc_ptr.add(1);

    let kv = (k as *const TValue).add(aux as usize);
    LUAU_ASSERT!(ttisstring!(kv));

    // fast-path should already have been checked, so we skip checking for it here
    let h = (*cl).env;
    let slot = (LUAU_INSN_C(insn) as i32) & (*h).nodemask8 as i32;

    // slow-path, may invoke Lua calls via __index metamethod
    let mut g = TValue::default();
    sethvalue!(l_ptr, &mut g, h);
    (*l_ptr).cachedslot = slot;
    vm_protect!(l_ptr, pc_ptr, {
      lua_v_gettable(l_ptr, &g as *const TValue, kv as *mut TValue, ra);
    });

    // save cachedslot to accelerate future lookups; patches currently executing instruction since pc-2 rolls back two pc++
    vm_patch_c(pc_ptr.offset(-2), (*l_ptr).cachedslot);

    pc_ptr
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeGETGLOBAL")]
pub unsafe extern "C-unwind" fn execute_getglobal_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_getglobal(l, pc, base, k) }
}
