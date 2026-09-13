use crate::records::scoped_spills::ScopedSpills;

impl ScopedSpills {
  pub fn scoped_spills_scoped_spills(&mut self, _other: &ScopedSpills) {
    // C++ deletes copy constructor: ScopedSpills(const ScopedSpills&) = delete;
    // No-op stub to satisfy Rust translation schedule.
  }
}
