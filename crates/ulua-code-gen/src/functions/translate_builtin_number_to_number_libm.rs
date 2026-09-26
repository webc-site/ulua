use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_number_to_number_libm(
  build: &mut IrBuilder,
  bfid: LuauBuiltinFunction,
  nparams: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  translate_builtin_number_to_number_libm_args(build, bfid, nparams, ra, arg, None, nresults, pcpos)
}

/// libm `number -> number` 内建共用核心：`args2` 为可选第二操作数（2 参内建
/// 以 `nparams >= 2` 门控）；LDEXP 的 NumToInt 定点转换在 2 参侧分支内完成。
pub fn translate_builtin_number_to_number_libm_args(
  build: &mut IrBuilder,
  bfid: LuauBuiltinFunction,
  nparams: i32,
  ra: i32,
  arg: i32,
  args2: Option<IrOp>,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  let min_params = if args2.is_some() { 2 } else { 1 };
  if nparams < min_params || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  if let Some(args) = args2 {
    builtin_check_double(build, args, pcpos);
  }

  let va = builtin_load_double(build, arg_reg);
  let vb = args2.map(|args| {
    let vb = builtin_load_double(build, args);
    if bfid == LuauBuiltinFunction::LBF_MATH_LDEXP {
      build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb)
    } else {
      vb
    }
  });

  let bfid_op = build.const_uint(bfid as u32);
  let res = match vb {
    Some(vb) => build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::InvokeLibm, bfid_op, va, vb),
    None => build.inst_ir_cmd_ir_op_ir_op(IrCmd::InvokeLibm, bfid_op, va),
  };

  builtin_store_number_result(build, ra, arg, res);

  BuiltinImplResult::FULL_ONE_RESULT
}
