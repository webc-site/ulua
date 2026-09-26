use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_new_table(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 NEWTABLE 指令主字(译码器保证在 sizecode 界内、u32 对齐),
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突。
  let pc_val = code[pcpos as usize];
  let ra = luau_insn_a(pc_val) as u8;
  let b = luau_insn_b(pc_val);
  // Safety: NEWTABLE 为 2 字编码, pc.add(1) 为其 AUX 字(哈希表大小), 译码器保证仍在 code/sizecode
  // 界内且 u32 对齐, 只读合法。
  let aux = code[pcpos as usize + 1];

  let savedpc_op = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_op);

  let array_size = if b == 0 { 0 } else { 1 << (b - 1) };
  let aux_op = build.const_uint(aux);
  let array_size_op = build.const_uint(array_size);
  let va = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NewTable, aux_op, array_size_op);

  let ra_op = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, ra_op, va);

  build.store_tag(ra_op, LuaType::Table as u8);

  build.inst_ir_cmd(IrCmd::CheckGc);
}
