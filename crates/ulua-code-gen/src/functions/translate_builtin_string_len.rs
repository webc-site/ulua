use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::builtin_linearop::builtin_store_double_result,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_string_len(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  let vm_exit = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(vm_reg_arg, LuaType::String as u8, vm_exit);

  let vm_reg_arg_for_load = build.vm_reg(arg as u8);
  let ts = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_arg_for_load);
  let len = build.inst_ir_cmd_ir_op(IrCmd::StringLen, ts);

  let len_num = build.inst_ir_cmd_ir_op(IrCmd::IntToNum, len);
  builtin_store_double_result(build, ra, len_num);

  BuiltinImplResult::FULL_ONE_RESULT
}
