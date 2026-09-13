extern crate alloc;

use alloc::string::String;

use crate::type_aliases::definition::Definition;

/// # Safety
/// 调用方须保证 `def` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn dump_def(def: *mut Definition) -> String {
  unsafe {
    if !def.is_null() {
      return (*def).versioned_name();
    }
  }
  String::from("?")
}
