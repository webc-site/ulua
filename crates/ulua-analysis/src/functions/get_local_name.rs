use core::ffi::CStr;
extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_local::AstLocal;

/// # Safety
/// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_local_name(local: *mut AstLocal) -> String {
  unsafe {
    if !local.is_null() && !(*local).name.value.is_null() {
      return CStr::from_ptr((*local).name.value)
        .to_string_lossy()
        .into_owned();
    }
  }
  String::from("?")
}
