//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:92:close_state`
//! Source: `VM/src/lstate.cpp:92-113` (hand-ported)

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    freestack::freestack, lua_c_freeall::luaC_freeall, lua_f_close::lua_f_close,
    lua_m_free::luaM_free_,
  },
  records::{lg::LG, t_string::tstring},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn close_state(l: *mut lua_State) {
  unsafe {
    let g = (*l).global;
    lua_f_close(l, (*l).stack); // close all upvalues for this thread
    luaC_freeall(l); // collect all objects
    LUAU_ASSERT!((*g).strt.nuse == 0);
    luaM_free_(
      l,
      (*(*l).global).strt.hash as *mut u8,
      (*(*l).global).strt.size as usize * size_of::<*mut tstring>(),
      0,
    );
    freestack(l, l);
    for page in (*g).freepages.iter() {
      LUAU_ASSERT!(page.is_null());
    }
    for page in (*g).freegcopages.iter() {
      LUAU_ASSERT!(page.is_null());
    }
    LUAU_ASSERT!((*g).allgcopages.is_null());
    LUAU_ASSERT!((*g).totalbytes == size_of::<LG>());
    LUAU_ASSERT!((*g).memcatbytes[0] == size_of::<LG>());
    for &bytes in (*g).memcatbytes.iter().skip(1) {
      LUAU_ASSERT!(bytes == 0);
    }

    if let Some(close) = (*(*l).global).ecb.close {
      close(l);
    }

    if let Some(frealloc) = (*g).frealloc {
      frealloc((*g).ud, l as *mut u8, size_of::<LG>(), 0);
    }
  }
}
