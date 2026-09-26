use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_linearop::{builtin_check_int_64, builtin_load_int_64, builtin_store_int_64},
    unrolled_args::check_unrolled,
    vm_reg_op::vm_reg_op,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_min_max(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
  min: bool,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  check_unrolled(build, arg, args, arg3, nparams, pcpos, builtin_check_int_64);

  let vm_reg_arg = build.vm_reg(arg as u8);
  let va = builtin_load_int_64(build, vm_reg_arg);
  let vb = builtin_load_int_64(build, args);

  let cond = if min {
    build.cond(IrCondition::LessEqual)
  } else {
    build.cond(IrCondition::Greater)
  };

  // vb < va ? vb : va
  let mut select_op =
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectInt64, va, vb, vb, va, cond);

  if nparams >= 3 {
    let vc = builtin_load_int_64(build, arg3);
    select_op = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::SelectInt64,
      vc,
      select_op,
      select_op,
      vc,
      cond,
    );
  }

  for i in 4..=nparams {
    let reg_idx = vm_reg_op(args) + (i - 2);
    let reg = build.vm_reg(reg_idx as u8);
    let vc = builtin_load_int_64(build, reg);

    select_op = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::SelectInt64,
      vc,
      select_op,
      select_op,
      vc,
      cond,
    );
  }

  builtin_store_int_64(build, ra, select_op);

  BuiltinImplResult::FULL_ONE_RESULT
}
