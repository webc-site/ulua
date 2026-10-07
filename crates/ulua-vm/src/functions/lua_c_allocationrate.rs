use ulua_common::clock_shim::monotonic_seconds;

use crate::{functions::getheapgrowth::getheapgrowth, records::lua_state::LuaState};

/// 读分配速率（`lua_allocationrate`）。`l` 以引用传入（存活由类型保证）；其 global
/// 开场字段指向同存活期的有效 `global_State`、GC 统计字段已随 GC 生命周期初始化均为
/// `lua_State` 结构不变量（只读算速率，不写对象、不抛错、不分配）。
/// cpp/VM/src/lapi.cpp:2180 lua_allocationrate。
pub fn lua_c_allocationrate(l: &LuaState) -> i64 {
  const DURATION_THRESHOLD: f64 = 1e-3; // 避免测量小于 1ms 的时间间隔

  const GCS_ATOMIC: u8 = 3;

  // r15-v1 迁移点：场域读数全经 `LuaState::gs_ref` 入口句柄语句内即取即用（见其
  // 契约）——本函数纯读、窗内仅单调钟读数与无副作用算术（无重入调用），原散点
  // 裸解引用逐位换为只读视图，借用语义与原形等价（零行为改动）；外层 unsafe
  // 块因场域访问已 safe 化而整块消解。
  let (current, duration) = if l.gs_ref().gcstate <= GCS_ATOMIC {
    (
      l.gs_ref().totalbytes,
      monotonic_seconds() - l.gs_ref().gcstats.endtimestamp,
    )
  } else {
    // 清扫阶段 totalbytes 不稳定，改用标记阶段结束时测得的速率
    (
      l.gs_ref().gcstats.atomicstarttotalsizebytes,
      l.gs_ref().gcstats.atomicstarttimestamp - l.gs_ref().gcstats.endtimestamp,
    )
  };

  if duration < DURATION_THRESHOLD {
    return -1;
  }

  (getheapgrowth(current, l.gs_ref().gcstats.endtotalsizebytes) as f64 / duration) as i64
}
