use crate::{
  functions::{getheapgrowth::getheapgrowth, getheaptriggererroroffset::getheaptriggererroroffset},
  records::global_state::global_State,
};

pub(crate) fn getheaptrigger(g: &mut global_State, heapgoal: usize) -> usize {
  const DURATION_THRESHOLD: f64 = 1e-3; // 避免测量小于 1ms 的时间间隔

  let gcstats = &g.gcstats;

  let allocationduration = gcstats.atomicstarttimestamp - gcstats.endtimestamp;

  if allocationduration < DURATION_THRESHOLD {
    return heapgoal;
  }

  let allocationrate = getheapgrowth(gcstats.atomicstarttotalsizebytes, gcstats.endtotalsizebytes)
    as f64
    / allocationduration;
  let markduration = gcstats.atomicstarttimestamp - gcstats.starttimestamp;

  let expectedgrowth = (markduration * allocationrate) as i64;
  let offset = getheaptriggererroroffset(g);
  // r16-d1：上式 `as i64` 是 f64→i64 饱和转位（Rust 定义：越域饱和到 i64::MIN/MAX，
  // 不 panic），gcstats 真实时钟抖动可令 `markduration * allocationrate` 溢出转
  // i64::MAX/MIN；cpp 侧（oracle，lgc.cpp）同式全程 double 域无溢出概念，但 Rust
  // 域里 `expectedgrowth + offset` 与 `heapgoal as i64 - _` 是整数运算，debug
  // profile 会「attempt to add with overflow」爆（门24 flaky 实证：tables.luau:681
  // obscuredalloc）。改 saturating 双点钉死。
  // 等价论证：①域内（和与差均可由 i64 表示）saturating_add/sub 与普通 +/- 逐位
  // 同值，行为不变；②域外时饱和值经下游钳位分支落回与原 clamp（cpp 无限精度
  // double 路径在 Rust 侧的既有等价体现）同侧——
  //   `growth_plus_offset` 饱和到 +i64::MAX ⇒ `heaptrigger = heapgoal - MAX` 为
  //   负巨值（可表示，不触发二次饱和）⇒ `< totalbytes` 分支 ⇒ totalbytes；
  //   与真值（heapgoal − 超出 MAX 的和，更负）同走该分支，净结果一致。
  //   `growth_plus_offset` 饱和到 −i64::MAX 量级（即 MIN）⇒ `saturating_sub` 的
  //   差为 +i64::MAX ⇒ `> heapgoal` 分支 ⇒ heapgoal；与真值（heapgoal − 低于
  //   MIN 的和，更正的巨值，同样 > heapgoal）同走该分支，净结果一致。
  // 故所有路径 debug 永不爆且钳位语义与 cpp double 域逐支等值。
  let growth_plus_offset = expectedgrowth.saturating_add(offset);
  let heaptrigger = (heapgoal as i64).saturating_sub(growth_plus_offset);

  // 清点（票 r16-d1 第 2 条）：本文件其余跨域算术无同类 debug 可爆路径——
  // `getheapgrowth(...) as f64 /`、`markduration * allocationrate` 均为 f64 运算
  // （浮点越域得 ±inf 不 panic，inf→`as i64` 按上注饱和）；`g.totalbytes as i64`、
  // `heapgoal as i64` 是 usize→i64 转位（值域为堆字节数，物理上 < 2^63，且转位
  // 本身非 panic 语义）；`heaptrigger as usize` 仅在 0 ≤ totalbytes ≤
  // heaptrigger ≤ heapgoal 区间命中，非负无绕回。无第二处需修。
  let totalbytes = g.totalbytes as i64;

  if heaptrigger < totalbytes {
    g.totalbytes
  } else if heaptrigger > heapgoal as i64 {
    heapgoal
  } else {
    heaptrigger as usize
  }
}

// 留证（先例普查，票 r16-d1）：仓内 `#[cfg(test)]` 单元测试先例集中于
// functions/ 下四个「无公开可达路径」的私有函数文件（utf_8_decode.rs:61、
// read.rs:28、luau_load.rs:106、lua_v_tostring.rs:65，均在测试模块前以注释
// 说明「为何不能迁 tests/」；另有 math_modf.rs:49 反例注记——其函数体已被
// 公开 fastcall 面复用故迁往 tests/）。本函数 `getheaptrigger` 为 `pub(crate)`
// GC 内部路径（唯一调用点 lua_c_step.rs:85），无 C ABI/公开 API 可达；经公开
// `lua_c_step` 触发需真实 GC 周期 + 真实时钟的 gcstats，无法确定性构造越域
// f64 乘积，故随先例在本文件尾置单元测试，直测算术窄面。
#[cfg(test)]
mod tests {
  use core::mem::zeroed;

  use super::*;
  use crate::records::gc_stats::GCStats;

