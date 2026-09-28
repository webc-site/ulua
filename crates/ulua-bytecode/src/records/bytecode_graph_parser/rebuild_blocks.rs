use core::mem::take;
use std::vec::Vec;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call, is_loop_jump::is_loop_jump,
  },
  macros::luau_assert::LUAU_ASSERT,
  records::instruction::Instruction,
};

use super::BytecodeGraphParser;
use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{bc_block::BcBlock, bc_block_edge::BcBlockEdge},
};
// ── abs-r139：并自 `methods/bytecode_graph_parser_rebuild_blocks.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn rebuild_blocks(&mut self, code: &[Instruction]) -> usize {
    let entry_block = self.make_block(0);
    self.func.entry_block = entry_block;
    // 出口块无起始 pc，cpp 用 kBlockNoStartPc 作哨兵
    let exit_block = self.make_block(BcBlock::K_BLOCK_NO_START_PC);
    self.func.exit_block = exit_block;
    let codesize = code.len() as u32;
    let mut i: u32 = 0;
    let mut current_block = entry_block;
    let mut instruction_count: usize = 0;

    while i < codesize {
      let insn = code[i as usize];
      let op = insn.luau_opcode();
      // cpp 88-91：目标落在 JUMPX 上时穿透一层，取 JUMPX 自身的跳转目标
      // （JUMP→JUMPX trampoline 的场景）。此前的移植计算了穿透结果却未赋回。
      // cpp 直接索引（契约保证目标在界内）；此处越界视为无目标以避免 panic。
      let mut target = get_jump_target(insn.raw(), i);
      if target >= 0 {
        let t = target as usize;
        if t < code.len() && code[t].opcode() == Some(LuauOpcode::LOP_JUMPX) {
          target = get_jump_target(code[t].raw(), t as u32);
        }
      }

      let needs_block = target >= 0
        && !is_fast_call(op)
        && op != LuauOpcode::LOP_JUMPX
        && !self.is_jump_trampoline(i, code);
      if needs_block {
        // 单次哈希查找取号：键命中直接用；未命中则 make_block（其内部 get_or_insert
        // 已把新块登记进表）。旧实现先 contains_key 再在 if 后 get(..).unwrap() 二次
        // 哈希，此处合并后连 unwrap 一并消除。
        let target_block_op = if let Some(&existing) = self.block_by_pc.get(&(target as u32)) {
          existing
        } else {
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
            // unwrap 100% 安全：循环以「命中键」或「block_start_pc==0」终止，而入口块
            // 已由 make_block(0) 注册进表，故终止时键必在。
            let prev_block_op = *self.block_by_pc.get(&block_start_pc).unwrap();
            // 先遍历旧块后继修正其前驱回指，再整体移交（mem::take 免拷贝），
            // 新块继承全部旧后继、旧块只保留指向新块的 fallthrough。
            // 后继表先快照出来：回指改写要同时可变异标目标块，索引式遍历会撞借用冲突。
            let prev_successors: Vec<BcBlockEdge> = self.func.blocks[prev_block_op.index as usize]
              .successors
              .iter()
              .copied()
              .collect();
            for edge in prev_successors {
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
          new_block_op
        };
        let edge_kind = if is_loop_jump(op) {
          BcBlockEdgeKind::Loop
        } else {
          BcBlockEdgeKind::Branch
        };
        self.add_successor(current_block, target_block_op, edge_kind);
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
