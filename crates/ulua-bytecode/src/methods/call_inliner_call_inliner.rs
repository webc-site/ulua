use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  bc_call_fb::BcCallFB, bc_function::BcFunction, bc_op::BcOp, call_inliner::CallInliner,
};

impl<'a> CallInliner<'a> {
  pub fn new(caller: &'a mut BcFunction, target: &'a mut BcFunction, call_op: BcOp) -> Self {
    let caller_ptr: *mut BcFunction = caller;
    let call_ref = unsafe { (&*caller_ptr).inst(call_op) };
    let call = unsafe { BcCallFB::from(caller_ptr, call_ref) };
    let call_params = call.params();
    let target_reg = call.base.get_out_reg();

    CallInliner {
      caller,
      target,
      call,
      call_params,
      target_reg,
      caller_blocks_size_before_inline: 0,
      caller_inst_size_before_inline: 0,
      caller_vm_const_size_before_inline: 0,
      caller_proto_size_before_inline: 0,
      caller_up_val_size_before_inline: 0,
      return_ops: Vec::new(),
      call_projections: DenseHashSet::new(BcOp::new()),
      var_arg_moves: DenseHashMap::new(BcOp::new()),
    }
  }
}
