use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_cmp::builtin_check_cmp,
    builtin_linearop::{builtin_check_int_64, builtin_load_int_64, builtin_store_int_64},
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_clamp(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 3 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);
  builtin_check_int_64(build, arg3, pcpos);

  let val = builtin_load_int_64(build, vm_reg_arg);
  let mi = builtin_load_int_64(build, args);
  let mx = builtin_load_int_64(build, arg3);

  // guard：min <= max
  builtin_check_cmp(
    build,
    IrCmd::CheckCmpInt64,
    mi,
    mx,
    IrCondition::LessEqual,
    pcpos,
  );

  // clamp：val < min 时取 min；结果 > max 时再取 max
  let cond_less = build.cond(IrCondition::Less);
  let clamped =
    build.inst_ir_cmd_initializer_list_ir_op(IrCmd::SelectInt64, &[val, mi, val, mi, cond_less]);

  let cond_greater = build.cond(IrCondition::Greater);
  let result = build.inst_ir_cmd_initializer_list_ir_op(
    IrCmd::SelectInt64,
    &[clamped, mx, clamped, mx, cond_greater],
  );

  builtin_store_int_64(build, ra, result);

  BuiltinImplResult::FULL_ONE_RESULT
}
