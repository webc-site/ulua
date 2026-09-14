use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::{copy_nonoverlapping, eq, null_mut},
};

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::luaM_newgco_,
  macros::{
    dummynode::dummynode, getaboundary::getaboundary, lua_c_init::luaC_init,
    lua_m_newarray::luaM_newarray,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[inline]
unsafe fn maybesetaboundary(t: *mut LuaTable, boundary: c_int) {
  unsafe {
    if (*t).union.aboundary <= 0 {
      (*t).union.aboundary = -boundary;
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_clone(l: *mut lua_State, tt: *mut LuaTable) -> *mut LuaTable {
  unsafe {
    let t = luaM_newgco_(l, size_of::<LuaTable>(), (*l).activememcat) as *mut LuaTable;

    luaC_init!(l, t, LuaType::Table as c_int);
    (*t).metatable = (*tt).metatable;
    (*t).tmcache = (*tt).tmcache;
    (*t).array = null_mut();
    (*t).sizearray = 0;
    (*t).lsizenode = 0;
    (*t).nodemask8 = 0;
    (*t).readonly = 0;
    (*t).safeenv = 0;
    (*t).node = dummynode as *mut LuaNode;
    (*t).union.lastfree = 0;

    if (*tt).sizearray != 0 {
      (*t).array = luaM_newarray!(l, (*tt).sizearray as usize, TValue, (*t).memcat);
      maybesetaboundary(t, getaboundary(tt));
      (*t).sizearray = (*tt).sizearray;

      copy_nonoverlapping((*tt).array, (*t).array, (*t).sizearray as usize);
    }

    if !eq((*tt).node, dummynode) {
      let size = 1i32 << (*tt).lsizenode;
      (*t).node = luaM_newarray!(l, size as usize, LuaNode, (*t).memcat);
      (*t).lsizenode = (*tt).lsizenode;
      (*t).nodemask8 = (*tt).nodemask8;
      copy_nonoverlapping((*tt).node, (*t).node, size as usize);
      (*t).union.lastfree = (*tt).union.lastfree;
    }

    t
  }
}

pub use lua_h_clone as luaH_clone;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaH_clone")]
pub unsafe extern "C-unwind" fn lua_h_clone_export(
  l: *mut lua_State,
  tt: *mut c_void,
) -> *mut c_void {
  unsafe { lua_h_clone(l, tt as *mut LuaTable).cast() }
}
