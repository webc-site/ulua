use crate::records::scoped_spills::ScopedSpills;

impl ScopedSpills {
  pub fn scoped_spills_operator_assign(&mut self, _other: &ScopedSpills) -> &mut ScopedSpills {
    // Assignment operator is deleted in C++ source; this is a no-op stub
    self
  }
}
