use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

#[cfg(feature = "luai_gcmetrics")]
use crate::functions::record_gc_state_step::record_gc_state_step;
#[cfg(feature = "luai_gcmetrics")]
use crate::functions::start_gc_cycle_metrics::start_gc_cycle_metrics;
use crate::{
  functions::{
    finish_gc_cycle_metrics::finish_gc_cycle_metrics, gcstep::gcstep,
    getheaptrigger::getheaptrigger, lua_clock::lua_clock,
  },
  type_aliases::lua_state::lua_State,
};

#[inline]
unsafe fn gc_interrupt(l: *mut lua_State, state: c_int) {
  unsafe {
    let g = &*(*l).global;
    if let Some(interrupt) = g.cb.interrupt {
      interrupt(l, state);
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_step(l: *mut lua_State, assist: bool) -> usize {
  let _ = assist;
  unsafe {
    let g = (*l).global;

    let lim = ((*g).gcstepsize as usize * (*g).gcstepmul as usize) / 100;
    LUAU_ASSERT!((*g).totalbytes >= (*g).gc_threshold);
    let debt = (*g).totalbytes - (*g).gc_threshold;

    gc_interrupt(l, 0);

    if (*g).gcstate == 0 {
      (*g).gcstats.starttimestamp = lua_clock();
    }

    #[cfg(feature = "luai_gcmetrics")]
    let lasttimestamp = lua_clock();
    #[cfg(feature = "luai_gcmetrics")]
    if (*g).gcstate == 0 {
      start_gc_cycle_metrics(g);
    }

    let lastgcstate = (*g).gcstate as i32;

    let work = gcstep(l, lim);

    #[cfg(feature = "luai_gcmetrics")]
    {
      record_gc_state_step(g, lastgcstate, lua_clock() - lasttimestamp, assist, work);
    }

    let actualstepsize = (work * 100) / (*g).gcstepmul as usize;

    if (*g).gcstate == 0 {
      let heapgoal = ((*g).totalbytes / 100) * (*g).gcgoal as usize;
      let heaptrigger = getheaptrigger(g, heapgoal);

      (*g).gc_threshold = heaptrigger;

      (*g).gcstats.heapgoalsizebytes = heapgoal;
      (*g).gcstats.endtimestamp = lua_clock();
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
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaC_step")]
pub unsafe extern "C-unwind" fn lua_c_step_export(l: *mut lua_State, assist: bool) -> usize {
  unsafe { lua_c_step(l, assist) }
}

pub use lua_c_step as luaC_step;
