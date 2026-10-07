use core::{ffi::c_void, mem::size_of, ptr::null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_m_newgco::lua_m_newgco, setarrayvector::setarrayvector, setnodevector::setnodevector,
  },
  macros::{dummynode::dummynode, lua_c_init::luaC_init},
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// `l` 须存活且分配器就绪（`setarrayvector`/`setnodevector` 可抛 ERR_MEM unwind）；`narray`/`nhash`
/// 须为经 luaO 向上取整后的非负预估值（负值虽走不到分配分支，但超大正值会在乘法换算中回绕成错误分配尺寸）。
/// 返回表未入栈，调用方须尽快置于可达槽位。cpp ltable.cpp:824。
pub(crate) unsafe fn lua_h_new(l: *mut LuaState, narray: i32, nhash: i32) -> *mut LuaTable {
  unsafe {
    let t = lua_m_newgco(l, size_of::<LuaTable>(), (*l).activememcat) as *mut LuaTable;

    luaC_init!(l, t, LuaType::Table as i32);
    // 既有约定（review.md §2 规则 3）：`metatable/array` 是 repr(C) LuaTable 的 ABI 字段
    // （元表可空入约同 `fasttm` 垫片；array 为数组段指针算式基址，null 即「无数组段」，
    // 与 sizearray==0 同现）——结构哨兵保留裸指针，判空收口于 table 簇既有约定
    (*t).metatable = null_mut();
    (*t).tmcache.set(!0u8);
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
/// C ABI 导出壳：`l`、`narray`、`nhash` 须满足内层 `lua_h_new` 契约（存活状态、非负预分配尺寸）；
/// 返回值仅是把 `LuaTable*` 按 `c_void` 宽化，调用方负责还原。cpp ltable.cpp:824。
pub unsafe extern "C-unwind" fn lua_h_new_export(
  l: *mut LuaState,
  narray: i32,
  nhash: i32,
) -> *mut c_void {
  // SAFETY: 导出壳透传参数调用同契约 `lua_h_new`，返回表指针按 c_void 宽化不改变地址
  unsafe { lua_h_new(l, narray, nhash).cast() }
}
