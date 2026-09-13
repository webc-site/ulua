/// 统一实现 C++ `writeu16/32/64` 的小端写入：先转小端，再原样拷贝字节。
/// C++ 中 writeu32 的大端分支与 writeu16/64 的无条件转换行为等价（均写入小端字节序），故收敛为一个宏。
/// 宏体使用绝对路径，展开到调用模块时无需额外导入。
macro_rules! writeu_le {
  ($name:ident, $ty:ty) => {
    /// # Safety
    /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
    #[inline]
    pub unsafe fn $name(target: *mut u8, value: $ty) -> *mut u8 {
      unsafe {
        let le_value = value.to_le();
        ::core::ptr::copy_nonoverlapping(
          ::core::ptr::addr_of!(le_value).cast::<u8>(),
          target,
          ::core::mem::size_of::<$ty>(),
        );
        target.add(::core::mem::size_of::<$ty>())
      }
    }
  };
}
pub(crate) use writeu_le;

writeu_le!(writeu_16, u16);
