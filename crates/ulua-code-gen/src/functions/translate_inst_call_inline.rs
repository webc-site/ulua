//! JIT 用户函数 call inlining（第 1 阶段最小可证子集：直通内联）。
//!
//! 判据（翻译期静态可判，任一不满足即整体放弃、走常规 CALL 发射）：
//! 1. callee 槽寄存器在本 proto 字节码内有唯一静态定义点，定义链仅由
//!    `NEWCLOSURE`/`DUPCLOSURE`（可经 ≤2 跳 `MOVE`）到达——callee proto
//!    编译期可辨；`GETUPVAL`/`GETTABLEKS`/`NAMECALL` 等运行时形态一律不做。
//! 2. callee 非变参、`numparams == 实参数`、指令数 ≤ 100、全函数仅末尾一条
//!    `RETURN` 且返回个数 == caller 期望、无回边（禁止循环体）。
//! 3. 体指令全部落在白名单（纯数据搬运/常量加载/数值算术/跳转）；试探翻译
//!    产物中不含 Fallback 块、慢路命令、`VmExit`/`VmConst`/`VmUpvalue`
//!    操作数——零慢路面，错误路径不可触达。
//! 4. 栈深：`ra + callee.maxstacksize ≤ caller.maxstacksize`。体寄存器按
//!    `callee reg k ↔ caller reg ra+k` 直通映射，与真实帧布局逐位一致；
//!    CALL 语义本就允许覆写 `ra+1..` 槽位，内联不扩大覆写面。
//!
//! 语义面：内联体零分配、零屏障、零慢路（GC barrier/safepoint 分布不变）；
//! 体 `INTERRUPT` 的 pcpos 重写为调用点，中断后解释器自 CALL 原位重放整次
//! 调用，与未内联路径逐值一致。

use core::ptr::null_mut;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_d, luau_insn_op},
  records::small_vector::SmallVector,
};
use ulua_vm::records::{closure::Closure, proto::Proto};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    ir::is_pseudo,
    proto_view::{child_proto, child_proto_ref, with_constant_value, with_proto},
    proto_views::code,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_const::IrConst, ir_function::IrFunction,
    ir_op::IrOp,
  },
  type_aliases::ir::{Instruction, IrOps},
};

/// callee 指令数上限（第 1 阶段保守线）
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

