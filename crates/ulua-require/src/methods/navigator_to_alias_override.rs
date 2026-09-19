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
  pub(crate) fn to_alias_override(&mut self, alias_unprefixed: &[u8]) -> (Error, bool) {
    match self.navigation_context.to_alias_override(alias_unprefixed) {
      NavigateResult::Success => (None, true),
      NavigateResult::NotFound => (None, false),
      NavigateResult::Ambiguous => (Some(invalid_alias(alias_unprefixed, true)), false),
    }
  }
}
