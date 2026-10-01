use crate::{
  enums::lua_type::LuaType,
  macros::{lua_lib_fn::lua_lib_fn, lua_upvalueindex::lua_upvalueindex},
  records::lua_state::LuaState,
};

/// base 库 `ipairs` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽为 table 否则抛错；upvalue 1 为迭代 C 函数——注册表绑定 ipairs 时由
/// `luaL_register` 约定成立；受保护帧、栈顶留 3 空槽；push 可触发 GC）。
/// cpp VM/src/lbaselib.cpp:248
pub fn lua_b_ipairs(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  l.push_value(lua_upvalueindex(1));
  l.push_value(1);
  l.push_integer(0);
  3
}

lua_lib_fn!(pub fn lua_b_ipairs @ref, lua_b_ipairs_arm);
