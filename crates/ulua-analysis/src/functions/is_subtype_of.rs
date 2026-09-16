use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean},
  macros::lua_l_error::luaL_error,
  records::lua_state,
};

use crate::{
  functions::{
    deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state,
    get_type_function_runtime::get_type_function_runtime, get_type_user_data::get_type_user_data,
  },
  records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn is_subtype_of(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
      luaL_error!(
        vm_l,
        "type.issubtypeof: expected 2 arguments, but got {}",
        argument_count
      );
    }

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    let runtime = get_type_function_runtime(l);
    let runtime_builder = &mut *(*runtime).runtime_builder;
    let ctx = &*runtime_builder.ctx;

    let sub_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
      self_ty,
      runtime_builder as *mut TypeFunctionRuntimeBuilderState,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
      luaL_error!(vm_l, "failed to deserialize the self type");
    }

    let super_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
      arg,
      runtime_builder as *mut TypeFunctionRuntimeBuilderState,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
      luaL_error!(vm_l, "failed to deserialize the argument type");
    }

    let result = (*ctx.subtyping.as_ptr()).is_subtype_type_id_type_id_not_null_scope(
      sub_ty,
      super_ty,
      ctx.scope.as_ptr(),
    );
    lua_pushboolean(vm_l, result.is_subtype as c_int);
    1
  }
}