/// 尝试把 `CALL` 站点内联展开进 caller IR；返回是否已内联（true 时调用方
/// 跳过常规 `SetSavedpc`+`CALL` 发射）。
pub(crate) fn try_translate_call_inline(
  build: &mut IrBuilder,
  caller_code: &[Instruction],
  i: i32,
  b_raw: u8,
  c_raw: u8,
) -> bool {
  // FASTCALL fallback 区内不做（第 1 阶段保守）；multret 实参/期望返回不做
  if build.active_fastcall_fallback || b_raw == 0 || c_raw == 0 {
    return false;
  }

  let nparams = b_raw as i32 - 1;
  let nresults = c_raw as i32 - 1;
  let ra = luau_insn_a(caller_code[i as usize]) as u8;

  let Some(callee) = resolve_callee_proto(build, caller_code, ra, i) else {
    return false;
  };
  if check_callee_bytecode(build, callee, nparams, nresults, ra).is_none() {
    return false;
  }

  let Some(callee_ref) = child_proto_ref(callee) else {
    return false;
  };
  let callee_insns = callee_ref.sizecode as usize;

  // 试探翻译：在独立 IrBuilder 上按既有管线构建 callee IR，任何慢路面即弃
  let hooks = build.host_hooks_ref();
  let mut trial = IrBuilder::ir_builder_ir_builder(hooks);
  // Safety: callee 为 caller proto `p[]` 数组内存活子原型（契约同
  // `translate_inst_new_closure`），codegen 期间 VM 持有、只读。
  unsafe { trial.build_function_ir(callee) };

  let trial_function = trial.function;
  if inline_ir_veto(&trial_function).is_some() {
    return false;
  }
  emit_inline(build, &trial_function, ra, nresults, i);
  // 内联发射证据：编译期单次打印（compile-once，不进热路径）
  eprintln!(
    "[call-inline] hit caller_pc={i} ra={ra} nresults={nresults} callee_insns={callee_insns}"
  );
  true
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
      LuauOpcode::LOP_NEWCLOSURE => RegDef::NewClosure(luau_insn_d(insn) as u32),
      LuauOpcode::LOP_DUPCLOSURE => RegDef::DupClosure(luau_insn_d(insn) as u32),
      LuauOpcode::LOP_MOVE => RegDef::Move(luau_insn_b(insn) as u8),
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
    LuauOpcode::LOP_MOVE
    | LuauOpcode::LOP_LOADN
    | LuauOpcode::LOP_LOADB
    | LuauOpcode::LOP_LOADK
    | LuauOpcode::LOP_LOADKX
    | LuauOpcode::LOP_ADD
    | LuauOpcode::LOP_SUB
    | LuauOpcode::LOP_MUL
    | LuauOpcode::LOP_DIV
    | LuauOpcode::LOP_IDIV
    | LuauOpcode::LOP_MOD
    | LuauOpcode::LOP_POW
    | LuauOpcode::LOP_ADDK
    | LuauOpcode::LOP_SUBK
    | LuauOpcode::LOP_MULK
    | LuauOpcode::LOP_DIVK
    | LuauOpcode::LOP_IDIVK
    | LuauOpcode::LOP_MODK
    | LuauOpcode::LOP_POWK
    | LuauOpcode::LOP_SUBRK
    | LuauOpcode::LOP_DIVRK
    | LuauOpcode::LOP_NOT
    | LuauOpcode::LOP_MINUS
    | LuauOpcode::LOP_LENGTH
    | LuauOpcode::LOP_NEWTABLE
    | LuauOpcode::LOP_DUPTABLE
    | LuauOpcode::LOP_GETTABLE
    | LuauOpcode::LOP_GETTABLEKS
    | LuauOpcode::LOP_GETTABLEN
    | LuauOpcode::LOP_GETGLOBAL
    | LuauOpcode::LOP_GETUPVAL
    | LuauOpcode::LOP_GETIMPORT
    | LuauOpcode::LOP_NAMECALL
    | LuauOpcode::LOP_NAMECALLUDATA
    | LuauOpcode::LOP_CONCAT
    | LuauOpcode::LOP_CALL
    | LuauOpcode::LOP_CALLFB
    | LuauOpcode::LOP_AND
    | LuauOpcode::LOP_ANDK
    | LuauOpcode::LOP_OR
    | LuauOpcode::LOP_ORK => reg == a,
    LuauOpcode::LOP_LOADNIL => reg >= a && reg <= a.saturating_add(b),
    LuauOpcode::LOP_FORNPREP
    | LuauOpcode::LOP_FORNLOOP
    | LuauOpcode::LOP_FORGPREP
    | LuauOpcode::LOP_FORGLOOP
    | LuauOpcode::LOP_FORGPREP_NEXT
    | LuauOpcode::LOP_FORGPREP_INEXT => reg >= a && reg <= a.saturating_add(2),
    LuauOpcode::LOP_PREPVARARGS | LuauOpcode::LOP_GETVARARGS => reg >= a,
    LuauOpcode::LOP_FASTCALL
    | LuauOpcode::LOP_FASTCALL1
    | LuauOpcode::LOP_FASTCALL2
    | LuauOpcode::LOP_FASTCALL2K
    | LuauOpcode::LOP_FASTCALL3 => reg >= a,
    // 显式非写族：跳转/比较跳/表存/RETURN/杂项
    LuauOpcode::LOP_NOP
    | LuauOpcode::LOP_JUMP
    | LuauOpcode::LOP_JUMPIF
    | LuauOpcode::LOP_JUMPIFNOT
    | LuauOpcode::LOP_JUMPIFLT
    | LuauOpcode::LOP_JUMPIFLE
    | LuauOpcode::LOP_JUMPIFNOTLT
    | LuauOpcode::LOP_JUMPIFNOTLE
    | LuauOpcode::LOP_JUMPIFEQ
    | LuauOpcode::LOP_JUMPIFNOTEQ
    | LuauOpcode::LOP_JUMPX
    | LuauOpcode::LOP_JUMPXEQKNIL
    | LuauOpcode::LOP_JUMPXEQKB
    | LuauOpcode::LOP_JUMPXEQKN
    | LuauOpcode::LOP_JUMPXEQKS
    | LuauOpcode::LOP_RETURN
    | LuauOpcode::LOP_SETTABLE
    | LuauOpcode::LOP_SETTABLEKS
    | LuauOpcode::LOP_SETTABLEN
    | LuauOpcode::LOP_SETUPVAL
    | LuauOpcode::LOP_SETGLOBAL
    | LuauOpcode::LOP_SETLIST
    | LuauOpcode::LOP_CLOSEUPVALS
    | LuauOpcode::LOP_COVERAGE
    | LuauOpcode::LOP_CAPTURE => false,
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

/// callee 字节码体判据：形态受限 + 栈深合并 + 单返回点。
fn check_callee_bytecode(
  build: &IrBuilder,
  callee: *mut Proto,
  nparams: i32,
  nresults: i32,
  ra: u8,
) -> Option<()> {
  let callee_ref = child_proto_ref(callee)?;
  // 非变参、实参数逐位对齐（缺失实参的 nil 填充语义第 1 阶段不承接）
  if callee_ref.is_vararg != 0 || callee_ref.numparams as i32 != nparams {
    return None;
  }

  let body = code(callee_ref);
  if body.len() > K_MAX_INLINE_INSNS {
    return None;
  }

  // 栈深合并：映射区必须落在 caller 已预留栈内
  let caller_maxstack = with_proto(build.function.proto_view(), |p| p.maxstacksize as usize)?;
  if ra as usize + callee_ref.maxstacksize as usize > caller_maxstack {
    return None;
  }

  // 体白名单 + 回边禁令 + 单返回点
  let mut return_results: Option<i32> = None;
  let mut return_count = 0;
  for (pc, &insn) in body.iter().enumerate() {
    let op = LuauOpcode::from(luau_insn_op(insn) as u8);

    // 跳转语义指令：目标必须体内部且前向（禁止循环）
    let target = get_jump_target(insn, pc as u32);
    if target >= 0 && (target <= pc as i32 || target as usize >= body.len()) {
      return None;
    }

    match op {
      LuauOpcode::LOP_NOP
      | LuauOpcode::LOP_MOVE
      | LuauOpcode::LOP_LOADNIL
      | LuauOpcode::LOP_LOADB
      | LuauOpcode::LOP_LOADN
      | LuauOpcode::LOP_LOADK
      | LuauOpcode::LOP_LOADKX
      | LuauOpcode::LOP_ADD
      | LuauOpcode::LOP_SUB
      | LuauOpcode::LOP_MUL
      | LuauOpcode::LOP_DIV
      | LuauOpcode::LOP_IDIV
      | LuauOpcode::LOP_MOD
      | LuauOpcode::LOP_POW
      | LuauOpcode::LOP_ADDK
      | LuauOpcode::LOP_SUBK
      | LuauOpcode::LOP_MULK
      | LuauOpcode::LOP_DIVK
      | LuauOpcode::LOP_IDIVK
      | LuauOpcode::LOP_MODK
      | LuauOpcode::LOP_POWK
      | LuauOpcode::LOP_SUBRK
      | LuauOpcode::LOP_DIVRK
      | LuauOpcode::LOP_NOT
      | LuauOpcode::LOP_MINUS
      | LuauOpcode::LOP_JUMP
      | LuauOpcode::LOP_JUMPIF
      | LuauOpcode::LOP_JUMPIFNOT
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFNOTLT
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPX
      | LuauOpcode::LOP_JUMPXEQKNIL
      | LuauOpcode::LOP_JUMPXEQKB
      | LuauOpcode::LOP_JUMPXEQKN
      | LuauOpcode::LOP_JUMPXEQKS => {}
      LuauOpcode::LOP_RETURN => {
        return_count += 1;
        if return_count > 1 || pc + 1 != body.len() {
          return None;
        }
        let b = luau_insn_b(insn) as i32 - 1;
        if b != nresults {
          return None;
        }
        return_results = Some(b);
      }
      _ => return None,
    }
  }

  if return_count != 1 || return_results.is_none() {
    return None;
  }
  Some(())
}

/// 试探翻译产物否决扫描：只认纯计算/控制命令白名单；任何 Fallback 块、
/// 慢路命令、`VmExit`/`VmConst`/`VmUpvalue` 操作数即整体放弃内联。
fn inline_ir_veto(f: &IrFunction) -> Option<&'static str> {
  use IrCmd as C;
  for b in &f.blocks {
    if b.kind == IrBlockKind::Fallback {
      return Some("fallback-block");
    }
    if b.start == u32::MAX {
      return Some("empty-block");
    }
  }
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
    C::INTERRUPT,
    C::RETURN,
  ];
  for inst in &f.instructions {
    if is_pseudo(inst.cmd) {
      return Some("pseudo");
    }
    if !K_ALLOWED.contains(&inst.cmd) {
      return Some("cmd");
    }
    for op in inst.ops.iter() {
      match op.kind() {
        IrOpKind::VmExit => return Some("vm-exit"),
        IrOpKind::VmConst | IrOpKind::VmUpvalue => return Some("vm-const-or-upval"),
        _ => {}
      }
    }
  }
  None
}

