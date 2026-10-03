

use crate::functions::get_tag::get_tag;
use crate::functions::throw_type_error::throw_type_error;
/// 对应 C++ 原生 `static int setFunctionGenerics(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1440`）。
use ulua_common::fflag;
use crate::{functions::{get_generics::get_generics, get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_type_user_data::get_type_user_data}, macros::{lua_check_args, lua_check_not_frozen, lua_check_tag}, records::type_function_function_type::TypeFunctionFunctionType};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn set_function_generics(l: &mut LuaState) -> i32 {
  unsafe {
    let self_ty = get_type_user_data(l, 1);
    let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);

    lua_check_tag!(
      l,
      tfft.is_null(),
      self_ty,
      "type.setgenerics: expected self to be a function, but got {} instead"
    );

    lua_check_not_frozen!(l, self_ty, "type.setgenerics");

    if fflag::LuauTypeFunctionRobustness.get() {
      lua_check_args!(l, > 2, "type.setgenerics: expected 2 arguments, but got {}");
    } else {
      lua_check_args!(l, > 3, "type.setgenerics: expected 3 arguments, but got {}");
    }

    let (generic_types, generic_packs) = get_generics(l, 2, "types.setgenerics");

    (*tfft).generics = generic_types;
    (*tfft).generic_packs = generic_packs;

    0
  }
}
