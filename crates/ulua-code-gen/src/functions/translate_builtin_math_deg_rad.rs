use core::f64::consts::PI;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_deg_rad(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);

  let rpd = PI / 180.0_f64;

  let varg = builtin_load_double(build, arg_reg);
  let rpd_op = build.const_double(rpd);
  let value = build.inst_ir_cmd_ir_op_ir_op(cmd, varg, rpd_op);
  builtin_store_number_result(build, ra, arg, value);

  BuiltinImplResult::FULL_ONE_RESULT
}
