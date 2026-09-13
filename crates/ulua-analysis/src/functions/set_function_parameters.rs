use core::ffi::c_int;

use ulua_common::FFlag;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  records::lua_state,
};

use crate::{
  functions::{
    create_function::get_type_pack_runtime,
    get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_user_data::get_type_user_data,
  },
  records::type_function_function_type::TypeFunctionFunctionType,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn set_function_parameters(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if !(1..=3).contains(&argument_count) {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setparameters: expected 1-3, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    if tfft.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setparameters: expected self to be a function, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    if FFlag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setparameters: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
        ),
      );
    }

    (*tfft).arg_types = get_type_pack_runtime(l, 2, 3);

    0
  }
}
