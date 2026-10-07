use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_h_getstr::lua_h_getstr, lua_v_gettable::lua_v_gettable},
  macros::{
    gval_2_slot::gval2slot, lua_o_nilobject::LUA_O_NILOBJECT, restorestack::restorestack,
    savestack::savestack, sethvalue::sethvalue, setobj_2_s::setobj_2_s,
  },
  records::{lua_state::LuaState, lua_table::LuaTable, slot::Slot},
  type_aliases::t_value::TValue,
};

/// 导入 id 每段 10 位索引的字段掩码（bit0-9 / bit10-19 / bit20-29）。
const K_ID_FIELD_MASK: u32 = 1023;

/// # Safety
/// `l` 指向存活 `LuaState`；`env` 为仅首步使用且当下受 GC 保护的导入环境表（TM 执行期间
/// 可能被回收，故后续步不再使用）；`k` 指向当前 Proto 的导入常量数组（只读侧：三段索引
/// 各取一槽作查找键，cpp 形参 `const TValue* k`），`id` 三段索引
/// （bit20-30 计数字段）须落在该数组界内；`res` 为可写栈槽句柄且每步前经 `restorestack!`
/// 重派生复原——服从「扩容先行、借用后派生」：栈重定位使旧句柄即刻失效，本函数逐次从
/// 恢复的裸槽地址重建；栈顶须余一次 `lua_v_gettable` 的 TM 调用空间。cpp lvm.h:29。
pub unsafe fn lua_v_getimport(
  l: *mut LuaState,
  env: *mut LuaTable,
  k: *const TValue,
  mut res: Slot<'_>,
  id: u32,
  propagatenil: bool,
) {
  // SAFETY: 契约保证 `l` 的 global 导入状态存活，pc 为当前 proto 内指令位置且 slot 索引落在导入数组界内
  unsafe {
    let count = id >> 30;
    LUAU_ASSERT!(count > 0);

    let id0 = ((id >> 20) & K_ID_FIELD_MASK) as usize;
    let id1 = ((id >> 10) & K_ID_FIELD_MASK) as usize;
    let id2 = (id & K_ID_FIELD_MASK) as usize;

    // after the first call to luaV_gettable, res may be invalid, and env may (sometimes) be garbage collected
    // we take care to not use env again and to restore res before every consecutive use
    let resp = savestack!(l, res.as_ptr());

    // ---- 两级直查快支（本 fork 实测扩展，cpp 无对应支）----
    // safeenv=0 时 GETIMPORT 每次走慢路全量解析（luaV_getimport 快路 kv 缓存仅在
    // safeenv 下合法），coroutines 类负载每轮付 8 次两级 `lua_v_gettable` 通用链。
    // import 各段键恒为编译期字符串常量，第一级 t 恒为 env 表；两级均为「表 + 无
    // 元表 + 字符串键」时逐级直查 `lua_h_getstr`，绕过 `lua_v_gettable` 的
    // MAXTAGLOOP 壳/Slot 句柄/重复 tag 分发。行为逐位同构论证：
    //  1. `lua_v_gettable` 对「表 + 无元表 + 字符串键」的轨迹 = `lua_h_get`(getstr)
    //     → 命中写 `L->cachedslot`(gval2slot) → 非 nil 直写返回；命中 nil 值槽 →
    //     `fasttm(null)` 恒空 → 原槽(nil)直写返回；miss(nilobject) → 不写
    //     cachedslot → nilobject 内容(tt=TNIL)直写返回。快支逐条对应；
    //  2. 任一前置不满足（元表非空 / 键非字符串 / L1 结果非表）→ 原路
    //     `lua_v_gettable` 完整接管（重查该级，含 TM 语义与 indexerror 路径；
    //     miss 重查罕见，仅首次解析/异常 env 形态）；
    //  3. L1 快支不触栈（getstr 无分配无 TM），`restorestack` 照旧等价；L2 接管后
    //     count==3 的第三级与原逻辑完全一致（对 L2 结果无条件原路查）。
    // global lookup for id0
    let key0 = &*k.add(id0);
    let mut l1_done = false;
    if (*env).metatable.is_null()
      && key0.is_string()
      && let Some(slot) = lua_h_getstr(&*env, key0.as_string_ptr())
    {
      let p = slot.as_const_ptr();
      (*l).cachedslot = gval2slot!(env, p);
      setobj_2_s!(l, res.as_ptr(), p);
      l1_done = true;
    }
    if !l1_done {
      let mut g = TValue::default();
      sethvalue!(l, &mut g, env);
      lua_v_gettable(l, Slot::from_ref(&g), Slot::from_ref(&*k.add(id0)), res);
    }

    // table lookup for id1
    if count < 2 {
      return;
    }

    res = Slot::from_raw(restorestack!(l, resp));
    if !propagatenil || !res.get().is_nil() {
      // L2 直查快支：前置与语义同 L1（见上）；count==3 时 L2 结果交由下方原第三级
      let key1 = &*k.add(id1);
      let t1 = res.get();
      let mut l2_done = false;
      if (*t1).is_table() {
        let h1 = (*t1).as_table_ptr();
        if (*h1).metatable.is_null() && key1.is_string() {
          match lua_h_getstr(&*h1, key1.as_string_ptr()) {
            Some(slot) => {
              let p = slot.as_const_ptr();
              (*l).cachedslot = gval2slot!(h1, p);
              setobj_2_s!(l, res.as_ptr(), p);
            }
            None => {
              setobj_2_s!(l, res.as_ptr(), LUA_O_NILOBJECT);
            }
          }
          l2_done = true;
        }
      }
      if !l2_done {
        lua_v_gettable(l, res, Slot::from_ref(key1), res);
      }
    }

    // table lookup for id2
    if count < 3 {
      return;
    }

    res = Slot::from_raw(restorestack!(l, resp));
    if !propagatenil || !res.get().is_nil() {
      lua_v_gettable(l, res, Slot::from_ref(&*k.add(id2)), res);
    }
  }
}
