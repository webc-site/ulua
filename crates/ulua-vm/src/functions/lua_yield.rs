use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::lua_d_throw, lua_g_pusherror::lua_g_pusherror},
  macros::api_check::api_check,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_yield(l: *mut LuaState, nresults: i32) -> i32 {
  unsafe {
    api_check!(l, nresults >= 0);
    api_check!(l, nresults as isize <= (*l).top.offset_from((*l).base));

    if (*l).n_ccalls > (*l).base_ccalls {
      lua_g_pusherror(
        l,
        c"attempt to yield across metamethod/C-call boundary".as_ptr(),
      );
      lua_d_throw(l, LuaStatus::ErrRun as i32);
    }

    (*l).base = (*l).top.offset(-(nresults as isize));
    (*l).status = LuaStatus::Yield as u8;
    -1
  }
}
