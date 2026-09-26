use ulua_common::{clock_shim::monotonic_seconds, fflag, macros::luau_assert::LUAU_ASSERT};

#[cfg(feature = "luai_gcmetrics")]
use crate::functions::record_gc_state_step::record_gc_state_step;
#[cfg(feature = "luai_gcmetrics")]
use crate::functions::start_gc_cycle_metrics::start_gc_cycle_metrics;
use crate::{
  functions::{
    finish_gc_cycle_metrics::finish_gc_cycle_metrics, gcstep::gcstep,
    getheaptrigger::getheaptrigger,
  },
  macros::{gc_percent_base::GC_PERCENT_BASE, gc_spause::GCSPAUSE},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须存活且 `(*l).global` 可读；`g.cb.interrupt` 若非空必须是符合 C ABI、可在 VM 栈上安全重入
/// 的解释器回调（可能抛错 unwind）。挂悬垂函数指针会直接跳转到非法地址。cpp lgc.cpp:130（GC_INTERRUPT 宏）。
#[inline]
unsafe fn gc_interrupt(l: *mut LuaState, state: i32) {
  // Safety: 契约保证 `l` 存活且中断回调（若有）按 C 约定可安全调用，块内仅透传 state
  unsafe {
    let g = &*(*l).global;
    if let Some(interrupt) = g.cb.interrupt {
      interrupt(l, state);
    }
  }
}

/// # Safety
/// `l` 须存活且处于允许 GC 步进的安全点：分配记账自洽（`totalbytes >= gc_threshold`，否则 debt
/// 减法回绕）、灰/白不变式成立，且调用方未在裸指针上持有仅靠本步存活的可回收对象（本函数可能清扫）。
/// 违反会误清活对象或使 GC 状态机错乱。cpp lgc.cpp:1301。
pub unsafe fn lua_c_step(l: *mut LuaState, assist: bool) -> usize {
  unsafe {
    let g = (*l).global;

    let mut lim = ((*g).gcstepsize as usize * (*g).gcstepmul as usize) / GC_PERCENT_BASE;
    LUAU_ASSERT!((*g).totalbytes >= (*g).gc_threshold);
    let debt = (*g).totalbytes - (*g).gc_threshold;
    // lgc.cpp:1303-1306：分配速度超过 GC 步进需要 assist 时，按欠账放大步长。
    // 本移植为单精度向量（LUA_VECTOR_DOUBLE == 0），条件只剩该旗标。
    if fflag::LuauBackedgeHeapCheck.get() && assist {
      let need = debt * (*g).gcstepmul as usize / GC_PERCENT_BASE;

      if need > lim {
        lim = need;
      }
    }

    gc_interrupt(l, GCSPAUSE);

    // 新周期起点
    if (*g).gcstate as i32 == GCSPAUSE {
      (*g).gcstats.starttimestamp = monotonic_seconds();
    }

    #[cfg(feature = "luai_gcmetrics")]
    let lasttimestamp = monotonic_seconds();
    #[cfg(feature = "luai_gcmetrics")]
    if (*g).gcstate as i32 == GCSPAUSE {
      start_gc_cycle_metrics(g);
    }

    let lastgcstate = (*g).gcstate as i32;

    let work = gcstep(l, lim);

    #[cfg(feature = "luai_gcmetrics")]
    {
      record_gc_state_step(
        g,
        lastgcstate,
        monotonic_seconds() - lasttimestamp,
        assist,
        work,
      );
    }

    let actualstepsize = (work * GC_PERCENT_BASE) / (*g).gcstepmul as usize;

    // 旧周期终点
    if (*g).gcstate as i32 == GCSPAUSE {
      let heapgoal = ((*g).totalbytes / GC_PERCENT_BASE) * (*g).gcgoal as usize;
      let heaptrigger = getheaptrigger(&mut *g, heapgoal);

      (*g).gc_threshold = heaptrigger;

      (*g).gcstats.heapgoalsizebytes = heapgoal;
      (*g).gcstats.endtimestamp = monotonic_seconds();
      (*g).gcstats.endtotalsizebytes = (*g).totalbytes;

      finish_gc_cycle_metrics(g);
    } else {
      (*g).gc_threshold = (*g).totalbytes + actualstepsize;

      if (*g).gc_threshold >= debt {
        (*g).gc_threshold -= debt;
      }
    }

    gc_interrupt(l, lastgcstate);

    actualstepsize
  }
}

/// # Safety
/// C ABI 导出壳：参数原样转发内层 `lua_c_step`，须满足其全部前提（存活 `l`、GC 安全点、记账自洽）；
/// 该边界还可能经 interrupt 回调 unwind 出 C。cpp lgc.cpp:1301。
pub unsafe extern "C-unwind" fn lua_c_step_export(l: *mut LuaState, assist: bool) -> usize {
  unsafe { lua_c_step(l, assist) }
}
