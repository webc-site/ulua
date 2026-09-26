use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_set_upval(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 SETUPVAL 指令主字(译码器保证在 sizecode 界内、Instruction=u32 对齐);
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突, 取代原先对 *pc 的两次重复读取。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let up = luau_insn_b(insn) as u8;

  let load_arg = build.vm_reg(ra);
  let value = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_arg);

  let upvalue = build.vm_upvalue(up);
  let undef = build.undef();
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SetUpvalue, upvalue, value, undef);
}
