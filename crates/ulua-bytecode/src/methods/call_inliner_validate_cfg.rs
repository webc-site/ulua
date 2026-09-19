use crate::{
  enums::{bc_block_flag::BcBlockFlag, bc_op_kind::BcOpKind},
  records::{bc_block::BcBlock, call_inliner::CallInliner},
  type_aliases::bc_edges::BcEdges,
};

impl<'a> CallInliner<'a> {
  pub fn validate_cfg(&self) -> bool {
    let validate_edges =
      |from: u32, edges: &BcEdges, mirror_dir: fn(&BcBlock) -> &BcEdges| -> bool {
        for edge in edges {
          if edge.target.kind != BcOpKind::Block
            || edge.target.index as usize >= self.caller.blocks.len()
          {
            return false;
          }

          let other = &self.caller.blocks[edge.target.index as usize];

          if (other.flags & BcBlockFlag::Dead as u8) != 0 {
            return false;
          }

          let mirror = mirror_dir(other);
          if !mirror.iter().any(|e| {
            e.kind == edge.kind && e.target.kind == BcOpKind::Block && e.target.index == from
          }) {
            return false;
          }
        }
        true
      };

    for (i, block) in self.caller.blocks.iter().enumerate() {
      if (block.flags & BcBlockFlag::Dead as u8) != 0 {
        continue;
      }

      if !validate_edges(i as u32, &block.successors, |b| &b.predecessors) {
        return false;
      }

      if !validate_edges(i as u32, &block.predecessors, |b| &b.successors) {
        return false;
      }
    }

    true
  }
}
