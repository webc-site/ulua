use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// base 库 `rawequal` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1、2 号槽均有值否则抛错回退；受保护帧；结果压栈需栈顶留 1 空槽；可触发 GC）。
/// cpp VM/src/lbaselib.cpp:158
pub fn lua_b_rawequal(l: &mut LuaState) -> i32 {
  l.check_any(1);
  l.check_any(2);

  let result = l.raw_equal(1, 2) as i32;
  l.push_boolean(result != 0);
  1
}

lua_lib_fn!(pub fn lua_b_rawequal @ref, lua_b_rawequal_arm);
