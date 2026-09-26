use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_d::luau_insn_d};
use ulua_vm::{enums::lua_type::LuaType, macros::lu_tag_iterator::LU_TAG_ITERATOR};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_condition::IrCondition},
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// 翻译 LOP_FORGPREP_NEXT 指令。
pub fn translate_inst_for_g_prep_next(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_for_g_prep(build, code, pcpos, false);
}

/// 翻译 LOP_FORGPREP_INEXT 指令。
pub fn translate_inst_for_g_prep_inext(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_for_g_prep(build, code, pcpos, true);
}

/// FORGPREP_NEXT 与 FORGPREP_INEXT 共享翻译实现。
fn translate_for_g_prep(build: &mut IrBuilder, code: &[Instruction], pcpos: i32, is_inext: bool) {
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;

  let target = build.block_at_inst((pcpos + 1 + luau_insn_d(insn)) as u32);
  let fallback = build.fallback_block(pcpos as u32);

  build.check_safe_env(pcpos);

  let vm_reg_ra_plus_1 = build.vm_reg(ra + 1);
  build.load_and_check_tag(vm_reg_ra_plus_1, LuaType::Table as u8, fallback);

  let vm_reg_ra_plus_2 = build.vm_reg(ra + 2);

  if is_inext {
    build.load_and_check_tag(vm_reg_ra_plus_2, LuaType::Number as u8, fallback);

    let num_c = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, vm_reg_ra_plus_2);
    let const_double_zero = build.const_double(0.0);
    let cond_not_equal = build.cond(IrCondition::NotEqual);
    let finish = build.block(IrBlockKind::Internal);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpCmpNum,
      num_c,
      const_double_zero,
      cond_not_equal,
      fallback,
      finish,
    );
    build.begin_block(finish);
  } else {
    build.load_and_check_tag(vm_reg_ra_plus_2, LuaType::Nil as u8, fallback);
  }

  let vm_reg_ra = build.vm_reg(ra);
  build.store_tag(vm_reg_ra, LuaType::Nil as u8);

  let const_int_zero = build.const_int(0);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, vm_reg_ra_plus_2, const_int_zero);

  let const_int_lu_tag_iterator = build.const_int(LU_TAG_ITERATOR);
  build.inst_ir_cmd_ir_op_ir_op(
    IrCmd::StoreExtra,
    vm_reg_ra_plus_2,
    const_int_lu_tag_iterator,
  );

  build.store_tag(vm_reg_ra_plus_2, LuaType::LightUserData as u8);

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
