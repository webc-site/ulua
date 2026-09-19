use core::mem::take;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call, is_loop_jump::is_loop_jump,
  },
  macros::{luau_assert::LUAU_ASSERT, luau_insn_op::luau_insn_op},
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
      let op = LuauOpcode::from((luau_insn_op(insn) & 0xff) as u8);
      // cpp 88-91：目标落在 JUMPX 上时穿透一层，取 JUMPX 自身的跳转目标
      // （JUMP→JUMPX trampoline 的场景）。此前的移植计算了穿透结果却未赋回。
      // cpp 直接索引（契约保证目标在界内）；此处越界视为无目标以避免 panic。
      let mut target = get_jump_target(insn, i);
      if target >= 0 {
        let t = target as usize;
        if t < code.len()
          && LuauOpcode::from((luau_insn_op(code[t]) & 0xff) as u8) == LuauOpcode::LOP_JUMPX
        {
          target = get_jump_target(code[t], t as u32);
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
            // 先遍历旧块后继修正其前驱回指，再整体移交（mem::take 免拷贝），
            // 新块继承全部旧后继、旧块只保留指向新块的 fallthrough。
            let successor_count = self.func.blocks[prev_block_op.index as usize]
              .successors
              .len();
            for k in 0..successor_count {
              let edge = self.func.blocks[prev_block_op.index as usize].successors[k];
              let target_block = self.func.block_op(edge.target);
              for back_edge in target_block.predecessors.iter_mut() {
                if back_edge.target == prev_block_op {
                  back_edge.target = new_block_op;
                }
              }
            }
            self.func.blocks[new_block_op.index as usize].successors =
              take(&mut self.func.blocks[prev_block_op.index as usize].successors);
            self.add_successor(prev_block_op, new_block_op, BcBlockEdgeKind::Fallthrough);
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
      // 单次 get 取代 contains_key + get 的双重哈希；make_block 已把 i 注册进表
      let block_at_i = if needs_block || (op == LuauOpcode::LOP_RETURN && i < codesize) {
        if let Some(&existing) = self.block_by_pc.get(&i) {
          Some(existing)
        } else {
          Some(self.make_block(i))
        }
      } else {
        self.block_by_pc.get(&i).copied()
      };

      if let Some(next_block) = block_at_i {
        if is_fallthrough(op) {
          self.add_successor(current_block, next_block, BcBlockEdgeKind::Fallthrough);
        }
        current_block = next_block;
      }
      instruction_count += 1;
    }
    instruction_count
  }
}
