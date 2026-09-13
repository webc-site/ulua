use core::ffi::{c_char, c_int};

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l, lua_pushboolean::lua_pushboolean,
    lua_pushlstring::lua_pushlstring, lua_pushnil::lua_pushnil,
  },
  records::lua_state,
};

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    get_tag::get_tag, get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::{
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_singleton_value(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.value: expected 1 argument, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfpt = get_type_function_type_id::<TypeFunctionPrimitiveType>(self_ty);
    if !tfpt.is_null() {
      if (*tfpt).r#type != Type::NilType {
        lua_l_error_l(
          vm_l,
          c"%s".as_ptr(),
          core::format_args!(
            "type.value: expected self to be a singleton, but got {} instead",
            get_tag(l, self_ty)
          ),
        );
      }

      lua_pushnil(vm_l);
      return 1;
    }

    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(self_ty);
    if tfst.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.value: expected self to be a singleton, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    if let Some(tfbst) = (*tfst).variant.get_if::<TypeFunctionBooleanSingleton>() {
      lua_pushboolean(vm_l, tfbst.value as c_int);
      return 1;
    }

    if let Some(tfsst) = (*tfst).variant.get_if::<TypeFunctionStringSingleton>() {
      let s = &tfsst.value;
      lua_pushlstring(vm_l, s.as_ptr() as *const c_char, s.len());
      return 1;
    }

    lua_l_error_l(
      vm_l,
      c"%s".as_ptr(),
      core::format_args!(
        "type.value: can't call `value` method on `{}` type",
        get_tag(l, self_ty)
      ),
    );
    0
  }
}
