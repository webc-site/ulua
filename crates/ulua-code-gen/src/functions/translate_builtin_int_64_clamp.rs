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

pub fn translate_builtin_int_64_clamp(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 3 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);
  let third_arg = if FFlag::LuauCodegenIntegerArg3Fix.get() {
    arg3
  } else {
    let vm_reg_op_args = vm_reg_op(args);
    build.vm_reg((vm_reg_op_args + 1) as u8)
  };
  builtin_check_int_64(build, third_arg, pcpos);

  let val = builtin_load_int_64(build, vm_reg_arg);
  let mi = builtin_load_int_64(build, args);
  let mx = builtin_load_int_64(build, third_arg);

  // guard: min <= max
  let cond_op = build.cond(IrCondition::LessEqual);
  let vm_exit_op = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_initializer_list_ir_op(IrCmd::CheckCmpInt64, &[mi, mx, cond_op, vm_exit_op]);

  // clamp: if val < min, use min; then if result > max, use max
  let cond_less = build.cond(IrCondition::Less);
  let clamped =
    build.inst_ir_cmd_initializer_list_ir_op(IrCmd::SelectInt64, &[val, mi, val, mi, cond_less]);

  let cond_greater = build.cond(IrCondition::Greater);
  let result = build.inst_ir_cmd_initializer_list_ir_op(
    IrCmd::SelectInt64,
    &[clamped, mx, clamped, mx, cond_greater],
  );

  let vm_reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_initializer_list_ir_op(IrCmd::StoreInt64, &[vm_reg_ra, result]);
  let tag_op = build.const_tag(LuaType::Integer as u8);
  build.inst_ir_cmd_initializer_list_ir_op(IrCmd::StoreTag, &[vm_reg_ra, tag_op]);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
