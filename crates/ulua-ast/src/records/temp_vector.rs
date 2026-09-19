pub struct TempVector<'a, T> {
  pub(crate) storage: *mut Vec<T>,
  pub(crate) offset: usize,
  pub(crate) size_: usize,
  pub(crate) _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Debug for TempVector<'a, T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let storage_len = unsafe { (*self.storage).len() };
    f.debug_struct("TempVector")
      .field("storage_len", &storage_len)
      .field("offset", &self.offset)
      .field("size_", &self.size_)
      .finish()
  }
}
use alloc::vec::Vec;
use core::{
  fmt::{Debug, Formatter, Result},
  marker::PhantomData,
};
