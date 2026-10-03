//! Source: `VM/src/lvmutils.cpp:292-350` (hand-ported)

use core::ptr::null;

use ulua_common::fflag;

use crate::{
  enums::tms::TMS,
  functions::{
    call_tm::call_tm, index_chain_cache::index_chain_write, lua_g_indexerror::lua_g_indexerror,
    lua_g_missingmembererror::lua_g_missingmembererror, lua_g_readonlyerror::check_writable,
    lua_h_get::lua_h_get, lua_o_rawequal_key::lua_o_rawequal_key,
    lua_t_gettmbyobj::lua_t_gettmbyobj, mainposition::mainposition, newkey::newkey_with_mp,
    walk_nodes::walk_nodes,
  },
  macros::{
    fasttm::fasttm, gkey::gval, gval_2_slot::gval2slot, invalidate_t_mcache::invalidate_tmcache,
    lua_c_barrier::lua_c_barrier, lua_c_barriert::luaC_barriert, lua_g_runerror::lua_g_runerror,
    lua_h_setslot::lua_h_setslot, luai_numeq::luai_numeq, luai_numisnan::luai_numisnan,
    maxtagloop::MAXTAGLOOP, objectvalue::objectvalue, setobj::setobj, setobj_2_class::setobj2class,
    setobj_2_t::setobj2t,
  },
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable, slot::Slot},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 数字键 + 无元表写快支主体（[`lua_v_settable`] 入口判定通过后进入）。
///
/// 行为逐位同构论证（与原 `lua_h_get` 探测 + `lua_h_setslot!` 写段对照）：
///  1. 探测谓词与 `lua_h_get` 对数字键的两分支逐位一致——整数键先 array 段判定
///     （`luaH_getnum` 口径）、hash 段用 `is_number && numeq` 谓词；非整数键用
///     `lua_o_rawequal_key` 谓词，均与 `lua_h_get` 现行分发相同；
///  2. metatable 已由入口判空，`fasttm(l, null, TmNewIndex)` 恒空，原路径 `tm`
///     必为 null、必走写段，跳过该次空查不改变任何可观察行为；
///  3. 命中 nil 值槽 → 返回槽指针（`lua_h_setslot!` 的旧槽复用支）；完全 miss →
///     `newkey_with_mp` 建键，传入的 `mp` 与 `newkey` 壳内重算值逐位同源（同 key
///     同表状态 `mainposition` 结果唯一）；
///  4. 写段（check_writable → invalidateTMcache/index_chain_write → cachedslot →
///     setobj2t → barriert）的判定顺序与屏障时序原样保留。
///
/// `#[inline(never)]` 的理由：本函数若内联回 `lua_v_settable`，后者体积膨胀会连锁
/// 改变 `lua_v_gettable → lua_h_get` 等热链的内联决策与解释器派发环布局（inherit3
/// 复测归因 +18% 即该连锁——`lua_h_get` 丢失内联后继承链查找每跳多付一次真实
/// call）。入口判定留在调用方，本函数只收探测与写段，`l` 为存活 `LuaState`、`h`
/// 为存活 `LuaTable`（元表空、键为非 NaN 数字），`val` 为取值源存活槽。
#[inline(never)]
unsafe fn settable_num_fastpath(
  l: *mut LuaState,
  h: *mut LuaTable,
  keyv: &TValue,
  val: *const TValue,
) -> bool {
  // SAFETY: 契约由调用方（lua_v_settable）保证：l/h 存活且互踞有效内存，探测链
  // 走与节点算术均落在表节点数组界内（与 lua_h_get 同一前提），写段屏障经 val
  // 读出的值指针存活至屏障完成。
  unsafe {
    let n = keyv.as_number();
    let k = n as i32;
    let exact = luai_numeq(k as f64, n);
    // 探测：`Ok(值槽)` 命中（含 nil 值槽），`Err(主位桶)` 完全 miss
    let probe: Result<*mut TValue, *mut LuaNode> =
      if exact && (k as u32).wrapping_sub(1) < (*h).sizearray as u32 {
        Ok((*h).array.add((k - 1) as usize))
      } else {
        let mp = mainposition(h, keyv);
        let hit = if exact {
          // 整数键 hash 段：`luaH_getnum` 谓词口径
          walk_nodes(mp, |node| {
            if (*node).key.is_number() && luai_numeq((*node).key.as_number(), n) {
              Some(gval!(node))
            } else {
              None
            }
          })
        } else {
          // 非整数数值键：`lua_h_get` hash 慢路的 rawequal 谓词口径
          walk_nodes(mp, |node| {
            if lua_o_rawequal_key(&(*node).key, keyv) != 0 {
              Some(gval!(node))
            } else {
              None
            }
          })
        };
        hit.ok_or(mp)
      };

    let newval = match probe {
      // 写段收敛为单份：Ok/Err 双份展开会加倍内联体积，重蹈布局漂移
      Ok(oldval) => oldval,
      Err(mp) => {
        // 新键落位（可能 rehash），`mp` 为写前探测预算的主位桶
        newkey_with_mp(l, h, keyv, mp)
      }
    };

    check_writable(l, h);
    invalidate_tmcache(&*h);
    index_chain_write(h);
    (*l).cachedslot = gval2slot!(h, newval);
    setobj2t!(l, newval, val);
    luaC_barriert!(l, h, val);
    true
  }
}

