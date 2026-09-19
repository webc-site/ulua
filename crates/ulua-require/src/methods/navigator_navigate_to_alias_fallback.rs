use crate::{
  enums::navigate_result::NavigateResult,
  functions::navigate_error::invalid_alias,
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn navigate_to_alias_fallback(&mut self, alias_unprefixed: &[u8]) -> Error {
    let result = self.navigation_context.to_alias_fallback(alias_unprefixed);

    if result == NavigateResult::Success {
      return None;
    }

    Some(invalid_alias(
      alias_unprefixed,
      result == NavigateResult::Ambiguous,
    ))
  }
}
