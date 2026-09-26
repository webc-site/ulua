use ulua_vm::enums::lua_type::LuaType;

use crate::records::ir_data::K_UNKNOWN_TAG;

/// typename → LuaType tag 查表：原十条 `name == "…" { return LUA_T… }` 同形 if 链的
/// 单源表，判定顺序与短路逐条一致。
const TYPENAME_TAGS: [(&str, u8); 9] = [
  ("nil", LuaType::Nil as u8),
  ("boolean", LuaType::Boolean as u8),
  ("number", LuaType::Number as u8),
  ("integer", LuaType::Integer as u8),
  ("string", LuaType::String as u8),
  ("table", LuaType::Table as u8),
  ("function", LuaType::Function as u8),
  ("thread", LuaType::Thread as u8),
  ("buffer", LuaType::Buffer as u8),
];

pub(crate) fn try_get_tag_for_typename(name: &str, for_typeof: bool) -> u8 {
  // typeof(vector) 可被环境变量改变（TODO: 支持该环境变量选项）——for_typeof 时本条整体跳过，
  // 与旧链 `name == "vector" && !for_typeof` 等价
  if name == "vector" && !for_typeof {
    return LuaType::Vector as u8;
  }

  for (t, tag) in TYPENAME_TAGS {
    if name == t {
      return tag;
    }
  }

  K_UNKNOWN_TAG
}
