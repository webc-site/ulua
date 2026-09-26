use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::{luau_insn_a::luau_insn_a, luau_insn_d::luau_insn_d},
};

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    is_expected_or_unknown_bytecode_type::is_expected_or_unknown_bytecode_type,
    translate_inst_binary::check_number_tag_guard,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_jump_if_cond(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  cond: IrCondition,
) {
  CODEGEN_ASSERT!(cond != IrCondition::Equal && cond != IrCondition::NotEqual);

  // Safety: pc 指向 proto.code 内一条 JUMPIF 比较指令主字(在 sizecode 界内, u32 对齐)。
  let pc_value = code[pcpos as usize];
  let ra = luau_insn_a(pc_value) as u8;
  // Safety: 该 JUMPIF* 为 2 字指令, pc.add(1) 为其 AUX 字(含 rb), 译码器已保证仍在
  // code/sizecode 界内且 u32 对齐, 读取合法。
  let rb = code[pcpos as usize + 1] as u8;

  let target = build.block_at_inst((pcpos + 1 + luau_insn_d(pc_value)) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
    && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
  {
    // eager 创建 fallback 的时序保持不变；守卫 fallback 臂经 get_initialized_fallback 复用该块。
    let mut fallback = build.fallback_block(pcpos as u32);

    check_number_tag_guard(build, ra, false, pcpos, &mut fallback);
    check_number_tag_guard(build, rb, false, pcpos, &mut fallback);

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

  // `Not*` 条件取回正向比较条件，并翻转 target/next 次序（CmpAny 走正向条件 + 反向跳转）
  let (cond, reverse) = match cond {
    IrCondition::NotLessEqual => (IrCondition::LessEqual, true),
    IrCondition::NotLess => (IrCondition::Less, true),
    IrCondition::NotEqual => (IrCondition::Equal, true),
    other => (other, false),
  };

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
