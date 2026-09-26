use crate::{
  functions::{lua_h_get::lua_h_get, lua_h_newkey::lua_h_newkey},
  macros::{invalidate_t_mcache::invalidate_tmcache, lua_o_nilobject::LUA_O_NILOBJECT},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// 取表槽（只查找、不写值）：命中返回既有槽，缺失经 [`lua_h_newkey`] 建键后返回值槽。
///
/// §11 pass C3 收口论证：本函数只覆盖 cpp `luaH_set`（ltable.cpp:1185-1193）的
/// 「取槽」半边——cpp 在此不写值、不带屏障；写入由调用层 `setobj2t` + `luaC_barriert`
/// 完成（lapi.cpp:1038-1039、lvmutils.cpp:316-317、lvmexecute.cpp:722-723），取槽与写
/// 在 cpp 中本就分处两位置，故不收敛为「取槽+写」单函数，保持分步。
///
/// # Safety
/// `l` 须存活（新键路径走 `lua_h_newkey`，可 rehash/抛 ERR_MEM）；`t` 须为存活
/// `LuaTable`；`key` 指向可读 `TValue`。返回槽 noalias 前提：写入期间对同一表的任何
/// 结构性写（newkey/rehash/setarrayvector）都会移动槽位，旧返回指针即刻失效；调用方
/// 须在「取槽→写」之间不触碰该表的再扩容路径（cpp 依赖同一前提）。cpp ltable.cpp:1185。
pub unsafe fn lua_h_set(l: *mut LuaState, t: *mut LuaTable, key: *const TValue) -> *mut TValue {
  // Safety: 契约保证 `t` 为存活 LuaTable、key 可读，块内节点查找与插入仅落在 sizenode/dummynode 规则界内
  unsafe {
    let p = lua_h_get(t, key);
    invalidate_tmcache(&mut *t);

    if p != LUA_O_NILOBJECT {
      p as *mut TValue
    } else {
      lua_h_newkey(l, t, key)
    }
  }
}
