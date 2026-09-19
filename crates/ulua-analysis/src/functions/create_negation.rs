use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::{
    type_function_function_type::TypeFunctionFunctionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::{lua_state::LuaState, type_function_type_variant::TypeFunctionTypeVariant},
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn create_negation(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "types.negationof: expected 1 argument, but got {}",
          argument_count
        ),
      );
    }

    let arg = get_type_user_data(l, 1);

    let table_type_ptr = get_type_function_type_id::<TypeFunctionTableType>(arg);
    let function_type_ptr = get_type_function_type_id::<TypeFunctionFunctionType>(arg);

    if !table_type_ptr.is_null() || !function_type_ptr.is_null() {
      let tag = get_tag(l, arg);
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "types.negationof: cannot perform negation on `{}` type",
          tag
        ),
      );
    }

    let negation = TypeFunctionNegationType { type_id: arg };
    alloc_type_user_data(l, TypeFunctionTypeVariant::Negation(negation), false);

    1
  }
}
