use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{copy_nonoverlapping, eq, null_mut},
};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, maybesetaboundary::maybesetaboundary},
  macros::{
    dummynode::dummynode, getaboundary::getaboundary, lua_c_init::luaC_init,
    lua_m_newarray::luaM_newarray,
  },
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须存活且分配器就绪（拷贝数组/节点时可抛 ERR_MEM unwind）；`tt` 须为存活 `LuaTable` 且
/// array/sizearray、node/lsizenode 自洽——两处 `copy_nonoverlapping` 直接以源表长度为界，失真即越界复制。
/// 返回的新表尚未入栈，调用方须尽快放进可达槽位。cpp ltable.cpp:1374。
pub unsafe fn lua_h_clone(l: *mut LuaState, tt: *mut LuaTable) -> *mut LuaTable {
  unsafe {
    let t = lua_m_newgco(l, size_of::<LuaTable>(), (*l).activememcat) as *mut LuaTable;

    luaC_init!(l, t, LuaType::Table as i32);
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

/// # Safety
/// C ABI 导出壳：`tt` 必须是擦除为 `c_void` 的真实 `LuaTable*` 且满足内层 `lua_h_clone` 全部前提；
/// 传任意指针会在还原后按表布局越界复制。cpp ltable.cpp:1374。
pub unsafe extern "C-unwind" fn lua_h_clone_export(
  l: *mut LuaState,
  tt: *mut c_void,
) -> *mut c_void {
  // Safety: 导出壳将 tt 还原为 LuaTable 后转发同契约 `lua_h_clone`
  unsafe { lua_h_clone(l, tt as *mut LuaTable).cast() }
}
