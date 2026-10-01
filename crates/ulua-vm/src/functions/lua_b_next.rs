use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// base 库 `next` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽为 table，不符即抛错；`next` 读写栈顶、可抛错，须在受保护帧内）。
/// cpp `lbaselib.cpp:216`。
pub fn lua_b_next(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  l.set_top(2);

  if l.next(1) {
    2
  } else {
    l.push_nil();
    1
  }
}

lua_lib_fn!(pub fn lua_b_next @ref, lua_b_next_arm);
