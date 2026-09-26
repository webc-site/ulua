use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b};

use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

/// AND / OR X 共用主体：`is_and` 取 true（"b and c" → truthy(b) ? c : b）/
/// false（"b or c" → truthy(b) ? b : c），仅 SelectIfTruthy 第 2/3 实参互换。
pub fn translate_inst_and_or_x(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  c: IrOp,
  is_and: bool,
) {
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;

  let rb_reg = build.vm_reg(rb);
  let lhs = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, rb_reg);
  let rhs = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, c);

  let result = if is_and {
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SelectIfTruthy, lhs, rhs, lhs)
  } else {
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SelectIfTruthy, lhs, lhs, rhs)
  };

  let ra_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, result);
}
