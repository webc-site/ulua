use alloc::{collections::VecDeque, vec::Vec};
use core::cmp;
use std::collections::HashSet;

use ulua_common::{
  FFlag,
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_loop_jump::is_loop_jump,
  },
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B,
    luau_insn_c::LUAU_INSN_C, luau_insn_d::LUAU_INSN_D, luau_insn_e::LUAU_INSN_E,
    luau_insn_op::LUAU_INSN_OP,
  },
};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  records::{
    bc_inst::BcInst, bc_op::BcOp, bc_op_hash::BcOpHash, bytecode_graph_parser::BytecodeGraphParser,
    loop_info::LoopInfo,
  },
  type_aliases::{bc_ops::BcOps, instruction::Instruction},
};
impl<'a> BytecodeGraphParser<'a> {
  /// 跳转类指令的图节点构建（cpp `parseJump`）。独立于主循环，避免每次迭代重建闭包。
  ///
  /// SAFETY：调用方须保证 `node_op` 指向重建过程中合法存活的指令节点
  /// （与 rebuild_graph 主循环相同的裸指针约定）。
  unsafe fn parse_jump(
    &mut self,
    op: LuauOpcode,
    jump_target: i32,
    insn: u32,
    aux: u32,
    node_op: BcOp,
  ) {
    // unsafe_op_in_unsafe_fn：显式圈定裸指针解引用与 unsafe 入口调用范围
    unsafe {
      let node: *mut BcInst = self.func.inst_op(node_op);
      (*node).op = op;
      match op {
        LuauOpcode::LOP_JUMPXEQKNIL => {
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_imm_input_bc_inst_bool(node, (aux >> 31) != 0);
          self.add_jump_input(node, jump_target);
        }
        LuauOpcode::LOP_JUMPXEQKB => {
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_imm_input_bc_inst_bool(node, (aux >> 31) != 0);
          self.add_jump_input(node, jump_target);
          self.add_imm_input_bc_inst_bool(node, (aux & 0x1) != 0);
        }
        LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_imm_input_bc_inst_bool(node, (aux >> 31) != 0);
          self.add_jump_input(node, jump_target);
          self.add_vm_const_input(node, aux & 0xFFFFFF);
        }
        LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_jump_input(node, jump_target);
        }
        LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT => {
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_vm_reg_input(node, aux as u8);
          self.add_jump_input(node, jump_target);
        }
        LuauOpcode::LOP_FORNPREP => {
          // forg loop protocol: A, A+1, A+2 are used for iteration protocol; A+3, ... are loop variables
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 1) as u8);
          self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 2) as u8);
          self.add_jump_input(node, jump_target);
          self.func.regs.insert(node_op, LUAU_INSN_A(insn) as u8);
          for (proj_idx, reg) in [
            (0u32, LUAU_INSN_A(insn)),
            (1, LUAU_INSN_A(insn) + 1),
            (2, LUAU_INSN_A(insn) + 2),
          ] {
            let proj = self.func.add_proj(node_op, proj_idx);
            self.add_producer(reg as u8, proj);
          }
        }
        LuauOpcode::LOP_FORNLOOP => {
          self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
          self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 1) as u8);
          self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 2) as u8);
          self.add_jump_input(node, jump_target);
        }
        _ => {
          LUAU_ASSERT!(false);
        }
      }
    }
  }

  pub fn rebuild_graph(&mut self, code: &[Instruction], lines: &[u32], pcs: &mut Vec<u32>) -> bool {
    unsafe {
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
        let op = LuauOpcode::from((LUAU_INSN_OP(insn) & 0xff) as u8);
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
        let node: *mut BcInst = self.func.inst_op(node_op);
        (*node).block = self.current_block;
        if (i as usize) < lines.len() {
          (*node).line = lines[i as usize];
        }
        (*node).op = op;

        pcs[i as usize] = node_op.index;

        match op {
          LuauOpcode::LOP_NOP | LuauOpcode::LOP_BREAK | LuauOpcode::LOP_NATIVECALL => {}

          LuauOpcode::LOP_FASTPCALL => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_B(insn) as i32);
            // 第三个 imm 承载指向 CALL 的原 C 域（跳转偏移），序列化时原样回填
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_NEWCLASS => {
            let b = LUAU_INSN_B(insn);
            if b != 0xff {
              self.add_vm_reg_input(node, b as u8);
            } else {
              self.add_empty_input(node);
            }
            self.add_imm_input_bc_inst_u32(node, LUAU_INSN_C(insn));
            self.add_vm_const_input(node, aux);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_LOADNIL => {
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_LOADB => {
            self.add_imm_input_bc_inst_bool(node, LUAU_INSN_B(insn) != 0);
            self.add_jump_input(node, get_jump_target(insn, i));
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_LOADN => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_D(insn));
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_LOADK => {
            self.add_vm_const_input(node, LUAU_INSN_D(insn) as u32);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_MOVE => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_GETGLOBAL => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
            self.add_vm_const_input(node, aux);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_SETGLOBAL => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as u8 as i32);
            self.add_vm_const_input(node, aux);
          }

          LuauOpcode::LOP_GETUPVAL => {
            self.add_upval_input(node, LUAU_INSN_B(insn));
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_SETUPVAL => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_upval_input(node, LUAU_INSN_B(insn));
          }

          LuauOpcode::LOP_CLOSEUPVALS => {
            (*node).ops.push_back(BcOp::bc_op_bc_op_kind_u32(
              BcOpKind::VmReg,
              LUAU_INSN_A(insn),
            ));
          }

          LuauOpcode::LOP_GETIMPORT => {
            self.add_vm_const_input(node, LUAU_INSN_D(insn) as u32);
            // aux 高 2 位为组件数，每 10 位为一个导入组件的常量索引
            let components_count = (aux >> 30) as i32;
            self.add_imm_input_bc_inst_i32(node, components_count);
            for component in 0..components_count {
              self.add_vm_const_input(node, (aux >> (20 - 10 * component)) & 0x3FF);
            }
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_GETTABLE => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_C(insn) as u8);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_SETTABLE => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_C(insn) as u8);
          }

          LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_GETTABLEKS => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
            self.add_vm_const_input(node, aux);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_SETTABLEKS => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
            self.add_vm_const_input(node, aux);
          }

          LuauOpcode::LOP_GETTABLEN => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, (LUAU_INSN_C(insn) + 1) as i32);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_SETTABLEN => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, (LUAU_INSN_C(insn) + 1) as i32);
          }

          LuauOpcode::LOP_NEWCLOSURE => {
            self.add_proto_input(node, LUAU_INSN_D(insn) as u32);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_NAMECALLUDATA | LuauOpcode::LOP_NAMECALL => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
            self.add_vm_const_input(node, aux);
            self.func.regs.insert(node_op, LUAU_INSN_A(insn) as u8);
            let __proj4 = self.func.add_proj(node_op, 0);
            self.add_producer(LUAU_INSN_A(insn) as u8, __proj4);
            let __proj5 = self.func.add_proj(node_op, 1);
            self.add_producer((LUAU_INSN_A(insn) + 1) as u8, __proj5);
          }

          LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
            let nparams = LUAU_INSN_B(insn) as i32 - 1;
            let nresults = LUAU_INSN_C(insn) as i32 - 1;
            let node: *mut BcInst = self.func.inst_op(node_op);
            self.add_imm_input_bc_inst_i32(node, nparams);
            self.add_imm_input_bc_inst_i32(node, nresults);
            if op == LuauOpcode::LOP_CALLFB {
              self.add_imm_input_bc_inst_i32(node, aux as i32);
            }

            // Call target.
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            // Fixed arguments.
            for j in 1..=nparams {
              self.add_vm_reg_input(node, (LUAU_INSN_A(insn) as i32 + j) as u8);
            }

            if nparams < 0 {
              let producers_up_to_top =
                self.find_producers_up_to_top(self.current_block, (LUAU_INSN_A(insn) + 1) as u8);
              let node: *mut BcInst = self.func.inst_op(node_op);
              for inp in producers_up_to_top {
                (*node).ops.push_back(inp);
              }
            }

            let block_producers: *mut _ = &mut self.producers[self.current_block.index as usize];
            self.apply_call(
              &mut *block_producers,
              node_op,
              LUAU_INSN_A(insn) as u8,
              nresults,
            );

            self.func.regs.insert(node_op, LUAU_INSN_A(insn) as u8);
            for j in 0..nresults {
              let proj = self.func.add_proj(node_op, j as u32);
              self.add_producer((LUAU_INSN_A(insn) as i32 + j) as u8, proj);
            }
          }

          LuauOpcode::LOP_RETURN => {
            let nresults = LUAU_INSN_B(insn) as i32 - 1;
            let node: *mut BcInst = self.func.inst_op(node_op);
            self.add_imm_input_bc_inst_i32(node, nresults);
            for j in 0..nresults {
              self.add_vm_reg_input(node, (LUAU_INSN_A(insn) as i32 + j) as u8);
            }
            if nresults < 0 {
              let producers_up_to_top =
                self.find_producers_up_to_top(self.current_block, LUAU_INSN_A(insn) as u8);
              let node: *mut BcInst = self.func.inst_op(node_op);
              for inp in producers_up_to_top {
                (*node).ops.push_back(inp);
              }
            }
            if nresults == 0 {
              let node: *mut BcInst = self.func.inst_op(node_op);
              (*node).ops.push_back(BcOp::bc_op_bc_op_kind_u32(
                BcOpKind::VmReg,
                LUAU_INSN_A(insn),
              ));
            }
          }

          LuauOpcode::LOP_JUMP => {
            if self.is_jump_trampoline(i, code) {
              // it is long jump trampoline
              let long_offset = LUAU_INSN_E(code[(i + 1) as usize]);
              i += get_op_length(LuauOpcode::LOP_JUMP) as u32
                + get_op_length(LuauOpcode::LOP_JUMPX) as u32;
              let next_insn = code[i as usize];
              let next_op = LuauOpcode::from((LUAU_INSN_OP(next_insn) & 0xff) as u8);
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
              self.add_jump_input(node, get_jump_target(insn, i));
            }
          }

          LuauOpcode::LOP_JUMPBACK => {
            // repeat .. until loops use it for back edge.
            self.add_jump_input(node, get_jump_target(insn, i));
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
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_C(insn) as u8);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_ADDK
          | LuauOpcode::LOP_SUBK
          | LuauOpcode::LOP_MULK
          | LuauOpcode::LOP_DIVK
          | LuauOpcode::LOP_MODK
          | LuauOpcode::LOP_POWK
          | LuauOpcode::LOP_ANDK
          | LuauOpcode::LOP_ORK => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_const_input(node, LUAU_INSN_C(insn));
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_CONCAT => {
            LUAU_ASSERT!(LUAU_INSN_B(insn) <= LUAU_INSN_C(insn));
            for param in LUAU_INSN_B(insn)..=LUAU_INSN_C(insn) {
              self.add_vm_reg_input(node, param as u8);
            }
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_NEWTABLE => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_B(insn) as i32);
            self.add_imm_input_bc_inst_i32(node, aux as i32);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_DUPTABLE => {
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
            self.add_vm_const_input(node, LUAU_INSN_D(insn) as u32);
          }

          LuauOpcode::LOP_SETLIST => {
            let count = LUAU_INSN_C(insn) as i32 - 1;
            self.add_imm_input_bc_inst_i32(node, aux as i32);
            self.add_imm_input_bc_inst_i32(node, count);
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            for param in 0..count {
              self.add_vm_reg_input(node, (LUAU_INSN_B(insn) as i32 + param) as u8);
            }
            if count < 0 {
              let producers_up_to_top =
                self.find_producers_up_to_top(self.current_block, LUAU_INSN_B(insn) as u8);
              let node: *mut BcInst = self.func.inst_op(node_op);
              for inp in producers_up_to_top {
                (*node).ops.push_back(inp);
              }
            }
          }

          LuauOpcode::LOP_FORGPREP
          | LuauOpcode::LOP_FORGPREP_NEXT
          | LuauOpcode::LOP_FORGPREP_INEXT => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 1) as u8);
            self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 2) as u8);
            let loop_insn_pc = get_jump_target(insn, i);
            self.add_jump_input(node, loop_insn_pc);
            let loop_insn = code[loop_insn_pc as usize];
            let loop_insn_op = LuauOpcode::from((LUAU_INSN_OP(loop_insn) & 0xff) as u8);
            LUAU_ASSERT!(
              loop_insn_pc + 1 < codesize as i32 && loop_insn_op == LuauOpcode::LOP_FORGLOOP
            );
            let vars = code[(loop_insn_pc + 1) as usize] as i32 & 0xFF;
            self.func.regs.insert(node_op, LUAU_INSN_A(insn) as u8);
            for idx in 0..=cmp::max(vars, 2) {
              let proj = self.func.add_proj(node_op, (2 + idx) as u32);
              self.add_producer((LUAU_INSN_A(insn) as i32 + 2 + idx) as u8, proj);
            }
          }

          LuauOpcode::LOP_FORGLOOP => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 1) as u8);
            self.add_vm_reg_input(node, (LUAU_INSN_A(insn) + 2) as u8);
            self.add_imm_input_bc_inst_bool(node, (aux >> 31) != 0);
            let vars = (aux & 0xFF) as i32;
            self.add_imm_input_bc_inst_i32(node, vars);
            self.add_jump_input(node, get_jump_target(insn, i));
          }

          LuauOpcode::LOP_FASTCALL => {
            // Note that FASTCALL will read the actual call arguments, such as argument/result registers and counts, from the CALL instruction
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
            // turn it in BcOp to CALL BcInst&.
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_FASTCALL1 => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            // turn it in BcOp to CALL BcInst&.
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_FASTCALL2 => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_reg_input(node, (aux & 0xFF) as u8);
            // turn it in BcOp to CALL BcInst&.
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_FASTCALL2K => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_const_input(node, aux);
            // turn it in BcOp to CALL BcInst&.
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_FASTCALL3 => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_reg_input(node, (aux & 0xFF) as u8);
            self.add_vm_reg_input(node, ((aux >> 8) & 0xFF) as u8);
            // turn it in BcOp to CALL BcInst&.
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_GETVARARGS => {
            (*node).ops.push_back(BcOp::bc_op_bc_op_kind_u32(
              BcOpKind::VmReg,
              LUAU_INSN_A(insn),
            ));
            let count = LUAU_INSN_B(insn) as i32 - 1;
            self.add_imm_input_bc_inst_i32(node, count);
            self.func.regs.insert(node_op, LUAU_INSN_A(insn) as u8);
            if count < 0 {
              let block_producers = &mut self.producers[self.current_block.index as usize];
              block_producers.multi_return = node_op;
              block_producers.multi_return_start = LUAU_INSN_A(insn) as u8;
              block_producers.invalid_after = 255;
            } else {
              for j in 0..count {
                let proj = self.func.add_proj(node_op, j as u32);
                self.add_producer((LUAU_INSN_A(insn) as i32 + j) as u8, proj);
              }
            }
          }

          LuauOpcode::LOP_DUPCLOSURE => {
            self.add_vm_const_input(node, LUAU_INSN_D(insn) as u32);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_PREPVARARGS => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_A(insn) as i32);
          }

          LuauOpcode::LOP_LOADKX => {
            self.add_vm_const_input(node, aux);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_JUMPX => {
            LUAU_ASSERT!(false);
            self.add_jump_input(node, get_jump_target(insn, i));
          }

          LuauOpcode::LOP_COVERAGE => {
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_E(insn));
          }

          LuauOpcode::LOP_CAPTURE => {
            let capture_type = LUAU_INSN_A(insn);
            self.add_imm_input_bc_inst_i32(node, capture_type as i32);
            if capture_type == LuauCaptureType::LCT_VAL as u32
              || capture_type == LuauCaptureType::LCT_REF as u32
            {
              self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            } else {
              self.add_upval_input(node, LUAU_INSN_B(insn));
            }
            self.add_imm_input_bc_inst_i32(node, LUAU_INSN_C(insn) as i32);
          }

          LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
            self.add_vm_const_input(node, LUAU_INSN_B(insn));
            self.add_vm_reg_input(node, LUAU_INSN_C(insn) as u8);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_IDIV => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_C(insn) as u8);
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_IDIVK => {
            self.add_vm_reg_input(node, LUAU_INSN_B(insn) as u8);
            self.add_vm_const_input(node, LUAU_INSN_C(insn));
            self.add_producer(LUAU_INSN_A(insn) as u8, node_op);
          }

          LuauOpcode::LOP_CMPPROTO => {
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_imm_input_bc_inst_i32(node, aux as i32);
            self.add_jump_input(node, get_jump_target(insn, i));
          }

          LuauOpcode::LOP_NEWCLASSMEMBER => {
            LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());
            self.add_vm_reg_input(node, LUAU_INSN_A(insn) as u8);
            self.add_vm_reg_input(node, LUAU_INSN_C(insn) as u8);
            self.add_vm_const_input(node, aux);
          }

          LuauOpcode::LOP__COUNT => {
            LUAU_ASSERT!(false);
          }
        }

        if is_loop_jump(op) {
          let target = get_jump_target(insn, i);
          LUAU_ASSERT!(target >= 0 && self.block_by_pc.contains_key(&(target as u32)));
          loops.push(LoopInfo {
            entry: *self.block_by_pc.get(&(target as u32)).unwrap(),
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
            let inst_ops: BcOps = {
              let inst = self.func.inst_op(*op);
              inst.ops.clone()
            };
            for (inp_idx, &inp) in inst_ops.as_slice().iter().enumerate() {
              let reg_it = self.func.regs.get(&inp);
              let Some(reg) = reg_it.copied() else {
                continue;
              };
              // try to find it in the same loop before
              if self.has_producer_before_bc_op_bc_op_bc_op_reg(loop_.entry, cur, *op, reg) {
                continue;
              }
              if let Some(forward_input) =
                self.find_forward_producer_in_range_bc_op_bc_op_bc_op_reg(cur, loop_.exit, *op, reg)
              {
                let inst: *mut BcInst = self.func.inst_op(*op);
                let op_val = {
                  let ops = &(*inst).ops;
                  ops[inp_idx]
                };
                let new_val = self.add_to_phi(cur, op_val, forward_input);
                let ops = &mut (*inst).ops;
                ops[inp_idx] = new_val;
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
}
