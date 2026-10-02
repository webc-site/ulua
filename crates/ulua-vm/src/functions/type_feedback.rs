//! J1 Phase 1a：运行时类型观测（`ULUA_TYPE_FEEDBACK=1` 启用）。
//!
//! 派发环头单点上报：每条指令记录 `(proto, pc, op)` 维度的 A/B 槽 tag 分布，
//! 聚入进程级哈希表。用途是 JIT 类型特化（readme/j1-bbv-design.md Phase 1a）
//! 的站点选点读数——多态站点与类型占比。
//!
//! 成本模型：未启用时 `enabled()` 为一次可预测分支（基准零影响）；启用态走
//! `record_slow`（哈希 + Mutex），只用于诊断运行，不进基准口径。

use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::records::proto::Proto;
use crate::type_aliases::instruction::Instruction;
use crate::type_aliases::t_value::TValue;

static ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
struct Site {
  count: u64,
  /// A 槽 tag 直方（下标 = LuaType as u8）
  ta: [u32; 256],
  /// B 槽 tag 直方（双寄存器操作数指令才有意义；单操作数指令记 0xFF）
  tb: [u32; 256],
}

impl Site {
  const ZERO: Site = Site { count: 0, ta: [0; 256], tb: [0; 256] };
}

fn table() -> &'static Mutex<HashMap<u64, Site>> {
  static TABLE: OnceLock<Mutex<HashMap<u64, Site>>> = OnceLock::new();
  TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 从环境激活观测（`ULUA_TYPE_FEEDBACK=1`）。须在任何 record 前调用一次。
pub fn init_from_env() {
  if env::var("ULUA_TYPE_FEEDBACK").is_ok() {
    ENABLED.store(true, Ordering::Relaxed);
  }
}

#[inline(always)]
pub fn enabled() -> bool {
  ENABLED.load(Ordering::Relaxed)
}

fn mix(mut h: usize) -> usize {
  h ^= h >> 33;
  h = h.wrapping_mul(0xff51afd7ed558ccd);
  h ^= h >> 33;
  h.wrapping_mul(0xc4ceb9fe1a85ec53) ^ (h >> 33)
}

/// 环头单点上报：`proto`/`pc` 定位站点，`ta`/`tb` 为 A/B 槽 tag（LuaType as u8）。
/// 无返回值、无 panic 面：表锁毒化只影响读数不影响执行。
#[inline(always)]
pub fn record(
  proto: *const Proto,
  pc: *const Instruction,
  op: u8,
  ta: u8,
  tb: u8,
) {
  if !ENABLED.load(Ordering::Relaxed) {
    return;
  }
  record_slow(proto as usize, pc as usize, op, ta, tb);
}

#[cold]
fn record_slow(proto: usize, pc: usize, op: u8, ta: u8, tb: u8) {
  let key = mix(proto ^ mix(pc)) as u64;
  let mut t = table().lock().unwrap_or_else(|e| e.into_inner());
  let site = t.entry(key).or_insert_with(|| Site::ZERO);
  site.count += 1;
  site.ta[ta as usize] += 1;
  site.tb[tb as usize] += 1;
  let _ = op; // op 可由读数侧按 pc 反查；此处仅计数
}

/// 读数：按 count 降序的站点表（`op` 由调用侧另配）。
pub fn dump() -> String {
  let t = table().lock().unwrap_or_else(|e| e.into_inner());
  let mut sites: Vec<(_, &Site)> = t.iter().collect();
  sites.sort_unstable_by_key(|(_, s)| Reverse(s.count));
  let mut out = String::new();
  for (key, s) in sites.iter().take(64) {
    let top = |arr: &[u32; 256]| -> String {
      let mut idx: Vec<(u32, usize)> = Vec::new();
      for (i, c) in arr.iter().enumerate() {
        if *c > 0 {
          idx.push((*c, i));
        }
      }
      idx.sort_unstable_by_key(|x| Reverse(x.0));
      idx.iter()
        .take(3)
        .map(|(c, i)| format!("{i}:{c}"))
        .collect::<Vec<_>>()
        .join(",")
    };
    out.push_str(&format!(
      "site {key:#x} count={:8} ta[{}] tb[{}]\n",
      s.count,
      top(&s.ta),
      top(&s.tb)
    ));
  }
  out
}

/// 便捷包装：从 TValue 直取 tag。
#[inline(always)]
pub fn record_tvs(
  proto: *const Proto,
  pc: *const Instruction,
  op: u8,
  ta_tv: &TValue,
  tb_tv: &TValue,
) {
  record(proto, pc, op, ta_tv.tt as u8, tb_tv.tt as u8);
}
