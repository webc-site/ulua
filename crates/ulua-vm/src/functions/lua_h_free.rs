use core::{mem::size_of, ptr::eq};

use crate::{
  functions::{lua_m_free::luaM_free_, lua_m_freegco::luaM_freegco_},
  macros::{dummynode::dummynode, sizenode::sizenode},
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::lua_State, lua_table::LuaTable},
  type_aliases::{lua_node::LuaNode, t_value::TValue},
};

pub(crate) unsafe fn lua_h_free(l: *mut lua_State, t: *mut LuaTable, page: *mut lua_Page) {
  unsafe {
    if !eq((*t).node as *const LuaNode, dummynode) {
      let size = sizenode!(t);
      luaM_free_(
        l,
        (*t).node as *mut u8,
        size as usize * size_of::<LuaNode>(),
        (*t).memcat,
      );
    }
    if !(*t).array.is_null() {
      luaM_free_(
        l,
        (*t).array as *mut u8,
        (*t).sizearray as usize * size_of::<TValue>(),
        (*t).memcat,
      );
    }
    luaM_freegco_(
      l,
      t as *mut GCObject,
      size_of::<LuaTable>(),
      (*t).memcat,
      page,
    );
  }
}
