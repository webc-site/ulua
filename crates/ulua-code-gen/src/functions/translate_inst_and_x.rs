use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B};

use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_and_x(
  build: &mut IrBuilder,
  pc: *const Instruction,
  _pcpos: i32,
  c: IrOp,
) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as u8;
  let rb = LUAU_INSN_B(insn) as u8;

  // "b and c" -> "truthy(b) ? c : b"
  let rb_reg = build.vm_reg(rb);
  let lhs = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, rb_reg);
  let rhs = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, c);

  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SelectIfTruthy, lhs, rhs, lhs);

  let ra_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, result);
}
