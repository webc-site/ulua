use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{
  constants::LBF_IR_MATH_LOG2,
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_log(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  // cpp IrTranslateBuiltins.cpp:184 `int libmId = LBF_MATH_LOG;`
  let mut libm_id = LuauBuiltinFunction::LBF_MATH_LOG as i32;
  let mut denom: Option<f64> = None;

  if nparams != 1 {
    let Some(y_val) = build.function.as_double_op(args) else {
      return BuiltinImplResult::NONE_FALLBACK;
    };

    if y_val == 2.0 {
      // CodeGen 内部虚拟 id（见 crate::constants，IrData.h:504）
      libm_id = LBF_IR_MATH_LOG2;
    } else if y_val == 10.0 {
      // cpp IrTranslateBuiltins.cpp:197 `libmId = LBF_MATH_LOG10;`
      libm_id = LuauBuiltinFunction::LBF_MATH_LOG10 as i32;
    } else {
      denom = Some(y_val.ln());
    }
  }

  let arg_op = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_op, pcpos);

  let va = builtin_load_double(build, arg_op);

  let const_libm_id = build.const_uint(libm_id as u32);
  let mut res = build.inst_ir_cmd_ir_op_ir_op(IrCmd::InvokeLibm, const_libm_id, va);

  if let Some(d) = denom {
    let const_d = build.const_double(d);
    res = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, res, const_d);
  }

  builtin_store_number_result(build, ra, arg, res);

  BuiltinImplResult::FULL_ONE_RESULT
}
