use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_not(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 NOT 指令主字(译码器保证在 sizecode 界内、Instruction=u32 对齐);
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突, 取代原先对 *pc 的两次重复读取。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;

  let rb_reg = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, rb_reg);
  let rb_reg = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadInt, rb_reg);

  let va = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NotAny, tb, vb);

  let ra_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_reg, va);
  let ra_reg = build.vm_reg(ra);
  build.store_tag(ra_reg, LuaType::Boolean as u8);
}
