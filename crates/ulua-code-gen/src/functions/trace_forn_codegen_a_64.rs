//! FORN trace 层 a64 生成码（阶段二 PoC 片 3）：录制 IR → 寄存器驻留原生环。
//!
//! 形态 = 自包含 C ABI 叶函数（无 gate 依赖、无 helper 调用、无栈帧）：
//! `fn(l, proto) -> u32`，返回 0 = 环正常走完/零跳（savedpc 已落环出口）、
//! 1 = 环内守卫 bail（savedpc 已落失败指令位点，解释器整条重做）。
//!
//! 寄存器驻留（trace 层对 method JIT 的核心增量——环变量与活值跨回边驻
//! 硬件寄存器，回边 phi 以寄存器承载）：
//! - d0 = idx 双精度 phi（递增/比较/写回逐位对齐解释器 `setnvalue` 的写回
//!   语义），w1 = idx 派生整数表示（寻址专用：快 flavor 与 d0 同步 +1；
//!   泛 flavor 逐迭代 fcvtzs 往返重导出）；
//! - d1 = limit、d2 = step（快 flavor 恒 1.0 / 泛 flavor 运行时值）、
//!   d3 = MOD 序列与泛 flavor 精确性往返的发射内暂存、d4/d5 = 累加器 phi、
//!   d6/d7 = 环不变 number；
//! - x4..x7 = 环不变 table 的 array 指针、w10..w13 = 对应 sizearray；
//! - d16..d19 = number 常量、d20..d27 = 环体临时；
//! - x8/x9/x14/x15 = 暂存（地址/tag/(savedpc 构建)/ci）。
//!
//! 全部 caller-saved（v8-v15 callee-saved、x18 平台保留、x19-x28 为 gate
//! 惯例不变量，一律规避），无 prologue。
//!
//! 守卫分层：入口守卫（三元组整性/step flavor/table tag/元表缺席/readonly/
//! 不变槽 tag/累加器 tag/中断钩子缺席）在 Rust 侧（trace_forn_registry）
//! 于调用前完成，拒绝路径零状态扰动；逐迭代守卫（(idx-1) 界检查——先于
//! 寻址发射防野指针、元素 tag==number——特化假设的运行期锚、泛 flavor 的
//! idx 精确性往返）在环内，失败即 bail 快照回落。元表/readonly 提至入口
//! 的合法性：环体为直线算术/表存取（无调用/无分配），单线程执行期间无
//! 任何代码可变更它们。
//!
//! 同元素证明传播（T2：守卫面收窄）：ArrayLoad/ArrayStore 的下标恒为环
//! 变量 phi——同一 trace 内全部此类访问落在同一 (table, idx-1) 元素上。
//! 首次全守卫通过即证得该元素「界内 + tnumber」；trace 体直线无调用无
//! 分配（表不容变更、写值全 number、单线程），证明跨指令存活至 trace 末，
//! 后续同表访问免界/tag 守卫、store 免 tag 重写（读改写形态 store 端
//! 7 指令 → 3 指令）。立即数下标（GET/SETTABLEN）与 phi 下标不同元素，
//! 不进证明面。
//!
//! 位一致红线：算术 fadd/fsub/fmul/fdiv 逐位同解释器双舍入；FMA 折叠
//! （fmla 单舍入）仅在 `LuauTraceFmaFold` 开启且 Mul 单-use 供 Add 时
//! 生成（trace 体内无检查点/重启契约，ULP 精度解锁由旗标隔离）。
//!
//! 出口快照：idx（值+tnumber tag）、全部环内写槽（writebacks，值+tag）落
//! 回 VM 栈槽，savedpc 落续延位——与解释器「每迭代覆写、出口留终值、
//! savedpc 指向续延指令」的可观测状态逐位对齐。

use alloc::vec::Vec;
use core::mem::{offset_of, size_of};

use ulua_vm::{
  enums::lua_type::LuaType,
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    lua_state::LuaState,
    lua_t_value::TValue,
    lua_table::LuaTable,
    proto::Proto,
    up_val::UpVal,
  },
  type_aliases::instruction::Instruction,
};

use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  functions::{
    emit_add_offset::emit_add_offset,
    trace_forn_ir::{
      K_MAX_TABLES, TArith, TCmpOp, TCond, TInst, TMathUnary, TUpvalKind, TVal, TableRef,
      TraceInst, TraceIr,
    },
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    label::Label,
    register_a_64::{RegisterA64, reg},
  },
  type_aliases::mem::mem,
};

// —— 固定寄存器规划（见模块注）——
const R_L: RegisterA64 = reg(KindA64::X, 0);
const R_PROTO: RegisterA64 = reg(KindA64::X, 1);
const W_RET: RegisterA64 = reg(KindA64::W, 0);
const W_IDX_I: RegisterA64 = reg(KindA64::W, 1);
const R_BASE: RegisterA64 = reg(KindA64::X, 2);
const R_CODE: RegisterA64 = reg(KindA64::X, 3);
const X_SCRATCH: RegisterA64 = reg(KindA64::X, 8);
/// X_SCRATCH 的低 32 位视（boolean 载荷字判定）
const W_LOW_SCRATCH: RegisterA64 = reg(KindA64::W, 8);
const W_TAG: RegisterA64 = reg(KindA64::W, 9);
/// 元表指针检查用的 64 位视（= W_TAG 同号寄存器；元表检查时 tag 已消费，
/// 覆盖安全）
const X_TAG_PTR: RegisterA64 = reg(KindA64::X, 9);
const X_PC: RegisterA64 = reg(KindA64::X, 14);
const X_CI: RegisterA64 = reg(KindA64::X, 15);
/// k 常量池基址寄存器（prologue GETIMPORT 拷贝专用，= W_TAG 同号；
/// 环内 W_TAG 启用前完成全部拷贝，覆盖安全）
const X_KPOOL: RegisterA64 = reg(KindA64::X, 9);
/// GETIMPORT 全 TValue 拷贝的值位暂存（prologue 段首个临时尚未启用）
const D_IMPORT: RegisterA64 = reg(KindA64::D, 20);
const D_IDX: RegisterA64 = reg(KindA64::D, 0);
const D_LIMIT: RegisterA64 = reg(KindA64::D, 1);
/// step 槽驻留：快 flavor 恒 1.0（fadd 递增），泛 flavor 为运行时 step 值
///（fadd 递增 + fcmpz 选向）。
const D_STEP: RegisterA64 = reg(KindA64::D, 2);
/// 单条发射内自包含的 FP 暂存（MOD 四操作序列 / 泛 flavor 的 idx 精确性
/// 往返），不跨指令存活——故 MOD 体与泛 flavor（逐迭代往返复用 d3）互斥，
/// 由装机面拒录。
const D_MOD: RegisterA64 = reg(KindA64::D, 3);

