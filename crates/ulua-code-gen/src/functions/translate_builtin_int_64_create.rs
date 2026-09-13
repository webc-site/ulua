use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};
pub fn translate_builtin_int_64_create(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);

  let arg_value = builtin_load_double(build, arg_reg);

  let integer_value = build.inst_ir_cmd_ir_op(IrCmd::NumToInt64, arg_value);
  let back_to_double = build.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, integer_value);

  let cond = build.cond(IrCondition::Equal);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::CheckCmpNum,
    back_to_double,
    arg_value,
    cond,
    exit,
  );

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, ra_reg, integer_value);

  let tag = build.const_tag(LuaType::Integer as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
