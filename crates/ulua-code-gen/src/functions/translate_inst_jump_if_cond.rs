use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::is_expected_or_unknown_bytecode_type::is_expected_or_unknown_bytecode_type,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_jump_if_cond(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
  mut cond: IrCondition,
) {
  CODEGEN_ASSERT!(cond != IrCondition::Equal && cond != IrCondition::NotEqual);

  let pc_value = unsafe { *pc };
  let ra = LUAU_INSN_A(pc_value) as u8;
  let rb = unsafe { *pc.add(1) } as u8;

  let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(pc_value)) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
    && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
  {
    let fallback = build.fallback_block(pcpos as u32);

    let reg_ra = build.vm_reg(ra);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_ra);
    let number_tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, ta, number_tag, fallback);

    let reg_rb = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    let number_tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, number_tag, fallback);

    let reg_ra = build.vm_reg(ra);
    let va = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_ra);
    let reg_rb = build.vm_reg(rb);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_rb);

    let cond_op = build.cond(cond);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpCmpNum,
      va,
      vb,
      cond_op,
      target,
      next,
    );

    build.begin_block(fallback);
  }

  let savedpc = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

  let mut reverse = false;

  if cond == IrCondition::NotLessEqual {
    reverse = true;
    cond = IrCondition::LessEqual;
  } else if cond == IrCondition::NotLess {
    reverse = true;
    cond = IrCondition::Less;
  } else if cond == IrCondition::NotEqual {
    reverse = true;
    cond = IrCondition::Equal;
  }

  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let cond_op = build.cond(cond);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpAny, reg_ra, reg_rb, cond_op);
  let zero = build.const_int(0);
  let equal = build.cond(IrCondition::Equal);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpCmpInt,
    result,
    zero,
    equal,
    if reverse { target } else { next },
    if reverse { next } else { target },
  );

  build.begin_block(next);
}
