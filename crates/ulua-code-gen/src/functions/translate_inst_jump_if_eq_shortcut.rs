use ulua_common::{
  FFlag, enums::luau_bytecode_type::LuauBytecodeType, macros::luau_insn_a::LUAU_INSN_A,
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
  functions::{
    get_initialized_fallback::get_initialized_fallback,
    is_expected_or_unknown_bytecode_type::is_expected_or_unknown_bytecode_type,
  },
  records::{ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_jump_if_eq_shortcut(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
  not_: bool,
) {
  let rr = LUAU_INSN_A(unsafe { *pc.add(2) });

  let ra = LUAU_INSN_A(unsafe { *pc });
  let rb = unsafe { *pc.add(1) };

  let next = build.block_at_inst((pcpos + 4) as u32);
  let mut fallback = IrOp::new();

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  // fast-path: number (when both operands are expected to be a number or are unknown)
  if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
    && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
  {
    let vm_reg_ra = build.vm_reg(ra as u8);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);
    let const_tag_number = build.const_tag(LuaType::Number as u8);
    let fallback_op = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
      build.vm_exit(pcpos as u32)
    } else {
      get_initialized_fallback(build, &mut fallback, pcpos)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, ta, const_tag_number, fallback_op);

    let vm_reg_rb = build.vm_reg(rb as u8);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_rb);
    let const_tag_number_2 = build.const_tag(LuaType::Number as u8);
    let fallback_op_2 = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
      build.vm_exit(pcpos as u32)
    } else {
      get_initialized_fallback(build, &mut fallback, pcpos)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, const_tag_number_2, fallback_op_2);

    let reg_ra = build.vm_reg(ra as u8);
    let va = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_ra);
    let reg_rb = build.vm_reg(rb as u8);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_rb);

    let const_tag_number_3 = build.const_tag(LuaType::Number as u8);
    let const_tag_number_4 = build.const_tag(LuaType::Number as u8);
    let cond = if not_ {
      IrCondition::NotEqual
    } else {
      IrCondition::Equal
    };
    let cond_op = build.cond(cond);

    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CmpSplitTvalue,
      const_tag_number_3,
      const_tag_number_4,
      va,
      vb,
      cond_op,
    );

    let reg_rr = build.vm_reg(rr as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg_rr, result);
    let reg_rr = build.vm_reg(rr as u8);
    let boolean_tag = build.const_tag(LuaType::Boolean as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_rr, boolean_tag);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    // If we don't need a fallback, we are done
    if fallback.kind() == IrOpKind::None {
      return;
    }

    // Otherwise, start the fallback block
    // Note that if the number fast-path is not taken at all code that would have been in the fallback is actually the main path
    build.begin_block(fallback);
  } else if FFlag::LuauCodegenInteger2.get()
    && is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_INTEGER)
    && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_INTEGER)
  {
    let vm_reg_ra = build.vm_reg(ra as u8);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);
    let const_tag_integer = build.const_tag(LuaType::Integer as u8);
    let fallback_op = if bc_types.a == LuauBytecodeType::LBC_TYPE_INTEGER.0 as u8 {
      build.vm_exit(pcpos as u32)
    } else {
      get_initialized_fallback(build, &mut fallback, pcpos)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, ta, const_tag_integer, fallback_op);

    let vm_reg_rb = build.vm_reg(rb as u8);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_rb);
    let const_tag_integer_2 = build.const_tag(LuaType::Integer as u8);
    let fallback_op_2 = if bc_types.b == LuauBytecodeType::LBC_TYPE_INTEGER.0 as u8 {
      build.vm_exit(pcpos as u32)
    } else {
      get_initialized_fallback(build, &mut fallback, pcpos)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, const_tag_integer_2, fallback_op_2);

    let reg_ra = build.vm_reg(ra as u8);
    let va = build.inst_ir_cmd_ir_op(IrCmd::LoadInt64, reg_ra);
    let reg_rb = build.vm_reg(rb as u8);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadInt64, reg_rb);

    let const_tag_integer_3 = build.const_tag(LuaType::Integer as u8);
    let const_tag_integer_4 = build.const_tag(LuaType::Integer as u8);
    let cond = if not_ {
      IrCondition::NotEqual
    } else {
      IrCondition::Equal
    };
    let cond_op = build.cond(cond);

    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CmpSplitTvalue,
      const_tag_integer_3,
      const_tag_integer_4,
      va,
      vb,
      cond_op,
    );

    let reg_rr = build.vm_reg(rr as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg_rr, result);
    let reg_rr = build.vm_reg(rr as u8);
    let boolean_tag = build.const_tag(LuaType::Boolean as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_rr, boolean_tag);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    // If we don't need a fallback, we are done
    if fallback.kind() == IrOpKind::None {
      return;
    }

    // Otherwise, start the fallback block
    // Note that if the number fast-path is not taken at all code that would have been in the fallback is actually the main path
    build.begin_block(fallback);
  }

  let savedpc = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

  let reg_ra = build.vm_reg(ra as u8);
  let reg_rb = build.vm_reg(rb as u8);
  let cond = build.cond(IrCondition::Equal);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpAny, reg_ra, reg_rb, cond);

  // CMP_ANY doesn't support NotEqual, but we can compute !result as 1-result
  if not_ {
    let const_int_1 = build.const_int(1);
    let result_2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, const_int_1, result);
    let reg_rr = build.vm_reg(rr as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg_rr, result_2);
  } else {
    let reg_rr = build.vm_reg(rr as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg_rr, result);
  }

  let reg_rr = build.vm_reg(rr as u8);
  let boolean_tag = build.const_tag(LuaType::Boolean as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_rr, boolean_tag);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
