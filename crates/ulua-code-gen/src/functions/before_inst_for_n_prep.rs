use ulua_common::macros::luau_insn_ops::luau_insn_a;

use crate::{
  functions::get_loop_step_k::get_loop_step_k,
  records::ir_builder::{IrBuilder, LoopInfo},
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn before_inst_for_n_prep(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // 界内约定:  契约保证 code 切片于 pcpos 指向存活且对齐的 Instruction，code[pcpos] 只读取出合法指令字后交 luau_insn_a
  // 取 A 域；此处为纯读、无别名冲突。
  let ra = luau_insn_a(code[pcpos as usize]) as i32;
  let step_k = get_loop_step_k(build, ra);
  build.numeric_loop_stack.push(LoopInfo {
    step: step_k,
    startpc: pcpos + 1,
  });
}
