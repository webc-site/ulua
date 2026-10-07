use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int checkTag(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1820`）。
use crate::{
  functions::{
    get_tag::get_tag, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
};
pub(crate) fn check_tag(l: &mut LuaState) -> i32 {
  lua_check_args!(l, != 2, "type.is: expected 2 arguments, but got {}");

  let self_ = get_type_user_data(l, 1);
  // cpp `std::string arg = luaL_checkstring(L, 2)` 本就落地拥有值；同样先取字节再
  // 继续，既对齐 oracle，也满足 `check_bytes` 的「持窗期间不得再动 state」契约，
  // 使后续 get_tag/push_boolean 的可变借用互不重叠。
  let arg = l.check_bytes(2).to_vec();

  let tag = get_tag(l, self_);
  l.push_boolean(tag.as_bytes() == arg);
  1
}
