use core::ptr::null;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockIteratorWrapper {
  pub(crate) it_begin: *const u32,
  pub(crate) it_end: *const u32,
}

impl Default for BlockIteratorWrapper {
  fn default() -> Self {
    Self {
      it_begin: null(),
      it_end: null(),
    }
  }
}

impl Iterator for BlockIteratorWrapper {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    if self.it_begin < self.it_end {
      let value = unsafe { *self.it_begin };
      self.it_begin = unsafe { self.it_begin.add(1) };
      Some(value)
    } else {
      None
    }
  }
}
