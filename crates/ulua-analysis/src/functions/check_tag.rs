use core::ffi::{CStr, c_int};

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l, lua_pushboolean::lua_pushboolean,
  },
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state,
};

use crate::{
  functions::{get_tag::get_tag, get_type_user_data::get_type_user_data},
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn check_tag(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!("type.is: expected 2 arguments, but got {}", argument_count),
      );
    }

    let self_ = get_type_user_data(l, 1);
    let arg = luaL_checkstring!(vm_l, 2);
    let arg_str = CStr::from_ptr(arg).to_string_lossy();

    let tag = get_tag(l, self_);
    lua_pushboolean(vm_l, if tag == arg_str { 1 } else { 0 });
    1
  }
}
