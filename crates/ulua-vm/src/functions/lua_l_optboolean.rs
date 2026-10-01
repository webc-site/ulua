use crate::records::lua_state::LuaState;

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub fn lua_l_optboolean(l: &mut LuaState, narg: i32, def: bool) -> bool {
  if l.is_none_or_nil(narg) {
    def
  } else {
    l.check_boolean(narg)
  }
}
