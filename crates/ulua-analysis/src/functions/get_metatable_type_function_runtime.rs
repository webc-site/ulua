use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l, lua_pushnil::lua_pushnil},
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::{
    type_function_extern_type::TypeFunctionExternType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_metatable(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.metatable: expected 1 arguments, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);

    let tfmt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tfmt.is_null() {
      if let Some(metatable) = (*tfmt).metatable {
        alloc_type_user_data(l, (*metatable).type_variant.clone(), false);
      } else {
        lua_pushnil(vm_l);
      }
      return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
      if let Some(metatable) = (*tfct).metatable {
        alloc_type_user_data(l, (*metatable).type_variant.clone(), false);
      } else {
        lua_pushnil(vm_l);
      }
      return 1;
    }

    lua_l_error_l(
      vm_l,
      c"%s".as_ptr(),
      core::format_args!(
        "type.metatable: expected self to be a table or class, but got {} instead",
        get_tag(l, self_ty)
      ),
    );
    0
  }
}
