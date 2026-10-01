use core::{ffi::c_void, ptr::null_mut};

use super::LuaState;
use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_getthreaddata::lua_getthreaddata, lua_isyieldable::lua_isyieldable, lua_resume::lua_resume,
    lua_setthreaddata::lua_setthreaddata, lua_status::lua_status, lua_yield::lua_yield,
  },
};

impl LuaState {
  /// # Safety
  /// `from` 若非空须指向有效存活的 `LuaState`。
  #[inline(always)]
  pub unsafe fn resume(&mut self, from: *mut LuaState, nargs: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_resume(self.as_mut_ptr(), from, nargs) }
  }

  /// 主线程恢复入口：`lua_resume` 契约中 `from == NULL`（主状态恢复、无父调用方）
  /// 的合法形态由本方法收口，调用方无需再传 null 哨兵指针。
  #[inline(always)]
  pub fn resume_main(&mut self, nargs: i32) -> i32 {
    // SAFETY: `lua_resume` 的 `# Safety` 明文允许 `from` 为 NULL（主状态恢复）；
    // `self` 经 `&mut self` 唯一借用，为存活 LuaState。
    // 既有约定（review.md §2）：c-API 边界合法实参——from=null 系主线程恢复契约形态，本方法单点收口（详见上方 SAFETY/doc），勿改 Option
    unsafe { lua_resume(self.as_mut_ptr(), null_mut(), nargs) }
  }

  #[inline(always)]
  pub fn yield_thread(&mut self, nresults: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_yield(self.as_mut_ptr(), nresults) }
  }

  #[inline(always)]
  pub fn is_yieldable(&self) -> bool {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_isyieldable(self.read_ptr()) != 0 }
  }

  #[inline(always)]
  pub fn status(&self) -> LuaStatus {
    let s = lua_status(self);
    LuaStatus::from_repr(s).unwrap_or(LuaStatus::Ok)
  }

  #[inline(always)]
  pub fn get_thread_data(&self) -> *mut c_void {
    lua_getthreaddata(self)
  }

  #[inline(always)]
  pub fn set_thread_data(&mut self, data: *mut c_void) {
    lua_setthreaddata(self, data)
  }
}
