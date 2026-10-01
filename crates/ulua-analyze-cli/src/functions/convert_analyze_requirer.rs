use ulua_cli_lib::enums::navigation_status::NavigationStatus;
use ulua_require::enums::navigate_result::NavigateResult;

pub fn convert(status: NavigationStatus) -> NavigateResult {
  match status {
    NavigationStatus::Success => NavigateResult::Success,
    NavigationStatus::Ambiguous => NavigateResult::Ambiguous,
    NavigationStatus::NotFound => NavigateResult::NotFound,
  }
}
