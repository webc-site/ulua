use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
    unrolled_args::{check_unrolled, fold_unrolled},
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_min_max(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  const K_MIN_MAX_UNROLLED_PARAMS: i32 = 5;

  if !(2..=K_MIN_MAX_UNROLLED_PARAMS).contains(&nparams) || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  check_unrolled(build, arg, args, arg3, nparams, pcpos, builtin_check_double);

  // 折叠方向 combine(next, acc) 与 cpp 一致：首步 cmd(varg2, varg1)、
  // 其后 cmd(新参, res)
  let res = fold_unrolled(
    build,
    arg,
    args,
    arg3,
    nparams,
    builtin_load_double,
    |b, acc, next| b.inst_ir_cmd_ir_op_ir_op(cmd, next, acc),
  );

  builtin_store_number_result(build, ra, arg, res);

  BuiltinImplResult::FULL_ONE_RESULT
}
