use alloc::vec::Vec;

use crate::{enums::ir_cmd::IrCmd, records::ir_inst::IrInst};

#[inline]
pub fn get_instruction_count_vector_ir_inst_ir_cmd(instructions: &Vec<IrInst>, cmd: IrCmd) -> u32 {
  let mut count = 0;
  for inst in instructions {
    if inst.cmd == cmd {
      count += 1;
    }
  }
  count
}
