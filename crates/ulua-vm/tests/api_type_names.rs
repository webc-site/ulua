//! `lua_typename_str` 类型名表契约（对照 cpp lapi.cpp 的 `luaO_typenames[]`：
//! 索引即 `LUA_T*` 类型 ID，`LUA_TNONE` 特判 "no value"，越界返回 None）。

use ulua_vm::functions::lua_typename::lua_typename_str;

#[test]
fn type_names_match_cpp_table() {
  assert_eq!(lua_typename_str(-1), Some("no value"));
  assert_eq!(lua_typename_str(0), Some("nil"));
  assert_eq!(lua_typename_str(1), Some("boolean"));
  assert_eq!(lua_typename_str(2), Some("userdata"));
  assert_eq!(lua_typename_str(3), Some("number"));
  assert_eq!(lua_typename_str(6), Some("string"));
  assert_eq!(lua_typename_str(7), Some("table"));
  assert_eq!(lua_typename_str(8), Some("function"));
  assert_eq!(lua_typename_str(11), Some("buffer"));
  assert_eq!(lua_typename_str(13), Some("object"));
  assert_eq!(lua_typename_str(14), None);
  assert_eq!(lua_typename_str(-2), None);
}
