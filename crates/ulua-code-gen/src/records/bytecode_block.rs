#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeBlock {
  /// 'start' and 'finish' define an inclusive range of instructions which belong to the block
  pub startpc: i32,
  pub finishpc: i32,
}

impl Default for BytecodeBlock {
  fn default() -> Self {
    Self {
      startpc: -1,
      finishpc: -1,
    }
  }
}
