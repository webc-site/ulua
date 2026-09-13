use ulua_common::FFlag;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  macros::lua_isnil::lua_isnil,
  records::lua_state,
};

use crate::{
  functions::{
    get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::{
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn set_table_prop(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if !(2..=3).contains(&argument_count) {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setproperty: expected 2-3 arguments, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if tftt.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setproperty: expected self to be a table, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    if FFlag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setproperty: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
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
          "type.setproperty: expected to be given a singleton type, but got {} instead",
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
          "type.setproperty: expected to be given a string singleton type, but got {} instead",
          get_tag(l, key)
        ),
      );
    }

    let key_name = tfsst.unwrap().value.clone();

    if argument_count == 2 || lua_isnil!(vm_l, 3) {
      (*tftt).props.remove(&key_name);
      return 0;
    }

    let value = get_type_user_data(l, 3);
    (*tftt).props.insert(
      key_name,
      TypeFunctionProperty {
        read_ty: Some(value),
        write_ty: Some(value),
      },
    );

    0
  }
}
