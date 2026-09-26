use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setnilvalue::setnilvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).top` 之后经 `ensure_stack(l, 1)` 留 1 个空槽（`setnilvalue` 写入并 `api_incr_top` 上移栈顶）；
/// 可分配栈，须在受保护帧调用。cpp/VM/src/lapi.cpp:683 lua_pushnil。
pub unsafe fn lua_pushnil(l: *mut LuaState) {
  unsafe {
    ensure_stack(l, 1);
    setnilvalue!((*l).top);
    api_incr_top!(l);
  }
}
