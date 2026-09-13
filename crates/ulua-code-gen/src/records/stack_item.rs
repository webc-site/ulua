#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct StackItem {
  pub block_idx: u32,
  pub it_pos: u32,
}

impl StackItem {
  pub const BLOCK_IDX: u32 = 0;
  pub const IT_POS: u32 = 0;
}
