use crate::records::class_user_data::ClassUserData;

impl ClassUserData {
  // The C++ virtual destructor is handled by Rust's Drop trait or trait object cleanup.
  // For a base class with a virtual destructor, we provide an empty Drop implementation
  // if needed, but usually Rust's Box<dyn Trait> handles this automatically.
}

impl Drop for ClassUserData {
  fn drop(&mut self) {
    // virtual ~ClassUserData() {}
  }
}
