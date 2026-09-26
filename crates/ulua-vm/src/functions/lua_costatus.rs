use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  macros::api_check::api_check,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` and `co` must be valid pointers to live `LuaState` instances belonging to the same global state.
pub unsafe fn lua_costatus(l: *mut LuaState, co: *mut LuaState) -> i32 {
  // Safety: 契约保证 `co` 指向存活协程状态，status 字段在调度器之外仍可读
  unsafe {
    api_check!(l, !co.is_null());
    api_check!(l, (*l).global == (*co).global);

    if co == l {
      return LuaCoStatus::CoRun as i32;
    }
    if (*co).status as i32 == LuaStatus::Yield as i32 {
      return LuaCoStatus::CoSus as i32;
    }
    if (*co).status as i32 == LuaStatus::Break as i32 {
      return LuaCoStatus::CoNor as i32;
    }
    if (*co).status != 0 {
      return LuaCoStatus::CoErr as i32;
    }
    if (*co).ci != (*co).base_ci {
      return LuaCoStatus::CoNor as i32;
    }
    if (*co).top == (*co).base {
      return LuaCoStatus::CoFin as i32;
    }
    LuaCoStatus::CoSus as i32
  }
}
