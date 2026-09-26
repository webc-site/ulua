use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_rotate(
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
  let vbi = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb);

  let shift = build.inst_ir_cmd_ir_op_ir_op(cmd, vaui, vbi);
  let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, shift);

  builtin_store_number_result(build, ra, arg, value);

  BuiltinImplResult::FULL_ONE_RESULT
}
