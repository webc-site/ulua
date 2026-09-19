use core::mem::{size_of, transmute_copy};
#[inline]
pub fn buffer_swapbe<T>(v: T) -> T {
  // 按宽度分发字节交换：宽度 1 交换是恒等操作，直接返回
  match size_of::<T>() {
    8 => unsafe { transmute_copy::<u64, T>(&transmute_copy::<T, u64>(&v).swap_bytes()) },
    4 => unsafe { transmute_copy::<u32, T>(&transmute_copy::<T, u32>(&v).swap_bytes()) },
    2 => unsafe { transmute_copy::<u16, T>(&transmute_copy::<T, u16>(&v).swap_bytes()) },
    _ => v,
  }
}
