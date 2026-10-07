use alloc::{boxed::Box, string::String};

use crate::{
  records::extern_type::ExternType, type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

/// 宿主注入的字符串补全回调（C++ `std::function` 成员直译）。
///
/// `dyn` 保留（review.md §4）：它按值存在 `AutocompleteArgs::callback` /
/// `FragmentAutocompleteOptions::callback` / `autocomplete` 系列**结构体字段与
/// pub 形参**上，`impl Trait` 不能作字段类型；闭包具体类型由宿主
/// （ulua-unit-test 补全夹具、ulua-web/ulua-analyze-cli 侧宿主）运行期决定，
/// 编译期不可枚举，enum_dispatch 无适用空间。
///
/// 第二参 cpp 侧是 `const void* methodContainingExternType`（宿主只透传、不解释
/// 的裸指针），Rust 侧收成类型化的 arena 只读引用 [`ExternType`]（review.md §2：
/// 纯逻辑层不外泄裸指针）；`None` 即 cpp 的 `nullptr`。
pub type StringCompletionCallback =
  Box<dyn Fn(String, Option<&'static ExternType>, Option<String>) -> Option<AutocompleteEntryMap>>;
