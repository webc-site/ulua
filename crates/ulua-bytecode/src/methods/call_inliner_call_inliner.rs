use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  bc_call_fb::BcCallFB,
  bc_function::{BcFunction, VmConst},
  bc_op::BcOp,
  call_inliner::CallInliner,
};

impl<'a> CallInliner<'a> {
  pub fn new(caller: &'a mut BcFunction, target: &'a mut BcFunction, call_op: BcOp) -> Self {
    // cpp `BcCallFB<VmConst> call{caller, call.op()}`：CALLFB 视图只在构造期取实参与
    // 结果寄存器，取完即释放对图的可变借用（图的所有权仍归 `caller` 字段）。
    let call = BcCallFB::<VmConst>::from(caller, call_op);
    let call_params = call.params();
    let target_reg = call.base.get_out_reg();

    CallInliner {
      caller,
      target,
      call_op,
      call_params,
      target_reg,
      caller_blocks_size_before_inline: 0,
      caller_inst_size_before_inline: 0,
      caller_vm_const_size_before_inline: 0,
      caller_proto_size_before_inline: 0,
      caller_up_val_size_before_inline: 0,
      return_ops: Vec::new(),
      return_sites: Vec::new(),
      call_projections: DenseHashSet::new(BcOp::new()),
      var_arg_moves: DenseHashMap::new(BcOp::new()),
      mapped_phis: DenseHashMap::new(BcOp::new()),
    }
  }
}
