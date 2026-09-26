use core::ffi::c_void;

use crate::{
  functions::lua_d_callny::lua_d_callny, records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
///
/// `l` and `ud` must be valid pointers.
pub unsafe fn lua_b_xpcallerr(l: *mut LuaState, ud: *mut c_void) {
  // Safety: 契约保证 `L` 为错误传播后的存活状态，消息值存于错误实例 payload 且块内仅按引用读回栈顶
  unsafe {
    let func: StkId = ud as StkId;
    lua_d_callny(l, func, 1);
  }
}
