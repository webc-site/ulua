use core::ffi::c_int;

use ulua_common::FFlag;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, deep_clone::deep_clone,
    get_type_function_runtime::get_type_function_runtime, get_type_user_data::get_type_user_data,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn deep_copy(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "types.copy: expected 1 arguments, but got {}",
          argument_count
        ),
      );
    }

    let arg = get_type_user_data(l, 1);
    let runtime = get_type_function_runtime(l);
    let copy = deep_clone(runtime, arg);

    if FFlag::LuauTypeFunctionRobustness.get() && copy.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!("types.copy: complexity limit reached during type copy"),
      );
    }

    alloc_type_user_data(l, (*copy).type_variant.clone(), false);
    1
  }
}
