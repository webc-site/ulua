use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::lua_d_throw, lua_g_pusherror::lua_g_pusherror},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_break(l: *mut LuaState) -> i32 {
  unsafe {
    if (*l).n_ccalls > (*l).base_ccalls {
      lua_g_pusherror(
        l,
        c"attempt to break across metamethod/C-call boundary".as_ptr(),
      );
      lua_d_throw(l, LuaStatus::ErrRun as i32);
    }

    (*l).status = LuaStatus::Break as u8;
    -1
  }
}
