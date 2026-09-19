//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：将 `NavigationStatus` 映射为
//! require 回调所需的 `luarequire_NavigateResult`（C-ABI 枚举）。两侧此前各
//! 有一份逐分支相同的实现，仅匹配写法不同（穷举 match 与 `_ =>` 恒等），
//! 语义完全一致。

use ulua_require::enums::luarequire_navigate_result::luarequire_NavigateResult;

use crate::enums::navigation_status::NavigationStatus;

pub fn convert_navigation_status(status: NavigationStatus) -> luarequire_NavigateResult {
  match status {
    NavigationStatus::Success => luarequire_NavigateResult::NAVIGATE_SUCCESS,
    NavigationStatus::Ambiguous => luarequire_NavigateResult::NAVIGATE_AMBIGUOUS,
    NavigationStatus::NotFound => luarequire_NavigateResult::NAVIGATE_NOT_FOUND,
  }
}
