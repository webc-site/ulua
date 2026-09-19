use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_concat(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let insn = unsafe { *pc };
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let rc = luau_insn_c(insn) as u8;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);

  let concat_arg1 = build.vm_reg(rb);
  let concat_arg2 = build.const_uint((rc as i32 - rb as i32 + 1) as u32);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CONCAT, concat_arg1, concat_arg2);

  let load_tvalue_arg = build.vm_reg(rb);
  let tvb = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_tvalue_arg);
  let store_tvalue_arg1 = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, store_tvalue_arg1, tvb);

  build.inst_ir_cmd(IrCmd::CheckGc);
}
