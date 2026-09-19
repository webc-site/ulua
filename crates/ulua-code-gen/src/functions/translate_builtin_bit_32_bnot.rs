use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_bnot(
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

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  let va = builtin_load_double(build, arg_reg);

  let vaui = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);
  let not_ = build.inst_ir_cmd_ir_op(IrCmd::BitnotUint, vaui);
  let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, not_);

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);

  if ra != arg {
    let tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
