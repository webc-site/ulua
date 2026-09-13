use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind, records::call_inliner::CallInliner,
  type_aliases::bc_edges::BcEdges,
};

impl<'a> CallInliner<'a> {
  pub fn has_edge(&self, edges: &BcEdges, kind: BcBlockEdgeKind) -> bool {
    for e in edges {
      if e.kind == kind {
        return true;
      }
    }
    false
  }
}
