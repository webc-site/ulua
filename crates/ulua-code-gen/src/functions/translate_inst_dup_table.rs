use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_dup_table(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let k = LUAU_INSN_D(unsafe { *pc }) as u32;

  let saved_pc = build.const_uint(pcpos as u32 + 1);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, saved_pc);

  let vm_k = build.vm_const(k);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_k);

  let va = build.inst_ir_cmd_ir_op(IrCmd::DupTable, table);

  let ra_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, ra_reg, va);

  let tag = build.const_tag(LuaType::Table as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);

  build.inst_ir_cmd(IrCmd::CheckGc);
}
