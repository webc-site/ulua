use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_int_64::builtin_check_int_64, builtin_load_int_64::builtin_load_int_64,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_int_64_to_number(
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
  builtin_check_int_64(build, arg_reg, pcpos);
  let arg_value = builtin_load_int_64(build, arg_reg);

  let ra_reg = build.vm_reg(ra as u8);
  let num = build.inst_ir_cmd_ir_op(IrCmd::Int64ToNum, arg_value);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, num);

  let tag = build.const_tag(3); // LUA_TNUMBER
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
