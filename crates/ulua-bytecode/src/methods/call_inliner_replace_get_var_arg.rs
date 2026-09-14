use core::cmp;

use crate::{
  records::{
    bc_function::{BcFunction, VmConst},
    bc_get_var_args::BcGetVarArgs,
    bc_load_nil::BcLoadNil,
    bc_move::BcMove,
    bc_op::BcOp,
    call_inliner::CallInliner,
  },
  type_aliases::reg::Reg,
};

impl<'a> CallInliner<'a> {
  pub fn replace_get_var_arg(&mut self, caller_block_op: BcOp, target_get_var_args_op: BcOp) {
    let target = self.target as *mut BcFunction;
    let target_get_var_args_ref = unsafe { (&*target).inst(target_get_var_args_op) };
    let mut get_var_args =
      unsafe { BcGetVarArgs::<VmConst>::from(target, target_get_var_args_ref) };
    let count = get_var_args.values_count();
    let count = if count < 0 {
      cmp::max(
        0,
        self.call_params.len() as i32 - self.target.numparams as i32,
      ) as usize
    } else {
      count as usize
    };

    // 迭代器生成 moves，替代 C 风格索引循环
    let moves = (0..count)
      .map(|i| {
        let target_reg = get_var_args.start_reg() as u32 + i as u32;
        let caller_reg = self.map_to_caller_reg(target_reg as Reg) as Reg;

        if (self.target.numparams as usize + i) < self.call_params.len() {
          let mut move_op = BcMove::<VmConst>::create(self.caller);
          move_op.set_src(self.call_params[self.target.numparams as usize + i]);
          move_op.set_out_reg(caller_reg);
          move_op.append_to(caller_block_op);
          move_op.op()
        } else {
          let mut load_nil = BcLoadNil::<VmConst>::create(self.caller);
          load_nil.set_out_reg(caller_reg);
          load_nil.append_to(caller_block_op);
          load_nil.op()
        }
      })
      .collect::<Vec<_>>();

    self.var_arg_moves.try_insert(target_get_var_args_op, moves);
  }
}
