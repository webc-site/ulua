use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder},
};

pub fn translate_builtin_math_unary(
  build: &mut IrBuilder,
  cmd: IrCmd,
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

  let arg_vm_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_vm_reg, pcpos);

  let varg = builtin_load_double(build, arg_vm_reg);
  let result = build.inst_ir_cmd_ir_op(cmd, varg);

  let ra_vm_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_vm_reg, result);

  if ra != arg {
    let t_number = build.const_tag(0x03); // LUA_TNUMBER
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_vm_reg, t_number);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
