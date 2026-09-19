use alloc::{collections::VecDeque, vec::Vec};
use core::cmp;
use std::collections::HashSet;

use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_loop_jump::is_loop_jump,
  },
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b,
    luau_insn_c::luau_insn_c, luau_insn_d::luau_insn_d, luau_insn_e::luau_insn_e,
    luau_insn_op::luau_insn_op,
  },
  records::small_vector::SmallVector,
};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp, bc_op_hash::BcOpHash, bytecode_graph_parser::BytecodeGraphParser,
    loop_info::LoopInfo,
  },
  type_aliases::instruction::Instruction,
};

impl<'a> BytecodeGraphParser<'a> {
  /// 跳转类指令的图节点构建（cpp `parseJump`）。独立于主循环，避免每次迭代重建闭包。
  ///
  /// 节点以 `BcOp` 标识（对齐 cpp `BcRef<BcInst>` 的"指令句柄"角色），每次写入都经
  /// `BcFunction::inst_op` 现取现用，不再跨调用持有裸指针。
  fn parse_jump(&mut self, op: LuauOpcode, jump_target: i32, insn: u32, aux: u32, node_op: BcOp) {
    self.func.inst_op(node_op).op = op;
    match op {
      LuauOpcode::LOP_JUMPXEQKNIL => {
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_imm_input_bc_inst_bool(node_op, (aux >> 31) != 0);
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_imm_input_bc_inst_bool(node_op, (aux >> 31) != 0);
        self.add_jump_input(node_op, jump_target);
        self.add_imm_input_bc_inst_bool(node_op, (aux & 0x1) != 0);
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_imm_input_bc_inst_bool(node_op, (aux >> 31) != 0);
        self.add_jump_input(node_op, jump_target);
        self.add_vm_const_input(node_op, aux & 0xFFFFFF);
      }
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_vm_reg_input(node_op, aux as u8);
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_FORNPREP => {
        // forg loop protocol: A, A+1, A+2 are used for iteration protocol; A+3, ... are loop variables
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 1) as u8);
        self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 2) as u8);
        self.add_jump_input(node_op, jump_target);
        self.func.regs.insert(node_op, luau_insn_a(insn) as u8);
        for (proj_idx, reg) in [
          (0u32, luau_insn_a(insn)),
          (1, luau_insn_a(insn) + 1),
          (2, luau_insn_a(insn) + 2),
        ] {
          let proj = self.func.add_proj(node_op, proj_idx);
          self.add_producer(reg as u8, proj);
        }
      }
      LuauOpcode::LOP_FORNLOOP => {
        self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
        self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 1) as u8);
        self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 2) as u8);
        self.add_jump_input(node_op, jump_target);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }

  pub fn rebuild_graph(&mut self, code: &[Instruction], lines: &[u32], pcs: &mut Vec<u32>) -> bool {
    let codesize = code.len() as u32;
    let instructions_count = self.rebuild_blocks(code);
    if self.block_by_pc.size() > Self::K_MAX_CFG_BLOCKS as usize {
      return false;
    }

    let mut loops: Vec<LoopInfo> = Vec::new();

    self
      .producers
      .resize(self.func.blocks.len(), Default::default());
    pcs.resize(codesize as usize, 0);

    self.current_block = self.func.entry_block;

    for i in 0..self.func.numparams {
      self.add_producer(i, BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmReg, i as u32));
    }

    // Create instructions.
    self.current_block = self.func.entry_block;
    self.func.instructions.reserve(instructions_count);

    let mut i: u32 = 0;
    while i < codesize {
      let insn = code[i as usize];
      let op = LuauOpcode::from((luau_insn_op(insn) & 0xff) as u8);
      let op_length = get_op_length(op) as u32;
      let aux = if op_length > 1 && i + 1 < codesize {
        code[(i + 1) as usize]
      } else {
        0
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
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_b(insn) as i32);
          // 第三个 imm 承载指向 CALL 的原 C 域（跳转偏移），序列化时原样回填
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_NEWCLASS => {
          let b = luau_insn_b(insn);
          if b != 0xff {
            self.add_vm_reg_input(node_op, b as u8);
          } else {
            self.add_empty_input(node_op);
          }
          self.add_imm_input_bc_inst_u32(node_op, luau_insn_c(insn));
          self.add_vm_const_input(node_op, aux);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_LOADNIL => {
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_LOADB => {
          self.add_imm_input_bc_inst_bool(node_op, luau_insn_b(insn) != 0);
          self.add_jump_input(node_op, get_jump_target(insn, i));
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_LOADN => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_d(insn));
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_LOADK => {
          self.add_vm_const_input(node_op, luau_insn_d(insn) as u32);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_MOVE => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_GETGLOBAL => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
          self.add_vm_const_input(node_op, aux);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_SETGLOBAL => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
          self.add_vm_const_input(node_op, aux);
        }

        LuauOpcode::LOP_GETUPVAL => {
          self.add_upval_input(node_op, luau_insn_b(insn));
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_SETUPVAL => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_upval_input(node_op, luau_insn_b(insn));
        }

        LuauOpcode::LOP_CLOSEUPVALS => {
          self
            .func
            .inst_op(node_op)
            .ops
            .push_back(BcOp::bc_op_bc_op_kind_u32(
              BcOpKind::VmReg,
              luau_insn_a(insn),
            ));
        }

        LuauOpcode::LOP_GETIMPORT => {
          self.add_vm_const_input(node_op, luau_insn_d(insn) as u32);
          // aux 高 2 位为组件数，每 10 位为一个导入组件的常量索引
          let components_count = (aux >> 30) as i32;
          self.add_imm_input_bc_inst_i32(node_op, components_count);
          for component in 0..components_count {
            self.add_vm_const_input(node_op, (aux >> (20 - 10 * component)) & 0x3FF);
          }
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_GETTABLE => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_c(insn) as u8);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_SETTABLE => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_c(insn) as u8);
        }

        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_GETTABLEKS => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
          self.add_vm_const_input(node_op, aux);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_SETTABLEKS => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
          self.add_vm_const_input(node_op, aux);
        }

        LuauOpcode::LOP_GETTABLEN => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, (luau_insn_c(insn) + 1) as i32);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_SETTABLEN => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, (luau_insn_c(insn) + 1) as i32);
        }

        LuauOpcode::LOP_NEWCLOSURE => {
          self.add_proto_input(node_op, luau_insn_d(insn) as u32);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_NAMECALLUDATA | LuauOpcode::LOP_NAMECALL => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
          self.add_vm_const_input(node_op, aux);
          self.func.regs.insert(node_op, luau_insn_a(insn) as u8);
          // A 与 A+1 两个寄存器各挂一个 proj（与 FORNPREP 同模式）
          for (proj_idx, reg) in [(0u32, luau_insn_a(insn)), (1, luau_insn_a(insn) + 1)] {
            let proj = self.func.add_proj(node_op, proj_idx);
            self.add_producer(reg as u8, proj);
          }
        }

        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = luau_insn_b(insn) as i32 - 1;
          let nresults = luau_insn_c(insn) as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, nparams);
          self.add_imm_input_bc_inst_i32(node_op, nresults);
          if op == LuauOpcode::LOP_CALLFB {
            self.add_imm_input_bc_inst_i32(node_op, aux as i32);
          }

          // Call target.
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          // Fixed arguments.
          for j in 1..=nparams {
            self.add_vm_reg_input(node_op, (luau_insn_a(insn) as i32 + j) as u8);
          }

          if nparams < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, (luau_insn_a(insn) + 1) as u8);
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }

          let current_block_idx = self.current_block.index as usize;
          BytecodeGraphParser::apply_call(
            &mut self.producers[current_block_idx],
            node_op,
            luau_insn_a(insn) as u8,
            nresults,
          );

          self.func.regs.insert(node_op, luau_insn_a(insn) as u8);
          for j in 0..nresults {
            let proj = self.func.add_proj(node_op, j as u32);
            self.add_producer((luau_insn_a(insn) as i32 + j) as u8, proj);
          }
        }

        LuauOpcode::LOP_RETURN => {
          let nresults = luau_insn_b(insn) as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, nresults);
          for j in 0..nresults {
            self.add_vm_reg_input(node_op, (luau_insn_a(insn) as i32 + j) as u8);
          }
          if nresults < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, luau_insn_a(insn) as u8);
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }
          if nresults == 0 {
            self
              .func
              .inst_op(node_op)
              .ops
              .push_back(BcOp::bc_op_bc_op_kind_u32(
                BcOpKind::VmReg,
                luau_insn_a(insn),
              ));
          }
        }

        LuauOpcode::LOP_JUMP => {
          if self.is_jump_trampoline(i, code) {
            // it is long jump trampoline
            let long_offset = luau_insn_e(code[(i + 1) as usize]);
            i += get_op_length(LuauOpcode::LOP_JUMP) as u32
              + get_op_length(LuauOpcode::LOP_JUMPX) as u32;
            // `is_jump_trampoline` 只保证 pc+1/pc+2 在界内，推进后的 `i` 仍可能越过
            // 码尾（不可信输入）；受检访问，越界即放弃建图而不是 panic。
            let Some(&next_insn) = code.get(i as usize) else {
              return false;
            };
            let next_op = LuauOpcode::from((luau_insn_op(next_insn) & 0xff) as u8);
            let next_op_length = get_op_length(next_op) as u32;
            let next_aux = if next_op_length > 1 && i + 1 < codesize {
              code[(i + 1) as usize]
            } else {
              0
            };
            self.parse_jump(
              next_op,
              (i as i32) + long_offset,
              next_insn,
              next_aux,
              node_op,
            );
          } else {
            self.add_jump_input(node_op, get_jump_target(insn, i));
          }
        }

        LuauOpcode::LOP_JUMPBACK => {
          // repeat .. until loops use it for back edge.
          self.add_jump_input(node_op, get_jump_target(insn, i));
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
          self.parse_jump(op, get_jump_target(insn, i), insn, aux, node_op);
        }

        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW
        | LuauOpcode::LOP_AND
        | LuauOpcode::LOP_OR => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_c(insn) as u8);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK
        | LuauOpcode::LOP_ANDK
        | LuauOpcode::LOP_ORK => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_const_input(node_op, luau_insn_c(insn));
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_CONCAT => {
          LUAU_ASSERT!(luau_insn_b(insn) <= luau_insn_c(insn));
          for param in luau_insn_b(insn)..=luau_insn_c(insn) {
            self.add_vm_reg_input(node_op, param as u8);
          }
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_NEWTABLE => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_b(insn) as i32);
          self.add_imm_input_bc_inst_i32(node_op, aux as i32);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_DUPTABLE => {
          self.add_producer(luau_insn_a(insn) as u8, node_op);
          self.add_vm_const_input(node_op, luau_insn_d(insn) as u32);
        }

        LuauOpcode::LOP_SETLIST => {
          let count = luau_insn_c(insn) as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, aux as i32);
          self.add_imm_input_bc_inst_i32(node_op, count);
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          for param in 0..count {
            self.add_vm_reg_input(node_op, (luau_insn_b(insn) as i32 + param) as u8);
          }
          if count < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, luau_insn_b(insn) as u8);
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }
        }

        LuauOpcode::LOP_FORGPREP
        | LuauOpcode::LOP_FORGPREP_NEXT
        | LuauOpcode::LOP_FORGPREP_INEXT => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 1) as u8);
          self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 2) as u8);
          let loop_insn_pc = get_jump_target(insn, i);
          self.add_jump_input(node_op, loop_insn_pc);
          // 跳转目标来自不可信输入：cpp 只在 `LUAU_ASSERT` 里守这一步，release 下
          // `code[loopInsnPc]` / `code[loopInsnPc + 1]` 就是越界读。这里改成受检访问，
          // 条件不满足就整体放弃建图（`rebuild_graph` 返回 false），
          // 原断言降级为 debug 契约保留。
          let Some(loop_pc) = usize::try_from(loop_insn_pc).ok() else {
            return false;
          };
          let (Some(&loop_insn), Some(&forgloop_insn)) = (code.get(loop_pc), code.get(loop_pc + 1))
          else {
            return false;
          };
          let loop_insn_op = LuauOpcode::from((luau_insn_op(loop_insn) & 0xff) as u8);
          LUAU_ASSERT!(loop_insn_op == LuauOpcode::LOP_FORGLOOP);
          let vars = forgloop_insn as i32 & 0xFF;
          self.func.regs.insert(node_op, luau_insn_a(insn) as u8);
          for idx in 0..=cmp::max(vars, 2) {
            let proj = self.func.add_proj(node_op, (2 + idx) as u32);
            self.add_producer((luau_insn_a(insn) as i32 + 2 + idx) as u8, proj);
          }
        }

        LuauOpcode::LOP_FORGLOOP => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 1) as u8);
          self.add_vm_reg_input(node_op, (luau_insn_a(insn) + 2) as u8);
          self.add_imm_input_bc_inst_bool(node_op, (aux >> 31) != 0);
          let vars = (aux & 0xFF) as i32;
          self.add_imm_input_bc_inst_i32(node_op, vars);
          self.add_jump_input(node_op, get_jump_target(insn, i));
        }

        LuauOpcode::LOP_FASTCALL => {
          // Note that FASTCALL will read the actual call arguments, such as argument/result registers and counts, from the CALL instruction
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
          // turn it in BcOp to CALL BcInst&.
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_FASTCALL1 => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          // turn it in BcOp to CALL BcInst&.
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_FASTCALL2 => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_reg_input(node_op, (aux & 0xFF) as u8);
          // turn it in BcOp to CALL BcInst&.
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_FASTCALL2K => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_const_input(node_op, aux);
          // turn it in BcOp to CALL BcInst&.
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_FASTCALL3 => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_reg_input(node_op, (aux & 0xFF) as u8);
          self.add_vm_reg_input(node_op, ((aux >> 8) & 0xFF) as u8);
          // turn it in BcOp to CALL BcInst&.
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_GETVARARGS => {
          self
            .func
            .inst_op(node_op)
            .ops
            .push_back(BcOp::bc_op_bc_op_kind_u32(
              BcOpKind::VmReg,
              luau_insn_a(insn),
            ));
          let count = luau_insn_b(insn) as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, count);
          self.func.regs.insert(node_op, luau_insn_a(insn) as u8);
          if count < 0 {
            let block_producers = &mut self.producers[self.current_block.index as usize];
            block_producers.multi_return = node_op;
            block_producers.multi_return_start = luau_insn_a(insn) as u8;
            block_producers.invalid_after = 255;
          } else {
            for j in 0..count {
              let proj = self.func.add_proj(node_op, j as u32);
              self.add_producer((luau_insn_a(insn) as i32 + j) as u8, proj);
            }
          }
        }

        LuauOpcode::LOP_DUPCLOSURE => {
          self.add_vm_const_input(node_op, luau_insn_d(insn) as u32);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_PREPVARARGS => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_a(insn) as i32);
        }

        LuauOpcode::LOP_LOADKX => {
          self.add_vm_const_input(node_op, aux);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_JUMPX => {
          LUAU_ASSERT!(false);
          self.add_jump_input(node_op, get_jump_target(insn, i));
        }

        LuauOpcode::LOP_COVERAGE => {
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_e(insn));
        }

        LuauOpcode::LOP_CAPTURE => {
          let capture_type = luau_insn_a(insn);
          self.add_imm_input_bc_inst_i32(node_op, capture_type as i32);
          if capture_type == LuauCaptureType::LCT_VAL as u32
            || capture_type == LuauCaptureType::LCT_REF as u32
          {
            self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          } else {
            self.add_upval_input(node_op, luau_insn_b(insn));
          }
          self.add_imm_input_bc_inst_i32(node_op, luau_insn_c(insn) as i32);
        }

        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          self.add_vm_const_input(node_op, luau_insn_b(insn));
          self.add_vm_reg_input(node_op, luau_insn_c(insn) as u8);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_IDIV => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_c(insn) as u8);
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_IDIVK => {
          self.add_vm_reg_input(node_op, luau_insn_b(insn) as u8);
          self.add_vm_const_input(node_op, luau_insn_c(insn));
          self.add_producer(luau_insn_a(insn) as u8, node_op);
        }

        LuauOpcode::LOP_CMPPROTO => {
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_imm_input_bc_inst_i32(node_op, aux as i32);
          self.add_jump_input(node_op, get_jump_target(insn, i));
        }

        LuauOpcode::LOP_NEWCLASSMEMBER => {
          LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
          self.add_vm_reg_input(node_op, luau_insn_a(insn) as u8);
          self.add_vm_reg_input(node_op, luau_insn_c(insn) as u8);
          self.add_vm_const_input(node_op, aux);
        }

        LuauOpcode::LOP__COUNT => {
          LUAU_ASSERT!(false);
        }
      }

      if is_loop_jump(op) {
        let target = get_jump_target(insn, i);
        // `rebuildBlocks` 会为每条跳转指令在目标 pc 建块，正常输入必然命中；
        // 但 target 由不可信字节码算出，cpp 的 `LUAU_ASSERT` 在 release 下是 no-op，
        // 紧跟的 `unwrap()` 就成了 panic 路径。改为受检查找，查不到即放弃建图。
        let Some(&entry) = u32::try_from(target)
          .ok()
          .and_then(|t| self.block_by_pc.get(&t))
        else {
          return false;
        };
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
    }

    for loop_ in &loops {
      let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
      let mut queue: Vec<BcOp> = Vec::new();
      queue.push(loop_.exit);
      while let Some(cur) = queue.pop() {
        if visited.contains(&cur) {
          continue;
        }
        visited.insert(cur);
        let predecessors: VecDeque<(BcBlockEdgeKind, BcOp)> = {
          let bl = self.func.block_op(cur);
          bl.predecessors.iter().map(|e| (e.kind, e.target)).collect()
        };
        let ops: VecDeque<BcOp> = {
          let bl = self.func.block_op(cur);
          bl.ops.clone()
        };

        for op in &ops {
          let inst_ops: SmallVector<BcOp, 4> = {
            let inst = self.func.inst_op(*op);
            inst.ops.clone()
          };
          for (inp_idx, &inp) in inst_ops.as_slice().iter().enumerate() {
            let Some(reg) = self.func.regs.get(&inp).copied() else {
              continue;
            };
            // try to find it in the same loop before
            if self.has_producer_before_bc_op_bc_op_bc_op_reg(loop_.entry, cur, *op, reg) {
              continue;
            }
            if let Some(forward_input) =
              self.find_forward_producer_in_range_bc_op_bc_op_bc_op_reg(cur, loop_.exit, *op, reg)
            {
              let op_val = self.func.inst(*op).operator_deref().ops[inp_idx];
              let new_val = self.add_to_phi(cur, op_val, forward_input);
              self.func.inst_op(*op).ops[inp_idx] = new_val;
              self.func.regs.insert(new_val, reg);
            }
          }
        }

        for &(ctrl, pred) in &predecessors {
          if ctrl != BcBlockEdgeKind::Loop && !visited.contains(&pred) {
            queue.push(pred);
          }
        }
      }
    }
    true
  }
}
