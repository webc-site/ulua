//! JIT 用户函数 call inlining（第 2 阶段：静态直通 + 运行时证据门）。
//!
//! 判据（翻译期可判，任一不满足即整体放弃、走常规 CALL 发射）：
//!
//! - callee 槽身份二选一。静态：callee 槽寄存器在本 proto 字节码内有唯一静态
//!   定义点，定义链仅由 `NEWCLOSURE`/`DUPCLOSURE`（可经 ≤2 跳 `MOVE`）到达——
//!   callee proto 编译期可辨，直通无守卫（第 1 阶段形态）。观测（第 2 阶段）：
//!   静态链不可辨（GETUPVAL/GETTABLEKS/NAMECALL 喂 CALL）时，取暖重编译注入的
//!   CALL 站点观测提示（`call_hints`，见 ulua-vm call_obs）——运行时闭包的
//!   proto 恒定证据替代「常量 proto 槽」判据，发射 `JumpEqTag` + `JumpCmpProtoid`
//!   （funid 立即数，装载期全局唯一、单调不复用）+ `CheckStackRoom`（栈余量
//!   运行时守卫），任一不满足落常规 CALL 回退块。NAMECALL 虚调用（方法表经
//!   `__index` 链解析，函数值落 ra 后紧邻 CALL）即由本路径承接。
//! - callee 非变参、`numparams == 实参数`、指令数 ≤ 100、无回边（禁止循环体）。
//! - 体白名单：纯数据搬运/常量加载/数值算术/跳转；刀 C 放行表读族
//!   `GETTABLEKS`（`self.field` 形态）；试探翻译产物经 Fallback 块折叠（见下）
//!   后不含慢路命令、不可物化的 `VmConst`/`VmUpvalue` 操作数；`VmExit` 出口
//!   统一重写为 `call_pc`（语义面见下）。
//! - 返回点：每条 `RETURN` 的个数 == caller 期望（逐 RETURN 对齐），允许多
//!   RETURN（第 2 阶段推广，见 emit_inline 的折叠循环）。
//! - 栈深：静态路径要求 `ra + callee.maxstacksize ≤ caller.maxstacksize`（映射区
//!   落在 caller 已预留栈内）；观测路径由 `CheckStackRoom` 运行时承接。体寄存器
//!   按 `callee reg k ↔ caller reg ra+1+k` 直通映射，与真实帧布局（base = ra+1）
//!   逐位一致；CALL 语义本就允许覆写 `ra+1..` 槽位，内联不扩大覆写面。
//!
//! 语义面（三处出口都收敛到「解释器自 CALL 原位重放整次调用」）：
//!
//! - 体 `INTERRUPT` 的 pcpos 重写为调用点（第 1 阶段已验证重放语义）；
//! - Fallback 块折叠：指向 Fallback 块的 Block 操作数全部改写为
//!   `VmExit(call_pc)`——内联体只保留快路，类型/形状不符即退出解释器，由 CALL
//!   原位重放承接元方法等慢路（重放语义与 INTERRUPT 同款）；callee 原生
//!   `VmExit(callee_pc)` 出口（类型侧 TABLE 的 CheckTag 失败臂等）同款重写为
//!   `call_pc`，重放点统一收敛到 CALL 原位；
//! - `GETTABLEKS` 的 IC 提示槽寻址（`GetSlotNodeAddr`）运行时读
//!   `R_CODE[pcpos].C`，内联体在 caller 上下文执行时 `R_CODE` 指向 caller 的
//!   code——callee pcpos 错位，发射端换主位探测 `GetHashNodeAddr`（编译期
//!   `ts->hash` 立即数，NAMECALL 第一跳同款）；主位 miss 落 `VmExit` 重放，
//!   碰撞链/元表行走由解释器慢路承接；
//! - 观测路径守卫失败：跳常规 CALL 回退块（SetSavedpc+CALL），与未内联发射逐位一致。
//!
//! 常量重定位（NAMECALL 阶段）：`VmConst` 经入口序言装载的 `R_CONSTANTS`
//! （= 当前 proto 的 `k` 基址）寻址，内联体在 caller 帧执行。number 常量逐值
//! 重 intern（`const_double` 直生）；字符串常量（`GETTABLEKS` key/`LOADK`）在
//! 发射前经 `proto_k_intern_string` 物化进 caller 常量表（复用既有同指针项或
//! 追加新槽），`VmConst` 下标重写到 caller 表——失败即整体放弃内联。

use core::ptr::null_mut;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag,
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_d, luau_insn_op},
  records::small_vector::SmallVector,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::proto_k_intern_string::proto_k_intern_string,
  records::{closure::Closure, proto::Proto},
};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    ir::{add_use, is_pseudo},
    proto_view::{child_proto, child_proto_ref, constant_number, with_constant_value, with_proto},
    proto_views::{code, string_constant_gc, string_constant_hash},
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_const::IrConst, ir_function::IrFunction,
    ir_op::IrOp,
  },
  type_aliases::ir::{Instruction, IrOps},
};

/// callee 指令数上限（保守线）
const K_MAX_INLINE_INSNS: usize = 100;
/// `MOVE` 定义链最大跳数
const K_MAX_DEF_CHAIN: usize = 2;

/// callee 槽寄存器的静态定义形态
#[derive(Clone, Copy)]
enum RegDef {
  /// `NEWCLOSURE A D`：子原型索引
  NewClosure(u32),
  /// `DUPCLOSURE A D`：模板闭包常量索引
  DupClosure(u32),
  /// `MOVE A B`：沿 B 寄存器续查
  Move(u8),
}

