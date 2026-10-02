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

  // cpp lgc.cpp getheaptrigger 同式 `heapgoal - (expectedgrowth + offset)`。堆在原子
  // 阶段收缩（shrinkstack 缩栈）时 atomicstarttotalsize 可小于 endtotalsize：
  // LuauGcHeapShrinkFix 关（源默认）走裸差，usize 回绕把 rate 抬到 ≈2^64/时长，
  // `as i64` 饱和成 i64::MAX 后 `expectedgrowth + offset` 在 debug 构建必然溢出 panic
  // （cpp release 下是 UB 回绕，被下方 clamp 静默掩盖成 totalbytes）。改 saturating：
  // 差值可表示的定义域内与 cpp 逐位等价；溢出角落与 cpp 的实际运行结果一致（回绕后
  // 仍落 `heaptrigger < totalbytes` 分支收口到 totalbytes），即上游
  // LuauGcHeapShrinkFix 针对的场景。saturating 算术编译为单指令，本函数仅 GC 周期
  // 收尾调用一次，不在热路径。
  let heaptrigger = (heapgoal as i64).saturating_sub(expectedgrowth.saturating_add(offset));

  let totalbytes = g.totalbytes as i64;

  if heaptrigger < totalbytes {
    g.totalbytes
  } else if heaptrigger > heapgoal as i64 {
    heapgoal
  } else {
    heaptrigger as usize
  }
}
