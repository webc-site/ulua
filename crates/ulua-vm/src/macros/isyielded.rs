use crate::{
  enums::lua_status::LuaStatus, macros::scheduled_reentry::SCHEDULED_REENTRY,
  records::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub const unsafe fn isyielded(l: *mut lua_State) -> bool {
  let status = unsafe { (*l).status as i32 };
  status == LuaStatus::Yield as i32
    || status == LuaStatus::Break as i32
    || status == SCHEDULED_REENTRY
}
