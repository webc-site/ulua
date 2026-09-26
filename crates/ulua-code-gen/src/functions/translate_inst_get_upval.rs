use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_b};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_get_upval(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 由译码器定位到 proto.code 内一条 GETUPVAL 指令起始, 在 sizecode 界内;
  // Instruction 为 u32 且 code 数组按 u32 对齐, 读一次 *pc 即取该指令字(合并原两处 *pc 读)。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let up = luau_insn_b(insn) as u8;

  let vm_upvalue = build.vm_upvalue(up);
  let value = build.inst_ir_cmd_ir_op(IrCmd::GetUpvalue, vm_upvalue);

  let vm_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, vm_reg, value);
}
