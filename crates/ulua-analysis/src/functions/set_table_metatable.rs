/// 对应 C++ 原生 `static int setTableMetatable(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1128`）。
use ulua_common::fflag;
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_not_frozen, lua_check_tag},
  records::type_function_table_type::TypeFunctionTableType,
};
pub(crate) fn set_table_metatable(l: &mut LuaState) -> i32 {
  unsafe {
    lua_check_args!(l, != 2, "type.setmetatable: expected 2 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);
    lua_check_tag!(
      l,
      tftt.is_none(),
      self_ty,
      "type.setmetatable: expected self to be a table, but got {} instead"
    );

    lua_check_not_frozen!(l, self_ty, "type.setmetatable");

    // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
    let tftt = tftt.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");

    let arg = get_type_user_data(l, 2);
    if get_type_function_type_id::<TypeFunctionTableType>(arg).is_none() {
      let tag_ty = if fflag::LuauTypeFunctionRobustness.get() {
        arg
      } else {
        self_ty
      };
      let tag = get_tag(l, tag_ty);
      throw_type_error(
        l,
        format_args!(
          "type.setmetatable: expected the argument to be a table, but got {} instead",
          tag
        ),
      );
    }

    tftt.metatable = Some(arg);

    0
  }
}
