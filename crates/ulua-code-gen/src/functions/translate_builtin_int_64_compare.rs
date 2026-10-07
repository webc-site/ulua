use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::builtin_linearop::{
    builtin_check_int_64, builtin_load_int_64, builtin_store_bool_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_compare(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
  cond: IrCondition,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);

  let va = builtin_load_int_64(build, vm_reg_arg);
  let vb = builtin_load_int_64(build, args);

  let cond_op = build.cond(cond);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt64, va, vb, cond_op);

  builtin_store_bool_result(build, ra, result);

  BuiltinImplResult::FULL_ONE_RESULT
}
