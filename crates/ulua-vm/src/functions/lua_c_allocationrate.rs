use ulua_common::clock_shim::monotonic_seconds;

use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global` 指向有效 global_State，其 `gcstate`/`totalbytes`/`gcstats.*` 时间戳字段
/// 已随 GC 生命周期初始化（读取只用于算速率，不写对象、不抛错、不分配）。cpp/VM/src/lapi.cpp:2180 lua_allocationrate。
pub unsafe fn lua_c_allocationrate(l: *mut LuaState) -> i64 {
  unsafe {
    let g = (*l).global;
    let duration_threshold: f64 = 1e-3; // avoid measuring intervals smaller than 1ms

    const GCS_ATOMIC: u8 = 3;

    if (*g).gcstate <= GCS_ATOMIC {
      let duration = monotonic_seconds() - (*g).gcstats.endtimestamp;

      if duration < duration_threshold {
        return -1;
      }

      return (((*g).totalbytes as f64 - (*g).gcstats.endtotalsizebytes as f64) / duration) as i64;
    }

    // totalbytes is unstable during the sweep, use the rate measured at the end of mark phase
    let duration = (*g).gcstats.atomicstarttimestamp - (*g).gcstats.endtimestamp;

    if duration < duration_threshold {
      return -1;
    }

    (((*g).gcstats.atomicstarttotalsizebytes as f64 - (*g).gcstats.endtotalsizebytes as f64)
      / duration) as i64
  }
}
