//! FORN trace 层运行期注册表（阶段二 PoC）：热环检测 + 录制/生成/安装 +
//! 入口裁决 + 出口快照回落，ecb 槽 `trace_forn_enter` 的安装方。
//!
//! 全链路（阶段二蓝图 PoC 级端到端）：
//! 1. **热环检测**——解释器 `h_fornprep`（旗标开）逐环入口问询本表；
//!    `(proto, FORNPREP pc)` 键计数达 `K_HEAT_THRESHOLD` 即触发录制。
//!    计数挂环入口而非回边：每入口一次哈希查找（非逐迭代），旗标关零成本。
//! 2. **录制**——`trace_forn_ir::record_forn_trace` 静态闭体走查（线性 IR +
//!    回边 phi + 类型特化分类），资格不符 `dead` 收口（防逐入口重试）。
//! 3. **生成/安装**——`trace_forn_codegen_a_64::emit_trace_a_64` 产出寄存器
//!    驻留原生码，`CodeAllocator` 独立分配（页生命周期由 [`InstalledTrace`]
//!    持有，drop = deallocate + destroy 配对）。
//! 4. **入口守卫**（Rust 侧，拒绝路径零状态扰动）：中断钩子缺席、step NaN、
//!    step flavor（装机锁定：快 flavor 仅 step==1.0 / 泛 flavor 任意非 NaN）、
//!    idx/limit 整性（快 flavor i32 域全检；泛 flavor 仅 PhiIdx 寻址形态校验
//!    idx 起点）、table tag/元表缺席/readonly、不变 number 槽与累加器 tag。
//!    元表/readonly 提至入口的合法性论证见
//!    trace_forn_codegen_a_64 模块注（直线体无调用/无分配，单线程不可变）。
//! 5. **执行**——自包含 C ABI 叶函数 `fn(l, proto) -> u32`：0 = 环走完
//!    （savedpc 已落环出口）、1 = 环内守卫 bail（savedpc 已落失败位点，
//!    解释器整条重做，无部分副作用）。
//! 6. **出口回落**——零跳（idx > limit，含 NaN）由本层 Rust 侧直接落
//!    savedpc = 环出口，不进原生码；出口/快照语义逐位对齐解释器
//!    （idx 写回值+tnumber tag、写回面终值、savedpc 续延位）。
//!
//! 生命周期：注册表挂在 `BaseCodeGenContext.forn_traces`（state 关闭时
//! `on_close_state` 的 Box 回收整体 drop，页随之 deallocate + destroy）。
//! 已装 trace 逐入口校验身份（指令字 + K 常量位型）：proto 回收后地址复用
//! 的新 proto 只在全部校验集逐位一致时命中——校验集即 trace 的全部语义
//! 消费面，逐位一致即语义一致，无 ABA 窗口；不一致即弃置重录（vm_patch_c
//! 类指令字回填也会走此路，收敛正确）。
//!
//! a64 专属：录制/生成/执行全链仅 aarch64 安装（`initialize_execution_
//! callbacks` 的 cfg 门），其余平台 ecb 槽恒 None，解释器零问询。

use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use core::{
  cmp::Ordering,
  fmt::{self, Debug, Formatter},
  mem::{size_of_val, transmute},
  ptr::{addr_of, null_mut},
  slice::from_raw_parts,
  sync::atomic::{AtomicU64, Ordering as AtomicOrdering},
};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag::{LuauJitFornTrace, LuauTraceFmaFold},
  macros::luau_insn_ops::{luau_insn_a, luau_insn_d, luau_insn_op},
};
use ulua_vm::{
  enums::lua_type::LuaType,
  records::{
    closure::Closure, global_state::global_State, lua_execution_callbacks::FORN_HEAT_ARMED,
    lua_state::LuaState, lua_t_value::TValue, lua_table::LuaTable, proto::Proto, up_val::UpVal,
  },
  type_aliases::{instruction::Instruction, stk_id::StkId},
};

use crate::{
  functions::{
    get_code_gen_context::get_code_gen_context,
    trace_forn_codegen_a_64::emit_trace_a_64,
    trace_forn_ir::{
      TArith, TInst, TMathUnary, TUpval, TUpvalKind, TVal, TraceIr, forn_body_eligible,
      record_forn_trace,
    },
    trace_forn_math_addr::slot_math_fn,
  },
  records::{code_allocation_data::CodeAllocationData, code_allocator::CodeAllocator},
};

