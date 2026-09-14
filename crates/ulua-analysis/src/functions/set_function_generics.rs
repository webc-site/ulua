use core::ffi::c_int;

use ulua_common::FFlag;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l},
  records::lua_state,
};

use crate::{
  functions::{
    get_generics::get_generics,
    get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_user_data::get_type_user_data,
  },
  records::type_function_function_type::TypeFunctionFunctionType,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn set_function_generics(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let self_ty = get_type_user_data(l, 1);
    let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);

    if tfft.is_null() {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setgenerics: expected self to be a function, but got {} instead",
          get_tag(l, self_ty)
        ),
      );
    }

    if FFlag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setgenerics: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
        ),
      );
    }

    let argument_count = lua_gettop(vm_l);

    if FFlag::LuauTypeFunctionRobustness.get() {
      if argument_count > 2 {
        lua_l_error_l(
          vm_l,
          c"%s".as_ptr(),
          core::format_args!(
            "type.setgenerics: expected 2 arguments, but got {}",
            argument_count
          ),
        );
      }
    } else if argument_count > 3 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.setgenerics: expected 3 arguments, but got {}",
          argument_count
        ),
      );
    }

    let (generic_types, generic_packs) = get_generics(l, 2, "types.setgenerics");

    (*tfft).generics = generic_types;
    (*tfft).generic_packs = generic_packs;

    0
  }
}
