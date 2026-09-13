use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_condition::IrCondition, ir_op_kind::IrOpKind,
  },
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_clamp(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  fallback: IrOp,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 3 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let block = build.block(IrBlockKind::Internal);
  CODEGEN_ASSERT!(args.kind() == IrOpKind::VmReg);

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  builtin_check_double(build, args, pcpos);
  builtin_check_double(build, arg3, pcpos);

  let min = builtin_load_double(build, args);
  let max = builtin_load_double(build, arg3);
  let cond = build.cond(IrCondition::NotLessEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpNum,
    min,
    max,
    cond,
    fallback,
    block,
  );
  build.begin_block(block);

  let arg_reg = build.vm_reg(arg as u8);
  let v = builtin_load_double(build, arg_reg);
  let r = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxNum, min, v);
  let clamped = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, max, r);

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, clamped);

  if ra != arg {
    let ra_reg = build.vm_reg(ra as u8);
    let tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::UsesFallback,
    actual_result_count: 1,
  }
}
