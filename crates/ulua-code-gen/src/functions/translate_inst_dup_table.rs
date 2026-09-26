use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_d::luau_insn_d};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_dup_table(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 DUPTABLE 指令起始(在 sizecode 界内、u32 对齐),
  // 读一次 *pc 得该指令字, 供 A/D 字段解码复用(合并原两处 *pc 读)。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let k = luau_insn_d(insn) as u32;

  let saved_pc = build.const_uint(pcpos as u32 + 1);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, saved_pc);

  let vm_k = build.vm_const(k);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_k);

  let va = build.inst_ir_cmd_ir_op(IrCmd::DupTable, table);

  let ra_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, ra_reg, va);

  build.store_tag(ra_reg, LuaType::Table as u8);

  build.inst_ir_cmd(IrCmd::CheckGc);
}
