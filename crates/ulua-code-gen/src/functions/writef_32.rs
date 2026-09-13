use crate::functions::writeu_32::writeu_32;

/// 小端机器上 `to_bits` 与直接拷贝字节等价；大端机器上 `writeu_32` 内部转小端，
/// 与 C++ `writef32` 的大端分支（经 u32 中转再转小端）行为一致，故收敛为 `to_bits` 转发。
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writef_32(target: *mut u8, value: f32) -> *mut u8 {
  unsafe { writeu_32(target, value.to_bits()) }
}
