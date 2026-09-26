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
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 指向存活 `LuaState`；`t`/`key`/`val` 指向对齐可读的 `TValue`（`key`/`val` 只读，
/// `val` 为当前帧内待写入的值源栈槽），表槽区可写且 GC 屏障
/// （`luaC_barriert!`）允许回写目标表。cpp lvmutils.cpp:292.
pub unsafe fn lua_v_settable(
  l: *mut LuaState,
  t: *const TValue,
  key: *const TValue,
  val: *const TValue,
) {
  let mut t = t;
  // Safety: 契约保证 `l` 为存活调用帧、TValue 指针可读/可写且对齐，块内取值、TM 调用与报错路径均在该帧栈界内
  unsafe {
    let mut temp = TValue::default();
    // 保留计数重复：MAXTAGLOOP 是 __newindex 链防失控的重试预算上限，不是数组下标；
    // 每轮沿元方法链把 t 换成下一级 TM 对象再走，无可迭代的数据序列
    for _ in 0..MAXTAGLOOP {
      let mut tm: *const TValue = null();
      if (*t).is_table() {
        let h = (*t).as_table_ptr();

        let oldval = lua_h_get(h, key);

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
          //        把白色值置灰，写前触发等于漏灰。三步在 cpp 同函数相邻成对，
          //        但仍保留分步形态与 SETOBJ/barrier 宏语义一一对应（C3 规程：
          //        不发明单函数收敛，屏障时机只照抄 cpp）。
          let newval = lua_h_setslot!(l, h, oldval, key);

          (*l).cachedslot = gval2slot!(h, newval);

          setobj2t!(l, newval, val);
          luaC_barriert!(l, h, val);
          return;
        }
      } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && (*t).is_object() {
        let inst = objectvalue!(t);
        let offset = lua_h_get((*(*inst).lclass).memberstooffset, key);
        if (*offset).is_nil() {
          lua_g_missingmembererror(l, t, key);
        }
        // cpp:329-334 以 uint32_t 承载偏移：负值（memberstooffset 被破坏）
        // 回绕成大数，必然落入 indexerror 干净报错，而非 i32 负数旁路通向
        // members 数组 wild write
        let offsetnum = (*offset).as_number() as u32;
        if offsetnum >= (*(*inst).lclass).numberofinstancemembers as u32 {
          lua_g_indexerror(l, t, key);
        }
        setobj2class!(l, (*inst).members.add(offsetnum as usize), val);
        lua_c_barrier!(l, inst, val);
        return;
      } else {
        tm = lua_t_gettmbyobj(l, t, TMS::TmNewIndex);
        if (*tm).is_nil() {
          lua_g_indexerror(l, t, key);
        }
      }

      if (*tm).is_function() {
        call_tm(l, tm, t, key, val);
        return;
      }
      // `temp` 跨迭代存活：`t` 指回它后在下一轮被覆写，与 cpp 局部 temp 同构
      setobj!(l, &mut temp, tm);
      t = &temp;
    }
    lua_g_runerror!(l, "'__newindex' chain too long; possible loop");
  }
}

/// # Safety
/// C ABI 导出壳：逐参数原样透传，前置条件与 [`lua_v_settable`] 相同。
pub unsafe extern "C-unwind" fn lua_v_settable_export(
  l: *mut LuaState,
  t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  // Safety: 导出壳原样转发同契约 `lua_v_settable`；l/t/key/val 满足其前置
  unsafe {
    lua_v_settable(l, t, key, val);
  }
}
