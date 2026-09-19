use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{
    bc_block_edge::BcBlockEdge, bc_function::VmConst, bc_load_nil::BcLoadNil, bc_move::BcMove,
    bc_op::BcOp, bc_return::BcReturn, call_inliner::CallInliner,
  },
};

impl<'a> CallInliner<'a> {
  /// cpp `replaceReturn(BcRef<BcBlock>& nextBlock, BcOp callerBlockOp, BcOp targetReturnOp)`：
  /// 把目标图的一条定长 RETURN 换成调用方块里的 MOVE/LOADNIL 序列，并把该块接到
  /// `nextBlock`。入参全部句柄化，可变访问经 `self.caller` 完成。
  pub fn replace_return(
    &mut self,
    next_block_op: BcOp,
    caller_block_op: BcOp,
    target_return_op: BcOp,
  ) -> bool {
    let mut ret = BcReturn::<VmConst>::from(self.target, target_return_op);
    let return_count = ret.return_count();
    if return_count < 0 {
      return false;
    }
    let values = ret.values();

    for (i, &src) in values.iter().enumerate() {
      let src = self.map_to_caller_op(src);
      let mut move_op = BcMove::<VmConst>::create(self.caller);
      move_op.set_src(src);
      move_op.set_out_reg(self.target_reg + i as u8);
      move_op.append_to(caller_block_op);
      let op = move_op.op();
      self.set_return_op(i as u32, op);
    }

    let call_res = self.call_view().return_count();
    LUAU_ASSERT!(call_res >= 0);
    let call_res = call_res as u32;

    for i in values.len() as u32..call_res {
      let mut load_nil = BcLoadNil::<VmConst>::create(self.caller);
      load_nil.set_out_reg(self.target_reg + i as u8);
      load_nil.append_to(caller_block_op);
      let op = load_nil.op();
      self.set_return_op(i, op);
    }

    self
      .caller
      .block_op(caller_block_op)
      .successors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: next_block_op,
      });
    self
      .caller
      .block_op(next_block_op)
      .predecessors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: caller_block_op,
      });

    true
  }
}
