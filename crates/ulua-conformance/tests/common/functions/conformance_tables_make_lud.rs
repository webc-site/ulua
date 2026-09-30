use core::ffi::{c_int, c_void};

use ulua_vm::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_topointer::lua_topointer},
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tables_make_lud(l: *mut LuaState) -> c_int {
  // Safety: `l` 为本用例存活的 LuaState；`is_number` 只探测参数 1 的类型，不改动栈。
  if unsafe { (*l).is_number(1) } {
    // Safety: `l` 存活；参数 1 已确认为数字，按无符号取出（失配按 cpp 抛 Lua 错误）后
    // 把该值本身当地址压成 light userdata。
    unsafe {
      let v = lua_l_checkunsigned(l, 1);
      (*l).push_lightuserdata(v as usize as *mut c_void);
    }
  } else {
    // Safety: `l` 存活；取参数 1 的内部指针（不可转换时为 null，下一行断言拦下）。
    let p = unsafe { lua_topointer(l, 1) };
    assert!(!p.is_null());
    // Safety: `p` 已确认非空，按 cpp 原样压为 light userdata。
    unsafe { (*l).push_lightuserdata(p as *mut c_void) };
  }

  1
}
