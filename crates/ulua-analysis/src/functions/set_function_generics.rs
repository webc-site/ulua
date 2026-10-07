/// 对应 C++ 原生 `static int setFunctionGenerics(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1440`）。
use ulua_common::fflag;
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    get_generics::get_generics,
    get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_user_data::get_type_user_data, throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_not_frozen, lua_check_tag},
  records::type_function_function_type::TypeFunctionFunctionType,
  type_aliases::type_function_type_id::AsTypeFunctionType,
};
pub(crate) fn set_function_generics(l: &mut LuaState) -> i32 {
  let self_ty = get_type_user_data(l, 1);
  let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);

  lua_check_tag!(
    l,
    tfft.is_none(),
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

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tfft = tfft.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
  tfft.generics = generic_types;
  tfft.generic_packs = generic_packs;

  0
}
