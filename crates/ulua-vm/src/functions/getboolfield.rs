use crate::{functions::lua_rawgetfield::lua_rawgetfield_bytes, records::lua_state::LuaState};

/// `key` 为纯 Rust 字节切片（如 `b"isdst"`），直接传给 `lua_rawgetfield_bytes`。
///
/// r19-w4 收形：首参 `*mut LuaState` → `&mut LuaState`，存活与独占交由类型承载；
/// 体内 `lua_rawgetfield_bytes`/`LuaState` 方法族均收 `&mut self`/`&self`，无调用方
/// 传入的裸指针参数被解引用（`key: &[u8]` 亦为引用），依 §2 假合规防线判例（dev
/// `SubtypingEnvironment::get_mapped_type_bounds` 降 safe 判例）由 `unsafe fn` 降为 `fn`。
pub(crate) fn getboolfield(l: &mut LuaState, key: &[u8]) -> i32 {
  lua_rawgetfield_bytes(l, -1, key);

  let is_nil = l.is_nil(-1);

  let res: i32 = if is_nil { -1 } else { l.to_boolean(-1) as i32 };

  l.pop(1);
  res
}