/// 热环阈值（环入口次数 / 回边次数，两面任一达阈即录制；matmul j 环按回边
/// 计约 8 个 i 迭代内达标，单层长环一次进入内即可达标）
pub(crate) const K_HEAT_THRESHOLD: u64 = 1000;
/// 单 state 注册表软上限（超限新环不再录制；PoC 形态护栏）
pub(crate) const K_REGISTRY_CAP: usize = 64;
/// 回边计数位点软上限（超限不再武装；PoC 形态护栏）
pub(crate) const K_BACKEDGE_CAP: usize = 128;
/// 单 trace 分配块（页对齐；一条 trace 一块，64KB 覆盖最大环体富余）
const K_TRACE_BLOCK_SIZE: usize = 64 * 1024;

// —— 运行统计（rt 测试与诚实报告的命中判据；常驻仪表非调试探针）——
static STAT_COMPILED: AtomicU64 = AtomicU64::new(0);
static STAT_EXECUTED: AtomicU64 = AtomicU64::new(0);
static STAT_BAILED: AtomicU64 = AtomicU64::new(0);
static STAT_ZERO_TRIP: AtomicU64 = AtomicU64::new(0);

/// trace 层命中统计快照：(安装数, 原生执行数, 环内 bail 数, 零跳数)。
pub fn forn_trace_stats() -> (u64, u64, u64, u64) {
  (
    STAT_COMPILED.load(AtomicOrdering::Relaxed),
    STAT_EXECUTED.load(AtomicOrdering::Relaxed),
    STAT_BAILED.load(AtomicOrdering::Relaxed),
    STAT_ZERO_TRIP.load(AtomicOrdering::Relaxed),
  )
}

/// 已安装 trace：可执行页宿主 + 入口守卫所需的特化面快照。
struct InstalledTrace {
  allocator: CodeAllocator,
  allocation: CodeAllocationData,
  /// 自包含叶函数入口（`fn(l, proto) -> u32`，call 点 transmute）
  code_start: *mut u8,
  /// 身份校验面（FORNPREP..=FORNLOOP 全指令字 + K 常量位型）
  words: Box<[u32]>,
  k_consts: Box<[(u32, u64)]>,
  /// GETIMPORT 面：(dst 槽, k 下标) + 缓存位型（身份对账用）
  imports: Vec<(u8, u32)>,
  import_ids: Box<[(u32, u128)]>,
  /// math 内建守卫槽（入口复检槽值仍为对应 math 单参 C 函数）
  math_fn_guards: Vec<(u8, TMathUnary)>,
  /// 环体引用的 upvalue 分类表（T8：入口复检 upref 形态/值面——cell 源是
  /// 跨入口可变的真值源，槽值只是脏快照）
  upvals: Vec<TUpval>,
  sizecode: u32,
  exit_pc: u32,
  /// FORN 三元组首槽 / 环不变 table 槽 / 不变 number 槽 / 累加器槽
  ra: u8,
  tables: Vec<u8>,
  inv_nums: Vec<u8>,
  accs: Vec<u8>,
  /// step==1.0 特化 flavor（装机时按录制现场的 step 锁定）：true 拒绝
  /// step≠1 的入口（快回边假设 step 恒 1）；false 泛 flavor 对任意非 NaN
  /// step 逐位正确（fcmpz 选向 + 逐迭代精确性校验）
  step_one_only: bool,
  /// 环体含 PhiIdx 下标数组访问（寻址依赖 idx 整数表示——泛 flavor 入口
  /// 须校验起点整性；非寻址形态（纯算术体）允许分数 idx 起点）
  phi_indexed: bool,
}

impl Drop for InstalledTrace {
  fn drop(&mut self) {
    // 页配对回收：先解除本分配的可执行属性（live_allocations 归零），
    // 随后 allocator 的 Drop（destroy）在账平前提下释放映射页。
    self.allocator.deallocate(self.allocation);
  }
}

/// 单环注册项：热度计数 + 安装态。
struct FornLoopEntry {
  count: u64,
  /// 录制/生成/安装失败 → 永久放弃（防逐入口重试税）
  dead: bool,
  /// 回边武装资格预扫缓存（None = 未扫；Some(false) = 环体静态不可录，
  /// 永不武装回边计数——免逐入口重扫税）
  eligible: Option<bool>,
  installed: Option<InstalledTrace>,
}

/// per-state FORN trace 注册表（挂在 `BaseCodeGenContext.forn_traces`）。
#[derive(Default)]
pub(crate) struct FornTraceRegistry {
  loops: BTreeMap<(usize, u32), FornLoopEntry>,
  /// 回边计数位点（T2）：(proto, FORNLOOP pcpos) → 计数单元。Box 保证计数
  /// 单元地址稳定（武装进 ecb IC 的是这份地址；BTreeMap 节内元素会位移，
  /// 不得直接持内值地址）。阈值触发/资格否定后整点移除。
  backedges: BTreeMap<(usize, u32), Box<u64>>,
}

