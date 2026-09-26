use core::ffi::{c_uint, c_void};

use crate::{
  functions::{lua_h_getnum::lua_h_getnum, newkey::newkey},
  macros::{lua_o_nilobject::LUA_O_NILOBJECT, setnvalue::setnvalue},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// 取整数键表槽（只查找、不写值）：数组区命中直返 `&t->array[key-1]`；哈希区命中经
/// [`lua_h_getnum`] 复用旧槽，缺失经 [`newkey`] 建键。
///
/// §11 pass C3 收口论证：cpp `luaH_setnum`（ltable.cpp:1247-1268）纯取槽——数组分支
/// 与哈希分支都不写值、都不带屏障（数组部分非灰色对象新增可达路径，屏障是**值写入**
/// 的属性而非**槽位取得**的属性）；写入方的屏障一律在调用层：lapi.cpp:1050-1051
/// （rawseti：setobj2t 后 luaC_barriert）、lapi.cpp:1887-1891（lua_ref：取槽后先读
/// `nvalue(slot)` 回收 free 链再 setobj2t+barriert，取/读/写三点分离）、
/// lbuiltins.cpp:1026-1027（luauF_tinsert 同型）。数值写入本身免屏障
/// （lapi.cpp:1910-1911 lua_unref「no barrier needed because value isn't
/// collectable」）。cpp 各写入点与取槽不同位置 ⇒ 保持分步，
/// 不收敛为「取槽+写」单函数。cpp flag-off 路径：`LuauSplitTableLookups` 默认 false
/// （ltable.cpp:38），Rust 未移植 newkeytagged 条件臂，固定走 newkey_DEPRECATED 等价
/// 路径（遗留清单，供 C3.5）。
///
/// # Safety
/// `l` 须存活（新键路径走 `newkey`，可 rehash/抛 ERR_MEM）；`t` 须为存活 `LuaTable` 且 array 区长度
/// 不小于 `sizearray`（数组路径按 `array.add(key-1)` 直接寻址）。返回的可写值槽在下一次 rehash 前有效
/// （noalias 前提：取槽与写之间不得对该表做结构性扩容，否则旧指针悬垂）。`sizearray` 与实区不符会越界取槽。cpp ltable.cpp:1247。
pub unsafe fn lua_h_setnum(l: *mut LuaState, t: *mut LuaTable, key: i32) -> *mut TValue {
  unsafe {
    // (1 <= key && key <= t->sizearray)
    if (key as c_uint).wrapping_sub(1) < (*t).sizearray as c_uint {
      return (*t).array.add((key - 1) as usize);
    }

    // hash fallback
    let p = lua_h_getnum(t, key);
    if p != LUA_O_NILOBJECT {
      p as *mut TValue
    } else {
      let mut k = TValue::default();
      setnvalue!(&mut k, key as f64);

      newkey(l, t, &k)
    }
  }
}

/// # Safety
/// C ABI 导出壳：`t` 必须是擦除为 `c_void` 的真实 `LuaTable*`，其余前提同内层 `lua_h_setnum`；
/// 非表指针还原后会在数组边界判断与取槽处越界。cpp ltable.cpp:1247。
pub unsafe extern "C-unwind" fn lua_h_setnum_export(
  l: *mut LuaState,
  t: *mut c_void,
  key: i32,
) -> *mut TValue {
  // Safety: 导出壳将 t 还原为 LuaTable 后转发同契约 `lua_h_setnum`，key 为存活数值键
  unsafe { lua_h_setnum(l, t as *mut LuaTable, key) }
}
