use crate::records::lua_state::LuaState;

/// # Safety
///
/// `l` 必须指向当前正在执行的协程 `LuaState` 且其 `ci` 为活动 `CallInfo`（写入
/// `ci->savedpc`，供错误/中断恢复时回读该 pc）；`pc` 指向当前 proto `code` 数组内。
#[inline(always)]
pub unsafe fn vm_protect_pc(l: *mut LuaState, pc: *const u32) {
  // Safety: 契约保证 `l` 存活且 `(*l).ci` 有效，savedpc 为其普通字段写入
  unsafe {
    (*(*l).ci).savedpc = pc;
  }
}
