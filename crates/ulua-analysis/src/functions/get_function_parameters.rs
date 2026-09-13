use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  records::lua_state,
};

use crate::{
  functions::{
    get_tag::get_tag, get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data, push_type_pack::push_type_pack,
  },
  records::type_function_function_type::TypeFunctionFunctionType,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_function_parameters(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.parameters: expected 1 arguments, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);

    if tfft.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.parameters: expected self to be a function, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    push_type_pack(l, (*tfft).arg_types);

    1
  }
}
