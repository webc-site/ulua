use ulua_common::macros::luau_insn_a::LUAU_INSN_A;

use crate::{
  functions::translate_inst_load_constant::translate_inst_load_constant,
  records::ir_builder::IrBuilder, type_aliases::instruction_ir_translation::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_load_kx(build: &mut IrBuilder, pc: *const Instruction) {
  translate_inst_load_constant(
    build,
    LUAU_INSN_A(unsafe { *pc }) as i32,
    unsafe { *pc.add(1) } as i32,
  );
}
