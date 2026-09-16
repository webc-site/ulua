use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_pushboolean::lua_pushboolean},
  records::lua_state,
};

use crate::{
  functions::{
    get_tag::get_tag, get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::type_function_generic_type::TypeFunctionGenericType,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_generic_is_pack(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let self_ty = get_type_user_data(l, 1);
    let tfgt = get_type_function_type_id::<TypeFunctionGenericType>(self_ty);

    if tfgt.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.ispack: expected self to be a generic, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    lua_pushboolean(vm_l, (*tfgt).is_pack as c_int);
    1
  }
}
