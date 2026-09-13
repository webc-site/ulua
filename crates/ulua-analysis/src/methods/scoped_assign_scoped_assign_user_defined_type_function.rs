use crate::records::scoped_assign::ScopedAssign;

impl<T: Clone> ScopedAssign<T> {
  pub fn new(target: &mut T, value: T) -> Self {
    let old_value = target.clone();
    *target = value;
    Self {
      target: target as *mut T,
      old_value,
    }
  }
}
