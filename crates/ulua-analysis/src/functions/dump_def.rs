extern crate alloc;

use alloc::string::String;

use crate::records::sym_def::SymDef;

/// # Safety
/// 调用方须保证 `def` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn dump_def(def: *mut SymDef) -> String {
  unsafe {
    if !def.is_null() {
      return (*def).versioned_name();
    }
  }
  String::from("?")
}