  /// 构造仅承载 `gcstats`/`totalbytes` 的裸 `global_State`。
  ///
  /// SAFETY: `global_State` 为 `#[repr(C)]` 纯 POD（整型/裸指针/`Option<fn>`/
  /// 数组，无 bool、无 niched enum、无 Drop）；null/0 正是 `lua_newstate` 同款
  /// 哨兵初值约定（lstate 移植注记：裸指针均作空/未挂链哨兵），
  /// `frealloc = None` 永不被调——本测试只驱动 `getheaptrigger` 及其内联的
  /// `getheapgrowth`/`getheaptriggererroroffset`，二者仅读写 `gcstats` 与
  /// `totalbytes`。块为栈局部值，随作用域结束平凡析构，无资源泄漏面。
  fn state(gcstats: GCStats, totalbytes: usize) -> global_State {
    let mut g: global_State = unsafe { zeroed() };
    g.gcstats = gcstats;
    g.totalbytes = totalbytes;
    g
  }

  /// 正方向巨值：`markduration*allocationrate` = 9e18*1.1 = 9.9e18 > i64::MAX
  /// ⇒ `as i64` 饱和到 i64::MAX，`+ offset(=663，手验：error_kb=+1 时
  /// `(0.45*0.9 + 0.54*0.9/2.0)*1024 = 663.552 → 663`，f64→i64 向零截断)`
  /// 即门24 实爆的 debug 加法点。saturating 后 growth_plus_offset=MAX ⇒
  /// heaptrigger = heapgoal−MAX 为负巨值 ⇒ `< totalbytes` 支 ⇒ totalbytes。
  #[test]
  fn getheaptrigger_saturating_positive_overflow_clamps_to_totalbytes() {
    let gcstats = GCStats {
      endtimestamp: 0.0,
      atomicstarttimestamp: 2.0, // allocationduration = 2.0 ≥ 1e-3
      starttimestamp: 0.9,       // markduration = 1.1
      atomicstarttotalsizebytes: 18_000_000_000_000_000_000, // 1.8e19
      endtotalsizebytes: 0,      // growth 非负，LuauGcHeapShrinkFix 两支同值
      heapgoalsizebytes: 18_000_000_000_000_000_000 - 1024, // error_kb = 1
      ..Default::default()
    };
    let mut g = state(gcstats, 1000);
    // 原式在 debug 下于 `expectedgrowth + offset` panic（门24 复现形）；修后
    // 不爆且落 `< totalbytes` 支。
    assert_eq!(getheaptrigger(&mut g, 5000), 1000);
    assert_eq!(g.totalbytes, 1000); // 即 totalbytes 支
    assert_eq!(g.gcstats.triggerintegral, 1); // offset 采样旁证：error_kb=+1
  }

  /// 负方向巨值：markduration = −98 ⇒ 乘积 −8.82e20 < i64::MIN ⇒ `as i64`
  /// 饱和到 i64::MIN，`+ (-663)` 为同向越域加（原式同点爆）。saturating 后
  /// growth_plus_offset=MIN ⇒ heaptrigger = saturating_sub(heapgoal, MIN) =
  /// +MAX ⇒ `> heapgoal` 支 ⇒ heapgoal。
  #[test]
  fn getheaptrigger_saturating_negative_overflow_clamps_to_heapgoal() {
    let gcstats = GCStats {
      endtimestamp: 0.0,
      atomicstarttimestamp: 2.0, // allocationduration = 2.0 ≥ 1e-3
      starttimestamp: 100.0,     // markduration = 2.0 − 100.0 = −98
      atomicstarttotalsizebytes: 18_000_000_000_000_000_000,
      endtotalsizebytes: 0,
      // heapgoalsizebytes − 前项差 −10240 ⇒ wrapping_sub/1024 低 32 位截断 = −10
      heapgoalsizebytes: 18_000_000_000_000_000_000 + 10240,
      ..Default::default()
    };
    let mut g = state(gcstats, 1000);
    assert_eq!(getheaptrigger(&mut g, 5000), 5000); // heapgoal 支
    assert_eq!(g.gcstats.triggerintegral, -10); // offset 采样旁证：error_kb=−10
  }

  /// 域内对照：乘积/和/差均可表示，saturating 与普通 +/- 逐位同值——
  /// expectedgrowth = 500、offset = 0 ⇒ heaptrigger = 10000−500 = 9500，
  /// 落在 [totalbytes=100, heapgoal=10000] 区间 ⇒ 原式值支（钳位次序未动）。
  #[test]
  fn getheaptrigger_in_range_returns_exact_original_value() {
    let gcstats = GCStats {
      endtimestamp: 0.0,
      atomicstarttimestamp: 2.0, // allocationduration = 2.0
      starttimestamp: 1.0,       // markduration = 1.0
      atomicstarttotalsizebytes: 1000,
      endtotalsizebytes: 0,    // rate = 1000/2 = 500 ⇒ growth = 500
      heapgoalsizebytes: 1000, // error_kb = 0 ⇒ offset = 0
      ..Default::default()
    };
    let mut g = state(gcstats, 100);
    assert_eq!(getheaptrigger(&mut g, 10_000), 10_000 - 500);
  }
}
