use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_map_1(
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

  let lua_tvector = LuaType::Vector as u8;
  let fallback = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(arg1, lua_tvector, fallback);

  let c0 = build.const_int(0);
  let c4 = build.const_int(4);
  let c8 = build.const_int(8);

  let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c0);
  let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c4);
  let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, c8);

  let xr = build.inst_ir_cmd_ir_op(cmd, x1);
  let yr = build.inst_ir_cmd_ir_op(cmd, y1);
  let zr = build.inst_ir_cmd_ir_op(cmd, z1);

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, ra_reg, xr, yr, zr);
  let tag = build.const_tag(lua_tvector);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
