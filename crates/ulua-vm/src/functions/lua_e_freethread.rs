//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:130:luaE_freethread`
//! Source: `VM/src/lstate.cpp:130-138` (hand-ported)

use core::{mem::size_of, ptr::null_mut};

use crate::{
  functions::{freestack::freestack, lua_m_freegco::luaM_freegco_},
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_e_freethread(l: *mut lua_State, l1: *mut lua_State, page: *mut lua_Page) {
  unsafe {
    let g = (*l).global;
    if let Some(userthread) = (*g).cb.userthread {
      userthread(null_mut(), l1);
    }

    freestack(l, l1);
    luaM_freegco_(
      l,
      l1 as *mut GCObject,
      size_of::<lua_State>(),
      (*l1).hdr.memcat,
      page,
    );
  }
}

pub use lua_e_freethread as luaE_freethread;
