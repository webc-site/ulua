use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_rotate(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_double(build, vm_reg_arg, pcpos);
  builtin_check_double(build, args, pcpos);

  let va = builtin_load_double(build, vm_reg_arg);
  let vb = builtin_load_double(build, args);

  let vaui = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);
  let vbi = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb);

  let shift = build.inst_ir_cmd_ir_op_ir_op(cmd, vaui, vbi);
  let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, shift);

  let vm_reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, vm_reg_ra, value);

  if ra != arg {
    let tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, tag);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
