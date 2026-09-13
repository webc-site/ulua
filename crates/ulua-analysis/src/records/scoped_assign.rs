#[derive(Debug)]
pub struct ScopedAssign<T> {
  pub(crate) target: *mut T,
  pub(crate) old_value: T,
}

unsafe impl<T: Send> Send for ScopedAssign<T> {}
unsafe impl<T: Sync> Sync for ScopedAssign<T> {}