/// 内联解析结果：静态 proto（直通）或观测证据（守卫）。
enum Callee {
  /// 编译期可辨 proto，直通无守卫。
  Static(*mut Proto),
  /// 观测证据：callee proto 裸址（判据用）+ funid（守卫立即数）。
  Observed { proto: *mut Proto, funid: u32 },
}

/// 尝试把 `CALL` 站点内联展开进 caller IR；返回是否已内联（true 时调用方
/// 跳过常规 `SetSavedpc`+`CALL` 发射）。
pub(crate) fn try_translate_call_inline(
  build: &mut IrBuilder,
  caller_code: &[Instruction],
  i: i32,
  b_raw: u8,
  c_raw: u8,
) -> bool {
  // FASTCALL fallback 区内不做（保守）；multret 实参/期望返回不做
  if build.active_fastcall_fallback || b_raw == 0 || c_raw == 0 {
    return false;
  }

  let nparams = b_raw as i32 - 1;
  let nresults = c_raw as i32 - 1;
  let ra = luau_insn_a(caller_code[i as usize]) as u8;

  let callee = match resolve_callee_proto(build, caller_code, ra, i) {
    Some(proto) => Callee::Static(proto),
    // 静态链不可辨：暖重编译注入的观测提示接棒（GETUPVAL/NAMECALL 形态）
    None => match observed_callee(build, i) {
      Some((proto, funid)) => Callee::Observed { proto, funid },
      None => return false,
    },
  };
  let callee_proto = match callee {
    Callee::Static(proto) | Callee::Observed { proto, .. } => proto,
  };
  if check_callee_bytecode(
    build,
    callee_proto,
    nparams,
    nresults,
    ra,
    matches!(callee, Callee::Static(_)),
  )
  .is_none()
  {
    return false;
  }

  let callee_ref = unsafe { &*callee_proto };
  let callee_insns = callee_ref.sizecode as usize;

  // 试探翻译：在独立 IrBuilder 上按既有管线构建 callee IR，任何慢路面即弃
  let hooks = build.host_hooks_ref();
  let mut trial = IrBuilder::ir_builder_ir_builder(hooks);
  // Safety: callee 为 caller proto `p[]` 数组内存活子原型（静态路径契约同
  // `translate_inst_new_closure`）或观测窗口内被 ra 槽闭包锚定的存活 proto
  // （同步暖重编译触发，见 call_obs 模块注），codegen 期间 VM 持有、只读。
  unsafe { trial.build_function_ir(callee_proto) };

  let mut trial_function = trial.function;
  if inline_ir_veto(&trial_function, callee_proto).is_some() {
    return false;
  }
  // NAMECALL 阶段：callee 字符串常量先物化进 caller 常量表（发射前完成——失败
  // 即整体放弃内联，发射面不留半途状态）。number 常量不走此表（emit 直生）。
  let Some(string_map) = relocate_string_consts(build, callee_proto, &trial_function) else {
    return false;
  };
  let guard = match callee {
    Callee::Static(_) => None,
    Callee::Observed { funid, .. } => {
      // CheckStackRoom 的槽界：base + (ra+1+callee.maxstack) ≤ stack_last，
      // 恰覆盖内联体的最高寻址槽（帧映射 ra+1+k，k ≤ maxstack-1）
      let maxstack = unsafe { (*callee_proto).maxstacksize as i32 };
      Some((funid, ra as i32 + 1 + maxstack))
    }
  };
  emit_inline(
    build,
    &mut trial_function,
    callee_proto,
    ra,
    nparams,
    nresults,
    i,
    guard,
    &string_map,
  );
  // 内联发射证据：编译期单次打印（compile-once，不进热路径）
  match guard {
    Some((funid, _)) => eprintln!(
      "[call-inline] hit caller_pc={i} ra={ra} nresults={nresults} callee_insns={callee_insns} observed funid={funid} relocated_strings={}",
      string_map.len()
    ),
    None => eprintln!(
      "[call-inline] hit caller_pc={i} ra={ra} nresults={nresults} callee_insns={callee_insns} static"
    ),
  }
  true
}

/// 观测提示查找：`call_hints` 里取本调用点 pc 的 `(callee proto 裸址, funid)`。
/// 仅暖重编译路径有提示（首译时观测尚未发生，走常规 CALL）。
fn observed_callee(build: &IrBuilder, call_pc: i32) -> Option<(*mut Proto, u32)> {
  if !fflag::LUAU_JIT_CALL_INLINE_OBS.get() {
    return None;
  }
  build
    .function
    .call_hints
    .iter()
    .find(|(pc, ..)| *pc == call_pc as u32)
    .map(|&(_, funid, proto_addr)| (proto_addr as *mut Proto, funid))
}

