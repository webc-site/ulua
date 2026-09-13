use core::ffi::c_int;

use ulua_vm::{functions::lua_l_error_l::lua_l_error_l, records::lua_state};

use crate::type_aliases::lua_state::LuaState;
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn set_table_write_indexer(l: *mut LuaState) -> c_int {
  unsafe {
    let msg =
      "type.setwriteindexer: luau does not yet support separate read/write types for indexers.";
    let fmt = c"%s";
    lua_l_error_l(
      l as *mut lua_state::LuaState,
      fmt.as_ptr(),
      core::format_args!("{}", msg),
    );
    0
  }
}
