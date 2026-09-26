use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_vector_tvalue_result::builtin_store_vector_tvalue_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_vector_lerp(
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
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg1 = build.vm_reg(arg as u8);
  let fallback = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(arg1, LuaType::Vector as u8, fallback);
  build.load_and_check_tag(args, LuaType::Vector as u8, fallback);
  builtin_check_double(build, arg3, pcpos);

  let a = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arg1);
  let b = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, args);
  let t = builtin_load_double(build, arg3);

  let float_t = build.inst_ir_cmd_ir_op(IrCmd::NumToFloat, t);
  let tvec = build.inst_ir_cmd_ir_op(IrCmd::FloatToVec, float_t);
  let one_float = build.const_double(1.0);
  let one_vec = build.inst_ir_cmd_ir_op(IrCmd::FloatToVec, one_float);
  let diff = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubVec, b, a);

  let res = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::MuladdVec, diff, tvec, a);
  let ret = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectVec, res, b, tvec, one_vec);
  builtin_store_vector_tvalue_result(build, ra, ret);

  BuiltinImplResult::FULL_ONE_RESULT
}
