use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_unary(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  // cpp IrTranslateBuiltins.cpp: [[maybe_unused]] args，仅签名与翻译器统一
  _args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  let va = builtin_load_double(build, arg_reg);

  let vaui = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);

  let bin = build.inst_ir_cmd_ir_op(cmd, vaui);

  let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, bin);

  builtin_store_number_result(build, ra, arg, value);

  BuiltinImplResult::FULL_ONE_RESULT
}
