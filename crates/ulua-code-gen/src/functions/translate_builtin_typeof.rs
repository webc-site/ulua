use crate::{
  enums::ir_cmd::IrCmd,
  functions::builtin_linearop::builtin_store_string_result,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_typeof(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  nresults: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  let name = build.inst_ir_cmd_ir_op(IrCmd::GetTypeof, vm_reg_arg);

  builtin_store_string_result(build, ra, name);

  BuiltinImplResult::FULL_ONE_RESULT
}
