//! `BytecodeBuilder` 之 调试期指令/变长/捕获校验链。

use std::{vec, vec::Vec};

use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag::DebugLuauUserDefinedClasses,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call, is_skip_c::is_skip_c,
  },
  macros::luau_assert::{LUAU_ASSERT, LUAU_ASSERTENABLED},
  records::{instruction::Instruction, small_vector::SmallVector},
};

use super::{BytecodeBuilder, K_INVALID_REG, scan};
use crate::{
  functions::decode_import_aux::decode_import_aux,
  macros::v_inst_accessors::{VCONST, VCONSTANY, VJUMP, VREG, VREGRANGE, VUPVAL},
};

// ── abs-r139：并自 `methods/bytecode_builder_validate_captures.rs` ──
/// cpp `std::array<bool, 256>`：寄存器编号是 u8，捕获位图固定 256 槽。
const CAPTURED_REG_COUNT: usize = 256;

/// cpp `validateCaptures` 内的局部 `struct BytecodeBlock`。
struct Block {
  startpc: usize,
  /// cpp 用 `-1` 表示"尚未定界"，这里用 `Option<usize>` 取代魔法数。
  finishpc: Option<usize>,
  captured: [bool; CAPTURED_REG_COUNT],
  visited: bool,
  in_worklist: bool,
  predecessors: SmallVector<u32, 4>,
  successors: SmallVector<u32, 4>,
}

impl Block {
  fn new(startpc: usize) -> Self {
    Self {
      startpc,
      finishpc: None,
      captured: [false; CAPTURED_REG_COUNT],
      visited: false,
      in_worklist: false,
      predecessors: SmallVector::new(),
      successors: SmallVector::new(),
    }
  }
}