/// NAMECALL 阶段：把 trial IR 引用的 callee 字符串常量重定位进 caller 常量表。
///
/// 扫 trial 指令的 `VmConst` 操作数，字符串项经 `proto_k_intern_string` intern
/// （同指针项复用既有下标），产出 `(callee aux → caller aux)` 映射；number 项
/// 不入表（emit 逐值直生）。宿主态缺席（纯 IR 工具链路径）或 intern 失败返回
/// `None`，调用方整体放弃内联（发射面不留半途状态）。
fn relocate_string_consts(
  build: &mut IrBuilder,
  callee: *mut Proto,
  trial: &IrFunction,
) -> Option<Vec<(u32, u32)>> {
  // 宿主态接线缺席 = 字符串重定位不可用；无 VmConst 时静态路径照常直通
  let has_string = trial.instructions.iter().any(|inst| {
    inst.ops.iter().any(|op| {
      op.kind() == IrOpKind::VmConst
        && with_constant_value(child_proto_ref(callee), op.index(), |tv| tv.is_string())
          .unwrap_or(false)
    })
  });
  if !has_string {
    return Some(Vec::new());
  }
  let l = build.function.l.map(|h| h.as_ptr())?;
  let caller = build.function.proto_view()?;
  let caller_ptr = core::ptr::from_ref(caller).cast_mut();
  let callee_ref = child_proto_ref(callee)?;

  let mut map: Vec<(u32, u32)> = Vec::new();
  for inst in &trial.instructions {
    for op in inst.ops.iter() {
      if op.kind() != IrOpKind::VmConst || map.iter().any(|(from, _)| *from == op.index()) {
        continue;
      }
      // number 项跳过（emit 逐值直生）；非常量下标/非字符串不可达（veto 保证），
      // 判定前不可触 `as_string` 断言
      let is_str =
        with_constant_value(Some(callee_ref), op.index(), |tv| tv.is_string()).unwrap_or(false);
      if !is_str {
        continue;
      }
      // Safety: `ts` 由 callee proto k 表锚定（观测窗口内被 ra 槽闭包锚定，同步
      // 暖重编译契约同 observed_callee）；intern 只读 ts、写入 caller k 表。
      let ts = string_constant_gc(callee_ref, op.index() as usize)?;
      let idx = unsafe { proto_k_intern_string(l, caller_ptr, ts) }?;
      map.push((op.index(), idx));
    }
  }
  Some(map)
}

/// 查字符串重定位映射：callee aux → caller aux（必命中，emit 前已物化）。
fn relocated_index(map: &[(u32, u32)], aux: u32) -> u32 {
  map
    .iter()
    .find(|(from, _)| *from == aux)
    .map(|&(_, to)| to)
    .unwrap_or_else(|| {
      CODEGEN_ASSERT!(false, "inline rewrite: unrelocated string const");
      0
    })
}

/// 收集 trial 的 Fallback 块号（emit 的 rewrite 链据此把慢路出口臂改写为出口
/// 块）。语义：内联体只保留快路，类型/形状不符即落该块（SetSavedpc+CALL+JUMP
/// 后继，与未内联发射逐位一致），元方法等慢路由 CALL lowering 承接——纯生成
/// 码内控制流，零解释器状态依赖（裸 `VmExit` 出口缺 ExitSync 值同步，见
/// emit_inline 注）。只收集不改写：fold 写入的 caller 域 Block op 会被 rewrite
/// 链再平移（Block 分支统一 `block_base+index`），必须留给 emit 特判。
fn collect_fallback_blocks(trial: &IrFunction) -> Vec<u32> {
  trial
    .blocks
    .iter()
    .enumerate()
    .filter(|(_, b)| b.kind == IrBlockKind::Fallback)
    .map(|(i, _)| i as u32)
    .collect()
}

/// 单定义扫描：`reg` 在整段字节码内的写点唯一且为链形态才返回定义。
/// `exclude_pc` 供首跳排除被内联的 CALL 自身（CALL 写返回值槽）。
fn single_def_reg(code: &[Instruction], reg: u8, exclude_pc: Option<usize>) -> Option<RegDef> {
  let mut def: Option<RegDef> = None;
  for (pc, &insn) in code.iter().enumerate() {
    if exclude_pc == Some(pc) {
      continue;
    }
    let op = LuauOpcode::from(luau_insn_op(insn) as u8);
    let a = luau_insn_a(insn) as u8;
    if !insn_writes_reg(op, a, luau_insn_b(insn) as u8, reg) {
      continue;
    }
    // 命中写点：仅三种链形态可作唯一定义，其余写法直接淘汰
    let kind = match op {
      LuauOpcode::LopNewclosure => RegDef::NewClosure(luau_insn_d(insn) as u32),
      LuauOpcode::LopDupclosure => RegDef::DupClosure(luau_insn_d(insn) as u32),
      LuauOpcode::LopMove => RegDef::Move(luau_insn_b(insn) as u8),
      _ => return None,
    };
    if def.is_some() {
      return None;
    }
    def = Some(kind);
  }
  def
}

