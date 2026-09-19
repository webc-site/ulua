use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_int_64::builtin_check_int_64, builtin_load_int_64::builtin_load_int_64,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_compare(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
  cond: IrCondition,
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

  let va = builtin_load_int_64(build, vm_reg_arg);
  let vb = builtin_load_int_64(build, args);

  let cond_op = build.cond(cond);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpInt64, va, vb, cond_op);

  let vm_reg_ra = build.vm_reg(ra as u8);
  let tag_value = build.const_tag(1); // LUA_TBOOLEAN
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, vm_reg_ra, result);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, tag_value);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
