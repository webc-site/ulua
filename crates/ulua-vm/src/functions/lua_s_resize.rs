use core::{ffi::c_int, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{lmod::lmod, lua_m_freearray::luaM_freearray, lua_m_newarray::luaM_newarray},
  records::{stringtable::Stringtable, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_s_resize(l: *mut lua_State, newsize: c_int) {
  unsafe {
    let newhash = luaM_newarray!(l, newsize as usize, *mut tstring, 0);
    let tb: *mut Stringtable = core::ptr::addr_of_mut!((*(*l).global).strt);

    let mut i = 0;
    while i < newsize {
      *newhash.add(i as usize) = null_mut();
      i += 1;
    }

    i = 0;
    while i < (*tb).size {
      let mut p = *(*tb).hash.add(i as usize);
      while !p.is_null() {
        let next = (*p).next;
        let h = (*p).hash;
        let h1 = lmod!(h, newsize) as c_int;
        LUAU_ASSERT!((h % newsize as u32) as c_int == lmod!(h, newsize));
        (*p).next = *newhash.add(h1 as usize);
        *newhash.add(h1 as usize) = p;
        p = next;
      }
      i += 1;
    }

    luaM_freearray!(l, (*tb).hash, (*tb).size as usize, *mut tstring, 0);
    (*tb).size = newsize;
    (*tb).hash = newhash;
  }
}

pub use lua_s_resize as luaS_resize;
