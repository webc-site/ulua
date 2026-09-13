use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l, lua_pushboolean::lua_pushboolean,
  },
  records::lua_state,
};

use crate::{functions::get_type_user_data::get_type_user_data, type_aliases::lua_state::LuaState};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn is_equal_to_type(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!("expected 2 arguments, but got {}", argument_count),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    lua_pushboolean(vm_l, (*self_ty).operator_eq(&*arg) as c_int);
    1
  }
}
