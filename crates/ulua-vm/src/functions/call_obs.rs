//! JIT call inlining 第 2 阶段：CALL 站点 callee 观测（COBS 侧表）。
//!
//! 阶段 1 的直通内联只能辨识「NEWCLOSURE/DUPCLOSURE 静态定义链」形态的 callee
//! 槽，五靶（spectralnorm/inherit3/oop/fib 等）的 callee 槽实为 GETUPVAL/
//! GETTABLEKS/NAMECALL 运行时值，编译期不可辨。本模块在 execdata 里为每条
//! CALL/CALLFB 指令建观测槽（COBS 侧表），由 native CALL 必经的 call_prolog
//! 记录实际被调闭包的 proto（funid + 指针）；proto 恒定满阈值后交上层触发该
//! caller proto 的暖重编译，发射端以内联体前置 `JumpCmpProtoid`（funid 立即数
//! 比较的 proto 守卫）承接——观测的是运行时闭包，proto 守卫兜住一切换闭包形态。
//!
//! COBS 表布局（execdata extra 区尾段，u32 单位；产出在 codegen 侧
//! `build_call_obs_table`，紧随 TSFB 表及其自描述尾字之后）：
//! ```text
//! [COBS_MAGIC, ncalls, (pc, state, funid, proto_lo, proto_hi)×ncalls][表长]
//! ```
//! `state = hits<<8 | flags`：bit0 = poly（观测到多 proto，弃内联）、
//! bit1 = sealed（已触发过暖重编译或已判多态，观测短路）。
//!
//! 取舍（相对「进程级哈希表」）：观测数据挂 execdata 随 proto 生命周期走，
//! 读侧无锁无哈希；代价是定位需前向扫描——但观测只发生在内联生效前的热身期
//! （站点被内联后 native CALL 不再经过 call_prolog），扫描开销自消，故不引入
//! 进程级表与额外失效协议。

use core::slice::from_raw_parts;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::luau_insn_ops::luau_insn_op,
};

use crate::{
  records::{closure::Closure, proto::Proto},
  type_aliases::instruction::Instruction,
};

/// COBS 魔数（'COBS'）。
const COBS_MAGIC: u32 = 0x434F_4253;
/// state 的 flags 位。
const K_FLAG_POLY: u32 = 1;
const K_FLAG_SEALED: u32 = 2;
/// proto 恒定计数达到该值即触发暖重编译。取值下界由最短热身负载约束：
/// spectralnorm 单轮 Av→eval_a 调用约 700 次，须在单轮内触发。
pub const K_TRIGGER_HITS: u32 = 200;
/// state hits 字段饱和上限（触发即 sealed，常态到不了）。
const K_HITS_CAP: u32 = 0xff_ffff;

/// 每 CALL 站点槽宽（pc, state, funid, proto_lo, proto_hi）。
const K_SLOT_WORDS: usize = 5;

/// 在 execdata extra 区定位 COBS 表。前向扫描：遇 TSFB 表按其自描述跳过，
/// 命中 COBS_MAGIC 且槽宽自洽即认定。返回（表头字下标，ncalls）。
///
/// # Safety
/// `data[sc .. sc+limit]` 界内可读（extra 区整体都在 execdata 分配内）。
unsafe fn locate_cobs(data: *const u32, sc: usize) -> Option<(usize, usize)> {
  unsafe {
    let mut i = 0usize;
    while i < sc {
      let w = *data.add(sc + i);
      if w == COBS_MAGIC {
        let ncalls = *data.add(sc + i + 1) as usize;
        // 自洽：槽宽 × ncalls + 头 + 尾字不超过扫描窗
        if ncalls > 0 && i + 2 + ncalls * K_SLOT_WORDS + 1 <= sc + 4096 {
          return Some((sc + i, ncalls));
        }
      } else if w == super::type_feedback::TSFB_MAGIC {
        // 跳过 TSFB 表（头 2 字 + 2 字/站点 + 尾字）
        let nslots = *data.add(sc + i + 1) as usize;
        if nslots > 0 && nslots <= 4096 {
          i += 2 + 2 * nslots + 1;
          continue;
        }
      }
      i += 1;
    }
    None
  }
}

