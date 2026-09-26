use core::ffi::c_void;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_rawrunprotected_ldo::lua_d_rawrunprotected, resume_findhandler::resume_findhandler,
    resume_finish::resume_finish, resume_handle::resume_handle, resume_start::resume_start,
  },
  records::lua_state::LuaState,
};

/// 对已出错协程直接注入错误驱动的恢复入口（cpp `lua_resumeerror`）。
///
/// # Safety
/// `l` 须为已置错误态、待恢复的协程 `LuaState`（`resume_start` 会校验并可能抛错），`from` 按 `resume_start`
/// 契约可为空（主状态恢复），非空时须为存活恢复发起方；`resume_findhandler` 取回的 `ci` 帧透传给
/// `resume_handle`（经 `lua_d_rawrunprotected` 保护帧承接再抛），`resume_finish` 可读写 `(*l).status/n_ccalls`。
pub unsafe fn lua_resumeerror(l: *mut LuaState, from: *mut LuaState) -> i32 {
  unsafe {
    let starterror = resume_start(l, from, 1);
    if starterror != 0 {
      return starterror;
    }

    if let Some(preresume) = (*(*l).global).cb.preresume {
      preresume(l);
    }

    let old_n_c_calls = (*l).n_ccalls;
    let old_n_c_calls_i32: i32 = old_n_c_calls as i32;

    let status = LuaStatus::ErrRun as i32;

    let ci = resume_findhandler(l);
    let mut status = status;
    if !ci.is_null() {
      (*l).status = status as u8;
      // Safety: 细粒度恢复入参契约——`ci` 为 `resume_findhandler` 取回的存活 handler
      // 帧（刚判非空），转 `void*` 透传，满足 `resume_handle` 的 `ud` 前置
      status = lua_d_rawrunprotected(l, Some(resume_handle), ci as *mut c_void);
    }

    // cpp ldo.cpp:840-847：单一公共尾部 resume_finish + postresume
    let result = resume_finish(l, status, old_n_c_calls_i32);

    if let Some(postresume) = (*(*l).global).cb.postresume {
      postresume(l);
    }

    result
  }
}
