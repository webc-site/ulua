use alloc::string::{String, ToString};

use crate::records::cli_file_resolver::CliFileResolver;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn cli_file_resolver_get_human_readable_module_name(
  _this: *const CliFileResolver,
  name: &String,
) -> String {
  if name == "-" {
    "stdin".to_string()
  } else {
    name.clone()
  }
}
