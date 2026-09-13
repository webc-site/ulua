use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::is_userdata_bytecode_type::is_userdata_bytecode_type,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_length(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let bc_types = build.function.get_bytecode_types_at(pcpos);

  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

  if is_userdata_bytecode_type(bc_types.a) {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, reg_ra, reg_rb);
    return;
  }

  let fallback = build.fallback_block(pcpos as u32);

  let vm_reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_rb);
  let const_tag_table = build.const_tag(LuaType::Table as u8);
  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, const_tag_table, exit_or_fallback);

  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_rb);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);

  let va = build.inst_ir_cmd_ir_op(IrCmd::TableLen, vb);
  let vai = build.inst_ir_cmd_ir_op(IrCmd::IntToNum, va);

  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, vai);
  let reg_ra = build.vm_reg(ra);
  let number_tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_ra, number_tag);

  let next = build.block_at_inst((pcpos + 1) as u32);
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::DoLen, reg_ra, reg_rb);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
