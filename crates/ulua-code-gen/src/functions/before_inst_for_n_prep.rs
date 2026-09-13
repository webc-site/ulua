use ulua_common::macros::luau_insn_a::LUAU_INSN_A;

use crate::{
  functions::get_loop_step_k::get_loop_step_k,
  records::ir_builder::{IrBuilder, LoopInfo},
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn before_inst_for_n_prep(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as i32;
  let step_k = get_loop_step_k(build, ra);
  build.numeric_loop_stack.push(LoopInfo {
    step: step_k,
    startpc: pcpos + 1,
  });
}
