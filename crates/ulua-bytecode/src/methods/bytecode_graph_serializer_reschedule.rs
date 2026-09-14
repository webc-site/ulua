use ulua_common::LUAU_ASSERT;

use crate::{
  enums::{bc_block_flag::BcBlockFlag, bc_op_kind::BcOpKind},
  records::{bc_op::BcOp, bytecode_graph_serializer::BytecodeGraphSerializer},
};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn reschedule(&mut self) -> Vec<BcOp> {
    let mut sorted_blocks: Vec<BcOp> = Vec::with_capacity(self.func.blocks.len());
    for (i, block) in self.func.blocks.iter().enumerate() {
      if (block.flags & BcBlockFlag::Dead as u8) == 0 {
        sorted_blocks.push(BcOp::bc_op_bc_op_kind_u32(BcOpKind::Block, i as u32));
      }
    }

    sorted_blocks.sort_by(|op_a, op_b| {
      let a_block = self.func.block_op(*op_a);
      let a_sortkey = a_block.sortkey;
      let a_chainkey = a_block.chainkey;

      let b_block = self.func.block_op(*op_b);
      let b_sortkey = b_block.sortkey;
      let b_chainkey = b_block.chainkey;

      if a_sortkey == b_sortkey {
        a_chainkey.cmp(&b_chainkey)
      } else {
        a_sortkey.cmp(&b_sortkey)
      }
    });

    LUAU_ASSERT!(sorted_blocks.last() == Some(&self.func.exit_block));
    sorted_blocks.pop();

    sorted_blocks
  }
}
