use crate::records::lua_state::LuaState;

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub fn lua_l_optinteger_64(l: &mut LuaState, narg: i32, def: i64) -> i64 {
  if l.is_none_or_nil(narg) {
    def
  } else {
    l.check_integer_64(narg)
  }
}
