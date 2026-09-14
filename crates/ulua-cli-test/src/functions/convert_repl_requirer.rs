use ulua_cli_lib::enums::navigation_status::NavigationStatus;
pub use ulua_require::enums::luarequire_navigate_result::luarequire_NavigateResult;

pub fn convert_navigation_status(status: NavigationStatus) -> luarequire_NavigateResult {
  match status {
    NavigationStatus::Success => luarequire_NavigateResult::NAVIGATE_SUCCESS,
    NavigationStatus::Ambiguous => luarequire_NavigateResult::NAVIGATE_AMBIGUOUS,
    NavigationStatus::NotFound => luarequire_NavigateResult::NAVIGATE_NOT_FOUND,
  }
}
