use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn translate_builtin_vector_map_1_x_4(
  build: &mut IrBuilder,
  cmd: IrCmd,
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
  build.load_and_check_tag(arg1, LuaType::Vector as u8, fallback);

  let value = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arg1);
  let ret = build.inst_ir_cmd_ir_op(cmd, value);

  let ra_reg = build.vm_reg(ra as u8);
  let tag = build.inst_ir_cmd_ir_op(IrCmd::TagVector, ret);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
