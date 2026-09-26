use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_cmp::{K_UINT32_WIDTH, builtin_check_int_const},
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_shift(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_double(build, vm_reg_arg, pcpos);
  builtin_check_double(build, args, pcpos);

  let va = builtin_load_double(build, vm_reg_arg);
  let vb = builtin_load_double(build, args);

  let vaui = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);

  let vbi = if let Some(vbd) = build.function.as_double_op(vb) {
    if vbd >= i32::MIN as f64 && vbd <= i32::MAX as f64 {
      build.const_int(vbd as i32)
    } else {
      build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb)
    }
  } else {
    build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb)
  };

  let known_good_shift = if let Some(val) = build.function.as_int_op(vbi) {
    (val as u32) < K_UINT32_WIDTH as u32
  } else {
    false
  };

  if !known_good_shift {
    builtin_check_int_const(build, vbi, K_UINT32_WIDTH, IrCondition::UnsignedLess, pcpos);
  }

  let shift = build.inst_ir_cmd_ir_op_ir_op(cmd, vaui, vbi);
  let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, shift);
  builtin_store_number_result(build, ra, arg, value);

  BuiltinImplResult::FULL_ONE_RESULT
}
