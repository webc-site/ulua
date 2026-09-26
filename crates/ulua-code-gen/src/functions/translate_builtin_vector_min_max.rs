use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_store_vector_tvalue_result::builtin_store_vector_tvalue_result,
    check_vec_args::check_vec_args,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_min_max(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  _arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  let arg1 = build.vm_reg(arg as u8);

  if !check_vec_args(build, nparams, 2, nresults, pcpos, &[arg1, args]) {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let value1 = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arg1);
  let value2 = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, args);

  let ret = build.inst_ir_cmd_ir_op_ir_op(cmd, value2, value1);

  builtin_store_vector_tvalue_result(build, ra, ret);

  BuiltinImplResult::FULL_ONE_RESULT
}
