use core::ffi::c_void;

use crate::{
  functions::{lua_h_getp::lua_h_getp, newkey::newkey},
  macros::{lua_o_nilobject::LUA_O_NILOBJECT, setpvalue::setpvalue},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// 取 lightuserdata 指针键表槽（只查找、不写值）：命中复用 [`lua_h_getp`] 旧槽，缺失经
/// [`newkey`] 建键。与 set/setnum/setstr 不同，cpp `luaH_setp`（ltable.cpp:1293-1307）
/// **不做** `invalidateTMcache`——Rust 同样不做，逐位一致。
///
/// §11 pass C3 收口论证：cpp 取槽无屏障；写入与屏障成对在调用层 lapi.cpp:1062-1063
/// （rawsetptagged：setobj2t 后 luaC_barriert）。cpp 取槽与写不同位置 ⇒ 保持分步。
///
/// # Safety
/// `l` 须存活（新键路径 `newkey` 可 rehash/抛 ERR_MEM）；`t` 须为存活 `LuaTable`；`key`/`tag` 必须
/// 成对来自同一 light 指针值（`tag` 是 `key` 的实际 LuaType 标签，如 LightUserData/Proto 等），
/// 查找与建键都按该 tag 哈希。tag 与指针种类不符会造成错误命中或以错误类型落键。cpp ltable.cpp:1293。
pub(crate) unsafe fn lua_h_setp(
  l: *mut LuaState,
  t: *mut LuaTable,
  key: *mut c_void,
  tag: i32,
) -> *mut TValue {
  // Safety: 契约保证 `t` 为存活 LuaTable 且 tag 匹配指针键类型，块内 lightuserdata 键哈希与写入不越 node 界
  unsafe {
    let p = lua_h_getp(t, key, tag);

    if p != LUA_O_NILOBJECT {
      p as *mut TValue
    } else {
      let mut k = TValue::default();

      // §11 pass C3：临时键 TValue 的裸三分量直写（value.p/extra[0]/tt）收口为
      // `setpvalue!` → `TValue::set_pvalue`（C1 收口方法，写入次序逐位一致），
      // ⇔ cpp `luaH_setp` 的 `setpvalue(&k, key, tag)`（ltable.cpp:1301）。零行为变更。
      setpvalue!(&mut k, key, tag);

      newkey(l, t, &k)
    }
  }
}
