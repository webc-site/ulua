use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_d};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_get_import(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: pc 指向 proto.code 内一条 GETIMPORT 主指令字(在 sizecode 界内, u32 对齐)。
  let pc_val = code[pcpos as usize];
  let ra = luau_insn_a(pc_val) as u8;
  let k = luau_insn_d(pc_val) as u32;
  // Safety: GETIMPORT 为 2 字指令, pc.add(1) 为其 AUX 字(import id), 译码器已保证仍在
  // code/sizecode 界内且 u32 对齐, 读取合法。
  let aux = code[pcpos as usize + 1];

  build.check_safe_env(pcpos);

  let ra_op = build.vm_reg(ra);
  let k_op = build.vm_const(k);
  let aux_op = build.const_import(aux);
  let pcpos_op = build.const_uint((pcpos + 1) as u32);

  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::GetCachedImport, ra_op, k_op, aux_op, pcpos_op);
}
