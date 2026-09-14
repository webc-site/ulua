use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::LUAU_INSN_OP};

use crate::{
  functions::get_op_length::get_op_length, type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_instruction_count_instruction_size(insns: *const Instruction, size: u32) -> u32 {
  let mut count: u32 = 0;
  let mut i: u32 = 0;

  while i < size {
    unsafe {
      count += 1;
      let op = LUAU_INSN_OP(*insns.add(i as usize)) as u8;
      let op_enum = LuauOpcode::from(op);
      i += get_op_length(op_enum) as u32;
    }
  }

  count
}
