use ulua_common::clock_shim::monotonic_seconds;

use crate::{functions::getheapgrowth::getheapgrowth, records::lua_state::LuaState};

/// 读分配速率（`lua_allocationrate`）。`l` 以引用传入（存活由类型保证）；其 `global`
/// 指向同存活期的有效 `global_State`、GC 统计字段已随 GC 生命周期初始化均为
/// `lua_State` 结构不变量（只读算速率，不写对象、不抛错、不分配）。
/// cpp/VM/src/lapi.cpp:2180 lua_allocationrate。
pub fn lua_c_allocationrate(l: &LuaState) -> i64 {
  let g = l.global;
  const DURATION_THRESHOLD: f64 = 1e-3; // 避免测量小于 1ms 的时间间隔

  const GCS_ATOMIC: u8 = 3;

  // SAFETY: `g` 为存活 LuaState 挂接的 global_State（结构不变量），块内只读
  // gcstate/totalbytes/gcstats 统计字段。
  unsafe {
    let (current, duration) = if (*g).gcstate <= GCS_ATOMIC {
      (
        (*g).totalbytes,
        monotonic_seconds() - (*g).gcstats.endtimestamp,
      )
    } else {
      // 清扫阶段 totalbytes 不稳定，改用标记阶段结束时测得的速率
      (
        (*g).gcstats.atomicstarttotalsizebytes,
        (*g).gcstats.atomicstarttimestamp - (*g).gcstats.endtimestamp,
      )
    };

    if duration < DURATION_THRESHOLD {
      return -1;
    }

    (getheapgrowth(current, (*g).gcstats.endtotalsizebytes) as f64 / duration) as i64
  }
}
