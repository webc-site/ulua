use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D};

use crate::{
  functions::translate_inst_load_constant::translate_inst_load_constant,
  records::ir_builder::IrBuilder, type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_load_k(build: &mut IrBuilder, pc: *const Instruction) {
  let insn = unsafe { *pc };
  translate_inst_load_constant(build, LUAU_INSN_A(insn) as i32, LUAU_INSN_D(insn));
}
