use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
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
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  let vm_exit = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(vm_reg_arg, LuaType::String as u8, vm_exit);

  let vm_reg_arg_for_load = build.vm_reg(arg as u8);
  let ts = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_arg_for_load);
  let len = build.inst_ir_cmd_ir_op(IrCmd::StringLen, ts);

  let ra_reg = build.vm_reg(ra as u8);
  let len_num = build.inst_ir_cmd_ir_op(IrCmd::IntToNum, len);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, len_num);
  let tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
