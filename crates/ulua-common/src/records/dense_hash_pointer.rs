use core::ffi::c_void;

#[derive(Debug, Clone, Copy, Default)]
pub struct DenseHashPointer;

impl DenseHashPointer {
  #[inline]
  pub fn hash(&self, key: *const c_void) -> usize {
    let addr = key as usize;
    (addr >> 4) ^ (addr >> 9)
  }
}

impl DenseHashPointer {
  #[inline]
  pub fn call(&self, key: *const c_void) -> usize {
    self.hash(key)
  }
}
