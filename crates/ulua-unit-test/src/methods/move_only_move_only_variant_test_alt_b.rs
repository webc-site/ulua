use crate::records::move_only::MoveOnly;

impl MoveOnly {
  pub fn move_only_move_only(&self, _rhs: &MoveOnly) -> Self {
    // C++: MoveOnly(const MoveOnly&) = delete;
    // This overload is deleted in C++; preserve that behavior at runtime.
    panic!("MoveOnly copy constructor is deleted in C++");
  }
}
