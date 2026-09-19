use ulua_common::macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_move(build: &mut IrBuilder, pc: *const Instruction) {
  let ra = luau_insn_a(unsafe { *pc }) as u8;
  let rb = luau_insn_b(unsafe { *pc }) as u8;

  let load_arg = build.vm_reg(rb);
  let load = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_arg);

  let store_arg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, store_arg, load);
}