impl Debug for FornTraceRegistry {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    // 环项含原始指针与页宿主，不进 Debug 面——只报模量
    f.debug_struct("FornTraceRegistry")
      .field("loops", &self.loops.len())
      .finish()
  }
}

/// f64 → i32 精确整数判定（fcvtzs 饱和 + 往返恒等，越界/分数/NaN 拒绝）。
fn exact_i32(v: f64) -> Option<i32> {
  let w = v as i64;
  if w as f64 == v && w >= i64::from(i32::MIN) && w <= i64::from(i32::MAX) {
    Some(w as i32)
  } else {
    None
  }
}

/// 入口守卫裁决。
enum Guard {
  /// 守卫全过，进原生执行
  Pass,
  /// 零跳（idx > limit，含 NaN 有序比较为假）：savedpc = 环出口即回落
  ZeroTrip,
  /// 不承接（解释器照常走 FORNPREP）
  Refuse,
}

/// 入口守卫（Rust 侧，拒绝路径零状态扰动；逐迭代守卫在生成码内）。
///
/// # Safety
/// `l`/`proto` 为解释器派发环契约的存活指针；三元组数值性由调用点
/// （h_fornprep 数值判定通过）保证，此处仍防御性复核。
unsafe fn entry_guard(l: *mut LuaState, e: &InstalledTrace, proto: *mut Proto) -> Guard {
  // Safety: 函数契约（解释器派发环存活性），裸指针解引用统一收口本块
  unsafe {
    // 中断钩子在场 → 不承接（解释器逐迭代处理 VM_INTERRUPT/钩子）
    let global: *mut global_State = (*l).global;
    if global.is_null() || (*global).cb.interrupt.is_some() {
      return Guard::Refuse;
    }
    // GETIMPORT 安全双守卫：safeenv != 0（环境对 import 安全）+ k[D] 非
    // nil（import 缓存已解析）——任一不成立即拒绝（解释器 GETIMPORT 慢路
    // 解析并写 k[D] 缓存，后续入口自然命中）。环体无调用无赋值，safeenv
    // 与缓存跨环不变，入口一次判定即覆盖全环
    let closure = if !e.imports.is_empty() || !e.upvals.is_empty() {
      let closure = (*(*l).ci).func;
      if !(*closure).is_function() {
        return Guard::Refuse;
      }
      // Safety: is_closure() 谓词命中后 as_closure_ptr 为同址类型化读
      let cl = (*closure).as_closure_ptr();
      if !e.imports.is_empty() && (*(*cl).env).safeenv == 0 {
        return Guard::Refuse;
      }
      Some(cl)
    } else {
      None
    };
    if !e.imports.is_empty() {
      let k = (*proto).k;
      for &(_, kidx) in &e.imports {
        // Safety: kidx 源自录制期 walk 的合法 k 下标（identity 面同源）
        let kv = &*k.add(kidx as usize);
        if kv.is_nil() {
          return Guard::Refuse;
        }
      }
    }
    // upvalue 形态/值面复检（T8）：cell 源逐入口重解——真 UpVal 的 cell =
    // (*uv).v（open→栈槽 / closed→storage，双态统一指针语义，态迁移只在
    // 调用/返回/CLOSEUPVAL 发生、均不在环体内，逐入口重解即自然消费）；
    // LCT_VAL 值内联 upref 的 cell 源 = upref 本体。upref 形态由 proto 捕获
    // 描述符决定、跨入口稳定，此处复核防形态失配的生成码错位
    if let Some(cl) = closure {
      for uv in &e.upvals {
        if usize::from(uv.idx) >= (*cl).nupvalues as usize {
          return Guard::Refuse;
        }
        // Safety: uprefs 为柔性数组语义（nupvalues 过分配，lua_f_new_lclosure
        // 布局保证），idx 已防御校验；upref/cell 读取只触 tt/gc 头（外层
        // unsafe 块契约覆盖）
        let ur = addr_of!((*cl).inner.l.uprefs).cast::<TValue>();
        let ur = ur.add(usize::from(uv.idx));
        let is_uv = (*ur).is_upval();
        if is_uv != uv.cell {
          return Guard::Refuse; // 生成码 prologue 形态锁定面
        }
        // cell 源 TValue：真 UpVal 经 v 指针解引（open/closed 双态统一），
        // 值内联 upref 即本体
        let src: *const TValue = if is_uv {
          (*(*ur).value.gc.cast::<UpVal>()).v
        } else {
          ur
        };
        // Safety: cell 源为闭包 upref 体系内的存活 TValue（GC 强可达）
        let tv = &*src;
        match uv.kind {
          TUpvalKind::Cfn(kind) => {
            // fn 载体真值源是 cell（槽值是上次 GETUPVAL 的脏快照）——按
            // cell 复检 (种)，防环外重绑定后错特化
            if !tv.is_function() || slot_math_fn(src) != Some(kind) {
              return Guard::Refuse;
            }
          }
          TUpvalKind::Num if uv.mutated => {
            // 变更载体值面由生成码逐访问 tag 守卫收口（非 number 即 bail
            // 回解释器），入口只钉形态
          }
          TUpvalKind::Num => {
            // 只读内联：值装载进 prologue 保留临时，非 number 拒承（解释器
            // 承接任意类型读）
            if !tv.is_number() {
              return Guard::Refuse;
            }
          }
        }
      }
    }
    // math 内建守卫槽复检：跨入口该局部可能被重赋值，特化环按槽拒承
    for &(s, kind) in &e.math_fn_guards {
      // Safety: 录制期 body 引用槽为活跃帧槽域合法下标
      let tv = &*(*l).base.add(usize::from(s));
      if !tv.is_function() || slot_math_fn(tv) != Some(kind) {
        return Guard::Refuse;
      }
    }
    let base = (*l).base;
    // Safety: 派发环契约保证 base 指向活跃帧槽域，ra..ra+2 为合法槽
    let triple = |i: u8| -> Option<f64> {
      let tv = &*base.add(i as usize);
      if tv.is_number() {
        // Safety: tt==tnumber 保证 value 联合体 n 臂有效
        Some(tv.value.n)
      } else {
        None
      }
    };
    let Some(limit) = triple(e.ra) else {
      return Guard::Refuse;
    };
    let Some(step) = triple(e.ra + 1) else {
      return Guard::Refuse;
    };
    let Some(idx) = triple(e.ra + 2) else {
      return Guard::Refuse;
    };
    // step 特化：NaN 拒承（解释器 `step > 0.0` 为假走 GE 路、首回合即退，
    // 拒承同观测）；快 flavor 仅 step==1.0（`idx + step` 与 fadd idx,1.0
    // 逐位一致）；泛 flavor 接受任意非 NaN step
    if step.is_nan() || (e.step_one_only && step != 1.0) {
      return Guard::Refuse;
    }
    // idx/limit 整性（整数表示寻址的前提）：快 flavor 沿用 T1 全检（idx
    // 寻址 + 递增不破整）；泛 flavor 仅 PhiIdx 寻址形态校验 idx 起点整性
    //（limit/非寻址 idx 只进 fcmp，任意 f64 逐位同解释器）
    if e.step_one_only {
      if exact_i32(idx).is_none() || exact_i32(limit).is_none() {
        return Guard::Refuse;
      }
    } else if e.phi_indexed && exact_i32(idx).is_none() {
      return Guard::Refuse;
    }
    // 零跳：与 fornprep_step 的选向式逐位一致——`step > 0 ? idx <= limit
    // : limit <= idx`，f64 偏序，NaN（partial_cmp 为 None）两向皆假 → 退出
    let can_iterate = if step > 0.0 {
      matches!(
        idx.partial_cmp(&limit),
        Some(Ordering::Less | Ordering::Equal)
      )
    } else {
      matches!(
        limit.partial_cmp(&idx),
        Some(Ordering::Less | Ordering::Equal)
      )
    };
    if !can_iterate {
      return Guard::ZeroTrip;
    }
    // 环不变 table 槽：tag / 元表缺席 / readonly（提至入口，合法性见模块注）
    for &s in &e.tables {
      let tv = &*base.add(s as usize);
      if !tv.is_table() {
        return Guard::Refuse;
      }
      // Safety: tt==ttable 保证 value 联合体 gc 臂有效（栈槽契约）
      let t = tv.value.gc.cast::<LuaTable>();
      if !(*t).metatable.is_null() || (*t).readonly != 0 {
        return Guard::Refuse;
      }
    }
    // 环不变 number 槽与累加器槽：tag 特化守卫
    for &s in e.inv_nums.iter().chain(e.accs.iter()) {
      let tv = &*base.add(s as usize);
      if !tv.is_number() {
        return Guard::Refuse;
      }
    }
    Guard::Pass
  }
}

