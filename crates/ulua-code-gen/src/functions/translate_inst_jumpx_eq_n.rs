use ulua_common::macros::{
  luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv::LUAU_INSN_AUX_KV,
  luau_insn_aux_not::LUAU_INSN_AUX_NOT, luau_insn_d::LUAU_INSN_D,
};
use ulua_vm::{enums::lua_type::LuaType, type_aliases::t_value::TValue};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_condition::IrCondition},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_jumpx_eq_n(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let aux = unsafe { *pc.add(1) };
  let not_ = LUAU_INSN_AUX_NOT(aux) != 0;

  let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(unsafe { *pc })) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);
  let check_value = build.block(IrBlockKind::Internal);

  let vm_reg_ra = build.vm_reg(ra);
  let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);

  let const_tag_number = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpEqTag,
    ta,
    const_tag_number,
    check_value,
    if not_ { target } else { next },
  );

  build.begin_block(check_value);
  let vm_reg_ra = build.vm_reg(ra);
  let va = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, vm_reg_ra);

  CODEGEN_ASSERT!(!build.function.proto.is_null());
  let protok: TValue = unsafe {
    *(*build.function.proto)
      .k
      .add(LUAU_INSN_AUX_KV(aux) as usize)
  };

  CODEGEN_ASSERT!(protok.tt == LuaType::Number as i32);
  let vb = build.const_double(unsafe { protok.value.n });

  let cond = build.cond(IrCondition::NotEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpNum,
    va,
    vb,
    cond,
    if not_ { target } else { next },
    if not_ { next } else { target },
  );

  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
