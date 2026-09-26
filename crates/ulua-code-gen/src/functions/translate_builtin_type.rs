use crate::{
  enums::ir_cmd::IrCmd,
  functions::builtin_linearop::builtin_store_string_result,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_type(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  // cpp IrTranslateBuiltins.cpp: [[maybe_unused]] args，仅签名与翻译器统一
  _args: IrOp,
  nresults: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  let tag = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_arg);
  let name = build.inst_ir_cmd_ir_op(IrCmd::GetType, tag);

  builtin_store_string_result(build, ra, name);

  BuiltinImplResult::FULL_ONE_RESULT
}