const fn tbl(i: usize) -> RegisterA64 {
  reg(KindA64::X, (4 + i) as u8)
}
const fn tbl_size(i: usize) -> RegisterA64 {
  reg(KindA64::W, (10 + i) as u8)
}
/// 派生表驻留位（= 第 4 环不变槽位 x7/w13；有派生表时环不变槽收缩为 3，
/// 录制面容量互斥保证不复用）。x7 驻行表 **array 指针**（寻址基与 Inv 槽
/// 同语义，TableLoad 逐迭代重取），w13 驻行表 sizearray；表对象指针不作
/// 驻留（T5-A 深因修复，见 TableLoad 臂注）
const DERIVED_TABLE: RegisterA64 = reg(KindA64::X, 7);
const DERIVED_SIZE: RegisterA64 = reg(KindA64::W, 13);
/// upvalue cell 指针驻留（T8：x14/x15 = X_PC/X_CI 同号——二者仅出口/bail 块
/// 的 emit_savedpc 使用，环体内空闲；出口/bail 块在环体发射完成后落盘，覆盖
/// 安全。cell = `(*UpVal).v`，open→栈槽 / closed→storage，逐入口重解——
/// luaF_close 只发生于调用/返回/CLOSEUPVAL，均不在环体内，环内态不可变）
const fn x_uv(i: usize) -> RegisterA64 {
  reg(KindA64::X, (14 + i) as u8)
}
const fn d_acc(i: usize) -> RegisterA64 {
  reg(KindA64::D, (4 + i) as u8)
}
const fn d_inv(i: usize) -> RegisterA64 {
  reg(KindA64::D, (6 + i) as u8)
}
const fn d_const(i: usize) -> RegisterA64 {
  reg(KindA64::D, (16 + i) as u8)
}
const fn d_temp(t: u8) -> RegisterA64 {
  reg(KindA64::D, 20 + t)
}
// t ∈ 0..K_MAX_TEMPS：0..8 → d20..d27，8..12 → d28..d31（`20 + t` 线性映射
// 自然跨段；d8..d15 callee-saved 规避域不涉）

const K_TVALUE_SIZE_LOG2: i32 = 4;
const K_INSN_SIZE: usize = size_of::<Instruction>();

fn slot_value(reg_index: u8) -> i32 {
  reg_index as i32 * size_of::<TValue>() as i32 + offset_of!(TValue, value) as i32
}

fn slot_tag(reg_index: u8) -> i32 {
  reg_index as i32 * size_of::<TValue>() as i32 + offset_of!(TValue, tt) as i32
}

/// 任意 f64 常量物化：4×movz/movk 拼位模式 + fmov_rr 转入 D 寄存器
///（a64 fpimm 只覆盖窄立即数子集；此序列无条件可用——trace 形态无数据池，
/// imm 序列即池）。
fn materialize_double(b: &mut AssemblyBuilderA64, dst: RegisterA64, v: f64) {
  let bits = v.to_bits();
  let xt = reg(KindA64::X, 12);
  b.movz(xt, (bits >> 48) as u16, 48);
  b.movk(xt, (bits >> 32) as u16, 32);
  b.movk(xt, (bits >> 16) as u16, 16);
  b.movk(xt, bits as u16, 0);
  b.fmov_rr(dst, xt);
}

/// 操作数 → 承载寄存器（固定规划，纯映射）。
fn val_reg(v: TVal) -> RegisterA64 {
  match v {
    TVal::Const(i) => d_const(i),
    TVal::Temp(t) => d_temp(t),
    TVal::InvNum(i) => d_inv(usize::from(i)),
    TVal::Acc(i) => d_acc(usize::from(i)),
    TVal::PhiIdx => D_IDX,
    // Cfn 为函数槽标记，不承载数值（CALL 特化臂直发射单指令面）；到达
    // 此处即录制面形态异常的防御位
    TVal::Cfn { .. } => D_MOD,
  }
}

/// 表源 → (array 指针, sizearray) 驻留寄存器对。
fn table_regs(t: TableRef) -> (RegisterA64, RegisterA64) {
  match t {
    TableRef::Inv(i) => (tbl(usize::from(i)), tbl_size(usize::from(i))),
    TableRef::Derived(_) => (DERIVED_TABLE, DERIVED_SIZE),
  }
}

/// N 形指令的 sizearray 驻留寄存器（界比较用）。
fn tbl_size_of(t: TableRef) -> RegisterA64 {
  table_regs(t).1
}

/// 算术种发射（dst 寄存器由调用方定：Arith → 环体临时，AccArith → phi 寄存器）。
fn emit_arith_op(b: &mut AssemblyBuilderA64, dst: RegisterA64, op: TArith, lhs: TVal, rhs: TVal) {
  match op {
    TArith::Add => b.fadd(dst, val_reg(lhs), val_reg(rhs)),
    TArith::Sub => b.fsub(dst, val_reg(lhs), val_reg(rhs)),
    TArith::Mul => b.fmul(dst, val_reg(lhs), val_reg(rhs)),
    TArith::Div => b.fdiv(dst, val_reg(lhs), val_reg(rhs)),
    // MOD = `a - floor(a/b) * b`（luai_nummod 同序同操作：fdiv→frintm→
    // fmul→fsub，四个 IEEE 正确舍入操作构造性逐位同解释器——NaN/inf/
    // 零除面同构，无需守卫与 bail）
    TArith::Mod => {
      b.fdiv(D_MOD, val_reg(lhs), val_reg(rhs));
      b.frintm(D_MOD, D_MOD);
      b.fmul(D_MOD, D_MOD, val_reg(rhs));
      b.fsub(dst, val_reg(lhs), D_MOD);
    }
  }
}

/// 下标操作数 → W_TAG 的 1 基整数表示：PhiIdx 走驻留整数快路，其余走
/// fcvtzs 加 scvtf 的往返精确性校验（分数/越域 idx 即 bail，类别 2，解释器
/// 慢路承接哈希查表/元表语义）。d3 为发射内暂存。
fn emit_index_exact(b: &mut AssemblyBuilderA64, index: TVal, bail: &mut Label) {
  let r = val_reg(index);
  b.fcvtzs(W_TAG, r);
  b.scvtf(D_MOD, W_TAG);
  b.fcmp(r, D_MOD);
  b.b_cond(ConditionA64::NotEqual, bail);
}

