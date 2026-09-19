use crate::records::fixture::Fixture;

impl Fixture {
  pub fn fixture_operator_assign(&mut self) {
    // C++: Fixture& operator=(const Fixture&) = delete;
    // This overload is deleted in C++; in Rust we implement it as a no-op stub
    // that panics if ever called, preserving the "deleted" semantics at runtime.
    panic!("Fixture assignment operator is deleted in C++");
  }
}
