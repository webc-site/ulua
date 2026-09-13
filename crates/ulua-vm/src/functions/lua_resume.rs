use core::ffi::{c_int, c_void};

use crate::{
  functions::{
    lua_d_rawrunprotected_ldo::luaD_rawrunprotected, resume::resume, resume_finish::resume_finish,
    resume_start::resume_start,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_resume(l: *mut lua_State, from: *mut lua_State, nargs: c_int) -> c_int {
  unsafe {
    let starterror = resume_start(l, from, nargs);
    if starterror != 0 {
      return starterror;
    }

    let old_n_ccalls = (*l).n_ccalls as c_int;
    let status = luaD_rawrunprotected(
      l,
      Some(resume),
      (*l).top.offset(-(nargs as isize)) as *mut c_void,
    );

    resume_finish(l, status, old_n_ccalls)
  }
}
