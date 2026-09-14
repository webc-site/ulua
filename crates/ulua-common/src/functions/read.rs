use core::{
  mem::{MaybeUninit, size_of},
  ptr::copy_nonoverlapping,
};

pub fn read<T: Copy>(data: &[u8], offset: &mut usize) -> T {
  let size = size_of::<T>();
  assert!(*offset + size <= data.len(), "read out of bounds");

  let mut result = MaybeUninit::<T>::uninit();
  unsafe {
    copy_nonoverlapping(
      data.as_ptr().add(*offset),
      result.as_mut_ptr() as *mut u8,
      size,
    );
    *offset += size;
    result.assume_init()
  }
}
