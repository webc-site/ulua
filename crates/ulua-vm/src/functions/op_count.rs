//! 动态 opcode 直方图与 opcode 转移计数：`vm-opcount` 测量特性专用的冷路径工具。
//!
//! 存在的理由是 Stage 2 的热点集合不能靠猜：`luau_execute.rs` 的 `HOT_ARMS` 取自本
//! 特性在 7 个回归用例上跑出的**动态**执行次数（累计占比 ≥95% 的 opcode），而转移
//! 计数进一步给出「哪些 opcode 彼此相邻」——相邻性决定热/冷分层切换的代价，光看频次
//! 选不出边界。
//!
//! 计数全部走 relaxed 原子：测量在单线程 VM 上进行，原子只为满足 `static` 的可变性
//! 要求，不提供跨线程序（多线程并发计数只会让总数近似，这正是测量想要的粒度）。
//! 整个模块由 `#[cfg(feature = "vm-opcount")]` 挂在 [`crate::functions`] 下，默认
//! 构建不产生任何指令。

use alloc::{format, string::String, vec::Vec};
use core::cmp::Reverse;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};

use ulua_common::enums::luau_opcode::LuauOpcode;

/// 256 之外的哨兵值：本层激活内「无前驱」，用于转移表的第一列。
const NO_PREV: u16 = 256;

/// 每 opcode 的执行次数，下标即 opcode 字节。
static OP_COUNT: [AtomicU64; 256] = [const { AtomicU64::new(0) }; 256];

/// `(前驱, 后继)` 转移次数；行优先，`prev == NO_PREV` 行是整次 VM 进入的第一条指令。
static TRANS: [AtomicU64; 257 * 256] = [const { AtomicU64::new(0) }; 257 * 256];

/// 全局前驱：转移统计要覆盖的是**动态指令流**，而 `vm-opcount` 下每条指令都经
/// [`crate::functions::luau_execute::tier_cold`] 的一次独立激活（热层出口被本特性
/// 编译掉，`vm_next!` 逐条 `become` 回冷层），按激活局部变量持有前驱会把所有边都记成
/// `<entry>`，等于没有转移数据。测量路径多一次 relaxed swap 可接受。
static PREV: AtomicU16 = AtomicU16::new(NO_PREV);

/// 记录一条派发。
#[inline(always)]
pub fn record(op: u8) {
  OP_COUNT[usize::from(op)].fetch_add(1, Ordering::Relaxed);
  let prev = PREV.swap(u16::from(op), Ordering::Relaxed);
  TRANS[usize::from(prev) * 256 + usize::from(op)].fetch_add(1, Ordering::Relaxed);
}

/// 输出直方图（按次数降序，附单次与累计占比）与转移表里次数最多的一批边。
pub fn dump() -> String {
  let mut counts: Vec<(u64, usize)> = (0..256).map(|op| (op_count(op), op)).collect();
  counts.sort_unstable_by_key(|x| Reverse(x.0));
  let total: u64 = counts.iter().map(|(n, _)| *n).sum();

  let mut out = String::new();
  let mut cum = 0_u64;
  for (n, op) in counts.iter().filter(|(n, _)| *n != 0) {
    cum += n;
    out.push_str(&format!(
      "{:>12} {:>7.3}% {:>7.3}%  {:?}\n",
      n,
      100.0 * *n as f64 / total as f64,
      100.0 * cum as f64 / total as f64,
      LuauOpcode::from(*op as u8),
    ));
  }
  out.push_str(&format!("total {total}\n"));

  let mut edges: Vec<(u64, u16, usize)> = Vec::new();
  for prev in 0..=256u16 {
    for op in 0..256 {
      let n = TRANS[usize::from(prev) * 256 + op].load(Ordering::Relaxed);
      if n != 0 {
        edges.push((n, prev, op));
      }
    }
  }
  edges.sort_unstable_by_key(|x| Reverse(x.0));
  for (n, prev, op) in edges.iter().take(40) {
    let from = if *prev == NO_PREV {
      String::from("<entry>")
    } else {
      format!("{:?}", LuauOpcode::from(*prev as u8))
    };
    out.push_str(&format!(
      "{:>12}  {from} -> {:?}\n",
      n,
      LuauOpcode::from(*op as u8)
    ));
  }
  out
}

/// 单个 opcode 的次数（测试与增量查询用）。
pub fn op_count(op: usize) -> u64 {
  OP_COUNT[op].load(Ordering::Relaxed)
}
