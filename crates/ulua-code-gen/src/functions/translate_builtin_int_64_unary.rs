use crate::{
  enums::ir_cmd::IrCmd,
  functions::builtin_linearop::{builtin_check_int_64, builtin_load_int_64, builtin_store_int_64},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_int_64_unary(
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

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);

  let va = builtin_load_int_64(build, vm_reg_arg);
  let result = build.inst_ir_cmd_ir_op(cmd, va);

  builtin_store_int_64(build, ra, result);

  BuiltinImplResult::FULL_ONE_RESULT
}
