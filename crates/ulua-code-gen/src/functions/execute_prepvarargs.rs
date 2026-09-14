use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_insn_a::LUAU_INSN_A};
use ulua_vm::{
  functions::lua_d_growstack::lua_d_growstack,
  macros::{
    cast_int::cast_int, clvalue::clvalue, setnilvalue::setnilvalue,
    setobj_2_s::setobj_2_s as setobj2s, stacklimitreached::stacklimitreached,
  },
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::vm_protect::vm_protect as VM_PROTECT,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_prepvarargs(
  l: *mut lua_State,
  pc: *const Instruction,
  // FFI 签名参数:传入的 base 值在 VM_PROTECT 覆盖前不使用,故命名带下划线
  _base: StkId,
  _k: *mut TValue,
) -> *const Instruction {
  unsafe {
    let l_ptr = l;
    let cl = clvalue!((*(*l_ptr).ci).func);
    let base: StkId;
    let mut pc_ptr = pc;
    let insn = *pc_ptr;
    pc_ptr = pc_ptr.add(1);
    let numparams = LUAU_INSN_A(insn) as i32;

    // all fixed parameters are copied after the top so we need more stack space
    VM_PROTECT!(l_ptr, pc_ptr, base, {
      let n = (*cl).stacksize as i32 + numparams;
      if stacklimitreached(l_ptr, n) {
        lua_d_growstack(l_ptr, n);
      } else {
        // condhardstacktests expansion:
        // In Luau, condhardstacktests is usually gated by a debug flag or internal check.
        // Since LuauCheckStackVariable was not found, we use the standard VM logic
        // which often omits the hard stack test in production or uses a different flag.
        // We'll follow the luaD_checkstack macro logic but skip the missing FFlag.
      }
    });

    LUAU_ASSERT!(cast_int!((*l_ptr).top.offset_from(base)) >= numparams);

    // move fixed parameters to final position
    let fixed = base; // first fixed argument
    let new_base = (*l_ptr).top; // final position of first argument

    for i in 0..numparams {
      setobj2s!(l_ptr, new_base.add(i as usize), fixed.add(i as usize));
      setnilvalue!(fixed.add(i as usize));
    }

    // rewire our stack frame to point to the new base
    (*(*l_ptr).ci).base = new_base;
    (*(*l_ptr).ci).top = new_base.add((*cl).stacksize as usize);

    (*l_ptr).base = new_base;
    (*l_ptr).top = (*(*l_ptr).ci).top;

    pc_ptr
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executePREPVARARGS")]
pub unsafe extern "C-unwind" fn execute_prepvarargs_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_prepvarargs(l, pc, base, k) }
}
