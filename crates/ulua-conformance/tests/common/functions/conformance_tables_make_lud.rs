use core::ffi::{c_int, c_void};

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{l_checkunsigned, state_mut, topointer};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tables_make_lud(l: *mut LuaState) -> c_int {
  // `is_number` 只探测参数 1 的类型，不改动栈。
  if state_mut(l).is_number(1) {
    // 参数 1 已确认为数字，按无符号取出（失配按 cpp 抛 Lua 错误）后
    // 把该值本身当地址压成 light userdata。
    let v = l_checkunsigned(l, 1);
    // Safety: `push_lightuserdata` 为 LuaState 的 unsafe 方法（载荷指针只入栈、
    // VM 从不解引用），此处载荷为数值比特样，无存续期耦合。
    unsafe { state_mut(l).push_lightuserdata(v as usize as *mut c_void) };
  } else {
    // 取参数 1 的内部指针（不可转换时为 null，下一行断言拦下）。
    let p = topointer(l, 1);
    assert!(!p.is_null());
    // `p` 已确认非空，按 cpp 原样压为 light userdata。
    // Safety: push_lightuserdata 为 LuaState 的 unsafe 方法（载荷只入栈不解引用）。
    unsafe { state_mut(l).push_lightuserdata(p as *mut c_void) };
  }

  1
}
