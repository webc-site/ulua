use ulua_common::macros::{luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_load_n(build: &mut IrBuilder, pc: *const Instruction) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
  let vm_reg = build.vm_reg(ra);
  let value = build.const_double(LUAU_INSN_D(unsafe { *pc }) as f64);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, vm_reg, value);
  let tag = build.const_tag(3);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg, tag);
}
