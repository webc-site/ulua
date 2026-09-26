use crate::{
  enums::lua_status::LuaStatus, macros::scheduled_reentry::SCHEDULED_REENTRY,
  records::lua_state::LuaState,
};

/// 读 `LuaState.status` 判断当前是否处于 yield/break/受控重入状态。
/// 以 `&LuaState` 接收者替代原 `*mut` 裸指针：仅读一个普通字段，无需 unsafe。
pub const fn isyielded(l: &LuaState) -> bool {
  let status = l.status as i32;
  status == LuaStatus::Yield as i32
    || status == LuaStatus::Break as i32
    || status == SCHEDULED_REENTRY
}
