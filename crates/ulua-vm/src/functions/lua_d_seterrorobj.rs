use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  macros::{lua_s_newliteral::lua_s_newliteral, setobj_2_s::setobj_2_s, setsvalue::setsvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaD_seterrorobj")]
pub unsafe fn lua_d_seterrorobj(l: *mut lua_State, errcode: c_int, oldtop: StkId) {
  unsafe {
    if errcode == LuaStatus::ErrMem as c_int {
      setsvalue!(
        l,
        oldtop,
        lua_s_newliteral(l, c"not enough memory".as_ptr())
      );
    } else if errcode == LuaStatus::ErrErr as c_int {
      setsvalue!(
        l,
        oldtop,
        lua_s_newliteral(l, c"error in error handling".as_ptr())
      );
    } else if errcode == LuaStatus::ErrSyntax as c_int || errcode == LuaStatus::ErrRun as c_int {
      // error message on current top
      setobj_2_s!(l, oldtop, (*l).top.offset(-1));
    }

    (*l).top = oldtop.offset(1);
  }
}

pub use lua_d_seterrorobj as luaD_seterrorobj;
