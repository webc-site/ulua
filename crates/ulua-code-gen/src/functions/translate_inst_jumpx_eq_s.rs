use ulua_common::macros::{
  luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv::LUAU_INSN_AUX_KV,
  luau_insn_aux_not::LUAU_INSN_AUX_NOT, luau_insn_d::LUAU_INSN_D,
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_jumpx_eq_s(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let pc_val = unsafe { *pc };
  let ra = LUAU_INSN_A(pc_val) as u8;
  let aux = unsafe { *pc.add(1) };
  let not_ = LUAU_INSN_AUX_NOT(aux) != 0;

  let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(pc_val)) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);
  let check_value = build.block(IrBlockKind::Internal);

  let vm_reg_ra = build.vm_reg(ra);
  let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);

  let const_tag_string = build.const_tag(LuaType::String as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpEqTag,
    ta,
    const_tag_string,
    check_value,
    if not_ { target } else { next },
  );

  build.begin_block(check_value);

  let vm_reg_ra = build.vm_reg(ra);
  let va = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_ra);
  let vm_const_kv = build.vm_const(LUAU_INSN_AUX_KV(aux));
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_const_kv);

  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpEqPointer,
    va,
    vb,
    if not_ { next } else { target },
    if not_ { target } else { next },
  );

  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
