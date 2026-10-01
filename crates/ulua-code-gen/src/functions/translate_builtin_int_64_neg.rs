use crate::{
  enums::ir_cmd::IrCmd,
  functions::builtin_linearop::{builtin_check_int_64, builtin_load_int_64, builtin_store_int_64},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_int_64_neg(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams != 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_op = build.vm_reg(arg as u8);

  builtin_check_int_64(build, arg_op, pcpos);

  let va = builtin_load_int_64(build, arg_op);
  let zero = build.const_int_64(0);
  let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, zero, va);

  builtin_store_int_64(build, ra, result);

  BuiltinImplResult::FULL_ONE_RESULT
}