/// FMA 折叠判定：`Add` 恰一方为非临时操作数、另一方为单次使用的 `Mul`
/// 临时 → 折 fmla（other_of 同刀前置——收集面与发射面必须同一判定，否则
/// 供体 Mul 被跳过而 fmla 未生成，读未定义寄存器）。
/// 返回 (被折叠临时, Mul 双操作数) 列表。
fn fma_folds(insts: &[TraceInst]) -> Vec<(u8, TVal, TVal)> {
  // 临时使用计数（任意操作数位上的出现）
  let mut uses = [0u8; 256];
  for ti in insts {
    let mut note = |v: &TVal| {
      if let TVal::Temp(t) = v {
        uses[usize::from(*t)] += 1;
      }
    };
    match ti.inst {
      TInst::ArrayLoad { index, .. } => note(&index),
      TInst::ArrayLoadN { .. } => {}
      TInst::ArrayStore { index, value, .. } => {
        note(&index);
        note(&value);
      }
      TInst::ArrayStoreN { value, .. } => note(&value),
      TInst::TableLoad { index, .. } => note(&index),
      TInst::CondLoad { index, .. } => note(&index),
      TInst::CondLoadN { .. } => {}
      TInst::MathUnary { arg, .. } => note(&arg),
      // T8：UpvalLoad 无 number 操作数（cell 直载）；UpvalStore 值位参与
      // 使用计数（供体判定面）
      TInst::UpvalLoad { .. } => {}
      TInst::UpvalStore { value, .. } => note(&value),
      TInst::AccArith { lhs, rhs, .. } | TInst::Arith { lhs, rhs, .. } => {
        note(&lhs);
        note(&rhs);
      }
      TInst::Branch {
        cond: TCond::Cmp { lhs, rhs, .. },
        ..
      } => {
        note(&lhs);
        note(&rhs);
      }
      TInst::Branch { .. } => {}
    }
  }
  let mut folds = Vec::new();
  for ti in insts {
    let TInst::Arith {
      op: TArith::Add,
      lhs,
      rhs,
      ..
    } = ti.inst
    else {
      continue;
    };
    // 形态闸：恰一方临时（供体位）、另一方非临时（fmla 初值位）
    if other_of(lhs, rhs).is_none() {
      continue;
    }
    let t = match (lhs, rhs) {
      (TVal::Temp(t), _) => t,
      (_, TVal::Temp(t)) => t,
      _ => continue,
    };
    if uses[usize::from(t)] != 1 {
      continue;
    }
    // 供体 = 该临时的唯一定义（临时单写纪律 ⟹ 至多一条 Mul 定义）
    let Some(def) = insts
      .iter()
      .find(|ci| matches!(ci.inst, TInst::Arith { dst, op: TArith::Mul, .. } if dst == t))
    else {
      continue;
    };
    let TInst::Arith {
      lhs: ml, rhs: mr, ..
    } = def.inst
    else {
      continue;
    };
    folds.push((t, ml, mr));
  }
  folds
}

/// 双操作数中的非折叠方（fmla 初值来源）。None = 形态异常（拒编译）。
fn other_of(lhs: TVal, rhs: TVal) -> Option<TVal> {
  match (lhs, rhs) {
    (TVal::Temp(_), other) if !matches!(other, TVal::Temp(_)) => Some(other),
    (other, TVal::Temp(_)) if !matches!(other, TVal::Temp(_)) => Some(other),
    _ => None,
  }
}

