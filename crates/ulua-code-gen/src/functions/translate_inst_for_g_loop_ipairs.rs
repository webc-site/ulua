use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag::LuauBackedgeHeapCheck,
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::{luau_insn_a::luau_insn_a, luau_insn_op::luau_insn_op},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_for_g_loop_ipairs(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: `pc` 依契约指向当前字节码 FORGLOOP 指令字，其 aux 字（`code[pcpos+1]`）紧随且在 code 数组界内；
  // 二者皆为 Copy 的 `Instruction`(u32)，各读一次复用于下方解码，避免重复解引用。
  let insn = code[pcpos as usize];
  let aux_insn = code[pcpos as usize + 1];
  let ra = luau_insn_a(insn) as u8;
  CODEGEN_ASSERT!((aux_insn as i32) < 0);

  let loop_repeat = build.block_at_inst(get_jump_target(insn, pcpos as u32) as u32);
  let op = LuauOpcode::from(luau_insn_op(insn) as u8);
  let loop_exit = build.block_at_inst((pcpos + get_op_length(op)) as u32);
  let fallback = build.fallback_block(pcpos as u32);

  let has_elem = build.block(IrBlockKind::Internal);

  let pcpos_op = build.const_uint(pcpos as u32);
  build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos_op);

  if LuauBackedgeHeapCheck.get() {
    build.inst_ir_cmd(IrCmd::CheckGc);
  }

  let reg_ra = build.vm_reg(ra);
  build.load_and_check_tag(reg_ra, LuaType::Nil as u8, fallback);

  let reg_table = build.vm_reg(ra + 1);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_table);
  let reg_index = build.vm_reg(ra + 2);
  let index = build.inst_ir_cmd_ir_op(IrCmd::LoadInt, reg_index);

  let elem_ptr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table, index);

  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table, index, loop_exit);

  let elem_tag = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, elem_ptr);
  // 与上方守卫共用同一 Nil tag 常量（const_tag 内部按值驻留，重复取值等价）
  let nil_tag = build.const_tag(LuaType::Nil as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpEqTag,
    elem_tag,
    nil_tag,
    loop_exit,
    has_elem,
  );
  build.begin_block(has_elem);

  let one = build.const_int(1);
  let next_index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt, index, one);

  let reg_iter = build.vm_reg(ra + 2);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg_iter, next_index);

  let next_index_num = build.inst_ir_cmd_ir_op(IrCmd::IntToNum, next_index);
  let reg_value = build.vm_reg(ra + 3);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_value, next_index_num);
  let reg_value = build.vm_reg(ra + 3);
  build.store_tag(reg_value, LuaType::Number as u8);

  let elem_tv = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, elem_ptr);
  let reg_elem = build.vm_reg(ra + 4);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_elem, elem_tv);

  build.inst_ir_cmd_ir_op(IrCmd::JUMP, loop_repeat);

  build.begin_block(fallback);
  let savedpc = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);
  let reg_ra = build.vm_reg(ra);
  let aux = build.const_int(aux_insn as i32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::ForgloopFallback,
    reg_ra,
    aux,
    loop_repeat,
    loop_exit,
  );

  if build.is_internal_block(loop_exit) {
    build.begin_block(loop_exit);
  }
}