/// 该指令是否可能写 `reg` 槽。已知非写指令显式排除；未知 opcode 保守视为
/// 写 A 槽（判据从紧）。
fn insn_writes_reg(op: LuauOpcode, a: u8, b: u8, reg: u8) -> bool {
  match op {
    LuauOpcode::LopMove
    | LuauOpcode::LopLoadn
    | LuauOpcode::LopLoadb
    | LuauOpcode::LopLoadk
    | LuauOpcode::LopLoadkx
    | LuauOpcode::LopAdd
    | LuauOpcode::LopSub
    | LuauOpcode::LopMul
    | LuauOpcode::LopDiv
    | LuauOpcode::LopIdiv
    | LuauOpcode::LopMod
    | LuauOpcode::LopPow
    | LuauOpcode::LopAddk
    | LuauOpcode::LopSubk
    | LuauOpcode::LopMulk
    | LuauOpcode::LopDivk
    | LuauOpcode::LopIdivk
    | LuauOpcode::LopModk
    | LuauOpcode::LopPowk
    | LuauOpcode::LopSubrk
    | LuauOpcode::LopDivrk
    | LuauOpcode::LopNot
    | LuauOpcode::LopMinus
    | LuauOpcode::LopLength
    | LuauOpcode::LopNewtable
    | LuauOpcode::LopDuptable
    | LuauOpcode::LopGettable
    | LuauOpcode::LopGettableks
    | LuauOpcode::LopGettablen
    | LuauOpcode::LopGetglobal
    | LuauOpcode::LopGetupval
    | LuauOpcode::LopGetimport
    | LuauOpcode::LopNamecall
    | LuauOpcode::LopNamecalludata
    | LuauOpcode::LopConcat
    | LuauOpcode::LopCall
    | LuauOpcode::LopCallfb
    | LuauOpcode::LopAnd
    | LuauOpcode::LopAndk
    | LuauOpcode::LopOr
    | LuauOpcode::LopOrk => reg == a,
    LuauOpcode::LopLoadnil => reg >= a && reg <= a.saturating_add(b),
    LuauOpcode::LopFornprep
    | LuauOpcode::LopFornloop
    | LuauOpcode::LopForgprep
    | LuauOpcode::LopForgloop
    | LuauOpcode::LopForgprepNext
    | LuauOpcode::LopForgprepInext => reg >= a && reg <= a.saturating_add(2),
    LuauOpcode::LopPrepvarargs | LuauOpcode::LopGetvarargs => reg >= a,
    LuauOpcode::LopFastcall
    | LuauOpcode::LopFastcall1
    | LuauOpcode::LopFastcall2
    | LuauOpcode::LopFastcall2k
    | LuauOpcode::LopFastcall3 => reg >= a,
    // 显式非写族：跳转/比较跳/表存/RETURN/杂项
    LuauOpcode::LopNop
    | LuauOpcode::LopJump
    | LuauOpcode::LopJumpif
    | LuauOpcode::LopJumpifnot
    | LuauOpcode::LopJumpiflt
    | LuauOpcode::LopJumpifle
    | LuauOpcode::LopJumpifnotlt
    | LuauOpcode::LopJumpifnotle
    | LuauOpcode::LopJumpifeq
    | LuauOpcode::LopJumpifnoteq
    | LuauOpcode::LopJumpx
    | LuauOpcode::LopJumpxeqknil
    | LuauOpcode::LopJumpxeqkb
    | LuauOpcode::LopJumpxeqkn
    | LuauOpcode::LopJumpxeqks
    | LuauOpcode::LopReturn
    | LuauOpcode::LopSettable
    | LuauOpcode::LopSettableks
    | LuauOpcode::LopSettablen
    | LuauOpcode::LopSetupval
    | LuauOpcode::LopSetglobal
    | LuauOpcode::LopSetlist
    | LuauOpcode::LopCloseupvals
    | LuauOpcode::LopCoverage
    | LuauOpcode::LopCapture => false,
    // 未知 opcode：保守视为写 A 槽
    _ => reg == a,
  }
}

/// 由 callee 槽寄存器的唯一定义链解出编译期 proto 指针。
fn resolve_callee_proto(
  build: &IrBuilder,
  caller_code: &[Instruction],
  reg: u8,
  call_pc: i32,
) -> Option<*mut Proto> {
  let mut cur = reg;
  for depth in 0..=K_MAX_DEF_CHAIN {
    let exclude = if depth == 0 {
      Some(call_pc as usize)
    } else {
      None
    };
    let def = single_def_reg(caller_code, cur, exclude)?;
    match def {
      RegDef::Move(src) => cur = src,
      RegDef::NewClosure(d) => {
        let proto_view = build.function.proto_view();
        let sizep = with_proto(proto_view, |p| p.sizep as u32)?;
        CODEGEN_ASSERT!(d < sizep);
        return child_proto(proto_view, d);
      }
      RegDef::DupClosure(d) => {
        // 模板闭包常量 k[d]：proto 编译期可读（同 env 时运行期还复用同一闭包对象）
        let proto_ptr = with_constant_value(build.function.proto_view(), d, |tv| {
          if tv.is_function() {
            // Safety: `tt == Function` 时活跃臂为 `Closure` 指针（VM 常量表构造契约）；
            // 常量闭包由编译器产出，必为 Lua 闭包（`is_c == 0`）。
            let cl: *mut Closure = unsafe { tv.as_closure_ptr() };
            unsafe {
              if (*cl).is_c == 0 {
                (*cl).inner.l.p
              } else {
                null_mut()
              }
            }
          } else {
            null_mut()
          }
        })?;
        if proto_ptr.is_null() {
          return None;
        }
        return Some(proto_ptr);
      }
    }
  }
  None
}

/// callee 字节码体判据：形态受限 + 栈深合并 + 逐 RETURN 个数对齐。
/// `enforce_maxstack`：静态路径（无守卫）要求映射区落在 caller 已预留栈内；
/// 观测路径由发射守卫 CheckStackRoom 运行时承接（失败落常规 CALL 自带扩栈），
/// 判据侧不再静态排除。
fn check_callee_bytecode(
  build: &IrBuilder,
  callee: *mut Proto,
  nparams: i32,
  nresults: i32,
  ra: u8,
  enforce_maxstack: bool,
) -> Option<()> {
  let callee_ref = child_proto_ref(callee)?;
  // 非变参、实参数逐位对齐（缺失实参的 nil 填充语义不承接）
  if callee_ref.is_vararg != 0 || callee_ref.numparams as i32 != nparams {
    return None;
  }

  let body = code(callee_ref);
  if body.len() > K_MAX_INLINE_INSNS {
    return None;
  }

  // 栈深合并：映射区必须落在 caller 已预留栈内
  let caller_maxstack = with_proto(build.function.proto_view(), |p| p.maxstacksize as usize)?;
  if enforce_maxstack && ra as usize + callee_ref.maxstacksize as usize > caller_maxstack {
    return None;
  }

  // 体白名单 + 回边禁令 + 逐 RETURN 个数对齐（允许多 RETURN 点）
  let mut return_count = 0;
  // 迭代按指令长度步进：GETTABLEKS/NAMECALL 等 2 字指令的 aux 字不是指令，
  // 逐字迭代会把 aux 误判 opcode（阶段 1 白名单全单字指令故未暴露）。
  let mut pc = 0usize;
  while pc < body.len() {
    let insn = body[pc];
    let op = LuauOpcode::from(luau_insn_op(insn) as u8);
    let op_len = get_op_length(op) as usize;
    if op_len == 0 || pc + op_len > body.len() {
      return None;
    }

    // 跳转语义指令：目标必须体内部且前向（禁止循环）
    let target = get_jump_target(insn, pc as u32);
    if target >= 0 && (target <= pc as i32 || target as usize >= body.len()) {
      return None;
    }

    if op == LuauOpcode::LopReturn {
      let b = luau_insn_b(insn) as i32 - 1;
      if b != nresults {
        return None;
      }
      return_count += 1;
      pc += op_len;
      continue;
    }
    if !body_insn_allowed(op) {
      return None;
    }
    pc += op_len;
  }

  if return_count == 0 {
    return None;
  }
  Some(())
}

