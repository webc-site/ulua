use alloc::vec::Vec;
use core::mem::take;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_block_flag::BcBlockFlag, bc_op_kind::BcOpKind},
  records::{
    bc_block::BcBlock, bc_function::VmConst, bc_jump::BcJump,
    bytecode_graph_serializer::BytecodeGraphSerializer,
  },
};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn emit_bytecode(&mut self) -> Vec<u32> {
    let schedule = self.reschedule();
    // cpp 700：初始哨兵 ~0u，死块/未发射指令的 pc 映射保持无效值
    let mut insns_pc: Vec<u32> = vec![u32::MAX; self.func.instructions.len()];

    for (i, &block_op) in schedule.iter().enumerate() {
      let fallthrough = {
        let block: &BcBlock = &self.func.blocks[block_op.index as usize];
        block
          .successors
          .iter()
          .find(|edge| edge.kind == BcBlockEdgeKind::Fallthrough)
          .map(|edge| edge.target)
      };
      // cpp 707：fallthrough 落在死块上时不补跳转
      if let Some(fallthrough_op) = fallthrough
        && fallthrough_op != self.func.exit_block
        && self.func.blocks[fallthrough_op.index as usize].flags & (BcBlockFlag::Dead as u8) == 0
        && schedule.get(i + 1) != Some(&fallthrough_op)
      {
        let mut jump = BcJump::<VmConst>::create(self.func);
        jump.set_target(fallthrough_op);
        jump.append_to(block_op);
        // cpp 713：jump 追加导致指令数增长，新槽位按 resize 默认补 0
        insns_pc.resize(self.func.instructions.len(), 0);
      }
      let ops = {
        let block: &mut BcBlock = self.func.block_op(block_op);
        block.startpc = self.bcb.get_debug_pc();
        block.ops.iter().cloned().collect::<Vec<_>>()
      };
      for op in ops {
        LUAU_ASSERT!(op.kind == BcOpKind::Inst);
        insns_pc[op.index as usize] = self.bcb.get_debug_pc();
        self.emit_instruction(op);
      }
    }

    let mut jumps = take(&mut self.jumps);
    for jump in jumps.iter_mut() {
      self.patch_jump(jump);
    }
    self.jumps = jumps;

    if self.error { Vec::new() } else { insns_pc }
  }
}
