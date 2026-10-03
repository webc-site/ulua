use crate::{
  functions::lua_l_getmetafield::lua_l_getmetafield_bytes, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// base 库 `getmetatable` 核心。调用序契约（正确性，非内存安全）：以 Lua 库函数
/// 约定被调（1 号槽有值、受保护帧、可分配/GC），`__metatable` 字段读取后结果压栈。
/// cpp/VM/src/lbaselib.cpp:88 luaB_getmetatable。
pub fn lua_b_getmetatable(l: &mut LuaState) -> i32 {
  l.check_any(1);

  if !l.get_metatable(1) {
    l.push_nil();
    return 1; // no metatable
  }

  lua_l_getmetafield_bytes(l, 1, b"__metatable");
  1 // returns either __metatable field (if present) or metatable
}

lua_lib_fn!(pub fn lua_b_getmetatable @ref, lua_b_getmetatable_arm);
