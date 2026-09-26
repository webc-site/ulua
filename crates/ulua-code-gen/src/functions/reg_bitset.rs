//! VmReg 寄存器位图公共辅助：收口各处重复的 `(regs[r / 64] & (1u64 << (r % 64)))`
//! 位测试/置位表达式。`regs` 为 64 位字序的位图（RegisterSet.regs 为 4×u64，共 256 位）。

/// 位图每字位数（编译期常量，来自 u64::BITS）
pub const REG_WORD_BITS: usize = u64::BITS as usize;

/// 测试 `reg` 位是否置位
#[inline]
pub fn reg_bit_test(regs: &[u64], reg: usize) -> bool {
  regs[reg / REG_WORD_BITS] & (1u64 << (reg % REG_WORD_BITS)) != 0
}

/// 置位 `reg` 位
#[inline]
pub fn reg_bit_set(regs: &mut [u64], reg: usize) {
  regs[reg / REG_WORD_BITS] |= 1u64 << (reg % REG_WORD_BITS);
}
