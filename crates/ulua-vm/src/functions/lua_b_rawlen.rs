use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// base 库 `rawlen` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调
/// （1 号槽为 table 或 string，否则 `arg_check` 抛错；受保护帧、结果压栈可分配/GC）。
/// cpp/VM/src/lbaselib.cpp:185 luaB_rawlen。
pub fn lua_b_rawlen(l: &mut LuaState) -> i32 {
  let tt = l.type_of(1);

  l.arg_check(
    matches!(tt, LuaType::Table | LuaType::String),
    1,
    "table or string expected",
  );

  let len = l.obj_len(1) as i32;
  l.push_integer(len);

  1
}

lua_lib_fn!(pub fn lua_b_rawlen @ref, lua_b_rawlen_arm);
