use crate::records::lua_state::LuaState;

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v43 收形后
/// 判空与取数全经安全门面 `is_none_or_nil`/`check_number`，体内无裸操作，故降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧：栈 `narg` 号位 none/nil 时回退 `def`（纯值，不读栈），
/// 否则经 `check_number` 要求数值（非数值抛 "number expected" 发散）。cpp laux.cpp:205
/// `luaL_optnumber`（`luaL_opt` 宏展开即此判序）。
pub(crate) fn lua_l_optnumber(l: &mut LuaState, narg: i32, def: f64) -> f64 {
  if l.is_none_or_nil(narg) {
    def
  } else {
    l.check_number(narg)
  }
}
