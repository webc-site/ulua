use core::cmp;
use std::vec::Vec;

use ulua_common::{
  collections::HashSet,
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag,
  functions::{get_op_length::get_op_length, is_loop_jump::is_loop_jump},
  macros::luau_assert::LUAU_ASSERT,
  records::{instruction::Instruction, small_vector::SmallVector},
};

use super::BytecodeGraphParser;
use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  functions::decode_import_aux::decode_import_aux,
  records::{
    bc_op::BcOp, bc_op_hash::BcOpHash, block_producers::PRODUCER_SENTINEL,
    bytecode_builder::K_INVALID_REG, loop_info::LoopInfo,
  },
};
// ── abs-r139：并自 `methods/bytecode_graph_parser_rebuild_graph.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// 跳转类指令的图节点构建（cpp `parseJump`）。独立于主循环，避免每次迭代重建闭包。
  ///
  /// 节点以 `BcOp` 标识（对齐 cpp `BcRef<BcInst>` 的"指令句柄"角色），每次写入都经
  /// `BcFunction::inst_op` 现取现用，不再跨调用持有裸指针。
  fn parse_jump(
    &mut self,
    op: LuauOpcode,
    jump_target: i32,
    insn: Instruction,
    aux: Instruction,
    node_op: BcOp,
  ) {
    self.func.inst_op(node_op).op = op;
    match op {
      LuauOpcode::LOP_JUMPXEQKNIL => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
        self.add_jump_input(node_op, jump_target);
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_kb() != 0);
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
        self.add_jump_input(node_op, jump_target);
        self.add_vm_const_input(node_op, aux.aux_kv());
      }
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_vm_reg_input(node_op, aux.aux_a());
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_FORNPREP => {
        // forg loop protocol: A, A+1, A+2 are used for iteration protocol; A+3, ... are loop variables
        self.add_vm_reg_input(node_op, insn.a());
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
        self.add_jump_input(node_op, jump_target);
        self.func.regs.insert(node_op, insn.a());
        for (proj_idx, reg) in [
          (0u32, insn.a() as u32),
          (1, insn.a() as u32 + 1),
          (2, insn.a() as u32 + 2),
        ] {
          let proj = self.func.add_proj(node_op, proj_idx);
          self.add_producer(reg as u8, proj);
        }
      }
      LuauOpcode::LOP_FORNLOOP => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
        self.add_jump_input(node_op, jump_target);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }

  /// KS/NAMECALL 族的 aux 域两种编码（镜像序列化侧 `emit_ks_aux` 的解析对偶）：
  /// UDATA 变体拆「低 16 位 KV16 常量 + 高 16 位 SLOT imm」
  /// （cpp BytecodeGraphParser.h:666-691/712-720），普通变体整字是常量索引。
  fn add_ks_aux_input(&mut self, node_op: BcOp, udata: bool, aux: Instruction) {
    if udata {
      self.add_vm_const_input(node_op, aux.aux_kv16() as u32);
      self.add_imm_input_bc_inst_i32(node_op, aux.aux_slot() as i32);
    } else {
      self.add_vm_const_input(node_op, aux.raw());
    }
  }

  /// 由字节码重建 SSA 图，并返回「指令下标 → 图内 `BcInst` 序号」映射（`insns_pc`）。
  /// 建图失败（块数超限、不可信输入越界、`error` 位）返回 `None`。
  pub(crate) fn rebuild_graph(&mut self, code: &[Instruction], lines: &[u32]) -> Option<Vec<u32>> {
    let codesize = code.len() as u32;
    let instructions_count = self.rebuild_blocks(code);
    if self.block_by_pc.size() > Self::K_MAX_CFG_BLOCKS as usize {
      return None;
    }

    let mut loops: Vec<LoopInfo> = Vec::new();

    self
      .producers
      .resize(self.func.blocks.len(), Default::default());
    // 每条指令占一格，初值 0；下方按 pc 逐格填入指令序号，故预铺满整段。
    let mut pcs = vec![0u32; codesize as usize];

    self.current_block = self.func.entry_block;

    // i 是入口参数寄存器号：既是 producers 表的键，又被原样编进 `BcOp::VmReg` 句柄，
    // 属图数据而非容器游标，故保留数值范围遍历。
    for i in 0..self.func.numparams {
      self.add_producer(i, BcOp::with(BcOpKind::VmReg, i as u32));
    }

    // Create instructions.
    self.current_block = self.func.entry_block;
    self.func.instructions.reserve(instructions_count);

    // i 是 pc：相对跳转按 `insn.jump_target(i)` 以 pc 为基准解偏移、`pcs[i]` 记录
    // 字节码槽位→图指令号的映射、trampoline 分支还要在体内二次推进 i（跳过合并的
    // JUMP+JUMPX 两字），迭代器形态表达不了这种体内重推进，故保留 pc 游走。
    let mut i: u32 = 0;
    while i < codesize {
      let insn = code[i as usize];
      // op_length/aux 声明为 mut：JUMP trampoline 分支会按合并后的落点指令
      // 重赋值（对齐 cpp BytecodeGraphParser.h:784-785），否则 2 字长条件跳转
      // 合并后循环底仍按 LOP_JUMP 的 1 推进，AUX 词被误解析成伪指令节点
      let op = insn.luau_opcode();
      let mut op_length = get_op_length(op) as u32;
      let aux = if op_length > 1 && i + 1 < codesize {
        code[(i + 1) as usize]
      } else {
        Instruction(0)
      };
      let node_op = self.func.add_inst();
      self
        .func
        .block_op(self.current_block)
        .append_instruction(node_op);
      {
        let node = self.func.inst_op(node_op);
        node.block = self.current_block;
        if (i as usize) < lines.len() {
          node.line = lines[i as usize];
        }
        node.op = op;
      }

      pcs[i as usize] = node_op.index;

      match op {
        LuauOpcode::LOP_NOP | LuauOpcode::LOP_BREAK | LuauOpcode::LOP_NATIVECALL => {}

        LuauOpcode::LOP_FASTPCALL => {
          self.add_imm_input_bc_inst_i32(node_op, insn.a() as i32);
          self.add_imm_input_bc_inst_i32(node_op, insn.b() as i32);
          // 第三个 imm 承载指向 CALL 的原 C 域（跳转偏移），序列化时原样回填
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
        }

        LuauOpcode::LOP_NEWCLASS => {
          let b = insn.b();
          if (b as u32) != K_INVALID_REG {
            self.add_vm_reg_input(node_op, b);
          } else {
            self.add_empty_input(node_op);
          }
          self.add_imm_input_bc_inst_u32(node_op, insn.c() as u32);
          self.add_vm_const_input(node_op, aux.raw());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADNIL => {
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADB => {
          self.add_imm_input_bc_inst_bool(node_op, insn.b() != 0);
          self.add_jump_input(node_op, insn.jump_target(i));
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADN => {
          self.add_imm_input_bc_inst_i32(node_op, insn.d() as i32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADK => {
          self.add_vm_const_input(node_op, insn.d() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_MOVE => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_GETGLOBAL => {
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_vm_const_input(node_op, aux.raw());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETGLOBAL => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_vm_const_input(node_op, aux.raw());
        }

        LuauOpcode::LOP_GETUPVAL => {
          self.add_upval_input(node_op, insn.b() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETUPVAL => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_upval_input(node_op, insn.b() as u32);
        }

        LuauOpcode::LOP_CLOSEUPVALS => {
          self
            .func
            .inst_op(node_op)
            .ops
            .push_back(BcOp::with(BcOpKind::VmReg, insn.a() as u32));
        }

        LuauOpcode::LOP_GETIMPORT => {
          self.add_vm_const_input(node_op, insn.d() as u32);
          // aux 高 2 位为组件数，每 10 位为一个导入组件的常量索引
          let (components_count, components) = decode_import_aux(aux.raw());
          self.add_imm_input_bc_inst_i32(node_op, components_count as i32);
          for &component in &components[..components_count.min(components.len() as u32) as usize] {
            self.add_vm_const_input(node_op, component);
          }
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETTABLE => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.b());
          self.add_vm_reg_input(node_op, insn.c());
        }

        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_GETTABLEKS => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_ks_aux_input(node_op, op == LuauOpcode::LOP_GETUDATAKS, aux);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_SETTABLEKS => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_ks_aux_input(node_op, op == LuauOpcode::LOP_SETUDATAKS, aux);
        }

        LuauOpcode::LOP_GETTABLEN => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, (insn.c() as i32) + 1);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETTABLEN => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, (insn.c() as i32) + 1);
        }

        LuauOpcode::LOP_NEWCLOSURE => {
          self.add_proto_input(node_op, insn.d() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_NAMECALLUDATA | LuauOpcode::LOP_NAMECALL => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_ks_aux_input(node_op, op == LuauOpcode::LOP_NAMECALLUDATA, aux);
          self.func.regs.insert(node_op, insn.a());
          // A 与 A+1 两个寄存器各挂一个 proj（与 FORNPREP 同模式）
          for (proj_idx, reg) in [(0u32, insn.a() as u32), (1, insn.a() as u32 + 1)] {
            let proj = self.func.add_proj(node_op, proj_idx);
            self.add_producer(reg as u8, proj);
          }
        }

        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = insn.b() as i32 - 1;
          let nresults = insn.c() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, nparams);
          self.add_imm_input_bc_inst_i32(node_op, nresults);
          if op == LuauOpcode::LOP_CALLFB {
            self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          }

          // Call target.
          self.add_vm_reg_input(node_op, insn.a());
          // Fixed arguments.
          // j 是第 j 个实参相对基址 A 的寄存器偏移（寄存器号即数据）；
          // nparams<0（变参实参表）时 1..=负数 为空，与原语义一致。
          for j in 1..=nparams {
            self.add_vm_reg_input(node_op, (insn.a() as i32 + j) as u8);
          }

          if nparams < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, insn.a().wrapping_add(1));
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }

          let current_block_idx = self.current_block.index as usize;
          BytecodeGraphParser::apply_call(
            &mut self.producers[current_block_idx],
            node_op,
            insn.a(),
            nresults,
          );

          self.func.regs.insert(node_op, insn.a());
          // j 是结果投影号（编进 `BcOp::Proj` 的 index），同时是相对基寄存器 A 的偏移；
          // nresults<0（变长调用）时范围为空，与原语义一致。
          for j in 0..nresults {
            let proj = self.func.add_proj(node_op, j as u32);
            self.add_producer((insn.a() as i32 + j) as u8, proj);
          }
        }

        LuauOpcode::LOP_RETURN => {
          let nresults = insn.b() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, nresults);
          // j 是第 j 个返回值寄存器相对基址 A 的偏移（寄存器号即数据）；nresults<0 时为空。
          for j in 0..nresults {
            self.add_vm_reg_input(node_op, (insn.a() as i32 + j) as u8);
          }
          if nresults < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, insn.a());
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }
          if nresults == 0 {
            self
              .func
              .inst_op(node_op)
              .ops
              .push_back(BcOp::with(BcOpKind::VmReg, insn.a() as u32));
          }
        }

        LuauOpcode::LOP_JUMP => {
          if self.is_jump_trampoline(i, code) {
            // it is long jump trampoline
            let long_offset = code[(i + 1) as usize].e();
            i += get_op_length(LuauOpcode::LOP_JUMP) as u32
              + get_op_length(LuauOpcode::LOP_JUMPX) as u32;
            // `is_jump_trampoline` 只保证 pc+1/pc+2 在界内，推进后的 `i` 仍可能越过
            // 码尾（不可信输入）；受检访问，越界即放弃建图而不是 panic。
            let next_insn = *code.get(i as usize)?;
            let next_op = next_insn.luau_opcode();
            let next_op_length = get_op_length(next_op) as u32;
            let next_aux = if next_op_length > 1 && i + 1 < codesize {
              code[(i + 1) as usize]
            } else {
              Instruction(0)
            };
            // 回写 op_length/aux（cpp BytecodeGraphParser.h:784-785 同步重赋
            // 值）：底部按合并后指令的长度推进，2 字长条件跳转的 AUX 词不再
            // 被误解析成伪指令。op 不回写——cpp 的 loop 判定在 needsBlock 建
            // 边处（:122，用当轮原始 op），trampoline 轮的 JUMP 非 loop 跳转；
            // Rust 底部 LoopInfo 登记同理按物理指令判定，合并 op 会令
            // is_loop_jump 查到尚未建块的目标 pc 而 misset
            op_length = next_op_length;
            self.parse_jump(
              next_op,
              (i as i32) + long_offset,
              next_insn,
              next_aux,
              node_op,
            );
          } else {
            self.add_jump_input(node_op, insn.jump_target(i));
          }
        }

        LuauOpcode::LOP_JUMPBACK => {
          // repeat .. until loops use it for back edge.
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        LuauOpcode::LOP_JUMPXEQKNIL
        | LuauOpcode::LOP_JUMPXEQKB
        | LuauOpcode::LOP_JUMPXEQKN
        | LuauOpcode::LOP_JUMPXEQKS
        | LuauOpcode::LOP_JUMPIF
        | LuauOpcode::LOP_JUMPIFNOT
        | LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT
        | LuauOpcode::LOP_FORNPREP
        | LuauOpcode::LOP_FORNLOOP => {
          self.parse_jump(op, insn.jump_target(i), insn, aux, node_op);
        }

        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW
        | LuauOpcode::LOP_AND
        | LuauOpcode::LOP_OR
        // 与算术族同形：B/C 寄存器输入 + A 产出（GETTABLE/IDIV 逐字节同体）
        | LuauOpcode::LOP_GETTABLE
        | LuauOpcode::LOP_IDIV => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_vm_reg_input(node_op, insn.c());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK
        | LuauOpcode::LOP_ANDK
        | LuauOpcode::LOP_ORK
        // 与 K 常量族同形：B 寄存器 + C 常量输入 + A 产出（IDIVK 逐字节同体）
        | LuauOpcode::LOP_IDIVK => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_vm_const_input(node_op, insn.c() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_CONCAT => {
          LUAU_ASSERT!(insn.b() <= insn.c());
          for param in insn.b()..=insn.c() {
            self.add_vm_reg_input(node_op, param);
          }
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_NEWTABLE => {
          self.add_imm_input_bc_inst_i32(node_op, insn.b() as i32);
          self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_DUPTABLE => {
          self.add_producer(insn.a(), node_op);
          self.add_vm_const_input(node_op, insn.d() as u32);
        }

        LuauOpcode::LOP_SETLIST => {
          let count = insn.c() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          self.add_imm_input_bc_inst_i32(node_op, count);
          self.add_vm_reg_input(node_op, insn.a());
          // param 是第 param 个待写入元素的寄存器偏移（基址取 B），寄存器号即数据；
          // count<0（变长尾参）时范围为空，改走下面的 findProducersUpToTop。
          for param in 0..count {
            self.add_vm_reg_input(node_op, (insn.b() as i32 + param) as u8);
          }
          if count < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, insn.b());
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }
        }

        LuauOpcode::LOP_FORGPREP
        | LuauOpcode::LOP_FORGPREP_NEXT
        | LuauOpcode::LOP_FORGPREP_INEXT => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
          let loop_insn_pc = insn.jump_target(i);
          self.add_jump_input(node_op, loop_insn_pc);
          // 跳转目标来自不可信输入：cpp 只在 `LUAU_ASSERT` 里守这一步，release 下
          // `code[loopInsnPc]` / `code[loopInsnPc + 1]` 就是越界读。这里改成受检访问，
          // 条件不满足就整体放弃建图（`rebuild_graph` 返回 `None`），
          // 原断言降级为 debug 契约保留。
          let loop_pc = usize::try_from(loop_insn_pc).ok()?;
          let (Some(&loop_insn), Some(&forgloop_insn)) = (code.get(loop_pc), code.get(loop_pc + 1))
          else {
            return None;
          };
          let loop_insn_op = loop_insn.luau_opcode();
          LUAU_ASSERT!(loop_insn_op == LuauOpcode::LOP_FORGLOOP);
          let vars = forgloop_insn.aux_a() as i32;
          self.func.regs.insert(node_op, insn.a());
          // idx 是迭代协议三元组之后的第 idx 个投影号（`2 + idx` 编进 `BcOp::Proj`），
          // 同时给出对应的循环变量寄存器偏移——两个都是数据，不是容器游标。
          for idx in 0..=cmp::max(vars, 2) {
            let proj = self.func.add_proj(node_op, (2 + idx) as u32);
            self.add_producer((insn.a() as i32 + 2 + idx) as u8, proj);
          }
        }

        LuauOpcode::LOP_FORGLOOP => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
          self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
          let vars = aux.aux_a() as i32;
          self.add_imm_input_bc_inst_i32(node_op, vars);
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        // FASTCALL 族五变体同骨架：首尾 imm 相同（A 域 + 指向 CALL 的 C 域偏移，
        // 序列化时原样回填）；中段实参按变体递增——1 加 B 寄存器、2/3 追加 aux
        // 低/高字节寄存器、2K 换 aux 常量（cpp BytecodeGraphParser.h:823-871）。
        LuauOpcode::LOP_FASTCALL
        | LuauOpcode::LOP_FASTCALL1
        | LuauOpcode::LOP_FASTCALL2
        | LuauOpcode::LOP_FASTCALL2K
        | LuauOpcode::LOP_FASTCALL3 => {
          self.add_imm_input_bc_inst_i32(node_op, insn.a() as i32);
          if op != LuauOpcode::LOP_FASTCALL {
            self.add_vm_reg_input(node_op, insn.b());
            if matches!(op, LuauOpcode::LOP_FASTCALL2 | LuauOpcode::LOP_FASTCALL3) {
              self.add_vm_reg_input(node_op, aux.aux_a());
            }
            if op == LuauOpcode::LOP_FASTCALL2K {
              self.add_vm_const_input(node_op, aux.raw());
            }
            if op == LuauOpcode::LOP_FASTCALL3 {
              self.add_vm_reg_input(node_op, aux.aux_b());
            }
          }
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
        }

        LuauOpcode::LOP_GETVARARGS => {
          self
            .func
            .inst_op(node_op)
            .ops
            .push_back(BcOp::with(BcOpKind::VmReg, insn.a() as u32));
          let count = insn.b() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, count);
          self.func.regs.insert(node_op, insn.a());
          if count < 0 {
            let block_producers = &mut self.producers[self.current_block.index as usize];
            block_producers.multi_return = node_op;
            block_producers.multi_return_start = insn.a();
            block_producers.invalid_after = PRODUCER_SENTINEL as i32;
          } else {
            // j 是第 j 个变参返回值的投影号（编进 `BcOp::Proj`），兼相对 A 的寄存器偏移。
            for j in 0..count {
              let proj = self.func.add_proj(node_op, j as u32);
              self.add_producer((insn.a() as i32 + j) as u8, proj);
            }
          }
        }

        LuauOpcode::LOP_DUPCLOSURE => {
          self.add_vm_const_input(node_op, insn.d() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_PREPVARARGS => {
          self.add_imm_input_bc_inst_i32(node_op, insn.a() as i32);
        }

        LuauOpcode::LOP_LOADKX => {
          self.add_vm_const_input(node_op, aux.raw());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_JUMPX => {
          LUAU_ASSERT!(false);
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        LuauOpcode::LOP_COVERAGE => {
          self.add_imm_input_bc_inst_i32(node_op, insn.e());
        }

        LuauOpcode::LOP_CAPTURE => {
          let capture_type = insn.a() as u32;
          self.add_imm_input_bc_inst_i32(node_op, capture_type as i32);
          if capture_type == LuauCaptureType::LCT_VAL as u32
            || capture_type == LuauCaptureType::LCT_REF as u32
          {
            self.add_vm_reg_input(node_op, insn.b());
          } else {
            self.add_upval_input(node_op, insn.b() as u32);
          }
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
        }

        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          self.add_vm_const_input(node_op, insn.b() as u32);
          self.add_vm_reg_input(node_op, insn.c());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_CMPPROTO => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        LuauOpcode::LOP_NEWCLASSMEMBER => {
          LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.c());
          self.add_vm_const_input(node_op, aux.raw());
        }

        LuauOpcode::LOP__COUNT => {
          LUAU_ASSERT!(false);
        }
      }

      if is_loop_jump(op) {
        let target = insn.jump_target(i);
        // `rebuildBlocks` 会为每条跳转指令在目标 pc 建块，正常输入必然命中；
        // 但 target 由不可信字节码算出，cpp 的 `LUAU_ASSERT` 在 release 下是 no-op，
        // 紧跟的 `unwrap()` 就成了 panic 路径。改为受检查找，查不到即放弃建图。
        let &entry = u32::try_from(target)
          .ok()
          .and_then(|t| self.block_by_pc.get(&t))?;
        loops.push(LoopInfo {
          entry,
          exit: self.current_block,
        });
      }

      i += op_length;
      // 单次 get 取代 contains_key + get 的双重哈希
      if let Some(&block) = self.block_by_pc.get(&i) {
        self.current_block = block;
      }

      // 不可信输入在 `add_vm_const_input` 等处只置 `error` 位（cpp 是 release
      // no-op 的 LUAU_ASSERT）：逐条指令收尾检查，置位即放弃建图返回 false。
      if self.error {
        return None;
      }
    }

    // visited/queue 提升为循环外复用（参照 `sccp_visit` 的 scratch 约定）：
    // 每个 loop 只 clear 不重分配，避免多层循环嵌套时的逐轮堆分配。
    let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
    let mut queue: Vec<BcOp> = Vec::new();
    // 前驱/块内指令/指令操作数三级快照同样循环外复用：元素全 Copy，
    // 每轮 clear 后重填（替代旧实现逐块 collect VecDeque + 双重 clone）。
    let mut pred_snap: SmallVector<(BcBlockEdgeKind, BcOp), 4> = SmallVector::new();
    let mut ops_snap: Vec<BcOp> = Vec::new();
    let mut inst_ops: SmallVector<BcOp, 4> = SmallVector::new();
    for loop_ in &loops {
      visited.clear();
      queue.clear();
      queue.push(loop_.exit);
      while let Some(cur) = queue.pop() {
        if visited.contains(&cur) {
          continue;
        }
        visited.insert(cur);
        {
          let bl = self.func.block_op(cur);
          pred_snap.clear();
          pred_snap.extend(bl.predecessors.iter().map(|e| (e.kind, e.target)));
          ops_snap.clear();
          ops_snap.extend(bl.ops.iter().copied());
        }

        for op in &ops_snap {
          inst_ops.clear();
          inst_ops.extend(self.func.inst(*op).operator_deref().ops.iter().copied());
          for (inp_idx, &inp) in inst_ops.as_slice().iter().enumerate() {
            let Some(reg) = self.func.regs.get(&inp).copied() else {
              continue;
            };
            // try to find it in the same loop before
            if self.has_producer_before(loop_.entry, cur, *op, reg) {
              continue;
            }
            if let Some(forward_input) =
              self.find_forward_producer_in_range(cur, loop_.exit, *op, reg)
            {
              let op_val = self.func.inst(*op).operator_deref().ops[inp_idx];
              let new_val = self.add_to_phi(cur, op_val, forward_input);
              self.func.inst_op(*op).ops[inp_idx] = new_val;
              self.func.regs.insert(new_val, reg);
            }
          }
        }

        for &(ctrl, pred) in &pred_snap {
          if ctrl != BcBlockEdgeKind::Loop && !visited.contains(&pred) {
            queue.push(pred);
          }
        }
      }
    }
    // 图重建成功：交回「pc → 指令序号」映射。
    Some(pcs)
  }
}
