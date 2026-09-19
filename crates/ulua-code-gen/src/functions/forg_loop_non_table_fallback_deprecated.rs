use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::lua_d_call::lua_d_call,
  macros::{setobj_2_s::setobj_2_s, ttisnil::ttisnil},
  records::lua_state::lua_State,
  type_aliases::t_value::TValue,
};

use crate::macros::vm_reg::VM_REG;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn forg_loop_non_table_fallback_deprecated(
  l: *mut lua_State,
  insn_a: i32,
  aux: i32,
) -> bool {
  unsafe {
    let l_ptr = l;

    let base: *mut TValue = (*l_ptr).base;
    let mut ra: *mut TValue = VM_REG!(insn_a, l_ptr, base);

    // note: it's safe to push arguments past top for complicated reasons (see lvmexecute.cpp)
    setobj_2_s!(l_ptr, ra.add(3 + 2), ra.add(2));
    setobj_2_s!(l_ptr, ra.add(3 + 1), ra.add(1));
    setobj_2_s!(l_ptr, ra.add(3), ra);

    (*l_ptr).top = ra.add(3 + 3); // func + 2 args (state and index)
    LUAU_ASSERT!((*l_ptr).top <= (*l_ptr).stack_last);

    lua_d_call(l_ptr, ra.add(3), aux as u8 as i32);
    (*l_ptr).top = (*(*l_ptr).ci).top;

    // recompute ra since stack might have been reallocated
    let base = (*l_ptr).base;
    ra = VM_REG!(insn_a, l_ptr, base);

    // copy first variable back into the iteration index
    setobj_2_s!(l_ptr, ra.add(2), ra.add(3));

    !ttisnil!(ra.add(3))
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_forgLoopNonTableFallback_DEPRECATED")]
pub unsafe extern "C-unwind" fn forg_loop_non_table_fallback_deprecated_export(
  l: *mut lua_State,
  insn_a: i32,
  aux: i32,
) -> bool {
  unsafe { forg_loop_non_table_fallback_deprecated(l, insn_a, aux) }
}
