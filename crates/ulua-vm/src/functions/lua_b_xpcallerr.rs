use core::ffi::c_void;

use crate::{
  functions::lua_d_callny::lua_d_callny, records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
///
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`ud` must be a valid pointer——
/// 须为错误传播后仍存活的 StkId 消息槽；`lua_d_callny` 仍收裸形，转手经一次 `l.as_mut_ptr()`
/// 就地重建（借用窗止于当句），屏障按 r16-v21 判例保留。
pub unsafe fn lua_b_xpcallerr(l: &mut LuaState, ud: *mut c_void) {
  // SAFETY: 契约保证消息槽 `ud` 存活，`lua_d_callny` 经 `as_mut_ptr` 重建的 `l` 为错误传播后的
  // 存活状态，消息值存于错误实例 payload 且块内仅按引用读回栈顶
  unsafe {
    let func: StkId = ud as StkId;
    lua_d_callny(l.as_mut_ptr(), func, 1);
  }
}
