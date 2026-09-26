//! Source: `VM/src/lvmexecute.cpp:3757-3841` (hand-ported)

use core::{ptr::null, slice::from_raw_parts_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_v_tryfunc_tm::lua_v_tryfunc_tm,
  macros::{
    incr_ci::incr_ci, lua_callinfo_native::LUA_CALLINFO_NATIVE,
    lua_d_checkstackfornewci::lua_d_checkstackfornewci, pcrc::PCRC, pcrlua::PCRLUA,
    pcryield::PCRYIELD, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 调用方须保证：`l` 存活、CallInfo 数组尚有空槽供 incr_ci!、`func` 为可读栈槽（非函数时
/// tryfuncTM 经 `l` 抛错，须在受保护帧内）；栈尾可经 lua_d_checkstackfornewci 扩容容纳新帧
/// stacksize；`nresults` 与调用方预留结果槽一致（负值为 MULTRET）；C 闭包 `f(l)` 可再入 VM/yield。
/// cpp lvmexecute.cpp:3856 `luau_precall`
/// C++ `int luau_precall(LuaState* l, StkId func, int nresults)`.
pub(crate) unsafe fn luau_precall(l: *mut LuaState, func: StkId, nresults: i32) -> i32 {
  // Safety: 契约保证 `l` 存活、func 槽可读且 nresults 符合调用方协议；新建 ci 与 top 推进均在 CallInfo 数组与栈界内
  unsafe {
    if !(*func).is_function() {
      lua_v_tryfunc_tm(l, func);
      // l->top is incremented by tryfuncTM
    }

    let ccl = (*(func as *const TValue)).as_closure_ptr();

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

      // fill unused parameters with nil
      // 缺失参数个数 = argend - top（负值按 0，与原 `while argi < argend` 游走
      // 严格等价），一次算清后定界切片逐格补 nil
      let argi = (*l).top;
      let argend = (*l).base.add((*p).numparams as usize);
      // Safety: [top, base+numparams) 由上方 lua_d_checkstackfornewci 扩栈保证为存活栈区间，
      // offset_from 的差值即该区间元素数，故切片长度不超过实占格数
      let missing = argend.offset_from(argi).max(0) as usize;
      for slot in from_raw_parts_mut(argi, missing) {
        setnilvalue!(slot); // complete missing arguments
      }
      let argi = argi.add(missing);
      (*l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

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

      // ci is our callinfo, cip is our parent
      let ci = (*l).ci;
      let cip = ci.sub(1);

      // copy return values into parent stack (but only up to nresults!),
      // fill the rest with nil
      // TODO: it might be worthwhile to handle the case when nresults==b explicitly?
      let res = (*ci).func;
      let vali = (*l).top.sub(n.max(0) as usize);
      let valend = (*l).top;

      // 源窗口元素数：契约保证 top-n <= top 且同属存活栈，一次 offset_from
      // 算清，取代原「指针逐格比较」的手写游走
      let avail = valend.offset_from(vali).max(0) as usize;
      // nresults<0（MULTRET）时全量拷贝（C++ `i != 0` 恒真），否则按 nresults 截断
      let ncopy = if nresults < 0 {
        avail
      } else {
        avail.min(nresults as usize)
      };
      // 保留索引遍历：j 同时是 dst 与 src 两个窗口的偏移，且窗口可重叠（C 帧返回的
      // 结果就紧跟在 res 之后，src = dst + Δ），借用成 &mut/& 切片会构成别名冲突，
      // 只能按正序逐格「先读后写」走指针（cpp lvmexecute.cpp:3922）
      for j in 0..ncopy {
        setobj_2_s!(l, res.add(j), vali.add(j) as *const TValue);
      }
      let mut res = res.add(ncopy);
      // 补 nil 数：仅 nresults>0 且源不足时补差额（等价原 `while i > 0` 尾循环）
      let nfill = if nresults > 0 {
        (nresults as usize) - ncopy
      } else {
        0
      };
      // Safety: res 起的 nfill 格属调用方（父帧）预留的结果槽，落在 func..父帧 top 内
      for slot in from_raw_parts_mut(res, nfill) {
        setnilvalue!(slot);
      }
      res = res.add(nfill);

      // pop the stack frame
      (*l).ci = cip;
      (*l).base = (*cip).base;
      (*l).top = res;

      PCRC
    }
  }
}
