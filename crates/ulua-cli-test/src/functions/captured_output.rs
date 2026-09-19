use alloc::string::String;
use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
};

use ulua_vm::{
  functions::lua_tolstring::lua_tolstring,
  macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop},
  type_aliases::lua_state::lua_State,
};

/// 读取全局 `capturedoutput`（对齐 cpp `getCapturedOutput`，
/// ReplFixture/ReplWithPathFixture 公共实现）。
///
/// # Safety
/// `l` 必须指向已初始化且定义了 `capturedoutput` 全局的 `lua_State`。
pub(crate) unsafe fn captured_output(l: *mut lua_State) -> String {
  unsafe {
    lua_getglobal(l, c"capturedoutput".as_ptr() as *const c_char);
    let str_ptr = lua_tolstring(l, -1, null_mut());
    let result = if str_ptr.is_null() {
      String::new()
    } else {
      CStr::from_ptr(str_ptr).to_string_lossy().into_owned()
    };
    lua_pop(l, 1);
    result
  }
}
