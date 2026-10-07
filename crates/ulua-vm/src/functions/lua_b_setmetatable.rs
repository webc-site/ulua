//! Source: `VM/src/lbaselib.cpp:75-85` (hand-ported)

use crate::{
  enums::lua_type::LuaType,
  functions::lua_l_getmetafield::lua_l_getmetafield_bytes,
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`luaL_error!`（终端
/// `lua_l_error_l`）已随 wave-6d 降为引用形安全门面，本函数体无剩余内存安全前提，
/// 屏障仅由 `@ref` 臂的调用序契约保留；`lua_l_getmetafield_bytes` 已随 r16-v43 收形为引用形；`l` 仍须处于受保护帧：栈 1 号位须为 table
/// （`check_type`）、2 号位须为 nil/table（`arg_expected`），`lua_l_getmetafield` 检出
/// `__metatable` 时经 `lua_l_error_l` 抛错，否则 `set_metatable` 写元表并可 GC。
/// cpp/VM/src/lbaselib.cpp:100 luaB_setmetatable。
pub(crate) unsafe fn lua_b_setmetatable(l: &mut LuaState) -> i32 {
  let t = l.type_of(2);
  l.check_type(1, LuaType::Table);
  l.arg_expected(t == LuaType::Nil || t == LuaType::Table, 2, "nil or table");
  if lua_l_getmetafield_bytes(&mut *l, 1, b"__metatable") != 0 {
    luaL_error!(l, "cannot change a protected metatable");
  }
  l.set_top(2);
  l.set_metatable(1);
  1
}

lua_lib_fn!(pub(crate) fn lua_b_setmetatable @ref, lua_b_setmetatable_arm);
