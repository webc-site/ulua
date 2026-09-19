/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
use core::ffi::c_char;
use core::{mem::size_of, ptr::read_unaligned};
pub(crate) unsafe fn read<T: Copy>(data: *const c_char, size: usize, offset: &mut usize) -> T {
  // cpp 此处为 `LUAU_ASSERT(size >= offset + sizeof(T))`（release 编译掉，损坏流直接 OOB read）；
  // Rust 以 panic 收口同等 UB 面，合法输入行为不变
  assert!(
    *offset + size_of::<T>() <= size,
    "truncated bytecode: read {} bytes at offset {} past size {}",
    size_of::<T>(),
    *offset,
    size
  );
  let result = unsafe {
    let src = data.add(*offset) as *const T;
    read_unaligned(src)
  };
  *offset += size_of::<T>();
  result
}
