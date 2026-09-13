use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_move(build: &mut IrBuilder, pc: *const Instruction) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

  let load_arg = build.vm_reg(rb);
  let load = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, load_arg);

  let store_arg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, store_arg, load);
}
