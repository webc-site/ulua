use crate::{enums::ir_cmd::IrCmd, records::ir_inst::IrInst};

#[inline]
pub fn count_instructions_with_cmd(instructions: &[IrInst], cmd: IrCmd) -> u32 {
  instructions.iter().filter(|inst| inst.cmd == cmd).count() as u32
}
