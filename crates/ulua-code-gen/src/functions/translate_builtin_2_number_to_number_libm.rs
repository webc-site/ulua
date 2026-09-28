use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{
  functions::translate_builtin_number_to_number_libm::translate_builtin_number_to_number_libm_args,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_2_number_to_number_libm(
  build: &mut IrBuilder,
  bfid: LuauBuiltinFunction,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  translate_builtin_number_to_number_libm_args(
    build,
    bfid,
    nparams,
    ra,
    arg,
    Some(args),
    nresults,
    pcpos,
  )
}
