use crate::records::fixture::Fixture;

impl Fixture {
  pub fn fixture_fixture(&mut self, _rhs: &Fixture) {
    // C++: Fixture(const Fixture&) = delete;
    // This overload is deleted in C++; preserve that behavior at runtime.
    panic!("Fixture copy constructor is deleted in C++");
  }
}
