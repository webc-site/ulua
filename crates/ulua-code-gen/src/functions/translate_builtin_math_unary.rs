use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_math_unary(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_vm_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_vm_reg, pcpos);

  let varg = builtin_load_double(build, arg_vm_reg);
  let result = build.inst_ir_cmd_ir_op(cmd, varg);

  builtin_store_number_result(build, ra, arg, result);

  BuiltinImplResult::FULL_ONE_RESULT
}