impl BytecodeBuilder<'_> {
  /// cpp `validateCaptures()`（`BytecodeBuilder.cpp:2133-2312`）：重建字节码基本块图，
  /// 沿控制流传播「仍被 `CAPTURE REF` 打开的寄存器」位图，断言任何可达的 `RETURN`
  /// 处所有捕获都已被 `CLOSEUPVALS` 关闭。
  ///
  /// `validate_instructions` 里的 `open_captures` 只做线性扫描，看不到
  /// 「一个分支捕获、另一个分支直接 return」这类控制流漏洞，故上游另建这份 CFG 数据流校验。
  pub(crate) fn validate_captures(&self) {
    let insns = self.instructions();
    // cpp 的 `int` 下标在此收敛为 usize；空指令流是退化输入（cpp 会越界读 insns[0]），
    // 编译器不会产出，早退避免 panic。
    if insns.is_empty() {
      return;
    }

    // 1) 标记跳转目标（与 validate_variadic 共用的第一遍，见 mark_jump_targets）。
    let jump_targets = mark_jump_targets(insns);

    // 2) 重建基本块：落在跳转目标上的指令另起一块；终结指令（跳转/RETURN）结束当前块。
    // 下方两处 `blocks.last_mut().unwrap()` 是 100% 安全的：`blocks` 以
    // `vec![Block::new(0)]` 起步、循环内只 push 不 pop，恒非空。
    let mut blocks: Vec<Block> = vec![Block::new(0)];
    let mut previ = 0usize;
    for (pc, end, insn) in scan::stepped(insns, 0) {
      let op = insn.luau_opcode();

      if pc != 0 && jump_targets[pc] {
        blocks.last_mut().unwrap().finishpc = Some(previ);
        blocks.push(Block::new(pc));
      }

      let target = get_jump_target(insn.raw(), pc as u32);
      if (target >= 0 && !is_fast_call(op)) || op == LuauOpcode::LOP_RETURN {
        blocks.last_mut().unwrap().finishpc = Some(pc);

        // 没有显式跳转承接 fallthrough 时，为后继指令新开一块
        if end < insns.len() && !jump_targets[end] {
          blocks.push(Block::new(end));
        }
      }

      previ = pc;
    }
    if let Some(last) = blocks.last_mut()
      && last.finishpc.is_none()
    {
      last.finishpc = Some(previ);
    }

    // 3) 指令 → 所属块
    let mut owner_block_idx = vec![u32::MAX; insns.len()];
    for (block_idx, block) in blocks.iter().enumerate() {
      let Some(finishpc) = block.finishpc else {
        continue;
      };
      for (pc, _end, _insn) in scan::stepped(insns, block.startpc) {
        if pc > finishpc {
          break;
        }
        owner_block_idx[pc] = block_idx as u32;
      }
    }

    // 4) 收集前驱/后继
    // block_idx 是块编号：它会被原样写进 `successors`/`predecessors` 边表（u32 数据），
    // 且循环内既要可变改 `blocks[block_idx]`、又要 `blocks.get_mut(target_idx)`，
    // 两个槽位的借用无法用一次 iter_mut 表达，故保留下标遍历。
    for block_idx in 0..blocks.len() {
      // cpp 把循环变量与 `insn/op` 声明在块循环体内、指令循环体外，好让块尾的
      // fallthrough 复用终结指令的取值（未定界的块因此读到初值 `LOP_NOP`）；此处同样外提。
      // 未定界的块不扫指令，但 fallthrough 边仍按 cpp 语义尝试连接。
      let mut pc = blocks[block_idx].startpc;
      let mut insn = Instruction(0);
      let mut op = LuauOpcode::LOP_NOP;
      let finishpc = blocks[block_idx].finishpc;

      if let Some(finishpc) = finishpc {
        for (i, end, next_insn) in scan::stepped(insns, pc) {
          if i > finishpc {
            break;
          }
          insn = next_insn;
          op = insn.luau_opcode();

          let target = get_jump_target(insn.raw(), i as u32);
          if target >= 0 && !is_fast_call(op) {
            // 非法跳转目标可能越界或落在未定界的字节上：cpp 靠 LUAU_ASSERT 兜底，
            // Rust 侧断言后跳过该边，不能让 panic 或越界下标改变校验结论。
            let target_idx = usize::try_from(target)
              .ok()
              .and_then(|t| owner_block_idx.get(t).copied())
              .unwrap_or_else(|| {
                LUAU_ASSERT!(false, "jump target must be owned by a block");
                u32::MAX
              });
            LUAU_ASSERT!(target_idx != u32::MAX);

            if target_idx != u32::MAX {
              blocks[block_idx].successors.push(target_idx);

              if let Some(pred_block) = blocks.get_mut(target_idx as usize) {
                pred_block.predecessors.push(block_idx as u32);
              }
            }
          }

          pc = end;
        }
      }

      // fallthrough 边也要接上（LOADB 的"跳过"分支除外）
      if is_fallthrough(op) && !(is_skip_c(op) && insn.c() != 0) && pc < insns.len() {
        let target_idx = owner_block_idx[pc];
        LUAU_ASSERT!(target_idx != u32::MAX && block_idx as u32 != target_idx);

        if target_idx != u32::MAX {
          blocks[block_idx].successors.push(target_idx);

          if let Some(pred_block) = blocks.get_mut(target_idx as usize) {
            pred_block.predecessors.push(block_idx as u32);
          }
        }
      }
    }

    // 5) 工作表数据流迭代：沿控制流广播被捕获的局部变量位图
    let mut worklist: Vec<u32> = vec![0];
    blocks[0].in_worklist = true;

    while let Some(idx) = worklist.pop() {
      let idx = idx as usize;
      let old_captured = blocks[idx].captured;
      let mut captured = old_captured;

      // 汇入所有已访问前驱的出口状态
      for &pred in &blocks[idx].predecessors {
        if blocks[pred as usize].visited {
          for (c, &p) in captured.iter_mut().zip(&blocks[pred as usize].captured) {
            *c |= p;
          }
        }
      }

      // cpp 中 `finishpc` 为 -1 时该扫描循环整体不执行，块仍被标注视并继续下发后继，
      // 故这里用 `if let` 取代 `continue`/伪默认值。
      if let Some(finishpc) = blocks[idx].finishpc {
        for (pc, _end, insn) in scan::stepped(insns, blocks[idx].startpc) {
          if pc > finishpc {
            break;
          }
          let op = insn.luau_opcode();

          match op {
            LuauOpcode::LOP_CLOSEUPVALS => {
              captured[insn.a() as usize..].fill(false);
            }
            LuauOpcode::LOP_CAPTURE => {
              if insn.a() == LuauCaptureType::LCT_REF as u8 {
                captured[insn.b() as usize] = true;
              }
            }
            LuauOpcode::LOP_RETURN => {
              LUAU_ASSERT!(captured.iter().all(|&c| !c));
            }
            _ => {}
          }
        }
      }

      let changed = captured != old_captured;
      blocks[idx].captured = captured;
      blocks[idx].visited = true;
      blocks[idx].in_worklist = false;

      // 后继：内容变化或未访问过则重新入队（先拷出索引，避免与 blocks 的可变借用重叠）
      let successors: SmallVector<u32, 4> = blocks[idx].successors.iter().copied().collect();
      for succ_idx in successors.iter().copied() {
        let succ = succ_idx as usize;
        if (!blocks[succ].visited || changed) && !blocks[succ].in_worklist {
          worklist.push(succ_idx);
          blocks[succ].in_worklist = true;
        }
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate_instructions.rs` ──
/// NAMECALL/NAMECALLUDATA 之后必须紧跟调用指令：常规形态是 CALL，
/// fastcall 重写后是 CALLFB（cpp `BytecodeBuilder.cpp:1979-1983/2004-2008`
/// 两处同为 `LOP_CALL || LOP_CALLFB` 判定，抽公共辅助避免再漂移）。
fn is_call_op(insn: Instruction) -> bool {
  matches!(
    insn.opcode(),
    Some(LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB)
  )
}

/// cpp `i += getOpLength(op); LUAU_ASSERT(i <= insns.size());`
/// （`BytecodeBuilder.cpp:1608-1610/2037-2039`）：按 op 长度步进后越过指令流末尾
/// 说明上一条指令声明的长度吞掉了尾部，走断言收口；不允许迭代器静默耗尽丢痕。
fn step_over_insn(insns: &[Instruction], i: usize, op: LuauOpcode) -> usize {
  let next = i + get_op_length(op) as usize;
  LUAU_ASSERT!(next <= insns.len());
  next
}

/// fastcall 族（FASTCALL/1/2/2K/3 与 FASTPCALL）臂尾共用检查：相对跳转目标
/// 处必须落在 CALL 上。cpp `BytecodeBuilder.cpp` 六处同型判定，收口单源。
fn assert_fastcall_target(insns: &[Instruction], i: usize, insn: Instruction) {
  LUAU_ASSERT!(insns[i + 1 + insn.c() as usize].opcode() == Some(LuauOpcode::LOP_CALL));
}

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn validate_instructions(&self) {
    LUAU_ASSERT!(self.current_function.is_some());
    // release 断言放行时按 id 0 校验，避免 cpp `functions[-1]` 的越界读。
    let current_function = self.current_function.unwrap_or(0) as usize;

    let func = &self.functions[current_function];
    let insns = self.instructions();

    // tag instruction offsets so that we can validate jumps
    let mut insnvalid = vec![0u8; insns.len()];

    let mut i = 0usize;
    while let Some(insn) = insns.get(i).copied() {
      let op = insn.luau_opcode();

      insnvalid[i] = 1;

      i = step_over_insn(insns, i, op);
    }

    // validate individual instructions
    let mut i = 0usize;
    while let Some(insn) = insns.get(i).copied() {
      let op = insn.luau_opcode();

      match op {
        LuauOpcode::LOP_LOADNIL => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_LOADB => {
          VREG!(insn.a(), func);
          let b_val = insn.b();
          LUAU_ASSERT!(b_val == 0 || b_val == 1);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
        }
        LuauOpcode::LOP_LOADN => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_LOADK => {
          VREG!(insn.a(), func);
          VCONSTANY!(insn.d() as usize, self.constants);
        }
        LuauOpcode::LOP_MOVE => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
        }
        LuauOpcode::LOP_GETGLOBAL | LuauOpcode::LOP_SETGLOBAL => {
          VREG!(insn.a(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUPVAL | LuauOpcode::LOP_SETUPVAL => {
          VREG!(insn.a(), func);
          VUPVAL!(insn.b(), func);
        }
        LuauOpcode::LOP_CLOSEUPVALS => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_GETIMPORT => {
          VREG!(insn.a(), func);
          VCONST!(insn.d() as usize, Import, self.constants);
          let id = insns[i + 1].raw();
          let (count, components) = decode_import_aux(id);
          LUAU_ASSERT!(count != 0); // import chain with length 1-3
          for &component in &components[..count.min(components.len() as u32) as usize] {
            VCONST!(component as usize, String, self.constants);
          }
        }
        LuauOpcode::LOP_GETTABLE | LuauOpcode::LOP_SETTABLE => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_SETTABLEKS => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETTABLEN | LuauOpcode::LOP_SETTABLEN => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
        }
        LuauOpcode::LOP_NEWCLOSURE => {
          VREG!(insn.a(), func);
          let proto_idx = insn.d() as usize;
          LUAU_ASSERT!(proto_idx < self.protos.len());
          let proto_val = self.protos[proto_idx];
          LUAU_ASSERT!(proto_val < self.functions.len() as u32);
          let numupvalues = self.functions[proto_val as usize].numupvalues as u32;

          // cpp CODEGEN_ASSERT 同款：CAPTURE 序列必须完整在指令流内
          LUAU_ASSERT!(i + 1 + numupvalues as usize <= insns.len());
          for cinsn in insns[i + 1..].iter().take(numupvalues as usize) {
            LUAU_ASSERT!(cinsn.opcode() == Some(LuauOpcode::LOP_CAPTURE));
          }
        }
        LuauOpcode::LOP_NAMECALL => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
          LUAU_ASSERT!(is_call_op(insns[i + 2]));
        }
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = (insn.b() as i32) - 1;
          let nresults = (insn.c() as i32) - 1;
          VREG!(insn.a(), func);
          VREGRANGE!(insn.a().wrapping_add(1), nparams, func);
          VREGRANGE!(insn.a(), nresults, func);
        }
        LuauOpcode::LOP_RETURN => {
          let nresults = (insn.b() as i32) - 1;
          VREGRANGE!(insn.a(), nresults, func);
        }
        LuauOpcode::LOP_JUMP => {
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
          VREG!(insn.a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT => {
          VREG!(insn.a(), func);
          VREG!(insns[i + 1].aux_a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKNIL | LuauOpcode::LOP_JUMPXEQKB => {
          VREG!(insn.a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKN => {
          VREG!(insn.a(), func);
          VCONST!(insns[i + 1].aux_kv() as usize, Number, self.constants);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKS => {
          VREG!(insn.a(), func);
          VCONST!(insns[i + 1].aux_kv() as usize, String, self.constants);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_IDIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_IDIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insn.c() as usize, Number, self.constants);
        }
        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          VREG!(insn.a(), func);
          VCONST!(insn.b() as usize, Number, self.constants);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_ANDK | LuauOpcode::LOP_ORK => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONSTANY!(insn.c() as usize, self.constants);
        }
        LuauOpcode::LOP_CONCAT => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
          LUAU_ASSERT!(insn.b() <= insn.c());
        }
        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
        }
        LuauOpcode::LOP_NEWTABLE => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_DUPTABLE => {
          VREG!(insn.a(), func);
          VCONST!(insn.d() as usize, Table, self.constants);
        }
        LuauOpcode::LOP_SETLIST => {
          let count = (insn.c() as i32) - 1;
          VREG!(insn.a(), func);
          VREGRANGE!(insn.b(), count, func);
        }
        LuauOpcode::LOP_FORNPREP | LuauOpcode::LOP_FORNLOOP => {
          VREG!(insn.a().wrapping_add(2), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FORGPREP => {
          VREG!(insn.a().wrapping_add(3), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FORGLOOP => {
          VREG!(
            insn.a().wrapping_add(2).wrapping_add(insns[i + 1].aux_a()),
            func
          );
          VJUMP!(insn.d(), i, insns, insnvalid);
          LUAU_ASSERT!(insns[i + 1].aux_a() >= 1);
        }
        LuauOpcode::LOP_FORGPREP_INEXT | LuauOpcode::LOP_FORGPREP_NEXT => {
          VREG!(insn.a().wrapping_add(4), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_GETVARARGS => {
          let nresults = (insn.b() as i32) - 1;
          VREGRANGE!(insn.a(), nresults, func);
        }
        LuauOpcode::LOP_DUPCLOSURE => {
          VREG!(insn.a(), func);
          VCONST!(insn.d() as usize, Closure, self.constants);
          let proto = self.constants[insn.d() as usize].as_closure();
          LUAU_ASSERT!(proto < self.functions.len() as u32);
          let numupvalues = self.functions[proto as usize].numupvalues as u32;

          // cpp CODEGEN_ASSERT 同款：CAPTURE 序列必须完整在指令流内
          LUAU_ASSERT!(i + 1 + numupvalues as usize <= insns.len());
          for cinsn in insns[i + 1..].iter().take(numupvalues as usize) {
            LUAU_ASSERT!(cinsn.opcode() == Some(LuauOpcode::LOP_CAPTURE));
            let capture_type = cinsn.a();
            LUAU_ASSERT!(
              capture_type == LuauCaptureType::LCT_VAL as u8
                || capture_type == LuauCaptureType::LCT_UPVAL as u8
            );
          }
        }
        LuauOpcode::LOP_PREPVARARGS => {
          LUAU_ASSERT!(insn.a() as u32 == func.numparams as u32);
          LUAU_ASSERT!(func.isvararg);
        }
        LuauOpcode::LOP_BREAK => {}
        LuauOpcode::LOP_JUMPBACK => {
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_LOADKX => {
          VREG!(insn.a(), func);
          VCONSTANY!(insns[i + 1].raw() as usize, self.constants);
        }
        LuauOpcode::LOP_JUMPX => {
          VJUMP!(insn.e(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FASTCALL => {
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
        }
        LuauOpcode::LOP_FASTCALL1 => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
        }
        LuauOpcode::LOP_FASTCALL2 => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
          VREG!(insns[i + 1].aux_a(), func);
        }
        LuauOpcode::LOP_FASTCALL2K => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
          VCONSTANY!(insns[i + 1].raw() as usize, self.constants);
        }
        LuauOpcode::LOP_FASTCALL3 => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
          VREG!(insns[i + 1].aux_a(), func);
          VREG!(insns[i + 1].aux_b(), func);
        }
        LuauOpcode::LOP_COVERAGE => {}
        LuauOpcode::LOP_CAPTURE => {
          let cap = LuauCaptureType::from_repr(insn.a());
          match cap {
            Some(LuauCaptureType::LctVal) | Some(LuauCaptureType::LctRef) => {
              VREG!(insn.b(), func);
            }
            Some(LuauCaptureType::LctUpval) => VUPVAL!(insn.b(), func),
            None => LUAU_ASSERT!(false, "Unsupported capture type"),
          }
        }
        LuauOpcode::LOP_NEWCLASSMEMBER => {
          VREG!(insn.a(), func);
          LUAU_ASSERT!(insn.b() == 0);
          VREG!(insn.c(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_SETUDATAKS => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].aux_kv16() as usize, String, self.constants);
        }
        LuauOpcode::LOP_NAMECALLUDATA => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].aux_kv16() as usize, String, self.constants);
          // cpp `BytecodeBuilder.cpp:2008`：CALLFB（fastcall 重写）同样合法
          LUAU_ASSERT!(is_call_op(insns[i + 2]));
        }
        LuauOpcode::LOP_CMPPROTO => {
          VREG!(insn.a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FASTPCALL => {
          // cpp `BytecodeBuilder.cpp:2017`：A 是 nresults 上限位，只允许 0/1
          LUAU_ASSERT!(insn.a() <= 1);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
        }
        LuauOpcode::LOP_NEWCLASS => {
          LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());
          VREG!(insn.a(), func);
          let super_reg = insn.b();
          LUAU_ASSERT!(
            super_reg as u32 == K_INVALID_REG || (super_reg as usize) < func.maxstacksize as usize
          );
          let flags = insn.c();
          LUAU_ASSERT!(flags == 0 || flags == 1);
          VCONST!(insns[i + 1].raw() as usize, ClassShape, self.constants);
        }
        _ => {
          LUAU_ASSERT!(false, "Unsupported opcode");
        }
      }

      i = step_over_insn(insns, i, op);
    }
    // 捕获闭合检查在 cpp 中由独立的 `validateCaptures`（CFG 分析）承担，
    // 已由 `BytecodeBuilder::validate` 统一调用，这里不再做线性近似。
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate_variadic.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 校验 MULTRET 序列：产出/消费变长序列的指令必须成对出现，
  /// 且序列内部（含消费指令）不得作为跳转目标。
  pub(crate) fn validate_variadic(&self) {
    let mut variadic_seq = false;
    let insns_slice = self.instructions();
    // 第一遍：标记全部跳转目标（与 validate_captures 共用的第一遍）
    let insn_targets = mark_jump_targets(insns_slice);

    // 第二遍：状态机校验 producer/consumer/neutral 的配对关系
    for (i, _end, insn) in scan::stepped(insns_slice, 0) {
      let op = insn.luau_opcode();

      if variadic_seq {
        LUAU_ASSERT!(!insn_targets[i]);
      }

      if op == LuauOpcode::LOP_CALL || op == LuauOpcode::LOP_CALLFB {
        // 注意：CALL 可能结束一个变长序列并同时开始新的序列
        if insn.b() == 0 {
          // 消费指令结束变长序列
          LUAU_ASSERT!(variadic_seq);
          variadic_seq = false;
        } else {
          // CALL 非中性指令，序列内只能是消费指令
          LUAU_ASSERT!(!variadic_seq);
        }

        if insn.c() == 0 {
          // 产出指令开启变长序列
          LUAU_ASSERT!(!variadic_seq);
          variadic_seq = true;
        }
      } else if op == LuauOpcode::LOP_GETVARARGS && insn.b() == 0 {
        // 产出指令开启变长序列
        LUAU_ASSERT!(!variadic_seq);
        variadic_seq = true;
      } else if (op == LuauOpcode::LOP_RETURN && insn.b() == 0)
        || (op == LuauOpcode::LOP_SETLIST && insn.c() == 0)
      {
        // 消费指令结束变长序列
        LUAU_ASSERT!(variadic_seq);
        variadic_seq = false;
      } else if op == LuauOpcode::LOP_FASTCALL || op == LuauOpcode::LOP_FASTPCALL {
        let call_target = (i as i32 + insn.c() as i32 + 1) as usize;
        LUAU_ASSERT!(
          call_target < insns_slice.len()
            && insns_slice[call_target].opcode() == Some(LuauOpcode::LOP_CALL)
        );

        if insns_slice[call_target].b() == 0 {
          // 消费指令链接的 CALL 稍后自行结束序列，此处只校验状态
          LUAU_ASSERT!(variadic_seq);
        } else {
          LUAU_ASSERT!(!variadic_seq);
        }
      } else if op == LuauOpcode::LOP_CLOSEUPVALS
        || op == LuauOpcode::LOP_NAMECALL
        || op == LuauOpcode::LOP_NAMECALLUDATA
        || op == LuauOpcode::LOP_GETIMPORT
        || op == LuauOpcode::LOP_MOVE
        || op == LuauOpcode::LOP_GETUPVAL
        || op == LuauOpcode::LOP_GETGLOBAL
        || op == LuauOpcode::LOP_GETTABLEKS
        || op == LuauOpcode::LOP_COVERAGE
      {
        // 变长序列内的中性指令：不改 L->top
      } else {
        LUAU_ASSERT!(!variadic_seq);
      }
    }

    LUAU_ASSERT!(!variadic_seq);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate.rs` ──
/// 变步长扫描指令流，标记每个**真实控制流转移**的落点槽位：跳转类指令的目标、
/// 且非 fastcall 族（其「目标」是指向配对 CALL 的相对偏移，不是控制流边）。
/// `validate_captures`（块重建）与 `validate_variadic`（序列目标检查）共用的
/// 第一遍。越界目标静默跳过——非法字节码可能算出越界 target，cpp 直接写下标
/// 是 UB，这里以受检写入收口（合法构建产物不会出现）。
pub(crate) fn mark_jump_targets(insns: &[Instruction]) -> Vec<bool> {
  let mut targets = vec![false; insns.len()];
  for (pc, _end, insn) in scan::stepped(insns, 0) {
    let op = insn.luau_opcode();
    let target = get_jump_target(insn.raw(), pc as u32);
    if target >= 0
      && !is_fast_call(op)
      && let Some(slot) = targets.get_mut(target as usize)
    {
      *slot = true;
    }
  }
  targets
}

impl<'a> BytecodeBuilder<'a> {
  /// cpp `BytecodeBuilder::validate()`：整段定义都在 `#ifdef LUAU_ASSERTENABLED`
  /// 之内，release 下不参与编译。这里用同一个运行时开关提前返回，`LUAU_ASSERTENABLED`
  /// 是 `const bool`，release 下整个函数体（含 `validate_instructions` 里 400 余行
  /// 校验）会被判为死分支消除，零开销。
  ///
  /// 注意：必须在 `encoder.encode` 置换操作码**之前**调用，否则 `LUAU_INSN_OP/D`
  /// 解出的全是置换后的值。
  pub(crate) fn validate(&self) {
    if !LUAU_ASSERTENABLED {
      return;
    }

    self.validate_instructions();
    self.validate_variadic();
    // cpp `validate()` 第三项：`validateCaptures`（CFG 版捕获闭合检查，
    // 取代旧版 `validateInstructions` 尾部的线性 open-captures 近似）。
    self.validate_captures();
  }
}