/// 体白名单字节码集（RETURN 由调用方按 nresults 单独对齐，不含在内）。
fn body_insn_allowed(op: LuauOpcode) -> bool {
  matches!(
    op,
    LuauOpcode::LopNop
      | LuauOpcode::LopMove
      | LuauOpcode::LopLoadnil
      | LuauOpcode::LopLoadb
      | LuauOpcode::LopLoadn
      | LuauOpcode::LopLoadk
      | LuauOpcode::LopLoadkx
      | LuauOpcode::LopAdd
      | LuauOpcode::LopSub
      | LuauOpcode::LopMul
      | LuauOpcode::LopDiv
      | LuauOpcode::LopIdiv
      | LuauOpcode::LopMod
      | LuauOpcode::LopPow
      | LuauOpcode::LopAddk
      | LuauOpcode::LopSubk
      | LuauOpcode::LopMulk
      | LuauOpcode::LopDivk
      | LuauOpcode::LopIdivk
      | LuauOpcode::LopModk
      | LuauOpcode::LopPowk
      | LuauOpcode::LopSubrk
      | LuauOpcode::LopDivrk
      | LuauOpcode::LopNot
      | LuauOpcode::LopMinus
      | LuauOpcode::LopJump
      | LuauOpcode::LopJumpif
      | LuauOpcode::LopJumpifnot
      | LuauOpcode::LopJumpiflt
      | LuauOpcode::LopJumpifle
      | LuauOpcode::LopJumpifnotlt
      | LuauOpcode::LopJumpifnotle
      | LuauOpcode::LopJumpifeq
      | LuauOpcode::LopJumpifnoteq
      | LuauOpcode::LopJumpx
      | LuauOpcode::LopJumpxeqknil
      | LuauOpcode::LopJumpxeqkb
      | LuauOpcode::LopJumpxeqkn
      | LuauOpcode::LopJumpxeqks
      // 刀 C：表读族——快路（数组/哈希命中）保留，形状/类型不符的慢路出口已
      // 由 Fallback 块折叠收敛为出口块（解释器自 CALL 原位重放）
      | LuauOpcode::LopGettableks
      | LuauOpcode::LopGettablen
      // NAMECALL 阶段：字符串连接（IR 面为 helper 调用 + CheckGc，见 K_ALLOWED 注）
      | LuauOpcode::LopConcat
  )
}

/// 试探翻译产物否决扫描：只认纯计算/控制命令白名单；任何（非 Fallback）空块、
/// 慢路命令、不可物化的 `VmConst`/`VmUpvalue` 操作数即整体放弃内联。`VmExit`
/// 出口放行任意下标——emit 侧统一重写为 `call_pc`（Fallback 折叠产出的
/// `call_pc` 出口与 callee 原生 `VmExit(callee_pc)` 出口同款重写，重放点收敛
/// 到 CALL 原位）。
fn inline_ir_veto(f: &IrFunction, callee: *mut Proto) -> Option<&'static str> {
  use IrCmd as C;
  // Fallback 块已由 fold_fallback_blocks 清空引用，不可达：整块（含其指令区间）
  // 豁免扫描——块内慢路命令 FallbackGettableks 等本就不发射
  let mut fallback_ranges: Vec<(u32, u32)> = Vec::new();
  for b in &f.blocks {
    if b.start == u32::MAX {
      return Some("empty-block");
    }
    if b.kind == IrBlockKind::Fallback {
      fallback_ranges.push((b.start, b.finish));
    }
  }
  let in_fallback = |idx: u32| {
    fallback_ranges
      .iter()
      .any(|(start, finish)| idx >= *start && idx <= *finish)
  };
  const K_ALLOWED: &[IrCmd] = &[
    C::NOP,
    C::LoadTag,
    C::LoadPointer,
    C::LoadDouble,
    C::LoadInt,
    C::LoadInt64,
    C::LoadFloat,
    C::LoadTvalue,
    C::StoreTag,
    C::StoreExtra,
    C::StorePointer,
    C::StoreDouble,
    C::StoreInt,
    C::StoreInt64,
    C::StoreTvalue,
    C::AddInt,
    C::SubInt,
    C::AddNum,
    C::SubNum,
    C::MulNum,
    C::DivNum,
    C::IdivNum,
    C::ModNum,
    C::MuladdNum,
    C::UnmNum,
    C::MinNum,
    C::MaxNum,
    C::FloorNum,
    C::CeilNum,
    C::RoundNum,
    C::SqrtNum,
    C::AbsNum,
    C::SignNum,
    C::SelectNum,
    C::IntToNum,
    C::NumToInt,
    C::NumToFloat,
    C::FloatToNum,
    C::NotAny,
    C::CmpAny,
    C::CmpInt,
    C::CmpTag,
    C::CmpSplitTvalue,
    C::JUMP,
    C::JumpIfTruthy,
    C::JumpIfFalsy,
    C::JumpEqTag,
    C::JumpCmpInt,
    C::JumpCmpNum,
    C::JumpCmpFloat,
    C::CheckTag,
    C::CheckTruthy,
    C::CheckCmpNum,
    C::CheckCmpInt,
    // 刀 C 扩展：泛型算术（类型侧未定的参数算术只产 helper 形态）——helper 自带
    // metamethod 保护，前置 SetSavedpc 已重写为调用点（见 emit_inline）；无独立
    // 出口块，内联安全。
    C::DoArith,
    C::SetSavedpc,
    C::INTERRUPT,
    C::RETURN,
    // NAMECALL 阶段扩展：GETTABLEKS 读侧快路——`GetSlotNodeAddr`（IC 提示槽）由
    // emit 重写为 `GetHashNodeAddr`（编译期 hash 主位探测，见模块注）；slot 不
    // 匹配的 `CheckSlotMatch` 失败臂已折叠为 `VmExit(call_pc)`。
    C::GetSlotNodeAddr,
    C::CheckSlotMatch,
    // NAMECALL 阶段扩展：CONCAT——helper 调用自带 metamethod 保护（前置
    // SetSavedpc 已重写为调用点，DoArith 同款）；CheckGc 为独立 GC 步进检查
    C::CONCAT,
    C::CheckGc,
  ];
  for (idx, inst) in f.instructions.iter().enumerate() {
    if in_fallback(idx as u32) {
      continue;
    }
    if is_pseudo(inst.cmd) {
      return Some("pseudo");
    }
    if !K_ALLOWED.contains(&inst.cmd) {
      return Some("cmd");
    }
    for op in inst.ops.iter() {
      match op.kind() {
        IrOpKind::VmExit => {}
        IrOpKind::VmConst => {
          // number 逐值直生、string 重定位进 caller 常量表（发射前物化，见
          // relocate_string_consts）；其余类型不可物化
          let materializable = with_constant_value(child_proto_ref(callee), op.index(), |tv| {
            tv.tt == LuaType::Number as i32 || tv.tt == LuaType::String as i32
          })
          .unwrap_or(false);
          if !materializable {
            return Some("vm-const-or-upval");
          }
        }
        IrOpKind::VmUpvalue => return Some("vm-const-or-upval"),
        _ => {}
      }
    }
  }
  None
}

