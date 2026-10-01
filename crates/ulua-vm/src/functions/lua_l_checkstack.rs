use crate::{
  functions::lua_checkstack::lua_checkstack, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// `l` 须为存活 `LuaState` 且处于受保护帧：`space` 为申请的空槽数（须 ≥0），`lua_checkstack(l,space)` 尝试扩容
/// `l.stack`；失败时经 `luaL_error` 抛 "stack overflow"（可分配、unwind）。成功后调用方方可写 `l.top` 起的 space 槽。
/// cpp VM/src/laux.cpp:158
pub fn lua_l_checkstack(l: &mut LuaState, space: i32, mes: &str) {
  unsafe {
    if lua_checkstack(l, space) == 0 {
      luaL_error!(l, "stack overflow ({})", mes);
    }
  }
}
