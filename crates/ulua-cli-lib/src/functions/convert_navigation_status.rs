//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：将 `NavigationStatus` 映射为
//! [`ulua_require::enums::navigate_result::NavigateResult`]。两侧此前各
//! 有一份逐分支相同的实现，仅匹配写法不同（穷举 match 与 `_ =>` 恒等），
//! 语义完全一致。

use ulua_require::enums::navigate_result::NavigateResult;

use crate::enums::navigation_status::NavigationStatus;

pub fn convert_navigation_status(status: NavigationStatus) -> NavigateResult {
  match status {
    NavigationStatus::Success => NavigateResult::Success,
    NavigationStatus::Ambiguous => NavigateResult::Ambiguous,
    NavigationStatus::NotFound => NavigateResult::NotFound,
  }
}
