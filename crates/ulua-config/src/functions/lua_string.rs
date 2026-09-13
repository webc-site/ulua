use alloc::string::String;
use core::{ffi::CStr, ptr::null_mut};

use ulua_vm::{functions::lua_tolstring::lua_tolstring, type_aliases::lua_state::lua_State};

/// 读取栈上字符串（对应 C++ `lua_tostring`），共享辅助。
///
/// # Safety
/// `l` 必须是有效 VM 状态，`index` 处需为字符串或可转换的数字。
pub(crate) unsafe fn lua_string(l: *mut lua_State, index: i32) -> String {
  unsafe {
    let ptr = lua_tolstring(l, index, null_mut());
    if ptr.is_null() {
      String::new()
    } else {
      CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
  }
}
