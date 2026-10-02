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
