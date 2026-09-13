use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_cmp_proto(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let aux = unsafe { *pc.add(1) };

  let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(unsafe { *pc }) as i32) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);
  let check_fun_id = build.block(IrBlockKind::Internal);

  let vm_reg_ra = build.vm_reg(ra);
  let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);
  let const_tag_function = build.const_tag(LuaType::Function as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpEqTag,
    ta,
    const_tag_function,
    check_fun_id,
    target,
  );

  build.begin_block(check_fun_id);
  let vm_reg_ra = build.vm_reg(ra);
  let ccl = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_ra);
  let vb = build.const_uint(aux);

  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JumpCmpProtoid, ccl, vb, next, target);

  // Fallthrough in original bytecode is implicit, so we start next internal block here
  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
