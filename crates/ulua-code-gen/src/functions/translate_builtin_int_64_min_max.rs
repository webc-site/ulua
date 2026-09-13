use ulua_common::FFlag;
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_int_64::builtin_check_int_64, builtin_load_int_64::builtin_load_int_64,
    vm_reg_op::vm_reg_op,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_min_max(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
  min: bool,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);

  let luau_codegen_integer_arg3_fix = FFlag::LuauCodegenIntegerArg3Fix.get();

  if luau_codegen_integer_arg3_fix {
    if nparams >= 3 {
      builtin_check_int_64(build, arg3, pcpos);
    }

    for i in 4..=nparams {
      let reg_idx = vm_reg_op(args) + (i - 2);
      let reg = build.vm_reg(reg_idx as u8);
      builtin_check_int_64(build, reg, pcpos);
    }
  }

  let va = builtin_load_int_64(build, vm_reg_arg);
  let vb = builtin_load_int_64(build, args);

  let cond = if min {
    build.cond(IrCondition::LessEqual)
  } else {
    build.cond(IrCondition::Greater)
  };

  let mut select_op =
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectInt64, va, vb, vb, va, cond);

  if luau_codegen_integer_arg3_fix && nparams >= 3 {
    let vc = builtin_load_int_64(build, arg3);
    select_op = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::SelectInt64,
      vc,
      select_op,
      select_op,
      vc,
      cond,
    );
  }

  let start_idx = if luau_codegen_integer_arg3_fix { 4 } else { 3 };
  for i in start_idx..=nparams {
    if !luau_codegen_integer_arg3_fix {
      let reg_idx = vm_reg_op(args) + (i - 2);
      let reg = build.vm_reg(reg_idx as u8);
      builtin_check_int_64(build, reg, pcpos);
    }

    let reg_idx = vm_reg_op(args) + (i - 2);
    let reg = build.vm_reg(reg_idx as u8);
    let vc = builtin_load_int_64(build, reg);

    select_op = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::SelectInt64,
      vc,
      select_op,
      select_op,
      vc,
      cond,
    );
  }

  let vm_reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, vm_reg_ra, select_op);
  let tag = build.const_tag(LuaType::Integer as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
