#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct RegisterSet {
  pub regs: [u64; 4],

  // 若 variadic 序列激活，记录其起始寄存器
  pub vararg_seq: bool,
  pub vararg_start: u8,
}
