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
/// r16-v4c 步骤 2 safe 化：`unsafe fn` → `pub(crate) fn`，体内真实裸触点收进两处窄
/// unsafe 窗（查询点 `&*t` 物化、新键点 `newkey` 转发），逐点注记。`l`/`t` 保持
/// `*mut` 裸形的选型论证：本函数唯一写点即 `newkey`（原语形不动，消费 `l: *mut
/// LuaState` + `t: *mut LuaTable`），l/t 均为纯转发——引用形化后须在被调链上再
/// `from_mut` 重建裸形，unsafe 总量不减且多出对偶中转；`pub(crate)` 可见性本在
/// `not_unsafe_ptr_arg_deref` 豁免域，无 lint 压力，故保持 cpp 镜像裸形。`key`
/// 与 [`lua_h_getp`] 同理纯位模式用（入哈希/比较/经 `setpvalue` 入载荷），不切片化。
///
/// # Safety（调用序契约，转授调用方）
/// `l` 须存活（新键路径 `newkey` 可 rehash/抛 ERR_MEM）；`t` 须为存活 `LuaTable`；`key`/`tag` 必须
/// 成对来自同一 light 指针值（`tag` 是 `key` 的实际 LuaType 标签，如 LightUserData/Proto 等），
/// 查找与建键都按该 tag 哈希。tag 与指针种类不符会造成错误命中或以错误类型落键。cpp ltable.cpp:1293。
pub(crate) fn lua_h_setp(
  l: *mut LuaState,
  t: *mut LuaTable,
  key: *mut c_void,
  tag: i32,
) -> *mut TValue {
  // SAFETY: 契约保证 `t` 指向存活 LuaTable，`&*t` 物化为共享借用仅在本语句存留
  // （一句一借），`lua_h_getp` 只读表取回旧槽或哨兵；`key`/`tag` 纯位模式用，无解引用。
  let p = unsafe { lua_h_getp(&*t, key, tag) };

  if p != LUA_O_NILOBJECT {
    p as *mut TValue
  } else {
    let mut k = TValue::default();

    // §11 pass C3：临时键 TValue 的裸三分量直写（value.p/extra[0]/tt）收口为
    // `setpvalue!` → `TValue::set_pvalue`（C1 收口方法，写入次序逐位一致），
    // ⇔ cpp `luaH_setp` 的 `setpvalue(&k, key, tag)`（ltable.cpp:1301）。零行为变更。
    // `k` 为本帧局部值，`set_pvalue` 为 safe 方法，`key` 在此仅入载荷位存储。
    setpvalue!(&mut k, key, tag);

    // SAFETY: 契约保证 `l` 存活（rehash 可经它 OOM 报错）、`t` 为存活 LuaTable，
    // `k` 为帧内 TValue 的共享只读借用；`newkey` 原语按 cpp C-ABI 镜像消费 l/t
    // 裸形（本行起 `&*t` 查询借用窗已闭合，无并存别名）。新键落位后返回槽有效期
    // 随 `t` 直至下一次结构性写表（newkey 本契约转授调用方）。
    unsafe { newkey(l, t, &k) }
  }
}
