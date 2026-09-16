use core::mem::{size_of, transmute_copy};
#[inline]
pub fn buffer_swapbe<T>(v: T) -> T {
  let size = size_of::<T>();
  if size == 8 {
    unsafe {
      let val = transmute_copy::<T, u64>(&v);
      let swapped = val.swap_bytes();
      transmute_copy::<u64, T>(&swapped)
    }
  } else if size == 4 {
    unsafe {
      let val = transmute_copy::<T, u32>(&v);
      let swapped = val.swap_bytes();
      transmute_copy::<u32, T>(&swapped)
    }
  } else if size == 2 {
    unsafe {
      let val = transmute_copy::<T, u16>(&v);
      let swapped = val.swap_bytes();
      transmute_copy::<u16, T>(&swapped)
    }
  } else {
    v
  }
}
