use core::{ffi::c_int, ptr::null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::luaM_newgco_, lua_m_toobig::lua_m_toobig},
  macros::{
    atom_undef::ATOM_UNDEF, lua_c_init::luaC_init, maxssize::MAXSSIZE, sizestring::sizestring,
  },
  records::t_string::tstring,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_bufstart(l: *mut lua_State, size: usize) -> *mut tstring {
  unsafe {
    if size > MAXSSIZE as usize {
      lua_m_toobig(l);
    }

    let ts = luaM_newgco_(l, sizestring(size), (*l).activememcat) as *mut tstring;

    luaC_init!(l, ts, LuaType::String as c_int);
    (*ts).atom = ATOM_UNDEF as i16;
    (*ts).hash = 0;
    (*ts).len = size as u32;
    (*ts).next = null_mut();

    ts
  }
}

pub use lua_s_bufstart as luaS_bufstart;
