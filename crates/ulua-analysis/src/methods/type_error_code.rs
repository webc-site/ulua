use crate::records::type_error::TypeError;

impl TypeError {
  #[inline]
  pub fn code(&self) -> i32 {
    Self::min_code() + self.data.index()
  }
}
