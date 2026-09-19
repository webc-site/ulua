use crate::enums::{
  luarequire_navigate_result::LuarequireNavigateResult, navigate_result::NavigateResult,
};

pub(crate) fn convert_navigate_result(result: LuarequireNavigateResult) -> NavigateResult {
  match result {
    LuarequireNavigateResult::NavigateSuccess => NavigateResult::Success,
    LuarequireNavigateResult::NavigateAmbiguous => NavigateResult::Ambiguous,
    LuarequireNavigateResult::NavigateNotFound => NavigateResult::NotFound,
  }
}
