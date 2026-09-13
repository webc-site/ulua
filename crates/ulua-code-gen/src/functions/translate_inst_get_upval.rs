use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_get_upval(build: &mut IrBuilder, pc: *const Instruction, _pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let up = LUAU_INSN_B(unsafe { *pc }) as u8;

  let vm_upvalue = build.vm_upvalue(up);
  let value = build.inst_ir_cmd_ir_op(IrCmd::GetUpvalue, vm_upvalue);

  let vm_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, vm_reg, value);
}
