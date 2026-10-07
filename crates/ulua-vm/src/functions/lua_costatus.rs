use std::ptr::eq;

use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  macros::api_check::api_check,
  records::lua_state::LuaState,
};

/// 读协程调度状态（`lua_costatus`）。`l`/`co` 以引用传入（存活由类型保证），须属同一
/// `global_State`（`api_check` debug 校验）；status/ci/stack 字段在调度器之外仍可读，
/// 纯只读、不抛错、不分配。
pub fn lua_costatus(l: &LuaState, co: &LuaState) -> i32 {
  api_check!(l, l.global == co.global);

  if eq(co, l) {
    return LuaCoStatus::CoRun as i32;
  }
  if co.status as i32 == LuaStatus::Yield as i32 {
    return LuaCoStatus::CoSus as i32;
  }
  if co.status as i32 == LuaStatus::Break as i32 {
    return LuaCoStatus::CoNor as i32;
  }
  if co.status != 0 {
    return LuaCoStatus::CoErr as i32;
  }
  if co.ci != co.base_ci {
    return LuaCoStatus::CoNor as i32;
  }
  if co.top == co.base {
    return LuaCoStatus::CoFin as i32;
  }
  LuaCoStatus::CoSus as i32
}