/// 生成 trace 原生码。`fma_fold` = `LuauTraceFmaFold`（编译期消费，安装后
/// 旗标翻转不影响已装 trace）。`step_one_only` = step==1.0 特化 flavor：
/// true 走 fadd 1.0 + 整数表示同步 +1 的快回边；false 走泛回边（fadd step +
/// fcmpz 选向 + 逐迭代 idx 精确性往返），flavor 在装机时按录制现场的 step
/// 锁定（泛 flavor 对 step==1 亦逐位正确，仅多几条选向开销）。
pub(crate) fn emit_trace_a_64(
  ir: &TraceIr,
  fma_fold: bool,
  step_one_only: bool,
) -> Option<Vec<u32>> {
  let folds = if fma_fold {
    fma_folds(&ir.insts)
  } else {
    Vec::new()
  };
  let folded_temps: Vec<u8> = folds.iter().map(|&(t, ..)| t).collect();

  // text=true：环体发射面小（≤32 指令），装配期反汇编常驻换取 finalize 校验面
  let mut b = AssemblyBuilderA64::new(true, 0);

  // —— prologue：参数与全环不变量一次装载驻留 ——
  b.ldr(R_BASE, mem(R_L, offset_of!(LuaState, base) as i32));
  b.ldr(R_CODE, mem(R_PROTO, offset_of!(Proto, code) as i32));
  // GETIMPORT 常量拷贝：k[D] 全 TValue（值 8B + tag 4B）→ 槽，位对齐
  // 解释器 fast-path `setobj(ra, kv)`；safeenv/非 nil 守卫已在 Rust 入口
  // 收口。**必须置于 fcvtzs w1（W_IDX_I=R_PROTO 同号）之前**——x1 在此后
  // 即为 idx 整数表示，不再是 proto 指针（曾致 ldr [x1,#8] 段错误）。
  // X_KPOOL(x9)/D_IMPORT(d20) 为 prologue 段专用，环内 W_TAG/临时启用前
  // 全部完成
  if !ir.imports.is_empty() {
    b.ldr(X_KPOOL, mem(R_PROTO, offset_of!(Proto, k) as i32));
    for &(slot, kidx) in &ir.imports {
      // kidx*16 ≤ 4080（录制面守卫 kidx ≤ 255）恒在 imm12 域，单指令偏移
      b.add_rr_u16(X_KPOOL, X_KPOOL, (kidx * 16) as u16);
      b.ldr(D_IMPORT, mem(X_KPOOL, 0));
      b.ldr(W_TAG, mem(X_KPOOL, 12));
      b.str(D_IMPORT, mem(R_BASE, slot_value(slot)));
      b.str(W_TAG, mem(R_BASE, slot_tag(slot)));
      b.sub_rr_u16(X_KPOOL, X_KPOOL, (kidx * 16) as u16);
    }
  }
  b.ldr(D_IDX, mem(R_BASE, slot_value(ir.ra + 2)));
  b.fcvtzs(W_IDX_I, D_IDX);
  b.ldr(D_LIMIT, mem(R_BASE, slot_value(ir.ra)));
  // step 驻留：快 flavor 物化 1.0（fadd 递增与解释器 `idx + step` 在 step
  // 恒 1 下逐位一致）；泛 flavor 装载 step 槽运行时值
  if step_one_only {
    materialize_double(&mut b, D_STEP, 1.0);
  } else {
    b.ldr(D_STEP, mem(R_BASE, slot_value(ir.ra + 1)));
  }
  for (i, &s) in ir.inv_nums.iter().enumerate() {
    b.ldr(d_inv(i), mem(R_BASE, slot_value(s)));
  }
  for (i, &s) in ir.accs.iter().enumerate() {
    b.ldr(d_acc(i), mem(R_BASE, slot_value(s)));
  }
  // 常量物化先于 table 装载（materialize_double 的 x12 暂存与 tbl_size(2/3)
  // 的 w12/w13 冲突——装载序即消解）
  for (i, &(_, v)) in ir.consts.iter().enumerate() {
    materialize_double(&mut b, d_const(i), v);
  }
  for (i, &s) in ir.tables.iter().enumerate() {
    b.ldr(X_SCRATCH, mem(R_BASE, slot_value(s)));
    b.ldr(tbl(i), mem(X_SCRATCH, offset_of!(LuaTable, array) as i32));
    b.ldr(
      tbl_size(i),
      mem(X_SCRATCH, offset_of!(LuaTable, sizearray) as i32),
    );
  }
  // —— upvalue 解析（T8）：closure = ci->func.value.gc，upref 柔性数组寻址。
  // 形态守卫（cell 面 ttisupval、值面 tnumber / math 内建种）已在 Rust 入口
  // 收口，此处纯装载。三分态：
  // - 变更载体：cell 指针驻 x14/x15（body 的 UpvalLoad/UpvalStore 经其读写）；
  // - 只读 Num：值直载保留临时（cell=false 走 upref 本体 / cell=true 经
  //   (*UpVal).v 一级间接），环内零代码；
  // - 只读 Cfn（nbody sqrt 形态）：零装载（值经 fn_slots 身份被 CALL 特化
  //   消费，入口守卫按 cell 源复检）
  for (i, uv) in ir.upvals.iter().enumerate() {
    b.ldr(X_SCRATCH, mem(R_L, offset_of!(LuaState, ci) as i32));
    b.ldr(X_SCRATCH, mem(X_SCRATCH, offset_of!(CallInfo, func) as i32));
    b.ldr(X_SCRATCH, mem(X_SCRATCH, offset_of!(TValue, value) as i32));
    // uprefs[B].value（Closure+LClosure 头 + B*16 + 8，B ≤ 255 恒在
    // 64 位 load 的 imm12 缩放域内）
    let upref_val_off = offset_of!(Closure, inner) as i32
      + offset_of!(LClosure, uprefs) as i32
      + i32::from(uv.idx) * size_of::<TValue>() as i32
      + offset_of!(TValue, value) as i32;
    match (uv.mutated, uv.kind, uv.inv_temp) {
      // 变更载体（cell=true 由录制面保证——mutated + 值内联 upref 拒录）：
      // cell 指针 = (*UpVal).v（open→栈槽 / closed→storage）驻 x_uv(i)
      (true, ..) => {
        b.ldr(X_SCRATCH, mem(X_SCRATCH, upref_val_off));
        b.ldr(x_uv(i), mem(X_SCRATCH, offset_of!(UpVal, v) as i32));
      }
      // 只读 Num：值直载保留临时（cell=true 经 (*UpVal).v 一级间接后 X_SCRATCH
      // 即 cell TValue 地址，偏移归零；cell=false = LCT_VAL 值内联 upref 本体）
      (false, TUpvalKind::Num, Some(t)) => {
        if uv.cell {
          b.ldr(X_SCRATCH, mem(X_SCRATCH, upref_val_off));
          b.ldr(X_SCRATCH, mem(X_SCRATCH, offset_of!(UpVal, v) as i32));
        }
        let value_off = if uv.cell { 0 } else { upref_val_off };
        b.ldr(d_temp(t), mem(X_SCRATCH, value_off));
      }
      // 只读 Cfn 载体：零装载（值经 fn_slots 身份被 CALL 特化消费）
      (false, TUpvalKind::Cfn(_), _) => {}
      // 不可达面：inv_temp 由 walk 惰性分配先于装机，emit 消费同一 IR；
      // 静默空发射会把环内读面引向未初始化 d-temp，IR 回归任何构建即炸
      // （本 tripwire 曾当场抓获 Cfn 影子分类死轨，T8b 入档）
      (false, TUpvalKind::Num, None) => {
        unreachable!("只读 Num 载体 inv_temp 未分配")
      }
    }
  }

  // —— 环体（回边 phi 经寄存器驻留传递：d0/w1 与累加器/临时在回边处原值
  // 存活，即 ZJIT/Maglev block-args 形态的物理实现）——
  let mut loop_head = Label::default();
  b.bind_label(&mut loop_head);
  let mut bails: Vec<(u32, u16, Label)> = Vec::new();
  // 分支目标标签（同目标去重单标签；target == 环尾者绑回边位点）
  let mut branch_labels: Vec<(u32, Label)> = Vec::new();
  for ti in &ir.insts {
    if let TInst::Branch { target, .. } = ti.inst
      && !branch_labels.iter().any(|&(t, _)| t == target)
    {
      branch_labels.push((target, Label::default()));
    }
  }
  // 回边位点标签（continue 语义：分支目标 == 环尾 FORNLOOP——落点 = 回边
  // phi 更新 + 递增 + 判定；区域 acc 的 phi 由直写面保持，无需重放 fmov）
  let fl_pc = ir.exit_pc - 1;
  // 同元素证明面：ArrayLoad/ArrayStore（下标恒 PhiIdx，即全 trace 同一元素）
  // 首次全守卫通过后，该 (table, idx-1) 元素即证得「界内 + tnumber」——
  // trace 体为直线算术/数组存取（无调用无分配：表不容变更、写值全 number、
  // 单线程），证明跨指令有效，后续同表访问免守卫、store 免 tag 重写。
  // 这一面 method JIT 结构不可达（其逐访存守卫绑定单 IR 指令，无跨指令
  // 证明生存期）。
  let mut proven = [false; K_MAX_TABLES];
  // 派生表存储穿透面：TableLoad 位点逐迭代收 readonly 守卫（一次性静态判定）
  let derived_stored = ir.insts.iter().any(|ti| {
    matches!(
      ti.inst,
      TInst::ArrayStore {
        table: TableRef::Derived(_),
        ..
      }
    )
  });
  for ti in &ir.insts {
    // 被 FMA 折叠消费的供体 Mul → 零发射（值由 fmla 处单舍入产出）
    if folded_temps.contains(&temp_dst_of(ti.inst)) {
      continue;
    }
    // 分支目标落此位点 → 绑标签
    if let Some((_, l)) = branch_labels.iter_mut().find(|(t, _)| *t == ti.pc) {
      b.bind_label(l);
    }
    match ti.inst {
      TInst::ArrayLoad {
        dst,
        table: TableRef::Inv(t),
        index: TVal::PhiIdx,
      } if proven[usize::from(t)] => {
        // 证明在手：idx-1 重算 + 寻址 + 值直取（无界/tag 守卫，无 bail 块）
        b.sub_rr_u16(W_TAG, W_IDX_I, 1);
        b.add_rrr_i32(X_SCRATCH, tbl(usize::from(t)), W_TAG, K_TVALUE_SIZE_LOG2);
        b.ldr(
          d_temp(dst),
          mem(X_SCRATCH, offset_of!(TValue, value) as i32),
        );
      }
      TInst::ArrayLoad { dst, table, index } => {
        // 全守卫载：下标整数化（非 PhiIdx 走精确性往返）→ 界 → tag==number
        let (xreg, wsize) = table_regs(table);
        let (mut bnd, mut tag) = (Label::default(), Label::default());
        if index == TVal::PhiIdx {
          b.sub_rr_u16(W_TAG, W_IDX_I, 1);
        } else {
          let mut ex = Label::default();
          emit_index_exact(&mut b, index, &mut ex);
          bails.push((ti.pc, 2, ex));
          b.sub_rr_u16(W_TAG, W_TAG, 1);
        }
        b.cmp_rr(W_TAG, wsize);
        b.b_cond(ConditionA64::UNSIGNED_GREATER_EQUAL, &mut bnd);
        b.add_rrr_i32(X_SCRATCH, xreg, W_TAG, K_TVALUE_SIZE_LOG2);
        emit_tag_guard_and_load(&mut b, d_temp(dst), &mut tag);
        bails.push((ti.pc, 0, bnd));
        bails.push((ti.pc, 1, tag));
        if let (TableRef::Inv(t), TVal::PhiIdx) = (table, index) {
          proven[usize::from(t)] = true;
        }
      }
      TInst::TableLoad { slot, src, index } => {
        // 派生表逐迭代装载：下标整数化 → 源界 → 元素 tag==ttable →（新表
        // sizearray/派生驻留 +）元表缺席 →（存储穿透时）readonly → 槽值回写
        // （值+tag，写回面排除本槽——逐迭代回写保持解释器「每迭代覆写」可见
        // 面）。驻留语义：x7 = 行表 **array 指针**、w13 = 行表 sizearray——
        // 派生 ArrayLoad/Store 的寻址基与 Inv 槽同为 array 指针（T5-A 深因
        // 修复：T4 版驻留对象指针，寻址基错位把读写砸进行表对象头部/字段，
        // j=1 store 的 f64 低 8 字节恰覆盖 tt/marked/memcat 旗标区——
        // checkliveness「gc dead/tt 失配」崩与「布尔载读值漂移」的共同真因）。
        // 对象指针仅在位点内经 X_SCRATCH（元素地址）转取：sizearray/元表/
        // readonly 读 + 槽回写（回写必须是对象指针，解释器可见面是 TValue）。
        let (xreg, wsize) = table_regs(src);
        let (mut bnd, mut tt, mut mt) = (Label::default(), Label::default(), Label::default());
        if index == TVal::PhiIdx {
          b.sub_rr_u16(W_TAG, W_IDX_I, 1);
        } else {
          let mut ex = Label::default();
          emit_index_exact(&mut b, index, &mut ex);
          bails.push((ti.pc, 2, ex));
          b.sub_rr_u16(W_TAG, W_TAG, 1);
        }
        b.cmp_rr(W_TAG, wsize);
        b.b_cond(ConditionA64::UNSIGNED_GREATER_EQUAL, &mut bnd);
        b.add_rrr_i32(X_SCRATCH, xreg, W_TAG, K_TVALUE_SIZE_LOG2);
        b.ldr(W_TAG, mem(X_SCRATCH, offset_of!(TValue, tt) as i32));
        b.cmp_u16(W_TAG, LuaType::Table as u16);
        b.b_cond(ConditionA64::NotEqual, &mut tt);
        b.ldr(X_TAG_PTR, mem(X_SCRATCH, offset_of!(TValue, value) as i32));
        b.ldr(
          DERIVED_SIZE,
          mem(X_TAG_PTR, offset_of!(LuaTable, sizearray) as i32),
        );
        b.ldr(
          DERIVED_TABLE,
          mem(X_TAG_PTR, offset_of!(LuaTable, array) as i32),
        );
        b.ldr(
          X_TAG_PTR,
          mem(X_TAG_PTR, offset_of!(LuaTable, metatable) as i32),
        );
        b.cbnz(X_TAG_PTR, &mut mt);
        bails.push((ti.pc, 0, bnd));
        bails.push((ti.pc, 3, tt));
        bails.push((ti.pc, 4, mt));
        if derived_stored {
          let mut ro = Label::default();
          b.ldr(X_TAG_PTR, mem(X_SCRATCH, offset_of!(TValue, value) as i32));
          b.ldrb(W_TAG, mem(X_TAG_PTR, offset_of!(LuaTable, readonly) as i32));
          b.cbnz(W_TAG, &mut ro);
          bails.push((ti.pc, 5, ro));
        }
        // 槽回写：值字 + tag（元素 tag 已验 ttable，常量物化等值；值必须是
        // 表对象指针——经 X_SCRATCH 转取，x7 现驻 array 指针不可直写）
        b.ldr(X_TAG_PTR, mem(X_SCRATCH, offset_of!(TValue, value) as i32));
        b.str(X_TAG_PTR, mem(R_BASE, slot_value(slot)));
        b.movz(W_TAG, LuaType::Table as u16, 0);
        b.str(W_TAG, mem(R_BASE, slot_tag(slot)));
      }
      TInst::ArrayLoadN { dst, table, c } => {
        // c < sizearray ⇔ !(size <= c)（无符号比较）；c ≤ 255 → c*16 ≤ 4080
        // 恒在 imm12 域内（录制面保证）。立即数下标与 PhiIdx 不同元素，不进证明面
        let (xreg, _) = table_regs(table);
        let mut bnd = Label::default();
        b.cmp_u16(tbl_size_of(table), u16::from(c));
        b.b_cond(ConditionA64::UnsignedLessEqual, &mut bnd);
        b.add_rr_u16(X_SCRATCH, xreg, u16::from(c) * 16);
        let mut tag = Label::default();
        emit_tag_guard_and_load(&mut b, d_temp(dst), &mut tag);
        bails.push((ti.pc, 0, bnd));
        bails.push((ti.pc, 1, tag));
      }
      TInst::ArrayStore {
        table: TableRef::Inv(t),
        value,
        index: TVal::PhiIdx,
      } if proven[usize::from(t)] => {
        // 证明在手：元素已界内且 tnumber，值全 number → 免守卫免 tag 写
        b.sub_rr_u16(W_TAG, W_IDX_I, 1);
        b.add_rrr_i32(X_SCRATCH, tbl(usize::from(t)), W_TAG, K_TVALUE_SIZE_LOG2);
        b.str(
          val_reg(value),
          mem(X_SCRATCH, offset_of!(TValue, value) as i32),
        );
      }
      TInst::ArrayStore {
        table,
        index,
        value,
      } => {
        // 全守卫存：下标整数化 → 界 → 值/tag 双写。派生表的 readonly/元表
        // 守卫已在其 TableLoad 位点逐迭代收口（直线体 TableLoad 必先于存）。
        let (xreg, wsize) = table_regs(table);
        let mut bnd = Label::default();
        if index == TVal::PhiIdx {
          b.sub_rr_u16(W_TAG, W_IDX_I, 1);
        } else {
          let mut ex = Label::default();
          emit_index_exact(&mut b, index, &mut ex);
          bails.push((ti.pc, 2, ex));
          b.sub_rr_u16(W_TAG, W_TAG, 1);
        }
        b.cmp_rr(W_TAG, wsize);
        b.b_cond(ConditionA64::UNSIGNED_GREATER_EQUAL, &mut bnd);
        b.add_rrr_i32(X_SCRATCH, xreg, W_TAG, K_TVALUE_SIZE_LOG2);
        emit_store_value(&mut b, val_reg(value));
        bails.push((ti.pc, 0, bnd));
        if let (TableRef::Inv(t), TVal::PhiIdx) = (table, index) {
          proven[usize::from(t)] = true;
        }
      }
      TInst::ArrayStoreN { table, c, value } => {
        // 立即数下标与 PhiIdx 不同元素，不进证明面
        let (xreg, _) = table_regs(table);
        let mut bnd = Label::default();
        b.cmp_u16(tbl_size_of(table), u16::from(c));
        b.b_cond(ConditionA64::UnsignedLessEqual, &mut bnd);
        b.add_rr_u16(X_SCRATCH, xreg, u16::from(c) * 16);
        emit_store_value(&mut b, val_reg(value));
        bails.push((ti.pc, 0, bnd));
      }
      TInst::Arith { dst, op, lhs, rhs } => {
        // FMA 折叠：Add 的单-use Mul 操作数 → fmov 初值 + fmla（单舍入）
        let fold = folds.iter().find_map(|&(t, ml, mr)| {
          consumed_by(t, lhs, rhs).then_some(()).and_then(|_| {
            let other = other_of(lhs, rhs)?;
            Some((ml, mr, other))
          })
        });
        match fold {
          Some((ml, mr, other)) => {
            b.fmov_rr(d_temp(dst), val_reg(other));
            b.fmla(d_temp(dst), val_reg(ml), val_reg(mr));
          }
          None => emit_arith_op(&mut b, d_temp(dst), op, lhs, rhs),
        }
      }
      // 区域写 acc：直写 phi 寄存器（跳转未走即自然保持旧值）；fmla 折叠
      // 初值位同理（供体 Mul 单-use 判定与 dst 寄存器无关）
      TInst::AccArith { acc, op, lhs, rhs } => {
        let dst = d_acc(usize::from(acc));
        let fold = folds.iter().find_map(|&(t, ml, mr)| {
          consumed_by(t, lhs, rhs).then_some(()).and_then(|_| {
            let other = other_of(lhs, rhs)?;
            Some((ml, mr, other))
          })
        });
        match fold {
          Some((ml, mr, other)) => {
            b.fmov_rr(dst, val_reg(other));
            b.fmla(dst, val_reg(ml), val_reg(mr));
          }
          None => emit_arith_op(&mut b, dst, op, lhs, rhs),
        }
      }
      // 布尔载（truthiness 面）：界守卫（证明在手则免）→ 值 + tag 双装载
      //（NO number tag 特化——boolean 值是合法面）→ 值/tag 回写槽（逐迭代
      // 保持新鲜，写回面排除）
      TInst::CondLoad {
        dst,
        slot,
        table,
        index,
      } => {
        let (xreg, wsize) = table_regs(table);
        let proven_hit =
          matches!(table, TableRef::Inv(t) if proven[usize::from(t)]) && index == TVal::PhiIdx;
        let mut bnd = Label::default();
        let bounds_emitted = if proven_hit {
          b.sub_rr_u16(W_TAG, W_IDX_I, 1);
          b.add_rrr_i32(X_SCRATCH, xreg, W_TAG, K_TVALUE_SIZE_LOG2);
          false
        } else if index == TVal::PhiIdx {
          b.sub_rr_u16(W_TAG, W_IDX_I, 1);
          b.cmp_rr(W_TAG, wsize);
          b.b_cond(ConditionA64::UNSIGNED_GREATER_EQUAL, &mut bnd);
          b.add_rrr_i32(X_SCRATCH, xreg, W_TAG, K_TVALUE_SIZE_LOG2);
          true
        } else {
          let mut ex = Label::default();
          emit_index_exact(&mut b, index, &mut ex);
          bails.push((ti.pc, 2, ex));
          b.sub_rr_u16(W_TAG, W_TAG, 1);
          b.cmp_rr(W_TAG, wsize);
          b.b_cond(ConditionA64::UNSIGNED_GREATER_EQUAL, &mut bnd);
          b.add_rrr_i32(X_SCRATCH, xreg, W_TAG, K_TVALUE_SIZE_LOG2);
          true
        };
        b.ldr(W_TAG, mem(X_SCRATCH, offset_of!(TValue, tt) as i32));
        b.ldr(
          d_temp(dst),
          mem(X_SCRATCH, offset_of!(TValue, value) as i32),
        );
        // 槽回写（真 tag + 值；exit/bail 后解释器读槽即本迭代装载值）
        b.str(d_temp(dst), mem(R_BASE, slot_value(slot)));
        b.str(W_TAG, mem(R_BASE, slot_tag(slot)));
        if bounds_emitted {
          bails.push((ti.pc, 0, bnd));
        }
      }
      TInst::CondLoadN {
        dst,
        slot,
        table,
        c,
      } => {
        let (xreg, wsize) = table_regs(table);
        let mut bnd = Label::default();
        b.cmp_u16(wsize, u16::from(c));
        b.b_cond(ConditionA64::UnsignedLessEqual, &mut bnd);
        b.add_rr_u16(X_SCRATCH, xreg, u16::from(c) * 16);
        b.ldr(W_TAG, mem(X_SCRATCH, offset_of!(TValue, tt) as i32));
        b.ldr(
          d_temp(dst),
          mem(X_SCRATCH, offset_of!(TValue, value) as i32),
        );
        b.str(d_temp(dst), mem(R_BASE, slot_value(slot)));
        b.str(W_TAG, mem(R_BASE, slot_tag(slot)));
        bails.push((ti.pc, 0, bnd));
      }
      // CALL 特化（math 单参内建族）：单指令直译（fabs/frintm/frintp/
      // frinta/fsqrt 均 IEEE 单指令面，与 Rust f64 方法的语义规格对应见
      // TMathUnary 注）。参数/返回皆 number 特化面（Temp/Const/
      // InvNum/Acc/PhiIdx 的特化不变量），内建槽由入口守卫复检；返回写
      // func 槽（值 + tnumber tag，位对齐解释器 CALL 返回的 setobj2t——
      // number 无 GC 屏障参与）
      TInst::MathUnary {
        dst,
        dst_slot,
        kind,
        arg,
        ..
      } => {
        match kind {
          TMathUnary::Sqrt => b.fsqrt(d_temp(dst), val_reg(arg)),
          TMathUnary::Abs => b.fabs(d_temp(dst), val_reg(arg)),
          TMathUnary::Floor => b.frintm(d_temp(dst), val_reg(arg)),
          TMathUnary::Ceil => b.frintp(d_temp(dst), val_reg(arg)),
          TMathUnary::Round => b.frinta(d_temp(dst), val_reg(arg)),
        }
        b.str(d_temp(dst), mem(R_BASE, slot_value(dst_slot)));
        b.movz(W_TAG, LuaType::Number as u16, 0);
        b.str(W_TAG, mem(R_BASE, slot_tag(dst_slot)));
      }
      // T8 变更载体 upvalue 读：cell 指针（x14/x15）值/tag 守卫 + 值装载。
      // tag==tnumber 是特化假设的运行期锚——非 number upvalue（字符串等）在此
      // bail（类别 6），savedpc 落 GETUPVAL 位点，解释器重做后承接任意类型
      TInst::UpvalLoad { dst, uv } => {
        let mut tag = Label::default();
        b.ldr(
          W_TAG,
          mem(x_uv(usize::from(uv)), offset_of!(TValue, tt) as i32),
        );
        b.cmp_u16(W_TAG, LuaType::Number as u16);
        b.b_cond(ConditionA64::NotEqual, &mut tag);
        b.ldr(
          d_temp(dst),
          mem(x_uv(usize::from(uv)), offset_of!(TValue, value) as i32),
        );
        bails.push((ti.pc, 6, tag));
      }
      // T8 upvalue 写：值 + tnumber tag 穿 cell 双写（open 态栈写穿透 /
      // closed 态 storage 写，同一 cell 指针语义）。屏障面：luaC_barriert
      // 谓词 iscollectable(v) 对 number 恒假 → 构造性零动作（解释器
      // h_setupval 的屏障在同值面同样零动作，T7 派生写同款论证）
      TInst::UpvalStore { uv, value } => {
        b.str(
          val_reg(value),
          mem(x_uv(usize::from(uv)), offset_of!(TValue, value) as i32),
        );
        b.movz(W_TAG, LuaType::Number as u16, 0);
        b.str(
          W_TAG,
          mem(x_uv(usize::from(uv)), offset_of!(TValue, tt) as i32),
        );
      }
      // 条件分支：真值面（tag 驻 W9，CondLoad 紧邻存活）/ 数值面（fcmp +
      // VS 门控，NaN 无序语义逐位对齐解释器比较臂）/ 常量折叠面（无条件跳）
      TInst::Branch { cond, target } => {
        let (_, tlabel) = branch_labels.iter_mut().find(|(t, _)| *t == target)?;
        match cond {
          TCond::Always => b.b_label(tlabel),
          TCond::Truth { temp, positive } => {
            // W9 = tag（CondLoad 紧邻存活），d_temp = 值；真值 = 非 nil 且
            // 非 false（boolean 载荷取低 32 位字）
            let (mut done, mut boolcase) = (Label::default(), Label::default());
            b.cmp_u16(W_TAG, LuaType::Nil as u16);
            if positive {
              // JUMPIF：nil 落穿；非 bool 非 nil 跳；bool true 跳
              b.b_cond(ConditionA64::Equal, &mut done);
              b.cmp_u16(W_TAG, LuaType::Boolean as u16);
              b.b_cond(ConditionA64::Equal, &mut boolcase);
              b.b_label(tlabel);
              b.bind_label(&mut boolcase);
              b.fmov_rr(X_SCRATCH, d_temp(temp));
              b.cmp_u16(W_LOW_SCRATCH, 0);
              b.b_cond(ConditionA64::NotEqual, tlabel);
              b.bind_label(&mut done);
            } else {
              // JUMPIFNOT：nil 跳；非 bool 非 nil 落穿；bool false 跳
              b.b_cond(ConditionA64::Equal, tlabel);
              b.cmp_u16(W_TAG, LuaType::Boolean as u16);
              b.b_cond(ConditionA64::Equal, &mut boolcase);
              b.b_label(&mut done);
              b.bind_label(&mut boolcase);
              b.fmov_rr(X_SCRATCH, d_temp(temp));
              b.cmp_u16(W_LOW_SCRATCH, 0);
              b.b_cond(ConditionA64::NotEqual, &mut done);
              b.b_label(tlabel);
              b.bind_label(&mut done);
            }
          }
          // nil 常量比较：tag 从 base 槽读（任意类型槽面，不依赖紧邻加载）
          TCond::Nil { slot, positive } => {
            let mut done = Label::default();
            b.ldr(W_TAG, mem(R_BASE, slot_tag(slot)));
            b.cmp_u16(W_TAG, LuaType::Nil as u16);
            if positive {
              b.b_cond(ConditionA64::Equal, tlabel);
            } else {
              b.b_cond(ConditionA64::Equal, &mut done);
              b.b_label(tlabel);
            }
            b.bind_label(&mut done);
          }
          TCond::Cmp {
            op,
            lhs,
            rhs,
            positive,
          } => {
            b.fcmp(val_reg(lhs), val_reg(rhs));
            let mut done = Label::default();
            match (op, positive) {
              (TCmpOp::Lt, true) => {
                b.b_cond(ConditionA64::Overflow, &mut done);
                b.b_cond(ConditionA64::Minus, tlabel);
              }
              (TCmpOp::Lt, false) => {
                b.b_cond(ConditionA64::Overflow, tlabel);
                b.b_cond(ConditionA64::Minus, &mut done);
                b.b_label(tlabel);
              }
              (TCmpOp::Le, true) => {
                b.b_cond(ConditionA64::Overflow, &mut done);
                b.b_cond(ConditionA64::UnsignedLessEqual, tlabel);
              }
              (TCmpOp::Le, false) => {
                b.b_cond(ConditionA64::Overflow, tlabel);
                b.b_cond(ConditionA64::UnsignedLessEqual, &mut done);
                b.b_label(tlabel);
              }
              (TCmpOp::Eq, true) => {
                b.b_cond(ConditionA64::Overflow, &mut done);
                b.b_cond(ConditionA64::Equal, tlabel);
              }
              (TCmpOp::Eq, false) => {
                b.b_cond(ConditionA64::Overflow, tlabel);
                b.b_cond(ConditionA64::Equal, &mut done);
                b.b_label(tlabel);
              }
            }
            b.bind_label(&mut done);
          }
        }
      }
    }
  }

  // —— 回边：phi 递增 + 环条件（逐位对齐 fornloop_step：`idx += step` 后按
  // step 符号选向——`step > 0 ? idx <= limit : limit <= idx`，f64 偏序同式，
  // NaN 无序 → 两向比较皆假 → 退出）——
  // —— continue 语义落点：分支目标 == 环尾（FORNLOOP）的标签绑在此处，
  // 区域 acc 的 phi 已由 Arm 内直写面就地保持，跳转未走即旧值，无需 fmov ——
  if let Some((_, l)) = branch_labels.iter_mut().find(|(t, _)| *t == fl_pc) {
    b.bind_label(l);
  }
  // —— 回边 phi 传递：累加器新值回填 phi 寄存器（block-args 物理形态——
  // 下一迭代读 Acc(i) 即 phi 入参 = 上迭代终值）；区域写 acc 豁免（直写面
  // 已就地更新，分支未走须保持旧值）——
  for &(slot, final_v) in ir.writebacks.iter() {
    if let Some(i) = ir.accs.iter().position(|&a| a == slot)
      && !ir.region_accs.contains(&slot)
    {
      b.fmov_rr(d_acc(i), val_reg(final_v));
    }
  }
  let mut exit = Label::default();
  if step_one_only {
    b.fadd(D_IDX, D_IDX, D_STEP);
    b.add_rr_u16(W_IDX_I, W_IDX_I, 1);
    b.fcmp(D_IDX, D_LIMIT);
    // VS（fcmp 仅无序置位）→ 出口；无序面已被前一支收口，循环臂按 FP 条件纪律
    // 取 LS（UnsignedLessEqual，C 清或 Z 置）——与 get_condition_fp 的
    // LessEqual 映射同源（LE 助记在无序面为真，禁用于 FP 比较）
    b.b_cond(ConditionA64::Overflow, &mut exit);
    b.b_cond(ConditionA64::UnsignedLessEqual, &mut loop_head);
    // （有序 GT 落穿即出口）
  } else {
    b.fadd(D_IDX, D_IDX, D_STEP);
    b.fcmp(D_IDX, D_LIMIT);
    b.b_cond(ConditionA64::Overflow, &mut exit);
    // 选向：fcmpz step，gt = 正步走 LE 臂，否则（含 ±0）走 GE 臂——与
    // 解释器 `step > 0.0` 的分支谓词逐位一致（NaN step 已被入口守卫拒承）
    b.fcmpz(D_STEP);
    let (mut pos, mut cont) = (Label::default(), Label::default());
    b.b_cond(ConditionA64::Greater, &mut pos);
    // step <= 0：cont ⇔ limit <= idx ⇔ idx >= limit（GE；无序已被 VS 收口）
    b.fcmp(D_IDX, D_LIMIT);
    b.b_cond(ConditionA64::GreaterEqual, &mut cont);
    b.b_label(&mut exit);
    b.bind_label(&mut pos);
    b.fcmp(D_IDX, D_LIMIT);
    b.b_cond(ConditionA64::UnsignedLessEqual, &mut cont);
    b.b_label(&mut exit);
    b.bind_label(&mut cont);
    // 提交 idx+step 到槽：回边 bail 若以环体头续延，解释器重跑环体而非
    // FORNLOOP——槽须已含本次递增（与 setnvalue 写回时点对齐）；正常出口
    // 路径的快照写回重写同值，幂等。
    b.str(D_IDX, mem(R_BASE, slot_value(ir.ra + 2)));
    // PhiIdx 寻址面：idx 须精确 i32（往返恒等校验，不精确 bail 回环体头，
    // 解释器慢路承接分数/越域 idx 的哈希查表语义）。泛 flavor 下 MOD 与
    // 本序列复用 d3，装机面已互斥拒录。
    if ir.insts.iter().any(|ti| {
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
    }) {
      let mut exact_bail = Label::default();
      b.fcvtzs(W_TAG, D_IDX);
      b.scvtf(D_MOD, W_TAG);
      b.fcmp(D_IDX, D_MOD);
      b.b_cond(ConditionA64::NotEqual, &mut exact_bail);
      b.mov_rr(W_IDX_I, W_TAG);
      b.b_label(&mut loop_head);
      // 回边 bail 块在函数尾部统一发射（bails 末位追加，见下方 bail 循环）
      bails.push((ir.insts[0].pc, 2, exact_bail));
    } else {
      b.b_label(&mut loop_head);
    }
  }

  // —— 出口快照：idx 写回（值 + tnumber tag，对齐 setnvalue）+ 写回面 +
  // savedpc = 环出口 + 返回 0 ——
  b.bind_label(&mut exit);
  emit_idx_writeback(&mut b, ir.ra + 2);
  emit_writebacks(&mut b, ir, &folded_temps);
  emit_savedpc(&mut b, ir.exit_pc);
  b.movz(W_RET, 0, 0);
  b.ret();

  // —— bail 快照（逐守卫两块：界/tag）：idx/写回面按当前进度落盘，
  // savedpc = 失败指令位点 + 返回 1（解释器整条重做该指令）。
  // PoC 诊断约定：返回值 = inst 序 × 8 + 1 + 类别（0=界 1=元素tag 2=idx
  // 精确性），registry 只判非零——运行期守卫类别的可观测面（rt 测试与
  // 诚实报告消费）；×8 间距为族 1 的派生表守卫类别（3=表tag 4=元表
  // 5=readonly）与 T8 的 upvalue tag（6）预留单调位面。
  for (idx, (pc, kind, label)) in bails.iter_mut().enumerate() {
    b.bind_label(label);
    emit_idx_writeback(&mut b, ir.ra + 2);
    emit_writebacks(&mut b, ir, &folded_temps);
    emit_savedpc(&mut b, *pc);
    b.movz(W_RET, idx as u16 * 8 + 1 + *kind, 0);
    b.ret();
  }

  if !b.finalize() {
    return None;
  }
  let live = b.get_code_size() as usize;
  Some(b.core.code[..live].to_vec())
}

