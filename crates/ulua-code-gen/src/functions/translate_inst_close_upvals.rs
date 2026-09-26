use ulua_common::macros::luau_insn_ops::luau_insn_a;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_close_upvals(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // 界内约定:  契约保证 code 切片于 pcpos 指向字节码缓冲内存活、对齐(Instruction=u32)的合法指令字，code[pcpos] 只读取
  // A 域作为 upvalue 关闭边界寄存器，纯读无别名冲突。
  let ra = luau_insn_a(code[pcpos as usize]) as u8;
  let vm_reg = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op(IrCmd::CloseUpvals, vm_reg);
}
