use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D};
use ulua_vm::{enums::lua_type::LuaType, macros::lu_tag_iterator::LU_TAG_ITERATOR};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_condition::IrCondition},
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_for_g_prep_inext(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
) {
  let insn = unsafe { *pc };
  let ra = LUAU_INSN_A(insn) as u8;

  let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(insn)) as u32);
  let fallback = build.fallback_block(pcpos as u32);
  let finish = build.block(IrBlockKind::Internal);

  build.check_safe_env(pcpos);

  let vm_reg_ra_plus_1 = build.vm_reg(ra + 1);
  let tag_b = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra_plus_1);
  let const_tag_table = build.const_tag(LuaType::Table as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_b, const_tag_table, fallback);

  let vm_reg_ra_plus_2 = build.vm_reg(ra + 2);
  let tag_c = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra_plus_2);
  let const_tag_number = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_c, const_tag_number, fallback);

  let num_c = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, vm_reg_ra_plus_2);
  let const_double_zero = build.const_double(0.0);
  let cond_not_equal = build.cond(IrCondition::NotEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpNum,
    num_c,
    const_double_zero,
    cond_not_equal,
    fallback,
    finish,
  );

  build.begin_block(finish);

  let vm_reg_ra = build.vm_reg(ra);
  let const_tag_nil = build.const_tag(LuaType::Nil as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, const_tag_nil);

  let vm_reg_ra_plus_2 = build.vm_reg(ra + 2);
  let const_int_zero = build.const_int(0);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, vm_reg_ra_plus_2, const_int_zero);

  let const_int_lu_tag_iterator = build.const_int(LU_TAG_ITERATOR);
  build.inst_ir_cmd_ir_op_ir_op(
    IrCmd::StoreExtra,
    vm_reg_ra_plus_2,
    const_int_lu_tag_iterator,
  );

  let const_tag_lightuserdata = build.const_tag(LuaType::LightUserData as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra_plus_2, const_tag_lightuserdata);

  build.inst_ir_cmd_ir_op(IrCmd::JUMP, target);

  build.begin_block(fallback);
  let const_uint_pcpos = build.const_uint(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(
    IrCmd::ForgprepXnextFallback,
    const_uint_pcpos,
    vm_reg_ra,
    target,
  );
}