/// 直读三元组 step 槽（flavor 锁定用；tnumber 由 h_fornprep 数值判定前置
/// 保证，录制约询仅在其后可达）。
///
/// # Safety
/// `l` 为派发环契约存活指针，`ra+1` 为活跃帧合法槽且 tag 为 tnumber。
unsafe fn read_step(l: *mut LuaState, ra: u8) -> f64 {
  // Safety: 契约见函数注
  unsafe { (*(*l).base.add(ra as usize + 1)).value.n }
}

/// ecb 槽导出：1 = 已承接（savedpc 已落续延位），0 = 不承接。
///
/// # Safety
/// `l`/`proto` 为解释器派发环契约的存活指针（h_fornprep 数值判定通过后
/// 调用），单线程串行。
pub unsafe extern "C-unwind" fn forn_trace_enter_export(
  l: *mut LuaState,
  proto: *mut Proto,
  pcpos: u32,
) -> i32 {
  // Safety: 契约透传至 forn_trace_enter
  i32::from(unsafe { forn_trace_enter(l, proto, pcpos) })
}

/// 入口裁决主体（热环检测 + 命中执行 + 录制安装）。
///
/// # Safety
/// 同 [`forn_trace_enter_export`]。
unsafe fn forn_trace_enter(l: *mut LuaState, proto: *mut Proto, pcpos: u32) -> bool {
  let ctx = unsafe { get_code_gen_context(l) };
  let Some(ctx) = ctx else {
    return false;
  };
  // Safety: proto 为派发环契约的存活对象，code/k 缓冲在其存活期合法
  let (sizecode, code, k) = unsafe {
    if (*proto).sizecode < 0 {
      return false;
    }
    let sizecode = (*proto).sizecode as u32;
    let code: &[Instruction] = from_raw_parts((*proto).code, sizecode as usize);
    let k: &[TValue] = if (*proto).sizek > 0 {
      from_raw_parts((*proto).k, (*proto).sizek as usize)
    } else {
      &[]
    };
    (sizecode, code, k)
  };
  if pcpos as usize >= code.len() {
    return false;
  }
  // 环出口 = FORNPREP 不续延路（fornprep_step：pc+1+d）；FORNLOOP 恰在其前
  // （record_forn_trace 复核闭环形态）
  let exit_pc = i64::from(pcpos) + 1 + i64::from(luau_insn_d(code[pcpos as usize]));
  if exit_pc < 0 || exit_pc > i64::from(sizecode) {
    return false;
  }
  let fornloop_pc = (exit_pc - 1) as u32;

  // —— 安装态：身份校验 → 入口守卫 → 零跳/原生执行；未承接落入热环计数 ——
  enum Outcome {
    Handled,
    NotHandled,
  }
  let mut outcome = Outcome::NotHandled;
  {
    // 拆借：loops（安装态/计数）与 backedges（回边计数单元）两域独立可变，
    // 武装 helper 因此能同时写entry 与计数表
    let FornTraceRegistry { loops, backedges } = &mut ctx.forn_traces;
    let key = (proto as usize, pcpos);
    if let Some(entry) = loops.get_mut(&key) {
      // 阶段 1（安装态判定，不变借用收口后再写 entry）：
      // None = 无安装；Some(None) = 身份失配弃置；Some(Some) = 有效安装
      let installed_state = match entry.installed.as_ref() {
        None => None,
        Some(inst) => {
          let live_words = &code[pcpos as usize..=fornloop_pc as usize];
          let identity_ok = inst.sizecode == sizecode
            && inst.words.len() == live_words.len()
            && inst.words.iter().zip(live_words).all(|(a, b)| a == b)
            && inst
              .k_consts
              .iter()
              .all(|&(ki, bits)| match k.get(ki as usize) {
                // tt==tnumber 于录制时验证，此处仅位型对账
                // Safety: n 臂有效性同上（录制时已验证）
                Some(kv) if kv.tt == LuaType::Number as i32 => {
                  unsafe { kv.value.n }.to_bits() == bits
                }
                _ => false,
              })
            && inst
              .import_ids
              .iter()
              .all(|&(ki, bits)| match k.get(ki as usize) {
                // (tt, value 8B) 合并位型对账：import 缓存被改写即失配重录
                // Safety: k 下标录制期合法（同上）
                Some(kv) => {
                  (((kv.tt as u128) << 64) | ((unsafe { kv.value.n } as u64) as u128)) == bits
                }
                None => false,
              });
          Some(if identity_ok { Some(inst) } else { None })
        }
      };
      match installed_state {
        Some(None) => {
          // 字节码面已变（地址复用 / 指令字回填）→ 弃置重录
          entry.installed = None;
          entry.count = 0;
        }
        Some(Some(inst)) => match unsafe { entry_guard(l, inst, proto) } {
          Guard::Refuse => outcome = Outcome::NotHandled,
          Guard::ZeroTrip => {
            // savedpc = 环出口（解释器从出口续延；idx 槽不动 = 零跳语义）
            // Safety: l/proto 派发环契约存活，ci 指向活跃调用帧
            unsafe {
              (*(*l).ci).savedpc = (*proto).code.add(inst.exit_pc as usize);
            }
            STAT_ZERO_TRIP.fetch_add(1, AtomicOrdering::Relaxed);
            outcome = Outcome::Handled;
          }
          Guard::Pass => {
            // Safety: code_start 指向 CodeAllocator 已切换为可执行页的
            // 生成码，形态即本模块发射的 `fn(l, proto) -> u32` 叶函数；
            // l/proto 为派发环契约存活指针，函数体不触碰 ctx。
            STAT_EXECUTED.fetch_add(1, AtomicOrdering::Relaxed);
            // Safety: 生成码 C ABI 自包含叶函数（无 unwind 跨界面）
            let ret = unsafe { call_trace(inst.code_start, l, proto) };
            if ret != 0 {
              STAT_BAILED.fetch_add(1, AtomicOrdering::Relaxed);
            }
            outcome = Outcome::Handled;
          }
        },
        None => {}
      }
      if !matches!(outcome, Outcome::Handled) {
        // —— 阶段 2：热环计数 + 阈值录制安装（未承接的入口照常解释执行）——
        entry.count += 1;
        if !entry.dead && entry.count >= K_HEAT_THRESHOLD {
          entry.dead = true; // 一次性尝试：失败不重试（防逐入口重试税）
          let ra = luau_insn_a(code[pcpos as usize]) as u8;
          // Safety: h_fornprep 数值判定通过后问询，ra+1 为合法 tnumber 槽
          let step = unsafe { read_step(l, ra) };
          if let Some(inst) = unsafe {
            record_and_install(
              code,
              k,
              pcpos,
              fornloop_pc,
              exit_pc as u32,
              step,
              (*l).base,
              // Safety: ci->func 为当前帧活跃闭包（派发环契约）
              (*(*(*l).ci).func).as_closure_ptr(),
            )
          } {
            entry.installed = Some(inst);
            STAT_COMPILED.fetch_add(1, AtomicOrdering::Relaxed);
          }
        }
        // —— T2 回边计数武装（仍未承接 = 无安装态）——
        arm_backedge_counter(l, backedges, entry, proto, pcpos, fornloop_pc, code);
      }
    } else if loops.len() < K_REGISTRY_CAP {
      loops.insert(
        key,
        FornLoopEntry {
          count: 1,
          dead: false,
          eligible: None,
          installed: None,
        },
      );
      // 首入口即武装：单/双入口环（T2-2 主形态）的第一次进入必须被回边
      // 计数覆盖，等到下次问询再武装会整段漏计
      // Safety: 刚插入的条目按 key 取回必然命中
      if let Some(entry) = loops.get_mut(&key) {
        arm_backedge_counter(l, backedges, entry, proto, pcpos, fornloop_pc, code);
      }
    }
  }
  matches!(outcome, Outcome::Handled)
}

