//! J1 Phase 1a：运行时类型观测（`ULUA_TYPE_FEEDBACK=1` 启用）。
//!
//! 派发环头单点上报：每条指令记录 `(proto, pc, op)` 维度的 A/B 槽 tag 分布，
//! 聚入进程级哈希表。用途是 JIT 类型特化（readme/j1-bbv-design.md Phase 1a）
//! 的站点选点读数——多态站点与类型占比。
//!
//! 成本模型：未启用时 `enabled()` 为一次可预测分支（基准零影响）；启用态走
//! `record_slow`（哈希 + Mutex），只用于诊断运行，不进基准口径。

use std::{
  cmp::Reverse,
  collections::HashMap,
  env,
  slice::from_raw_parts,
  sync::{
    Mutex, OnceLock,
    atomic::{AtomicBool, Ordering},
  },
};

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  records::proto::Proto,
  type_aliases::{instruction::Instruction, t_value::TValue},
};

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
  const ZERO: Site = Site {
    count: 0,
    ta: [0; 256],
    tb: [0; 256],
  };
}

fn table() -> &'static Mutex<HashMap<u64, Site>> {
  static TABLE: OnceLock<Mutex<HashMap<u64, Site>>> = OnceLock::new();
  TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 从环境激活观测（`ULUA_TYPE_FEEDBACK=1`）。须在任何 record 前调用一次。
pub fn init_from_env() {
  if env::var("ULUA_TYPE_FEEDBACK").is_ok() || env::var("ULUA_TSFB").is_ok() {
    ENABLED.store(true, Ordering::Relaxed);
  }
}

#[inline(always)]
pub fn enabled() -> bool {
  ENABLED.load(Ordering::Relaxed)
}

/// 64 位 splitmix 终混（显式 u64：32 位目标上 usize 为 4 字节，
/// `>>33` 与 64 位字面量都会溢出——CI wasm/linux-32 构建实测）。
fn mix(mut h: u64) -> u64 {
  h ^= h >> 33;
  h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
  h ^= h >> 33;
  h.wrapping_mul(0xc4ce_b9fe_1a85_ec53) ^ (h >> 33)
}

/// 环头单点上报：`proto`/`pc` 定位站点，`ta`/`tb` 为 A/B 槽 tag（LuaType as u8）。
/// 无返回值、无 panic 面：表锁毒化只影响读数不影响执行。
#[inline(always)]
pub fn record(proto: *const Proto, pc: *const Instruction, op: u8, ta: u8, tb: u8) {
  if !ENABLED.load(Ordering::Relaxed) {
    return;
  }
  record_slow(proto as usize, pc as usize, op, ta, tb);
}

#[cold]
fn record_slow(proto: usize, pc: usize, op: u8, ta: u8, tb: u8) {
  let key = mix(u64::from(proto as u32)) ^ mix(u64::from(pc as u32));
  let mut t = table().lock().unwrap_or_else(|e| e.into_inner());
  let site = t.entry(key).or_insert(Site::ZERO);
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
      idx
        .iter()
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

// ---------------------------------------------------------------------------
// J1 Phase 2a：native execdata TSFB 侧表（布局由 codegen 侧
// create_native_proto_exec_data 的 build_tsfb_table 产出）：
//   execdata[0..sizecode]              指令偏移
//   execdata[sizecode]                 TSFB_MAGIC = 0x5453_4642
//   execdata[sizecode+1]               nslots
//   execdata[sizecode+2 + 2*i]         站点 i 的 pc（升序）
//   execdata[sizecode+3 + 2*i]         state = hits<<8 | last_tag
// ---------------------------------------------------------------------------

const TSFB_MAGIC: u32 = 0x5453_4642;

/// 在 extra 区（自 data[sizecode] 起）定位 TSFB 表：前向扫描 MAGIC，
/// 校验 nslots 与 pc 升序自洽。返回（表头字下标，nslots）。
///
/// # Safety
/// `data[sc .. sc+limit]` 界内可读（extra 区 + 表自身都在分配内）。
unsafe fn locate_tsfb(data: *const u32, sc: usize, pc_off: u32) -> Option<(usize, usize)> {
  unsafe {
    let mut i = 0usize;
    while i < sc {
      if *data.add(sc + i) == TSFB_MAGIC {
        let nslots = *data.add(sc + i + 1) as usize;
        let pairs = 2 * nslots;
        if nslots > 0 && nslots <= 4096 && i + 2 + pairs <= sc + 4096 {
          // 自洽：pc 升序且不超过 sizecode
          let mut ok = true;
          let mut prev = 0u32;
          for s in 0..nslots {
            let pc = *data.add(sc + i + 2 + 2 * s);
            if pc < prev || pc >= sc as u32 {
              ok = false;
              break;
            }
            prev = pc;
          }
          if ok {
            // 站点存在性可选（pc_off == u32::MAX 表示仅读数，不要求命中）
            if pc_off == u32::MAX {
              return Some((sc + i, nslots));
            }
            let (mut lo, mut hi) = (0usize, nslots);
            while lo < hi {
              let mid = (lo + hi) / 2;
              if *data.add(sc + i + 2 + 2 * mid) < pc_off {
                lo = mid + 1;
              } else {
                hi = mid;
              }
            }
            if lo < nslots && *data.add(sc + i + 2 + 2 * lo) == pc_off {
              return Some((sc + i, nslots));
            }
            return None;
          }
        }
      }
      i += 1;
    }
    None
  }
}

/// 已见过的带 TSFB 侧表的 proto（读数用；仅诊断运行填充）。
fn tsfb_protos() -> &'static Mutex<Vec<usize>> {
  static P: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
  P.get_or_init(|| Mutex::new(Vec::new()))
}

