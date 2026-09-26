#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct NumberedInstruction {
  pub inst_idx: u32,
  pub start_pos: u32,
  pub finish_pos: u32,
}
