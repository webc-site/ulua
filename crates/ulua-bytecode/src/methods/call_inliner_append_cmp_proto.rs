use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{
    bc_block::BcBlock, bc_block_edge::BcBlockEdge, bc_cmp_proto::BcCmpProto, bc_function::VmConst,
    bc_op::BcOp, bc_ref::BcRef, call_inliner::CallInliner,
  },
};

impl<'a> CallInliner<'a> {
  pub fn append_cmp_proto(
    &mut self,
    prev_block: &mut BcRef<'a, BcBlock>,
    target_op: BcOp,
    target_proto_id: u32,
  ) {
    let call_block = self.call.base.operator_deref().block;
    {
      let mut cmp_proto = BcCmpProto::<VmConst>::create(self.caller);
      cmp_proto.set_closure(target_op);
      cmp_proto.set_proto_id(target_proto_id);
      cmp_proto.set_fallback(call_block);
      cmp_proto.append_to(prev_block.op);
    }
    prev_block
      .operator_deref_mut()
      .successors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Branch,
        target: call_block,
      });
    self.caller.blocks[call_block.index as usize]
      .predecessors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Branch,
        target: prev_block.op,
      });
  }
}
