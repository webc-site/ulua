use core::{ffi::c_char, slice};

mod _inner {
  use super::*;

  /// FNV-1a 32 位偏移基数。
  const FNV_OFFSET_BASIS: u32 = 2166136261;
  /// FNV-1a 32 位质数。
  const FNV_PRIME: u32 = 16777619;

  /// # Safety
  /// `data` 必须在 `size` 字节范围内可读，或者 `size` 为 0。
  pub unsafe fn hash_range(data: *const c_char, size: usize) -> usize {
    if size == 0 || data.is_null() {
      return FNV_OFFSET_BASIS as usize;
    }
    // SAFETY：上方已保证 data 非空且指向 size 个可读字节。
    let slice = unsafe { slice::from_raw_parts(data as *const u8, size) };
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in slice {
      hash ^= byte as u32;
      hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash as usize
  }
}

pub use _inner::{hash_range, hash_range as hashRange};
