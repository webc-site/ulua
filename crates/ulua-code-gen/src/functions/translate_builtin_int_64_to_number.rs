use crate::{
  enums::ir_cmd::IrCmd,
  functions::builtin_linearop::{
    builtin_check_int_64, builtin_load_int_64, builtin_store_double_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_int_64_to_number(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, arg_reg, pcpos);
  let arg_value = builtin_load_int_64(build, arg_reg);

  let num = build.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, arg_value);
  builtin_store_double_result(build, ra, num);

  BuiltinImplResult::FULL_ONE_RESULT
}