/// 回边计数武装（T2）：资格面成立且位点未安装/未放弃时，把计数单元地址
/// 武装进 ecb IC——解释器 FORNLOOP 臂对非空计数单元内联递增（T3 单载荷门：
/// 静态旗标读已移出逐回边热路，「非空 ⇔ 旗标开」由本函数仅从旗标开的入口
/// 问询路径可达保证；翻转窗口由 [`forn_trace_backedge`] 复核收口），精确达阈
/// 回调慢路（录制装配 + 解除武装）。与入口计数并存互补：
/// 入口面覆盖高频进入（含零跳/短体环），回边面覆盖低入口频次 × 高迭代量
/// 的单层环。资格预扫结果缓存于 entry（不可录形态一次性判定，免逐入口
/// 重扫税）。
fn arm_backedge_counter(
  l: *mut LuaState,
  backedges: &mut BTreeMap<(usize, u32), Box<u64>>,
  entry: &mut FornLoopEntry,
  proto: *mut Proto,
  pcpos: u32,
  fornloop_pc: u32,
  code: &[Instruction],
) {
  if entry.installed.is_some() || entry.dead {
    return;
  }
  let eligible = entry
    .eligible
    .get_or_insert_with(|| forn_body_eligible(code, pcpos, fornloop_pc));
  if !*eligible || backedges.len() >= K_BACKEDGE_CAP {
    return;
  }
  let counter = backedges.entry((proto as usize, fornloop_pc)).or_default();
  // Safety: l/global 派发环契约存活；ecb IC 为本注册表私有面，同刻仅一位点
  // 驻留（单槽），由本模块武装/解除
  let ecb = unsafe { &mut (*(*l).global).ecb };
  // T3 单载荷门：进程级武装槽上收在位信号（解释器热路一次链深 0 静态读），
  // 身份键/靶值留在 per-state ecb。单线程串行写入，Relaxed 宽松序足够。
  FORN_HEAT_ARMED.store(&mut **counter, AtomicOrdering::Relaxed);
  ecb.forn_heat_proto = proto as usize;
  ecb.forn_heat_pc = fornloop_pc;
  ecb.forn_heat_target = K_HEAT_THRESHOLD;
}

