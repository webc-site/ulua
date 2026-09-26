use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_concat(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // 界内约定:  契约保证 code 切片于 pcpos 指向字节码缓冲内存活、对齐(Instruction=u32)的合法指令字，code[pcpos] 只读
  // 取出整条指令供 A/B/C 域提取，纯读无别名冲突。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let rc = luau_insn_c(insn) as u8;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);

  let concat_arg1 = build.vm_reg(rb);
  let concat_arg2 = build.const_uint((rc as i32 - rb as i32 + 1) as u32);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CONCAT, concat_arg1, concat_arg2);

  let load_tvalue_arg = build.vm_reg(rb);
  let tvb = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_tvalue_arg);
  let store_tvalue_arg1 = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, store_tvalue_arg1, tvb);

  build.inst_ir_cmd(IrCmd::CheckGc);
}
