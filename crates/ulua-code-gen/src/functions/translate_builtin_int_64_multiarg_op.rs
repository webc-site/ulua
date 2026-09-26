use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_linearop::{
      builtin_check_int_64, builtin_load_int_64, builtin_store_bool_result, builtin_store_int_64,
    },
    unrolled_args::{check_unrolled, fold_unrolled},
  },
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
  },
};

pub fn translate_builtin_int_64_multiarg_op(
  build: &mut IrBuilder,
  cmd: IrCmd,
  btest: bool,
  identity: i64,
  bargs: BuiltinArgs,
) -> BuiltinImplResult {
  let BuiltinArgs {
    ra,
    arg,
    args,
    arg3,
    nparams,
    nresults,
    pcpos,
  } = bargs;

  const K_INT64_BINARY_OP_UNROLLED_PARAMS: i32 = 5;

  if nparams > K_INT64_BINARY_OP_UNROLLED_PARAMS || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  if nparams == 0 {
    if btest {
      let const_int_1 = build.const_int(1);
      builtin_store_bool_result(build, ra, const_int_1);
    } else {
      let const_int64_identity = build.const_int_64(identity);
      builtin_store_int_64(build, ra, const_int64_identity);
    }
    return BuiltinImplResult::FULL_ONE_RESULT;
  }

  check_unrolled(build, arg, args, arg3, nparams, pcpos, builtin_check_int_64);

  let res = fold_unrolled(
    build,
    arg,
    args,
    arg3,
    nparams,
    builtin_load_int_64,
    |b, acc, v| b.inst_ir_cmd_ir_op_ir_op(cmd, acc, v),
  );

  if btest {
    let const_int64_0 = build.const_int_64(0);
    let cond = build.cond(IrCondition::NotEqual);
    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt64, res, const_int64_0, cond);
    builtin_store_bool_result(build, ra, result);
  } else {
    builtin_store_int_64(build, ra, res);
  }

  BuiltinImplResult::FULL_ONE_RESULT
}
