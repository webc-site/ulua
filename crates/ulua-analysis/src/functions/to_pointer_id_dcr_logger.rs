extern crate alloc;

use alloc::string::{String, ToString};

pub(crate) fn to_pointer_id<T>(ptr: *const T) -> String {
  (ptr as usize).to_string()
}
