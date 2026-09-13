use crate::records::scoped_reg_x_64::ScopedRegX64;

impl ScopedRegX64 {
  pub fn scoped_reg_x_64_operator_assign(&mut self, _other: &ScopedRegX64) -> &mut ScopedRegX64 {
    // Assignment operator is deleted in C++ source; this is a no-op stub
    self
  }
}
