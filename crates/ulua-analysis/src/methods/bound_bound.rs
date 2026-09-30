use crate::records::bound::Bound;

impl<T> Bound<T> {
  pub fn bound_t(bound_to: T) -> Self {
    Self { bound_to }
  }
}
