use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{
    bc_block::BcBlock,
    bc_block_edge::BcBlockEdge,
    bc_function::{BcFunction, VmConst},
    bc_load_nil::BcLoadNil,
    bc_move::BcMove,
    bc_op::BcOp,
    bc_ref::BcRef,
    bc_return::BcReturn,
    call_inliner::CallInliner,
  },
};

impl<'a> CallInliner<'a> {
  pub fn replace_return(
    &mut self,
    next_block: &mut BcRef<'a, BcBlock>,
    caller_block_op: BcOp,
    target_return_op: BcOp,
  ) -> bool {
    let target = self.target as *mut BcFunction;
    let target_return_ref = unsafe { (&*target).inst(target_return_op) };
    let mut ret = unsafe { BcReturn::<VmConst>::from(target, target_return_ref) };
    let return_count = ret.return_count();
    if return_count < 0 {
      return false;
    }
    let values = ret.values();

    let mut i = 0;
    while i < values.len() as u32 {
      let src = self.map_to_caller_op(values[i as usize]);
      let mut move_op = BcMove::<VmConst>::create(self.caller);
      move_op.set_src(src);
      move_op.set_out_reg(self.target_reg + i as u8);
      move_op.append_to(caller_block_op);
      let op = move_op.op();
      self.set_return_op(i, op);
      i += 1;
    }

    let call_res = self.call.return_count();
    LUAU_ASSERT!(call_res >= 0);
    let call_res = call_res as u32;

    while i < call_res {
      let mut load_nil = BcLoadNil::<VmConst>::create(self.caller);
      load_nil.set_out_reg(self.target_reg + i as u8);
      load_nil.append_to(caller_block_op);
      let op = load_nil.op();
      self.set_return_op(i, op);
      i += 1;
    }

    let mut caller_block = self.caller.block(caller_block_op);
    caller_block
      .operator_deref_mut()
      .successors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: next_block.op,
      });
    next_block
      .operator_deref_mut()
      .predecessors
      .push_back(BcBlockEdge {
        kind: BcBlockEdgeKind::Fallthrough,
        target: caller_block_op,
      });

    true
  }
}
