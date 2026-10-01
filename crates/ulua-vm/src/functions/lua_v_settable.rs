//! Source: `VM/src/lvmutils.cpp:182-240` (hand-ported)

use core::ptr::null;

use ulua_common::fflag;

use crate::{
  enums::tms::TMS,
  functions::{
    call_tm::call_tm, lua_g_indexerror::lua_g_indexerror,
    lua_g_missingmembererror::lua_g_missingmembererror, lua_g_readonlyerror::check_writable,
    lua_h_get::lua_h_get, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    fasttm::fasttm, gval_2_slot::gval2slot, lua_c_barrier::lua_c_barrier,
    lua_c_barriert::luaC_barriert, lua_g_runerror::lua_g_runerror, lua_h_setslot::lua_h_setslot,
    maxtagloop::MAXTAGLOOP, objectvalue::objectvalue, setobj::setobj, setobj_2_class::setobj2class,
    setobj_2_t::setobj2t,
  },
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

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
