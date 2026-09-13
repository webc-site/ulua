use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_lerp(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  _fallback: IrOp,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 3 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let ra_op = build.vm_reg(ra as u8);
  let arg_op = build.vm_reg(arg as u8);

  builtin_check_double(build, arg_op, pcpos);
  builtin_check_double(build, args, pcpos);
  builtin_check_double(build, arg3, pcpos);

  let a = builtin_load_double(build, arg_op);
  let b = builtin_load_double(build, args);
  let t = builtin_load_double(build, arg3);

  let sub_ba = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubNum, b, a);
  let l = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::MuladdNum, sub_ba, t, a);
  let one = build.const_double(1.0);
  let r = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SelectNum, l, b, t, one);

  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_op, r);

  if ra != arg {
    let tag_number = build.const_tag(3); // LUA_TNUMBER
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_op, tag_number);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
