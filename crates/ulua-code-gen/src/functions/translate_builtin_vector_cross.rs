use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_store_vector_result::builtin_store_vector_result, check_vec_args::check_vec_args,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_cross(
  build: &mut IrBuilder,
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

  let c0 = build.const_int(0);
  let c4 = build.const_int(4);
  let c8 = build.const_int(8);

  let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c0);
  let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, c0);

  let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c4);
  let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, c4);

  let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c8);
  let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, c8);

  let y1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, z2);
  let z1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, y2);
  let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, y1z2, z1y2);

  let z1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, x2);
  let x1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, z2);
  let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, z1x2, x1z2);

  let x1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, y2);
  let y1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, x2);
  let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, x1y2, y1x2);

  builtin_store_vector_result(build, ra, xr, yr, zr);

  BuiltinImplResult::FULL_ONE_RESULT
}
