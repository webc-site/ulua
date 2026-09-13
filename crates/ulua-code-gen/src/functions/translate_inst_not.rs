use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_not(build: &mut IrBuilder, pc: *const Instruction) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

  let rb_reg = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, rb_reg);
  let rb_reg = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadInt, rb_reg);

  let va = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NotAny, tb, vb);

  let ra_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_reg, va);
  let ra_reg = build.vm_reg(ra);
  let boolean_tag = build.const_tag(LuaType::Boolean as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, boolean_tag);
}
