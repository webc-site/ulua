use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l,
    lua_rawseti::lua_rawseti,
  },
  records::lua_state,
};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime_alt_o::get_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  records::{
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_union_type::TypeFunctionUnionType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_components(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.components: expected 1 argument, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);

    let tfut = get_type_function_type_id::<TypeFunctionUnionType>(self_ty);
    if !tfut.is_null() {
      let arg_size = (*tfut).components.len() as i32;

      lua_createtable(vm_l, arg_size, 0);
      for i in 0..arg_size {
        let component = {
          let c = &(*tfut).components;
          c[i as usize]
        };
        alloc_type_user_data(l, (*component).type_variant.clone(), false);
        lua_rawseti(vm_l, -2, i + 1);
      }

      return 1;
    }

    let tfit = get_type_function_type_id::<TypeFunctionIntersectionType>(self_ty);
    if !tfit.is_null() {
      let arg_size = (*tfit).components.len() as i32;

      lua_createtable(vm_l, arg_size, 0);
      for i in 0..arg_size {
        let component = {
          let c = &(*tfit).components;
          c[i as usize]
        };
        alloc_type_user_data(l, (*component).type_variant.clone(), false);
        lua_rawseti(vm_l, -2, i + 1);
      }

      return 1;
    }

    let tag = get_tag(l, self_ty);
    lua_l_error_l(
      vm_l,
      c"%s".as_ptr(),
      core::format_args!("type.components: cannot call components of `{}` type", tag),
    );
    0
  }
}
