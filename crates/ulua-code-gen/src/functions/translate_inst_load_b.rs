use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_load_b(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let pc_val = unsafe { *pc };
  let ra = luau_insn_a(pc_val) as u8;
  let b = luau_insn_b(pc_val);
  let c = luau_insn_c(pc_val);

  let ra_op = build.vm_reg(ra);
  let b_op = build.const_int(b as i32);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, ra_op, b_op);

  let tag_op = build.const_tag(1); // LUA_TBOOLEAN = 1
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_op, tag_op);

  if c != 0 {
    let target = pcpos + 1 + (c as i32);
    let block = build.block_at_inst(target as u32);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
  }
}
