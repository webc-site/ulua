use core::ffi::{CStr, c_int};

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_l_optboolean::lua_l_optboolean},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state,
};

use crate::{
  functions::alloc_type_user_data::alloc_type_user_data,
  records::type_function_generic_type::TypeFunctionGenericType,
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_generic(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let name_ptr = luaL_checkstring!(vm_l, 1);
    let is_pack = lua_l_optboolean(vm_l, 2, false);

    let name_cstr = CStr::from_ptr(name_ptr);
    if name_cstr.to_bytes().is_empty() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!("types.generic: generic name cannot be empty"),
      );
    }

    let generic_type = TypeFunctionGenericType {
      is_named: true,
      is_pack,
      name: name_cstr.to_string_lossy().into_owned(),
    };

    alloc_type_user_data(l, TypeFunctionTypeVariant::Generic(generic_type), false);

    1
  }
}
