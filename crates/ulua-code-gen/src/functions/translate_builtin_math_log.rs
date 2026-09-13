use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
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
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let mut libm_id = 12; // LBF_MATH_LOG
  let mut denom: Option<f64> = None;

  if nparams != 1 {
    let y = build.function.as_double_op(args);

    if y.is_none() {
      return BuiltinImplResult {
        r#type: BuiltinImplType::None,
        actual_result_count: -1,
      };
    }

    let y_val = y.unwrap();
    if y_val == 2.0 {
      libm_id = 256; // LBF_IR_MATH_LOG2
    } else if y_val == 10.0 {
      libm_id = 13; // LBF_MATH_LOG10
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

  let ra_op = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_op, res);

  if ra != arg {
    let const_tag = build.const_tag(0); // LUA_TNUMBER
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_op, const_tag);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
