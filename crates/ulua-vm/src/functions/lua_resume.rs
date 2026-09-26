use core::ffi::c_void;

use crate::{
  functions::{
    lua_d_rawrunprotected_ldo::lua_d_rawrunprotected, resume::resume, resume_finish::resume_finish,
    resume_start::resume_start,
  },
  records::lua_state::LuaState,
};

/// 协程恢复公开入口：建档→受保护驱动 `resume`→收尾（cpp `lua_resume`）。
///
/// # Safety
/// 本函数经 `lua_d_rawrunprotected` 受保护地驱动 `resume`，会执行目标协程的 Lua 代码、可分配/GC/抛错；
/// `l` 须为待恢复协程状态且栈上已压入 `nargs`（`nargs>=0`）个实参（`top-base>=nargs`）；`from` 按 `resume_start`
/// 契约可为 NULL（主状态恢复），非空时须为同一 `global` 下的存活 `LuaState`。cpp `ldo.cpp:803`。
pub unsafe fn lua_resume(l: *mut LuaState, from: *mut LuaState, nargs: i32) -> i32 {
  unsafe {
    let starterror = resume_start(l, from, nargs);
    if starterror != 0 {
      return starterror;
    }

    // Some profilers can use preresume/postresume to push one Luau execution
    // frame covering the entire resume and pop it on exit.（cpp ldo.cpp:795-798）
    if let Some(preresume) = (*(*l).global).cb.preresume {
      preresume(l);
    }

    let old_n_ccalls = (*l).n_ccalls as i32;
    // Safety: 粗粒度恢复入参契约——`resume_start` 的 `api_check!` 已核
    // `top - base >= nargs`，`top - nargs` 即首实参栈槽，满足 `resume` 的 `ud` 前置
    let status = lua_d_rawrunprotected(
      l,
      Some(resume),
      (*l).top.offset(-(nargs as isize)) as *mut c_void,
    );

    let result = resume_finish(l, status, old_n_ccalls);

    if let Some(postresume) = (*(*l).global).cb.postresume {
      postresume(l);
    }

    result
  }
}
