use crate::records::move_only::MoveOnly;

impl MoveOnly {
  pub fn operator_assign(&mut self, _rhs: &MoveOnly) -> &mut Self {
    // C++: MoveOnly& operator=(const MoveOnly&) = delete;
    // This overload is deleted in C++; in Rust we implement it as a no-op stub
    // that panics if ever called, preserving the "deleted" semantics at runtime.
    panic!("MoveOnly assignment operator is deleted in C++");
  }
}
