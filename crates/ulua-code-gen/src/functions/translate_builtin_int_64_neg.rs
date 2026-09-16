use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_int_64::builtin_check_int_64, builtin_load_int_64::builtin_load_int_64,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_int_64_neg(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams != 1 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let ra_op = build.vm_reg(ra as u8);
  let arg_op = build.vm_reg(arg as u8);

  builtin_check_int_64(build, arg_op, pcpos);

  let va = builtin_load_int_64(build, arg_op);
  let zero = build.const_int_64(0);
  let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, zero, va);

  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, ra_op, result);
  let tag = build.const_tag(LuaType::Integer as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_op, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
