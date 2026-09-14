use ulua_common::macros::{
  luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C,
};
use ulua_vm::enums::tms::TMS;

use crate::{
  functions::translate_inst_binary_numeric::translate_inst_binary_numeric,
  records::ir_builder::IrBuilder, type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_binary_k(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
  tm: TMS,
) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as i32;
  let rb = LUAU_INSN_B(insn) as i32;
  let rc = LUAU_INSN_C(insn);

  let opb = build.vm_reg(rb as u8);
  let opc = build.vm_const(rc);

  translate_inst_binary_numeric(build, ra, rb, -1, opb, opc, pcpos, tm);
}
