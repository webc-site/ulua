use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_v_gettable::lua_v_gettable,
  macros::{restorestack::restorestack, savestack::savestack, sethvalue::sethvalue},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 指向存活 `LuaState`；`env` 为仅首步使用且当下受 GC 保护的导入环境表（TM 执行期间
/// 可能被回收，故后续步不再使用）；`k` 指向当前 Proto 的导入常量数组，`id` 三段索引
/// （bit20-30 计数字段）须落在该数组界内；`res` 为可写栈槽且每步前经 `restorestack!` 复原；
/// 栈顶须余一次 `lua_v_gettable` 的 TM 调用空间。cpp lvm.h:29.
pub unsafe fn lua_v_getimport(
  l: *mut LuaState,
  env: *mut LuaTable,
  k: *mut TValue,
  mut res: StkId,
  id: u32,
  propagatenil: bool,
) {
  // Safety: 契约保证 `l` 的 global 导入状态存活，pc 为当前 proto 内指令位置且 slot 索引落在导入数组界内
  unsafe {
    let count = id >> 30;
    LUAU_ASSERT!(count > 0);

    let id0 = ((id >> 20) & 1023) as usize;
    let id1 = ((id >> 10) & 1023) as usize;
    let id2 = (id & 1023) as usize;

    // after the first call to luaV_gettable, res may be invalid, and env may (sometimes) be garbage collected
    // we take care to not use env again and to restore res before every consecutive use
    let resp = savestack!(l, res);

    // global lookup for id0
    let mut g = TValue::default();
    sethvalue!(l, &mut g, env);
    lua_v_gettable(l, &g, k.add(id0), res);

    // table lookup for id1
    if count < 2 {
      return;
    }

    res = restorestack!(l, resp);
    if !propagatenil || !(*res).is_nil() {
      lua_v_gettable(l, res, k.add(id1), res);
    }

    // table lookup for id2
    if count < 3 {
      return;
    }

    res = restorestack!(l, resp);
    if !propagatenil || !(*res).is_nil() {
      lua_v_gettable(l, res, k.add(id2), res);
    }
  }
}
