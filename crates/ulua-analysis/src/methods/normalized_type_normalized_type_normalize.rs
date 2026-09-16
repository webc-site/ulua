use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  /// In C++, this constructor is deleted to prevent default construction.
  /// In Rust, `NormalizedType` does not implement `Default`.
  pub fn normalized_type() {
    panic!("NormalizedType default constructor is deleted");
  }
}