/// 指针 → (lo, hi) 双字（32 位目标 hi 恒 0）。
#[inline]
fn ptr_words(p: usize) -> (u32, u32) {
  #[cfg(target_pointer_width = "64")]
  {
    (p as u32, (p >> 32) as u32)
  }
  #[cfg(target_pointer_width = "32")]
  {
    (p as u32, 0)
  }
}

/// (lo, hi) 双字 → 指针。
#[inline]
fn words_ptr(lo: u32, hi: u32) -> usize {
  #[cfg(target_pointer_width = "64")]
  {
    (lo as usize) | ((hi as usize) << 32)
  }
  #[cfg(target_pointer_width = "32")]
  {
    let _ = hi;
    lo as usize
  }
}

/// 由 savedpc 反推 CALL 指令 pc：CALL 的 savedpc 常量为 pc+1、CALLFB（带 aux 槽）
/// 为 pc+2，两候选各验证 opcode 后取匹配者。
///
/// # Safety
/// `savedpc` 须落在 `proto->code` 数组内或其后 1 槽（CALL 发射前 SetSavedpc 的契约）。
pub unsafe fn call_pc_of(proto: *const Proto, savedpc: *const Instruction) -> Option<u32> {
  unsafe {
    let sc = (*proto).sizecode as usize;
    let code = (*proto).code;
    let off = savedpc.offset_from(code);
    if off <= 0 || off as usize > sc {
      return None;
    }
    for cand in (off - 2).max(0)..off {
      let insn = *code.add(cand as usize);
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);
      if matches!(op, LuauOpcode::LopCall | LuauOpcode::LopCallfb) {
        return Some(cand as u32);
      }
    }
    None
  }
}

