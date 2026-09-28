use core::ptr::null_mut;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, lua_m_toobig::lua_m_toobig},
  macros::{
    atom_undef::ATOM_UNDEF, lua_c_init::luaC_init, maxssize::MAXSSIZE, sizestring::sizestring,
  },
  records::{lua_state::LuaState, t_string::tstring},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_bufstart(l: *mut LuaState, size: usize) -> *mut tstring {
  unsafe {
    if size > MAXSSIZE as usize {
      lua_m_toobig(l);
    }

    let ts = lua_m_newgco(l, sizestring(size), (*l).activememcat) as *mut tstring;
    luaC_init!(l, ts, LuaType::String as i32);

    let s = &mut *ts;
    s.atom = ATOM_UNDEF as i16;
    s.hash = 0;
    s.len = size as u32;
    s.next = null_mut();

    ts
  }
}
