use crate::{
  enums::ir_cmd::IrCmd,
  functions::{builtin_linearop::builtin_store_double_result, check_vec_args::check_vec_args},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn translate_builtin_vector_dot(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  // cpp translateVectorDot: [[maybe_unused]] arg3（仅 x64 路径使用）
  _arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  let arg1 = build.vm_reg(arg as u8);

  if !check_vec_args(build, nparams, 2, nresults, pcpos, &[arg1, args]) {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let a = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arg1);
  let b = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, args);

  let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DotVec, a, b);
  let sum = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, sum);

  builtin_store_double_result(build, ra, sum);

  BuiltinImplResult::FULL_ONE_RESULT
}
