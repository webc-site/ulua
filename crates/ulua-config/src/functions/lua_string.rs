use alloc::string::String;
use core::{ffi::CStr, ptr::null_mut};

use ulua_vm::{functions::lua_tolstring::lua_tolstring, type_aliases::lua_state::lua_State};

/// 读取栈上字符串（对应 C++ `lua_tostring`），共享辅助。
///
/// # Safety
/// `l` 必须是有效 VM 状态，`index` 处需为字符串或可转换的数字。
pub(crate) unsafe fn lua_string(l: *mut lua_State, index: i32) -> String {
  // SAFETY: 调用方保证 l 有效且 index 处可转换为字符串
  let ptr = unsafe { lua_tolstring(l, index, null_mut()) };
  if ptr.is_null() {
    String::new()
  } else {
    // SAFETY: lua_tolstring 返回 NUL 结尾字符串指针
    unsafe { CStr::from_ptr(ptr) }
      .to_string_lossy()
      .into_owned()
  }
}
