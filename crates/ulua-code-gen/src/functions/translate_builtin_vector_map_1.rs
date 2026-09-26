use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_store_vector_result::builtin_store_vector_result, check_vec_args::check_vec_args,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_map_1(
  build: &mut IrBuilder,
  cmd: IrCmd,
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

  let c0 = build.const_int(0);
  let c4 = build.const_int(4);
  let c8 = build.const_int(8);

  let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c0);
  let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c4);
  let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c8);

  let xr = build.inst_ir_cmd_ir_op(cmd, x1);
  let yr = build.inst_ir_cmd_ir_op(cmd, y1);
  let zr = build.inst_ir_cmd_ir_op(cmd, z1);

  builtin_store_vector_result(build, ra, xr, yr, zr);

  BuiltinImplResult::FULL_ONE_RESULT
}
