use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
};
use ulua_vm::enums::tms::TMS;

use crate::{
  functions::translate_inst_binary_numeric::translate_inst_binary_numeric,
  records::ir_builder::IrBuilder, type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_binary(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
  tm: TMS,
) {
  let ra = luau_insn_a(unsafe { *pc }) as u8;
  let rb = luau_insn_b(unsafe { *pc }) as u8;
  let rc = luau_insn_c(unsafe { *pc }) as u8;

  let opb = build.vm_reg(rb);
  let opc = build.vm_reg(rc);

  translate_inst_binary_numeric(build, ra as i32, rb as i32, rc as i32, opb, opc, pcpos, tm);
}
