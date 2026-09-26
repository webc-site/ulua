#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeBlock {
  /// 'start' 与 'finish' 定义属于本 block 的指令闭区间
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
