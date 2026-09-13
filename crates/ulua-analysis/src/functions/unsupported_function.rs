use core::ffi::c_int;

use ulua_vm::{functions::lua_l_error_l::lua_l_error_l, records::lua_state};

use crate::type_aliases::lua_state::LuaState;
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn unsupported_function(l: *mut LuaState) -> c_int {
  unsafe {
    let msg = "this function is not supported in type functions";
    let fmt = c"%s";
    lua_l_error_l(
      l as *mut lua_state::LuaState,
      fmt.as_ptr(),
      core::format_args!("{}", msg),
    );
    0
  }
}
