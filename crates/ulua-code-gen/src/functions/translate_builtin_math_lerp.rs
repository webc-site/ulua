use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_lerp(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  _fallback: IrOp,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 3 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_op = build.vm_reg(arg as u8);

  builtin_check_double(build, arg_op, pcpos);
  builtin_check_double(build, args, pcpos);
  builtin_check_double(build, arg3, pcpos);

  let a = builtin_load_double(build, arg_op);
  let b = builtin_load_double(build, args);
  let t = builtin_load_double(build, arg3);

  let sub_ba = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubNum, b, a);
  let l = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::MuladdNum, sub_ba, t, a);
  let one = build.const_double(1.0);
  let r = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectNum, l, b, t, one);

  builtin_store_number_result(build, ra, arg, r);

  BuiltinImplResult::FULL_ONE_RESULT
}
