use ulua_common::fflag;
use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int setFunctionReturns(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1402`）。
use crate::{
  functions::{
    create_function::get_type_pack_runtime,
    get_mutable_type_function_runtime::get_mutable_type_function_type_id,
    get_type_user_data::get_type_user_data,
  },
  macros::{lua_check_args, lua_check_not_frozen, lua_check_tag},
  records::type_function_function_type::TypeFunctionFunctionType,
};
use crate::{
  functions::{get_tag::get_tag, throw_type_error::throw_type_error},
  type_aliases::type_function_type_id::AsTypeFunctionType,
};
pub(crate) fn set_function_returns(l: &mut LuaState) -> i32 {
  lua_check_args!(
    l,
    2..=3,
    "type.setreturns: expected 1-3 arguments, but got {}"
  );

  let self_ty = get_type_user_data(l, 1);
  let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
  lua_check_tag!(
    l,
    tfft.is_none(),
    self_ty,
    "type.setreturns: expected self to be a function, but got {} instead"
  );

  lua_check_not_frozen!(l, self_ty, "type.setreturns");

  // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
  let tfft = tfft.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
  tfft.ret_types = get_type_pack_runtime(l, 2, 3);

  0
}
