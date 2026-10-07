use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::builtin_linearop::{builtin_check_double, builtin_load_double},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_is_nan(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_double(build, vm_reg_arg, pcpos);

  let varg = builtin_load_double(build, vm_reg_arg);

  let t_number = build.const_tag(LuaType::Number as u8);
  let t_boolean = build.const_tag(LuaType::Boolean as u8);

  let cond_op = build.cond(IrCondition::NotEqual);

  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::CmpSplitTvalue,
    t_number,
    t_number,
    varg,
    varg,
    cond_op,
  );

  let vm_reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, vm_reg_ra, result);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, t_boolean);

  BuiltinImplResult::FULL_ONE_RESULT
}
