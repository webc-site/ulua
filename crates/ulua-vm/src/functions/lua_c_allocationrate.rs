use ulua_common::clock_shim::monotonic_seconds;

use crate::records::lua_state::LuaState;

/// 读分配速率（`lua_allocationrate`）。`l` 以引用传入（存活由类型保证）；其 `global`
/// 指向同存活期的有效 `global_State`、GC 统计字段已随 GC 生命周期初始化均为
/// `lua_State` 结构不变量（只读算速率，不写对象、不抛错、不分配）。
/// cpp/VM/src/lapi.cpp:2180 lua_allocationrate。
pub fn lua_c_allocationrate(l: &LuaState) -> i64 {
  let g = l.global;
  let duration_threshold: f64 = 1e-3; // avoid measuring intervals smaller than 1ms

  const GCS_ATOMIC: u8 = 3;

  // SAFETY: `g` 为存活 LuaState 挂接的 global_State（结构不变量），块内只读
  // gcstate/totalbytes/gcstats 统计字段。
  unsafe {
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
