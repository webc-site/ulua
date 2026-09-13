use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_int_64::builtin_check_int_64, builtin_load_int_64::builtin_load_int_64,
    vm_reg_op::vm_reg_op,
  },
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
  },
};

pub fn translate_builtin_int_64_multiarg_op(
  build: &mut IrBuilder,
  cmd: IrCmd,
  btest: bool,
  identity: i64,
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

  const K_INT64_BINARY_OP_UNROLLED_PARAMS: i32 = 5;

  if nparams > K_INT64_BINARY_OP_UNROLLED_PARAMS || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  if nparams == 0 {
    let vm_reg_ra = build.vm_reg(ra as u8);
    if btest {
      let const_int_1 = build.const_int(1);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, vm_reg_ra, const_int_1);
      let const_tag_bool = build.const_tag(1); // LUA_TBOOLEAN
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, const_tag_bool);
    } else {
      let const_int64_identity = build.const_int_64(identity);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, vm_reg_ra, const_int64_identity);
      let const_tag_int = build.const_tag(LuaType::Integer as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, const_tag_int);
    }
    return BuiltinImplResult {
      r#type: BuiltinImplType::Full,
      actual_result_count: 1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);

  if nparams >= 2 {
    builtin_check_int_64(build, args, pcpos);
  }

  if nparams >= 3 {
    builtin_check_int_64(build, arg3, pcpos);
  }

  for i in 4..=nparams {
    let base_reg = vm_reg_op(args);
    let reg = build.vm_reg((base_reg + (i - 2)) as u8);
    builtin_check_int_64(build, reg, pcpos);
  }

  let mut res = builtin_load_int_64(build, vm_reg_arg);

  if nparams >= 2 {
    let vb = builtin_load_int_64(build, args);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, vb);
  }

  if nparams >= 3 {
    let vc = builtin_load_int_64(build, arg3);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, vc);
  }

  for i in 4..=nparams {
    let base_reg = vm_reg_op(args);
    let reg = build.vm_reg((base_reg + (i - 2)) as u8);
    let vc = builtin_load_int_64(build, reg);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, res, vc);
  }

  let vm_reg_ra = build.vm_reg(ra as u8);
  if btest {
    let const_int64_0 = build.const_int_64(0);
    let cond = build.cond(IrCondition::NotEqual);
    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt64, res, const_int64_0, cond);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, vm_reg_ra, result);
    let const_tag_bool = build.const_tag(1); // LUA_TBOOLEAN
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, const_tag_bool);
  } else {
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, vm_reg_ra, res);
    let const_tag_int = build.const_tag(LuaType::Integer as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, const_tag_int);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
