use ulua_cli_lib::enums::navigation_status::NavigationStatus;
pub use ulua_require::enums::luarequire_navigate_result::{
  LuarequireNavigateResult, LuarequireNavigateResult as luarequire_NavigateResult,
};

pub fn convert(status: NavigationStatus) -> LuarequireNavigateResult {
  match status {
    NavigationStatus::Success => LuarequireNavigateResult::NavigateSuccess,
    NavigationStatus::Ambiguous => LuarequireNavigateResult::NavigateAmbiguous,
    _ => LuarequireNavigateResult::NavigateNotFound,
  }
}
