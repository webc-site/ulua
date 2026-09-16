use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_aux_not::LUAU_INSN_AUX_NOT};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_jumpx_eq_nil_shortcut(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let rr = LUAU_INSN_A(unsafe { *pc.add(2) });

  let ra = LUAU_INSN_A(unsafe { *pc });
  let aux = unsafe { *pc.add(1) };
  let not_ = LUAU_INSN_AUX_NOT(aux) != 0;

  let next = build.block_at_inst((pcpos + 4) as u32);

  let vm_reg_ra = build.vm_reg(ra as u8);
  let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);

  let const_tag_nil = build.const_tag(LuaType::Nil as u8);
  let cond = build.cond(if not_ {
    IrCondition::NotEqual
  } else {
    IrCondition::Equal
  });
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpTag, ta, const_tag_nil, cond);

  let vm_reg_rr = build.vm_reg(rr as u8);
  let const_tag_boolean = build.const_tag(LuaType::Boolean as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_rr, const_tag_boolean);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, vm_reg_rr, result);

  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  // Fallthrough in original bytecode is implicit, so we start next internal block here
  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
