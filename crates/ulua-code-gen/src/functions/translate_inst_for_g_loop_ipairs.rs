use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_op::LUAU_INSN_OP},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_for_g_loop_ipairs(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  CODEGEN_ASSERT!((unsafe { *pc.add(1) } as i32) < 0);

  let loop_repeat = build.block_at_inst(get_jump_target(unsafe { *pc }, pcpos as u32) as u32);
  let op = LuauOpcode::from(LUAU_INSN_OP(unsafe { *pc }) as u8);
  let loop_exit = build.block_at_inst((pcpos + get_op_length(op)) as u32);
  let fallback = build.fallback_block(pcpos as u32);

  let has_elem = build.block(IrBlockKind::Internal);

  let pcpos_op = build.const_uint(pcpos as u32);
  build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos_op);

  let reg_ra = build.vm_reg(ra);
  let tag_a = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_ra);
  let nil_tag = build.const_tag(LuaType::Nil as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_a, nil_tag, fallback);

  let reg_table = build.vm_reg(ra + 1);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_table);
  let reg_index = build.vm_reg(ra + 2);
  let index = build.inst_ir_cmd_ir_op(IrCmd::LoadInt, reg_index);

  let elem_ptr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, table, index);

  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, table, index, loop_exit);

  let elem_tag = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, elem_ptr);
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
  let number_tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_value, number_tag);

  let elem_tv = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, elem_ptr);
  let reg_elem = build.vm_reg(ra + 4);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_elem, elem_tv);

  build.inst_ir_cmd_ir_op(IrCmd::JUMP, loop_repeat);

  build.begin_block(fallback);
  let savedpc = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);
  let reg_ra = build.vm_reg(ra);
  let aux = build.const_int(unsafe { *pc.add(1) } as i32);
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
