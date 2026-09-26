use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_store_vector_tvalue_result::builtin_store_vector_tvalue_result,
    check_vec_args::check_vec_args,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_normalize(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  _arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  let arg1 = build.vm_reg(arg as u8);

  if !check_vec_args(build, nparams, 1, nresults, pcpos, &[arg1]) {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let zero = build.const_int(0);
  let a = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, arg1, zero);
  let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DotVec, a, a);

  let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
  let one = build.const_double(1.0);
  let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivFloat, one, mag);
  let invvec = build.inst_ir_cmd_ir_op(IrCmd::FloatToVec, inv);

  let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulVec, a, invvec);
  builtin_store_vector_tvalue_result(build, ra, result);

  BuiltinImplResult::FULL_ONE_RESULT
}
