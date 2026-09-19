use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{
    bc_block_edge::BcBlockEdge, bc_cmp_proto::BcCmpProto, bc_function::VmConst, bc_op::BcOp,
    call_inliner::CallInliner,
  },
};

impl<'a> CallInliner<'a> {
  /// cpp `appendCmpProto(BcRef<BcBlock>& prevBlock, BcOp targetOp, uint32_t targetProtoId)`：
  /// 在 `prevBlock` 末尾追加一条 CMPProto 作为回退分支，并与调用块互连成边。
  /// 入参改为块句柄 `BcOp`（`BcRef` 已只读）。
  pub fn append_cmp_proto(&mut self, prev_block_op: BcOp, target_op: BcOp, target_proto_id: u32) {
    let call_block = self.call_block_op();
    {
      let mut cmp_proto = BcCmpProto::<VmConst>::create(self.caller);
      cmp_proto.set_closure(target_op);
      cmp_proto.set_proto_id(target_proto_id);
      cmp_proto.set_fallback(call_block);
      cmp_proto.append_to(prev_block_op);
    }
    self
      .caller
      .block_op(prev_block_op)
      .successors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Branch,
        target: call_block,
      });
    self
      .caller
      .block_op(call_block)
      .predecessors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Branch,
        target: prev_block_op,
      });
  }
}
