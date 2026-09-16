use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_normalize(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  _arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  let arg1 = build.vm_reg(arg as u8);

  if nparams != 1 || nresults > 1 || arg1.kind() == IrOpKind::Constant {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let fallback = build.vm_exit(pcpos as u32);
  let tag_vector = LuaType::Vector as u8;
  build.load_and_check_tag(arg1, tag_vector, fallback);

  let zero = build.const_int(0);
  let a = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, arg1, zero);
  let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DotVec, a, a);

  let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
  let one = build.const_double(1.0);
  let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivFloat, one, mag);
  let invvec = build.inst_ir_cmd_ir_op(IrCmd::FloatToVec, inv);

  let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulVec, a, invvec);
  let result_tagged = build.inst_ir_cmd_ir_op(IrCmd::TagVector, result);

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, result_tagged);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
