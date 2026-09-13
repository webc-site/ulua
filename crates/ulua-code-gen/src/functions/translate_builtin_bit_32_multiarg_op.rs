// File-scope constant from C++ source
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
    vm_reg_op::vm_reg_op,
  },
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
  },
};
const K_BIT32_BINARY_OP_UNROLLED_PARAMS: i32 = 5;

pub fn translate_builtin_bit_32_multiarg_op(
  build: &mut IrBuilder,
  cmd: IrCmd,
  btest: bool,
  bargs: BuiltinArgs,
) -> BuiltinImplResult {
  let BuiltinArgs {
    ra,
    arg,
    args,
    arg3,
    nparams,
    nresults,
    pcpos,
  } = bargs;

  if !(1..=K_BIT32_BINARY_OP_UNROLLED_PARAMS).contains(&nparams) || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_double(build, vm_reg_arg, pcpos);

  if nparams >= 2 {
    builtin_check_double(build, args, pcpos);
  }

  if nparams >= 3 {
    builtin_check_double(build, arg3, pcpos);
  }

  for i in 4..=nparams {
    let reg_index = vm_reg_op(args) + (i - 2);
    let vm_reg = build.vm_reg(reg_index as u8);
    builtin_check_double(build, vm_reg, pcpos);
  }

  let va = builtin_load_double(build, vm_reg_arg);
  let mut res = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);

  if nparams >= 2 {
    let vb = builtin_load_double(build, args);
    let arg_op = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, vb);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, arg_op);
  }

  if nparams >= 3 {
    let vc = builtin_load_double(build, arg3);
    let arg_op = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, vc);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, arg_op);
  }

  for i in 4..=nparams {
    let reg = build.vm_reg((vm_reg_op(args) + (i - 2)) as u8);
    let vc = builtin_load_double(build, reg);
    let arg_op = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, vc);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, arg_op);
  }

  if btest {
    let zero = build.const_int(0);
    let cond = build.cond(IrCondition::NotEqual);
    let value = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt, res, zero, cond);
    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_reg, value);
    let t_boolean = build.const_tag(0x01); // LUA_TBOOLEAN
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, t_boolean);
  } else {
    let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, res);
    let ra_reg = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);

    if ra != arg {
      let t_number = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, t_number);
    }
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
