use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::builtin_check_double::builtin_check_double,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_number_to_2_number(
  build: &mut IrBuilder,
  bfid: LuauBuiltinFunction,
  nparams: i32,
  ra: i32,
  arg: i32,
  _args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 1 || nresults > 2 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let arg_op = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_op, pcpos);

  let bfid_op = build.const_uint(bfid as u32);
  let ra_op = build.vm_reg(ra as u8);
  let nresults_op = build.const_int(if nresults == 1 { 1 } else { 2 });

  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::FASTCALL, bfid_op, ra_op, arg_op, nresults_op);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 2,
  }
}
