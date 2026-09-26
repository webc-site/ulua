use ulua_common::macros::luau_insn_ops::{luau_insn_a, luau_insn_d};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_cmp_proto(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: `pc` 依契约指向当前字节码数组内的合法指令字（CMP 类指令，其 aux 字紧随其后）；
  // `code[pcpos]` 读 Copy 的 `Instruction`(u32)，`*pc.add(1)` 读界内的 aux 字。一次读取复用 ra/offset 解码。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let aux = code[pcpos as usize + 1];

  let target = build.block_at_inst((pcpos + 1 + luau_insn_d(insn) as i32) as u32);
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

  // 原字节码的 fallthrough 是隐式的，因此从这里开启下一个内部 block
  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
