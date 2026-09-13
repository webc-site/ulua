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
  records::type_function_negation_type::TypeFunctionNegationType,
  type_aliases::{lua_state::LuaState, type_function_type_id::TypeFunctionTypeId},
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_negated_value(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.inner: expected 1 argument, but got {}",
          argument_count
        ),
      );
    }

    let self_ty: TypeFunctionTypeId = get_type_user_data(l, 1);
    let tfnt = get_type_function_type_id::<TypeFunctionNegationType>(self_ty);

    if !tfnt.is_null() {
      alloc_type_user_data(l, (*(*tfnt).type_id).type_variant.clone(), false);
    } else {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.inner: cannot call inner method on non-negation type: `{}` type",
          get_tag(l, self_ty)
        ),
      );
    }

    1
  }
}
