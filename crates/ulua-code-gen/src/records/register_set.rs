#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct RegisterSet {
  pub regs: [u64; 4],

  // If variadic sequence is active, we track register from which it starts
  pub vararg_seq: bool,
  pub vararg_start: u8,
}
