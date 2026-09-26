use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_cmp::builtin_check_cmp,
    builtin_linearop::{builtin_check_double, builtin_load_double, builtin_store_int_64},
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};
pub fn translate_builtin_int_64_create(
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
  builtin_check_double(build, arg_reg, pcpos);

  let arg_value = builtin_load_double(build, arg_reg);

  let integer_value = build.inst_ir_cmd_ir_op(IrCmd::NumToInt64, arg_value);
  let back_to_double = build.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, integer_value);

  builtin_check_cmp(
    build,
    IrCmd::CheckCmpNum,
    back_to_double,
    arg_value,
    IrCondition::Equal,
    pcpos,
  );

  builtin_store_int_64(build, ra, integer_value);

  BuiltinImplResult::FULL_ONE_RESULT
}
