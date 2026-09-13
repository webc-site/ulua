use core::ffi::c_void;

use crate::records::temp_buffer::TempBuffer;

impl<T> TempBuffer<T> {
  /// C++ `TempBuffer& operator=(const TempBuffer&) = delete;`
  ///
  /// In Rust, we represent a deleted copy assignment operator by not providing
  /// a public assignment method for `TempBuffer`.
  pub fn operator_assign(&mut self, _other: &TempBuffer<c_void>) {
    panic!("TempBuffer copy assignment operator is deleted");
  }
}
