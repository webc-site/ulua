use crate::{
  functions::translate_inst_get_global::translate_global_access, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// 翻译 SETGLOBAL 指令。
pub fn translate_inst_set_global(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_global_access(build, code, pcpos, false);
}
