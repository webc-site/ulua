use alloc::string::String;
use std::sync::OnceLock;

use crate::functions::get_resource_path_0::get_resource_path_0;

pub fn get_resource_path() -> Option<String> {
  static PATH: OnceLock<Option<String>> = OnceLock::new();
  PATH.get_or_init(get_resource_path_0).clone()
}
