use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn translate_builtin_vector_dot(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  let arg1 = build.vm_reg(arg as u8);

  if nparams != 2
    || nresults > 1
    || arg1.kind() == IrOpKind::Constant
    || args.kind() == IrOpKind::Constant
  {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let fallback = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(arg1, LuaType::Vector as u8, fallback);
  build.load_and_check_tag(args, LuaType::Vector as u8, fallback);

  let a = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arg1);
  let b = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, args);

  let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DotVec, a, b);
  let sum = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, sum);

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, sum);
  let tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  let _ = arg3;

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
