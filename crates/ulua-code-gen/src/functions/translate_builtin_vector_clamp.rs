use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_condition::IrCondition, ir_op_kind::IrOpKind,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn translate_builtin_vector_clamp(
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
  let arg1 = build.vm_reg(arg as u8);

  if nparams != 3
    || nresults > 1
    || arg1.kind() == IrOpKind::Constant
    || args.kind() == IrOpKind::Constant
    || arg3.kind() == IrOpKind::Constant
  {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let lua_tvector = LuaType::Vector as u8;
  let exit = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(arg1, lua_tvector, exit);
  let exit = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(args, lua_tvector, exit);
  let exit = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(arg3, lua_tvector, exit);

  let block1 = build.block(IrBlockKind::Internal);
  let block2 = build.block(IrBlockKind::Internal);
  let block3 = build.block(IrBlockKind::Internal);

  let zero = build.const_int(0);
  let x = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, zero);
  let zero = build.const_int(0);
  let xmin = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, zero);
  let zero = build.const_int(0);
  let xmax = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg3, zero);

  let cond = build.cond(IrCondition::NotLessEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpFloat,
    xmin,
    xmax,
    cond,
    fallback,
    block1,
  );

  build.begin_block(block1);

  let four = build.const_int(4);
  let y = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, four);
  let four = build.const_int(4);
  let ymin = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, four);
  let four = build.const_int(4);
  let ymax = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg3, four);

  let cond = build.cond(IrCondition::NotLessEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpFloat,
    ymin,
    ymax,
    cond,
    fallback,
    block2,
  );

  build.begin_block(block2);

  let eight = build.const_int(8);
  let z = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg1, eight);
  let eight = build.const_int(8);
  let zmin = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, args, eight);
  let eight = build.const_int(8);
  let zmax = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg3, eight);

  let cond = build.cond(IrCondition::NotLessEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpFloat,
    zmin,
    zmax,
    cond,
    fallback,
    block3,
  );

  build.begin_block(block3);

  let xtemp = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxFloat, xmin, x);
  let xclamped = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinFloat, xmax, xtemp);

  let ytemp = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxFloat, ymin, y);
  let yclamped = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinFloat, ymax, ytemp);

  let ztemp = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MaxFloat, zmin, z);
  let zclamped = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinFloat, zmax, ztemp);

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::StoreVector,
    ra_reg,
    xclamped,
    yclamped,
    zclamped,
  );
  let tag = build.const_tag(lua_tvector);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::UsesFallback,
    actual_result_count: 1,
  }
}
