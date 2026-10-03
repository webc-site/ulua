

use crate::functions::get_tag::get_tag;
use crate::functions::throw_type_error::throw_type_error;
use ulua_common::fflag;
/// 对应 C++ 原生 `static int setFunctionParameters(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1363`）。
use crate::{functions::{create_function::get_type_pack_runtime, get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_type_user_data::get_type_user_data}, macros::{lua_check_args, lua_check_not_frozen, lua_check_tag}, records::type_function_function_type::TypeFunctionFunctionType};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn set_function_parameters(l: &mut LuaState) -> i32 {
  unsafe {
    lua_check_args!(l, 1..=3, "type.setparameters: expected 1-3, but got {}");

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    lua_check_tag!(
      l,
      tfft.is_null(),
      self_ty,
      "type.setparameters: expected self to be a function, but got {} instead"
    );

    lua_check_not_frozen!(l, self_ty, "type.setparameters");

    (*tfft).arg_types = get_type_pack_runtime(l, 2, 3);

    0
  }
}
