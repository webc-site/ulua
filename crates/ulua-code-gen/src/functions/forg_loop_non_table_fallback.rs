use ulua_vm::{
  functions::lua_d_performcally::lua_d_performcally,
  macros::{setobj_2_s::setobj_2_s as setobj2s, ttisnil::ttisnil},
  records::lua_state::lua_State,
  type_aliases::t_value::TValue,
};

use crate::macros::vm_reg::VM_REG;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn forg_loop_non_table_fallback(l: *mut lua_State, insn_a: i32, aux: i32) -> i32 {
  unsafe {
    let l_ptr = l;
    let base: *mut TValue = (*l_ptr).base;
    let mut ra: *mut TValue = VM_REG!(insn_a, l_ptr, base);

    // note: it's safe to push arguments past top for complicated reasons (see lvmexecute.cpp)
    setobj2s!(l_ptr, ra.add(3 + 2), ra.add(2));
    setobj2s!(l_ptr, ra.add(3 + 1), ra.add(1));
    setobj2s!(l_ptr, ra.add(3), ra);

    (*l_ptr).top = ra.add(3 + 3); // func + 2 args (state and index)
    ulua_common::LUAU_ASSERT!((*l_ptr).top <= (*l_ptr).stack_last);

    if lua_d_performcally(l_ptr, ra.add(3), aux as u8 as i32) {
      return -1; // yield/break, caller must exit native execution
    }

    (*l_ptr).top = (*(*l_ptr).ci).top;

    // recompute ra since stack might have been reallocated
    let base = (*l_ptr).base;
    ra = VM_REG!(insn_a, l_ptr, base);

    // copy first variable back into the iteration index
    setobj2s!(l_ptr, ra.add(2), ra.add(3));

    if ttisnil!(ra.add(3)) { 0 } else { 1 }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_forgLoopNonTableFallback")]
pub unsafe extern "C-unwind" fn forg_loop_non_table_fallback_export(
  l: *mut lua_State,
  insn_a: i32,
  aux: i32,
) -> i32 {
  unsafe { forg_loop_non_table_fallback(l, insn_a, aux) }
}