/// 内联发射：把 trial IR 机械改写后追加进 caller。
///
/// 改写规则：`Inst`/`Block` 索引平移；`VmReg k → VmReg(ra+k)`；`Constant`
/// 逐值重 intern（`INTERRUPT` 的 pcpos 常量重写为调用点）；末尾 `RETURN`
/// 替换为「返回值下移拷贝 + JUMP 后继块」。
fn emit_inline(build: &mut IrBuilder, trial: &IrFunction, ra: u8, nresults: i32, call_pc: i32) {
  let inst_base = build.function.instructions.len() as u32;
  let block_base = build.function.blocks.len() as u32;

  // 1:1 克隆 trial 块，全部降为 Internal（非 caller 字节码块）
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

  // 终结当前块：JUMP 进体入口
  let entry_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_base + trial.entry_block);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, entry_op);

  // 后继块：CALL 之后若无既存跳转目标块，则为残尾新建 Internal 块承接
  let next_pc = (call_pc + get_op_length(LuauOpcode::LOP_CALL)) as u32;
  CODEGEN_ASSERT!((next_pc as usize) < build.inst_index_to_block.len());
  let continuation = if build.inst_index_to_block[next_pc as usize] != u32::MAX {
    build.block_at_inst(next_pc)
  } else {
    build.push_block(IrBlockKind::Internal, next_pc)
  };

  for (bi, b) in trial.blocks.iter().enumerate() {
    if b.start == u32::MAX || b.finish == u32::MAX {
      continue;
    }
    let block_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_base + bi as u32);
    build.begin_block(block_op);

    for idx in b.start..=b.finish {
      let inst = &trial.instructions[idx as usize];

      // 单返回点收口：返回值下移拷贝后跳后继块，替换 RETURN 终结指令
      if inst.cmd == IrCmd::RETURN {
        let src_a = vm_reg_index(&inst.ops[0]);
        for k in 0..nresults {
          let src = build.vm_reg((ra as i32 + src_a + k) as u8);
          let dst = build.vm_reg((ra as i32 + k) as u8);
          let value = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, src);
          build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, dst, value);
        }
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, continuation);
        continue;
      }

      let mut ops: IrOps = SmallVector::new();
      for op in inst.ops.iter() {
        let rewritten = if inst.cmd == IrCmd::INTERRUPT {
          // INTERRUPT 操作数即 pcpos：重写为调用点，中断后解释器自 CALL
          // 原位重放整次调用（与其余 vmexit 出口同语义）
          build.const_uint(call_pc as u32)
        } else {
          rewrite_op(build, trial, op, ra, inst_base, block_base)
        };
        ops.push(rewritten);
      }
      build.inst_ir_cmd_ir_ops(inst.cmd, &ops);
    }
  }

  // 残尾承接块开启：后续 caller 指令继续落入（若有）
  if build.inst_index_to_block[next_pc as usize] == u32::MAX {
    build.begin_block(continuation);
  }
}

/// 单操作数机械改写。
fn rewrite_op(
  build: &mut IrBuilder,
  trial: &IrFunction,
  op: &IrOp,
  ra: u8,
  inst_base: u32,
  block_base: u32,
) -> IrOp {
  match op.kind() {
    IrOpKind::None | IrOpKind::Undef | IrOpKind::Condition => *op,
    IrOpKind::Inst => IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, inst_base + op.index()),
    IrOpKind::Block => IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_base + op.index()),
    IrOpKind::VmReg => {
      let mapped = ra as i32 + vm_reg_index(op);
      CODEGEN_ASSERT!(mapped <= u8::MAX as i32);
      IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, mapped as u32)
    }
    IrOpKind::Constant => materialize_const(build, &trial.constants, op),
    // 试探否决已排除；到此即编译器内部不变量被破坏
    IrOpKind::VmConst | IrOpKind::VmUpvalue | IrOpKind::VmExit => {
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
