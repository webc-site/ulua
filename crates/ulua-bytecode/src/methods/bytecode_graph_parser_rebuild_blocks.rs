use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call, is_loop_jump::is_loop_jump,
  },
  macros::{luau_assert::LUAU_ASSERT, luau_insn_op::LUAU_INSN_OP},
};

use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind, records::bytecode_graph_parser::BytecodeGraphParser,
  type_aliases::instruction::Instruction,
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn rebuild_blocks(&mut self, code: &[Instruction]) -> usize {
    let entry_block = self.make_block(0);
    self.func.entry_block = entry_block;
    let exit_block = self.make_block(0xFFFFFFFFu32);
    self.func.exit_block = exit_block;
    let codesize = code.len() as u32;
    let mut i: u32 = 0;
    let mut current_block = entry_block;
    let mut instruction_count: usize = 0;

    while i < codesize {
      let insn = code[i as usize];
      let op = LuauOpcode::from((LUAU_INSN_OP(insn) & 0xff) as u8);
      let target = get_jump_target(insn, i);
      if target >= 0 && (target as usize) < code.len() {
        let target_insn = code[target as usize];
        let target_op = LuauOpcode::from((LUAU_INSN_OP(target_insn) & 0xff) as u8);
        if target_op == LuauOpcode::LOP_JUMPX {
          let target2 = get_jump_target(target_insn, target as u32);
          if target2 >= 0 && (target2 as usize) < code.len() {
            let target2_insn = code[target2 as usize];
            let target2_op = LuauOpcode::from((LUAU_INSN_OP(target2_insn) & 0xff) as u8);
            if target2_op == LuauOpcode::LOP_JUMPX {
              // Double jumpx is not expected, but handle it anyway
              let target3 = get_jump_target(target2_insn, target2 as u32);
              if target3 >= 0 && (target3 as usize) < code.len() {
                let target3_insn = code[target3 as usize];
                let target3_op = LuauOpcode::from((LUAU_INSN_OP(target3_insn) & 0xff) as u8);
                if target3_op == LuauOpcode::LOP_JUMPX {
                  // Triple jumpx - stop here
                } else {
                  // Use target3 as final target
                }
              }
            } else {
              // Use target as final target
            }
          } else {
            // Use target as final target
          }
        } else {
          // Use target as final target
        }
      }

      let needs_block = target >= 0
        && !is_fast_call(op)
        && op != LuauOpcode::LOP_JUMPX
        && !self.is_jump_trampoline(i, code);
      if needs_block {
        if !self.block_by_pc.contains_key(&(target as u32)) {
          let new_block_op = self.make_block(target as u32);
          if (target as u32) < i {
            // We are jumping back.
            // The new block was created in the middle of the existing one.
            // We need to maintain predecessor/successor relations.
            let mut block_start_pc = target as u32 - 1;
            while !self.block_by_pc.contains_key(&block_start_pc) && block_start_pc != 0 {
              block_start_pc -= 1;
            }
            LUAU_ASSERT!(self.block_by_pc.contains_key(&block_start_pc));
            let prev_block_op = *self.block_by_pc.get(&block_start_pc).unwrap();
            // Steal successors of the previous block.
            let successors = self.func.blocks[prev_block_op.index as usize]
              .successors
              .clone();
            self.func.blocks[new_block_op.index as usize].successors = successors.clone();
            // Now it should only fallthrough to the new block.
            self.func.blocks[prev_block_op.index as usize]
              .successors
              .clear();
            self.add_successor(prev_block_op, new_block_op, BcBlockEdgeKind::Fallthrough);
            // Update all successors to have the new block as a predecessor instead of the old one.
            for edge in &successors {
              let target_block = self.func.block_op(edge.target);
              for back_edge in target_block.predecessors.iter_mut() {
                if back_edge.target == prev_block_op {
                  back_edge.target = new_block_op;
                }
              }
            }
          }
        }
        let edge_kind = if is_loop_jump(op) {
          BcBlockEdgeKind::Loop
        } else {
          BcBlockEdgeKind::Branch
        };
        self.add_successor(
          current_block,
          *self.block_by_pc.get(&(target as u32)).unwrap(),
          edge_kind,
        );
      }
      if op == LuauOpcode::LOP_RETURN {
        self.add_successor(current_block, exit_block, BcBlockEdgeKind::Fallthrough);
      }
      let op_len = get_op_length(op) as u32;
      i += op_len;
      if (needs_block || (op == LuauOpcode::LOP_RETURN && i < codesize))
        && !self.block_by_pc.contains_key(&i)
      {
        self.make_block(i);
      }

      if self.block_by_pc.contains_key(&i) {
        if is_fallthrough(op) {
          self.add_successor(
            current_block,
            *self.block_by_pc.get(&i).unwrap(),
            BcBlockEdgeKind::Fallthrough,
          );
        }
        current_block = *self.block_by_pc.get(&i).unwrap();
      }
      instruction_count += 1;
    }
    instruction_count
  }
}
