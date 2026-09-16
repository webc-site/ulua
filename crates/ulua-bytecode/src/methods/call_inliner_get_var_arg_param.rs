use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{bc_op::BcOp, call_inliner::CallInliner};

impl<'a> CallInliner<'a> {
  pub fn get_var_arg_param(&self, get_var_args_op: BcOp, idx: u32) -> BcOp {
    let moves = self.var_arg_moves.get(&get_var_args_op);
    LUAU_ASSERT!(moves.is_some() && idx < moves.unwrap().len() as u32);
    moves.unwrap()[idx as usize]
  }
}
