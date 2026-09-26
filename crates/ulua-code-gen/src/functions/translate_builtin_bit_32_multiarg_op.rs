// 来自 C++ 源文件的文件级常量

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double, builtin_store_bool_result},
    builtin_store_number_result::builtin_store_number_result,
    unrolled_args::{check_unrolled, fold_unrolled},
  },
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
    ir_op::IrOp,
  },
};
const K_BIT32_BINARY_OP_UNROLLED_PARAMS: i32 = 5;

/// bit32 展开折叠的 per-参变换：double 装载后转 uint
fn load_uint(build: &mut IrBuilder, op: IrOp) -> IrOp {
  let v = builtin_load_double(build, op);
  build.inst_ir_cmd_ir_op(IrCmd::NumToUint, v)
}

pub fn translate_builtin_bit_32_multiarg_op(
  build: &mut IrBuilder,
  cmd: IrCmd,
  btest: bool,
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

  if !(1..=K_BIT32_BINARY_OP_UNROLLED_PARAMS).contains(&nparams) || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  check_unrolled(build, arg, args, arg3, nparams, pcpos, builtin_check_double);

  let res = fold_unrolled(build, arg, args, arg3, nparams, load_uint, |b, acc, v| {
    b.inst_ir_cmd_ir_op_ir_op(cmd, acc, v)
  });

  if btest {
    let zero = build.const_int(0);
    let cond = build.cond(IrCondition::NotEqual);
    let value = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt, res, zero, cond);
    builtin_store_bool_result(build, ra, value);
  } else {
    let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, res);
    builtin_store_number_result(build, ra, arg, value);
  }

  BuiltinImplResult::FULL_ONE_RESULT
}