/// # Safety
/// `l` 指向存活 `LuaState`；`t`/`key`/`val` 为对齐可读的槽句柄（三侧在本函数内均只读：
/// `key`/`val` 为取值源，`val` 原语义为当前帧内待写入的值源栈槽）；表槽区可写且 GC
/// 屏障（`luaC_barriert!`）允许回写目标表——屏障经句柄读出的值指针把白色值置灰，
/// 前提与旧裸指针形态一致：`val` 所指内存活至屏障完成。句柄跨调用存续服从
/// 「扩容先行、借用后派生」不变量，调用期间槽地址不得迁移。cpp lvmutils.cpp:292.
pub unsafe fn lua_v_settable(l: *mut LuaState, mut t: Slot<'_>, key: Slot<'_>, val: Slot<'_>) {
  // SAFETY: 契约保证 `l` 为存活调用帧、槽句柄可读/可写且对齐，块内取值、TM 调用与报错路径均在该帧栈界内
  unsafe {
    let mut temp = TValue::default();
    // 保留计数重复：MAXTAGLOOP 是 __newindex 链防失控的重试预算上限，不是数组下标；
    // 每轮沿元方法链把 t 换成下一级 TM 对象再走，无可迭代的数据序列
    for _ in 0..MAXTAGLOOP {
      let mut tm: *const TValue = null();
      if t.get().is_table() {
        let h = t.get().as_table_ptr();

        // 数字键 + 无元表快支（本 fork 实测扩展，cpp 无对应支）：写前探测一次拿到
        // 「值槽」或「miss 主位桶」，miss 时直传 [`newkey_with_mp`]，消除原路径
        // `lua_h_get` 探测与 `newkey` 壳内 `mainposition` 的双重哈希定位。入口判定
        // 留在本函数（一次 metatable 读 + tag 比较 + NaN 排除），探测与写段整体
        // 外移到 [`settable_num_fastpath`]：本函数体积若随快支膨胀，会连锁改变
        // `lua_v_gettable → lua_h_get` 等热链的内联决策与派发环布局（inherit3
        // 复测归因 +18% 即该连锁，见 fastpath 函数文档）。行为逐位同构论证见彼处。
        // NaN 键不入快支：原路径 `lua_h_newkey` 对 NaN 键抛「table index is NaN」，
        // 快支无该校验，排除后走原路保持错误行为逐位一致（非 NaN 数字键才落快支）。
        if (*h).metatable.is_null()
          && key.get().is_number()
          && !luai_numisnan(key.get().as_number())
          && settable_num_fastpath(l, h, key.get(), val.as_const_ptr())
        {
          return;
        }

        let oldval = lua_h_get(h, key.get());

        if (*oldval).is_nil() {
          tm = fasttm(l, (*h).metatable, TMS::TmNewIndex);
        }

        if !(*oldval).is_nil() || tm.is_null() {
          check_writable(l, h);

          // ⇔ cpp lvmutils.cpp:312-317，屏障时序逐位同序、不合并：
          //   :312 lua_h_setslot（invalidateTMcache + 复用 oldval 槽或 luaH_newkey 建键，
          //        可能 rehash——此后 newval 才是本轮唯一有效槽指针）
          //   :314 cachedslot 记录（纯整数侧写，不动内存值）
          //   :316 setobj2t 写值（=setobj，本身无屏障，lobject.h:266）
          //   :317 luaC_barriert 表屏障——必须在值落槽之后：屏障经指针追踪 `val`
          //        句柄读出的值把白色值置灰，写前触发等于漏灰。三步在 cpp 同函数相邻
          //        成对，但仍保留分步形态与 SETOBJ/barrier 宏语义一一对应（C3 规程：
          //        不发明单函数收敛，屏障时机只照抄 cpp）。
          let newval = lua_h_setslot!(l, h, oldval, key.get());

          (*l).cachedslot = gval2slot!(h, newval);

          setobj2t!(l, newval, val.as_const_ptr());
          luaC_barriert!(l, h, val.as_const_ptr());
          return;
        }
      } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && t.get().is_object() {
        let inst = objectvalue!(t.as_const_ptr());
        let offset = lua_h_get((*(*inst).lclass).memberstooffset, key.get());
        if (*offset).is_nil() {
          lua_g_missingmembererror(l, t.as_const_ptr(), key.as_const_ptr());
        }
        // cpp:329-334 以 uint32_t 承载偏移：负值（memberstooffset 被破坏）
        // 回绕成大数，必然落入 indexerror 干净报错，而非 i32 负数旁路通向
        // members 数组 wild write
        let offsetnum = (*offset).as_number() as u32;
        if offsetnum >= (*(*inst).lclass).numberofinstancemembers as u32 {
          lua_g_indexerror(l, t.as_const_ptr(), key.as_const_ptr());
        }
        setobj2class!(
          l,
          (*inst).members.add(offsetnum as usize),
          val.as_const_ptr()
        );
        lua_c_barrier!(l, inst, val.as_const_ptr());
        return;
      } else {
        tm = lua_t_gettmbyobj(l, t.as_const_ptr(), TMS::TmNewIndex);
        if (*tm).is_nil() {
          lua_g_indexerror(l, t.as_const_ptr(), key.as_const_ptr());
        }
      }

      if (*tm).is_function() {
        call_tm(
          l,
          tm,
          t.as_const_ptr(),
          key.as_const_ptr(),
          val.as_const_ptr(),
        );
        return;
      }
      // `temp` 跨迭代存活：`t` 指回它后在下一轮被覆写，与 cpp 局部 temp 同构
      setobj!(l, &mut temp, tm);
      // SAFETY: `temp` 为函数局部独占可写 TValue，跨迭代地址稳定且本轮写毕后即被
      // 下一轮读面消费（与原 `t = &temp` 裸指针形态同址同序）；`&raw mut` 不经过
      // 借用检查保留旧窗口，重复派生句柄间无 Rust 别名保证（模块纪律）。
      t = Slot::from_raw(&raw mut temp);
    }
    lua_g_runerror!(l, "'__newindex' chain too long; possible loop");
  }
}

/// # Safety
/// C ABI 导出壳：签名与符号不动，在裸指针边界处显式重建句柄后转调 [`lua_v_settable`]，
/// 前置条件与该函数相同。
pub unsafe extern "C-unwind" fn lua_v_settable_export(
  l: *mut LuaState,
  t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  // SAFETY: 导出壳原样转调同契约 `lua_v_settable`；l/t/key/val 满足其前置。
  // 句柄重建归本壳：`t` 为只读存活 TValue（被调方仅用读面，`from_ref` 纪律一致）；
  // `key`/`val` 为栈槽/受栈持有内存，调用期间不迁移，解引用窗口止于本调用。
  unsafe {
    lua_v_settable(
      l,
      Slot::from_ref(&*t),
      Slot::from_raw(key),
      Slot::from_raw(val),
    );
  }
}
