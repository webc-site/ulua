use core::{
  ffi::{c_char, c_int, c_uint},
  ptr::{addr_of_mut, copy_nonoverlapping},
};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::luaM_newgco_, lua_m_toobig::lua_m_toobig, lua_s_resize::luaS_resize},
  macros::{
    atom_undef::ATOM_UNDEF, lmod::lmod, lua_c_init::luaC_init, maxssize::MAXSSIZE,
    sizestring::sizestring,
  },
  records::{stringtable::Stringtable, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn newlstr(
  l: *mut lua_State,
  str_: *const c_char,
  len: usize,
  mut h: c_uint,
) -> *mut tstring {
  unsafe {
    if len > MAXSSIZE as usize {
      lua_m_toobig(l);
    }

    let ts = luaM_newgco_(l, sizestring(len), (*l).activememcat) as *mut tstring;

    luaC_init!(l, ts, LuaType::String as c_int);
    (*ts).atom = ATOM_UNDEF as i16;
    (*ts).hash = h;
    (*ts).len = len as c_uint;

    // len==0 时 str_ 允许为 null（cpp memcpy(...,0) 语义），Rust 零长度 copy 是 UB，跳过
    if len != 0 {
      copy_nonoverlapping(str_, (*ts).data.as_mut_ptr(), len);
    }
    *(*ts).data.as_mut_ptr().add(len) = 0;

    let tb: *mut Stringtable = addr_of_mut!((*(*l).global).strt);
    h = lmod!(h, (*tb).size) as c_uint;
    (*ts).next = *(*tb).hash.add(h as usize);
    *(*tb).hash.add(h as usize) = ts;

    (*tb).nuse = (*tb).nuse.wrapping_add(1);
    if (*tb).nuse > (*tb).size as u32 && (*tb).size <= c_int::MAX / 2 {
      luaS_resize(l, (*tb).size * 2);
    }

    ts
  }
}