/// 环控制变量写回（值 + tnumber tag，逐位对齐解释器 setnvalue）。
fn emit_idx_writeback(b: &mut AssemblyBuilderA64, idx_slot: u8) {
  b.str(D_IDX, mem(R_BASE, slot_value(idx_slot)));
  b.movz(W_TAG, LuaType::Number as u16, 0);
  b.str(W_TAG, mem(R_BASE, slot_tag(idx_slot)));
}

/// 写回面：环内写槽按终值落盘（值 + tnumber tag——全部写值皆 number 特化面）。
/// 累加器槽写 **phi 寄存器 d_acc**（非 final_v 寄存器）：d_acc 在任意 bail/
/// 出口点恒持当前累计值（prologue 装载 + 逐回边 fmov 回填），而 final_v 的
/// 承载寄存器（如 ADD 结果临时）在 bail 落于定义点之前时是未初始化垃圾——
/// 快照写垃圾进槽即污染解释器重做路径（T1 起潜伏，嵌套表载 bail 面首次
/// 暴露）；出口点两者等值（回边 fmov 已回填），故此修正对出口零差异。
/// `folded` = FMA 折叠供体临时集：供体 Mul 寄存器无定义写（fmla 单舍入面
/// 代为产出），其槽写回读未定义寄存器即落垃圾——该槽为编译器临时（环外
/// 死值，位次预扫无条件写但无出口可观测读），跳过保持入口值。
fn emit_writebacks(b: &mut AssemblyBuilderA64, ir: &TraceIr, folded: &[u8]) {
  for &(slot, v) in &ir.writebacks {
    if let TVal::Temp(t) = v
      && folded.contains(&t)
    {
      continue;
    }
    let reg = match ir.accs.iter().position(|&a| a == slot) {
      Some(i) => d_acc(i),
      None => val_reg(v),
    };
    b.str(reg, mem(R_BASE, slot_value(slot)));
    b.movz(W_TAG, LuaType::Number as u16, 0);
    b.str(W_TAG, mem(R_BASE, slot_tag(slot)));
  }
}