/// 内联发射：把 trial IR 机械改写后追加进 caller。
///
/// 改写规则：`Inst`/`Block` 索引平移；`VmReg k → VmReg(ra+k)`；`Constant`
/// 逐值重 intern（`INTERRUPT` 的 pcpos 常量重写为调用点）；`VmConst` number
/// 逐值直生、string 查 `string_map` 重定位（caller 常量表下标，见
/// relocate_string_consts）；`GetSlotNodeAddr` 重写为 `GetHashNodeAddr`（IC
/// 提示槽寻址依赖 R_CODE，内联后错位，见模块注）；每条 `RETURN` 替换为「返回
/// 值下移拷贝 + JUMP 后继块」（多 RETURN 点各自折叠）；`VmExit` 出口统一重写
/// 为 `call_pc`。
/// `guard_funid` 非空（观测路径）时入口前置 proto 守卫：`JumpEqTag`(Function)
/// + `JumpCmpProtoid`(funid)，失败落常规 `SetSavedpc`+`CALL` 回退块。
#[allow(clippy::too_many_arguments)]
fn emit_inline(
  build: &mut IrBuilder,
  trial: &mut IrFunction,
  callee: *mut Proto,
  ra: u8,
  nparams: i32,
  nresults: i32,
  call_pc: i32,
  guard: Option<(u32, i32)>,
  string_map: &[(u32, u32)],
) {
  // 慢路出口块：句柄先建（fold 与守卫失败臂都以它为改写/跳转目标），内容在
  // guard 分发后统一发射。出口语义 = 常规 CALL 重放（SetSavedpc+CALL+JUMP 后继，
  // 与未内联发射逐位一致）——不用裸 `VmExit`：该形态缺 ExitSync 值同步（VM 存活
  // 值物化回栈的 reg_stores 只为构建期登记的出口建立），内联体出口落到它会把
  // 寄存器驻留值丢在生成码里，解释器重放读到栈上旧值。
  let exit_blk = build.push_block(IrBlockKind::Internal, call_pc as u32);
  let fallback_set = collect_fallback_blocks(trial);

  // trial→caller 指令映射：Fallback 折叠与 RETURN 替换打破「1:1 克隆」的
  // 指令序号对应（未发射的 Fallback 块指令在 trial 索引域留洞），Inst 操作数
  // 平移必须查表，不能再用 inst_base+idx 的基数假设。
  let mut inst_map: Vec<Option<u32>> = vec![None; trial.instructions.len()];
  let block_base = build.function.blocks.len() as u32;

  // 1:1 克隆 trial 块，全部降为 Internal（非 caller 字节码块）；Fallback 块
  // 已折叠无引用，壳照克隆以保持索引对齐，内容不发射
  for _ in 0..trial.blocks.len() {
    build.function.blocks.push(IrBlock {
      kind: IrBlockKind::Internal,
      flags: 0,
      use_count: 0,
      start: u32::MAX,
      finish: u32::MAX,
      sortkey: 0,
      chainkey: 0,
      expected_next_block: u32::MAX,
      startpc: call_pc as u32,
      label: Default::default(),
    });
  }

  // 后继块：CALL 之后若无既存跳转目标块，则为残尾新建 Internal 块承接
  let next_pc = (call_pc + get_op_length(LuauOpcode::LopCall)) as u32;
  CODEGEN_ASSERT!((next_pc as usize) < build.inst_index_to_block.len());
  let continuation = if build.inst_index_to_block[next_pc as usize] != u32::MAX {
    build.block_at_inst(next_pc)
  } else {
    build.push_block(IrBlockKind::Internal, next_pc)
  };

  let entry_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_base + trial.entry_block);
  match guard {
    None => {
      build.inst_ir_cmd_ir_op(IrCmd::JUMP, entry_op);
      add_use(&mut build.function, entry_op);
    }
    Some((funid, needed_slots)) => {
      // proto 守卫：tag==Function 且 closure->p->funid==观测值、栈余量覆盖 callee
      // 工作槽（CheckStackRoom 失败落常规 CALL，其 call_prolog 自带扩栈），才进内联体
      let check_blk = build.push_block(IrBlockKind::Internal, call_pc as u32);
      let check_room = build.push_block(IrBlockKind::Internal, call_pc as u32);
      let ra_tag = build.vm_reg(ra);
      let tag_load = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, ra_tag);
      let const_fn = build.const_tag(LuaType::Function as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        tag_load,
        const_fn,
        check_blk,
        exit_blk,
      );
      add_use(&mut build.function, tag_load);
      add_use(&mut build.function, check_blk);
      add_use(&mut build.function, exit_blk);
      build.begin_block(check_blk);
      let ra_ptr = build.vm_reg(ra);
      let ccl = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, ra_ptr);
      let const_funid = build.const_uint(funid);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpProtoid,
        ccl,
        const_funid,
        check_room,
        exit_blk,
      );
      add_use(&mut build.function, ccl);
      add_use(&mut build.function, check_room);
      add_use(&mut build.function, exit_blk);
      build.begin_block(check_room);
      let needed = build.const_int(needed_slots);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckStackRoom, needed, exit_blk);
      add_use(&mut build.function, needed);
      add_use(&mut build.function, exit_blk);
      // CheckStackRoom 通过：显式跳内联体入口（guard 非终结符，物理落穿目标
      // 取决于块布局，不可依赖）
      build.inst_ir_cmd_ir_op(IrCmd::JUMP, entry_op);
      add_use(&mut build.function, entry_op);
    }
  }

  // 慢路出口块内容：常规 CALL 发射（语义与未内联逐位一致；__call 元方法、
  // 参数补 nil、native/explainer 分派全部由 call_prolog/CALL lowering 承接）
  build.begin_block(exit_blk);
  // savedpc 公式与 ir_builder 常规 CALL 臂一致（CALL 长 1，CALLFB 才有 aux）
  let savedpc = if fflag::LuauCallFeedback.get() {
    call_pc + 2
  } else {
    call_pc + 1
  };
  let savedpc_op = build.const_uint(savedpc as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_op);
  {
    let ra_op = build.vm_reg(ra);
    let b_op = build.const_int(nparams);
    let c_op = build.const_int(nresults);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CALL, ra_op, b_op, c_op);
    for op_ref in [ra_op, b_op, c_op] {
      add_use(&mut build.function, op_ref);
    }
  }
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, continuation);
  add_use(&mut build.function, continuation);

  for (bi, b) in trial.blocks.iter().enumerate() {
    // Fallback 块已折叠（引用全部改写为 VmExit 出口），不可达不发射
    if b.kind == IrBlockKind::Fallback || b.start == u32::MAX || b.finish == u32::MAX {
      continue;
    }
    let block_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_base + bi as u32);
    build.begin_block(block_op);

    for idx in b.start..=b.finish {
      let inst = &trial.instructions[idx as usize];

      // RETURN 折叠：返回值下移拷贝后跳后继块（多 RETURN 点各自收口）
      if inst.cmd == IrCmd::RETURN {
        let src_a = vm_reg_index(&inst.ops[0]);
        for k in 0..nresults {
          // src：callee reg(src_a+k) 物理 = ra+1+src_a+k（帧映射见 rewrite_op）；
          // dst：CALL 语义返回值落在 ra..ra+nresults-1（覆盖函数值槽与实参区）。
          let src = build.vm_reg((ra as i32 + 1 + src_a + k) as u8);
          let dst = build.vm_reg((ra as i32 + k) as u8);
          let value = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, src);
          build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, dst, value);
        }
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, continuation);
        // 手工发射面自行记账 use（inst_ir_cmd 系列不自动维护，块 use 亦然）
        add_use(&mut build.function, continuation);
        continue;
      }

      // GETTABLEKS 的 IC 提示槽寻址重写：GetSlotNodeAddr 运行时读
      // `R_CODE[pcpos].C`（cached slot），内联体在 caller 上下文执行时 R_CODE 是
      // caller 的 code——callee pcpos 错位。换主位探测 GetHashNodeAddr（编译期
      // `ts->hash` 立即数，NAMECALL 第一跳同款）：主位 miss 落 VmExit 由解释器
      // 重放 GETTABLEKS 慢路承接（碰撞链/元表行走 + patch_c 回填），语义闭合。
      if inst.cmd == IrCmd::GetSlotNodeAddr {
        let table = rewrite_op(build, trial, &inst.ops[0], ra, &inst_map, block_base);
        // hash 读 callee k[aux] 的字符串（Safety：callee 存活性契约同试探翻译，
        // 见 try_translate_call_inline 的 build_function_ir 注）
        let hash = string_constant_hash(unsafe { &*callee }, inst.ops[2].index() as usize);
        let hash_op = build.const_uint(hash);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddr, table, hash_op);
        inst_map[idx as usize] = Some(build.function.instructions.len() as u32 - 1);
        add_use(&mut build.function, table);
        add_use(&mut build.function, hash_op);
        continue;
      }

      let mut ops: IrOps = SmallVector::new();
      for op in inst.ops.iter() {
        let rewritten = if inst.cmd == IrCmd::INTERRUPT {
          // INTERRUPT 操作数即 pcpos：重写为调用点，中断后解释器自 CALL
          // 原位重放整次调用（与其余慢路出口同语义）
          build.const_uint(call_pc as u32)
        } else if inst.cmd == IrCmd::SetSavedpc {
          // 慢路保护点（泛型算术/CONCAT helper 前置）：同 INTERRUPT 语义，pcpos
          // 重写为调用点下沿（常规 CALL 臂的 savedpc 公式），解释器重放从 CALL
          // 原位续
          build.const_uint((call_pc + 1) as u32)
        } else if op.kind() == IrOpKind::VmConst {
          // 常量物化（veto 已判定可物化）：number——callee k[aux] 的 f64 值重
          // intern 进 caller 常量池，操作数落 LoadDouble 指令；string——查映射
          // 重定位为 caller 常量表下标（R_CONSTANTS 基址寻址，见模块注）
          let is_str =
            with_constant_value(child_proto_ref(callee), op.index(), |tv| tv.is_string())
              .unwrap_or(false);
          if is_str {
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmConst, relocated_index(string_map, op.index()))
          } else {
            let value = with_constant_value(child_proto_ref(callee), op.index(), |tv| {
              constant_number(tv)
            })
            .unwrap_or_default();
            build.const_double(value)
          }
        } else if op.kind() == IrOpKind::VmExit {
          // callee 原生出口（类型侧 CheckTag 失败臂等）：同慢路出口收敛，落常规
          // CALL 重放块（裸 VmExit 缺 ExitSync 值同步，见 exit_blk 注）
          exit_blk
        } else if op.kind() == IrOpKind::Block && fallback_set.binary_search(&op.index()).is_ok() {
          // trial 的 Fallback 块引用（慢路出口臂）：改写为出口块。须在 rewrite_op
          // 的 Block 平移前特判——平移后的壳号与本改写写进同一 op 会撞号
          exit_blk
        } else {
          rewrite_op(build, trial, op, ra, &inst_map, block_base)
        };
        ops.push(rewritten);
      }
      build.inst_ir_cmd_ir_ops(inst.cmd, &ops);
      inst_map[idx as usize] = Some(build.function.instructions.len() as u32 - 1);
      for op_ref in ops.as_slice() {
        add_use(&mut build.function, *op_ref);
      }
    }
  }

  // 残尾承接块开启：后续 caller 指令继续落入（若有）
  if build.inst_index_to_block[next_pc as usize] == u32::MAX {
    build.begin_block(continuation);
  }
}

