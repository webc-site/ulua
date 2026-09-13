use core::ffi::c_void;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_rawrunprotected_ldo::lua_d_rawrunprotected, resume_findhandler::resume_findhandler,
    resume_finish::resume_finish, resume_handle::resume_handle, resume_start::resume_start,
  },
  macros::cast_byte::cast_byte,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_resumeerror")]
pub unsafe fn lua_resumeerror(l: *mut lua_State, from: *mut lua_State) -> i32 {
  unsafe {
    let starterror = resume_start(l, from, 1);
    if starterror != 0 {
      return starterror;
    }

    let old_n_c_calls = (*l).n_ccalls;
    let old_n_c_calls_i32: i32 = old_n_c_calls as i32;

    let status = LuaStatus::ErrRun as i32;

    let ci = resume_findhandler(l);
    if !ci.is_null() {
      (*l).status = cast_byte!(status);
      let status_result = lua_d_rawrunprotected(l, Some(resume_handle), ci as *mut c_void);
      return resume_finish(l, status_result, old_n_c_calls_i32);
    }

    resume_finish(l, status, old_n_c_calls_i32)
  }
}
