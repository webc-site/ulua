use alloc::{boxed::Box, string::String};
use core::option::Option;

use crate::{
  records::extern_type::ExternType, type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

/// 宿主注入的字符串补全回调（C++ `std::function` 成员直译）。`dyn` 保留：
/// 闭包具体类型由调用方运行期决定，编译期不可枚举，无法单态化。
pub type StringCompletionCallback =
  Box<dyn Fn(String, Option<*const ExternType>, Option<String>) -> Option<AutocompleteEntryMap>>;
