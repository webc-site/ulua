use crate::{
  functions::{lua_h_getstr::lua_h_getstr, newkey::newkey},
  macros::{
    invalidate_t_mcache::invalidate_tmcache, lua_o_nilobject::LUA_O_NILOBJECT, setsvalue::setsvalue,
  },
  records::{lua_state::LuaState, lua_table::LuaTable, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// 取字符串键表槽（只查找、不写值）：命中复用 [`lua_h_getstr`] 旧槽，缺失经 [`newkey`]
/// 建键；与 cpp 同型在查找后无条件 `invalidateTMcache`（ltable.cpp:1277）。
///
/// §11 pass C3 收口论证：cpp `luaH_setstr`（ltable.cpp:1275-1291）纯取槽、无屏障；写入
/// 与屏障成对在调用层——lapi.cpp:1026-1027（rawsetfield）、lvmexecute.cpp:718→722-723
/// （SETTABLEKS：取槽→patch cachedslot→setobj2t→barriert，屏障落在写之后、且与取槽隔
/// 一条 patch 语句）、ltablib.cpp:359-360（tpack：`setnvalue` 数值写免屏障）、
/// lclass.cpp:172-174（addclassmember：`setnvalue` 后 cpp 显式成对 `luaC_barrier`，
/// Rust 逐位保留该同步对）、lapi.cpp:2207-2210（registeruserdatadirectfield：
/// `setpvalue` light 指针写免屏障）。cpp 取槽与写不同位置 ⇒ 保持分步，不收敛为单函数。
///
/// # Safety
/// `l` 须为存活 `LuaState`；`t` 须为存活且非只读的 `LuaTable`（键表），`key` 须为存活 `TString`；
/// 命中既有键时返回其 `lua_h_getstr` 槽（非 LUA_O_NILOBJECT）；缺失时 `newkey(l,t,&k)` 可能 rehash/扩数组并触发 GC，
/// 返回的新槽指针在 rehash 后仍有效（调用方须在同一 top 稳定窗口内使用）。返回的 `*mut TValue` 恒非空、落在表内存内。
/// noalias 前提：写入期间对同表的再结构性写（newkey/rehash）会移动槽位使旧指针失效。
/// cpp VM/src/ltable.cpp:1275
pub unsafe fn lua_h_setstr(l: *mut LuaState, t: *mut LuaTable, key: *mut tstring) -> *mut TValue {
  unsafe {
    let p = lua_h_getstr(t, key);
    invalidate_tmcache(t);

    if p != LUA_O_NILOBJECT {
      p as *mut TValue
    } else {
      let mut k = TValue::default();
      setsvalue!(l, &mut k, key);

      newkey(l, t, &k)
    }
  }
}
