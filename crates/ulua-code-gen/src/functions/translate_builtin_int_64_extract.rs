use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_cmp::{K_INT64_MAX_SHIFT, K_INT64_WIDTH, builtin_check_int_64_const},
    builtin_linearop::{builtin_check_int_64, builtin_load_int_64, builtin_store_int_64},
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_extract(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);

  let n = builtin_load_int_64(build, vm_reg_arg);
  let f = builtin_load_int_64(build, args);

  let value: IrOp = if nparams == 2 {
    builtin_check_int_64_const(build, f, 0, IrCondition::GreaterEqual, pcpos);
    builtin_check_int_64_const(build, f, K_INT64_MAX_SHIFT, IrCondition::LessEqual, pcpos);

    let shifted = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, n, f);
    let const_1 = build.const_int_64(1);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandInt64, shifted, const_1)
  } else {
    builtin_check_int_64(build, arg3, pcpos);
    let w = builtin_load_int_64(build, arg3);
    let fw = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt64, f, w);

    builtin_check_int_64_const(build, f, 0, IrCondition::GreaterEqual, pcpos);
    builtin_check_int_64_const(build, f, K_INT64_MAX_SHIFT, IrCondition::LessEqual, pcpos);
    builtin_check_int_64_const(build, w, 1, IrCondition::GreaterEqual, pcpos);
    builtin_check_int_64_const(build, w, K_INT64_WIDTH, IrCondition::LessEqual, pcpos);
    builtin_check_int_64_const(build, fw, K_INT64_WIDTH, IrCondition::LessEqual, pcpos);

    let const_64 = build.const_int_64(K_INT64_WIDTH);
    let shift_amount = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, const_64, w);
    let const_minus_1 = build.const_int_64(-1);
    let mask = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, const_minus_1, shift_amount);

    let shifted = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, n, f);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandInt64, shifted, mask)
  };

  builtin_store_int_64(build, ra, value);

  BuiltinImplResult::FULL_ONE_RESULT
}
