#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct NumberedInstruction {
  pub inst_idx: u32,
  pub start_pos: u32,
  pub finish_pos: u32,
}

impl NumberedInstruction {
  pub const INST_IDX: u32 = 0;
  pub const START_POS: u32 = 0;
  pub const FINISH_POS: u32 = 0;
}
