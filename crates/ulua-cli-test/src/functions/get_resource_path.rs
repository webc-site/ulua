use alloc::string::String;
use std::sync::OnceLock;

use crate::functions::get_resource_path_0::get_resource_path_0;

/// 借用 OnceLock 缓存的资源路径，免 `String` clone（零拷贝）。
pub fn get_resource_path() -> Option<&'static str> {
  static PATH: OnceLock<Option<String>> = OnceLock::new();
  PATH.get_or_init(get_resource_path_0).as_deref()
}
