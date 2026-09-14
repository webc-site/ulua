use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{bc_block_edge::BcBlockEdge, bc_op::BcOp, call_inliner::CallInliner},
  type_aliases::bc_edges::BcEdges,
};

impl<'a> CallInliner<'a> {
  pub fn set_fallthrough(&mut self, edges: &mut BcEdges, entry: BcOp) {
    for e in edges.iter_mut() {
      if e.kind == BcBlockEdgeKind::Fallthrough {
        e.target = entry;
        return;
      }
    }
    edges.push_back(BcBlockEdge {
      kind: BcBlockEdgeKind::Fallthrough,
      target: entry,
    });
  }
}
