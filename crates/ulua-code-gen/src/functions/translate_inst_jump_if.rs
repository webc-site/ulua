use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_d};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_jump_if(build: &mut IrBuilder, code: &[Instruction], pcpos: i32, not_: bool) {
  // 界内约定:  契约保证 code 切片于 pcpos 处含一条 JUMPIF 指令主字(在 sizecode 界内、Instruction=u32 对齐);
  // code[pcpos] 只读取出该字供 A/D 域提取, 纯读无别名冲突, 复用同一次读取取代原先两读。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn);

  let target = build.block_at_inst((pcpos + 1 + luau_insn_d(insn)) as u32);
  let next = build.block_at_inst((pcpos + 1) as u32);

  let vm_reg_op = build.vm_reg(ra as u8);

  if not_ {
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfFalsy, vm_reg_op, target, next);
  } else {
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfTruthy, vm_reg_op, target, next);
  }

  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
