#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CodeGenCounter {
  RegularBlockExecuted = 1,
  FallbackBlockExecuted = 2,
  VmExitTaken = 3,
}
