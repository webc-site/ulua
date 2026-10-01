use core::sync::atomic::{AtomicI32, Ordering};

pub const CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS: i32 = 0;
pub const CONFORMANCE_INTERRUPT_MODE_INFLOOP: i32 = 1;
/// cpp `Conformance.test.cpp:3719-3727`：`hang1..hang7` 用的中断，计数到 1000 后**每次**
/// 中断都抛错（不清零），指数级递归的 hang7 才能靠这个"立刻再错"掐断 sibling 分支。
pub const CONFORMANCE_INTERRUPT_MODE_HANG: i32 = 2;
/// cpp `Conformance.test.cpp:3745-3756`：`hangpcall` 用的中断，每满 1000 次抛一次错并把
/// 计数清零，好让外层循环里的下一次 `pcall` 又能跑满 1000 次中断。
pub const CONFORMANCE_INTERRUPT_MODE_HANG_PCALL: i32 = 3;

pub struct ConformanceInterruptState {
  pub mode: AtomicI32,
  pub index: AtomicI32,
}

impl ConformanceInterruptState {
  pub const fn new() -> Self {
    Self {
      mode: AtomicI32::new(CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS),
      index: AtomicI32::new(0),
    }
  }

  pub fn reset(&self, mode: i32) {
    self.index.store(0, Ordering::SeqCst);
    self.mode.store(mode, Ordering::SeqCst);
  }

  pub fn index(&self) -> i32 {
    self.index.load(Ordering::SeqCst)
  }
}

impl Default for ConformanceInterruptState {
  fn default() -> Self {
    Self::new()
  }
}

pub static CONFORMANCE_INTERRUPT_STATE: ConformanceInterruptState =
  ConformanceInterruptState::new();
