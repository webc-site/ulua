//! Source: `VM/src/lvmexecute.cpp:3757-3841` (hand-ported)

use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    c_slice_mut, copy_results_pop_frame::copy_results_pop_frame, lua_v_tryfunc_tm::lua_v_tryfunc_tm,
  },
  macros::{
    incr_ci::incr_ci, lua_callinfo_native::LUA_CALLINFO_NATIVE,
    lua_d_checkstackfornewci::lua_d_checkstackfornewci, pcrc::PCRC, pcrlua::PCRLUA,
    pcryield::PCRYIELD,
  },
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 调用方须保证：`l` 存活、CallInfo 数组尚有空槽供 incr_ci!、`func` 为可读栈槽（非函数时
/// tryfuncTM 经 `l` 抛错，须在受保护帧内）；栈尾可经 lua_d_checkstackfornewci 扩容容纳新帧
/// stacksize；`nresults` 与调用方预留结果槽一致（负值为 MULTRET）；C 闭包 `f(l)` 可再入 VM/yield。
///
/// 帧参数形态台账（B2-2b）：`func` 保留 [`StkId`]——入参即落库 `(*ci).func`/派生
/// `base = func + 1`（CallInfo 裸字段为 `records/call_info.rs` DELIBERATE DEVIATION
/// 裁决），Slot 化只剩 from_raw→as_ptr 零收益往返且外溢 performcall/resume 消费点
/// （同 B2-1 copy_results 判定，§9.4）；返回 i32 为 PCRLUA/PCRC/PCRYIELD 协议值，非布尔。
/// cpp lvmexecute.cpp:3856 `luau_precall`
/// C++ `int luau_precall(LuaState* l, StkId func, int nresults)`.
pub(crate) unsafe fn luau_precall(l: *mut LuaState, func: StkId, nresults: i32) -> i32 {
  // SAFETY: 契约保证 `l` 存活、func 槽可读且 nresults 符合调用方协议；新建 ci 与 top 推进均在 CallInfo 数组与栈界内
  unsafe {
    if !(*func).is_function() {
      lua_v_tryfunc_tm(l, Slot::from_raw(func));
      // l->top is incremented by tryfuncTM
    }

    // 边界实参隐式转换即派生只读借用（`(*func)` 的 place 经 &self 收口，
    // `as *const TValue` 冗余 cast 退役）
    let ccl = (*func).as_closure_ptr();

    incr_ci!(l);
    let ci = (*l).ci;
    (*ci).func = func;
    (*ci).base = func.add(1);
    (*ci).top = (*l).top.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;

    (*l).base = (*ci).base;
    // Note: l->top is assigned externally

    lua_d_checkstackfornewci(l, (*ccl).stacksize as i32);
    LUAU_ASSERT!((*ci).top <= (*l).stack_last);

    if (*ccl).is_c == 0 {
      let p = {
        let l = &(*ccl).inner.l;
        l.p
      };

      // fill unused parameters with nil：未提供实参的形参槽补 nil。指针步进折叠为
      // c_slice_mut 视图 + iter_mut（B1b 切片化先例；槽距经同分配内 offset_from 求取，
      // top ≥ argend 时截零，与旧 `argi < argend` 谓词逐位一致；setnilvalue! 即
      // TValue::set_nil，写面与旧宏同体）
      let argend = (*l).base.add((*p).numparams as usize);
      let nfill = argend.offset_from((*l).top).max(0) as usize;
      for slot in c_slice_mut((*l).top, nfill).iter_mut() {
        slot.set_nil();
      }
      (*l).top = if (*p).is_vararg != 0 {
        (*l).top.add(nfill)
      } else {
        (*ci).top
      };

      (*ci).savedpc = (*p).code;

      // VM_HAS_NATIVE
      if (*p).exectarget != 0 && !(*p).execdata.is_null() {
        (*ci).flags = LUA_CALLINFO_NATIVE as u32;
      }

      PCRLUA
    } else {
      let f = {
        let c = &(*ccl).inner.c;
        c.f
      };
      let n = match f {
        Some(f) => f(l),
        None => 0,
      };

      // yield
      if n < 0 {
        return PCRYIELD;
      }

      // ci is our callinfo, cip is our parent：C 路径返回收尾——把 [top-n, top) 按
      // nresults 上限拷回 func（不足补 nil）并弹帧，`l->top` 恒收在结果尾
      // （拷回-补 nil-弹帧三连单源见 copy_results_pop_frame；
      // TODO: it might be worthwhile to handle the case when nresults==b explicitly?）
      (*l).top = copy_results_pop_frame(l, (*l).top.sub(n.max(0) as usize), (*l).top, nresults);

      PCRC
    }
  }
}
