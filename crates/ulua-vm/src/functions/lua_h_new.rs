use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::null_mut,
};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_m_newgco::luaM_newgco_, setarrayvector::setarrayvector, setnodevector::setnodevector,
  },
  macros::{dummynode::dummynode, lua_c_init::luaC_init},
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_new(l: *mut lua_State, narray: c_int, nhash: c_int) -> *mut LuaTable {
  unsafe {
    let t = luaM_newgco_(l, size_of::<LuaTable>(), (*l).activememcat) as *mut LuaTable;

    luaC_init!(l, t, LuaType::Table as c_int);
    (*t).metatable = null_mut();
    (*t).tmcache = !0u8;
    (*t).array = null_mut();
    (*t).sizearray = 0;
    (*t).union.lastfree = 0;
    (*t).lsizenode = 0;
    (*t).readonly = 0;
    (*t).safeenv = 0;
    (*t).nodemask8 = 0;
    (*t).node = dummynode as *mut LuaNode;

    if narray > 0 {
      setarrayvector(l, t, narray);
    }

    if nhash > 0 {
      setnodevector(l, t, nhash);
    }

    t
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaH_new")]
pub unsafe extern "C-unwind" fn lua_h_new_export(
  l: *mut lua_State,
  narray: c_int,
  nhash: c_int,
) -> *mut c_void {
  unsafe { lua_h_new(l, narray, nhash).cast() }
}

pub use lua_h_new as luaH_new;
