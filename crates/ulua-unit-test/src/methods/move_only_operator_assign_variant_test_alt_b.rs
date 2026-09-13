use crate::records::move_only::MoveOnly;

impl MoveOnly {
  pub fn operator_assign_mut(&mut self) {
    // C++: MoveOnly& operator=(MoveOnly&&) = default;
  }
}
