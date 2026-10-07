use crate::{
  functions::{lua_isnumber::lua_isnumber, lua_rawgetfield::lua_rawgetfield_bytes},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// `key` 为纯 Rust 字节切片（如 `b"sec"`），直接传给 `lua_rawgetfield_bytes`。
///
/// r19-w4 收形并降 safe：首参 `*mut LuaState` → `&mut LuaState`（存活与独占交由类型承载）；
/// 体内 `lua_rawgetfield_bytes`/`lua_isnumber` 均收引用形，`to_integer`/`pop` 为 `LuaState`
/// 方法，参数收形后签名不再有调用方传入的裸指针，依 §2 假合规防线判例
/// （dev `SubtypingEnvironment::get_mapped_type_bounds`）降 `fn`；缺失字段的抛错分支经
/// `lua_l_error_l`（发散），已随 wave-6d 降为引用形安全门面，直传借用无 `unsafe` 残留。
pub(crate) fn getfield(l: &mut LuaState, key: &[u8], d: i32) -> i32 {
  lua_rawgetfield_bytes(l, -1, key);

  if lua_isnumber(l, -1) != 0 {
    // `lua_tointeger!` 即 `i32`，无需再 cast
    let res = l.to_integer(-1).unwrap_or(0);
    l.pop(1);
    res
  } else {
    if d < 0 {
      let key_text = String::from_utf8_lossy(key);
      luaL_error!(l, "field '{}' missing in date table", key_text);
    }
    l.pop(1);
    d
  }
}
