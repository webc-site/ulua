use core::ptr::swap;

use crate::records::scoped_assign::ScopedAssign;
impl<T> Drop for ScopedAssign<T> {
  fn drop(&mut self) {
    unsafe {
      swap(self.target, &mut self.old_value);
    }
  }
}
