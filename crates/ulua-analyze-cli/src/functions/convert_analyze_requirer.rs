use ulua_cli_lib::enums::navigation_status::NavigationStatus;

use crate::type_aliases::navigate_result::NavigateResult;

pub fn convert(status: NavigationStatus) -> NavigateResult {
  match status {
    NavigationStatus::Success => NavigateResult::Success,
    NavigationStatus::Ambiguous => NavigateResult::Ambiguous,
    NavigationStatus::NotFound => NavigateResult::NotFound,
  }
}

// Pinned overload name advertised by the dependency cards.
pub use convert as convert_navigation_status;
