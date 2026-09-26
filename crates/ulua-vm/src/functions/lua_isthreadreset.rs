use crate::{enums::lua_status::LuaStatus, records::lua_state::LuaState};

/// # Safety
///
/// `l` 须为有效存活的 `LuaState`。
pub unsafe fn lua_isthreadreset(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 指向存活 LuaState，仅重置其 status 字段且调用期间不与调度器并发
  unsafe {
    ((*l).ci == (*l).base_ci && (*l).base == (*l).top && (*l).status == LuaStatus::Ok as u8) as i32
  }
}
