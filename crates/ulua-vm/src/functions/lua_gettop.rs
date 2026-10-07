use crate::records::lua_state::LuaState;

/// 读栈深（`lua_gettop`）。`l` 以引用传入（存活由类型保证）；`top`/`base` 为同一栈
/// 数组内的合法槽指针是 `lua_State` 结构不变量（与 [`LuaState::get_top`] 同一前提），
/// 仅做槽距读数、不解引用元素。cpp `lapi.cpp:262`。
pub fn lua_gettop(l: &LuaState) -> i32 {
  l.get_top()
}