/// 单操作数机械改写。`inst_map` 为 trial→caller 指令映射（Fallback 折叠与
/// RETURN 替换造成的非 1:1 序号，见 emit_inline）。
fn rewrite_op(
  build: &mut IrBuilder,
  trial: &IrFunction,
  op: &IrOp,
  ra: u8,
  inst_map: &[Option<u32>],
  block_base: u32,
) -> IrOp {
  match op.kind() {
    IrOpKind::None | IrOpKind::Undef | IrOpKind::Condition => *op,
    IrOpKind::Inst => {
      let mapped = inst_map[op.index() as usize].unwrap_or_else(|| {
        CODEGEN_ASSERT!(false, "inline rewrite: unmapped trial inst");
        u32::MAX
      });
      IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, mapped)
    }
    IrOpKind::Block => IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_base + op.index()),
    IrOpKind::VmReg => {
      // 真实 CALL 帧布局 base = ra+1（call_prolog: ci->base = ra+1），callee reg k
      // 物理 = base+k = ra+1+k；内联体逐位复刻该布局，参数读 ra+1..ra+nparams 与
      // 实参槽对位（阶段 1 的 ra+k 映射在带参 callee 下读函数值槽，系错位缺陷）。
      let mapped = ra as i32 + 1 + vm_reg_index(op);
      CODEGEN_ASSERT!(mapped <= u8::MAX as i32);
      IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, mapped as u32)
    }
    IrOpKind::Constant => materialize_const(build, &trial.constants, op),
    // 慢路出口已在 emit_inline 的 rewrite 链统一落出口块；到此即编译器内部
    // 不变量被破坏
    IrOpKind::VmExit => {
      CODEGEN_ASSERT!(false, "inline rewrite: vm-exit op leaked");
      *op
    }
    // 试探否决已排除；到此即编译器内部不变量被破坏
    IrOpKind::VmConst | IrOpKind::VmUpvalue => {
      CODEGEN_ASSERT!(false, "inline rewrite: vetoed op kind leaked");
      *op
    }
  }
}

/// trial 常量操作数逐值重 intern 进 caller 常量池。
fn materialize_const(build: &mut IrBuilder, constants: &[IrConst], op: &IrOp) -> IrOp {
  match &constants[op.index() as usize] {
    IrConst::Int(v) => build.const_int(*v),
    IrConst::Int64(v) => build.const_int_64(*v),
    IrConst::Uint(v) => build.const_uint(*v),
    IrConst::Double(v) => build.const_double(*v),
    IrConst::Tag(v) => build.const_tag(*v),
    // GETIMPORT 载体，白名单字节码不可能产出
    IrConst::Import(_) => {
      CODEGEN_ASSERT!(false, "inline rewrite: Import const leaked");
      *op
    }
  }
}

/// 读 `VmReg` 操作数的寄存器槽号。
fn vm_reg_index(op: &IrOp) -> i32 {
  CODEGEN_ASSERT!(op.kind() == IrOpKind::VmReg);
  op.index() as i32
}
