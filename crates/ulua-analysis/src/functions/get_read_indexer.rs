use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_createtable::lua_createtable, lua_gettop::lua_gettop, lua_l_error_l::lua_l_error_l,
    lua_pushnil::lua_pushnil, lua_setfield::lua_setfield,
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
    type_function_extern_type::TypeFunctionExternType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_read_indexer(l: *mut LuaState) -> c_int {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
      lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
          "type.readindexer: expected 1 arguments, but got {}",
          argument_count
        ),
      );
    }

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tftt.is_null() {
      if (*tftt).indexer.is_none() {
        lua_pushnil(vm_l);
      } else {
        lua_createtable(vm_l, 0, 2);
        let indexer = (*tftt).indexer.as_ref().unwrap();
        alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, c"index".as_ptr());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, c"result".as_ptr());
      }
      return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
      if (*tfct).indexer.is_none() {
        lua_pushnil(vm_l);
      } else {
        lua_createtable(vm_l, 0, 2);
        let indexer = (*tfct).indexer.as_ref().unwrap();
        alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, c"index".as_ptr());
        alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
        lua_setfield(vm_l, -2, c"result".as_ptr());
      }
      return 1;
    }

    lua_l_error_l(
      vm_l,
      c"%s".as_ptr(),
      core::format_args!(
        "type.readindexer: expected self to be either a table or class, but got {} instead",
        get_tag(l, self_ty)
      ),
    );
    0
  }
}