/// 具名调用垫（lldb 断点锚点；逻辑与直调同形）。
#[inline(never)]
unsafe fn call_trace(code_start: *mut u8, l: *mut LuaState, proto: *const Proto) -> u32 {
  type TraceFn = unsafe extern "C" fn(*mut LuaState, *const Proto) -> u32;
  // Safety: code_start 指向已切换可执行页的本模块发射产物
  let f: TraceFn = unsafe { transmute(code_start) };
  // Safety: 生成码叶函数，l/proto 契约存活
  unsafe { f(l, proto) }
}

/// ecb 回边慢路导出（T2）：回边计数精确达阈，录制装配 + 解除武装。
///
/// # Safety
/// `l`/`proto` 为解释器派发环契约的存活指针（h_fornloop 内联计数达阈时
/// 调用），单线程串行；本函数不扰动解释器状态（savedpc/栈/寄存器面零触碰）。
pub unsafe extern "C-unwind" fn forn_trace_backedge_export(
  l: *mut LuaState,
  proto: *mut Proto,
  pcpos: u32,
) {
  // Safety: 契约透传至 forn_trace_backedge
  unsafe { forn_trace_backedge(l, proto, pcpos) }
}

/// 回边阈值主体：FORNLOOP 位点 → 结构恒等式定 FORNPREP（回边 d 域恒指环体
/// 头 = fornprep + 1，故 fornprep = fornloop + d）→ 录制装配（入口路径同款
/// 一次性收口，成败皆 dead 收口防逐回边重试）→ 无论成败解除武装并回收计数
/// 单元——解释器臂下一回边即静默。
///
/// T3 单载荷门：热路（解释器回边臂）已无旗标读，「计数单元非空 ⇔ 旗标开」
/// 由武装侧唯一可达性保证；旗标中途翻转（实验层仅测试会做）的窗口在此复核
/// 收口——关态抵达只解除武装，不录制不安装。
///
/// # Safety
/// 同 [`forn_trace_backedge_export`]。
unsafe fn forn_trace_backedge(l: *mut LuaState, proto: *mut Proto, fornloop_pc: u32) {
  // 先解除武装再动注册表：解释器下一回边起零计数税（进程级武装槽 + per-state
  // 身份键同步清除）
  FORN_HEAT_ARMED.store(null_mut(), AtomicOrdering::Relaxed);
  // Safety: l/global 派发环契约存活
  unsafe { (*(*l).global).ecb.forn_heat_proto = 0 };
  // 旗标中途翻转防御：关态只解除武装（录制安装面在旗标开才可达）
  if !LuauJitFornTrace.get() {
    return;
  }
  // 上下文缺席 → 无武装面，静默
  let ctx = unsafe { get_code_gen_context(l) };
  let Some(ctx) = ctx else {
    return;
  };
  // Safety: proto 为派发环契约的存活对象，code/k 缓冲在其存活期合法
  let (sizecode, code, k) = unsafe {
    if (*proto).sizecode < 0 || fornloop_pc as usize >= (*proto).sizecode as usize {
      return;
    }
    let sizecode = (*proto).sizecode as u32;
    let code: &[Instruction] = from_raw_parts((*proto).code, sizecode as usize);
    let k: &[TValue] = if (*proto).sizek > 0 {
      from_raw_parts((*proto).k, (*proto).sizek as usize)
    } else {
      &[]
    };
    (sizecode, code, k)
  };
  let fl_insn = code[fornloop_pc as usize];
  if LuauOpcode::from(luau_insn_op(fl_insn) as u8) != LuauOpcode::LOP_FORNLOOP {
    return;
  }
  // fornprep = fornloop + d（回边恒指环体头 fornprep+1；record 复核闭环形态）
  let fornprep_pc = i64::from(fornloop_pc) + i64::from(luau_insn_d(fl_insn));
  if fornprep_pc < 0 || fornprep_pc >= i64::from(sizecode) {
    return;
  }
  let fornprep_pc = fornprep_pc as u32;
  let exit_pc = i64::from(fornprep_pc) + 1 + i64::from(luau_insn_d(code[fornprep_pc as usize]));
  if exit_pc < 0 || exit_pc > i64::from(sizecode) {
    return;
  }
  let reg = &mut ctx.forn_traces;
  // 计数单元回收（成败皆然）：阈值已触发过一次，单元生命周期就此收口
  reg.backedges.remove(&(proto as usize, fornloop_pc));
  let Some(entry) = reg.loops.get_mut(&(proto as usize, fornprep_pc)) else {
    return;
  };
  if entry.installed.is_some() || entry.dead {
    return; // 入口路径已收口（并发面：同刻仅一位点武装，单线程无竞争）
  }
  entry.dead = true; // 一次性尝试：失败不重试（防逐回边重试税）
  let ra = luau_insn_a(code[fornprep_pc as usize]) as u8;
  // Safety: 回边位点活跃（h_fornloop 数值判定同面前置），ra+1 合法 tnumber 槽
  let step = unsafe { read_step(l, ra) };
  if let Some(inst) = unsafe {
    record_and_install(
      code,
      k,
      fornprep_pc,
      fornloop_pc,
      exit_pc as u32,
      step,
      (*l).base,
      // Safety: ci->func 为当前帧活跃闭包（派发环契约）
      (*(*(*l).ci).func).as_closure_ptr(),
    )
  } {
    entry.installed = Some(inst);
    STAT_COMPILED.fetch_add(1, AtomicOrdering::Relaxed);
  }
}

