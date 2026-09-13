use core::{
  mem::{MaybeUninit, size_of},
  ptr::copy_nonoverlapping,
};
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn read<T>(data: *const u8, offset: &mut usize) -> T {
  unsafe {
    let mut result = MaybeUninit::<T>::uninit();
    copy_nonoverlapping(
      data.add(*offset),
      result.as_mut_ptr() as *mut u8,
      size_of::<T>(),
    );
    *offset += size_of::<T>();
    result.assume_init()
  }
}