/// savedpc 落位（ci->savedpc = code + pcpos，解释器续延协议同 copatch bail）。
fn emit_savedpc(b: &mut AssemblyBuilderA64, pcpos: u32) {
  emit_add_offset(b, X_PC, R_CODE, pcpos as usize * K_INSN_SIZE);
  b.ldr(X_CI, mem(R_L, offset_of!(LuaState, ci) as i32));
  b.str(X_PC, mem(X_CI, offset_of!(CallInfo, savedpc) as i32));
}

/// 元素 tag 守卫 + 值装载（值字读入 d 寄存器；tag 特化假设的运行期锚）。
fn emit_tag_guard_and_load(b: &mut AssemblyBuilderA64, dst: RegisterA64, bail: &mut Label) {
  b.ldr(W_TAG, mem(X_SCRATCH, offset_of!(TValue, tt) as i32));
  b.cmp_u16(W_TAG, LuaType::Number as u16);
  b.b_cond(ConditionA64::NotEqual, bail);
  b.ldr(dst, mem(X_SCRATCH, offset_of!(TValue, value) as i32));
}

/// 数组写：值字 + tnumber tag 字（解释器 setobj2t 双写同构；number 无 GC
/// 屏障参与，readonly/元表已提至入口守卫）。
fn emit_store_value(b: &mut AssemblyBuilderA64, value: RegisterA64) {
  b.str(value, mem(X_SCRATCH, offset_of!(TValue, value) as i32));
  b.movz(W_TAG, LuaType::Number as u16, 0);
  b.str(W_TAG, mem(X_SCRATCH, offset_of!(TValue, tt) as i32));
}

fn temp_dst_of(inst: TInst) -> u8 {
  match inst {
    TInst::Arith { dst, .. } => dst,
    _ => u8::MAX,
  }
}

/// `t` 是否为 lhs/rhs 中被折叠的操作数位。
fn consumed_by(t: u8, lhs: TVal, rhs: TVal) -> bool {
  lhs == TVal::Temp(t) || rhs == TVal::Temp(t)
}