/// call_prolog/native 快路观测入口：记录 `(caller proto, call pc)` 站点的实际
/// callee 身份，proto 恒定满 [`K_TRIGGER_HITS`] 时返回 true（调用方据此触发该
/// caller 的暖重编译）。execdata 缺失/无 COBS 表/pc 不匹配一律静默返回 false。
///
/// # Safety
/// `caller`/`ccl` 须为存活 Proto/Closure 且 caller 的 `execdata`（若非空）为本模块
/// 布局的堆分配数据区；`call_pc` 须为 caller 字节码界内的 CALL/CALLFB 指令下标。
pub unsafe fn call_obs_record_at(caller: *mut Proto, call_pc: u32, ccl: *mut Closure) -> bool {
  unsafe {
    if (*ccl).is_c != 0 {
      return false;
    }
    let d = (*caller).execdata;
    if d.is_null() {
      return false;
    }
    let sc = (*caller).sizecode as usize;
    let data = d as *mut u32;
    let (cobs0, ncalls) = match locate_cobs(data, sc) {
      Some(x) => x,
      None => return false,
    };
    // pc 升序二分
    let base = data.add(cobs0 + 2);
    let (mut lo, mut hi) = (0usize, ncalls);
    while lo < hi {
      let mid = (lo + hi) / 2;
      if *base.add(K_SLOT_WORDS * mid) < call_pc {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    if lo == ncalls || *base.add(K_SLOT_WORDS * lo) != call_pc {
      return false;
    }

    let callee_p = (*ccl).inner.l.p;
    let funid = (*callee_p).funid;
    let (plo, phi) = ptr_words(callee_p as usize);

    let state_ptr = base.add(K_SLOT_WORDS * lo + 1);
    let st = *state_ptr;
    let hits = st >> 8;
    let flags = st & 0xff;

    if flags & K_FLAG_SEALED != 0 {
      return false;
    }
    if flags & K_FLAG_POLY != 0 {
      return false;
    }

    let funid_ptr = base.add(K_SLOT_WORDS * lo + 2);
    let proto_ptr = base.add(K_SLOT_WORDS * lo + 3);
    if hits == 0 {
      // 首观测：落槽
      *funid_ptr = funid;
      *proto_ptr = plo;
      *proto_ptr.add(1) = phi;
      *state_ptr = (1 << 8) | flags;
      return false;
    }
    if *funid_ptr != funid {
      // proto 漂移：poly 定案，sealed 短路后续观测
      *state_ptr = (hits << 8) | K_FLAG_POLY | K_FLAG_SEALED;
      return false;
    }
    let new_hits = (hits + 1).min(K_HITS_CAP);
    if new_hits < K_TRIGGER_HITS {
      *state_ptr = (new_hits << 8) | flags;
      return false;
    }
    // 满阈值：sealed 防重触发，交上层同步暖重编译
    *state_ptr = (new_hits << 8) | flags | K_FLAG_SEALED;
    true
  }
}

/// 编译期读数（暖重编译路径）：返回该 proto 各 CALL 站点中观测恒定的
/// `(pc, funid, callee proto 裸址)` 列表。发射端以 funid 作守卫立即数、以
/// proto 裸址走 callee 字节码判据（同步触发窗口内被 ra 槽闭包锚定）。
///
/// # Safety
/// `proto` 须为存活 Proto 且其 `execdata`（若非空）为本模块布局的堆分配数据区。
pub unsafe fn call_obs_hints_for(proto: *const Proto) -> Vec<(u32, u32, usize)> {
  unsafe {
    let mut out = Vec::new();
    let d = (*proto).execdata;
    if d.is_null() {
      return out;
    }
    let sc = (*proto).sizecode as usize;
    let data = d as *const u32;
    let (cobs0, ncalls) = match locate_cobs(data, sc) {
      Some(x) => x,
      None => return out,
    };
    let base = data.add(cobs0 + 2);
    for s in 0..ncalls {
      let slot = base.add(K_SLOT_WORDS * s);
      let (pc, st) = (*slot, *slot.add(1));
      // sealed 位只表示「已触发过暖重编译」（防重复触发），满阈值的 sealed 槽
      // 恰是本轮编译要消费的有效证据；仅 poly（多态定案）才排除。
      if st >> 8 < K_TRIGGER_HITS || st & K_FLAG_POLY != 0 {
        continue;
      }
      let funid = *slot.add(2);
      let p = words_ptr(*slot.add(3), *slot.add(4));
      out.push((pc, funid, p));
    }
    out
  }
}

/// 诊断读数：各 proto 各 CALL 站点的 pc/hits/funid 概览（luau-run 读数用）。
///
/// # Safety
/// 契约同 [`call_obs_hints_for`]。
pub unsafe fn call_obs_dump(protos: &[usize]) -> String {
  let mut out = String::new();
  for (i, &p) in protos.iter().enumerate() {
    let proto = p as *const Proto;
    unsafe {
      let d = (*proto).execdata;
      if d.is_null() {
        continue;
      }
      let sc = (*proto).sizecode as usize;
      let data = d as *const u32;
      let (cobs0, ncalls) = match locate_cobs(data, sc) {
        Some(x) => x,
        None => continue,
      };
      out.push_str(&format!("== proto#{i} sizecode={sc} cobs_sites={ncalls}\n"));
      let code = from_raw_parts((*proto).code, sc);
      let slots = from_raw_parts(data.add(cobs0 + 2), ncalls * K_SLOT_WORDS);
      for s in 0..ncalls {
        let w = &slots[s * K_SLOT_WORDS..s * K_SLOT_WORDS + K_SLOT_WORDS];
        let op = code
          .get(w[0] as usize)
          .copied()
          .map_or(0xff, |insn| insn as u8);
        out.push_str(&format!(
          "  site pc={:5} op={op:3} hits={:6} funid={:5} state_flags={:x}\n",
          w[0],
          w[1] >> 8,
          w[2],
          w[1] & 0xff
        ));
      }
    }
  }
  out
}
