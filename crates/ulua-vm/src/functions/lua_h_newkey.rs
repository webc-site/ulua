use core::ptr::null;

use crate::{
  functions::{lua_g_runerror_l::lua_g_runerror_l, luai_vecisnan::luai_vecisnan, newkey::newkey},
  macros::luai_numisnan::luai_numisnan,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// 为表新键占位并返回其（仍为 nil 的）可写值槽：先拒 nil/NaN 键，再转 [`newkey`]
/// （cpp `luaH_newkey` 的 flag-off 分支 ltable.cpp:1235-1244 + `newkey_DEPRECATED`
/// ltable.cpp:922-977；`LuauSplitTableLookups` 默认 false（ltable.cpp:38），
/// `newkeytagged` 条件屏障臂未移植，见遗留清单供 C3.5）。
///
/// §11 pass C3 收口论证：本链唯一的 cpp「同步成对」写点在这里——`newkey` 尾部
/// `setnodekey(L, mp, key); luaC_barriert(L, t, key)`（ltable.cpp:974-975）把**键**
/// 写入节点与键侧屏障收在同一函数体内、相邻两条语句，Rust `newkey.rs` 已在位照抄；
/// 注意该屏障只保护新写入的键，**值**槽的写入与屏障仍归调用层（见 lua_h_set 系列注释），
/// 不得顺手合并。
///
/// # Safety
/// `l` 须存活（错误路径经其抛 Lua 错误）；`t` 须为存活 `LuaTable`，`key` 指向可读 `TValue`
/// （nil/NaN 键在内部抛错、不返回）。`newkey` 可能 rehash：返回槽可用，但调用方此前持有的任何
/// 表内旧槽指针即刻失效，继续写即悬垂。cpp ltable.cpp:1195。
pub(crate) unsafe fn lua_h_newkey(
  l: *mut LuaState,
  t: *mut LuaTable,
  key: *const TValue,
) -> *mut TValue {
  // Safety: 契约保证 `t` 为存活 LuaTable 且 key 可读，块内 rehash/节点搬运仅触及 array 与 sizenode 界内槽位
  unsafe {
    if (*key).is_nil() {
      lua_g_runerror_l(l, null(), format_args!("table index is nil"));
    } else if (*key).is_number() && luai_numisnan((*key).as_number()) {
      lua_g_runerror_l(l, null(), format_args!("table index is NaN"));
    } else if (*key).is_vector() && luai_vecisnan((*key).as_vector_ref().as_ptr()) {
      lua_g_runerror_l(l, null(), format_args!("table index contains NaN"));
    }

    newkey(l, t, key)
  }
}
