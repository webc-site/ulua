use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  fflag,
  macros::{luau_insn_a::luau_insn_a, luau_insn_d::luau_insn_d},
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

#[inline]
fn emit_tag_check_direct(build: &mut IrBuilder, reg: IrOp, tag: u8, fallback: IrOp) {
  let loaded_tag = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg);
  let const_tag = build.const_tag(tag);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, loaded_tag, const_tag, fallback);
}

#[inline]
fn emit_tag_check_shortcut(
  build: &mut IrBuilder,
  fallback: &mut IrOp,
  pcpos: i32,
  reg: IrOp,
  tag: u8,
  is_exact: bool,
) {
  let loaded_tag = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg);
  let const_tag = build.const_tag(tag);
  let fallback_op = if is_exact {
    build.vm_exit(pcpos as u32)
  } else {
    get_initialized_fallback(build, fallback, pcpos)
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, loaded_tag, const_tag, fallback_op);
}

#[inline]
fn emit_store_bool(build: &mut IrBuilder, rr: u8, result: IrOp, next: IrOp) {
  let reg_rr = build.vm_reg(rr);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, reg_rr, result);
  let reg_rr = build.vm_reg(rr);
  build.store_tag(reg_rr, LuaType::Boolean as u8);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}

/// JUMPIFEQ 常规跳转路径的 NUMBER/INTEGER 孪生臂共用主体（按 `ty` 选
/// Number/LoadDouble/JumpCmpNum 或 Integer/LoadInt64/JumpCmpInt64）。
#[inline]
fn emit_regular_cmp(
  build: &mut IrBuilder,
  pcpos: i32,
  ra: u8,
  rb: u8,
  not_: bool,
  target: IrOp,
  next: IrOp,
  ty: LuauBytecodeType,
) {
  let is_number = ty.0 as u8 == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
  let tag = if is_number {
    LuaType::Number as u8
  } else {
    LuaType::Integer as u8
  };
  let load_cmd = if is_number {
    IrCmd::LoadDouble
  } else {
    IrCmd::LoadInt64
  };
  let cmp_cmd = if is_number {
    IrCmd::JumpCmpNum
  } else {
    IrCmd::JumpCmpInt64
  };

  let fallback = build.fallback_block(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);

  emit_tag_check_direct(build, reg_ra, tag, fallback);
  emit_tag_check_direct(build, reg_rb, tag, fallback);

  let va = build.inst_ir_cmd_ir_op(load_cmd, reg_ra);
  let vb = build.inst_ir_cmd_ir_op(load_cmd, reg_rb);

  let cond = build.cond(IrCondition::NotEqual);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    cmp_cmd,
    va,
    vb,
    cond,
    if not_ { target } else { next },
    if not_ { next } else { target },
  );

  build.begin_block(fallback);
}

/// JUMPIFEQ 快捷求值路径的 NUMBER/INTEGER 孪生臂共用主体（按 `ty` 选
/// Number/LoadDouble 或 Integer/LoadInt64）。返回 `false` 表示快路径完整
/// 成立（未建 fallback 块，调用方直接返回）；返回 `true` 表示已建立
/// fallback 块并跳入，调用方需继续发射通用比较路径。
#[inline]
fn emit_shortcut_cmp(
  build: &mut IrBuilder,
  fallback: &mut IrOp,
  pcpos: i32,
  ra: u8,
  rb: u8,
  rr: u8,
  next: IrOp,
  not_: bool,
  ty: LuauBytecodeType,
) -> bool {
  let is_number = ty.0 as u8 == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
  let tag = if is_number {
    LuaType::Number as u8
  } else {
    LuaType::Integer as u8
  };
  let load_cmd = if is_number {
    IrCmd::LoadDouble
  } else {
    IrCmd::LoadInt64
  };
  let bc_types = build.function.get_bytecode_types_at(pcpos);
  let exact_a = bc_types.a == ty.0 as u8;
  let exact_b = bc_types.b == ty.0 as u8;

  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);

  emit_tag_check_shortcut(build, fallback, pcpos, reg_ra, tag, exact_a);
  emit_tag_check_shortcut(build, fallback, pcpos, reg_rb, tag, exact_b);

  let va = build.inst_ir_cmd_ir_op(load_cmd, reg_ra);
  let vb = build.inst_ir_cmd_ir_op(load_cmd, reg_rb);

  let tag_1 = build.const_tag(tag);
  let tag_2 = build.const_tag(tag);
  let cond = if not_ {
    IrCondition::NotEqual
  } else {
    IrCondition::Equal
  };
  let cond_op = build.cond(cond);

  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::CmpSplitTvalue,
    tag_1,
    tag_2,
    va,
    vb,
    cond_op,
  );

  emit_store_bool(build, rr, result, next);

  if fallback.kind() == IrOpKind::None {
    return false;
  }
  build.begin_block(*fallback);
  true
}

/// 翻译 JUMPIFEQ / JUMPIFNOTEQ 指令常规跳转路径。
pub fn translate_inst_jump_if_eq(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  not_: bool,
) {
  let pc_val = code[pcpos as usize];
  let ra = luau_insn_a(pc_val) as u8;
  let rb = code[pcpos as usize + 1] as u8;

  let target = build.block_at_inst((pcpos + 1 + luau_insn_d(pc_val)) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
    && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
  {
    emit_regular_cmp(
      build,
      pcpos,
      ra,
      rb,
      not_,
      target,
      next,
      LuauBytecodeType::LBC_TYPE_NUMBER,
    );
  } else if fflag::LuauCodegenInteger3.get()
    && fflag::LuauCodegenIntegerCompare.get()
    && is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_INTEGER)
    && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_INTEGER)
  {
    emit_regular_cmp(
      build,
      pcpos,
      ra,
      rb,
      not_,
      target,
      next,
      LuauBytecodeType::LBC_TYPE_INTEGER,
    );
  }

  let savedpc = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let cond = build.cond(IrCondition::Equal);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpAny, reg_ra, reg_rb, cond);
  let zero = build.const_int(0);
  let cond = build.cond(IrCondition::Equal);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpInt,
    result,
    zero,
    cond,
    if not_ { target } else { next },
    if not_ { next } else { target },
  );

  build.begin_block(next);
}

/// 翻译 JUMPIFEQ / JUMPIFNOTEQ 指令 fast-path 快捷求值路径（3 字指令序列）。
pub fn translate_inst_jump_if_eq_shortcut(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  not_: bool,
) {
  let rr = luau_insn_a(code[pcpos as usize + 2]) as u8;
  let ra = luau_insn_a(code[pcpos as usize]) as u8;
  let rb = code[pcpos as usize + 1] as u8;

  let next = build.block_at_inst((pcpos + 4) as u32);
  let mut fallback = IrOp::new();

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  let shortcut_type =
    if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
      && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
    {
      Some(LuauBytecodeType::LBC_TYPE_NUMBER)
    } else if fflag::LuauCodegenInteger3.get()
      && is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_INTEGER)
      && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_INTEGER)
    {
      Some(LuauBytecodeType::LBC_TYPE_INTEGER)
    } else {
      None
    };

  if let Some(expected_type) = shortcut_type
    && !emit_shortcut_cmp(
      build,
      &mut fallback,
      pcpos,
      ra,
      rb,
      rr,
      next,
      not_,
      expected_type,
    )
  {
    return;
  }

  let savedpc = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let cond = build.cond(IrCondition::Equal);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpAny, reg_ra, reg_rb, cond);

  let final_result = if not_ {
    let const_int_1 = build.const_int(1);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, const_int_1, result)
  } else {
    result
  };

  emit_store_bool(build, rr, final_result, next);
}
