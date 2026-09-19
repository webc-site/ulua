use crate::{
  enums::navigate_result::NavigateResult,
  functions::navigate_error::{ambiguous, suffixed},
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

/// cpp 固定文案。
const RESET_FAILED: &[u8] = b"could not reset to requiring context";

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn reset_to_requirer(&mut self) -> Error {
    let result = self.navigation_context.reset_to_requirer();
    if result == NavigateResult::Success {
      return None;
    }

    Some(suffixed(RESET_FAILED, ambiguous(result)))
  }
}
