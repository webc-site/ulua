use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{
  functions::{
    emit_builtin_math_frexp::emit_builtin_math_frexp,
    emit_builtin_math_modf::emit_builtin_math_modf,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_x_64::AssemblyBuilderX64, ir_reg_alloc_x_64::IrRegAllocX64},
};

pub fn emit_builtin_ir_reg_alloc_x_64_assembly_builder_x_64_i32_i32_i32_i32(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  bfid: i32,
  ra: i32,
  arg: i32,
  nresults: i32,
) {
  match bfid as u8 {
    x if x == LuauBuiltinFunction::LBF_MATH_FREXP as u8 => {
      CODEGEN_ASSERT!(nresults == 1 || nresults == 2);
      emit_builtin_math_frexp(regs, build, ra, arg, nresults);
    }
    x if x == LuauBuiltinFunction::LBF_MATH_MODF as u8 => {
      CODEGEN_ASSERT!(nresults == 1 || nresults == 2);
      emit_builtin_math_modf(regs, build, ra, arg, nresults);
    }
    _ => {
      CODEGEN_ASSERT!(false, "Missing x64 lowering");
    }
  }
}
