//! Source: `VM/src/lvmexecute.cpp:3843-3872` (hand-ported)

use crate::{
  functions::copy_results_pop_frame::pop_frame_copy_results, records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 调用方须保证：`(*l).ci` 是正在返回的 Lua 帧且父帧 cip 存在（CallInfo 尚未 pop）；
/// `first..(*l).top` 为同属存活栈的连续窗口且 first<=top；目标 `(*ci).func` 起的写入窗口
/// 已由调用方（OP_CALL/performcall）预留，nresults 与预留结果槽数一致（负值为 LUA_MULTRET）。
/// cpp lvmexecute.cpp:3956 `luau_poscall`
/// C++ `void luau_poscall(LuaState* l, StkId first)`.
pub(crate) unsafe fn luau_poscall(l: *mut LuaState, first: StkId) {
  // Safety: 契约保证 `l->ci` 为正在返回的存活帧，父帧 cip 有效；结果搬回 func..func+nresults
  // 不越过父帧栈区（拷回-补 nil-弹帧-top 收口三连单源见 pop_frame_copy_results）
  unsafe {
    // finish interrupted execution of `OP_CALL'
    pop_frame_copy_results(l, first, (*l).top, (*(*l).ci).nresults);
  }
}