/// guard-miss 观测：把 `(pc_off, tag)` 记入 proto 的 TSFB 侧表
/// （hits 饱和递增、last_tag 覆写）。execdata 缺失/无侧表时静默返回。
///
/// # Safety
/// `proto` 须为存活 Proto 且其 `execdata`（若非空）为本模块布局的堆分配数据区。
pub unsafe fn tsfb_bump(proto: *const Proto, pc_off: u32, tag: u8) {
  // J4 spike P0：未启用观测时零成本返回——此前每次 fallback 都付 locate 扫描 +
  // Mutex + 线性查重，正在污染 oop 类基准（恰好惩罚 fallback 多的负载）。
  if !ENABLED.load(Ordering::Relaxed) {
    return;
  }
  // SAFETY: 契约保证 proto 存活；execdata/sizecode 为同址字段读，state 写落在
  // 分配的 extra 区界内（locate_tsfb 已校验表自洽）。
  unsafe {
    let d = (*proto).execdata;
    if d.is_null() {
      return;
    }
    let sc = (*proto).sizecode as usize;
    let data = d as *const u32;
    // TSFB 表位置自描述扫描（不依赖 header 偏移）：extra 区自 data[sizecode] 起，
    // 命中 MAGIC 且 (nslots, pc 升序≤sizecode) 自洽即认定。诊断路径，有界扫描可接受。
    let (tsfb0, nslots) = match locate_tsfb(data, sc, pc_off) {
      Some((t, n)) => (t, n),
      None => return,
    };
    let base = data.add(tsfb0 + 2);
    // pc 升序二分
    let (mut lo, mut hi) = (0usize, nslots);
    while lo < hi {
      let mid = (lo + hi) / 2;
      if *base.add(2 * mid) < pc_off {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    if lo == nslots || *base.add(2 * lo) != pc_off {
      return;
    }
    let state_ptr = base.add(2 * lo + 1) as *mut u32;
    let st = *state_ptr;
    let hits = (st >> 8).min(0xff_ffff);
    *state_ptr = ((hits + 1) << 8) | tag as u32;

    {
      let mut ps = tsfb_protos().lock().unwrap_or_else(|e| e.into_inner());
      if !ps.contains(&(proto as usize)) {
        ps.push(proto as usize);
      }
    }
  }
}

/// TSFB 读数：各 proto 各站点的 pc/hits/last-tag 概览。
pub fn tsfb_dump() -> String {
  let protos = tsfb_protos()
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .clone();
  eprintln!("[tsfb-dump] registered protos: {}", protos.len());
  let mut out = String::new();
  for (i, p) in protos.iter().enumerate() {
    let proto = *p as *const Proto;
    unsafe {
      let d = (*proto).execdata;
      if d.is_null() {
        continue;
      }
      let sc = (*proto).sizecode as usize;
      let data = d as *const u32;
      let (_, nslots) = match locate_tsfb(data, sc, u32::MAX) {
        Some(x) => x,
        None => continue,
      };
      out.push_str(&format!("== proto#{i} sizecode={sc} tsfb_sites={nslots}\n"));
      // 最小 unsafe 边界内一次性取指令区与 TSFB 站点区切片，后续全部为安全
      // 切片访问（调试读数路径，越界检查开销无碍）；op 语义与原裸指针读逐点
      // 一致：pc 界内取指令字低字节，越界回 0xff。
      let code = from_raw_parts((*proto).code, sc);
      let sites = from_raw_parts(data.add(sc + 2), 2 * nslots);
      for pair in sites.chunks_exact(2) {
        let (pc, st) = (pair[0], pair[1]);
        let op = code
          .get(pc as usize)
          .copied()
          .map_or(0xff, |insn| insn as u8);
        out.push_str(&format!(
          "  site pc={pc:5} op={op:3} hits={:6} last_tag={}\n",
          st >> 8,
          st & 0xff
        ));
      }
    }
  }
  out
}

/// J1 Phase 2b 选点入口：遍历已注册 TSFB 表，返回 hits ≥ `min_hits` 且
/// 末 tag 占比 ≥ `min_share` 的站点 `(proto, pc, tag)`——暖重编译/特化的候选集。
pub fn tsfb_over_threshold(min_hits: u32, min_share: f64) -> Vec<(usize, u32, u8)> {
  let protos = tsfb_protos()
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .clone();
  let mut out = Vec::new();
  for p in protos.iter() {
    let proto = *p as *const Proto;
    unsafe {
      let d = (*proto).execdata;
      if d.is_null() {
        continue;
      }
      let sc = (*proto).sizecode as usize;
      let data = d as *const u32;
      let (_, nslots) = match locate_tsfb(data, sc, u32::MAX) {
        Some(x) => x,
        None => continue,
      };
      // 同上：站点区一次取切片，安全迭代（读数语义与原逐点裸读一致）。
      let sites = from_raw_parts(data.add(sc + 2), 2 * nslots);
      for pair in sites.chunks_exact(2) {
        let (pc, st) = (pair[0], pair[1]);
        let hits = (st >> 8) as u64;
        let tag = (st & 0xff) as u8;
        if hits >= u64::from(min_hits) {
          out.push((proto as usize, pc, tag));
        }
        let _ = min_share; // last_tag 单值占比 = 1.0（state 只记末 tag）；接口留扩展
      }
    }
  }
  out
}

/// J1 Phase 2b：从 proto 当前 execdata 的 TSFB 侧表产出类型提示
/// （GETTABLEKS 站点：pc → (B 寄存器 = 接收者, 观测 tag)）。
/// 暖重编译路径消费：hint 注入分析器细化 ANY → 观测 tag。
///
/// # Safety
/// `proto` 须为存活 Proto 且其 `execdata`（若非空）为本模块布局的堆分配数据区；
/// `code` 界内可读。
pub unsafe fn tsfb_hints_for(proto: *const Proto) -> Vec<(u32, u8, u8)> {
  // SAFETY: 契约保证 proto 存活、execdata/code 为同址字段读；locate_tsfb 界内扫描。
  unsafe {
    let mut out = Vec::new();
    let d = (*proto).execdata;
    if d.is_null() {
      return out;
    }
    let sc = (*proto).sizecode as usize;
    let data = d as *const u32;
    let (_, nslots) = match locate_tsfb(data, sc, u32::MAX) {
      Some(x) => x,
      None => return out,
    };
    let code = (*proto).code;
    for s in 0..nslots {
      let pc = *data.add(sc + 2 + 2 * s);
      let st = *data.add(sc + 3 + 2 * s);
      let tag = (st & 0xff) as u8;
      if pc as usize >= sc {
        continue;
      }
      let insn = *code.add(pc as usize);
      let op = (insn & 0xff) as u8;
      // GETTABLEKS：B 即接收者寄存器
      if op == LuauOpcode::LopGettableks as u8 {
        let reg_b = ((insn >> 8) & 0xff) as u8;
        out.push((pc, reg_b, tag));
      }
    }
    out
  }
}
