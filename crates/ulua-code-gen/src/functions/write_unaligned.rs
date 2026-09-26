use core::{
  mem::size_of,
  ptr::{addr_of, copy_nonoverlapping},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writeu_8(target: *mut u8, value: u8) -> *mut u8 {
  // Safety: 契约保证 target 指向存活可写字节；u8 对齐要求平凡，写 1 字节并
  // 后移恰 size_of::<u8>()（1）字节，指针始终落在同一分配内。
  unsafe {
    *target = value;
    target.add(size_of::<u8>())
  }
}

macro_rules! writeu_le {
  ($name:ident, $ty:ty) => {
    /// # Safety
    /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
    #[inline]
    pub unsafe fn $name(target: *mut u8, value: $ty) -> *mut u8 {
      // Safety: 契约保证 target 指向存活、可写的字节缓冲且至少余 size_of::<$ty>() 字节;
      // le_value 为栈上局部, addr_of! 取其地址有效, copy_nonoverlapping 源/目的不重叠、逐字节无对齐要求,
      // 写入小端表示后返回 target.add(size) 仍落在同一分配内(至多 one-past-end)。
      unsafe {
        let le_value = value.to_le();
        copy_nonoverlapping(addr_of!(le_value).cast::<u8>(), target, size_of::<$ty>());
        target.add(size_of::<$ty>())
      }
    }
  };
}

writeu_le!(writeu_16, u16);
writeu_le!(writeu_32, u32);
writeu_le!(writeu_64, u64);

/// 小端机器上 `to_bits` 与直接拷贝字节等价；大端机器上 `writeu_32` 内部转小端，
/// 与 C++ `writef32` 的大端分支（经 u32 中转再转小端）行为一致，故收敛为 `to_bits` 转发。
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writef_32(target: *mut u8, value: f32) -> *mut u8 {
  unsafe { writeu_32(target, value.to_bits()) }
}

/// 小端机器上 `to_bits` 与直接拷贝字节等价；大端机器上 `writeu_64` 内部转小端，
/// 与 C++ `writef64` 的大端分支（经 u64 中转再转小端）行为一致，故收敛为 `to_bits` 转发。
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn writef_64(target: *mut u8, value: f64) -> *mut u8 {
  unsafe { writeu_64(target, value.to_bits()) }
}