/// 阈值触发：录制 → 生成 → 独立分配安装。任一步失败返回 None（dead 收口）。
///
/// step 为录制现场的运行时值（调用点从三元组槽直读，h_fornprep 数值判定
/// 通过保证 tnumber）：step==1.0 锁快 flavor；step≠1 锁泛 flavor——环体含
/// MOD 时拒绝安装（MOD 序列与泛回边的精确性往返复用 d3，固定规划互斥）。
///
/// # Safety
/// `code`/`k` 派生自存活 proto（forn_trace_enter 契约）；`cl` 为当前帧活跃
/// 闭包（`ci->func`），upvalue 形态实测消费。
unsafe fn record_and_install(
  code: &[Instruction],
  k: &[TValue],
  pcpos: u32,
  fornloop_pc: u32,
  exit_pc: u32,
  step: f64,
  base: StkId,
  cl: *mut Closure,
) -> Option<InstalledTrace> {
  // Safety: 契约透传
  let ir: TraceIr = unsafe { record_forn_trace(code, k, pcpos, fornloop_pc, base, cl) }?;
  let phi_indexed = ir.insts.iter().any(|ti| {
    matches!(
      ti.inst,
      TInst::ArrayLoad {
        index: TVal::PhiIdx,
        ..
      } | TInst::ArrayStore {
        index: TVal::PhiIdx,
        ..
      }
    )
  });
  let mod_used = ir.insts.iter().any(|ti| {
    matches!(
      ti.inst,
      TInst::Arith {
        op: TArith::Mod,
        ..
      }
    )
  });
  let step_one_only = step == 1.0;
  if !step_one_only && mod_used {
    return None; // MOD 体 + 泛 flavor：d3 复用互斥，保守回解释器
  }
  let code_bytes = emit_trace_a_64(&ir, LuauTraceFmaFold.get(), step_one_only)?;
  let mut allocator = CodeAllocator::default();
  allocator.code_allocator_usize_usize(K_TRACE_BLOCK_SIZE, K_TRACE_BLOCK_SIZE);
  // Safety: 切片视图派生自本对象存活的 Vec<u32>（4 字节对齐、长度换算不越界）
  let bytes: &[u8] = unsafe {
    from_raw_parts(
      code_bytes.as_ptr().cast(),
      code_bytes.len() * size_of_val(&code_bytes[0]),
    )
  };
  let allocation = allocator.allocate(&[], bytes);
  if allocation.code_start.is_null() {
    allocator.destroy();
    return None;
  }
  Some(InstalledTrace {
    code_start: allocation.code_start,
    words: ir.identity.words,
    k_consts: ir.identity.k_consts,
    imports: ir.imports,
    import_ids: ir.identity.import_ids,
    math_fn_guards: ir.math_fn_guards,
    upvals: ir.upvals,
    sizecode: ir.identity.sizecode,
    exit_pc,
    ra: ir.ra,
    tables: ir.tables,
    inv_nums: ir.inv_nums,
    accs: ir.accs,
    step_one_only,
    phi_indexed,
    allocator,
    allocation,
  })
}
