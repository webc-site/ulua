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
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn read_table_prop(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.readproperty: expected 2 arguments, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if tftt.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.readproperty: expected self to be either a table, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    let key = get_type_user_data(l, 2);
    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
    if tfst.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.readproperty: expected to be given a singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    let tfsst = (*tfst).variant.get_if_1();
    if tfsst.is_none() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.readproperty: expected to be given a string singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    let key_name = &tfsst.unwrap().value;
    let prop = (*tftt).props.get(key_name);
    if prop.is_none() {
      lua_pushnil(vm_l);
      return 1;
    }

    if let Some(read_ty) = prop.unwrap().read_ty {
      alloc_type_user_data(l, (*read_ty).type_variant.clone(), false);
    } else {
      lua_pushnil(vm_l);
    }

    1
  }
}
