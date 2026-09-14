//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:99:index2addr`
//! Source: `VM/src/lapi.cpp:99-118` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::pseudo_2_addr::pseudo_2_addr,
  macros::{
    api_check::api_check, lua_o_nilobject::luaO_nilobject, lua_registryindex::LUA_REGISTRYINDEX,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn index2addr(l: *mut lua_State, idx: c_int) -> StkId {
  unsafe {
    if idx > 0 {
      let o = (*l).base.add((idx - 1) as usize);
      api_check!(l, idx as isize <= (*(*l).ci).top.offset_from((*l).base));
      if o >= (*l).top {
        luaO_nilobject as *mut TValue
      } else {
        o
      }
    } else if idx > LUA_REGISTRYINDEX {
      api_check!(
        l,
        idx != 0 && (-idx) as isize <= (*l).top.offset_from((*l).base)
      );
      (*l).top.offset(idx as isize)
    } else {
      pseudo_2_addr(l, idx)
    }
  }
}

pub use index2addr as index_2_addr;
